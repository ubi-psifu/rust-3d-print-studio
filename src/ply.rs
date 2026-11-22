//! Geometrieaustausch als ASCII-PLY 1.0 mit indizierten Dreiecken.
use crate::{cad, geometry::Mesh, project::Project};
use glam::DVec3;
use std::io::{self, Write};

pub fn write(mut out: impl Write, p: &Project) -> io::Result<()> {
    let meshes: Vec<_> = p.objects.iter().map(|o| o.mesh()).collect();
    let nv: usize = meshes.iter().map(|m| m.vertices.len()).sum();
    let nf: usize = meshes.iter().map(|m| m.faces.len()).sum();
    writeln!(out, "ply\nformat ascii 1.0\ncomment Einheit Millimeter\nelement vertex {nv}\nproperty double x\nproperty double y\nproperty double z\nelement face {nf}\nproperty list uchar int vertex_indices\nend_header")?;
    for mesh in &meshes {
        for p in &mesh.vertices {
            writeln!(out, "{} {} {}", p.x, p.y, p.z)?;
        }
    }
    let mut base = 0;
    for mesh in &meshes {
        for &[a, b, c] in &mesh.faces {
            writeln!(out, "3 {} {} {}", base + a, base + b, base + c)?;
        }
        base += mesh.vertices.len();
    }
    Ok(())
}

pub fn read(data: &[u8]) -> Result<Mesh, String> {
    let mut lines = std::str::from_utf8(data)
        .map_err(|_| "ASCII-PLY erwartet")?
        .lines()
        .map(str::trim);
    if lines.next() != Some("ply") || lines.next() != Some("format ascii 1.0") {
        return Err("Nur ASCII-PLY 1.0 wird unterstützt".into());
    }
    let count = |s: &str| {
        s.parse::<usize>()
            .ok()
            .filter(|n| (1..=80000).contains(n))
            .ok_or("Ungültige PLY-Elementanzahl")
    };
    let (mut nv, mut nf) = (0, 0);
    let mut axes = Vec::new();
    let mut has_indices = false;
    loop {
        let line = lines.next().ok_or("Unvollständiger PLY-Header")?;
        let fields: Vec<_> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["comment" | "obj_info", ..] => (),
            ["element", "vertex", n] if nv == 0 && nf == 0 => nv = count(n)?,
            ["property", "float" | "double" | "float32" | "float64", axis @ ("x" | "y" | "z")]
                if nv > 0 && nf == 0 && !axes.contains(axis) =>
            {
                axes.push(*axis)
            }
            ["element", "face", n] if axes.len() == 3 && nf == 0 => nf = count(n)?,
            ["property", "list", "uchar" | "uint8", "int" | "int32" | "uint" | "uint32", "vertex_indices" | "vertex_index"]
                if nf > 0 && !has_indices =>
            {
                has_indices = true
            }
            ["end_header"] if has_indices => break,
            _ => return Err("Nicht unterstützte PLY-Eigenschaften oder Header-Reihenfolge".into()),
        }
    }
    let mut vertices = Vec::with_capacity(nv);
    for _ in 0..nv {
        let mut pt = DVec3::ZERO;
        let mut fields = lines
            .next()
            .ok_or("Fehlender PLY-Punkt")?
            .split_whitespace();
        for axis in &axes {
            let val = fields
                .next()
                .and_then(|v| v.parse::<f64>().ok())
                .filter(|v| v.is_finite())
                .ok_or("Ungültige PLY-Koordinate")?;
            pt[match *axis {
                "x" => 0,
                "y" => 1,
                _ => 2,
            }] = val;
        }
        if fields.next().is_some() {
            return Err("Zusätzliche PLY-Koordinaten".into());
        }
        vertices.push(pt);
    }
    let mut faces = Vec::with_capacity(nf);
    for _ in 0..nf {
        let mut fields = lines
            .next()
            .ok_or("Fehlende PLY-Fläche")?
            .split_whitespace();
        let n = fields
            .next()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|n| (3..=256).contains(n))
            .ok_or("Ungültige PLY-Flächengröße")?;
        let mut ids = Vec::with_capacity(n);
        for _ in 0..n {
            ids.push(
                fields
                    .next()
                    .and_then(|v| v.parse::<usize>().ok())
                    .filter(|i| *i < nv)
                    .ok_or("Ungültiger PLY-Punktindex")?,
            );
        }
        if fields.next().is_some() || faces.len() + n - 2 > 80000 {
            return Err("Ungültiger PLY-Flächenumfang".into());
        }
        if n == 3 {
            faces.push([ids[0], ids[1], ids[2]]);
        } else {
            let polygon: Vec<_> = ids.iter().map(|&i| vertices[i]).collect();
            faces.extend(
                cad::cap(&polygon)?
                    .into_iter()
                    .map(|tri| tri.map(|i| ids[i])),
            );
        }
    }
    if lines.any(|s| !s.is_empty()) {
        return Err("Zusätzliche PLY-Daten".into());
    }
    Ok(Mesh { vertices, faces })
}
