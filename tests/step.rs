use rust_3d_print_studio::{
    files::{load, save, Output},
    geometry::{Object, Shape},
};
use std::path::Path;

#[test]
fn step_rejects_unsupported_models_without_replacing_the_target() {
    let mut p = load(Path::new("examples/modelle/Referenzzylinder.r3dp")).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("Körper.step");
    save(&target, &p, Output::StepCylinder, false).unwrap();
    let original = std::fs::read_to_string(&target).unwrap();
    assert!(original.contains("MANIFOLD_SOLID_BREP"));
    assert!(original.contains("CYLINDRICAL_SURFACE"));
    p.objects[0].size.y = 15.;
    assert!(save(&target, &p, Output::StepCylinder, true).is_err());
    p.objects[0].size.y = 14.;
    p.objects.push(Object::new(Shape::Box, glam::DVec3::ZERO));
    assert!(save(&target, &p, Output::StepCylinder, true).is_err());
    p.objects.remove(0);
    assert!(save(&target, &p, Output::StepCylinder, true).is_err());
    assert_eq!(std::fs::read_to_string(target).unwrap(), original);
}

#[test]
fn step_tube_uses_feature_diameter_for_the_inner_wall() {
    use rust_3d_print_studio::cad::Feature;
    let mut project = load(Path::new("examples/modelle/Referenzrohr.r3dp")).unwrap();
    let object = &mut project.objects[0];
    object.feature = Some(Feature::Primitive {
        shape: Shape::Tube,
        size: object.size,
        bore: 6.0,
    });
    object.rebuild().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("Rohr.stp");
    save(&path, &project, Output::StepCylinder, false).unwrap();
    let step = std::fs::read_to_string(path).unwrap();
    assert_eq!(step.matches("CYLINDRICAL_SURFACE").count(), 2);
    assert!(step.contains("CYLINDRICAL_SURFACE('',#46,3.0000000000000000E0)"));
}

#[test]
fn bored_box_keeps_independent_width_and_depth() {
    let mut project = load(Path::new("examples/modelle/Referenzbohrquader.r3dp")).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Quader.step");
    project.objects[0].size.x = 36.;
    project.objects[0].bore = 5.;
    save(&path, &project, Output::StepCylinder, false).unwrap();
    let original = std::fs::read_to_string(&path).unwrap();
    assert_eq!(original.matches("ADVANCED_FACE(").count(), 7);
    assert!(original.contains("CYLINDRICAL_SURFACE('',#200,2.5000000000000000E0)"));
    project.objects[0].bore = project.objects[0].size.y;
    assert!(save(&path, &project, Output::StepCylinder, true).is_err());
    assert_eq!(original, std::fs::read_to_string(path).unwrap());
}
