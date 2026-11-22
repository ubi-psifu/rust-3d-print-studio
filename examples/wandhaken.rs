use glam::DVec3;
use rust_3d_print_studio::{
    cad::{Feature, Hook},
    files::{save, Output},
    geometry::{Object, Shape},
    project::Project,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut p = Project {
        name: "J-Wandhaken".into(),
        ..Project::default()
    };
    let mut o = Object::new(Shape::Box, DVec3::new(128., 128., 0.));
    o.name = "Wandhaken".into();
    o.color = [203, 209, 219];
    o.feature = Some(Feature::Hook(Hook::default()));
    o.rebuild()?;
    p.view.target = o.position + DVec3::Z * 18.;
    p.view.distance = 150.;
    p.objects.push(o);
    for f in [Output::Project, Output::Stl, Output::ThreeMf] {
        save(
            &std::path::Path::new("examples/modelle/Wandhaken").with_extension(f.ext()),
            &p,
            f,
            true,
        )?;
    }
    p.name = "Wandhaken · Einzelfeatures".into();
    p.objects[0].feature = Some(Hook::default().features()?);
    p.objects[0].rebuild()?;
    save(
        std::path::Path::new("examples/modelle/Wandhaken-Einzelfeatures.r3dp"),
        &p,
        Output::Project,
        true,
    )?;
    Ok(())
}
