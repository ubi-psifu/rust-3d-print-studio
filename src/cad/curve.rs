use super::{require, Result};
use glam::{DVec3, DVec4};
use serde::{Deserialize, Serialize};
use std::f64::consts::TAU;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Curve {
    Sketch(super::sketch::Sketch),
    Polyline(Vec<DVec3>),
    Bezier(Vec<DVec3>),
    Nurbs {
        points: Vec<DVec3>,
        weights: Vec<f64>,
        degree: usize,
        knots: Vec<f64>,
    },
    Arc {
        center: DVec3,
        u: DVec3,
        v: DVec3,
        start: f64,
        angle: f64,
    },
}
impl Curve {
    pub fn sample(&self, steps: usize) -> Result<Vec<DVec3>> {
        require(
            (4..=96).contains(&steps),
            "Auflösung muss zwischen 4 und 96 liegen",
        )?;
        let result = match self {
            Self::Sketch(s) => s.sample(steps)?,
            Self::Polyline(p) => {
                check(p)?;
                p.clone()
            }
            Self::Bezier(p) => {
                check(p)?;
                require(
                    (p.len() - 1) % 3 == 0,
                    "Bézierzug benötigt 3n+1 Kontrollpunkte",
                )?;
                let mut out = vec![p[0]];
                for c in p.windows(4).step_by(3) {
                    for i in 1..=steps {
                        let t = i as f64 / steps as f64;
                        let a = 1. - t;
                        out.push(
                            c[0] * a.powi(3)
                                + c[1] * (3. * a * a * t)
                                + c[2] * (3. * a * t * t)
                                + c[3] * t.powi(3),
                        );
                    }
                }
                out
            }
            Self::Nurbs {
                points,
                weights,
                degree,
                knots,
            } => {
                let n = points.len();
                let k = *degree;
                check(points)?;
                require(
                    (1..=5).contains(&k) && n > k && weights.len() == n && knots.len() == n + k + 1,
                    "Grad, Knoten und Gewichte der NURBS prüfen",
                )?;
                require(
                    weights.iter().all(|w| (0.0001..=10000.).contains(w))
                        && knots.iter().all(|v| v.is_finite())
                        && knots.windows(2).all(|w| w[0] <= w[1])
                        && knots[k] < knots[n],
                    "Ungültiger Knotenvektor oder Gewicht",
                )?;
                let mut out = Vec::new();
                for span in k..n {
                    if knots[span] == knots[span + 1] {
                        continue;
                    }
                    for i in usize::from(!out.is_empty())..=steps {
                        let t =
                            knots[span] + (knots[span + 1] - knots[span]) * i as f64 / steps as f64;
                        let mut d: Vec<DVec4> = (span - k..=span)
                            .map(|j| points[j].extend(1.) * weights[j])
                            .collect();
                        for r in 1..=k {
                            for j in (r..=k).rev() {
                                let a = span - k + j;
                                let den = knots[a + k - r + 1] - knots[a];
                                let w = if den > 0. { (t - knots[a]) / den } else { 0. };
                                d[j] = d[j - 1].lerp(d[j], w);
                            }
                        }
                        out.push(d[k].truncate() / d[k].w);
                    }
                }
                out
            }
            Self::Arc {
                center,
                u,
                v,
                start,
                angle,
            } => {
                require(
                    angle.abs() > 0.001 && angle.abs() <= 360. && u.cross(*v).length() > 1e-7,
                    "Bogenachsen oder Winkel ungültig",
                )?;
                (0..=steps)
                    .map(|i| {
                        let t = (start + angle * i as f64 / steps as f64) * TAU / 360.;
                        *center + *u * t.cos() + *v * t.sin()
                    })
                    .collect()
            }
        };
        require(
            result
                .iter()
                .all(|p| p.is_finite() && p.abs().max_element() <= 10000.),
            "Kurve außerhalb des Rechenbereichs",
        )?;
        Ok(result)
    }
}
fn check(p: &[DVec3]) -> Result<()> {
    require(
        (2..=64).contains(&p.len())
            && p.iter()
                .all(|v| v.is_finite() && v.abs().max_element() <= 10000.),
        "Skizze benötigt 2–64 gültige Kontrollpunkte",
    )
}
