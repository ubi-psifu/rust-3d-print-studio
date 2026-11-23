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
