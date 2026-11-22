//! Vorschläge für die direkte Bearbeitung einer Auswahl in Plattenkoordinaten.
use crate::{
    geometry::Object,
    project::{Project, MAX_OBJECTS},
};
use glam::{DVec2, DVec3};
use std::collections::BTreeSet;
#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Grid,
    Circle,
    Copy,
    Move,
    Align,
    Distribute,
}
#[derive(Clone, PartialEq)]
pub struct Options {
    pub mode: Mode,
    pub cols: usize,
    pub rows: usize,
    pub count: usize,
    pub gap: f64,
    pub step: DVec2,
    pub axis: usize,
    pub edge: usize,
    pub reference: usize,
    pub radius: f64,
    pub turn: bool,
    pub reverse: [bool; 2],
    pub skipped: BTreeSet<usize>,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Grid,
            cols: 4,
            rows: 3,
            count: 6,
            gap: 4.,
            step: DVec2::new(24., 0.),
            axis: 0,
            edge: 1,
            reference: 0,
            radius: 35.,
            turn: false,
            reverse: [false; 2],
            skipped: BTreeSet::new(),
        }
    }
}
pub struct Item {
    pub id: usize,
    pub cell: usize,
    pub delta: DVec2,
    pub angle: f64,
    pub lo: DVec3,
    pub hi: DVec3,
    pub skip: bool,
    pub issue: Option<String>,
}
pub struct Plan {
    pub items: Vec<Item>,
    pub copy: bool,
    pub error: Option<String>,
}
impl Plan {
    pub fn allowed(&self) -> bool {
        self.error.is_none()
            && self.items.iter().any(|i| !i.skip)
            && self.items.iter().all(|i| i.skip || i.issue.is_none())
    }
    pub fn apply(&self, p: &mut Project) -> Result<Vec<usize>, String> {
        if !self.allowed() {
            return Err("Vorschau enthält Konflikte".into());
        }
        let mut ids = vec![];
        for item in self.items.iter().filter(|i| !i.skip) {
            let id = if self.copy {
                let mut o = p.objects[item.id].clone();
                let suffix = format!(" · {}", p.objects.len() + 1);
                o.name = o
                    .name
                    .chars()
                    .take(120 - suffix.chars().count())
                    .collect::<String>()
                    + &suffix;
                p.objects.push(o);
                p.objects.len() - 1
            } else {
                item.id
            };
            let o = &mut p.objects[id];
            o.position += item.delta.extend(0.);
            if item.angle != 0. {
                o.rotation.z = (o.rotation.z + item.angle).rem_euclid(360.)
            }
            ids.push(id);
        }
        Ok(ids)
    }
}
pub struct Scene {
    pub bounds: Vec<(DVec3, DVec3)>,
    pub ids: Vec<usize>,
}
impl Scene {
    pub fn new(objects: &[Object], ids: &[usize]) -> Self {
        Self {
            bounds: objects.iter().map(|o| o.mesh().bounds()).collect(),
            ids: ids.to_vec(),
        }
    }
    pub fn plan(&self, p: &Project, s: &Options) -> Plan {
        let copy = matches!(s.mode, Mode::Grid | Mode::Circle | Mode::Copy);
        let mut plan = Plan {
            items: vec![],
            copy,
            error: None,
        };
        let mut ids = self.ids.clone();
        if ids.is_empty() || ids.iter().any(|&i| i >= self.bounds.len()) {
            plan.error = Some("Körper auswählen".into());
            return plan;
        }
        let axis = s.axis.min(1);
        let lo = ids
            .iter()
            .map(|&i| self.bounds[i].0)
            .reduce(DVec3::min)
            .unwrap();
        let hi = ids
            .iter()
            .map(|&i| self.bounds[i].1)
            .reduce(DVec3::max)
            .unwrap();
        let mut place = |id: usize, cell: usize, d: DVec2, angle: f64| {
            let (a, b) = self.bounds[id];
            let (lo, hi) = if angle == 0. {
                (a + d.extend(0.), b + d.extend(0.))
            } else {
                let mut o = p.objects[id].clone();
                o.rotation.z += angle;
                o.position += d.extend(0.);
                o.mesh().bounds()
            };
            plan.items.push(Item {
                id,
                cell,
                delta: d,
                angle,
                lo,
                hi,
                skip: copy && s.skipped.contains(&cell),
                issue: None,
            });
        };
        match s.mode {
            Mode::Align | Mode::Distribute if ids.len() < 2 => {
                plan.error = Some("Mindestens zwei Körper auswählen".into())
            }
            Mode::Align => {
                let edge = |i: usize| {
                    let (a, b) = self.bounds[i];
                    match s.edge {
                        0 => a[axis],
                        1 => (a[axis] + b[axis]) / 2.,
                        _ => b[axis],
                    }
                };
                let reference = ids[s.reference.min(ids.len() - 1)];
                for i in ids {
                    if i != reference {
                        let mut d = DVec2::ZERO;
                        d[axis] = edge(reference) - edge(i);
                        place(i, 0, d, 0.)
                    }
                }
            }
            Mode::Distribute => {
                ids.sort_by(|&a, &b| self.bounds[a].0[axis].total_cmp(&self.bounds[b].0[axis]));
                let mut previous = self.bounds[ids[0]].1[axis];
                for i in ids.into_iter().skip(1) {
                    let (a, b) = self.bounds[i];
                    let mut d = DVec2::ZERO;
                    d[axis] = previous + s.gap - a[axis];
                    previous = b[axis] + d[axis];
                    place(i, 0, d, 0.)
                }
            }
            Mode::Move => {
                for i in ids {
                    place(i, 0, s.step, 0.);
                }
            }
            _ => {
                let n = match s.mode {
                    Mode::Grid => s.rows.saturating_mul(s.cols),
                    Mode::Copy => s.count.saturating_add(1),
                    _ => s.count,
                };
                if !(2..=128).contains(&n) {
                    plan.error = Some("2 bis 128 Felder wählen".into());
                    return plan;
                }
                let active = (1..n).filter(|c| !s.skipped.contains(c)).count();
                if active * ids.len() + p.objects.len() > MAX_OBJECTS {
                    plan.error = Some("Objektgrenze 128 überschritten".into());
                }
                let stride = (hi - lo).truncate() + DVec2::splat(s.gap);
                let center = (hi + lo).truncate() * 0.5;
                let pivot = center - DVec2::X * s.radius;
                for c in 1..n {
                    let theta = std::f64::consts::TAU * c as f64 / n as f64;
                    let rot = DVec2::from_angle(theta);
                    for &i in &ids {
                        let d = match s.mode {
                            Mode::Grid => {
                                DVec2::new(
                                    (c % s.cols) as f64 * if s.reverse[0] { -1. } else { 1. },
                                    (c / s.cols) as f64 * if s.reverse[1] { -1. } else { 1. },
                                ) * stride
                            }
                            Mode::Circle => {
                                let point = if s.turn {
                                    p.objects[i].position.truncate()
                                } else {
                                    center
                                };
                                pivot + rot.rotate(point - pivot) - point
                            }
                            _ => s.step * c as f64,
                        };
                        place(
                            i,
                            c,
                            d,
                            if s.mode == Mode::Circle && s.turn {
                                theta.to_degrees()
                            } else {
                                0.
                            },
                        );
                    }
                }
            }
        }
        if plan.error.is_some() {
            return plan;
        }
        let moved: BTreeSet<_> = plan.items.iter().map(|i| i.id).collect();
        let intersect =
            |a: DVec3, b: DVec3, c: DVec3, d: DVec3| (b.min(d) - a.max(c)).min_element() > 1e-7;
        for item in plan.items.iter_mut().filter(|i| !i.skip) {
            item.issue = p.printer.warning(item.lo, item.hi).map(str::to_owned);
            if item.issue.is_none() {
                if let Some((i, _)) = self.bounds.iter().enumerate().find(|(i, (a, b))| {
                    (copy || !moved.contains(i)) && intersect(item.lo, item.hi, *a, *b)
                }) {
                    item.issue = Some(format!("Überschneidung mit {}", p.objects[i].name));
                }
            }
        }
        for a in 0..plan.items.len() {
            for b in a + 1..plan.items.len() {
                let (x, y) = (&plan.items[a], &plan.items[b]);
                if !x.skip
                    && !y.skip
                    && (!copy || x.cell != y.cell)
                    && intersect(x.lo, x.hi, y.lo, y.hi)
                {
                    plan.items[a]
                        .issue
                        .get_or_insert("Hüllboxen überschneiden sich".into());
                    plan.items[b]
                        .issue
                        .get_or_insert("Hüllboxen überschneiden sich".into());
                }
            }
        }
        if !copy && plan.items.iter().all(|i| i.delta.length_squared() < 1e-16) {
            plan.error = Some("Auswahl liegt bereits am Ziel".into())
        }
        plan
    }
}
