use crate::geometry::Object;
use glam::DVec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Printer {
    pub name: String,
    pub bed: [f64; 2],
    pub nominal_z: f64,
    pub print_z: f64,
    pub excluded: Vec<[f64; 4]>,
    pub nozzle: f64,
    pub filament: f64,
}
impl Default for Printer {
    fn default() -> Self {
        serde_json::from_str(include_str!("../profiles/p1s.json"))
            .expect("Eingebettetes Profil ist geprüft")
    }
}
impl Printer {
    pub fn valid(&self) -> bool {
        let [x, y] = self.bed;
        self.name.len() <= 128
            && [x, y, self.nominal_z, self.print_z]
                .iter()
                .all(|v| (1.0..=2000.).contains(v))
            && self.print_z <= self.nominal_z
            && (0.1..=2.).contains(&self.nozzle)
            && (1.0..=4.).contains(&self.filament)
            && self.excluded.len() <= 32
            && self
                .excluded
                .iter()
                .all(|&[a, b, c, d]| 0. <= a && a < c && c <= x && 0. <= b && b < d && d <= y)
    }
    pub fn warning(&self, lo: DVec3, hi: DVec3) -> Option<&'static str> {
        let eps = 1e-7;
        if lo.min_element() < -eps
            || hi.x > self.bed[0] + eps
            || hi.y > self.bed[1] + eps
            || hi.z > self.print_z + eps
        {
            Some("Profilgrenze überschritten")
        } else if self
            .excluded
            .iter()
            .any(|&[a, b, c, d]| hi.x > a && hi.y > b && lo.x < c && lo.y < d)
        {
            Some("Hüllbox berührt Sperrzone")
        } else {
            None
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct View {
    pub target: DVec3,
    pub yaw: f64,
    pub pitch: f64,
    pub distance: f64,
    pub perspective: bool,
    pub grid: f64,
}
impl Default for View {
    fn default() -> Self {
        Self {
            target: DVec3::new(128., 128., 0.),
            yaw: -0.85,
            pitch: 0.72,
            distance: 430.,
            perspective: false,
            grid: 10.,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub version: u32,
    pub name: String,
    pub printer: Printer,
    pub view: View,
    pub objects: Vec<Object>,
}
impl Default for Project {
    fn default() -> Self {
        Self {
            version: 1,
            name: "Unbenannt".into(),
            printer: Printer::default(),
            view: View::default(),
            objects: vec![],
        }
    }
}
pub const MAX_OBJECTS: usize = 128;
impl Project {
    pub fn valid(&self) -> bool {
        self.version == 1
            && self.name.len() <= 160
            && self.objects.len() <= MAX_OBJECTS
            && self.printer.valid()
            && self.objects.iter().all(Object::valid)
            && self.view.target.is_finite()
            && self.view.target.abs().max_element() <= 20000.
            && self.view.yaw.is_finite()
            && (-std::f64::consts::FRAC_PI_2..=std::f64::consts::FRAC_PI_2)
                .contains(&self.view.pitch)
            && (8.0..=5000.).contains(&self.view.distance)
            && (1.0..=100.).contains(&self.view.grid)
    }
}
#[derive(Default)]
pub struct History {
    past: Vec<Project>,
    future: Vec<Project>,
}
impl History {
    pub fn record(&mut self, before: Project) {
        if self.past.len() == 64 {
            self.past.remove(0);
        }
        self.past.push(before);
        self.future.clear();
    }
    pub fn undo(&mut self, p: &mut Project) -> bool {
        self.past.pop().is_some_and(|s| {
            self.future.push(std::mem::replace(p, s));
            true
        })
    }
    pub fn redo(&mut self, p: &mut Project) -> bool {
        self.future.pop().is_some_and(|s| {
            self.past.push(std::mem::replace(p, s));
            true
        })
    }
}
