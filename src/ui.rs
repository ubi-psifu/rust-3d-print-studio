use crate::renderer;
use egui_macroquad::egui::{self, Color32, RichText};
use glam::DVec3;
use rust_3d_print_studio::{
    files::{self, Output},
    geometry::{Object, Shape},
    project::{History, Printer, Project, View, MAX_OBJECTS},
};
use std::path::{Path, PathBuf};
#[derive(Clone, Copy)]
enum Action {
    New,
    Open,
    Quit,
}
#[derive(Clone, Copy)]
enum FileTask {
    Open,
    Profile,
    Write(Output),
}
#[derive(Clone, Copy)]
enum Dialog {
    File(FileTask),
    Overwrite(Output),
    Discard(Action),
    Help,
}
pub struct Studio {
    p: Project,
    hist: History,
    sel: Option<usize>,
    draft: Option<Object>,
    cache: Vec<renderer::Solid>,
    pub dark: bool,
    wire: bool,
    snap: bool,
    rect: egui::Rect,
    px: f32,
    path: Option<PathBuf>,
    entry: String,
    dialog: Option<Dialog>,
    message: String,
    dirty: bool,
    pub quit: bool,
    shortcut: bool,
    keys: [egui::Key; 3],
}
const ACCENT: Color32 = Color32::from_rgb(91, 201, 182);
impl Studio {
    pub fn new(path: Option<PathBuf>) -> Self {
        let mut app = Self {
            p: Project::default(),
            hist: History::default(),
            sel: None,
            draft: None,
            cache: vec![],
            dark: true,
            wire: false,
            snap: false,
            rect: egui::Rect::NOTHING,
            px: 1.,
            path: None,
            entry: String::new(),
            dialog: None,
            message: "Bereit".into(),
            dirty: false,
            quit: false,
            shortcut: true,
            keys: [egui::Key::Z, egui::Key::Y, egui::Key::S],
        };
        if let Some(path) = path {
            match files::load(&path) {
                Ok(p) => {
                    app.p = p;
                    app.path = Some(path);
                    app.select(Some(0));
                }
                Err(e) => app.message = e.to_string(),
            }
        }
        app
    }
    fn select(&mut self, i: Option<usize>) {
        self.sel = i.filter(|i| *i < self.p.objects.len());
        self.draft = self.sel.map(|i| self.p.objects[i].clone());
    }
    fn remember(&mut self) {
        self.hist.record(self.p.clone());
        self.dirty = true;
    }
    fn add(&mut self, shape: Shape) {
        if self.p.objects.len() >= MAX_OBJECTS {
            self.message = "Objektgrenze erreicht".into();
            return;
        }
        self.remember();
        let [x, y] = self.p.printer.bed;
        let mut o = Object::new(shape, DVec3::new(x / 2., y / 2., 0.));
        o.name = format!("{} {}", shape.label(), self.p.objects.len() + 1);
        self.p.objects.push(o);
        self.select(Some(self.p.objects.len() - 1));
        self.message = "Körper erstellt".into();
    }
    fn duplicate(&mut self) {
        if let Some(i) = self.sel {
            if self.p.objects.len() >= MAX_OBJECTS {
                self.message = "Objektgrenze erreicht".into();
                return;
            }
            self.remember();
            let mut o = self.p.objects[i].clone();
            o.position.x = (o.position.x + 5.).min(10000.);
            self.p.objects.push(o);
            self.select(Some(self.p.objects.len() - 1));
        }
    }
    fn delete(&mut self) {
        if let Some(i) = self.sel {
            self.remember();
            self.p.objects.remove(i);
            self.select(i.checked_sub(1).or(Some(0)));
        }
    }
    fn undo(&mut self, redo: bool) {
        let changed = if redo {
            self.hist.redo(&mut self.p)
        } else {
            self.hist.undo(&mut self.p)
        };
        if changed {
            self.dirty = true;
            self.select(self.sel.or(Some(0)));
        }
    }
    fn place(&mut self, center: bool) {
        if let Some(i) = self.sel {
            let (lo, hi) = self.p.objects[i].mesh().bounds();
            let mut delta = DVec3::new(0., 0., -lo.z);
            if center {
                delta.x = (self.p.printer.bed[0] - lo.x - hi.x) * 0.5;
                delta.y = (self.p.printer.bed[1] - lo.y - hi.y) * 0.5;
            }
            self.remember();
            self.p.objects[i].position += delta;
            self.select(Some(i));
        }
    }
    fn focus(&mut self) {
        if let Some(i) = self.sel {
            let (a, b) = self.p.objects[i].mesh().bounds();
            self.p.view.target = (a + b) * 0.5;
            self.p.view.distance = ((b - a).length() * 2.).clamp(8., 5000.);
        }
    }
    fn choose(&mut self, t: FileTask) {
        self.entry = match t {
            FileTask::Write(f) => self
                .path
                .clone()
                .unwrap_or_else(|| "Modell.r3dp".into())
                .with_extension(f.ext())
                .to_string_lossy()
                .into(),
            _ => String::new(),
        };
        self.dialog = Some(Dialog::File(t));
        self.message.clear();
    }
    fn action(&mut self, a: Action) {
        if self.dirty {
            self.dialog = Some(Dialog::Discard(a));
        } else {
            self.proceed(a)
        }
    }
    fn proceed(&mut self, a: Action) {
        self.dialog = None;
        match a {
            Action::New => {
                self.p = Project::default();
                self.path = None;
                self.hist = History::default();
                self.select(None);
                self.dirty = false;
                self.message = "Neues Projekt".into();
            }
            Action::Open => self.choose(FileTask::Open),
            Action::Quit => self.quit = true,
        }
    }
    pub fn request_quit(&mut self) {
        self.action(Action::Quit);
    }
    fn write(&mut self, f: Output, overwrite: bool) {
        let path = PathBuf::from(self.entry.trim());
        match files::save(&path, &self.p, f, overwrite) {
            Ok(()) => {
                if f == Output::Project {
                    self.path = Some(path);
                    self.dirty = false;
                }
                self.message = format!("Gespeichert · {}", self.entry);
                self.dialog = None
            }
            Err(e) => {
                self.message = format!("Speichern fehlgeschlagen · {e}");
                self.dialog = Some(Dialog::File(FileTask::Write(f)));
            }
        }
    }
    fn save(&mut self) {
        if let Some(p) = &self.path {
            self.entry = p.to_string_lossy().into();
            self.write(Output::Project, true)
        } else {
            self.choose(FileTask::Write(Output::Project))
        }
    }
    fn file_submit(&mut self, t: FileTask) {
        let path = Path::new(self.entry.trim());
        match t {
            FileTask::Open => match files::load(path) {
                Ok(p) => {
                    self.p = p;
                    self.path = Some(path.into());
                    self.select(Some(0));
                    self.hist = History::default();
                    self.dirty = false;
                    self.dialog = None;
                    self.message = "Projekt geöffnet".into();
                }
                Err(e) => self.message = format!("Öffnen fehlgeschlagen · {e}"),
            },
            FileTask::Profile => {
                let result = files::read_limited(path)
                    .and_then(|b| Ok(serde_json::from_slice::<Printer>(&b)?));
                match result {
                    Ok(profile) if profile.valid() => {
                        self.remember();
                        self.p.printer = profile;
                        self.dialog = None;
                        self.message = "Druckerprofil übernommen".into();
                    }
                    _ => self.message = "Ungültiges Druckerprofil".into(),
                }
            }
            FileTask::Write(f) => {
                if path.exists() {
                    self.dialog = Some(Dialog::Overwrite(f));
                } else {
                    self.write(f, false)
                }
            }
        }
    }
    fn dialogs(&mut self, ctx: &egui::Context) {
        let Some(d) = self.dialog else { return };
        egui::Modal::new("dialog".into()).show(ctx,|ui|{ui.set_min_width(370.);match d{
 Dialog::Help=>{ui.heading("Bedienung");ui.label("Links klicken: Körper auswählen\nRechts ziehen: Ansicht drehen\nMitte ziehen: Ansicht verschieben\nMausrad: zoomen");ui.separator();ui.label("Projekt öffnen: R3DP\nNetzexport: STL binär, OBJ und Modell-3MF in mm.\nWeitere Importformate und CAD-Operationen sind offen.");},
 Dialog::Discard(a)=>{ui.heading("Ungespeicherte Änderungen");ui.label("Sollen die Änderungen verworfen werden?");if ui.button("Verwerfen und fortfahren").clicked(){self.proceed(a);}},
 Dialog::Overwrite(f)=>{ui.heading("Vorhandene Datei ersetzen?");ui.label(&self.entry);if ui.button("Datei ersetzen").clicked(){self.write(f,true);}},
 Dialog::File(t)=>{ui.heading(match t{FileTask::Open=>"Projekt öffnen",FileTask::Profile=>"Druckerprofil laden",FileTask::Write(Output::Project)=>"Projekt speichern",_=>"Netz exportieren"});ui.label("Dateipfad mit Endung");ui.text_edit_singleline(&mut self.entry);
 if matches!(t,FileTask::Write(Output::Stl|Output::Obj|Output::ThreeMf)){ui.label("Exportiert Dreiecksnetze in mm. Konstruktionsparameter bleiben im R3DP-Projekt.");}
 if ui.button("Bestätigen").clicked(){self.file_submit(t);}
 if !self.message.is_empty(){ui.colored_label(Color32::from_rgb(233,155,92),&self.message);}}}
 if ui.button(if matches!(d,Dialog::Help){"Schließen"}else{"Abbrechen"}).clicked(){self.dialog=None;}
 });
    }
    fn views(&mut self, ui: &mut egui::Ui) {
        for (label, yaw, pitch) in [
            ("Isometrisch", -std::f64::consts::FRAC_PI_4, 0.61548),
            ("Vorne", -std::f64::consts::FRAC_PI_2, 0.),
            ("Hinten", std::f64::consts::FRAC_PI_2, 0.),
            ("Links", std::f64::consts::PI, 0.),
            ("Rechts", 0., 0.),
            (
                "Oben",
                -std::f64::consts::FRAC_PI_2,
                std::f64::consts::FRAC_PI_2,
            ),
            (
                "Unten",
                -std::f64::consts::FRAC_PI_2,
                -std::f64::consts::FRAC_PI_2,
            ),
        ] {
            if item(ui, label) {
                self.p.view.yaw = yaw;
                self.p.view.pitch = pitch;
            }
        }
        if item(ui, "Druckbett einpassen") {
            self.p.view = View::default();
            let [x, y] = self.p.printer.bed;
            self.p.view.target = DVec3::new(x / 2., y / 2., 0.);
            self.p.view.distance = x.max(y) * 1.75;
        }
        if item(ui, "Auswahl fokussieren") {
            self.focus();
        }
        ui.checkbox(&mut self.p.view.perspective, "Perspektive");
        ui.checkbox(&mut self.wire, "Drahtgitter");
    }
    fn menus(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Datei", |ui| {
                if item(ui, "Neues Projekt") {
                    self.action(Action::New)
                }
                if item(ui, "Projekt öffnen …") {
                    self.action(Action::Open)
                }
                if item(ui, "Speichern") {
                    self.save()
                }
                if item(ui, "Speichern unter …") {
                    self.choose(FileTask::Write(Output::Project))
                }
                ui.separator();
                for (label, f) in [
                    ("STL binär exportieren …", Output::Stl),
                    ("3MF exportieren …", Output::ThreeMf),
                    ("OBJ exportieren …", Output::Obj),
                ] {
                    if item(ui, label) {
                        self.choose(FileTask::Write(f))
                    }
                }
            });
            ui.menu_button("Bearbeiten", |ui| {
                if item(ui, "Rückgängig") {
                    self.undo(false)
                }
                if item(ui, "Wiederholen") {
                    self.undo(true)
                }
                if item(ui, "Duplizieren") {
                    self.duplicate()
                }
                if item(ui, "Löschen") {
                    self.delete()
                }
            });
            ui.menu_button("Ansicht", |ui| self.views(ui));
            ui.menu_button("Erstellen", |ui| {
                for s in Shape::ALL {
                    if item(ui, s.label()) {
                        self.add(s)
                    }
                }
            });
            ui.menu_button("Modellieren", |ui| {
                if item(ui, "Quader mit Durchgangsbohrung") {
                    self.add(Shape::BoredBox)
                }
                if item(ui, "Rohr mit Innenbohrung") {
                    self.add(Shape::Tube)
                }
            });
            ui.menu_button("Werkzeuge", |ui| {
                ui.checkbox(&mut self.snap, "Position am Raster einrasten");
                if item(ui, "Auswahl fokussieren") {
                    self.focus()
                }
            });
            ui.menu_button("Druckvorbereitung", |ui| {
                if item(ui, "Auf Druckbett absenken") {
                    self.place(false)
                }
                if item(ui, "Mittig platzieren") {
                    self.place(true)
                }
                if item(ui, "Druckerprofil laden …") {
                    self.choose(FileTask::Profile)
                }
            });
            ui.menu_button("Einstellungen", |ui| {
                ui.checkbox(&mut self.dark, "Dunkle Oberfläche");
                ui.checkbox(&mut self.shortcut, "Tastenkürzel aktiv");
                for (i, label) in ["Rückgängig", "Wiederholen", "Speichern"]
                    .into_iter()
                    .enumerate()
                {
                    ui.horizontal(|ui| {
                        ui.label(label);
                        egui::ComboBox::from_id_salt(("taste", i))
                            .selected_text(format!("Strg/⌘ + {}", self.keys[i].name()))
                            .show_ui(ui, |ui| {
                                for key in [
                                    egui::Key::Z,
                                    egui::Key::Y,
                                    egui::Key::S,
                                    egui::Key::U,
                                    egui::Key::R,
                                    egui::Key::D,
                                ] {
                                    if (self.keys[i] == key || !self.keys.contains(&key))
                                        && ui
                                            .selectable_label(self.keys[i] == key, key.name())
                                            .clicked()
                                    {
                                        self.keys[i] = key;
                                    }
                                }
                            });
                    });
                }
                ui.add(egui::Slider::new(&mut self.p.view.grid, 1.0..=50.).text("Raster (mm)"));
            });
            ui.menu_button("Hilfe", |ui| {
                if item(ui, "Bedienung und Dateiformate") {
                    self.dialog = Some(Dialog::Help)
                }
            });
        });
    }
    fn properties(&mut self, ui: &mut egui::Ui) {
        ui.heading("Objekte");
        ui.add_space(5.);
        egui::ScrollArea::vertical()
            .id_salt("objects")
            .max_height(145.)
            .show(ui, |ui| {
                for i in 0..self.p.objects.len() {
                    if ui
                        .selectable_label(self.sel == Some(i), &self.p.objects[i].name)
                        .clicked()
                    {
                        self.select(Some(i));
                    }
                }
            });
        ui.separator();
        ui.heading("Eigenschaften");
        let Some(d) = &mut self.draft else {
            ui.label("Körper erstellen oder auswählen.");
            return;
        };
        ui.label("Bezeichnung");
        ui.text_edit_singleline(&mut d.name);
        vector(ui, "Abmessungen · mm", &mut d.size, 0.1..=2000.);
        if d.shape.hollow() {
            ui.horizontal(|ui| {
                ui.label("Bohrung Ø");
                ui.add(egui::DragValue::new(&mut d.bore).speed(0.1).suffix(" mm"));
            });
        }
        vector(ui, "Position · mm", &mut d.position, -10000.0..=10000.);
        vector(ui, "Rotation · Grad", &mut d.rotation, -3600.0..=3600.);
        ui.horizontal(|ui| {
            ui.label("Körperfarbe");
            ui.color_edit_button_srgb(&mut d.color);
        });
        ui.add_space(8.);
        if ui.button("Parameter übernehmen").clicked() {
            let mut obj = d.clone();
            if self.snap {
                let step = self.p.view.grid;
                obj.position = (obj.position / step).round() * step;
            }
            if obj.valid() {
                if let Some(i) = self.sel {
                    self.remember();
                    self.p.objects[i] = obj;
                    self.select(Some(i));
                    self.message = "Parameter übernommen".into();
                }
            } else {
                self.message = "Parameter prüfen · Bohrung muss kleiner als der Körper sein".into();
            }
        }
    }
    pub fn frame(&mut self, ctx: &egui::Context) {
        ctx.set_visuals(if self.dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });
        egui::TopBottomPanel::top("menus").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong("Rust 3D Print Studio");
                ui.separator();
                ui.label(
                    self.path
                        .as_ref()
                        .and_then(|p| p.file_name())
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_else(|| self.p.name.as_str().into()),
                );
                if self.dirty {
                    ui.label("•");
                }
            });
            self.menus(ui);
        });
        renderer::sync(&mut self.cache, &self.p);
        let count: usize = self.cache.iter().map(|s| s.mesh.faces.len()).sum();
        let warnings = self
            .cache
            .iter()
            .filter(|s| self.p.printer.warning(s.bounds.0, s.bounds.1).is_some())
            .count();
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.strong("mm");
                ui.separator();
                ui.label(format!("{count} Dreiecke"));
                if warnings > 0 {
                    ui.colored_label(
                        Color32::from_rgb(231, 151, 86),
                        format!("{warnings} Körper außerhalb des nutzbaren Bereichs"),
                    );
                } else if count > 0 {
                    ui.label("Innerhalb der Profilgrenzen");
                }
                if let Some(i) = self.sel {
                    let v = self.p.objects[i].position;
                    ui.label(format!("X {:.1}   Y {:.1}   Z {:.1}", v.x, v.y, v.z));
                }
                ui.separator();
                ui.label(&self.message);
            });
        });
        egui::SidePanel::left("tools")
            .exact_width(151.)
            .show(ctx, |ui| {
                ui.add_space(12.);
                ui.heading("Körper");
                ui.add_space(7.);
                for s in Shape::ALL {
                    if ui
                        .add_sized([135., 30.], egui::Button::new(s.label()))
                        .clicked()
                    {
                        self.add(s);
                    }
                }
                ui.add_space(12.);
                ui.separator();
                ui.heading("Platzieren");
                if ui.button("Auf Bett absenken").clicked() {
                    self.place(false)
                }
                if ui.button("Mittig platzieren").clicked() {
                    self.place(true)
                }
                ui.add_space(12.);
                ui.separator();
                ui.strong(&self.p.printer.name);
                let pr = &self.p.printer;
                ui.label(format!("{} × {} × {} mm", pr.bed[0], pr.bed[1], pr.print_z));
                ui.small("Orange · Sperrfläche");
            });
        egui::SidePanel::right("inspector")
            .default_width(245.)
            .width_range(220.0..=360.)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("inspector_scroll")
                    .show(ui, |ui| {
                        ui.add_space(12.);
                        self.properties(ui);
                    });
            });
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let (rect, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
                self.rect = rect;
                self.px = ctx.pixels_per_point();
                if self.dialog.is_none() && rect.is_positive() {
                    let v = &mut self.p.view;
                    let delta = response.drag_delta();
                    if response.dragged_by(egui::PointerButton::Secondary) {
                        v.yaw -= f64::from(delta.x) * 0.009;
                        v.pitch = (v.pitch + f64::from(delta.y) * 0.009).clamp(-1.568, 1.568);
                    }
                    if response.dragged_by(egui::PointerButton::Middle) {
                        let (_, right, up) = renderer::basis(v);
                        v.target += (-right * delta.x + up * delta.y).as_dvec3() * v.distance * 0.8
                            / f64::from(rect.height());
                        v.target = v.target.clamp(DVec3::splat(-20000.), DVec3::splat(20000.));
                    }
                    if response.hovered() {
                        let wheel = ctx.input(|i| i.raw_scroll_delta.y);
                        v.distance =
                            (v.distance * (-f64::from(wheel) * 0.0025).exp()).clamp(8., 5000.);
                    }
                    if response.clicked() {
                        if let Some(pos) = response.interact_pointer_pos() {
                            renderer::sync(&mut self.cache, &self.p);
                            let cam = renderer::camera(&self.p.view, rect, self.px);
                            self.select(renderer::pick(
                                &cam,
                                2. * (pos.x - rect.left()) / rect.width() - 1.,
                                1. - 2. * (pos.y - rect.top()) / rect.height(),
                                &self.cache,
                            ));
                        }
                    }
                }
                ui.painter().text(
                    rect.left_top() + egui::vec2(16., 14.),
                    egui::Align2::LEFT_TOP,
                    if self.p.view.perspective {
                        "Perspektive"
                    } else {
                        "Orthografisch"
                    },
                    egui::FontId::proportional(15.),
                    if self.dark {
                        ACCENT
                    } else {
                        Color32::from_rgb(32, 95, 91)
                    },
                );
            });
        if self.shortcut && self.dialog.is_none() && !ctx.wants_keyboard_input() {
            let (z, y, s, del) = ctx.input(|i| {
                (
                    i.modifiers.command && i.key_pressed(self.keys[0]),
                    i.modifiers.command && i.key_pressed(self.keys[1]),
                    i.modifiers.command && i.key_pressed(self.keys[2]),
                    i.key_pressed(egui::Key::Delete),
                )
            });
            if z {
                self.undo(false)
            }
            if y {
                self.undo(true)
            }
            if s {
                self.save()
            }
            if del {
                self.delete()
            }
        }
        self.dialogs(ctx);
    }
    pub fn render(&mut self) {
        if self.rect.is_positive() {
            renderer::sync(&mut self.cache, &self.p);
            let cam = renderer::camera(&self.p.view, self.rect, self.px);
            renderer::draw(&cam, &self.p, &self.cache, self.sel, self.wire, self.dark);
        }
    }
}
fn item(ui: &mut egui::Ui, text: &str) -> bool {
    let hit = ui.button(text).clicked();
    if hit {
        ui.close_menu();
    }
    hit
}
fn vector(ui: &mut egui::Ui, label: &str, v: &mut DVec3, range: std::ops::RangeInclusive<f64>) {
    ui.add_space(7.);
    ui.label(RichText::new(label).strong());
    ui.horizontal_wrapped(|ui| {
        for (axis, value) in [("X", &mut v.x), ("Y", &mut v.y), ("Z", &mut v.z)] {
            ui.add(
                egui::DragValue::new(value)
                    .prefix(axis.to_owned() + " ")
                    .speed(0.25)
                    .range(range.clone()),
            );
        }
    });
}
