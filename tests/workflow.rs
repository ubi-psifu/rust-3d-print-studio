use glam::DVec3;
use rust_3d_print_studio::{
    files::{load, save, Output},
    geometry::{Object, Shape},
    project::{History, Project},
};
use std::{collections::HashMap, fs, io::Read};
fn model(shape: Shape) -> Project {
    let mut p = Project::default();
    p.objects
        .push(Object::new(shape, DVec3::new(128., 128., 0.)));
    p
}
#[test]
fn all_primitives_are_closed_and_oriented() {
    for shape in Shape::ALL {
        let mesh = model(shape).objects[0].mesh();
        let mut edges = HashMap::new();
        let mut volume = 0.;
        for [a, b, c] in mesh.faces {
            let [u, v, w] = [a, b, c].map(|i| mesh.vertices[i]);
            assert!((v - u).cross(w - u).length() > 1e-8);
            volume += u.dot(v.cross(w)) / 6.;
            for (x, y) in [(a, b), (b, c), (c, a)] {
                let e = edges.entry((x.min(y), x.max(y))).or_insert((0, 0));
                e.0 += 1;
                e.1 += if x < y { 1 } else { -1 };
            }
        }
        assert!(volume > 0.);
        assert!(edges.values().all(|e| *e == (2, 0)), "{shape:?}");
    }
}
#[test]
fn cube_dimensions_and_rotation() {
    let mut o = model(Shape::Box).objects.remove(0);
    let (lo, hi) = o.mesh().bounds();
    assert!((hi - lo - DVec3::splat(20.)).length() < 1e-9);
    o.size.x = 40.;
    o.rotation.z = 90.;
    let (lo, hi) = o.mesh().bounds();
    assert!((hi - lo - DVec3::new(20., 40., 20.)).length() < 1e-9);
}
#[test]
fn bore_keeps_channel_open() {
    let o = model(Shape::BoredBox).objects.remove(0);
    let m = o.mesh();
    for face in m.faces {
        let pts = face.map(|i| m.vertices[i] - o.position);
        let center = (pts[0] + pts[1] + pts[2]) / 3.;
        assert!(center.truncate().length() > 3.9);
    }
}
#[test]
fn bounds_and_exclusion() {
    let mut p = model(Shape::Box);
    let o = &mut p.objects[0];
    let (lo, hi) = o.mesh().bounds();
    assert_eq!(p.printer.warning(lo, hi), None);
    o.position = DVec3::new(12., 12., 0.);
    let (lo, hi) = o.mesh().bounds();
    assert!(p.printer.warning(lo, hi).unwrap().contains("Sperrzone"));
    o.size.z = 251.;
    let (lo, hi) = o.mesh().bounds();
    assert!(p.printer.warning(lo, hi).unwrap().contains("Profilgrenze"));
}
#[test]
fn undo_and_branch() {
    let mut p = model(Shape::Box);
    let mut h = History::default();
    h.record(p.clone());
    p.objects.clear();
    assert!(h.undo(&mut p));
    assert_eq!(p.objects.len(), 1);
    assert!(h.redo(&mut p));
    assert!(p.objects.is_empty());
    assert!(h.undo(&mut p));
    h.record(p.clone());
    assert!(!h.redo(&mut p));
}
#[test]
fn project_roundtrip_and_replace() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("modell.r3dp");
    let mut p = model(Shape::BoredBox);
    save(&path, &p, Output::Project, false).unwrap();
    assert_eq!(load(&path).unwrap(), p);
    p.objects[0].bore = 4.;
    assert!(save(&path, &p, Output::Project, false).is_err());
    assert_ne!(load(&path).unwrap(), p);
    save(&path, &p, Output::Project, true).unwrap();
    assert_eq!(load(&path).unwrap(), p);
}
#[test]
fn corrupt_input_is_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bad.r3dp");
    for bad in [
        b"{".as_slice(),
        br#"{"version":900}"#,
        &vec![32u8; 2 * 1024 * 1024 + 1],
    ] {
        fs::write(&path, bad).unwrap();
        assert!(load(&path).is_err());
    }
    let mut v = serde_json::to_value(model(Shape::Box)).unwrap();
    v["extension"] = true.into();
    fs::write(&path, serde_json::to_vec(&v).unwrap()).unwrap();
    assert!(load(&path).is_err());
}
#[test]
fn mesh_exports() {
    let dir = tempfile::tempdir().unwrap();
    let mut p = model(Shape::Box);
    p.objects
        .push(Object::new(Shape::BoredBox, DVec3::new(25., 30., 0.)));
    let n = p
        .objects
        .iter()
        .map(|o| o.mesh().faces.len())
        .sum::<usize>();
    for f in [Output::Stl, Output::Obj, Output::ThreeMf] {
        let path = dir.path().join("test").with_extension(f.ext());
        save(&path, &p, f, false).unwrap();
        match f {
            Output::Stl => {
                let b = fs::read(path).unwrap();
                assert_eq!(
                    u32::from_le_bytes(b[80..84].try_into().unwrap()) as usize,
                    n
                );
                assert_eq!(b.len(), 84 + 50 * n);
            }
            Output::Obj => assert_eq!(
                fs::read_to_string(path)
                    .unwrap()
                    .lines()
                    .filter(|l| l.starts_with("f "))
                    .count(),
                n
            ),
            Output::ThreeMf => {
                let mut z = zip::ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
                assert_eq!(z.len(), 3);
                let mut xml = String::new();
                z.by_name("3D/3dmodel.model")
                    .unwrap()
                    .read_to_string(&mut xml)
                    .unwrap();
                assert!(xml.contains("unit=\"millimeter\""));
                assert_eq!(xml.matches("<item ").count(), 2);
                assert_eq!(xml.matches("<triangle ").count(), n);
            }
            _ => unreachable!(),
        }
    }
}
