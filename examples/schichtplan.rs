//! Erzeugt ohne Herstellerslicer einen maßstäblichen Schichtplan.
use rust_3d_print_studio::{files, layers};
use std::{io, path::PathBuf};
fn main() -> io::Result<()> {
    let args: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    let [project, output] = args.as_slice() else {
        return Err(io::Error::other(
            "Projekt.r3dp und neue Schichtplan.zip angeben",
        ));
    };
    let n = layers::save(&files::load(project)?, 0.2, 15, output)?;
    println!("{n} Modellschichten gespeichert");
    Ok(())
}
