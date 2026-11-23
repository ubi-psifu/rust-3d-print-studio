//! Export aller unterstützten Körper aus einer R3DP-Datei.
use rust_3d_print_studio::files::{load, save, save_step_selection, Output};
use std::{io, path::PathBuf};

fn main() -> io::Result<()> {
    let paths: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    let [source, target, numbers @ ..] = paths.as_slice() else {
        return Err(io::Error::other(
            "Aufruf: step_export Quelle.r3dp Ziel.step [Objektnummer …]",
        ));
    };
    let project = load(source)?;
    if numbers.is_empty() {
        return save(target, &project, Output::StepCylinder, false);
    }
    let indices = numbers
        .iter()
        .map(|n| {
            n.to_str()
                .and_then(|s| s.parse::<usize>().ok())
                .and_then(|i| i.checked_sub(1))
                .ok_or_else(|| io::Error::other("Objektnummern beginnen bei 1."))
        })
        .collect::<io::Result<Vec<_>>>()?;
    save_step_selection(target, &project, &indices, false)
}
