//! Belegung in Bettkoordinaten anhand der gedrehten XY-Hüllboxen.
use crate::{geometry::Object, project::Printer};
use glam::{DVec2, DVec3};

#[derive(Clone, Copy, Debug)]
pub struct Footprint {
    pub lo: DVec2,
    pub hi: DVec2,
}
impl Footprint {
    fn object(o: &Object) -> Self {
        let (a, b) = o.mesh().bounds();
        Self {
            lo: a.truncate(),
            hi: b.truncate(),
        }
    }
    fn translate(self, d: DVec2) -> Self {
        Self {
            lo: self.lo + d,
            hi: self.hi + d,
        }
    }
    fn pad(self, d: f64) -> Self {
        Self {
            lo: self.lo - DVec2::splat(d),
            hi: self.hi + DVec2::splat(d),
        }
    }
}
struct Plate {
    size: DVec2,
    blocked: Vec<Footprint>,
    gap: f64,
}
impl Plate {
    fn new(p: &Printer, gap: f64) -> Self {
        Self {
            size: DVec2::from_array(p.bed),
            gap,
            blocked: p
                .excluded
                .iter()
                .map(|&[a, b, c, d]| Footprint {
                    lo: DVec2::new(a, b),
                    hi: DVec2::new(c, d),
                })
                .collect(),
        }
    }
    fn occupy(&mut self, r: Footprint) {
        self.blocked.push(r.pad(self.gap));
    }
    // Bei jeder Hindernisoberkante beginnt ein möglicher neuer Streifen.
    fn next(&self, size: DVec2) -> Option<DVec2> {
        const EPS: f64 = 1e-8;
        let mut rows = vec![0.];
        rows.extend(self.blocked.iter().map(|r| r.hi.y).filter(|y| *y > 0.));
        rows.sort_by(f64::total_cmp);
        rows.dedup();
        for y in rows {
            if y + size.y > self.size.y + EPS {
                break;
            }
            let mut spans: Vec<_> = self
                .blocked
                .iter()
                .filter(|r| r.lo.y < y + size.y - EPS && r.hi.y > y + EPS)
                .map(|r| (r.lo.x, r.hi.x))
                .collect();
            spans.sort_by(|a, b| a.0.total_cmp(&b.0));
            let mut x: f64 = 0.;
            for (left, right) in spans {
                if x + size.x <= left + EPS {
                    break;
                }
                x = x.max(right);
            }
            if x + size.x <= self.size.x + EPS {
                return Some(DVec2::new(x, y));
            }
        }
        None
    }
}
pub fn copy_offsets(
    printer: &Printer,
    existing: &[Object],
    group: &[Object],
    gap: f64,
    limit: usize,
) -> Vec<DVec2> {
    if group.is_empty() {
        return vec![];
    }
    let mut plate = Plate::new(printer, gap);
    for o in existing {
        plate.occupy(Footprint::object(o));
    }
    let boxes: Vec<_> = group.iter().map(Footprint::object).collect();
    let mut envelope = boxes[0];
    for b in &boxes[1..] {
        envelope.lo = envelope.lo.min(b.lo);
        envelope.hi = envelope.hi.max(b.hi);
    }
    let mut result = vec![];
    for _ in 0..limit {
        let Some(at) = plate.next(envelope.hi - envelope.lo) else {
            break;
        };
        let delta = at - envelope.lo;
        result.push(delta);
        plate.occupy(envelope.translate(delta));
    }
    result
}
pub fn arrange(printer: &Printer, objects: &[Object], gap: f64) -> Option<Vec<DVec3>> {
    let mut plate = Plate::new(printer, gap);
    let bounds: Vec<_> = objects.iter().map(|o| o.mesh().bounds()).collect();
    let mut order: Vec<_> = (0..objects.len()).collect();
    order.sort_by(|&i, &j| {
        let span = |k: usize| {
            let d = bounds[k].1 - bounds[k].0;
            d.x * d.y
        };
        span(j).total_cmp(&span(i))
    });
    let mut result = vec![DVec3::ZERO; objects.len()];
    for i in order {
        let (a, b) = bounds[i];
        if b.z - a.z > printer.print_z + 1e-7 {
            return None;
        }
        let size = (b - a).truncate();
        let at = plate.next(size)?;
        result[i] = (at - a.truncate()).extend(-a.z);
        plate.occupy(Footprint {
            lo: at,
            hi: at + size,
        });
    }
    Some(result)
}
