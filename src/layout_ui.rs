use egui_macroquad::egui::{self, Color32};
use glam::DVec2;
use rust_3d_print_studio::{
    layout::{Mode, Options, Plan, Scene},
    project::Project,
};
pub struct Editor {
    pub s: Options,
    scene: Scene,
    cache: Option<(Options, Plan)>,
}
impl Editor {
    pub fn new(p: &Project, ids: &[usize], delta: DVec2) -> Self {
        Self {
            s: Options {
                reference: ids.len().saturating_sub(1),
                step: delta,
                ..Default::default()
            },
            scene: Scene::new(&p.objects, ids),
            cache: None,
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui, p: &Project) -> Option<Plan> {
        let s = &mut self.s;
        ui.heading("Plattenwerkzeuge");
        ui.horizontal_wrapped(|ui| {
            for (mode, text) in [
                (Mode::Grid, "Rechteck"),
                (Mode::Circle, "Kreis"),
                (Mode::Copy, "Kopierschritt"),
                (Mode::Move, "Versatz"),
                (Mode::Align, "Ausrichten"),
                (Mode::Distribute, "Verteilen"),
            ] {
                if ui.selectable_value(&mut s.mode, mode, text).changed() {
                    s.skipped.clear();
                }
            }
        });
        match s.mode {
            Mode::Grid => {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut s.cols)
                            .range(1..=128)
                            .prefix("Spalten "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut s.rows)
                            .range(1..=128)
                            .prefix("Reihen "),
                    );
                    mm(ui, "Abstand", &mut s.gap, 0.);
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut s.reverse[0], "Nach links");
                    ui.checkbox(&mut s.reverse[1], "Nach vorne (−Y)");
                });
            }
            Mode::Circle => {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut s.count)
                            .range(2..=128)
                            .prefix("Anzahl "),
                    );
                    mm(ui, "Radius", &mut s.radius, 0.1);
                    ui.checkbox(&mut s.turn, "Mitdrehen");
                });
                ui.small("Kreismitte liegt um den Radius links von der Auswahl.");
            }
            Mode::Move | Mode::Copy => {
                ui.horizontal(|ui| {
                    mm(ui, "ΔX", &mut s.step.x, -2000.);
                    mm(ui, "ΔY", &mut s.step.y, -2000.);
                    if s.mode == Mode::Copy {
                        ui.add(
                            egui::DragValue::new(&mut s.count)
                                .range(1..=127)
                                .prefix("Kopien "),
                        );
                    }
                });
            }
            _ => {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut s.axis, 0, "X-Achse");
                    ui.selectable_value(&mut s.axis, 1, "Y-Achse");
                    if s.mode == Mode::Distribute {
                        mm(ui, "Kantenabstand", &mut s.gap, 0.);
                    }
                });
                if s.mode == Mode::Align {
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_salt("ref")
                            .selected_text(&p.objects[self.scene.ids[s.reference]].name)
                            .show_ui(ui, |ui| {
                                for (k, &i) in self.scene.ids.iter().enumerate() {
                                    ui.selectable_value(&mut s.reference, k, &p.objects[i].name);
                                }
                            });
                        for (i, t) in ["Minimum", "Mitte", "Maximum"].iter().enumerate() {
                            ui.selectable_value(&mut s.edge, i, *t);
                        }
                    });
                }
            }
        }
        if self.cache.as_ref().is_none_or(|(last, _)| last != s) {
            self.cache = Some((s.clone(), self.scene.plan(p, s)));
        }
        let plan = &self.cache.as_ref()?.1;
        let bed = DVec2::from_array(p.printer.bed);
        let mut lo = DVec2::ZERO;
        let mut hi = bed;
        for i in &plan.items {
            lo = lo.min(i.lo.truncate());
            hi = hi.max(i.hi.truncate());
        }
        let (canvas, r) = ui.allocate_exact_size(egui::vec2(490., 260.), egui::Sense::click());
        let scale = (470. / (hi.x - lo.x)).min(245. / (hi.y - lo.y)) as f32;
        let mid = (hi + lo) * 0.5;
        let pos = |v: DVec2| {
            canvas.center() + egui::vec2((v.x - mid.x) as f32 * scale, (mid.y - v.y) as f32 * scale)
        };
        let rect = |a: DVec2, b: DVec2| egui::Rect::from_two_pos(pos(a), pos(b));
        let g = ui.painter_at(canvas);
        g.rect_filled(canvas, 3., Color32::from_rgb(20, 28, 39));
        g.rect_filled(rect(DVec2::ZERO, bed), 0., Color32::from_rgb(58, 72, 86));
        for &[a, b, c, d] in &p.printer.excluded {
            g.rect_filled(
                rect(DVec2::new(a, b), DVec2::new(c, d)),
                0.,
                Color32::from_rgb(151, 79, 65),
            );
        }
        for &(a, b) in &self.scene.bounds {
            g.rect_stroke(
                rect(a.truncate(), b.truncate()),
                0.,
                (1., Color32::WHITE),
                egui::StrokeKind::Inside,
            );
        }
        let mut click = None;
        for item in &plan.items {
            let box2 = rect(item.lo.truncate(), item.hi.truncate());
            let color = if item.skip {
                Color32::GRAY
            } else if item.issue.is_some() {
                Color32::LIGHT_RED
            } else {
                Color32::from_rgb(103, 220, 173)
            };
            if !item.skip {
                g.rect_filled(box2, 0., color.gamma_multiply(0.5));
            }
            g.rect_stroke(box2, 0., (1., color), egui::StrokeKind::Inside);
            if plan.copy && box2.width() > 14. {
                g.text(
                    box2.center(),
                    egui::Align2::CENTER_CENTER,
                    (item.cell + 1).to_string(),
                    egui::FontId::proportional(10.),
                    Color32::WHITE,
                );
            }
            if plan.copy
                && r.clicked()
                && r.interact_pointer_pos().is_some_and(|q| box2.contains(q))
            {
                click = Some(item.cell);
            }
        }
        if let Some(k) = click {
            if !s.skipped.remove(&k) {
                s.skipped.insert(k);
            }
        }
        if plan.copy {
            ui.small("Feld anklicken zum Auslassen oder Zurückholen. Rot markiert Konflikte.");
        }
        let msg = plan
            .error
            .clone()
            .or_else(|| {
                plan.items.iter().find_map(|i| {
                    i.issue
                        .as_ref()
                        .map(|m| format!("Feld {} · {m}", i.cell + 1))
                })
            })
            .unwrap_or_else(|| {
                format!(
                    "{} Körper {}",
                    plan.items.iter().filter(|i| !i.skip).count(),
                    if plan.copy {
                        "hinzufügen"
                    } else {
                        "verschieben"
                    }
                )
            });
        ui.label(msg);
        if ui
            .add_enabled(
                plan.allowed() && click.is_none(),
                egui::Button::new("Übernehmen"),
            )
            .clicked()
        {
            return self.cache.take().map(|c| c.1);
        }
        None
    }
}
fn mm(ui: &mut egui::Ui, label: &str, v: &mut f64, lower: f64) {
    ui.label(label);
    ui.add(
        egui::DragValue::new(v)
            .range(lower..=2000.)
            .speed(0.25)
            .suffix(" mm"),
    );
}
