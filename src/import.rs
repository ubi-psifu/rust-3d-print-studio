use crate::{
    cad::{self, Feature},
    geometry::{Mesh, Object, Shape},
    project::MAX_OBJECTS,
};
use glam::{DMat3, DVec3};
use quick_xml::{events::Event, Reader};
use std::{collections::HashMap, fs::File, io::Read, path::Path};
type R<T> = Result<T, String>;
fn read(r: impl Read) -> R<Vec<u8>> {
    let mut data = vec![];
    r.take(8_388_609)
        .read_to_end(&mut data)
        .map_err(|e| e.to_string())?;
    if data.len() > 8_388_608 {
        return Err("Netzdaten überschreiten 8 MiB".into());
    }
    Ok(data)
}
fn point<'a>(mut t: impl Iterator<Item = &'a str>) -> R<DVec3> {
    let mut a = [0.; 3];
    for n in &mut a {
        *n = t
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|v| v.is_finite())
            .ok_or("Ungültige Koordinate")?
    }
    Ok(DVec3::from_array(a))
}
#[derive(Default)]
struct Builder {
    mesh: Mesh,
    ids: HashMap<[u64; 3], usize>,
}
impl Builder {
    fn triangle(&mut self, t: [DVec3; 3]) -> R<()> {
        if self.mesh.faces.len() >= 80000 {
            return Err("Mehr als 80000 Dreiecke".into());
        }
        self.mesh.faces.push(t.map(|v| {
            *self
                .ids
                .entry(v.to_array().map(|x| if x == 0. { 0 } else { x.to_bits() }))
                .or_insert_with(|| {
                    self.mesh.vertices.push(v);
                    self.mesh.vertices.len() - 1
                })
        }));
        Ok(())
    }
}
fn stl(d: &[u8]) -> R<Mesh> {
    let mut b = Builder::default();
    if d.len() >= 84
        && 84 + 50 * u32::from_le_bytes(d[80..84].try_into().unwrap()) as u64 == d.len() as u64
    {
        for f in d[84..].chunks_exact(50) {
            b.triangle(std::array::from_fn(|i| {
                DVec3::from_array(std::array::from_fn(|j| {
                    let k = 12 + i * 12 + j * 4;
                    f32::from_le_bytes(f[k..k + 4].try_into().unwrap()) as f64
                }))
            }))?;
        }
    } else {
        let mut open = false;
        let mut tri = vec![];
        for line in std::str::from_utf8(d).map_err(|_| "Ungültige STL")?.lines() {
            let mut t = line.split_whitespace();
            match t.next() {
                Some("facet") if !open => {
                    tri.clear();
                    open = true
                }
                Some("vertex") if open && tri.len() < 3 => tri.push(point(t)?),
                Some("endfacet") if open && tri.len() == 3 => {
                    b.triangle([tri[0], tri[1], tri[2]])?;
                    open = false
                }
                None | Some("solid" | "endsolid" | "outer" | "endloop") => (),
                _ => return Err("Ungültige STL-Facette".into()),
            }
        }
        if open {
            return Err("Unvollständige STL".into());
        }
    }
    Ok(b.mesh)
}
fn obj(d: &[u8], base: &str) -> R<Vec<(String, Mesh)>> {
    let mut parts = vec![];
    let mut b = Builder::default();
    let mut pool = vec![];
    let mut name = base.to_owned();
    for l in std::str::from_utf8(d)
        .map_err(|_| "OBJ benötigt UTF-8")?
        .lines()
    {
        let mut t = l.split('#').next().unwrap_or("").split_whitespace();
        match t.next() {
            Some("v") => pool.push(point(t)?),
            Some("o") => {
                if !b.mesh.faces.is_empty() {
                    parts.push((name, std::mem::take(&mut b).mesh))
                }
                name = t.collect::<Vec<_>>().join(" ");
                if parts.len() >= MAX_OBJECTS {
                    return Err("Zu viele Objekte".into());
                }
            }
            Some("f") => {
                let mut v = vec![];
                for k in t {
                    let id = k
                        .split('/')
                        .next()
                        .and_then(|n| n.parse::<i64>().ok())
                        .ok_or("Ungültiger OBJ-Index")?;
                    let i = if id > 0 {
                        id - 1
                    } else {
                        pool.len() as i64 + id
                    };
                    v.push(
                        *pool
                            .get(i as usize)
                            .ok_or("OBJ-Index außerhalb des Bereichs")?,
                    );
                    if v.len() > 256 {
                        return Err("Polygon zu groß".into());
                    }
                }
                if v.len() == 3 {
                    b.triangle([v[0], v[1], v[2]])?
                } else if v.len() > 3 {
                    for f in cad::cap(&v)? {
                        b.triangle(f.map(|i| v[i]))?
                    }
                } else {
                    return Err("Unvollständige OBJ-Fläche".into());
                }
            }
            Some("l" | "p" | "surf" | "curv") => {
                return Err("OBJ enthält nicht unterstützte Geometrie".into())
            }
            _ => (),
        }
    }
    if !b.mesh.faces.is_empty() {
        parts.push((name, b.mesh))
    }
    Ok(parts)
}
fn model(f: File) -> R<Vec<(String, Mesh)>> {
    if f.metadata().map_err(|e| e.to_string())?.len() > 32 * 1024 * 1024 {
        return Err("Modellcontainer zu groß".into());
    }
    let mut z = zip::ZipArchive::new(f).map_err(|e| e.to_string())?;
    if z.len() > 256
        || z.file_names().any(|n| {
            n.to_lowercase().starts_with("metadata/")
                || n.ends_with(".gcode")
                || n.ends_with(".model") && n != "3D/3dmodel.model"
        })
    {
        return Err("Druck- oder Projektcontainer: bitte als Modell-3MF exportieren".into());
    }
    let d = read(z.by_name("3D/3dmodel.model").map_err(|e| e.to_string())?)?;
    let mut r = Reader::from_reader(d.as_slice());
    let mut parts = vec![];
    let mut mesh = None;
    let mut resources = HashMap::new();
    let mut scale = 1.;
    let mut count = 0;
    loop {
        match r.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) | Event::Empty(e) => {
                let mut a = HashMap::new();
                for k in e.attributes() {
                    let k = k.map_err(|e| e.to_string())?;
                    a.insert(
                        k.key.as_ref().to_vec(),
                        k.decoded_and_normalized_value(
                            quick_xml::XmlVersion::Implicit1_0,
                            r.decoder(),
                        )
                        .map_err(|e| e.to_string())?
                        .into_owned(),
                    );
                }
                let get = |n: &[u8]| a.get(n).map(String::as_str);
                let idx = |n: &[u8]| {
                    get(n)
                        .and_then(|v| v.parse::<usize>().ok())
                        .ok_or("Ungültiger Index")
                };
                match e.local_name().as_ref() {
                    b"model" => {
                        if get(b"requiredextensions").is_some() {
                            return Err("3MF-Erweiterung erforderlich".into());
                        }
                        scale = match get(b"unit").unwrap_or("millimeter") {
                            "millimeter" => 1.,
                            "centimeter" => 10.,
                            "meter" => 1000.,
                            "inch" => 25.4,
                            "foot" => 304.8,
                            "micron" => 0.001,
                            _ => return Err("Unbekannte Einheit".into()),
                        };
                    }
                    b"object" => {
                        let id = idx(b"id")?;
                        if mesh.is_some()
                            || resources.contains_key(&id)
                            || resources.len() >= MAX_OBJECTS
                        {
                            return Err("Ungültige Ressource".into());
                        }
                        mesh = Some((
                            id,
                            get(b"name").unwrap_or("3MF-Körper").to_owned(),
                            Mesh::default(),
                        ));
                    }
                    b"vertex" => mesh
                        .as_mut()
                        .ok_or("Punkt ohne Netz")?
                        .2
                        .vertices
                        .push(point(
                            [get(b"x"), get(b"y"), get(b"z")].into_iter().flatten(),
                        )?),
                    b"triangle" => mesh.as_mut().ok_or("Fläche ohne Netz")?.2.faces.push([
                        idx(b"v1")?,
                        idx(b"v2")?,
                        idx(b"v3")?,
                    ]),
                    b"item" => {
                        let (n, m): &(String, Mesh) = resources
                            .get(&idx(b"objectid")?)
                            .ok_or("Unbekannte Ressource")?;
                        count += m.faces.len();
                        if count > 80000 || parts.len() >= MAX_OBJECTS {
                            return Err("Instanzgrenze überschritten".into());
                        }
                        let mut m = m.clone();
                        let mut mat = DMat3::IDENTITY;
                        let mut off = DVec3::ZERO;
                        if let Some(t) = get(b"transform") {
                            let v = t
                                .split_whitespace()
                                .map(str::parse::<f64>)
                                .collect::<Result<Vec<_>, _>>()
                                .map_err(|_| "Ungültige Transformation")?;
                            if v.len() != 12 || !v.iter().all(|x| x.is_finite()) {
                                return Err("Ungültige Transformation".into());
                            }
                            mat = DMat3::from_cols_array(v[..9].try_into().unwrap());
                            off = DVec3::from_slice(&v[9..]);
                        }
                        for v in &mut m.vertices {
                            *v = (mat * *v + off) * scale
                        }
                        parts.push((n.clone(), m));
                    }
                    b"resources" | b"mesh" | b"vertices" | b"triangles" | b"build"
                    | b"metadata" => (),
                    _ => return Err("Nicht unterstützte 3MF-Ressource".into()),
                }
            }
            Event::End(e) if e.local_name().as_ref() == b"object" => {
                let (id, n, m) = mesh.take().ok_or("Ungültiges Objektende")?;
                resources.insert(id, (n, m));
            }
            Event::DocType(_) => return Err("DTD unzulässig".into()),
            Event::Eof => break,
            _ => (),
        }
    }
    Ok(parts)
}
pub fn load(path: &Path, scale: f64) -> R<Vec<Object>> {
    let f = File::open(path).map_err(|e| e.to_string())?;
    let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Netz");
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    let parts = match ext.as_str() {
        "3mf" => model(f)?,
        "stl" => vec![(name.to_owned(), stl(&read(f)?)?)],
        "obj" => obj(&read(f)?, name)?,
        _ => return Err("STL, OBJ oder Modell-3MF wählen".into()),
    };
    if parts.is_empty() {
        return Err("Datei enthält kein Netz".into());
    }
    let mut objects = vec![];
    for (n, mut m) in parts {
        if ext != "3mf" {
            for v in &mut m.vertices {
                *v *= scale
            }
        }
        let mut m = cad::surfaces::repair(&m, 1e-8)?;
        let (a, b) = m.bounds();
        let at = DVec3::new((a.x + b.x) / 2., (a.y + b.y) / 2., a.z);
        for v in &mut m.vertices {
            *v -= at
        }
        let mut o = Object::new(Shape::Box, at);
        o.name = n.chars().take(120).collect();
        o.feature = Some(Feature::Polygon(m));
        o.rebuild()?;
        if !o.valid() {
            return Err("Ungültige Objektmaße".into());
        }
        objects.push(o)
    }
    Ok(objects)
}
