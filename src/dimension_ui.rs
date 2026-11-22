//! Bemaßungsansicht im lokalen Koordinatensystem der Skizze.
use egui_macroquad::egui::{self, Color32, Pos2, Ui};
use glam::DVec3;
use rust_3d_print_studio::cad::sketch::{RuleKind, Sketch};
use std::sync::Arc;

#[derive(Clone)]
struct Preview {
    source: Sketch,
    points: Vec<DVec3>,
    contour: Vec<DVec3>,
    error: Option<String>,
}
#[derive(Default, Clone)]
struct Selection {
    first: Option<usize>,
    second: Option<usize>,
    third: Option<usize>,
    angle: bool,
    value: f64,
    plane: usize,
    message: String,
}
impl Selection {
    fn pair(&mut self, a: usize, b: usize, points: &[DVec3]) {
        self.third = None;
        self.first = Some(a);
        self.second = Some(b);
        self.value = points[a].distance(points[b]);
        self.message.clear();
    }
}

pub fn show(ui: &mut Ui, s: &mut Sketch) {
    let id = ui.id().with("grafische_masse");
    let mut selection = ui.data(|d| d.get_temp::<Selection>(id)).unwrap_or_default();
    let cache = id.with("vorschau");
    let preview = match ui.data(|d| d.get_temp::<Arc<Preview>>(cache)) {
        Some(old) if old.source == *s => old,
        _ => {
            let (points, contour, error) = match s.solved_points() {
                Ok(points) => match s.contour_from_points(&points, 24) {
                    Ok(contour) => (points, contour, None),
                    Err(e) => (points, Vec::new(), Some(e)),
                },
                Err(e) => (Vec::new(), Vec::new(), Some(e)),
            };
            let new = Arc::new(Preview {
                source: s.clone(),
                points,
                contour,
                error,
            });
            selection.first = None;
            selection.second = None;
            selection.third = None;
            ui.data_mut(|d| d.insert_temp(cache, new.clone()));
            new
        }
    };
    if let Some(error) = &preview.error {
        ui.label(error);
    }
    if preview.points.is_empty() {
        return;
    }
    ui.horizontal(|ui| {
        let changed = ui
            .selectable_value(&mut selection.angle, false, "Abstand")
            .changed();
        if ui
            .selectable_value(&mut selection.angle, true, "Innenwinkel")
            .changed()
            || changed
        {
            selection.first = None;
            selection.second = None;
            selection.third = None;
            selection.message.clear();
        }
    });
    ui.small(if selection.angle {
        "Endpunkt, Scheitel und zweiten Endpunkt anklicken."
    } else {
        "Zwei Punkte oder ein vorhandenes Maß anklicken."
    });
    if !s.planar {
        ui.horizontal(|ui| {
            for (i, text) in ["XY", "XZ", "YZ"].iter().enumerate() {
                ui.selectable_value(&mut selection.plane, i, *text);
            }
        });
    }
    let (x, y) = [(0, 1), (0, 2), (1, 2)][if s.planar { 0 } else { selection.plane }];
    let size = egui::vec2(ui.available_width().clamp(80., 500.), 180.);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 4., Color32::from_rgb(24, 33, 45));
    let mut low = DVec3::splat(f64::INFINITY);
    let mut high = -low;
    for &p in preview.points.iter().chain(&preview.contour) {
        low = low.min(p);
        high = high.max(p);
    }
    let mid = (low + high) * 0.5;
    let span = high - low;
    let scale =
        ((size.x - 66.) as f64 / span[x].max(1.)).min((size.y - 64.) as f64 / span[y].max(1.));
    let pixel = |p: DVec3| {
        rect.center()
            + egui::vec2(
                ((p[x] - mid[x]) * scale) as f32,
                ((mid[y] - p[y]) * scale) as f32,
            )
    };
    for pair in preview.contour.windows(2) {
        painter.line_segment([pixel(pair[0]), pixel(pair[1])], (1.5, Color32::LIGHT_BLUE));
    }
    let click = response
        .interact_pointer_pos()
        .filter(|_| response.clicked());
    let mut label_hit = None;
    for r in s.rules.iter().filter(|r| r.kind == RuleKind::Distance) {
        let [a, b, _, _] = r.ids;
        let (pa, pb) = (pixel(preview.points[a]), pixel(preview.points[b]));
        let ray = pb - pa;
        let u = ray.normalized();
        let mut n = egui::vec2(-u.y, u.x);
        if n.dot(pa - rect.center()) < 0. {
            n = -n;
        }
        let offset = n * 15.;
        let (start, end) = (pa + offset, pb + offset);
        let color = Color32::from_rgb(246, 204, 104);
        for (p, q) in [(pa, start), (pb, end), (start, end)] {
            painter.line_segment([p, q], (1., color));
        }
        if ray.length() > 8. {
            painter.arrow(start + u * 6., -u * 6., (1., color));
            painter.arrow(end - u * 6., u * 6., (1., color));
        }
        let text = painter.text(
            start.lerp(end, 0.5) + n * 7.,
            egui::Align2::CENTER_CENTER,
            format!("{:.2}", r.value),
            egui::FontId::proportional(11.),
            color,
        );
        if click.is_some_and(|p| text.expand(2.).contains(p)) {
            label_hit = Some((a, b, None, r.value));
        }
    }
    for r in s.rules.iter().filter(|r| r.kind == RuleKind::Angle) {
        let [a, b, c, _] = r.ids;
        let [pa, pb, pc] = [a, b, c].map(|i| pixel(preview.points[i]));
        let u = (pa - pb).normalized();
        let v = (pc - pb).normalized();
        let color = Color32::LIGHT_GREEN;
        let radius = 0.4 * pa.distance(pb).min(pc.distance(pb)).min(50.);
        painter.line_segment([pb + u * radius, pb], (1.5, color));
        painter.line_segment([pb, pb + v * radius], (1.5, color));
        let label = painter.text(
            pb + (u + v).normalized() * (radius + 8.),
            egui::Align2::CENTER_CENTER,
            format!("{:.2}°", r.value),
            egui::FontId::proportional(11.),
            color,
        );
        if click.is_some_and(|p| label.contains(p)) {
            label_hit = Some((a, b, Some(c), r.value));
        }
    }
    for (i, &p) in preview.points.iter().enumerate() {
        let color = if [selection.first, selection.second, selection.third].contains(&Some(i)) {
            Color32::GOLD
        } else {
            Color32::WHITE
        };
        painter.circle_filled(pixel(p), 4., color);
    }
    if let Some((a, b, c, value)) = label_hit {
        selection.pair(a, b, &preview.points);
        selection.angle = c.is_some();
        selection.third = c;
        selection.value = value;
    } else if let Some(pos) = click {
        if let Some(i) = nearest(&preview.points, pixel, pos) {
            if let (true, Some(a), Some(b), None) = (
                selection.angle,
                selection.first,
                selection.second,
                selection.third,
            ) {
                if i != a && i != b {
                    selection.third = Some(i);
                    let u = preview.points[a] - preview.points[b];
                    let v = preview.points[i] - preview.points[b];
                    selection.value = u.cross(v).length().atan2(u.dot(v)).to_degrees();
                }
            } else if let (Some(a), None) = (selection.first, selection.second) {
                if a != i {
                    selection.pair(a, i, &preview.points);
                }
            } else {
                selection.third = None;
                selection.first = Some(i);
                selection.second = None;
                selection.message.clear();
            }
        }
    }
    if let (Some(a), Some(b)) = (selection.first, selection.second) {
        if selection.angle && selection.third.is_none() {
            ui.small(format!(
                "Scheitel P{b} gewählt · zweiten Endpunkt anklicken"
            ));
        } else {
            ui.label(match selection.third {
                Some(c) => format!("Innenwinkel P{a} – P{b} – P{c}"),
                None => format!("Abstand P{a} – P{b}"),
            });
            let suffix = if selection.angle { " °" } else { " mm" };
            ui.add(
                egui::DragValue::new(&mut selection.value)
                    .speed(0.1)
                    .suffix(suffix)
                    .max_decimals(4),
            );
            if ui.button("Maß setzen").clicked() {
                let result = match selection.third {
                    Some(c) => s.set_angle(a, b, c, selection.value),
                    None => s.set_distance(a, b, selection.value),
                };
                selection.message = match result {
                    Ok(()) => "Maß im Entwurf gesetzt. Rechts Parameter übernehmen.".into(),
                    Err(e) => e,
                };
            }
        }
    } else if let Some(a) = selection.first {
        ui.small(format!("P{a} gewählt · zweiten Punkt anklicken"));
    }
    if !selection.message.is_empty() {
        ui.label(&selection.message);
    }
    ui.data_mut(|d| d.insert_temp(id, selection));
}
fn nearest(points: &[DVec3], pixel: impl Fn(DVec3) -> Pos2, pos: Pos2) -> Option<usize> {
    points
        .iter()
        .enumerate()
        .map(|(i, &p)| (i, pixel(p).distance(pos)))
        .filter(|(_, d)| *d <= 9.)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(i, _)| i)
}
