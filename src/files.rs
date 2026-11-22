use crate::project::Project;
use std::{
    fs::File,
    io::{self, BufWriter, Read, Write},
    path::Path,
};
use tempfile::NamedTempFile;
use zip::{write::SimpleFileOptions, ZipWriter};

const LIMIT: u64 = 2 * 1024 * 1024;
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
pub fn read_limited(path: &Path) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    File::open(path)?.take(LIMIT + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > LIMIT {
        return Err(invalid("Datei überschreitet 2 MiB"));
    }
    Ok(bytes)
}
pub fn load(path: &Path) -> io::Result<Project> {
    let mut p: Project = serde_json::from_slice(&read_limited(path)?)
        .map_err(|e| invalid(&format!("Ungültige Projektdatei: {e}")))?;
    if p.version == 1 && p.objects.iter().all(|o| o.feature.is_none()) {
        p.version = 2;
    }
    if p.version != 2 || p.objects.len() > crate::project::MAX_OBJECTS {
        return Err(invalid("Projektversion oder Objektanzahl ungültig"));
    }
    for o in &mut p.objects {
        o.rebuild().map_err(|e| invalid(&e))?;
    }
    if !p.valid() {
        return Err(invalid(
            "Unbekannte Version oder unzulässige Projektparameter",
        ));
    }
    Ok(p)
}
#[derive(Clone, Copy, PartialEq)]
pub enum Output {
    Project,
    Stl,
    StlAscii,
    Ply,
    ThreeMf,
    Obj,
}
impl Output {
    pub fn ext(self) -> &'static str {
        match self {
            Self::Project => "r3dp",
            Self::Stl | Self::StlAscii => "stl",
            Self::Ply => "ply",
            Self::ThreeMf => "3mf",
            Self::Obj => "obj",
        }
    }
}
pub fn save(path: &Path, p: &Project, format: Output, overwrite: bool) -> io::Result<()> {
    if !p.valid() {
        return Err(invalid("Ungültige Projektparameter"));
    }
    if format != Output::Project && p.objects.is_empty() {
        return Err(invalid("Projekt enthält keine Körper"));
    }
    if !path
        .extension()
        .is_some_and(|s| s.eq_ignore_ascii_case(format.ext()))
    {
        return Err(invalid("Dateiendung passt nicht zum gewählten Format"));
    }
    let dir = path
        .parent()
        .filter(|d| !d.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut temp = NamedTempFile::new_in(dir)?;
    {
        let mut out = BufWriter::new(temp.as_file_mut());
        match format {
            Output::Project => {
                let data = serde_json::to_vec_pretty(p)?;
                if data.len() as u64 > LIMIT {
                    return Err(invalid("Projekt überschreitet die Lesegrenze von 2 MiB"));
                }
                out.write_all(&data)?;
            }
            Output::StlAscii => stl_ascii(&mut out, p)?,
            Output::Ply => crate::ply::write(&mut out, p)?,
            Output::Stl => stl(&mut out, p)?,
            Output::Obj => obj(&mut out, p)?,
            Output::ThreeMf => three_mf(&mut out, p, None)?,
        }
        out.flush()?;
    }
    temp.as_file().sync_all()?;
    if overwrite {
        temp.persist(path)?;
    } else {
        temp.persist_noclobber(path)?;
    }
    Ok(())
}
fn stl(mut out: impl Write, p: &Project) -> io::Result<()> {
    let meshes: Vec<_> = p.objects.iter().map(|o| o.mesh()).collect();
    out.write_all(&[0; 80])?;
    out.write_all(&(meshes.iter().map(|m| m.faces.len()).sum::<usize>() as u32).to_le_bytes())?;
    for m in meshes {
        for f in m.faces {
            let [a, b, c] = f.map(|i| m.vertices[i]);
            for v in [(b - a).cross(c - a).normalize(), a, b, c] {
                for value in v.to_array() {
                    out.write_all(&(value as f32).to_le_bytes())?;
                }
            }
            out.write_all(&[0; 2])?;
        }
    }
    Ok(())
}
fn stl_ascii(mut out: impl Write, p: &Project) -> io::Result<()> {
    writeln!(out, "solid Platte")?;
    for object in &p.objects {
        let mesh = object.mesh();
        for face in mesh.faces {
            let [a, b, c] = face.map(|i| mesh.vertices[i]);
            let n = (b - a).cross(c - a).normalize_or_zero();
            writeln!(
                out,
                "  facet normal {} {} {}\n    outer loop",
                n.x, n.y, n.z
            )?;
            for v in [a, b, c] {
                writeln!(out, "      vertex {} {} {}", v.x, v.y, v.z)?;
            }
            writeln!(out, "    endloop\n  endfacet")?;
        }
    }
    writeln!(out, "endsolid Platte")
}
fn obj(mut out: impl Write, p: &Project) -> io::Result<()> {
    let mut base = 1;
    for (i, o) in p.objects.iter().enumerate() {
        writeln!(out, "o Koerper_{}", i + 1)?;
        let m = o.mesh();
        for v in &m.vertices {
            writeln!(out, "v {} {} {}", v.x, v.y, v.z)?;
        }
        for [a, b, c] in m.faces {
            writeln!(out, "f {} {} {}", a + base, b + base, c + base)?;
        }
        base += m.vertices.len();
    }
    Ok(())
}
pub(crate) fn xml_text(s: &str) -> String {
    s.chars()
        .filter(|&c| c >= ' ' && c != '\u{fffe}' && c != '\u{ffff}')
        .collect::<String>()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
pub(crate) fn three_mf(
    out: impl Write + io::Seek,
    p: &Project,
    settings: Option<&crate::slicer::Preparation>,
) -> io::Result<()> {
    let mut z = ZipWriter::new(out);
    let opt = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    z.start_file("[Content_Types].xml", opt)?;
    z.write_all(br#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="config" ContentType="application/octet-stream"/><Default Extension="model" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/></Types>"#)?;
    z.start_file("_rels/.rels", opt)?;
    z.write_all(br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Target="/3D/3dmodel.model" Id="rel0" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/></Relationships>"#)?;
    z.start_file("3D/3dmodel.model", opt)?;
    write!(
        z,
        r#"<model unit="millimeter" xml:lang="de-DE" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02" xmlns:BambuStudio="http://schemas.bambulab.com/package/2021">"#
    )?;
    if let Some(settings) = settings {
        write!(
            z,
            r#"<metadata name="Application">{}</metadata><metadata name="BambuStudio:3mfVersion">1</metadata><metadata name="Designer">Rust 3D Print Studio</metadata>"#,
            settings.application()
        )?;
    }
    write!(z, "<resources>")?;
    for (i, o) in p.objects.iter().enumerate() {
        let m = o.mesh();
        write!(
            z,
            r#"<object id="{}" type="model" name="{}"><mesh><vertices>"#,
            i + 1,
            xml_text(&o.name)
        )?;
        for v in m.vertices {
            write!(z, r#"<vertex x="{}" y="{}" z="{}"/>"#, v.x, v.y, v.z)?;
        }
        write!(z, "</vertices><triangles>")?;
        for [a, b, c] in m.faces {
            write!(z, r#"<triangle v1="{a}" v2="{b}" v3="{c}"/>"#)?;
        }
        write!(z, "</triangles></mesh></object>")?;
    }
    write!(z, "</resources><build>")?;
    for i in 1..=p.objects.len() {
        write!(z, r#"<item objectid="{i}"/>"#)?;
    }
    write!(z, "</build></model>")?;
    if let Some(settings) = settings {
        z.start_file("Metadata/project_settings.config", opt)?;
        serde_json::to_writer(&mut z, &settings.config)?;
        z.start_file("Metadata/model_settings.config", opt)?;
        write!(z, "<config>")?;
        for (i, o) in p.objects.iter().enumerate() {
            write!(
                z,
                r#"<object id="{}"><metadata key="name" value="{}"/><metadata key="extruder" value="{}"/></object>"#,
                i + 1,
                xml_text(&o.name),
                o.filament_slot
            )?;
        }
        write!(z, r#"<plate><metadata key="plater_id" value="1"/>"#)?;
        for id in 1..=p.objects.len() {
            write!(
                z,
                r#"<model_instance><metadata key="object_id" value="{id}"/><metadata key="instance_id" value="0"/></model_instance>"#
            )?;
        }
        write!(z, "</plate></config>")?;
    }
    z.finish()?;
    Ok(())
}
