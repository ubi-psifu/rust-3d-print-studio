use super::{curve::Curve, require, Result};
use glam::DVec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RuleKind {
    Distance,
    Parallel,
    Tangent,
    Mirror,
    Coincident,
    Fixed,
    Horizontal,
    Vertical,
    Perpendicular,
    EqualLength,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub kind: RuleKind,
    pub ids: [usize; 4],
    pub value: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SegmentKind {
    Line,
    Bezier,
    Arc,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub kind: SegmentKind,
    pub ids: [usize; 4],
    pub construction: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sketch {
    pub points: Vec<DVec3>,
    pub origin: DVec3,
    pub u: DVec3,
    pub v: DVec3,
    pub planar: bool,
    #[serde(default)]
    pub bidirectional: bool,
    pub rules: Vec<Rule>,
    pub segments: Vec<Segment>,
}
impl Default for Sketch {
    fn default() -> Self {
        let pts = vec![
            DVec3::new(-14., -8., 0.),
            DVec3::new(14., -8., 0.),
            DVec3::new(14., 8., 0.),
            DVec3::new(-14., 8., 0.),
        ];
        Self {
            points: pts,
            origin: DVec3::ZERO,
            u: DVec3::X,
            v: DVec3::Y,
            planar: true,
            rules: vec![],
            bidirectional: true,
            segments: (0..4)
                .map(|i| Segment {
                    kind: SegmentKind::Line,
                    ids: [i, (i + 1) % 4, 0, 0],
                    construction: false,
                })
                .collect(),
        }
    }
}
impl Sketch {
    fn target(&self, r: &Rule, p: &[DVec3]) -> Result<(usize, DVec3)> {
        let [a, b, c, d] = r.ids;
        require(
            r.ids.iter().all(|i| *i < p.len()),
            "Bedingung verweist auf fehlenden Punkt",
        )?;
        let unit = |v: DVec3| {
            v.try_normalize()
                .ok_or("Richtung der Bedingung ist unbestimmt".to_owned())
        };
        Ok(match r.kind {
            RuleKind::Fixed
            | RuleKind::Horizontal
            | RuleKind::Vertical
            | RuleKind::Perpendicular
            | RuleKind::EqualLength => {
                return Err("Für diese Bedingung den gemeinsamen Solver aktivieren".into())
            }
            RuleKind::Distance => {
                require(r.value > 0. && r.value <= 10000., "Längenmaß ungültig")?;
                (b, p[a] + unit(p[b] - p[a])? * r.value)
            }
            RuleKind::Coincident => (b, p[a]),
            RuleKind::Parallel | RuleKind::Tangent => {
                let dir = unit(p[b] - p[a])?;
                let base = if matches!(r.kind, RuleKind::Tangent) {
                    b
                } else {
                    c
                };
                let old = p[d] - p[base];
                let sign = if old.dot(dir) < 0. { -1. } else { 1. };
                (d, p[base] + sign * dir * old.length())
            }
            RuleKind::Mirror => {
                let dir = unit(p[b] - p[a])?;
                let foot = p[a] + dir * dir.dot(p[c] - p[a]);
                (d, foot * 2. - p[c])
            }
        })
    }
    pub fn solved_points(&self) -> Result<Vec<DVec3>> {
        require(
            (2..=64).contains(&self.points.len())
                && self.segments.len() <= 96
                && self.rules.len() <= 64,
            "Skizzenumfang überschritten",
        )?;
        let mut p = self.points.clone();
        if self.planar {
            for a in &mut p {
                a.z = 0.;
            }
        }
        require(
            p.iter().all(|v| v.is_finite()),
            "Ungültige Punktkoordinaten",
        )?;
        if self.bidirectional {
            super::sketch_solver::solve(&mut p, &self.rules, self.planar)?;
            return Ok(p);
        }
        for _ in 0..32 {
            let mut delta: f64 = 0.;
            for rule in &self.rules {
                let (i, to) = self.target(rule, &p)?;
                delta = delta.max(p[i].distance(to));
                p[i] = to;
            }
            if delta < 1e-8 {
                break;
            }
        }
        for rule in &self.rules {
            let (i, to) = self.target(rule, &p)?;
            require(
                p[i].distance(to) < 1e-6,
                "Widersprüchliche Skizzenbedingungen",
            )?;
        }
        Ok(p)
    }
    pub fn sample(&self, steps: usize) -> Result<Vec<DVec3>> {
        let mut p = self.solved_points()?;
        let u = self.u.normalize_or_zero();
        let n = u.cross(self.v).normalize_or_zero();
        require(
            n.length_squared() > 0.9,
            "Arbeitsebene benötigt unabhängige Achsen",
        )?;
        let v = n.cross(u);
        for a in &mut p {
            *a = self.origin + u * a.x + v * a.y + n * a.z;
        }
        let mut out: Vec<DVec3> = vec![];
        for s in &self.segments {
            require(
                s.ids.iter().all(|i| *i < p.len()),
                "Segment verweist auf fehlenden Punkt",
            )?;
            if s.construction {
                continue;
            }
            let [a, b, c, d] = s.ids.map(|i| p[i]);
            let seg = match s.kind {
                SegmentKind::Line => vec![a, b],
                SegmentKind::Bezier => Curve::Bezier(vec![a, b, c, d]).sample(steps)?,
                SegmentKind::Arc => {
                    let x = b - a;
                    let y = c - a;
                    let norm = x.cross(y);
                    require(norm.length() > 1e-8, "Bogenpunkte liegen auf einer Geraden")?;
                    let center = a
                        + (y.cross(norm) * x.length_squared() + norm.cross(x) * y.length_squared())
                            / (2. * norm.length_squared());
                    let u = a - center;
                    let v = norm.normalize().cross(u);
                    let angle = |p: DVec3| {
                        (p - center)
                            .dot(v)
                            .atan2((p - center).dot(u))
                            .to_degrees()
                            .rem_euclid(360.)
                    };
                    let end = angle(c);
                    let angle = if angle(b) <= end { end } else { end - 360. };
                    Curve::Arc {
                        center,
                        u,
                        v,
                        start: 0.,
                        angle,
                    }
                    .sample(steps)?
                }
            };
            if let Some(prev) = out.last() {
                require(
                    prev.distance(seg[0]) < 1e-6,
                    "Kontursegmente sind nicht verbunden",
                )?;
                out.extend_from_slice(&seg[1..]);
            } else {
                out = seg;
            }
        }
        require(out.len() >= 2, "Skizze enthält keine Kontur")?;
        Ok(out)
    }
}
