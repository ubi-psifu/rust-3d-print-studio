//! Offline-Abnahme über denselben Einstiegspunkt wie die Oberfläche.
use rust_3d_print_studio::{engine, files, slicer::Preparation};
use std::{io, path::PathBuf};
fn main() -> io::Result<()> {
    let args: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    let [exe, project, template, output] = args.as_slice() else {
        return Err(io::Error::other(
            "Slicerprogramm, Projekt.r3dp, Vorlage.3mf und neue Ausgabe.gcode.3mf angeben",
        ));
    };
    println!(
        "{}",
        engine::slice(
            &exe.canonicalize()?,
            output,
            &files::load(project)?,
            &Preparation::load(template)?
        )?
    );
    Ok(())
}
