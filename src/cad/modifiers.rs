use super::{boolean, loft, orient, require, Mesh, Operation, Result};
use glam::{DMat3, DVec3};
use std::collections::HashMap;
pub fn shell(mut m: Mesh, t: f64, open: bool) -> Result<Mesh> {
    let (lo, hi) = m.bounds();
    require(
        t > 0.01 && t < (hi - lo).min_element() / 2.,
        "Wandstärke außerhalb des gültigen Bereichs",
    )?;
    let count = m.vertices.len();
    let mut matrices = vec![DMat3::IDENTITY * 1e-9; count];
    let mut normals = vec![DVec3::ZERO; count];
    for f in &m.faces {
        let [a, b, c] = f.map(|i| m.vertices[i]);
        let n = (b - a).cross(c - a).normalize();
        let outer = DMat3::from_cols(n * n.x, n * n.y, n * n.z);
        for &i in f {
            matrices[i] += outer;
            if !(open && f.iter().all(|j| (m.vertices[*j].z - hi.z).abs() < 1e-7)) {
                normals[i] += n;
            }
        }
    }
    let inner: Vec<_> = (0..count)
        .map(|i| m.vertices[i] - matrices[i].inverse() * normals[i] * t)
        .collect();
    m.vertices.extend(inner);
    let faces = std::mem::take(&mut m.faces);
    let mut boundary = HashMap::new();
    let mut removed = 0;
    for f in faces {
        let [a, b, c] = f;
        if open && f.iter().all(|i| (m.vertices[*i].z - hi.z).abs() < 1e-7) {
            removed += 1;
            continue;
        }
        m.faces.extend([f, [a + count, c + count, b + count]]);
        if open {
            for (i, j) in [(a, b), (b, c), (c, a)] {
                if boundary.remove(&(j, i)).is_none() {
                    boundary.insert((i, j), ());
                }
            }
        }
    }
    if open {
        require(removed > 0, "Keine ebene obere Öffnungsfläche vorhanden")?;
        for (i, j) in boundary.into_keys() {
            m.faces
                .extend([[j, i, i + count], [j, i + count, j + count]]);
        }
    }
    Ok(m)
}
pub fn smooth(rings: &[Vec<DVec3>], steps: usize, guide: &[DVec3]) -> Result<Mesh> {
    require(
        (2..=32).contains(&rings.len())
            && (2..=24).contains(&steps)
            && rings.iter().all(|r| r.len() == rings[0].len())
            && (3..=128).contains(&rings[0].len())
            && rings.len() * rings[0].len() * steps <= 65000,
        "Loftprofile oder Auflösung ungültig",
    )?;
    require(
        guide.is_empty() || guide.len() == rings.len() - 1,
        "Ein Führungsversatz je Profilintervall erforderlich",
    )?;
    let mut out = vec![];
    for i in 0..rings.len() - 1 {
        for j in 0..steps {
            let t = j as f64 / steps as f64;
            let guide = guide.get(i).copied().unwrap_or_default() * (4. * t * (1. - t));
            let ring = (0..rings[0].len())
                .map(|k| {
                    let [a, b, c, d] =
                        [i.saturating_sub(1), i, i + 1, (i + 2).min(rings.len() - 1)]
                            .map(|s| rings[s][k]);
                    (2. * b
                        + (-a + c) * t
                        + (2. * a - 5. * b + 4. * c - d) * t * t
                        + (-a + 3. * b - 3. * c + d) * t.powi(3))
                        * 0.5
                        + guide
                })
                .collect();
            out.push(ring);
        }
    }
    out.push(rings.last().unwrap().clone());
    loft(&out, false)
}

pub fn clip(m: &Mesh, point: DVec3, normal: DVec3) -> Result<Mesh> {
    let n = normal.normalize_or_zero();
    require(n.length_squared() > 0.9, "Schnittebene ohne Normale")?;
    let u = n.any_orthonormal_vector();
    let v = n.cross(u);
    let r = m
        .vertices
        .iter()
        .map(|p| p.distance(point))
        .fold(1., f64::max)
        + 1.;
    let ring = vec![
        point + (-u - v) * r,
        point + (u - v) * r,
        point + (u + v) * r,
        point + (-u + v) * r,
    ];
    let mut tool = loft(
        &[ring.clone(), ring.iter().map(|p| *p + n * r).collect()],
        false,
    )?;
    orient(&mut tool);
    boolean(m, &tool, Operation::Intersection)
}
pub fn finish(mut m: Mesh, edge: usize, r1: f64, r2: f64, steps: usize) -> Result<Mesh> {
    require(
        (0.01..=100.).contains(&r1) && (0.01..=100.).contains(&r2) && (1..=16).contains(&steps),
        "Radien oder Auflösung ungültig",
    )?;
    let mut edges = std::collections::BTreeMap::<(usize, usize), Vec<DVec3>>::new();
    for f in &m.faces {
        let [a, b, c] = f.map(|i| m.vertices[i]);
        let n = (b - a).cross(c - a).normalize();
        for (i, j) in [(f[0], f[1]), (f[1], f[2]), (f[2], f[0])] {
            edges.entry((i.min(j), i.max(j))).or_default().push(n);
        }
    }
    let ((a, b), ns) = edges
        .into_iter()
        .filter(|(_, ns)| ns.len() == 2 && ns[0].dot(ns[1]) < 0.9999)
        .nth(edge)
        .ok_or("Kantennummer außerhalb der scharfen Kanten")?;
    let start = m.vertices[a];
    let delta = m.vertices[b] - start;
    let dir = delta.normalize();
    require(
        m.vertices
            .iter()
            .all(|p| ns.iter().all(|n| (*p - start).dot(*n) < 1e-6)),
        "Diese Kante benötigt einen lokalen B-Rep-Übergang; nur konvexe Stützkanten verfügbar",
    )?;
    let bis = (ns[0] + ns[1]).normalize();
    let half = ns[0].angle_between(ns[1]) / 2.;
    let side = (ns[1] - ns[0]).normalize();
    for i in 1..=steps {
        let angle = half * (2. * i as f64 / (steps + 1) as f64 - 1.);
        let n = bis * angle.cos() + side * angle.sin();
        let k = if steps == 1 {
            half.sin()
        } else {
            angle.cos() / half.cos() - 1.
        };
        let outward = n + dir * (k * (r2 - r1) / delta.length());
        m = clip(&m, start - n * (r1 * k), -outward)?;
    }
    Ok(m)
}
