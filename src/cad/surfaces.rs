use super::{require, Mesh, Result};
use glam::DVec3;
use std::collections::{HashMap, HashSet, VecDeque};
// Flächengitter werden zeilenweise trianguliert. Gemeinsame Ränder werden vernäht.
pub fn sew(patches: &[Vec<Vec<DVec3>>], tol: f64) -> Result<Mesh> {
    require(
        !patches.is_empty() && patches.len() <= 128,
        "Flächenverbund benötigt 1–128 Patches",
    )?;
    let mut mesh = Mesh::default();
    for grid in patches {
        require(
            (2..=96).contains(&grid.len())
                && (2..=96).contains(&grid[0].len())
                && grid.iter().all(|r| r.len() == grid[0].len()),
            "Patch muss ein rechteckiges Punktgitter bilden",
        )?;
        let start = mesh.vertices.len();
        let n = grid[0].len();
        require(start + grid.len() * n < 65000, "Zu viele Flächenpunkte")?;
        mesh.vertices.extend(grid.iter().flatten().copied());
        for y in 0..grid.len() - 1 {
            for x in 0..n - 1 {
                let k = start + y * n + x;
                mesh.faces
                    .extend([[k, k + 1, k + n + 1], [k, k + n + 1, k + n]]);
            }
        }
    }
    repair(&mesh, tol)
}
pub fn repair(m: &Mesh, tol: f64) -> Result<Mesh> {
    require(
        (1e-9..=0.5).contains(&tol) && m.vertices.len() <= 150000 && m.faces.len() <= 150000,
        "Schweißtoleranz oder Netzgröße ungültig",
    )?;
    let mut cells = HashMap::<[i64; 3], Vec<usize>>::new();
    let mut out = Mesh::default();
    let mut remap = vec![];
    for &p in &m.vertices {
        require(
            p.is_finite() && p.abs().max_element() <= 20000.,
            "Ungültiger Eckpunkt",
        )?;
        let [x, y, z] = p.to_array().map(|v| (v / tol).floor() as i64);
        let mut found = None;
        'near: for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                for k in z - 1..=z + 1 {
                    if let Some(ids) = cells.get(&[i, j, k]) {
                        for &v in ids {
                            if p.distance(out.vertices[v]) <= tol {
                                found = Some(v);
                                break 'near;
                            }
                        }
                    }
                }
            }
        }
        remap.push(found.unwrap_or_else(|| {
            let id = out.vertices.len();
            out.vertices.push(p);
            cells.entry([x, y, z]).or_default().push(id);
            id
        }));
    }
    let mut seen = HashSet::new();
    for &face in &m.faces {
        require(
            face.iter().all(|i| *i < remap.len()),
            "Ungültige Flächenreferenz",
        )?;
        let f = face.map(|i| remap[i]);
        let [a, b, c] = f.map(|i| out.vertices[i]);
        if (b - a).cross(c - a).length() <= 1e-12 {
            continue;
        }
        let mut key = f;
        key.sort();
        if seen.insert(key) {
            out.faces.push(f);
        }
    }
    let mut edges = HashMap::new();
    let mut adj = vec![Vec::new(); out.faces.len()];
    for (i, &[a, b, c]) in out.faces.iter().enumerate() {
        for (u, v) in [(a, b), (b, c), (c, a)] {
            let slot = edges.entry((u.min(v), u.max(v))).or_insert_with(Vec::new);
            slot.push((i, u < v));
        }
    }
    for list in edges.values() {
        require(
            list.len() == 2,
            "Offener oder mehrdeutiger Rand: Kontur manuell schließen",
        )?;
        let (i, a) = list[0];
        let (j, b) = list[1];
        adj[i].push((j, a == b));
        adj[j].push((i, a == b));
    }
    let mut flips = vec![None; adj.len()];
    for i in 0..adj.len() {
        if flips[i].is_some() {
            continue;
        }
        let mut q = VecDeque::from([(i, false)]);
        while let Some((j, flip)) = q.pop_front() {
            if let Some(prev) = flips[j] {
                require(prev == flip, "Flächenverbund nicht orientierbar")?;
                continue;
            }
            flips[j] = Some(flip);
            for &(k, opposite) in &adj[j] {
                q.push_back((k, flip ^ opposite));
            }
        }
    }
    for (f, flip) in out.faces.iter_mut().zip(flips) {
        if flip == Some(true) {
            f.swap(1, 2);
        }
    }
    super::orient(&mut out);
    super::validate(&out)?;
    Ok(out)
}
pub fn cube_patches() -> Vec<Vec<Vec<DVec3>>> {
    let p = |i: usize| {
        DVec3::new(
            if i & 1 == 0 { -10. } else { 10. },
            if i & 2 == 0 { -10. } else { 10. },
            if i & 4 == 0 { 0. } else { 20. },
        )
    };
    [
        [0, 1, 2, 3],
        [4, 5, 6, 7],
        [0, 1, 4, 5],
        [2, 3, 6, 7],
        [0, 2, 4, 6],
        [1, 3, 5, 7],
    ]
    .map(|[a, b, c, d]| vec![vec![p(a), p(b)], vec![p(c), p(d)]])
    .to_vec()
}
