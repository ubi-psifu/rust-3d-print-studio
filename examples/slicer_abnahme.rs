//! Schreibt eine ungeslicte Zweifarb-Testplatte mit einer lokalen P1S-Vorlage.
use glam::DVec3;
use rust_3d_print_studio::{
    geometry::{Object, Shape},
    project::Project,
    slicer::{self, Preparation},
};
use std::{io, path::PathBuf};
fn main() -> io::Result<()> {
    let args: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    let [source, target] = args.as_slice() else {
        return Err(io::Error::other(
            "P1S-Vorlage mit zwei Filamenten und neue Ausgabe.3mf angeben",
        ));
    };
    let mut p = Project::default();
    for (x, y, slot, name) in [
        (40., 50., 1, "Teil & Sockel"),
        (202., 191., 2, "Gegenstück"),
    ] {
        let mut o = Object::new(Shape::Box, DVec3::new(x, y, 0.));
        o.name = name.into();
        o.filament_slot = slot;
        p.objects.push(o);
    }
    let mut prep = Preparation::load(source)?;
    prep.override_process(0.16, 4, 27, true)?;
    slicer::export(target, &p, Some(&prep))
}
