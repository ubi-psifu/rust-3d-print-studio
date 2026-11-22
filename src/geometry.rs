use glam::{DQuat, DVec3, EulerRot};
use serde::{Deserialize, Serialize};
use std::f64::consts::TAU;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shape {
    Box,
    Cylinder,
    Cone,
    Pyramid,
    Tube,
    BoredBox,
    Prism,
}
impl Shape {
    pub const ALL: [Self; 7] = [
        Self::Box,
        Self::Cylinder,
        Self::Cone,
        Self::Pyramid,
        Self::Tube,
        Self::BoredBox,
        Self::Prism,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Box => "Quader",
            Self::Cylinder => "Zylinder",
            Self::Cone => "Kegel",
            Self::Pyramid => "Pyramide",
            Self::Tube => "Rohr",
            Self::BoredBox => "Bohrquader",
            Self::Prism => "Sechskant",
        }
    }
    pub fn hollow(self) -> bool {
        matches!(self, Self::Tube | Self::BoredBox)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<crate::cad::Feature>,
    #[serde(skip)]
    cached: Option<std::sync::Arc<Mesh>>,
    pub name: String,
    pub shape: Shape,
    pub size: DVec3,
    pub position: DVec3,
    pub rotation: DVec3,
    pub bore: f64,
    #[serde(default = "first_slot", skip_serializing_if = "is_first_slot")]
    pub filament_slot: usize,
    pub color: [u8; 3],
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mesh {
    pub vertices: Vec<DVec3>,
    pub faces: Vec<[usize; 3]>,
}
fn first_slot() -> usize {
    1
}
fn is_first_slot(s: &usize) -> bool {
    *s == 1
}
impl Object {
    pub fn new(shape: Shape, position: DVec3) -> Self {
        Self {
            name: shape.label().into(),
            feature: None,
            cached: None,
            shape,
            size: DVec3::splat(20.),
            position,
            rotation: DVec3::ZERO,
            bore: 8.,
            filament_slot: 1,
            color: [70, 181, 172],
        }
    }
    pub fn valid(&self) -> bool {
        self.feature.is_some() == self.cached.is_some()
            && (1..=16).contains(&self.filament_slot)
            && self.name.chars().count() <= 120
            && self.position.is_finite()
            && self.position.abs().max_element() <= 10000.
            && self.rotation.is_finite()
            && self.rotation.abs().max_element() <= 3600.
            && self.size.is_finite()
            && self.size.min_element() >= 0.1
            && self.size.max_element() <= 2000.
            && self.bore.is_finite()
            && (!self.shape.hollow()
                || (self.bore >= 0.02 && self.bore < self.size.x.min(self.size.y)))
    }
    pub fn rebuild(&mut self) -> Result<(), String> {
        if let Some(f) = &self.feature {
            let mesh = f.build()?;
            let (lo, hi) = mesh.bounds();
            self.size = hi - lo;
            self.cached = Some(std::sync::Arc::new(mesh));
        } else {
            self.cached = None;
        }
        Ok(())
    }
    pub fn mesh(&self) -> Mesh {
        let mut m = match &self.cached {
            Some(m) => (**m).clone(),
            None => self.primitive(),
        };
        let r = self.rotation * (TAU / 360.);
        let rot = DQuat::from_euler(EulerRot::ZYX, r.z, r.y, r.x);
        for v in &mut m.vertices {
            *v = rot * *v + self.position;
        }
        m
    }
    fn primitive(&self) -> Mesh {
        let square = matches!(self.shape, Shape::Box | Shape::BoredBox | Shape::Pyramid);
        let n = match self.shape {
            Shape::Box | Shape::Pyramid => 4,
            Shape::Prism => 6,
            _ => 48,
        };
        let mut m = Mesh::default();
        let h = self.size.z;
        for z in [0., h] {
            for i in 0..n {
                let angle = TAU * (i as f64 / n as f64) + if square { TAU / 8. } else { 0. };
                let (s, c) = angle.sin_cos();
                let div = if square { s.abs().max(c.abs()) } else { 1. };
                m.vertices.push(DVec3::new(
                    c * self.size.x / (2. * div),
                    s * self.size.y / (2. * div),
                    z,
                ));
            }
        }
        let pointed = matches!(self.shape, Shape::Cone | Shape::Pyramid);
        if pointed {
            m.vertices.truncate(n);
        }
        let inner = m.vertices.len();
        if self.shape.hollow() {
            for i in 0..2 * n {
                let a = TAU * (i % n) as f64 / n as f64 + if square { TAU / 8. } else { 0. };
                m.vertices.push(DVec3::new(
                    self.bore * 0.5 * a.cos(),
                    self.bore * 0.5 * a.sin(),
                    if i < n { 0. } else { h },
                ));
            }
        } else {
            m.vertices.extend([DVec3::ZERO, DVec3::Z * h]);
        }
        for i in 0..n {
            let j = (i + 1) % n;
            if pointed {
                m.faces.push([i, j, inner + 1]);
            } else {
                m.faces.extend([[i, j, j + n], [i, j + n, i + n]]);
            }
            if self.shape.hollow() {
                let (a, b) = (inner + i, inner + j);
                m.faces.extend([
                    [i, a, b],
                    [i, b, j],
                    [i + n, j + n, b + n],
                    [i + n, b + n, a + n],
                    [a, b + n, b],
                    [a, a + n, b + n],
                ]);
            } else {
                m.faces.push([inner, j, i]);
                if !pointed {
                    m.faces.push([inner + 1, n + i, n + j]);
                }
            }
        }
        m
    }
}
impl Mesh {
    pub fn bounds(&self) -> (DVec3, DVec3) {
        self.vertices.iter().fold(
            (DVec3::splat(f64::INFINITY), DVec3::splat(f64::NEG_INFINITY)),
            |(lo, hi), &p| (lo.min(p), hi.max(p)),
        )
    }
}
