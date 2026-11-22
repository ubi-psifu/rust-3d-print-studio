//! Breitphase entlang X und Trennachsenprüfung für nicht benachbarte Dreiecke.
use super::{require, Result};
use crate::geometry::Mesh;
use glam::DVec3;
pub fn intersections(m: &Mesh) -> Result<()> {
    let mut boxes: Vec<_> = m
        .faces
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let [a, b, c] = f.map(|j| m.vertices[j]);
            (i, a.min(b).min(c), a.max(b).max(c))
        })
        .collect();
    boxes.sort_unstable_by(|a, b| a.1.x.total_cmp(&b.1.x));
    let mut tested = 0usize;
    let mut candidates = 0usize;
    for (k, &(i, lo, hi)) in boxes.iter().enumerate() {
        for &(j, b, c) in &boxes[k + 1..] {
            candidates += 1;
            require(
                candidates < 60_000_000,
                "Prüfaufwand überschritten; Tessellierung reduzieren",
            )?;
            if b.x > hi.x + 1e-9 {
                break;
            }
            if b.y > hi.y + 1e-9 || b.z > hi.z + 1e-9 || lo.y > c.y + 1e-9 || lo.z > c.z + 1e-9 {
                continue;
            }
            let f = m.faces[i];
            let g = m.faces[j];
            if f.iter().any(|x| g.contains(x)) {
                continue;
            }
            tested += 1;
            require(
                tested < 4_000_000,
                "Zu viele Dreieckspaare für die Selbstschnittprüfung",
            )?;
            if overlap(f.map(|n| m.vertices[n]), g.map(|n| m.vertices[n])) {
                return Err(format!(
                    "Selbstschnitt zwischen Flächen {} und {}",
                    i + 1,
                    j + 1
                ));
            }
        }
    }
    Ok(())
}
fn overlap(a: [DVec3; 3], b: [DVec3; 3]) -> bool {
    let e = [a[1] - a[0], a[2] - a[1], a[0] - a[2]];
    let f = [b[1] - b[0], b[2] - b[1], b[0] - b[2]];
    let n = e[0].cross(e[1]).normalize();
    let m = f[0].cross(f[1]).normalize();
    let separates = |axis: DVec3| {
        let len = axis.length();
        if len < 1e-14 {
            return false;
        }
        let axis = axis / len;
        let x = a.map(|p| (p - a[0]).dot(axis));
        let y = b.map(|p| (p - a[0]).dot(axis));
        let min = |v: [f64; 3]| v[0].min(v[1]).min(v[2]);
        let max = |v: [f64; 3]| v[0].max(v[1]).max(v[2]);
        max(x) < min(y) - 1e-10 || max(y) < min(x) - 1e-10
    };
    if separates(n) || separates(m) {
        return false;
    }
    for u in e {
        if separates(n.cross(u)) {
            return false;
        }
        for v in f {
            if separates(u.cross(v)) {
                return false;
            }
        }
    }
    !f.iter().any(|v| separates(m.cross(*v)))
}
