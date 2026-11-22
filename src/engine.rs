//! Lokale Slicing-Ausführung; nur der externe Slicer erzeugt Maschinenbefehle.
use crate::{
    project::Project,
    slicer::{self, Preparation},
};
use serde_json::Value;
use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
fn invalid(msg: &str) -> io::Error {
    io::Error::other(msg)
}
fn read_report(path: &Path) -> io::Result<Value> {
    let mut bytes = Vec::new();
    File::open(path)?.take(2_097_153).read_to_end(&mut bytes)?;
    if bytes.len() > 2_097_152 {
        return Err(invalid("Slicerbericht zu groß"));
    }
    Ok(serde_json::from_slice(&bytes)?)
}
pub fn slice(exe: &Path, target: &Path, p: &Project, prep: &Preparation) -> io::Result<String> {
    if !target
        .file_name()
        .and_then(|s| s.to_str())
        .is_some_and(|s| s.to_lowercase().ends_with(".gcode.3mf"))
    {
        return Err(invalid("Ausgabedatei mit Endung .gcode.3mf wählen"));
    }
    if target.exists() {
        return Err(invalid("Ausgabedatei existiert bereits"));
    }
    if !exe.is_absolute() || !exe.is_file() {
        return Err(invalid(
            "Vollständigen Programmpfad für Bambu Studio wählen",
        ));
    }
    for o in &p.objects {
        let (a, b) = o.mesh().bounds();
        if let Some(reason) = p.printer.warning(a, b) {
            return Err(invalid(&format!("{}: {reason}", o.name)));
        }
    }
    let dir = tempfile::tempdir_in(
        target
            .parent()
            .filter(|d| !d.as_os_str().is_empty())
            .unwrap_or(Path::new(".")),
    )?;
    slicer::export(&dir.path().join("platte.3mf"), p, Some(prep))?;
    let mut process = Command::new(exe)
        .current_dir(dir.path())
        .args([
            "--arrange",
            "0",
            "--orient",
            "0",
            "--slice",
            "1",
            "--export-3mf",
            "ergebnis.gcode.3mf",
            "platte.3mf",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let start = Instant::now();
    let status = loop {
        if let Some(s) = process.try_wait()? {
            break s;
        }
        if start.elapsed() > Duration::from_secs(3600) {
            let _ = process.kill();
            let _ = process.wait();
            return Err(invalid("Zeitlimit für lokalen Slicer erreicht"));
        }
        std::thread::sleep(Duration::from_millis(120));
    };
    let report = read_report(&dir.path().join("result.json"));
    if !status.success() {
        let message = report
            .as_ref()
            .ok()
            .and_then(|v| v["error_string"].as_str())
            .unwrap_or("Kein Fehlerbericht der Engine");
        return Err(invalid(&format!(
            "Lokaler Slicer beendet ({status}): {message}"
        )));
    }
    let report = report?;
    if report["return_code"].as_i64() != Some(0)
        || report["sliced_plates"].as_array().map(Vec::len) != Some(1)
    {
        return Err(invalid(
            "Engine bestätigt keine erfolgreich geslicte Einzelplatte",
        ));
    }
    let path = dir.path().join("ergebnis.gcode.3mf");
    let mut archive = zip::ZipArchive::new(File::open(&path)?)?;
    let mut gcode = archive.by_name("Metadata/plate_1.gcode")?;
    if !(128..=1_073_741_824).contains(&gcode.size()) {
        return Err(invalid("Ungültige Größe der Werkzeugwegdatei"));
    }
    let mut head = vec![0; gcode.size().min(8192) as usize];
    gcode.read_exact(&mut head)?;
    let layers = String::from_utf8_lossy(&head)
        .lines()
        .find_map(|s| {
            s.strip_prefix("; total layer number: ")
                .and_then(|v| v.trim().parse::<u32>().ok())
        })
        .filter(|n| *n > 0)
        .ok_or_else(|| invalid("Schichtzahl im Ergebnis fehlt"))?;
    // Der ZIP-Leser prüft die CRC beim vollständigen Lesen.
    let rest = gcode.size() - head.len() as u64;
    if io::copy(&mut gcode.take(rest + 1), &mut io::sink())? != rest {
        return Err(invalid("Unvollständige Werkzeugwege"));
    }
    drop(archive);
    let mut output = tempfile::NamedTempFile::new_in(dir.path())?;
    io::copy(&mut File::open(path)?, output.as_file_mut())?;
    output.as_file().sync_all()?;
    output.persist_noclobber(target)?;
    let plate = &report["sliced_plates"][0];
    let duration = plate["total_predication"]
        .as_f64()
        .map(|s| format!(" · {:.0} min geschätzt", s / 60.))
        .unwrap_or_default();
    let warning = plate["warning_message"]
        .as_str()
        .filter(|s| !s.is_empty())
        .map(|s| format!(" · Slicerhinweis: {s}"))
        .unwrap_or_default();
    Ok(format!(
        "{layers} Schichten{duration}{warning} · {}",
        fs::canonicalize(target)?.display()
    ))
}
