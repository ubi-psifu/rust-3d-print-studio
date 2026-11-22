//! Export der unabhängigen Rust-Schichtplanung als maßstäbliche SVG-Zeichnungen.
use crate::{
    project::Project,
    section::{cut_mesh, line_fill, Contour},
};
use std::{
    io::{self, Write},
    path::Path,
};
use zip::{write::SimpleFileOptions, ZipWriter};
pub fn save(p: &Project, height: f64, density: u8, path: &Path) -> io::Result<usize> {
    if !p.valid()
        || p.objects.is_empty()
        || !(0.05..=1.).contains(&height)
        || density > 100
        || !path
            .extension()
            .is_some_and(|s| s.eq_ignore_ascii_case("zip"))
    {
        return Err(io::Error::other(
            "Gültige Platte, Schichtparameter und ZIP-Dateinamen angeben",
        ));
    }
    if path.exists() {
        return Err(io::Error::other("Ausgabedatei existiert bereits"));
    }
    let mut meshes = Vec::new();
    let mut top: f64 = 0.;
    for o in &p.objects {
        let mesh = o.mesh();
        let (lo, hi) = mesh.bounds();
        if let Some(w) = p.printer.warning(lo, hi) {
            return Err(io::Error::other(format!("{}: {w}", o.name)));
        }
        top = top.max(hi.z);
        meshes.push((o, mesh, lo.z, hi.z));
    }
    let n = (top / height - 1e-10).ceil().max(1.) as usize;
    if n > 10_000 {
        return Err(io::Error::other("Mehr als 10.000 Schichten"));
    }
    let mut temp = tempfile::NamedTempFile::new_in(
        path.parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new(".")),
    )?;
    let mut zip = ZipWriter::new(temp.as_file_mut());
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let [w, h] = p.printer.bed;
    let mut segments = 0;
    for layer in 1..=n {
        let lower = (layer - 1) as f64 * height;
        let upper = (layer as f64 * height).min(top);
        let z = (lower + upper) * 0.5;
        let mut contours = Vec::new();
        let mut count = 0;
        for (o, m, lo, hi) in &meshes {
            if z >= *lo && z < *hi {
                let lines = cut_mesh(m, z, &mut count).map_err(io::Error::other)?;
                if !lines.is_empty() {
                    contours.push(Contour {
                        slot: o.filament_slot,
                        color: o.color,
                        lines,
                    });
                }
            }
        }
        let fill = if density == 0 {
            Vec::new()
        } else {
            line_fill(
                &contours,
                p.printer.nozzle * 100. / f64::from(density),
                layer % 2 == 0,
            )
            .map_err(io::Error::other)?
        };
        segments += count + fill.iter().map(|c| c.lines.len()).sum::<usize>();
        if segments > 4_000_000 {
            return Err(io::Error::other(
                "Schichtplan überschreitet vier Millionen Segmente",
            ));
        }
        zip.start_file(format!("Schichten/{layer:05}.svg"), options)?;
        write!(
            zip,
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}mm" height="{h}mm" viewBox="0 0 {w} {h}"><title>Schicht {layer}, Z {z:.4} mm</title><g transform="translate(0 {h}) scale(1 -1)" fill="none">"#
        )?;
        for (name, group) in [("Konturen", &contours), ("Fuellung", &fill)] {
            write!(zip, r#"<g id="{name}">"#)?;
            for c in group {
                let [r, g, b] = c.color;
                write!(
                    zip,
                    r##"<path data-filament="{}" stroke="#{r:02x}{g:02x}{b:02x}" stroke-width="0.2" d=""##,
                    c.slot
                )?;
                for [a, b] in &c.lines {
                    write!(zip, "M{:.4} {:.4}L{:.4} {:.4}", a.x, a.y, b.x, b.y)?;
                }
                write!(zip, "\"/>")?;
            }
            write!(zip, "</g>")?;
        }
        write!(zip, "</g></svg>")?;
    }
    zip.start_file("Schichtplan.json", options)?;
    serde_json::to_writer(
        &mut zip,
        &serde_json::json!({"version":1,"projekt":p.name,"einheit":"mm","schichten":n,"schichthoehe":height,"linienfuellung_prozent":density,"duese":p.printer.nozzle,"modellhoehe":top,"druckbett":p.printer.bed}),
    )?;
    zip.finish()?;
    temp.as_file().sync_all()?;
    temp.persist_noclobber(path)?;
    Ok(n)
}
