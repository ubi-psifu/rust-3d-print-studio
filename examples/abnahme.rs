//! Erzeugt die Abnahmedateien in einem neuen Ordner.
use glam::DVec3;
use rust_3d_print_studio::{
    files::{save, Output},
    geometry::{Object, Shape},
    project::Project,
};
fn main() -> std::io::Result<()> {
    let dir = std::env::args_os()
        .nth(1)
        .ok_or_else(|| std::io::Error::other("Zielordner angeben"))?;
    let dir = std::path::Path::new(&dir);
    std::fs::create_dir(dir)?;
    for (name, shape, size) in [
        ("Wuerfel", Shape::Box, DVec3::splat(20.)),
        ("Bohrquader", Shape::BoredBox, DVec3::new(55., 32., 16.)),
        ("Profilgrenze", Shape::Box, DVec3::new(20., 20., 251.)),
    ] {
        let mut p = Project {
            name: name.into(),
            ..Project::default()
        };
        let mut o = Object::new(shape, DVec3::new(128., 128., 0.));
        o.size = size;
        p.objects.push(o);
        for f in [Output::Project, Output::Stl, Output::ThreeMf] {
            save(&dir.join(name).with_extension(f.ext()), &p, f, false)?;
        }
    }
    Ok(())
}
