//! Lokale Projektion von Skizzenbedingungen in den Koordinaten der Arbeitsebene.
use super::sketch::{Rule, RuleKind};
use super::{require, Result};
use glam::DVec3;

impl Rule {
    pub fn point_count(&self) -> usize {
        match self.kind {
            RuleKind::Fixed => 1,
            RuleKind::Distance
            | RuleKind::Horizontal
            | RuleKind::Vertical
            | RuleKind::Coincident => 2,
            _ => 4,
        }
    }

    fn residual(&self, p: &[DVec3]) -> Result<DVec3> {
        let a = p[self.ids[0]];
        if self.kind == RuleKind::Fixed {
            return Ok(DVec3::ZERO);
        }
        let b = p[self.ids[1]];
        let ab = b - a;
        let unit = |v: DVec3| {
            v.try_normalize()
                .ok_or_else(|| "Eine Richtungsstrecke hat Länge null".to_string())
        };
        let scalar = |v| DVec3::new(v, 0., 0.);
        Ok(match self.kind {
            RuleKind::Distance => {
                require(
                    (0.0..=10000.).contains(&self.value) && self.value > 0.,
                    "Längenmaß ungültig",
                )?;
                scalar(ab.length() - self.value)
            }
            RuleKind::Horizontal => DVec3::new(ab.y, ab.z, 0.),
            RuleKind::Vertical => DVec3::new(ab.x, ab.z, 0.),
            RuleKind::Coincident => ab,
            kind => {
                let d = p[self.ids[3]];
                if kind == RuleKind::Tangent {
                    // AB und BD sind die beiden Tangentengriffe am gemeinsamen Punkt B.
                    let dir = unit(ab)?;
                    let handle = d - b;
                    unit(handle)?;
                    return Ok(handle - dir * handle.dot(dir));
                }
                let c = p[self.ids[2]];
                let cd = d - c;
                match kind {
                    RuleKind::EqualLength => scalar(ab.length() - cd.length()),
                    RuleKind::Perpendicular => {
                        unit(cd)?;
                        scalar(unit(ab)?.dot(cd))
                    }
                    RuleKind::Parallel => {
                        let u = unit(ab)?;
                        unit(cd)?;
                        cd - u * cd.dot(u)
                    }
                    RuleKind::Mirror => {
                        let u = unit(ab)?;
                        d + c - 2. * (a + u * (c - a).dot(u))
                    }
                    _ => unreachable!(),
                }
            }
        })
    }
}

pub(super) fn solve(p: &mut [DVec3], rules: &[Rule], planar: bool) -> Result<()> {
    const TOL: f64 = 1e-8;
    let mut fixed = vec![false; p.len()];
    for rule in rules {
        require(
            rule.ids[..rule.point_count()].iter().all(|i| *i < p.len()),
            "Bedingung verweist auf fehlenden Punkt",
        )?;
        if rule.kind == RuleKind::Fixed {
            fixed[rule.ids[0]] = true;
        }
    }
    for _ in 0..160 {
        for r in rules {
            for axis in 0..3 {
                let residual = r.residual(p)?[axis];
                if residual.abs() < TOL {
                    continue;
                }
                let mut gradient = [(0usize, DVec3::ZERO); 4];
                let indices = &r.ids[..r.point_count()];
                let mut norm = 0.;
                for (k, &id) in indices.iter().enumerate() {
                    if fixed[id] || indices[..k].contains(&id) {
                        continue;
                    }
                    let mut g = DVec3::ZERO;
                    for dim in 0..if planar { 2 } else { 3 } {
                        let pos = p[id][dim];
                        let h = 1e-6 * pos.abs().max(1.);
                        p[id][dim] = pos + h;
                        let plus = r.residual(p);
                        p[id][dim] = pos - h;
                        let minus = r.residual(p);
                        p[id][dim] = pos;
                        g[dim] = (plus?[axis] - minus?[axis]) / (2. * h);
                    }
                    norm += g.length_squared();
                    gradient[k] = (id, g);
                }
                if norm > 1e-14 {
                    for (id, g) in gradient {
                        p[id] -= g * (residual / norm);
                    }
                }
            }
        }
        let mut satisfied = true;
        for rule in rules {
            if rule.residual(p)?.abs().max_element() > TOL {
                satisfied = false;
                break;
            }
        }
        if satisfied && p.iter().all(|v| v.is_finite()) {
            return Ok(());
        }
    }
    for (index, rule) in rules.iter().enumerate() {
        if rule.residual(p)?.abs().max_element() > TOL {
            return Err(format!(
                "Bedingung {} bleibt unerfüllt. Fixierungen, Maße und Ausgangslage prüfen.",
                index + 1
            ));
        }
    }
    Err("Skizzenlösung ist numerisch ungültig".into())
}
