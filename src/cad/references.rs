//! Geometrische Bezüge im lokalen, auf die Hüllbox normierten Koordinatensystem.
use super::{require, Mesh, Result};
use glam::DVec3;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

type Edge = ((usize, usize), [DVec3; 2]);
pub(super) fn sharp(m: &Mesh) -> Vec<Edge> {
    let mut edges = BTreeMap::<(usize, usize), Vec<DVec3>>::new();
    for f in &m.faces {
        let [a, b, c] = f.map(|i| m.vertices[i]);
        let n = (b - a).cross(c - a).normalize();
        for (i, j) in [(f[0], f[1]), (f[1], f[2]), (f[2], f[0])] {
            edges.entry((i.min(j), i.max(j))).or_default().push(n);
        }
    }
    edges
        .into_iter()
        .filter_map(|(key, n)| match n.as_slice() {
            [a, b] if a.dot(*b) < 0.9999 => Some((key, [*a, *b])),
            _ => None,
        })
        .collect()
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeRef {
    ends: [DVec3; 2],
    normals: [DVec3; 2],
}
impl EdgeRef {
    fn signature(m: &Mesh, (lo, hi): (DVec3, DVec3), edge: Edge) -> Self {
        let span = hi - lo;
        let ((a, b), normals) = edge;
        Self {
            ends: [a, b].map(|i| (m.vertices[i] - lo) / span),
            normals: normals.map(|n| (n * span).normalize()),
        }
    }
    pub fn bind(m: &Mesh, index: usize) -> Result<Self> {
        let edges = sharp(m);
        let e = *edges
            .get(index)
            .ok_or("Scharfe Kante ist nicht vorhanden")?;
        let reference = Self::signature(m, m.bounds(), e);
        reference.resolve(m, &edges)?;
        Ok(reference)
    }
    pub(super) fn resolve(&self, m: &Mesh, edges: &[Edge]) -> Result<Edge> {
        let bounds = m.bounds();
        let close = |a: DVec3, b: DVec3| a.distance(b) <= 1e-6;
        let mut result = None;
        for &edge in edges {
            let candidate = Self::signature(m, bounds, edge);
            let [n, k] = candidate.normals;
            if !((close(n, self.normals[0]) && close(k, self.normals[1]))
                || (close(k, self.normals[0]) && close(n, self.normals[1])))
            {
                continue;
            }
            let [a, b] = candidate.ends;
            let ((i, j), ns) = edge;
            let hit = if close(a, self.ends[0]) && close(b, self.ends[1]) {
                ((i, j), ns)
            } else if close(b, self.ends[0]) && close(a, self.ends[1]) {
                ((j, i), ns)
            } else {
                continue;
            };
            require(
                result.is_none(),
                "Mehrdeutiger Kantenbezug. Kante erneut binden.",
            )?;
            result = Some(hit);
        }
        result.ok_or_else(|| {
            "Gebundene Kante fehlt oder hat ihre Form geändert. Kante erneut binden.".into()
        })
    }
}

/// Ebene und ein innerer Punkt im normierten lokalen Koordinatensystem.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaneRef {
    normal: DVec3,
    point: DVec3,
}

impl PlaneRef {
    pub fn bind(mesh: &Mesh, triangle: usize) -> Result<Self> {
        let face = mesh
            .faces
            .get(triangle)
            .ok_or("Dreiecksnummer fehlt im Außenkörper")?;
        let (lo, hi) = mesh.bounds();
        let [a, b, c] = face.map(|i| (mesh.vertices[i] - lo) / (hi - lo));
        let normal = (b - a)
            .cross(c - a)
            .try_normalize()
            .ok_or("Fläche ist entartet")?;
        let reference = Self {
            normal,
            point: (a + b + c) / 3.,
        };
        reference.patch(mesh)?;
        Ok(reference)
    }

    fn patch(self, m: &Mesh) -> Result<Vec<bool>> {
        const EPS: f64 = 1e-7;
        let (lo, hi) = m.bounds();
        let mut anchored = false;
        let mask = m
            .faces
            .iter()
            .map(|f| {
                let tri = f.map(|i| (m.vertices[i] - lo) / (hi - lo));
                let [a, b, c] = tri;
                let n = (b - a).cross(c - a).normalize();
                let matches = n.distance(self.normal) < EPS
                    && tri
                        .iter()
                        .all(|p| (*p - self.point).dot(self.normal).abs() < EPS);
                if matches {
                    anchored |= [(a, b), (b, c), (c, a)].iter().all(|(p, q)| {
                        let e = *q - *p;
                        e.cross(self.point - *p).dot(n) >= -EPS * e.length()
                    });
                }
                matches
            })
            .collect();
        require(
            anchored,
            "Gebundene Öffnungsfläche fehlt oder hat sich verändert. Fläche erneut binden.",
        )?;
        connected_patch(m, mask)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
    Front,
    Back,
    Bottom,
    Top,
    Plane(PlaneRef),
}
impl Side {
    pub const ALL: [(Self, &'static str); 6] = [
        (Self::Left, "Links (−X)"),
        (Self::Right, "Rechts (+X)"),
        (Self::Front, "Vorne (−Y)"),
        (Self::Back, "Hinten (+Y)"),
        (Self::Bottom, "Unten (−Z)"),
        (Self::Top, "Oben (+Z)"),
    ];
    pub(super) fn patch(self, m: &Mesh) -> Result<Vec<bool>> {
        let (axis, upper) = match self {
            Self::Plane(reference) => return reference.patch(m),
            Self::Left => (0, false),
            Self::Right => (0, true),
            Self::Front => (1, false),
            Self::Back => (1, true),
            Self::Bottom => (2, false),
            Self::Top => (2, true),
        };
        let (lo, hi) = m.bounds();
        let v = if upper { hi[axis] } else { lo[axis] };
        let mask: Vec<_> = m
            .faces
            .iter()
            .map(|f| f.iter().all(|&i| (m.vertices[i][axis] - v).abs() < 1e-7))
            .collect();
        connected_patch(m, mask)
    }
}

fn connected_patch(m: &Mesh, mask: Vec<bool>) -> Result<Vec<bool>> {
    let first = mask
        .iter()
        .position(|&b| b)
        .ok_or("Gewählte Außenseite enthält keine ebene Öffnungsfläche")?;
    let mut edge_face = HashMap::new();
    let mut adj = vec![vec![]; mask.len()];
    for (i, &f) in m.faces.iter().enumerate().filter(|(i, _)| mask[*i]) {
        for (a, b) in [(f[0], f[1]), (f[1], f[2]), (f[2], f[0])] {
            if let Some(other) = edge_face.insert((a.min(b), a.max(b)), i) {
                adj[i].push(other);
                adj[other].push(i);
            }
        }
    }
    let mut seen = vec![false; mask.len()];
    let mut stack = vec![first];
    while let Some(i) = stack.pop() {
        if !seen[i] {
            seen[i] = true;
            stack.extend(&adj[i]);
        }
    }
    require(
        seen == mask,
        "Gewählte Außenseite enthält mehrere getrennte Flächen",
    )?;
    Ok(mask)
}
