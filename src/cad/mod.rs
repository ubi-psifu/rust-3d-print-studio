//! Parametrische Features in Millimetern. Nur das abgeleitete Netz wird tesselliert.
pub mod accessories;
pub mod curve;
pub mod dimensions;
mod hook;
pub mod inspect;
mod modifiers;
pub use modifiers::clip;
pub mod references;
pub mod sketch;
mod sketch_solver;
pub mod surfaces;
use crate::geometry::Mesh;
use boolmesh::prelude::*;
use curve::Curve;
use glam::{DQuat, DVec2, DVec3, EulerRot};
pub use hook::{Hook, HookStyle};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, f64::consts::TAU};
pub type Result<T> = std::result::Result<T, String>;
fn require(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    Union,
    Difference,
    Intersection,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Feature {
    Case(accessories::Case),
    Relief(accessories::Relief),
    Driven {
        source: Box<Feature>,
        variables: std::collections::BTreeMap<String, f64>,
        bindings: Vec<dimensions::Binding>,
    },
    EdgeFinish {
        source: Box<Feature>,
        edge: usize,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reference: Option<references::EdgeRef>,
        r1: f64,
        r2: f64,
        steps: usize,
    },
    Sewn {
        patches: Vec<Vec<Vec<DVec3>>>,
        tolerance: f64,
    },
    Hook(Hook),
    SketchExtrude {
        sketch: sketch::Sketch,
        height: f64,
        end_scale: f64,
        steps: usize,
    },
    SmoothLoft {
        rings: Vec<Vec<DVec3>>,
        steps: usize,
        guide: Vec<DVec3>,
    },
    Shell {
        source: Box<Feature>,
        thickness: f64,
        open: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        side: Option<references::Side>,
    },
    Lattice {
        source: Box<Feature>,
        offsets: [DVec3; 8],
        symmetric: bool,
    },
    Cut {
        source: Box<Feature>,
        point: DVec3,
        normal: DVec3,
    },

    Primitive {
        shape: crate::geometry::Shape,
        size: DVec3,
        bore: f64,
    },
    Sweep {
        path: Curve,
        profile: Vec<DVec2>,
        steps: usize,
        up: DVec3,
        end_scale: f64,
        twist: f64,
        round_tip: bool,
        closed: bool,
    },
    Loft {
        rings: Vec<Vec<DVec3>>,
    },
    Extrude {
        profile: Vec<DVec3>,
        direction: DVec3,
        end_scale: f64,
    },
    Revolve {
        profile: Vec<DVec3>,
        origin: DVec3,
        axis: DVec3,
        angle: f64,
        steps: usize,
    },
    Boolean {
        operation: Operation,
        left: Box<Feature>,
        right: Box<Feature>,
    },
    Transform {
        source: Box<Feature>,
        translation: DVec3,
        rotation: DVec3,
        scale: DVec3,
    },
    Deform {
        source: Box<Feature>,
        twist: f64,
        taper: f64,
        bend: f64,
    },
    Polygon(Mesh),
}
impl Feature {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Case(c) => {
                if c.lid {
                    "Gehäusedeckel"
                } else {
                    "Gehäuse"
                }
            }
            Self::Relief(r) => {
                if matches!(r.source, accessories::Art::Text { .. }) {
                    "Schild und Anhänger"
                } else {
                    "SVG-Relief"
                }
            }
            Self::Primitive { shape, .. } => shape.label(),
            Self::SketchExtrude { .. } => "Skizzenextrusion",
            Self::SmoothLoft { .. } => "Geführter Loft",
            Self::Shell { .. } => "Aushöhlung",
            Self::Lattice { .. } => "Kontrollgitter",
            Self::Cut { .. } => "Ebenenschnitt",
            Self::EdgeFinish { .. } => "Kantenbearbeitung",
            Self::Driven { .. } => "Maßbeziehungen",
            Self::Sewn { .. } => "Flächenverbund",
            Self::Hook(h) => h.style.label(),
            Self::Sweep { .. } => "Pfadextrusion",
            Self::Loft { .. } => "Loft",
            Self::Extrude { .. } => "Extrusion",
            Self::Revolve { .. } => "Rotation",
            Self::Boolean { operation, .. } => match operation {
                Operation::Union => "Vereinigung",
                Operation::Difference => "Differenz",
                Operation::Intersection => "Schnittmenge",
            },
            Self::Transform { .. } => "Transformation",
            Self::Deform { .. } => "Verformung",
            Self::Polygon(_) => "Direktes Polygonnetz",
        }
    }
    pub fn build(&self) -> Result<Mesh> {
        std::panic::catch_unwind(|| {
            let mut budget = 128;
            let mut mesh = self.evaluate(&mut budget, 0)?;
            orient(&mut mesh);
            validate(&mesh)?;
            inspect::intersections(&mesh)?;
            Ok(mesh)
        })
        .map_err(|_| "Geometriekern konnte diese Schnittlage nicht berechnen".to_owned())?
    }
    fn evaluate(&self, budget: &mut usize, depth: usize) -> Result<Mesh> {
        require(*budget > 0 && depth <= 24, "Konstruktionsbaum zu groß")?;
        *budget -= 1;
        let mesh = match self {
            Self::Relief(r) => r.build()?,
            Self::Case(c) => c.build()?,
            Self::Primitive { shape, size, bore } => {
                let mut o = crate::geometry::Object::new(*shape, DVec3::ZERO);
                o.size = *size;
                o.bore = *bore;
                require(o.valid(), "Grundkörpermaße ungültig")?;
                o.mesh()
            }
            Self::SketchExtrude {
                sketch,
                height,
                end_scale,
                steps,
            } => {
                let mut profile = sketch.sample(*steps)?;
                require(
                    profile.len() > 3 && profile[0].distance(*profile.last().unwrap()) < 1e-6,
                    "Extrusion benötigt geschlossene Skizze",
                )?;
                profile.pop();
                Feature::Extrude {
                    profile,
                    direction: sketch.u.cross(sketch.v).normalize() * *height,
                    end_scale: *end_scale,
                }
                .evaluate(budget, depth + 1)?
            }
            Self::SmoothLoft {
                rings,
                steps,
                guide,
            } => modifiers::smooth(rings, *steps, guide)?,
            Self::Shell {
                source,
                thickness,
                open,
                side,
            } => modifiers::shell(
                source.evaluate(budget, depth + 1)?,
                *thickness,
                *open,
                *side,
            )?,
            Self::Lattice {
                source,
                offsets,
                symmetric,
            } => {
                let mut m = source.evaluate(budget, depth + 1)?;
                let (lo, hi) = m.bounds();
                let span = hi - lo;
                require(
                    span.min_element() > 1e-8,
                    "Kontrollgitter benötigt räumlichen Körper",
                )?;
                for v in &mut m.vertices {
                    let p = (*v - lo) / span;
                    let mut delta = DVec3::ZERO;
                    for i in 0..8 {
                        let w = |axis: usize| {
                            if i & (1 << axis) != 0 {
                                p[axis]
                            } else {
                                1. - p[axis]
                            }
                        };
                        let mut d = offsets[if *symmetric { i & !1 } else { i }];
                        if *symmetric && i & 1 != 0 {
                            d.x *= -1.;
                        }
                        delta += d * w(0) * w(1) * w(2);
                    }
                    *v += delta;
                }
                m
            }
            Self::Cut {
                source,
                point,
                normal,
            } => {
                let m = source.evaluate(budget, depth + 1)?;
                modifiers::clip(&m, *point, *normal)?
            }
            Self::Driven {
                source,
                variables,
                bindings,
            } => dimensions::drive(source, variables, bindings)?.evaluate(budget, depth + 1)?,
            Self::EdgeFinish {
                source,
                edge,
                reference,
                r1,
                r2,
                steps,
            } => modifiers::finish(
                source.evaluate(budget, depth + 1)?,
                *edge,
                reference.as_ref(),
                *r1,
                *r2,
                *steps,
            )?,
            Self::Sewn { patches, tolerance } => surfaces::sew(patches, *tolerance)?,
            Self::Hook(h) => h.features()?.evaluate(budget, depth + 1)?,
            Self::Sweep {
                path,
                profile,
                steps,
                up,
                end_scale,
                twist,
                round_tip,
                closed,
            } => {
                let mut pts = path.sample(*steps)?;
                if *closed && pts[0].distance(*pts.last().unwrap_or(&pts[0])) < 1e-7 {
                    pts.pop();
                }
                require(
                    (3..=128).contains(&profile.len())
                        && (0.01..=20.).contains(end_scale)
                        && twist.abs() <= 3600.
                        && pts.len() * profile.len() < 65000,
                    "Sweep-Profil, Auflösung oder Skalierung ungültig",
                )?;
                require(
                    !closed
                        || (!round_tip
                            && (*end_scale - 1.).abs() < 1e-9
                            && twist.abs() < 1e-9
                            && pts.len() >= 3),
                    "Geschlossener Sweep benötigt konstante Skalierung und offene Endoptionen",
                )?;
                let mut rings = Vec::new();
                let mut u = *up;
                let mut prev = None;
                let mut lengths = vec![0.];
                for w in pts.windows(2) {
                    let d = w[1].distance(w[0]);
                    require(d > 1e-7, "Pfad enthält doppelte Punkte")?;
                    lengths.push(lengths.last().copied().unwrap_or_default() + d);
                }
                for (i, &p) in pts.iter().enumerate() {
                    let last = pts.len() - 1;
                    let a = if i > 0 {
                        pts[i - 1]
                    } else if *closed {
                        pts[last]
                    } else {
                        p
                    };
                    let b = if i < last {
                        pts[i + 1]
                    } else if *closed {
                        pts[0]
                    } else {
                        p
                    };
                    let tangent = ((p - a).normalize_or_zero() + (b - p).normalize_or_zero())
                        .normalize_or_zero();
                    require(
                        tangent.length_squared() > 0.9,
                        "Pfad enthält eine Umkehrstelle",
                    )?;
                    if let Some(old) = prev {
                        u = DQuat::from_rotation_arc(old, tangent) * u;
                    }
                    u = (u - tangent * u.dot(tangent)).normalize_or_zero();
                    require(
                        u.length_squared() > 0.9,
                        "Profilachse parallel zur Pfadtangente",
                    )?;
                    prev = Some(tangent);
                    let t = lengths[i] / lengths[last];
                    let side = DQuat::from_axis_angle(tangent, (t * twist).to_radians()) * u;
                    let other = tangent.cross(side);
                    let scale = 1. + t * (end_scale - 1.);
                    let ring: Vec<_> = profile
                        .iter()
                        .map(|v| p + (side * v.x + other * v.y) * scale)
                        .collect();
                    if i == last && *round_tip {
                        let radius = profile.iter().map(|v| v.length()).fold(0., f64::max) * scale;
                        rings.push(ring.clone());
                        for k in 1..=6 {
                            let angle = k as f64 / 7. * TAU / 4.;
                            rings.push(
                                ring.iter()
                                    .map(|q| {
                                        p + (*q - p) * angle.cos() + tangent * radius * angle.sin()
                                    })
                                    .collect(),
                            );
                        }
                        let mut m = loft(&rings, false)?;
                        let offset = m.vertices.len() - profile.len();
                        m.faces.retain(|f| !f.iter().all(|j| *j >= offset));
                        let end = m.vertices.len();
                        m.vertices.push(p + tangent * radius);
                        for j in 0..profile.len() {
                            m.faces
                                .push([offset + j, offset + (j + 1) % profile.len(), end]);
                        }
                        orient(&mut m);
                        return Ok(m);
                    }
                    rings.push(ring);
                }
                loft(&rings, *closed)?
            }
            Self::Loft { rings } => loft(rings, false)?,
            Self::Extrude {
                profile,
                direction,
                end_scale,
            } => {
                require(
                    !profile.is_empty() && (0.01..=100.).contains(end_scale),
                    "Extrusionsprofil ungültig",
                )?;
                let center = profile.iter().sum::<DVec3>() / profile.len() as f64;
                loft(
                    &[
                        profile.clone(),
                        profile
                            .iter()
                            .map(|p| center + *direction + (*p - center) * *end_scale)
                            .collect(),
                    ],
                    false,
                )?
            }
            Self::Revolve {
                profile,
                origin,
                axis,
                angle,
                steps,
            } => {
                require(
                    (4..=96).contains(steps)
                        && axis.length() > 1e-8
                        && angle.abs() > 0.01
                        && angle.abs() <= 360.,
                    "Drehachse, Winkel oder Auflösung ungültig",
                )?;
                let close = angle.abs() == 360.;
                let rings = (0..steps + usize::from(!close))
                    .map(|i| {
                        let rot = DQuat::from_axis_angle(
                            axis.normalize(),
                            (angle * i as f64 / *steps as f64).to_radians(),
                        );
                        profile
                            .iter()
                            .map(|p| *origin + rot * (*p - *origin))
                            .collect()
                    })
                    .collect::<Vec<_>>();
                loft(&rings, close)?
            }
            Self::Boolean {
                operation,
                left,
                right,
            } => {
                let a = left.evaluate(budget, depth + 1)?;
                let b = right.evaluate(budget, depth + 1)?;
                boolean(&a, &b, *operation)?
            }
            Self::Transform {
                source,
                translation,
                rotation,
                scale,
            } => {
                require(
                    scale.is_finite() && scale.abs().min_element() > 0.00001,
                    "Skalierung darf nicht null sein",
                )?;
                let mut m = source.evaluate(budget, depth + 1)?;
                let r = *rotation * TAU / 360.;
                let q = DQuat::from_euler(EulerRot::ZYX, r.z, r.y, r.x);
                for p in &mut m.vertices {
                    *p = q * (*p * *scale) + *translation;
                }
                m
            }
            Self::Deform {
                source,
                twist,
                taper,
                bend,
            } => {
                require(
                    (0.02..=20.).contains(taper) && twist.abs() <= 3600. && bend.abs() <= 300.,
                    "Verformungsparameter ungültig",
                )?;
                let mut m = source.evaluate(budget, depth + 1)?;
                let (lo, hi) = m.bounds();
                let h = hi.z - lo.z;
                require(h > 1e-6, "Körperhöhe fehlt")?;
                let mid = (lo + hi) * 0.5;
                for p in &mut m.vertices {
                    let t = (p.z - lo.z) / h;
                    let v = DQuat::from_rotation_z((twist * t).to_radians())
                        * DVec3::new(p.x - mid.x, p.y - mid.y, 0.)
                        * (1. + (taper - 1.) * t);
                    let b = bend.to_radians();
                    let (x, z) = if b.abs() < 1e-7 {
                        (v.x, t * h)
                    } else {
                        let r = h / b;
                        let a = b * t;
                        (r - (r - v.x) * a.cos(), (r - v.x) * a.sin())
                    };
                    *p = DVec3::new(mid.x + x, mid.y + v.y, lo.z + z);
                }
                m
            }
            Self::Polygon(m) => {
                validate(m)?;
                m.clone()
            }
        };
        let mut mesh = mesh;
        orient(&mut mesh);
        Ok(mesh)
    }
}
fn volume(m: &Mesh) -> f64 {
    m.faces
        .iter()
        .map(|f| {
            let [a, b, c] = f.map(|i| m.vertices[i]);
            a.dot(b.cross(c)) / 6.
        })
        .sum()
}
fn orient(m: &mut Mesh) {
    if volume(m) < 0. {
        for f in &mut m.faces {
            f.swap(1, 2);
        }
    }
}
pub fn validate(m: &Mesh) -> Result<()> {
    require(
        !m.faces.is_empty()
            && m.faces.len() <= 150000
            && m.vertices
                .iter()
                .all(|p| p.is_finite() && p.abs().max_element() <= 20000.),
        "Leerer, zu großer oder numerisch ungültiger Körper",
    )?;
    let mut edges = HashMap::new();
    for &[a, b, c] in &m.faces {
        require(
            a < m.vertices.len() && b < m.vertices.len() && c < m.vertices.len(),
            "Ungültiger Flächenindex",
        )?;
        require(
            (m.vertices[b] - m.vertices[a])
                .cross(m.vertices[c] - m.vertices[a])
                .length()
                > 1e-12,
            "Entartete Dreiecksfläche",
        )?;
        for (i, j) in [(a, b), (b, c), (c, a)] {
            let e = edges.entry((i.min(j), i.max(j))).or_insert((0, 0));
            e.0 += 1;
            e.1 += if i < j { 1 } else { -1 };
        }
    }
    require(
        edges.values().all(|e| *e == (2, 0)) && volume(m).abs() > 1e-8,
        "Offene oder inkonsistent orientierte Oberfläche",
    )
}
fn boolean(a: &Mesh, b: &Mesh, op: Operation) -> Result<Mesh> {
    fn convert(m: &Mesh) -> Result<Manifold> {
        validate(m)?;
        Manifold::new(
            &m.vertices
                .iter()
                .flat_map(|p| p.to_array())
                .collect::<Vec<_>>(),
            &m.faces.iter().flatten().copied().collect::<Vec<_>>(),
        )
    }
    let m = compute_boolean(
        &convert(a)?,
        &convert(b)?,
        match op {
            Operation::Union => OpType::Add,
            Operation::Difference => OpType::Subtract,
            Operation::Intersection => OpType::Intersect,
        },
    )
    .map_err(|e| format!("Körperschnitt fehlgeschlagen: {e}"))?;
    require(m.nf > 0, "Operation ergibt ein leeres Volumen")?;
    Ok(Mesh {
        vertices: m
            .ps
            .iter()
            .map(|v| DVec3::from_array(v.to_array()))
            .collect(),
        faces: m
            .hs
            .chunks_exact(3)
            .map(|h| [h[0].tail, h[1].tail, h[2].tail])
            .collect(),
    })
}
fn loft(rings: &[Vec<DVec3>], closed: bool) -> Result<Mesh> {
    require(rings.len() >= 2, "Mindestens zwei Profile anlegen")?;
    let n = rings[0].len();
    require(
        (3..=256).contains(&n)
            && rings.len() * n <= 65000
            && rings
                .iter()
                .all(|r| r.len() == n && r.iter().all(|p| p.is_finite())),
        "Profile müssen gleich viele gültige Punkte besitzen",
    )?;
    let mut m = Mesh {
        vertices: rings.concat(),
        faces: vec![],
    };
    for k in 0..rings.len() - usize::from(!closed) {
        let a = k * n;
        let b = (k + 1) % rings.len() * n;
        for i in 0..n {
            let j = (i + 1) % n;
            m.faces
                .extend([[a + i, a + j, b + j], [a + i, b + j, b + i]]);
        }
    }
    if !closed {
        for (id, rev) in [(0, true), (rings.len() - 1, false)] {
            for f in cap(&rings[id])? {
                let [a, b, c] = f.map(|i| i + id * n);
                m.faces.push(if rev { [c, b, a] } else { [a, b, c] });
            }
        }
    }
    Ok(m)
}
// Ohrabschneiden für ebene, auch konkave Abschlusskonturen.
pub(crate) fn cap(r: &[DVec3]) -> Result<Vec<[usize; 3]>> {
    let normal = (0..r.len())
        .map(|i| (r[i] - r[0]).cross(r[(i + 1) % r.len()] - r[0]))
        .sum::<DVec3>()
        .normalize_or_zero();
    require(
        normal.length_squared() > 0.9 && r.iter().all(|p| (*p - r[0]).dot(normal).abs() < 1e-5),
        "Endprofil muss eben sein",
    )?;
    let xy: Vec<_> = r
        .iter()
        .map(|p| {
            let a = *p - r[0];
            let u = (r[1] - r[0]).normalize_or_zero();
            DVec2::new(a.dot(u), a.dot(normal.cross(u)))
        })
        .collect();
    let mut ix: Vec<_> = (0..r.len()).collect();
    let mut out = vec![];
    while ix.len() > 3 {
        let mut hit = None;
        for i in 0..ix.len() {
            let a = ix[(i + ix.len() - 1) % ix.len()];
            let b = ix[i];
            let c = ix[(i + 1) % ix.len()];
            let side = |u: usize, v: usize, w: usize| (xy[v] - xy[u]).perp_dot(xy[w] - xy[u]);
            if side(a, b, c) <= 1e-12 {
                continue;
            }
            if !ix.iter().any(|&j| {
                j != a
                    && j != b
                    && j != c
                    && side(a, b, j) >= -1e-12
                    && side(b, c, j) >= -1e-12
                    && side(c, a, j) >= -1e-12
            }) {
                out.push([a, b, c]);
                hit = Some(i);
                break;
            }
        }
        let i = hit.ok_or("Profil schneidet sich oder enthält doppelte Punkte")?;
        ix.remove(i);
    }
    out.push([ix[0], ix[1], ix[2]]);
    Ok(out)
}
pub fn ellipse(rx: f64, ry: f64, n: usize) -> Vec<DVec2> {
    (0..n)
        .map(|i| {
            let a = i as f64 * TAU / n as f64;
            DVec2::new(rx * a.cos(), ry * a.sin())
        })
        .collect()
}
