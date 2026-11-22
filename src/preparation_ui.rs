use egui_macroquad::egui;
use rust_3d_print_studio::{
    project::{History, Project},
    slicer::{self, Preparation},
};
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver},
};
pub struct Panel {
    template: String,
    loaded: Option<Preparation>,
    program: String,
    output: String,
    sliced_output: String,
    override_values: bool,
    layer: f64,
    walls: u8,
    infill: u8,
    support: bool,
    tab: usize,
    section: crate::section_ui::SectionView,
    busy: Option<Receiver<String>>,
    status: String,
}
impl Default for Panel {
    fn default() -> Self {
        Self {
            template: String::new(),
            loaded: None,
            program: slicer::detect()
                .map(|p| p.display().to_string())
                .unwrap_or_default(),
            output: "Druckplatte.3mf".into(),
            sliced_output: "Druckplatte.gcode.3mf".into(),
            override_values: false,
            layer: 0.2,
            walls: 2,
            infill: 15,
            support: false,
            tab: 0,
            section: Default::default(),
            busy: None,
            status: String::new(),
        }
    }
}
impl Panel {
    pub fn reopen(&mut self) {
        self.section.reset();
    }
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        p: &mut Project,
        history: &mut History,
        dirty: &mut bool,
    ) {
        ui.set_min_width(470.);
        ui.heading("Platte vorbereiten");
        ui.label(format!(
            "{}   ·   {} × {} mm   ·   {} Objekte",
            p.printer.name,
            p.printer.bed[0],
            p.printer.bed[1],
            p.objects.len()
        ));
        ui.separator();
        ui.horizontal(|ui| {
            for (i, s) in ["Druckprofil", "Filamente", "Schnittansicht", "Übergabe"]
                .iter()
                .enumerate()
            {
                ui.selectable_value(&mut self.tab, i, *s);
            }
        });
        ui.add_space(8.);
        match self.tab {
            0 => {
                ui.label("Vorlage aus Bambu Studio (.3mf, ungeslict)");
                if ui.text_edit_singleline(&mut self.template).changed() {
                    self.loaded = None;
                    self.override_values = false;
                }
                if ui.button("Profil aus Vorlage laden").clicked() {
                    match Preparation::load(Path::new(self.template.trim())) {
                        Ok(v) => {
                            self.layer = v.label("layer_height").parse().unwrap_or(0.2);
                            self.walls = v.label("wall_loops").parse().unwrap_or(2);
                            self.infill = v
                                .label("sparse_infill_density")
                                .trim_end_matches('%')
                                .parse()
                                .unwrap_or(15);
                            self.support = v.label("enable_support") == "1";
                            self.override_values = false;
                            self.status.clear();
                            self.loaded = Some(v)
                        }
                        Err(e) => {
                            self.loaded = None;
                            self.status = format!("Vorlage: {e}")
                        }
                    }
                }
                if let Some(v) = &self.loaded {
                    ui.label(v.label("printer_settings_id"));
                    ui.label(v.label("print_settings_id"));
                    ui.checkbox(
                        &mut self.override_values,
                        "Prozesswerte für diese Platte ändern",
                    );
                    ui.add_enabled_ui(self.override_values, |ui| {
                        egui::Grid::new("process-fields").show(ui, |ui| {
                            ui.label("Schichthöhe");
                            ui.add(
                                egui::DragValue::new(&mut self.layer)
                                    .speed(0.01)
                                    .range(0.02..=1.)
                                    .suffix(" mm"),
                            );
                            ui.end_row();
                            ui.label("Wandlinien");
                            ui.add(egui::DragValue::new(&mut self.walls).range(1..=20));
                            ui.end_row();
                            ui.label("Füllung");
                            ui.add(
                                egui::DragValue::new(&mut self.infill)
                                    .range(0..=100)
                                    .suffix(" %"),
                            );
                            ui.end_row();
                        });
                        ui.checkbox(&mut self.support, "Stützstrukturen aktivieren");
                    });
                } else {
                    ui.label("Ohne Vorlage wird eine Modell-3MF übergeben. Druckprofil und Material wählst du dann im Slicer.");
                }
            }
            1 => {
                if let Some(v) = &self.loaded {
                    let slots = v.slots();
                    for (i, name) in slots.iter().enumerate() {
                        ui.horizontal(|ui| {
                            if let Some(c) = v.slot_color(i) {
                                let (r, _) = ui.allocate_exact_size(
                                    egui::vec2(16., 16.),
                                    egui::Sense::hover(),
                                );
                                ui.painter().rect_filled(
                                    r,
                                    3.,
                                    egui::Color32::from_rgb(c[0], c[1], c[2]),
                                );
                            }
                            ui.label(format!("{}   {}", i + 1, name));
                        });
                    }
                    ui.separator();
                    ui.label("Filament je Objekt");
                    egui::ScrollArea::vertical()
                        .max_height(220.)
                        .show(ui, |ui| {
                            for i in 0..p.objects.len() {
                                let mut slot = p.objects[i].filament_slot;
                                ui.horizontal(|ui| {
                                    ui.label(&p.objects[i].name);
                                    ui.add(egui::DragValue::new(&mut slot).range(1..=slots.len()));
                                });
                                if slot != p.objects[i].filament_slot {
                                    history.record(p.clone());
                                    p.objects[i].filament_slot = slot;
                                    self.section.reset();
                                    *dirty = true;
                                }
                            }
                        });
                } else {
                    ui.label("Lade unter Druckprofil eine Vorlage mit den gewünschten Filamenten.");
                }
            }
            2 => self.section.show(ui, p, self.loaded.as_ref()),
            _ => {
                ui.label("Neue Ausgabedatei (.3mf)");
                ui.text_edit_singleline(&mut self.output);
                ui.label("Bambu Studio – Programmpfad");
                ui.text_edit_singleline(&mut self.program);
                ui.label("Schichtvorschau, Bemalen und die Zuordnung zum physischen AMS erfolgen in Bambu Studio.");
                let ready = self.busy.is_none()
                    && !p.objects.is_empty()
                    && (self.template.trim().is_empty() || self.loaded.is_some());
                ui.add_enabled_ui(ready, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("3MF speichern").clicked() {
                            self.start(p, false, false)
                        }
                        if ui
                            .add_enabled(
                                !self.program.trim().is_empty(),
                                egui::Button::new("Speichern und im Slicer öffnen"),
                            )
                            .clicked()
                        {
                            self.start(p, true, false)
                        }
                    });
                    ui.separator();
                    ui.label("Lokale Druckdatei (.gcode.3mf)");
                    ui.text_edit_singleline(&mut self.sliced_output);
                    if ui
                        .add_enabled(
                            self.loaded.is_some() && !self.program.trim().is_empty(),
                            egui::Button::new("Platte lokal slicen"),
                        )
                        .on_disabled_hover_text(
                            "P1S-Projektvorlage laden und lokalen Slicer angeben",
                        )
                        .clicked()
                    {
                        self.start(p, false, true);
                    }
                });
                if ui.button("Geslicte Datei zur Vorschau öffnen").clicked() {
                    match slicer::launch(
                        Path::new(self.program.trim()),
                        Path::new(self.sliced_output.trim()),
                    ) {
                        Ok(()) => {
                            self.status = "Druckdatei an die lokale Vorschau übergeben".into()
                        }
                        Err(e) => self.status = format!("Vorschau: {e}"),
                    }
                }
            }
        }
        if let Some(rx) = &self.busy {
            match rx.try_recv() {
                Ok(s) => {
                    self.status = s;
                    self.busy = None
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.status = "Export unterbrochen".into();
                    self.busy = None
                }
                Err(_) => {
                    ui.spinner();
                }
            }
        }
        if !self.status.is_empty() {
            ui.separator();
            ui.label(&self.status);
        }
    }
    fn start(&mut self, p: &Project, open: bool, slice: bool) {
        let mut prep = self.loaded.clone();
        if let Some(v) = &mut prep {
            if self.override_values {
                if let Err(e) =
                    v.override_process(self.layer, self.walls, self.infill, self.support)
                {
                    self.status = e.to_string();
                    return;
                }
            }
        }
        let path = PathBuf::from(if slice {
            self.sliced_output.trim()
        } else {
            self.output.trim()
        });
        let exe = PathBuf::from(self.program.trim());
        let p = p.clone();
        let (tx, rx) = mpsc::channel();
        self.busy = Some(rx);
        self.status = if slice {
            "Lokale Engine berechnet Werkzeugwege …"
        } else {
            "Platte wird exportiert …"
        }
        .into();
        std::thread::spawn(move || {
            if slice {
                let result = prep
                    .as_ref()
                    .ok_or_else(|| std::io::Error::other("Vorlage fehlt"))
                    .and_then(|v| rust_3d_print_studio::engine::slice(&exe, &path, &p, v));
                let _ = tx.send(result.unwrap_or_else(|e| format!("Slicing fehlgeschlagen: {e}")));
                return;
            }
            let result = match slicer::export(&path, &p, prep.as_ref()) {
                Err(e) => format!("Export fehlgeschlagen: {e}"),
                Ok(()) => {
                    let dest = path.canonicalize().unwrap_or(path);
                    if open {
                        match slicer::launch(&exe, &dest) {
                            Ok(()) => format!("An Bambu Studio übergeben: {}", dest.display()),
                            Err(e) => {
                                format!("{} gespeichert. Start fehlgeschlagen: {e}", dest.display())
                            }
                        }
                    } else {
                        format!("Gespeichert: {}", dest.display())
                    }
                }
            };
            let _ = tx.send(result);
        });
    }
}
