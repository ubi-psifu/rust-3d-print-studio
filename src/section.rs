//! Horizontale Schnittkanten in Weltkoordinaten. Eine gemeinsame Kante gehört zur oberen Seite.
use crate::project::Project;
use glam::{DVec2, DVec3};
pub struct Contour {
    pub slot: usize,
    pub color: [u8; 3],
    pub lines: Vec<[DVec2; 2]>,
}
pub fn intersect(p: &Project, z: f64) -> Result<Vec<Contour>, String> {
    let mut result = Vec::new();
    let mut count = 0;
    for object in &p.objects {
        let mesh = object.mesh();
        let lines = cut_mesh(&mesh, z, &mut count)?;
        if !lines.is_empty() {
            result.push(Contour {
                slot: object.filament_slot,
                color: object.color,
                lines,
            });
        }
    }
    Ok(result)
}

pub(crate) fn cut_mesh(
    mesh: &crate::geometry::Mesh,
    z: f64,
    count: &mut usize,
) -> Result<Vec<[DVec2; 2]>, String> {
    let mut lines = Vec::new();
    for f in &mesh.faces {
        let v = f.map(|i| mesh.vertices[i]);
        let mut hits = [DVec3::ZERO; 2];
        let mut n = 0;
        for (a, b) in [(v[0], v[1]), (v[1], v[2]), (v[2], v[0])] {
            if (a.z <= z) != (b.z <= z) {
                hits[n] = a.lerp(b, (z - a.z) / (b.z - a.z));
                n += 1;
            }
        }
        if n == 2 && hits[0].distance_squared(hits[1]) > 1e-16 {
            *count += 1;
            if *count > 120_000 {
                return Err("Schnitt überschreitet 120.000 Segmente".into());
            }
            lines.push(hits.map(|q| q.truncate()));
        }
    }
    Ok(lines)
}

/// Abwechselndes rechtwinkliges Linienraster. Die Gerade/Ungerade-Regel erhält Innenkonturen.
pub fn line_fill(contours: &[Contour], pitch: f64, vertical: bool) -> Result<Vec<Contour>, String> {
    if !pitch.is_finite() || pitch < 0.05 {
        return Err("Ungültiger Linienabstand".into());
    }
    let axes = |p: DVec2| if vertical { DVec2::new(p.y, p.x) } else { p };
    let mut output = Vec::new();
    let mut count = 0;
    for c in contours {
        let mut rows = std::collections::BTreeMap::<i64, Vec<f64>>::new();
        for [a, b] in &c.lines {
            let (mut a, mut b) = (axes(*a), axes(*b));
            if a.y > b.y {
                std::mem::swap(&mut a, &mut b)
            }
            let start = (a.y / pitch - 0.5).ceil() as i64;
            let end = (b.y / pitch - 0.5).ceil() as i64;
            for row in start..end {
                count += 1;
                if count > 240_000 {
                    return Err("Füllmuster überschreitet das Vorschaulimit".into());
                }
                let y = (row as f64 + 0.5) * pitch;
                rows.entry(row)
                    .or_default()
                    .push(a.x + (b.x - a.x) * (y - a.y) / (b.y - a.y));
            }
        }
        let mut lines = Vec::new();
        for (row, mut hits) in rows {
            if hits.len() % 2 != 0 {
                return Err("Offene Kontur in der Schicht".into());
            }
            hits.sort_by(f64::total_cmp);
            let y = (row as f64 + 0.5) * pitch;
            for pair in hits.chunks_exact(2) {
                if pair[1] - pair[0] > 1e-8 {
                    lines.push([axes(DVec2::new(pair[0], y)), axes(DVec2::new(pair[1], y))]);
                }
            }
        }
        output.push(Contour {
            slot: c.slot,
            color: c.color,
            lines,
        });
    }
    Ok(output)
}
