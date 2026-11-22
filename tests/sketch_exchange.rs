use glam::DVec3;
use rust_3d_print_studio::{
    cad::sketch::{Rule, RuleKind::*, Sketch},
    files::{self, Output},
    import, ply,
};
fn rule(kind: rust_3d_print_studio::cad::sketch::RuleKind, ids: [usize; 4], value: f64) -> Rule {
    Rule { kind, ids, value }
}
#[test]
fn free_and_anchored_distance() {
    let mut s = Sketch::default();
    s.rules.push(rule(Distance, [0, 1, 0, 0], 40.));
    let p = s.solved_points().unwrap();
    assert!((p[0].x + 20.).abs() < 1e-7 && (p[1].x - 20.).abs() < 1e-7);
    s.rules.push(rule(Fixed, [0; 4], 0.));
    let p = s.solved_points().unwrap();
    assert_eq!(p[0], s.points[0]);
    assert!((p[1].x - 26.).abs() < 1e-7);
    s.rules.push(rule(Fixed, [1; 4], 0.));
    assert!(s.solved_points().is_err());
}
#[test]
fn old_sketch_uses_directed_constraints() {
    let mut json = serde_json::to_value(Sketch::default()).unwrap();
    json.as_object_mut().unwrap().remove("bidirectional");
    let mut s: Sketch = serde_json::from_value(json).unwrap();
    s.rules.push(rule(Distance, [0, 1, 0, 0], 40.));
    let p = s.solved_points().unwrap();
    assert_eq!(p[0], s.points[0]);
    assert!((p[1].x - 26.).abs() < 1e-7);
}
#[test]
fn spatial_equal_and_orthogonal() {
    let s = Sketch {
        planar: false,
        points: vec![DVec3::ZERO, DVec3::X * 8., DVec3::new(1., 3., 4.)],
        rules: vec![
            rule(Fixed, [0; 4], 0.),
            rule(Fixed, [1; 4], 0.),
            rule(Perpendicular, [0, 1, 0, 2], 0.),
            rule(EqualLength, [0, 1, 0, 2], 0.),
        ],
        ..Default::default()
    };
    let p = s.solved_points().unwrap();
    assert!(p[2].x.abs() < 1e-7 && (p[2].length() - 8.).abs() < 1e-7);
    assert!(p[2].z > 1.);
}
#[test]
fn signaled_invalid_index() {
    let mut s = Sketch::default();
    s.rules.push(rule(Fixed, [256; 4], 0.));
    assert!(s.sample(16).is_err());
}
#[test]
fn dimensioned_example_and_geometry_exchange() {
    let p = files::load(std::path::Path::new(
        "examples/modelle/Bemasste-Platte.r3dp",
    ))
    .unwrap();
    let model = p.objects[0].mesh();
    let (a, b) = model.bounds();
    assert!((b - a - DVec3::new(48., 30., 6.)).length() < 1e-6);
    let dir = tempfile::tempdir().unwrap();
    for format in [Output::Ply, Output::StlAscii] {
        let file = dir.path().join(format!("Platte.{}", format.ext()));
        files::save(&file, &p, format, false).unwrap();
        let parts = import::load(&file, 1.).unwrap();
        assert_eq!(parts.len(), 1);
        let mesh = parts[0].mesh();
        let (lo, hi) = mesh.bounds();
        assert_eq!(mesh.faces.len(), model.faces.len());
        assert!(lo.distance(a) < 1e-6 && hi.distance(b) < 1e-6);
    }
}
#[test]
fn ply_rejects_truncation_binary_attributes_and_invalid_indices() {
    let header="ply\nformat ascii 1.0\nelement vertex 3\nproperty float z\nproperty float x\nproperty float y\nelement face 1\nproperty list uchar int vertex_indices\nend_header\n";
    let ok = format!("{header}0 0 0\n0 1 0\n0 0 1\n3 0 1 2\n");
    assert_eq!(ply::read(ok.as_bytes()).unwrap().vertices[1], DVec3::X);
    for bad in [
        header.to_string(),
        ok.replace("ascii", "binary_little_endian"),
        ok.replace("3 0 1 2", "3 0 1 3"),
        ok.replace("element face 1", "property uchar red\nelement face 1"),
    ] {
        assert!(ply::read(bad.as_bytes()).is_err());
    }
}
