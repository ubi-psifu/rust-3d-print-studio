//! Export eines einzelnen Kreiszylinders aus einer R3DP-Datei.
use rust_3d_print_studio::files::{load, save, Output};
use std::{io, path::PathBuf};

fn main() -> io::Result<()> {
    let paths: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    let [source, target] = paths.as_slice() else {
        return Err(io::Error::other(
            "Aufruf: step_export Quelle.r3dp Ziel.step",
        ));
    };
    save(target, &load(source)?, Output::StepCylinder, false)
}
