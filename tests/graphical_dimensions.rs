use glam::DVec3;
use rust_3d_print_studio::{
    cad::{
        sketch::{Rule, RuleKind, Sketch},
        Feature,
    },
    files::{self, Output},
    project::History,
};
use std::path::Path;
#[test]
fn indexed_references_survive_insertion_and_removal() {
    let mut s = Sketch::default();
    s.set_distance(0, 1, 40.).unwrap();
    let before = s.clone();
    let outline = s.sample(16).unwrap();
    s.insert_point_after(0).unwrap();
    assert_eq!(s.segments[0].ids[..2], [0, 2]);
    assert_eq!(s.rules[0].ids[..2], [0, 2]);
    assert_eq!(s.sample(16).unwrap(), outline);
    let inserted = s.clone();
    assert!(s.remove_unused_point(2).is_err());
    assert_eq!(s, inserted);
    s.remove_unused_point(1).unwrap();
    assert_eq!(s, before);
}
#[test]
fn rule_only_reference_blocks_removal() {
    let mut s = Sketch::default();
    s.insert_point_after(0).unwrap();
    s.rules.push(Rule {
        kind: RuleKind::Fixed,
        ids: [1, 0, 0, 0],
        value: 0.,
    });
    assert!(s.remove_unused_point(1).unwrap_err().contains("Bedingung"));
}
#[test]
fn conflicting_dimension_keeps_valid_rules() {
    let mut s = Sketch::default();
    for id in [0, 1] {
        s.rules.push(Rule {
            kind: RuleKind::Fixed,
            ids: [id, 0, 0, 0],
            value: 0.,
        });
    }
    let before = s.clone();
    assert!(s.set_distance(0, 1, 50.).is_err());
    assert_eq!(s, before);
    assert!(s.set_distance(0, 0, 1.).is_err());
    assert!(s.set_distance(0, 9, 1.).is_err());
}
#[test]
fn dimensioned_lug_round_trip() {
    let mut p = files::load(Path::new("examples/modelle/Bemasste-Lasche.r3dp")).unwrap();
    let original = p.clone();
    let mut history = History::default();
    history.record(p.clone());
    let o = &mut p.objects[0];
    let Some(Feature::SketchExtrude { sketch, .. }) = &mut o.feature else {
        panic!()
    };
    let count = sketch.rules.len();
    sketch.set_distance(1, 0, 60.).unwrap();
    assert_eq!(sketch.rules.len(), count);
    o.rebuild().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Lasche.r3dp");
    files::save(&path, &p, Output::Project, false).unwrap();
    let restored = files::load(&path).unwrap();
    let file = dir.path().join("Lasche.3mf");
    files::save(&file, &restored, Output::ThreeMf, false).unwrap();
    let imported = rust_3d_print_studio::import::load(&file, 1.).unwrap();
    let (lo, hi) = imported[0].mesh().bounds();
    assert!((hi - lo - DVec3::new(60., 30., 6.)).length() < 1e-5);
    assert!(history.undo(&mut p));
    assert_eq!(p, original);
}

#[test]
fn angle_is_spatial_and_updates_in_reverse_order() {
    let mut sketch = Sketch {
        planar: false,
        ..Default::default()
    };
    sketch.points[2].z = 12.;
    sketch.set_angle(0, 1, 2, 65.).unwrap();
    sketch.set_angle(2, 1, 0, 120.).unwrap();
    assert_eq!(sketch.rules.len(), 1);
    let p = sketch.solved_points().unwrap();
    let angle = (p[0] - p[1]).angle_between(p[2] - p[1]).to_degrees();
    assert!((angle - 120.).abs() < 1e-5);
    let restored: Sketch = serde_json::from_slice(&serde_json::to_vec(&sketch).unwrap()).unwrap();
    assert_eq!(restored.solved_points().unwrap(), p);
}
#[test]
fn angle_rejects_conflicts_and_invalid_arms_without_modifying_sketch() {
    let mut s = Sketch::default();
    for id in [0, 1, 2] {
        s.rules.push(Rule {
            kind: RuleKind::Fixed,
            ids: [id, 0, 0, 0],
            value: 0.,
        });
    }
    s.set_angle(0, 1, 2, 90.).unwrap();
    let before = s.clone();
    for value in [180., 0., 60., f64::NAN] {
        assert!(s.set_angle(0, 1, 2, value).is_err());
    }
    assert!(s.set_angle(0, 0, 1, 90.).is_err());
    assert!(s.set_angle(0, 1, 99, 90.).is_err());
    assert_eq!(s, before);
    s.points[0] = s.points[1];
    assert!(s.solved_points().is_err());
}
#[test]
fn angle_drives_example_extrusion_and_survives_project_roundtrip() {
    let mut p = files::load(Path::new("examples/modelle/Winkellasche.r3dp")).unwrap();
    let o = &mut p.objects[0];
    let Some(Feature::SketchExtrude { sketch, .. }) = &mut o.feature else {
        panic!()
    };
    sketch.set_angle(0, 1, 2, 75.).unwrap();
    o.rebuild().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Winkel.r3dp");
    files::save(&path, &p, Output::Project, false).unwrap();
    let restored = files::load(&path).unwrap();
    let (lo, hi) = restored.objects[0].mesh().bounds();
    assert!(((hi - lo).y - 20. * 75_f64.to_radians().sin()).abs() < 1e-5);
}
