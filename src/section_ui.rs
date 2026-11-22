use egui_macroquad::egui::{self, Color32};
use rust_3d_print_studio::{
    project::Project,
    section::{intersect, line_fill, Contour},
    slicer::Preparation,
};
use std::sync::mpsc::{self, Receiver};
type ResultAtHeight = ((f64, u8, bool, bool), Result<Vec<Contour>, String>);
#[derive(Default)]
pub struct SectionView {
    z: f64,
    fill: bool,
    path: String,
    exporting: Option<Receiver<String>>,
    status: String,
    density: u8,
    layer: u32,
    height: f64,
    top: Option<f64>,
    done: Option<ResultAtHeight>,
    job: Option<Receiver<ResultAtHeight>>,
}
impl SectionView {
    pub fn reset(&mut self) {
        self.done = None;
        self.job = None;
        self.top = None;
    }
    pub fn show(&mut self, ui: &mut egui::Ui, p: &Project, profile: Option<&Preparation>) {
        if ui
            .checkbox(&mut self.fill, "Eigene Rust-Schichtplanung")
            .changed()
        {
            self.reset();
        }
        if self.fill {
            if self.height == 0. {
                self.height = 0.2;
                self.density = 15;
                self.layer = 1;
            }
            let old = (self.density, self.height, self.layer);
            ui.horizontal(|ui| {
                ui.label("Schichthöhe");
                ui.add(
                    egui::DragValue::new(&mut self.height)
                        .speed(0.01)
                        .range(0.05..=1.)
                        .suffix(" mm"),
                );
                ui.label("Linienfüllung");
                ui.add(
                    egui::DragValue::new(&mut self.density)
                        .range(0..=100)
                        .suffix(" %"),
                );
            });
            let top = *self.top.get_or_insert_with(|| {
                p.objects
                    .iter()
                    .map(|o| o.mesh().bounds().1.z)
                    .fold(0., f64::max)
            });
            let max = ((top / self.height - 1e-10).ceil() as u32).max(1);
            self.layer = self.layer.clamp(1, max);
            ui.add(egui::Slider::new(&mut self.layer, 1..=max).text("Schicht"));
            self.z = ((f64::from(self.layer) - 1.) * self.height
                + (f64::from(self.layer) * self.height).min(top))
                * 0.5;
            if old != (self.density, self.height, self.layer) {
                self.done = None;
            }
            if self.path.is_empty() && self.status.is_empty() {
                self.path = "Schichtplan.zip".into();
            }
            ui.horizontal(|ui| {
                ui.label("ZIP");
                ui.text_edit_singleline(&mut self.path);
                if ui
                    .add_enabled(
                        self.exporting.is_none(),
                        egui::Button::new("Alle Schichten speichern"),
                    )
                    .clicked()
                {
                    let path = std::path::PathBuf::from(self.path.trim());
                    let p = p.clone();
                    let height = self.height;
                    let density = self.density;
                    let (tx, rx) = mpsc::channel();
                    self.exporting = Some(rx);
                    self.status = "Schichtplan wird geschrieben …".into();
                    std::thread::spawn(move || {
                        let result = rust_3d_print_studio::layers::save(&p, height, density, &path);
                        let _ = tx.send(match result {
                            Ok(n) => format!("{n} SVG-Schichten gespeichert: {}", path.display()),
                            Err(e) => e.to_string(),
                        });
                    });
                }
            });
            if let Some(rx) = &self.exporting {
                match rx.try_recv() {
                    Ok(msg) => {
                        self.status = msg;
                        self.exporting = None
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        self.status = "Schichtexport unterbrochen".into();
                        self.exporting = None
                    }
                    Err(_) => {
                        ui.spinner();
                    }
                }
            }
            if !self.status.is_empty() {
                ui.label(&self.status);
            }
            ui.label(format!(
                "Schnittebene {:.2} mm · geometrischer Füllplan",
                self.z
            ));
        } else {
            ui.add(
                egui::Slider::new(&mut self.z, 0.0..=p.printer.print_z)
                    .text("Schnitthöhe Z")
                    .suffix(" mm"),
            );
        }
        let key = (self.z, self.density, self.fill, self.layer % 2 == 0);
        if let Some(rx) = &self.job {
            match rx.try_recv() {
                Ok(result) => {
                    self.done = Some(result);
                    self.job = None
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.done = Some((key, Err("Schnitt unterbrochen".into())));
                    self.job = None
                }
                Err(_) => (),
            }
        }
        if self.job.is_none() && self.done.as_ref().is_none_or(|(k, _)| *k != key) {
            let (tx, rx) = mpsc::channel();
            self.job = Some(rx);
            let z = self.z;
            let p = p.clone();
            std::thread::spawn(move || {
                let result = intersect(&p, z).and_then(|mut c| {
                    if key.2 && key.1 > 0 {
                        let fill =
                            line_fill(&c, p.printer.nozzle * 100. / f64::from(key.1), key.3)?;
                        c.extend(fill);
                    }
                    Ok(c)
                });
                let _ = tx.send((key, result));
            });
        }
        let [w, h] = p.printer.bed;
        let scale = (300. / w.max(h)) as f32;
        let (r, _) = ui.allocate_exact_size(
            egui::vec2(w as f32 * scale, h as f32 * scale),
            egui::Sense::hover(),
        );
        let painter = ui.painter_at(r);
        painter.rect_filled(r, 3., Color32::from_rgb(23, 33, 46));
        let pos =
            |x: f64, y: f64| r.left_bottom() + egui::vec2(x as f32 * scale, -(y as f32) * scale);
        for i in (0..=w as usize).step_by(20) {
            painter.line_segment(
                [pos(i as f64, 0.), pos(i as f64, h)],
                (0.5, Color32::DARK_GRAY),
            );
        }
        for i in (0..=h as usize).step_by(20) {
            painter.line_segment(
                [pos(0., i as f64), pos(w, i as f64)],
                (0.5, Color32::DARK_GRAY),
            );
        }
        for [x, y, a, b] in p.printer.excluded.iter().copied() {
            painter.rect_filled(
                egui::Rect::from_two_pos(pos(x, y), pos(a, b)),
                0.,
                Color32::from_rgb(99, 64, 56),
            );
        }
        match &self.done {
            Some((z, Ok(contours))) if *z == key => {
                for c in contours {
                    let [r, g, b] = profile
                        .and_then(|v| v.slot_color(c.slot - 1))
                        .unwrap_or(c.color);
                    for [a, b2] in &c.lines {
                        painter.line_segment(
                            [pos(a.x, a.y), pos(b2.x, b2.y)],
                            (1.7, Color32::from_rgb(r, g, b)),
                        );
                    }
                }
                ui.label(format!(
                    "{} geschnittene Objekte · Geometriekonturen in Draufsicht",
                    if self.fill && self.density > 0 {
                        contours.len() / 2
                    } else {
                        contours.len()
                    }
                ));
            }
            Some((z, Err(e))) if *z == key => {
                ui.label(e);
            }
            _ => {
                ui.spinner();
            }
        }
    }
}
