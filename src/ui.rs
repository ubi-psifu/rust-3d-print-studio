use crate::{feature_ui, help, renderer};
use egui_macroquad::egui::{self, Color32, RichText};
use glam::{DVec2, DVec3};
use rust_3d_print_studio::cad::{inspect, validate, Feature, Operation};
use rust_3d_print_studio::plate;
use rust_3d_print_studio::{
    files::{self, Output},
    geometry::{Object, Shape},
    project::{History, Printer, Project, View, MAX_OBJECTS},
};
use std::path::{Path, PathBuf};
// Auch ein innerhalb desselben Frames losgelassenes Strg gehört zum Tastendruck.
fn command_pressed(events: &[egui::Event], shortcut: egui::Key) -> bool {
    events.iter().any(|event| {
        matches!(event,
        egui::Event::Key { key, pressed: true, modifiers, .. }
            if *key == shortcut && modifiers.command)
    })
}

#[derive(Clone, Copy)]
enum Action {
    Recover(usize),
    New,
    Open,
    Quit,
}
#[derive(Clone, Copy)]
enum FileTask {
    Import,
    Svg,
    Open,
    Profile,
    Write(Output),
}
#[derive(Clone, Copy)]
enum Dialog {
    Recovery,
    File(FileTask),
    Overwrite(Output),
    Discard(Action),
    Combine,
    Prepare,
    Layout,
    Case,
}
struct MoveDrag {
    axis: Option<usize>,
    hit: DVec3,
    starts: Vec<(usize, DVec3)>,
    lo: DVec2,
    hi: DVec2,
    recorded: bool,
}
struct GeometryPicker {
    edges: Option<Vec<(usize, usize)>>,
    solid: renderer::Solid,
    view: View,
    hint: String,
}
impl GeometryPicker {
    fn new(object: &Object, view: &View) -> Result<Self, String> {
        let (source, picking_edge) = match &object.feature {
            Some(Feature::Shell { source, .. }) => (source, false),
            Some(Feature::EdgeFinish { source, .. }) => (source, true),
            _ => return Err("Zum Auswählen eine Aushöhlung oder Kantenbearbeitung öffnen.".into()),
        };
        let mut outer = object.clone();
        outer.feature = Some((**source).clone());
        outer.rebuild()?;
        let edges = picking_edge.then(|| {
            rust_3d_print_studio::cad::references::EdgeRef::candidates(&outer.local_mesh())
        });
        let solid = renderer::Solid::new(&outer);
        let (lo, hi) = solid.bounds;
        let mut view = view.clone();
        view.target = (lo + hi) * 0.5;
        view.distance = ((hi - lo).length() * 2.).clamp(8., 5000.);
        Ok(Self {
            edges,
            solid,
            view,
            hint: if picking_edge {
                "Klicke von der sichtbaren Fläche aus auf eine scharfe Kante."
            } else {
                "Klicke auf die gewünschte Fläche des Außenkörpers."
            }
            .into(),
        })
    }
}
struct EditPreview {
    section: Option<(renderer::Solid, f64)>,
    show_section: bool,
    index: usize,
    solid: renderer::Solid,
    view: View,
    original: bool,
}
enum CalculatedEdit {
    Apply(Box<Object>),
    Preview(Box<EditPreview>),
}
pub struct Studio {
    help: help::Help,
    hover_help: Option<&'static str>,
    backups: rust_3d_print_studio::recovery::Worker,
    checkpoint: std::time::Instant,
    copies: Vec<(std::time::SystemTime, PathBuf)>,
    layout: Option<crate::layout_ui::Editor>,
    axis: Option<usize>,
    step: DVec2,
    case: rust_3d_print_studio::cad::accessories::Case,
    job: Option<std::sync::mpsc::Receiver<Result<Vec<Object>, String>>>,
    unit: f64,
    auto_place: bool,
    measure: Vec<DVec3>,
    magnet: bool,
    plane_pick: bool,
    geometry_picker: Option<GeometryPicker>,
    preparing_picker: Option<std::sync::mpsc::Receiver<Result<GeometryPicker, String>>>,
    preparation: crate::preparation_ui::Panel,
    selected: Vec<usize>,
    clipboard: Vec<Object>,
    drag: Option<MoveDrag>,
    marquee: Option<(egui::Pos2, Vec<usize>)>,
    gap: f64,
    p: Project,
    tool: usize,
    preview: Option<EditPreview>,
    section_percent: u8,
    pending: Option<(
        usize,
        std::sync::mpsc::Receiver<Result<CalculatedEdit, String>>,
    )>,
    operand: usize,
    operation: Operation,
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
        let copies = rust_3d_print_studio::recovery::directory()
            .and_then(|d| rust_3d_print_studio::recovery::list(&d))
            .unwrap_or_default();
        let offer_recovery = path.is_none() && !copies.is_empty();
        let mut app = Self {
            help: Default::default(),
            hover_help: None,
            backups: Default::default(),
            checkpoint: std::time::Instant::now(),
            copies,
            layout: None,
            axis: None,
            step: DVec2::new(24., 0.),
            case: Default::default(),
            job: None,
            unit: 1.,
            auto_place: true,
            measure: vec![],
            magnet: true,
            plane_pick: false,
            geometry_picker: None,
            preparing_picker: None,
            preparation: Default::default(),
            p: Project::default(),
            tool: 0,
            selected: vec![],
            clipboard: vec![],
            drag: None,
            marquee: None,
            gap: 3.,
            pending: None,
            preview: None,
            section_percent: 50,
            operand: 0,
            operation: Operation::Difference,
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
        } else {
            let mut o = Object::new(Shape::BoredBox, DVec3::new(128., 128., 0.));
            o.size = DVec3::new(80., 80., 40.);
            o.bore = 28.;
            o.name = "Bauteil 01".into();
            o.color = [201, 207, 218];
            app.p.objects.push(o);
            app.select(Some(0));
            app.p.view.distance = 360.;
        }
        if offer_recovery {
            app.dialog = Some(Dialog::Recovery);
        }
        app
    }
    fn select(&mut self, i: Option<usize>) {
        self.sel = i.filter(|i| *i < self.p.objects.len());
        self.selected = self.sel.into_iter().collect();
        self.refresh_draft();
    }
    fn refresh_draft(&mut self) {
        self.preview = None;
        self.pending = None;
        self.geometry_picker = None;
        self.preparing_picker = None;
        self.draft = self.sel.map(|i| self.p.objects[i].clone());
    }
    fn toggle_selection(&mut self, i: Option<usize>, extend: bool) {
        if !extend {
            self.select(i);
            return;
        }
        let Some(i) = i else { return };
        if self.selected.contains(&i) {
            self.selected.retain(|j| *j != i)
        } else {
            self.selected.push(i)
        }
        self.sel = self.selected.last().copied();
        self.refresh_draft();
    }
    fn select_all(&mut self) {
        self.selected = (0..self.p.objects.len()).collect();
        self.sel = self.selected.last().copied();
        self.refresh_draft();
    }
    fn remember(&mut self) {
        self.measure.clear();
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
    fn feature_object(&mut self, f: Feature) {
        let [x, y] = self.p.printer.bed;
        self.feature_at(f, DVec3::new(x / 2., y / 2., 0.));
    }
    fn feature_at(&mut self, f: Feature, at: DVec3) {
        if self.p.objects.len() >= MAX_OBJECTS {
            self.message = "Objektgrenze erreicht".into();
            return;
        }
        let mut o = Object::new(Shape::Box, at);
        o.name = f.name().into();
        o.color = [196, 207, 218];
        o.feature = Some(f);
        match o.rebuild() {
            Ok(()) => {
                self.remember();
                self.p.objects.push(o);
                self.select(Some(self.p.objects.len() - 1));
                self.focus();
                self.message = "Konstruktion erstellt".into();
            }
            Err(e) => self.message = e,
        }
    }
    fn source(o: &Object) -> Feature {
        o.feature.clone().unwrap_or(Feature::Primitive {
            shape: o.shape,
            size: o.size,
            bore: o.bore,
        })
    }
    fn combine(&mut self) {
        let Some(i) = self.sel else { return };
        let j = self.operand;
        if i == j || j >= self.p.objects.len() {
            self.message = "Zwei verschiedene Körper auswählen".into();
            return;
        }
        let origin = self.p.objects[i].position;
        let positioned = |o: &Object| Feature::Transform {
            source: Box::new(Self::source(o)),
            translation: o.position - origin,
            rotation: o.rotation,
            scale: DVec3::ONE,
        };
        let f = Feature::Boolean {
            operation: self.operation,
            left: Box::new(positioned(&self.p.objects[i])),
            right: Box::new(positioned(&self.p.objects[j])),
        };
        let mut o = self.p.objects[i].clone();
        o.rotation = DVec3::ZERO;
        o.feature = Some(f);
        match o.rebuild() {
            Ok(()) => {
                self.remember();
                self.p.objects[i] = o;
                self.p.objects.remove(j);
                self.select(Some(i - usize::from(j < i)));
                self.dialog = None;
                self.message = "Körperschnitt übernommen".into();
            }
            Err(e) => self.message = e,
        }
    }
    fn check_model(&mut self) {
        if let Some(i) = self.sel {
            let mesh = self.p.objects[i].mesh();
            self.message = match validate(&mesh).and_then(|_| inspect::intersections(&mesh)) {
                Ok(()) => {
                    let (a, b) = mesh.bounds();
                    let mut overhang = 0.;
                    for f in &mesh.faces {
                        let [p, q, r] = f.map(|j| mesh.vertices[j]);
                        let n = (q - p).cross(r - p);
                        if n.normalize_or_zero().z < -std::f64::consts::FRAC_1_SQRT_2
                            && p.z.max(q.z).max(r.z) > a.z + 0.01
                        {
                            overhang += n.length() / 2.;
                        }
                    }
                    format!("{} · geschlossen · Selbstschnittprüfung bestanden · Überhang >45°: {:.1} mm²",self.p.printer.warning(a,b).unwrap_or("Profilgrenzen eingehalten"),overhang)
                }
                Err(e) => e,
            };
        } else {
            self.message = "Zuerst einen Körper auswählen".into();
        }
    }
    fn selection_copy(&self) -> Vec<Object> {
        self.selected
            .iter()
            .map(|i| self.p.objects[*i].clone())
            .collect()
    }
    fn copy(&mut self) {
        if !self.selected.is_empty() {
            self.clipboard = self.selection_copy();
            self.message = format!("{} Körper kopiert", self.clipboard.len());
        }
    }
    fn duplicate(&mut self) {
        self.insert_copies(self.selection_copy(), false);
    }
    fn insert_copies(&mut self, group: Vec<Object>, fill: bool) {
        if group.is_empty() {
            self.message = "Keine Körper zum Einfügen".into();
            return;
        }
        let capacity = (MAX_OBJECTS - self.p.objects.len()) / group.len();
        if capacity == 0 {
            self.message = "Objektgrenze erreicht".into();
            return;
        }
        let offsets = plate::copy_offsets(
            &self.p.printer,
            &self.p.objects,
            &group,
            self.gap,
            if fill { capacity } else { 1 },
        );
        if offsets.is_empty() {
            self.message = "Kein freier Platz für die Auswahl mit diesem Abstand".into();
            return;
        }
        self.remember();
        let first = self.p.objects.len();
        for offset in offsets {
            for src in &group {
                let mut obj = src.clone();
                obj.position += offset.extend(0.);
                let suffix = format!(" · {}", self.p.objects.len() + 1);
                obj.name = obj
                    .name
                    .chars()
                    .take(120 - suffix.chars().count())
                    .collect::<String>()
                    + &suffix;
                self.p.objects.push(obj);
            }
        }
        self.selected = (first..self.p.objects.len()).collect();
        self.sel = self.selected.last().copied();
        self.refresh_draft();
        self.message = format!("{} Kopien platziert", self.selected.len());
    }
    fn arrange_plate(&mut self) {
        if self.p.objects.is_empty() {
            return;
        }
        let Some(offsets) = plate::arrange(&self.p.printer, &self.p.objects, self.gap) else {
            self.message = "Anordnung passt mit diesem Abstand nicht auf die Platte".into();
            return;
        };
        if offsets.iter().any(|d| d.length_squared() > 1e-12) {
            self.remember();
            for (o, delta) in self.p.objects.iter_mut().zip(offsets) {
                o.position += delta;
            }
            self.refresh_draft();
        }
        self.top_view();
        self.message = "Alle Körper auf dem Druckbett angeordnet".into();
    }
    fn top_view(&mut self) {
        let [w, h] = self.p.printer.bed;
        self.p.view.target = DVec3::new(w / 2., h / 2., 0.);
        self.p.view.pitch = std::f64::consts::FRAC_PI_2;
        self.p.view.yaw = -std::f64::consts::FRAC_PI_2;
        self.p.view.perspective = false;
        let aspect = if self.rect.is_positive() {
            f64::from(self.rect.aspect_ratio())
        } else {
            1.
        };
        self.p.view.distance = (h.max(w / aspect) * 1.4).clamp(8., 5000.);
    }
    fn delete(&mut self) {
        if self.selected.is_empty() {
            return;
        }
        self.remember();
        self.selected.sort_unstable();
        for &i in self.selected.iter().rev() {
            self.p.objects.remove(i);
        }
        self.select(None);
    }
    fn plate_pointer(&mut self, response: &egui::Response, rect: egui::Rect, ui: &egui::Ui) {
        let ctx = ui.ctx();
        let ndc = |p: egui::Pos2| {
            (
                2. * (p.x - rect.left()) / rect.width() - 1.,
                1. - 2. * (p.y - rect.top()) / rect.height(),
            )
        };
        let cam = renderer::camera(&self.p.view, rect, self.px);
        let matrix = macroquad::camera::Camera::matrix(&cam);
        let project = |p: DVec3| {
            let q = matrix * p.as_vec3().extend(1.);
            let q = q.truncate() / q.w;
            egui::pos2(
                rect.center().x + q.x * rect.width() * 0.5,
                rect.center().y - q.y * rect.height() * 0.5,
            )
        };
        if self.plane_pick || self.tool == 7 {
            if response.clicked_by(egui::PointerButton::Primary) {
                if let Some(pos) = response.interact_pointer_pos() {
                    let (x, y) = ndc(pos);
                    if let Some((i, face, hit)) = renderer::surface(&cam, x, y, &self.cache) {
                        let mesh = &self.cache[i].mesh;
                        let t = mesh.faces[face].map(|j| mesh.vertices[j]);
                        let mut at = hit.as_dvec3();
                        if self.magnet {
                            let mut best = 10.;
                            for q in [
                                t[0],
                                t[1],
                                t[2],
                                (t[0] + t[1]) * 0.5,
                                (t[1] + t[2]) * 0.5,
                                (t[2] + t[0]) * 0.5,
                            ] {
                                let d = project(q).distance(pos);
                                if d < best {
                                    at = q;
                                    best = d
                                }
                            }
                        }
                        if self.plane_pick {
                            let u = (t[1] - t[0]).normalize();
                            let n = u.cross(t[2] - t[0]).normalize();
                            let sketch = rust_3d_print_studio::cad::sketch::Sketch {
                                u,
                                v: n.cross(u),
                                ..Default::default()
                            };
                            self.plane_pick = false;
                            self.feature_at(
                                Feature::SketchExtrude {
                                    sketch,
                                    height: 4.,
                                    end_scale: 1.,
                                    steps: 16,
                                },
                                at,
                            );
                        } else {
                            if self.measure.len() == 2 {
                                self.measure.clear();
                            }
                            self.measure.push(at);
                        }
                    }
                }
            }
            if self.tool == 7 {
                for &p in &self.measure {
                    ui.painter().circle_filled(project(p), 4., ACCENT);
                }
                if let [a, b] = self.measure.as_slice() {
                    ui.painter()
                        .line_segment([project(*a), project(*b)], (2., ACCENT));
                }
            }
            return;
        }
        let mut handle = None;
        if self.tool == 0 {
            if let Some(i) = self.sel {
                let (a, b) = self.cache[i].bounds;
                let anchor = (a + b) * 0.5;
                let base = project(anchor);
                for (axis, v, col, label) in [
                    (0, DVec3::X, Color32::from_rgb(240, 143, 100), "X"),
                    (1, DVec3::Y, Color32::from_rgb(131, 222, 178), "Y"),
                ] {
                    let units = project(anchor + v) - base;
                    if units.length() < 0.05 {
                        continue;
                    }
                    let dir = units.normalized();
                    let tip = base + dir * 48.;
                    ui.painter().arrow(base + dir * 10., dir * 38., (2., col));
                    ui.painter().text(
                        tip + dir * 8.,
                        egui::Align2::CENTER_CENTER,
                        label,
                        egui::FontId::proportional(12.),
                        col,
                    );
                    if ctx
                        .input(|s| s.pointer.press_origin())
                        .is_some_and(|p| p.distance(tip) < 10.)
                    {
                        handle = Some((i, axis, anchor));
                    }
                }
            }
        }
        let multi = ctx.input(|i| i.modifiers.shift || i.modifiers.command);
        if response.drag_started_by(egui::PointerButton::Primary) {
            if let Some(start) = ctx.input(|i| i.pointer.press_origin()) {
                let (x, y) = ndc(start);
                let hit = handle
                    .and_then(|(i, _, a)| {
                        renderer::xy_at(&cam, x, y, a.z as f32)
                            .map(|v| (i, v.extend(a.z).as_vec3()))
                    })
                    .or_else(|| renderer::pick(&cam, x, y, &self.cache));
                if let Some((i, hit)) = hit {
                    if !self.selected.contains(&i) {
                        self.toggle_selection(Some(i), multi);
                    }
                    let mut lo = DVec2::splat(f64::INFINITY);
                    let mut hi = -lo;
                    let starts = self
                        .selected
                        .iter()
                        .map(|&i| {
                            let (a, b) = self.cache[i].bounds;
                            lo = lo.min(a.truncate());
                            hi = hi.max(b.truncate());
                            (i, self.p.objects[i].position)
                        })
                        .collect();
                    self.drag = Some(MoveDrag {
                        axis: handle.map(|(_, axis, _)| axis).or(self.axis),
                        hit: hit.as_dvec3(),
                        starts,
                        lo,
                        hi,
                        recorded: false,
                    });
                } else {
                    self.marquee =
                        Some((start, if multi { self.selected.clone() } else { vec![] }));
                }
            }
        }
        if let Some((start, base)) = self.marquee.take() {
            let pos = ctx.input(|i| i.pointer.latest_pos()).unwrap_or(start);
            let area = egui::Rect::from_two_pos(start, pos).intersect(rect);
            ui.painter()
                .rect_filled(area, 0., Color32::from_rgba_unmultiplied(80, 180, 160, 32));
            ui.painter().rect_stroke(
                area,
                0.,
                egui::Stroke::new(1., ACCENT),
                egui::StrokeKind::Inside,
            );
            let mut ids = base.clone();
            let matrix = macroquad::camera::Camera::matrix(&cam);
            for (i, solid) in self.cache.iter().enumerate() {
                let (a, b) = solid.bounds;
                let clip = matrix * ((a + b) * 0.5).as_vec3().extend(1.);
                if clip.w <= 0. {
                    continue;
                }
                let p = clip.truncate() / clip.w;
                let screen = egui::pos2(
                    rect.center().x + p.x * rect.width() * 0.5,
                    rect.center().y - p.y * rect.height() * 0.5,
                );
                if area.contains(screen) && !ids.contains(&i) {
                    ids.push(i);
                }
            }
            if ids != self.selected {
                self.selected = ids;
                self.sel = self.selected.last().copied();
                self.refresh_draft();
            }
            if ctx.input(|i| i.pointer.primary_down()) {
                self.marquee = Some((start, base));
            }
            return;
        }
        if let Some(mut drag) = self.drag.take() {
            if let Some(pos) = response.interact_pointer_pos() {
                let (x, y) = ndc(pos);
                if let Some(at) = renderer::xy_at(&cam, x, y, drag.hit.z as f32) {
                    let mut d = at - drag.hit.truncate();
                    if self.snap {
                        let anchor = drag.starts[0].1.truncate();
                        let step = self.p.view.grid;
                        d = ((anchor + d) / step).round() * step - anchor;
                    }
                    let bed = DVec2::from_array(self.p.printer.bed);
                    for axis in 0..2 {
                        let low = -drag.lo[axis];
                        let high = bed[axis] - drag.hi[axis];
                        if low <= high {
                            d[axis] = d[axis].clamp(low, high);
                        }
                    }
                    if let Some(axis) = drag.axis {
                        d[1 - axis] = 0.;
                    }
                    if d.length_squared() > 1e-8 {
                        self.step = d;
                    }
                    if d.length_squared() > 1e-8 || drag.recorded {
                        if !drag.recorded {
                            self.remember();
                            drag.recorded = true;
                        }
                        for &(i, start) in &drag.starts {
                            self.p.objects[i].position = start + d.extend(0.);
                        }
                        if let (Some(i), Some(draft)) = (self.sel, &mut self.draft) {
                            draft.position = self.p.objects[i].position;
                        }
                        self.message = format!("Verschoben · ΔX {:.1} · ΔY {:.1} mm", d.x, d.y);
                    }
                } else {
                    self.message = "Zum Verschieben die Draufsicht wählen".into();
                }
            }
            if ctx.input(|i| i.pointer.primary_down()) {
                self.drag = Some(drag);
            }
        } else if response.clicked_by(egui::PointerButton::Primary) {
            if let Some(pos) = response.interact_pointer_pos() {
                let (x, y) = ndc(pos);
                self.toggle_selection(
                    renderer::pick(&cam, x, y, &self.cache).map(|hit| hit.0),
                    multi,
                );
            }
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
    fn bounds_selected(&self) -> Option<(DVec3, DVec3)> {
        self.selected
            .iter()
            .map(|i| self.p.objects[*i].mesh().bounds())
            .reduce(|(a, b), (c, d)| (a.min(c), b.max(d)))
    }
    fn place(&mut self, center: bool) {
        if let Some((lo, hi)) = self.bounds_selected() {
            let mut delta = DVec3::new(0., 0., -lo.z);
            if center {
                delta.x = (self.p.printer.bed[0] - lo.x - hi.x) * 0.5;
                delta.y = (self.p.printer.bed[1] - lo.y - hi.y) * 0.5;
            }
            self.remember();
            for &i in &self.selected {
                self.p.objects[i].position += delta;
            }
            self.refresh_draft();
        }
    }
    fn focus(&mut self) {
        if let Some((a, b)) = self.bounds_selected() {
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
        self.measure.clear();
        self.plane_pick = false;
        self.dialog = None;
        match a {
            Action::New => {
                self.backups.clear();
                self.p = Project::default();
                self.path = None;
                self.hist = History::default();
                self.select(None);
                self.dirty = false;
                self.message = "Neues Projekt".into();
            }
            Action::Open => self.choose(FileTask::Open),
            Action::Quit => {
                self.backups.clear();
                self.quit = true;
            }
            Action::Recover(i) => {
                if let Some((_, path)) = self.copies.get(i) {
                    match files::load(path) {
                        Ok(project) => {
                            self.p = project;
                            self.hist = History::default();
                            self.path = None;
                            self.select(Some(0));
                            self.dirty = true;
                            self.message="Sicherung als Kopie geöffnet. Mit Strg+S einen Projektpfad wählen.".into();
                        }
                        Err(e) => self.message = format!("Wiederherstellung fehlgeschlagen · {e}"),
                    }
                }
            }
        }
    }
    pub fn request_quit(&mut self) {
        self.preview = None;
        self.pending = None;
        self.geometry_picker = None;
        self.preparing_picker = None;
        self.action(Action::Quit);
    }
    fn write(&mut self, f: Output, overwrite: bool) {
        let path = PathBuf::from(self.entry.trim());
        match files::save(&path, &self.p, f, overwrite) {
            Ok(()) => {
                if f == Output::Project {
                    self.backups.clear();
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
            FileTask::Import => {
                let path = path.to_owned();
                let scale = self.unit;
                self.new_parts(move || rust_3d_print_studio::import::load(&path, scale));
            }
            FileTask::Svg => {
                let path = path.to_owned();
                self.auto_place = true;
                self.new_parts(move || {
                    let bytes = files::read_limited(&path).map_err(|e| e.to_string())?;
                    let r = rust_3d_print_studio::svg::load(&bytes)?;
                    let mut o = Object::new(Shape::Box, DVec3::ZERO);
                    o.name = "SVG-Relief".into();
                    o.feature = Some(Feature::Relief(r));
                    o.rebuild()?;
                    Ok(vec![o])
                });
            }
            FileTask::Open => match files::load(path) {
                Ok(p) => {
                    self.backups.clear();
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
    fn open_layout(&mut self) {
        if self.selected.is_empty() {
            self.message = "Mindestens einen Körper auswählen".into();
            return;
        }
        self.layout = Some(crate::layout_ui::Editor::new(
            &self.p,
            &self.selected,
            self.step,
        ));
        self.dialog = Some(Dialog::Layout);
    }
    fn new_parts(&mut self, f: impl FnOnce() -> Result<Vec<Object>, String> + Send + 'static) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.job = Some(rx);
        self.dialog = None;
        std::thread::spawn(move || {
            let _ = tx.send(f());
        });
    }
    fn add_parts(&mut self, mut parts: Vec<Object>) -> Result<(), String> {
        let first = self.p.objects.len();
        if parts.is_empty() || first + parts.len() > MAX_OBJECTS {
            return Err("Ungültige Objektanzahl".into());
        }
        if self.auto_place {
            let d = plate::copy_offsets(&self.p.printer, &self.p.objects, &parts, self.gap, 1)
                .into_iter()
                .next()
                .ok_or("Kein freier Platz für die Gruppe")?;
            let z = parts
                .iter()
                .map(|o| o.mesh().bounds().0.z)
                .fold(f64::INFINITY, f64::min);
            for o in &mut parts {
                o.position += d.extend(-z);
                let (a, b) = o.mesh().bounds();
                if let Some(w) = self.p.printer.warning(a, b) {
                    return Err(w.into());
                }
            }
        }
        let mut next = self.p.clone();
        next.objects.extend(parts);
        if !next.valid()
            || serde_json::to_vec_pretty(&next)
                .map_err(|e| e.to_string())?
                .len()
                > 2 * 1024 * 1024
        {
            return Err("Grenzen des R3DP-Projekts überschritten".into());
        }
        self.remember();
        self.p = next;
        self.selected = (first..self.p.objects.len()).collect();
        self.sel = self.selected.last().copied();
        self.refresh_draft();
        self.focus();
        Ok(())
    }
    fn new_case(&mut self) {
        let c = self.case.clone();
        self.auto_place = true;
        self.new_parts(move || {
            let mut parts = vec![];
            for lid in [false, true] {
                let mut c = c.clone();
                c.lid = lid;
                let at = if lid {
                    c.inside.x + 2. * c.wall + 5.
                } else {
                    0.
                };
                let mut o = Object::new(Shape::Box, DVec3::new(at, 0., 0.));
                let f = Feature::Case(c);
                o.name = f.name().into();
                o.feature = Some(f);
                o.rebuild()?;
                parts.push(o);
            }
            Ok(parts)
        });
    }
    fn layout_dialog(&mut self, ui: &mut egui::Ui) {
        if let Some(editor) = &mut self.layout {
            if let Some(plan) = editor.show(ui, &self.p) {
                if matches!(
                    editor.s.mode,
                    rust_3d_print_studio::layout::Mode::Move
                        | rust_3d_print_studio::layout::Mode::Copy
                ) {
                    self.step = editor.s.step
                }
                let mut next = self.p.clone();
                match plan.apply(&mut next) {
                    Ok(ids) => {
                        if !next.valid()
                            || serde_json::to_vec_pretty(&next)
                                .map_or(true, |b| b.len() > 2 * 1024 * 1024)
                        {
                            self.message = "Projektgrenze überschritten".into();
                            self.dialog = None;
                            return;
                        }
                        self.remember();
                        self.p = next;
                        if plan.copy {
                            self.selected = ids;
                            self.sel = self.selected.last().copied();
                        }
                        self.refresh_draft();
                        self.dialog = None;
                        self.message = "Plattenbearbeitung übernommen".into();
                    }
                    Err(e) => self.message = e,
                }
            }
        }
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.dialog = None;
        }
    }
    fn help_button(&mut self, ui: &mut egui::Ui, label: &str, topic: &'static str) -> bool {
        let response = ui
            .button(label)
            .on_hover_text(format!("{topic} · F1 für Hilfe"));
        if response.hovered() {
            self.hover_help = Some(topic);
        }
        response.clicked()
    }
    fn context_topic(&self) -> &'static str {
        match self.dialog {
            Some(Dialog::Recovery) => help::RECOVER,
            Some(Dialog::Layout) => self
                .layout
                .as_ref()
                .map_or(help::PLATE, |e| help::layout(e.s.mode)),
            Some(Dialog::Case) => help::CASE,
            Some(Dialog::Prepare) => self.preparation.help_topic(),
            Some(Dialog::Combine) => "Boolesche",
            Some(Dialog::File(FileTask::Import)) => help::IMPORT,
            Some(Dialog::File(FileTask::Profile)) => help::PROFILE,
            Some(Dialog::File(FileTask::Svg)) => help::CASE,
            Some(Dialog::File(_) | Dialog::Overwrite(_) | Dialog::Discard(_)) => help::FILE,
            None => self.hover_help.unwrap_or(help::TOOLS[self.tool]),
        }
    }
    fn dialogs(&mut self, ctx: &egui::Context) {
        let Some(d) = self.dialog else { return };
        egui::Modal::new("dialog".into()).show(ctx,|ui|{ui.set_min_width(370.);match d{
 Dialog::Recovery=>{
     ui.heading("Projektsicherung wiederherstellen");
     ui.label("Eine Sicherung als neues Projekt öffnen. Die ursprüngliche Datei bleibt erhalten.");
     let mut chosen=None;
     egui::ScrollArea::vertical().max_height(260.).show(ui,|ui|{
         for (i,(time,path)) in self.copies.iter().enumerate(){
             let minutes=time.elapsed().unwrap_or_default().as_secs()/60;
             let session=path.parent().and_then(Path::file_name).and_then(|s|s.to_str()).unwrap_or("Sicherung");
             if ui.button(format!("{session} · vor {minutes} Min.")).on_hover_text(path.display().to_string()).clicked(){chosen=Some(i);}
         }
     });
     if self.copies.is_empty(){ui.label("Keine Sicherung gefunden.");}
     if let Some(i)=chosen{self.action(Action::Recover(i));}
 },
 Dialog::Layout=>self.layout_dialog(ui),
 Dialog::Case=>{ui.heading("Gehäuse mit Deckel");vector(ui,"Innenmaße (mm)",&mut self.case.inside,4.0..=500.);for(label,v)in [("Wandstärke",&mut self.case.wall),("Spiel je Seite",&mut self.case.play),("Einstecktiefe",&mut self.case.lip)]{ui.horizontal(|ui|{ui.label(label);ui.add(egui::DragValue::new(v).speed(0.1).suffix(" mm"));});}if ui.button("Gehäuse und Deckel erzeugen").clicked(){self.new_case();}},
 Dialog::Prepare=>{self.preparation.show(ui,&mut self.p,&mut self.hist,&mut self.dirty);if let (Some(i),Some(d))=(self.sel,self.draft.as_mut()){d.filament_slot=self.p.objects[i].filament_slot;}},
 Dialog::Combine=>{ui.heading("Boolesche Operation");ui.label("Der ausgewählte Körper ist der Grundkörper.");ui.horizontal(|ui|{for(op,name)in [(Operation::Union,"Vereinigung"),(Operation::Difference,"Differenz"),(Operation::Intersection,"Schnittmenge")]{ui.selectable_value(&mut self.operation,op,name);}});ui.label("Werkzeugkörper");for(i,o)in self.p.objects.iter().enumerate(){if Some(i)!=self.sel{ui.selectable_value(&mut self.operand,i,&o.name);}}if ui.button("Berechnen und übernehmen").clicked(){self.combine();}ui.label(&self.message);},
 Dialog::Discard(a)=>{ui.heading("Ungespeicherte Änderungen");ui.label("Sollen die Änderungen verworfen werden?");if ui.button("Verwerfen und fortfahren").clicked(){self.proceed(a);}},
 Dialog::Overwrite(f)=>{ui.heading("Vorhandene Datei ersetzen?");ui.label(&self.entry);if ui.button("Datei ersetzen").clicked(){self.write(f,true);}},
 Dialog::File(t)=>{ui.heading(match t{FileTask::Import=>"Netz importieren",FileTask::Svg=>"SVG-Relief laden",FileTask::Open=>"Projekt öffnen",FileTask::Profile=>"Druckerprofil laden",FileTask::Write(Output::Project)=>"Projekt speichern",_=>"Netz exportieren"});ui.label("Dateipfad mit Endung");ui.text_edit_singleline(&mut self.entry);
 if matches!(t,FileTask::Import){ui.label("Geschlossene STL-, OBJ-, ASCII-PLY- oder Modell-3MF-Geometrie");ui.horizontal(|ui|{ui.label("STL/OBJ/PLY-Einheit");ui.selectable_value(&mut self.unit,1.,"mm");ui.selectable_value(&mut self.unit,10.,"cm");ui.selectable_value(&mut self.unit,25.4,"Zoll");});ui.checkbox(&mut self.auto_place,"Zusammen auf der Platte absetzen");}
 if matches!(t,FileTask::Write(Output::Stl|Output::StlAscii|Output::Ply|Output::Obj|Output::ThreeMf)){ui.label("Exportiert Dreiecksnetze in mm. Konstruktionsparameter bleiben im R3DP-Projekt.");}
 if ui.button("Bestätigen").clicked(){self.file_submit(t);}
 if !self.message.is_empty(){ui.colored_label(Color32::from_rgb(233,155,92),&self.message);}}}
 if ui.small_button("? Hilfe zu dieser Funktion").clicked(){self.help.show_topic(self.context_topic());}
 if ui.button(if matches!(d,Dialog::Prepare){"Schließen"}else{"Abbrechen"}).clicked(){self.dialog=None;}
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
                if item(ui, "Netz importieren …") {
                    self.choose(FileTask::Import);
                }
                if item(ui, "Neues Projekt") {
                    self.action(Action::New)
                }
                if item(ui, "Projekt öffnen …") {
                    self.action(Action::Open)
                }
                if item(ui, "Speichern") {
                    self.save()
                }
                if item(ui, "Sicherung wiederherstellen …") {
                    match rust_3d_print_studio::recovery::directory()
                        .and_then(|dir| rust_3d_print_studio::recovery::list(&dir))
                    {
                        Ok(items) => {
                            self.copies = items;
                            self.dialog = Some(Dialog::Recovery);
                        }
                        Err(e) => self.message = format!("Sicherungen nicht lesbar · {e}"),
                    }
                }
                if item(ui, "Speichern unter …") {
                    self.choose(FileTask::Write(Output::Project))
                }
                ui.separator();
                for (label, f) in [
                    ("STL binär exportieren …", Output::Stl),
                    ("STL ASCII exportieren …", Output::StlAscii),
                    ("PLY ASCII exportieren …", Output::Ply),
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
                if item(ui, "Alle auswählen") {
                    self.select_all();
                }
                if item(ui, "Kopieren") {
                    self.copy();
                }
                if item(ui, "Einfügen") {
                    self.insert_copies(self.clipboard.clone(), false);
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
                if item(ui, "Gehäuse mit Deckel …") {
                    self.dialog = Some(Dialog::Case);
                }
                if item(ui, "SVG als Relief …") {
                    self.choose(FileTask::Svg);
                }
                if item(ui, "Schild und Anhänger") {
                    self.feature_object(Feature::Relief(Default::default()));
                }
                for s in Shape::ALL {
                    if item(ui, s.label()) {
                        self.add(s)
                    }
                }
            });
            ui.menu_button("Modellieren", |ui| {
                for (k,label) in ["Aushöhlen","Kontrollgitter","Ebener Schnitt","Maßbeziehungen"].iter().enumerate(){if item(ui,label){if let Some(o)=&mut self.draft{let source=Box::new(Self::source(o));o.feature=Some(match k{0=>Feature::Shell{source,thickness:1.5,open:true,side:Some(rust_3d_print_studio::cad::references::Side::Top)},1=>Feature::Lattice{source,offsets:[DVec3::ZERO;8],symmetric:false},2=>Feature::Cut{source,point:DVec3::Z*(o.size.z/2.),normal:DVec3::Z},_=>Feature::Driven{source,variables:std::collections::BTreeMap::from([("mass1".into(),16.)]),bindings:vec![]}});self.message="Feature im Eigenschaftenbereich übernehmen".into();}}}

                if item(ui,"Kante abrunden / fasen"){if let Some(o)=&mut self.draft{o.feature=Some(Feature::EdgeFinish{source:Box::new(Self::source(o)),edge:0,reference:None,r1:2.,r2:2.,steps:6});self.message="Kantenmaße im Entwurf bearbeiten".into();}}
                for(name,f)in feature_ui::templates(){if item(ui,name){self.feature_object(f);}}
                if item(ui,"Boolesche Operation …"){self.dialog=Some(Dialog::Combine);}
                if item(ui,"Biegen / Verdrehen / Verjüngen"){if let Some(o)=&mut self.draft{o.feature=Some(Feature::Deform{source:Box::new(Self::source(o)),twist:0.,bend:0.,taper:1.});self.message="Verformung im Entwurf bearbeiten".into();}}
                if item(ui,"In direktes Polygonnetz umwandeln …"){if let Some(o)=&mut self.draft{let mut local=o.clone();local.position=DVec3::ZERO;local.rotation=DVec3::ZERO;o.feature=Some(Feature::Polygon(local.mesh()));self.message="Entwurf als Netz: Übernehmen ersetzt den Parameterbaum. Rückgängig möglich.".into();}}

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
                if item(ui, "Platte vorbereiten …") {
                    self.preparation.reopen();
                    self.dialog = Some(Dialog::Prepare);
                }
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
                                    egui::Key::P,
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
                if item(ui, "Hilfe zum Werkzeug · F1") {
                    self.help.show_topic(self.context_topic());
                }
                if item(ui, "Anleitung durchsuchen") {
                    self.help.show_topic(help::GUIDE);
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
                        .selectable_label(self.selected.contains(&i), &self.p.objects[i].name)
                        .clicked()
                    {
                        self.toggle_selection(
                            Some(i),
                            ui.input(|v| v.modifiers.shift || v.modifiers.command),
                        );
                    }
                }
            });
        ui.separator();
        if self.selected.len() > 1 {
            ui.label(format!(
                "{} ausgewählt · Eigenschaften des aktiven Körpers",
                self.selected.len()
            ));
        }
        ui.heading("Abmessungen");
        let Some(d) = &mut self.draft else {
            ui.label("Körper erstellen oder auswählen.");
            return;
        };
        ui.label("Bezeichnung");
        ui.text_edit_singleline(&mut d.name);
        if let Some(f) = &mut d.feature {
            ui.label(format!(
                "X {:.2}   Y {:.2}   Z {:.2} mm",
                d.size.x, d.size.y, d.size.z
            ));
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.strong(f.name());
                if ui
                    .small_button("?")
                    .on_hover_text("Hilfe zu diesem Konstruktionsschritt")
                    .clicked()
                {
                    self.help.show_topic(help::feature(f));
                }
            });
            let choose_geometry = match f {
                Feature::Shell { .. } => ui.button("Öffnungsfläche in 3D wählen …").clicked(),
                Feature::EdgeFinish { .. } => ui.button("Kante in 3D wählen …").clicked(),
                _ => false,
            };
            ui.push_id(("objektentwurf", self.sel), |ui| feature_ui::edit(ui, f));
            if choose_geometry {
                let outer = d.clone();
                let view = self.p.view.clone();
                let (tx, rx) = std::sync::mpsc::channel();
                self.preparing_picker = Some(rx);
                std::thread::spawn(move || {
                    let _ = tx.send(GeometryPicker::new(&outer, &view));
                });
            }
        } else {
            vector(ui, "Abmessungen · mm", &mut d.size, 0.1..=2000.);
        }
        if d.feature.is_none() && d.shape.hollow() {
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
    }
    fn calculate_draft(&mut self, preview: bool) {
        let percent = self.section_percent;
        let mut view = self.p.view.clone();
        let (Some(i), Some(mut obj)) = (self.sel, self.draft.clone()) else {
            return;
        };
        if self.snap && obj.position != self.p.objects[i].position {
            let step = self.p.view.grid;
            obj.position = (obj.position / step).round() * step;
        }
        let (tx, rx) = std::sync::mpsc::channel();
        self.pending = Some((i, rx));
        self.message = "Konstruktion wird berechnet".into();
        std::thread::spawn(move || {
            let result = obj.rebuild().and_then(|()| {
                if !obj.valid() {
                    Err("Abmessungen, Position und Bohrungsdurchmesser prüfen".into())
                } else if preview {
                    let solid = renderer::Solid::new(&obj);
                    let (lo, hi) = solid.bounds;
                    view.target = (hi + lo) * 0.5;
                    view.distance = ((hi - lo).length() * 2.).clamp(8., 5000.);
                    let section = if matches!(obj.feature, Some(Feature::Shell { .. })) {
                        Some(
                            solid
                                .horizontal_section(percent)
                                .map_err(|err| format!("Schnittvorschau fehlgeschlagen · {err}"))?,
                        )
                    } else {
                        None
                    };
                    Ok(CalculatedEdit::Preview(Box::new(EditPreview {
                        index: i,
                        show_section: section.is_some(),
                        section,
                        solid,
                        view,
                        original: false,
                    })))
                } else {
                    Ok(CalculatedEdit::Apply(Box::new(obj)))
                }
            });
            let _ = tx.send(result);
        });
    }
    fn await_edit(&mut self, ctx: &egui::Context) -> bool {
        use std::sync::mpsc::TryRecvError;
        let Some((index, rx)) = &self.pending else {
            return false;
        };
        let i = *index;
        match rx.try_recv() {
            Err(TryRecvError::Empty) => {
                egui::Modal::new("berechnung".into()).show(ctx, |ui| {
                    ui.spinner();
                    ui.label("Konstruktion wird berechnet und geprüft …");
                });
                true
            }
            result => {
                self.pending = None;
                match result {
                    Ok(Ok(CalculatedEdit::Apply(obj))) => self.commit_edit(i, *obj),
                    Ok(Ok(CalculatedEdit::Preview(preview))) => self.preview = Some(*preview),
                    Ok(Err(e)) => self.message = e,
                    Err(_) => self.message = "Neuberechnung abgebrochen".into(),
                };
                false
            }
        }
    }
    fn commit_edit(&mut self, index: usize, obj: Object) {
        self.remember();
        self.p.objects[index] = obj;
        self.select(Some(index));
        self.message = "Parameter übernommen".into();
    }
    fn show_preview(&mut self, ctx: &egui::Context) {
        let Some(preview) = self.preview.as_mut() else {
            return;
        };
        if self.help.open {
            self.help.draw(ctx);
            return;
        }
        let mut back = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        let mut apply = false;
        let mut help = ctx.input(|i| i.key_pressed(egui::Key::F1));
        egui::TopBottomPanel::top("ergebnisleiste").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if let Some((_, z)) = &preview.section {
                    ui.strong("Schalenvorschau");
                    ui.add_enabled_ui(!preview.original, |ui| {
                        ui.checkbox(
                            &mut preview.show_section,
                            format!("Schnitt bei Z {z:.2} mm"),
                        );
                    });
                } else {
                    ui.strong("Kantenvorschau");
                }
                ui.checkbox(&mut preview.original, "Bisherigen Körper anzeigen");
                apply = ui.button("Ergebnis übernehmen").clicked();
                back |= ui.button("Zurück zum Entwurf").clicked();
                help |= ui.small_button("Hilfe (F1)").clicked();
            });
        });
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let (rect, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
                self.rect = rect;
                self.px = ctx.pixels_per_point();
                navigate(&mut preview.view, &response, rect);
            });
        if apply {
            let index = preview.index;
            if let Some(result) = self.preview.take() {
                self.commit_edit(index, result.solid.into_model());
            }
        } else if back {
            self.preview = None;
        } else if help {
            self.help.show_topic(if preview.section.is_some() {
                "Schalenvorschau"
            } else {
                "Kantenvorschau"
            });
        }
    }
    fn choose_geometry(&mut self, ctx: &egui::Context) {
        if !self.help.open && ctx.input(|i| i.key_pressed(egui::Key::F1)) {
            self.help.show_topic("Kanten und Öffnungsflächen binden");
        }
        if self.help.open {
            self.help.draw(ctx);
            return;
        }
        let Some(picker) = &mut self.geometry_picker else {
            return;
        };
        let mut close = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        egui::TopBottomPanel::top("flaechenauswahl").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.strong(if picker.edges.is_some() {
                    "Kante wählen"
                } else {
                    "Öffnungsfläche wählen"
                });
                ui.label(&picker.hint);
                close |= ui.button("Abbrechen").clicked();
                if ui.small_button("Hilfe (F1)").clicked() {
                    self.help.show_topic("Kanten und Öffnungsflächen binden");
                }
            });
        });
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
                self.rect = rect;
                self.px = ctx.pixels_per_point();
                if close || !rect.is_positive() { return; }
                navigate(&mut picker.view, &response, rect);
                let Some(pos) = response.hover_pos() else { return };
                let clicked = response.clicked_by(egui::PointerButton::Primary);
                let cam = renderer::camera(&picker.view, rect, self.px);
                let x = 2. * (pos.x - rect.left()) / rect.width() - 1.;
                let y = 1. - 2. * (pos.y - rect.top()) / rect.height();
                let Some((_, face, _)) = renderer::surface(&cam, x, y, std::slice::from_ref(&picker.solid)) else {
                    if clicked { picker.hint = "Kein Treffer. Bitte auf den Außenkörper klicken.".into(); }
                    return;
                };
                let matrix = macroquad::camera::Camera::matrix(&cam);
                let corners = picker.solid.mesh.faces[face].map(|i| {
                    let point = matrix.project_point3(picker.solid.mesh.vertices[i].as_vec3());
                    rect.center() + egui::vec2(point.x*rect.width()/2., -point.y*rect.height()/2.)
                });
                if let Some(edges) = &picker.edges {
                    let Some((index, segment)) = nearest_sharp_edge(edges, picker.solid.mesh.faces[face], corners, pos) else {
                        if clicked { picker.hint = "Näher an eine scharfe Außenkante klicken.".into(); }
                        return;
                    };
                    ui.painter().line_segment(segment, (4., ACCENT));
                    if clicked {
                        match picker.solid.edge(index) {
                            Ok(bound) => {
                                if let Some(Feature::EdgeFinish { edge, reference, .. }) = self.draft.as_mut().and_then(|d|d.feature.as_mut()) {
                                    *edge = index;
                                    *reference = Some(bound);
                                    self.message = "Kante gewählt. Mit „Parameter übernehmen“ die Bearbeitung anwenden.".into();
                                }
                                close = true;
                            }
                            Err(e) => picker.hint = e,
                        }
                    }
                    return;
                }
                ui.painter().add(egui::Shape::closed_line(corners.to_vec(), (2., ACCENT)));
                if clicked {
                    match picker.solid.opening(face) {
                        Ok(reference) => {
                            if let Some(Feature::Shell { side, open, .. }) = self.draft.as_mut().and_then(|d| d.feature.as_mut()) {
                                *side = Some(rust_3d_print_studio::cad::references::Side::Plane(reference));
                                *open = true;
                                self.message = "Öffnungsfläche gewählt. Mit „Parameter übernehmen“ die Schale neu berechnen.".into();
                            }
                            close = true;
                        }
                        Err(e) => picker.hint = e,
                    }
                }
            });
        if close {
            self.geometry_picker = None;
        }
    }
    pub fn frame(&mut self, ctx: &egui::Context) {
        self.hover_help = None;
        if self.checkpoint.elapsed() >= std::time::Duration::from_secs(30) {
            if self.dirty {
                self.backups.checkpoint(&self.p);
            }
            self.checkpoint = std::time::Instant::now();
        }
        if let Some(error) = self.backups.error() {
            self.message = error;
        }

        if let Some(rx) = &self.job {
            match rx.try_recv() {
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    egui::Modal::new("teile".into()).show(ctx, |ui| {
                        ui.spinner();
                        ui.label("Geometrie wird berechnet und geprüft …");
                    });
                    return;
                }
                r => {
                    self.job = None;
                    self.message = match r
                        .map_err(|_| "Berechnung unterbrochen".to_owned())
                        .and_then(|r| r)
                        .and_then(|parts| self.add_parts(parts))
                    {
                        Ok(()) => "Körper übernommen".into(),
                        Err(e) => e,
                    };
                }
            }
        }
        if self.await_edit(ctx) {
            return;
        }
        ctx.set_visuals(if self.dark {
            {
                let mut v = egui::Visuals::dark();
                v.panel_fill = Color32::from_rgb(37, 47, 62);
                v.window_fill = Color32::from_rgb(34, 43, 56);
                v.extreme_bg_color = Color32::from_rgb(24, 33, 44);
                v.widgets.inactive.weak_bg_fill = Color32::from_rgb(51, 63, 80);
                v.selection.bg_fill = Color32::from_rgb(58, 105, 92);
                v.widgets.active.bg_fill = v.selection.bg_fill;
                v.override_text_color = Some(Color32::from_rgb(222, 229, 236));
                v
            }
        } else {
            egui::Visuals::light()
        });
        if let Some(rx) = &self.preparing_picker {
            use std::sync::mpsc::TryRecvError;
            match rx.try_recv() {
                Err(TryRecvError::Empty) => {
                    egui::Modal::new("aussenkoerper".into()).show(ctx, |ui| {
                        ui.spinner();
                        ui.label("Außenkörper wird berechnet …");
                    });
                    return;
                }
                result => {
                    self.preparing_picker = None;
                    match result {
                        Ok(Ok(picker)) => self.geometry_picker = Some(picker),
                        Ok(Err(error)) => self.message = error,
                        Err(_) => self.message = "Berechnung der Außenfläche abgebrochen.".into(),
                    }
                }
            }
        }
        if self.preview.is_some() && self.dialog.is_none() {
            self.show_preview(ctx);
            return;
        }
        if self.geometry_picker.is_some() && self.dialog.is_none() {
            self.choose_geometry(ctx);
            return;
        }
        egui::TopBottomPanel::top("menus").show(ctx, |ui| {
            let (bar, _) =
                ui.allocate_exact_size(egui::vec2(ui.available_width(), 28.), egui::Sense::hover());
            ui.painter().text(
                bar.center(),
                egui::Align2::CENTER_CENTER,
                "Rust 3D Print Studio",
                egui::FontId::proportional(17.),
                ui.visuals().text_color(),
            );
            let filename = self
                .path
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|s| s.to_string_lossy())
                .unwrap_or_else(|| self.p.name.as_str().into());
            ui.painter().text(
                bar.left_center(),
                egui::Align2::LEFT_CENTER,
                format!("{}{}", filename, if self.dirty { " •" } else { "" }),
                egui::FontId::proportional(13.),
                ui.visuals().text_color(),
            );
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
                ui.strong(&self.p.printer.name);
                ui.separator();
                ui.strong("Maßeinheit: mm");
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
                ui.add_space(8.);
                for (i, label) in [
                    "Auswahl",
                    "Körper",
                    "Skizze",
                    "Drehen",
                    "Skalieren",
                    "Bohrung",
                    "Schneiden",
                    "Messen",
                ]
                .iter()
                .enumerate()
                {
                    let mut b = egui::Button::new(*label);
                    if self.tool == i {
                        b = b.fill(Color32::from_rgb(58, 105, 92));
                    }
                    let response = ui
                        .add_sized([133., 34.], b)
                        .on_hover_text(format!("{} · F1 für Hilfe", help::TOOLS[i]));
                    if response.hovered() {
                        self.hover_help = Some(help::TOOLS[i]);
                    }
                    if response.clicked() {
                        self.tool = i;
                        self.plane_pick = false;
                    }
                }
                ui.separator();
                match self.tool {
                    0 => {
                        ui.small("Ziehen zum Verschieben. Umschalt + Klick für mehrere Körper.");
                        if self.help_button(ui, "Plattenwerkzeuge …", help::PLATE) {
                            self.open_layout();
                        }
                        ui.horizontal(|ui| {
                            for (k, t) in [(None, "XY"), (Some(0), "X"), (Some(1), "Y")] {
                                ui.selectable_value(&mut self.axis, k, t);
                            }
                        });
                        if self.help_button(ui, "Draufsicht", "Ansicht steuern") {
                            self.top_view();
                        }
                        if ui.button("Alle auswählen").clicked() {
                            self.select_all();
                        }
                        ui.checkbox(&mut self.snap, "Am Raster einrasten");
                        ui.add(
                            egui::DragValue::new(&mut self.gap)
                                .range(0.0..=30.)
                                .speed(0.5)
                                .prefix("Abstand ")
                                .suffix(" mm"),
                        );
                        if self.help_button(ui, "Alle anordnen", "Druckplatte belegen") {
                            self.arrange_plate();
                        }
                        if self.help_button(ui, "Mit Kopien füllen", "Druckplatte belegen") {
                            self.insert_copies(self.selection_copy(), true);
                        }
                        if ui.button("Auswahl einpassen").clicked() {
                            self.focus()
                        }
                        if ui.button("Duplizieren").clicked() {
                            self.duplicate()
                        }
                    }
                    1 => {
                        ui.menu_button("Körper erstellen", |ui| {
                            for s in Shape::ALL {
                                if item(ui, s.label()) {
                                    self.add(s);
                                }
                            }
                        });
                        if ui.button("J-Wandhaken").clicked() {
                            self.feature_object(Feature::Hook(Default::default()));
                        }
                    }
                    2 => {
                        if ui
                            .button(if self.plane_pick {
                                "Flächenwahl abbrechen"
                            } else {
                                "Skizze auf Fläche"
                            })
                            .clicked()
                        {
                            self.plane_pick = !self.plane_pick;
                        }
                        if self.plane_pick {
                            ui.small("Position auf einer Modellfläche anklicken.");
                        }
                        for (name, f) in feature_ui::templates()
                            .into_iter()
                            .filter(|(_, f)| !matches!(f, Feature::Hook(_)))
                        {
                            if self.help_button(ui, name, help::feature(&f)) {
                                self.feature_object(f);
                            }
                        }
                    }
                    3 => {
                        if let Some(d) = &mut self.draft {
                            vector(ui, "Drehung °", &mut d.rotation, -3600.0..=3600.);
                            ui.small("Rechts übernehmen");
                        }
                    }
                    4 => {
                        if let Some(d) = &mut self.draft {
                            if d.feature.is_some() {
                                if ui.button("Skalierung anfügen").clicked() {
                                    d.feature = Some(Feature::Transform {
                                        source: Box::new(Self::source(d)),
                                        scale: DVec3::ONE,
                                        rotation: DVec3::ZERO,
                                        translation: DVec3::ZERO,
                                    });
                                }
                            } else {
                                vector(ui, "Maße · mm", &mut d.size, 0.1..=2000.);
                            }
                            ui.small("Rechts übernehmen");
                        }
                    }
                    5 => {
                        if ui.button("Bohrquader erstellen").clicked() {
                            self.add(Shape::BoredBox)
                        }
                        if self.help_button(ui, "Körper abziehen …", "Boolesche") {
                            self.operation = Operation::Difference;
                            self.dialog = Some(Dialog::Combine);
                        }
                    }
                    6 => {
                        if self.help_button(ui, "Körperschnitt …", "Boolesche") {
                            self.dialog = Some(Dialog::Combine)
                        }
                    }
                    _ => {
                        ui.small("Zwei Punkte auf Körpern anklicken.");
                        ui.checkbox(&mut self.magnet, "Ecken / Kantenmitten fangen");
                        if let [a, b] = self.measure.as_slice() {
                            let d = *b - *a;
                            ui.strong(format!("{:.3} mm", d.length()));
                            ui.label(format!("ΔX {:.2}   ΔY {:.2}   ΔZ {:.2}", d.x, d.y, d.z));
                        }
                        if let Some(i) = self.sel {
                            let (a, b) = self.p.objects[i].mesh().bounds();
                            for (label, v) in [("X", b.x - a.x), ("Y", b.y - a.y), ("Z", b.z - a.z)]
                            {
                                ui.label(format!("{label}   {v:.2} mm"));
                            }
                        }
                    }
                }
            });
        egui::SidePanel::right("inspector")
            .default_width(245.)
            .width_range(220.0..=360.)
            .show(ctx, |ui| {
                egui::TopBottomPanel::bottom("modellaktionen").show_inside(ui, |ui| {
                    ui.add_space(6.);
                    let feature = self.draft.as_ref().and_then(|d| d.feature.as_ref());
                    if matches!(feature, Some(Feature::Shell { .. })) {
                        ui.add(
                            egui::Slider::new(&mut self.section_percent, 1..=99)
                                .text("Schnitthöhe %"),
                        );
                    }
                    if matches!(
                        feature,
                        Some(Feature::EdgeFinish { .. } | Feature::Shell { .. })
                    ) && ui.button("Vorschau anzeigen").clicked()
                    {
                        self.calculate_draft(true);
                    }
                    if self.draft.is_some()
                        && ui
                            .add_sized(
                                [ui.available_width(), 32.],
                                egui::Button::new("Parameter übernehmen"),
                            )
                            .clicked()
                    {
                        self.calculate_draft(false);
                    }
                    if ui
                        .add_sized(
                            [ui.available_width(), 32.],
                            egui::Button::new("Modell prüfen").fill(Color32::from_rgb(49, 92, 82)),
                        )
                        .clicked()
                    {
                        self.check_model();
                    }
                    if ui
                        .add_sized(
                            [ui.available_width(), 32.],
                            egui::Button::new("3MF exportieren")
                                .fill(Color32::from_rgb(58, 105, 92)),
                        )
                        .clicked()
                    {
                        self.choose(FileTask::Write(Output::ThreeMf));
                    }
                    if ui
                        .add_sized(
                            [ui.available_width(), 32.],
                            egui::Button::new("Platte vorbereiten …"),
                        )
                        .clicked()
                    {
                        self.preparation.reopen();
                        self.dialog = Some(Dialog::Prepare);
                    }
                    ui.add_space(6.);
                });
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
                if !self.help.open && self.dialog.is_none() && rect.is_positive() {
                    navigate(&mut self.p.view, &response, rect);
                    renderer::sync(&mut self.cache, &self.p);
                    self.plate_pointer(&response, rect, ui);
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
        if self.shortcut
            && !self.help.open
            && self.marquee.is_none()
            && self.drag.is_none()
            && self.dialog.is_none()
            && !ctx.wants_keyboard_input()
        {
            let (all, copy, paste, dup) = ctx.input(|i| {
                let pressed = |key| command_pressed(&i.events, key);
                (
                    pressed(egui::Key::A),
                    pressed(egui::Key::C) || i.events.contains(&egui::Event::Copy),
                    pressed(egui::Key::V)
                        || i.events.iter().any(|e| matches!(e, egui::Event::Paste(_))),
                    pressed(egui::Key::D),
                )
            });
            if all {
                self.select_all();
            }
            if copy {
                self.copy();
            }
            use macroquad::prelude::{is_key_down, is_key_pressed, KeyCode};
            let paste_native = is_key_pressed(KeyCode::V)
                && [
                    KeyCode::LeftControl,
                    KeyCode::RightControl,
                    KeyCode::LeftSuper,
                    KeyCode::RightSuper,
                ]
                .into_iter()
                .any(is_key_down);
            if paste || paste_native {
                self.insert_copies(self.clipboard.clone(), false);
            }
            if dup {
                self.duplicate();
            }

            let (z, y, s, del) = ctx.input(|i| {
                (
                    command_pressed(&i.events, self.keys[0]),
                    command_pressed(&i.events, self.keys[1]),
                    command_pressed(&i.events, self.keys[2]),
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
        if !self.help.open && macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F1) {
            self.help.show_topic(self.context_topic());
        }
        if !self.help.open {
            self.dialogs(ctx);
        }
        if self.help.open {
            self.help.draw(ctx);
        }
    }
    pub fn render(&mut self) {
        if self.rect.is_positive() {
            if let Some(preview) = &self.preview {
                let solid = if preview.original {
                    self.cache.get(preview.index).unwrap_or(&preview.solid)
                } else if let Some((section, _)) =
                    preview.section.as_ref().filter(|_| preview.show_section)
                {
                    section
                } else {
                    &preview.solid
                };
                let cam = renderer::camera(&preview.view, self.rect, self.px);
                renderer::draw(
                    &cam,
                    &self.p,
                    std::slice::from_ref(solid),
                    &[],
                    false,
                    self.dark,
                );
                return;
            }
            if let Some(picker) = &self.geometry_picker {
                let cam = renderer::camera(&picker.view, self.rect, self.px);
                renderer::draw(
                    &cam,
                    &self.p,
                    std::slice::from_ref(&picker.solid),
                    &[],
                    false,
                    self.dark,
                );
                return;
            }
            renderer::sync(&mut self.cache, &self.p);
            let cam = renderer::camera(&self.p.view, self.rect, self.px);
            renderer::draw(
                &cam,
                &self.p,
                &self.cache,
                &self.selected,
                self.wire,
                self.dark,
            );
        }
    }
}
// Nur Kanten des vordersten getroffenen Dreiecks sind an dieser Stelle sichtbar.
fn nearest_sharp_edge(
    edges: &[(usize, usize)],
    face: [usize; 3],
    corners: [egui::Pos2; 3],
    mouse: egui::Pos2,
) -> Option<(usize, [egui::Pos2; 2])> {
    let mut best = None;
    let mut limit = 10.;
    for (a, b) in [(0, 1), (1, 2), (2, 0)] {
        let (i, j) = (face[a], face[b]);
        let Ok(index) = edges.binary_search(&(i.min(j), i.max(j))) else {
            continue;
        };
        let delta = corners[b] - corners[a];
        let length_sq = delta.length_sq();
        if length_sq < 1e-6 {
            continue;
        }
        let t = ((mouse - corners[a]).dot(delta) / length_sq).clamp(0., 1.);
        let distance = mouse.distance(corners[a] + delta * t);
        if distance < limit {
            limit = distance;
            best = Some((index, [corners[a], corners[b]]));
        }
    }
    best
}

fn navigate(v: &mut View, response: &egui::Response, rect: egui::Rect) {
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
        let wheel = response.ctx.input(|i| i.raw_scroll_delta.y);
        v.distance = (v.distance * (-f64::from(wheel) * 0.0025).exp()).clamp(8., 5000.);
    }
}
fn item(ui: &mut egui::Ui, text: &str) -> bool {
    let hit = ui.button(text).clicked();
    if hit {
        ui.close_menu();
    }
    hit
}
pub(crate) fn vector(
    ui: &mut egui::Ui,
    label: &str,
    v: &mut DVec3,
    range: std::ops::RangeInclusive<f64>,
) {
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

#[cfg(test)]
mod shortcut_tests {
    use super::*;
    fn key(pressed: bool, command: bool) -> egui::Event {
        egui::Event::Key {
            key: egui::Key::S,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers {
                command,
                ..Default::default()
            },
        }
    }

    #[test]
    fn short_command_survives_release_before_frame() {
        let events = [key(true, true), key(false, false)];
        assert!(command_pressed(&events, egui::Key::S));
        assert!(!command_pressed(&events, egui::Key::D));
    }

    #[test]
    fn modifier_pressed_after_letter_does_not_trigger_command() {
        assert!(!command_pressed(
            &[key(true, false), key(false, true)],
            egui::Key::S
        ));
    }
}

#[cfg(test)]
mod edge_pick_tests {
    use super::*;
    #[test]
    fn hit_requires_visible_sharp_segment_within_pointer_tolerance() {
        let corners = [
            egui::pos2(10., 10.),
            egui::pos2(110., 10.),
            egui::pos2(10., 110.),
        ];
        let edges = [(1, 3), (2, 3), (5, 6)];
        let pick = |x, y| {
            nearest_sharp_edge(&edges, [3, 1, 2], corners, egui::pos2(x, y)).map(|hit| hit.0)
        };
        assert_eq!(pick(50., 12.), Some(0));
        assert_eq!(pick(12., 50.), Some(1));
        assert_eq!(pick(60., 60.), None); // koplanare Diagonale
        assert_eq!(pick(30., 30.), None);
    }
}
