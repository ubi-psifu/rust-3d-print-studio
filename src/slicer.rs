//! Lokale Übergabe einer Platte an eine separat installierte Bambu-Studio-Version.
use crate::{files, project::Project};
use serde_json::Value;
use std::{
    fs::File,
    io::{self, BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::Command,
};
use zip::ZipArchive;

fn invalid(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, s)
}
#[derive(Clone)]
pub struct Preparation {
    application: String,
    pub(crate) config: Value,
}
impl Preparation {
    pub(crate) fn application(&self) -> &str {
        &self.application
    }
    pub fn label(&self, k: &str) -> &str {
        self.config[k].as_str().unwrap_or("")
    }
    pub fn slot_color(&self, i: usize) -> Option<[u8; 3]> {
        let s = self.config["filament_colour"]
            .get(i)?
            .as_str()?
            .strip_prefix('#')?;
        if !s.is_ascii() || !(s.len() == 6 || s.len() == 8) {
            return None;
        }
        Some([
            u8::from_str_radix(&s[..2], 16).ok()?,
            u8::from_str_radix(&s[2..4], 16).ok()?,
            u8::from_str_radix(&s[4..6], 16).ok()?,
        ])
    }
    pub fn slots(&self) -> Vec<&str> {
        self.config["filament_settings_id"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|v| v.as_str().unwrap_or(""))
            .collect()
    }
    pub fn load(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        if file.metadata()?.len() > 256 * 1024 * 1024 {
            return Err(invalid("Vorlage überschreitet 256 MiB"));
        }
        let mut zip = ZipArchive::new(file)?;
        if zip.len() > 2048
            || zip
                .file_names()
                .any(|n| n.to_ascii_lowercase().ends_with(".gcode"))
        {
            return Err(invalid(
                "Ungeslictes Bambu-Projekt mit höchstens 2048 Einträgen erforderlich",
            ));
        }
        let mut prefix = Vec::new();
        zip.by_name("3D/3dmodel.model")?
            .take(16384)
            .read_to_end(&mut prefix)?;
        let header = String::from_utf8_lossy(&prefix);
        let version = header
            .split_once(r#"<metadata name="Application">BambuStudio-"#)
            .and_then(|(_, s)| s.split_once('<'))
            .map(|(v, _)| v)
            .filter(|v| {
                !v.is_empty() && v.len() < 32 && v.bytes().all(|b| b.is_ascii_digit() || b == b'.')
            })
            .ok_or_else(|| invalid("Eine in Bambu Studio gespeicherte Projektvorlage wählen"))?;
        let mut bytes = Vec::new();
        zip.by_name("Metadata/project_settings.config")?
            .take(2_097_153)
            .read_to_end(&mut bytes)?;
        if bytes.len() > 2_097_152 {
            return Err(invalid("Profileinstellungen überschreiten 2 MiB"));
        }
        let config: Value = serde_json::from_slice(&bytes)?;
        let prep = Self {
            application: format!("BambuStudio-{version}"),
            config,
        };
        if prep.label("printer_model") != "Bambu Lab P1S"
            || prep.label("printer_settings_id").is_empty()
            || prep.label("print_settings_id").is_empty()
            || !(1..=16).contains(&prep.slots().len())
            || prep.slots().contains(&"")
        {
            return Err(invalid(
                "Vollständiges P1S-Projekt mit Druck-, Prozess- und Filamentprofil erforderlich",
            ));
        }
        let n = prep.slots().len();
        for (key, size) in [
            ("inherits_group", n + 2),
            ("different_settings_to_system", n + 2),
        ] {
            if !prep.config[key]
                .as_array()
                .is_some_and(|a| a.len() == size && a.iter().all(Value::is_string))
            {
                return Err(invalid(
                    "Unvollständige Filament- oder Profilzuordnung in der Vorlage",
                ));
            }
        }
        Ok(prep)
    }
    pub fn override_process(
        &mut self,
        layer: f64,
        walls: u8,
        infill: u8,
        support: bool,
    ) -> io::Result<()> {
        let nozzle = self.nozzle()?;
        if !(0.02..=nozzle).contains(&layer) || !(1..=20).contains(&walls) || infill > 100 {
            return Err(invalid("Ungültige Schichthöhe, Wandzahl oder Füllung"));
        }
        for (k, v) in [
            ("layer_height", layer.to_string()),
            ("wall_loops", walls.to_string()),
            ("sparse_infill_density", format!("{infill}%")),
            ("enable_support", u8::from(support).to_string()),
        ] {
            self.config[k] = Value::String(v);
        }
        Ok(())
    }
    fn nozzle(&self) -> io::Result<f64> {
        self.config["nozzle_diameter"]
            .get(0)
            .and_then(Value::as_str)
            .and_then(|s| s.parse().ok())
            .filter(|v| (0.1..=2.).contains(v))
            .ok_or_else(|| invalid("Düsendurchmesser fehlt im Profil"))
    }
    fn validate(&self, p: &Project) -> io::Result<()> {
        let [x, y] = p.printer.bed;
        let expected = [(0., 0.), (x, 0.), (x, y), (0., y)];
        let area = self.config["printable_area"]
            .as_array()
            .ok_or_else(|| invalid("Druckbettkontur fehlt"))?;
        let points: Vec<_> = area
            .iter()
            .filter_map(|v| {
                let (a, b) = v.as_str()?.split_once('x')?;
                Some((a.parse::<f64>().ok()?, b.parse::<f64>().ok()?))
            })
            .collect();
        if points.len() != 4
            || !expected.iter().all(|e| points.contains(e))
            || (self.nozzle()? - p.printer.nozzle).abs() > 0.0001
        {
            return Err(invalid(
                "Bettmaße oder Düse der Vorlage passen nicht zum aktiven Druckerprofil",
            ));
        }
        if p.objects
            .iter()
            .any(|o| o.filament_slot > self.slots().len())
        {
            return Err(invalid("Eine Filamentnummer fehlt in der Vorlage"));
        }
        Ok(())
    }
}
pub fn export(path: &Path, p: &Project, prep: Option<&Preparation>) -> io::Result<()> {
    let Some(prep) = prep else {
        return files::save(path, p, files::Output::ThreeMf, false);
    };
    if !p.valid()
        || p.objects.is_empty()
        || !path
            .extension()
            .is_some_and(|s| s.eq_ignore_ascii_case("3mf"))
    {
        return Err(invalid(
            "Gültige Platte und Ausgabename mit Endung .3mf erforderlich",
        ));
    }
    prep.validate(p)?;
    let mut tmp = tempfile::NamedTempFile::new_in(
        path.parent()
            .filter(|s| !s.as_os_str().is_empty())
            .unwrap_or(Path::new(".")),
    )?;
    {
        let mut writer = BufWriter::new(tmp.as_file_mut());
        files::three_mf(&mut writer, p, Some(prep))?;
        writer.flush()?;
    }
    tmp.as_file().sync_all()?;
    tmp.persist_noclobber(path)?;
    Ok(())
}
pub fn detect() -> Option<PathBuf> {
    let mut paths = vec![PathBuf::from(
        "/Applications/BambuStudio.app/Contents/MacOS/BambuStudio",
    )];
    if let Some(root) = std::env::var_os("ProgramFiles") {
        paths.push(
            PathBuf::from(root)
                .join("Bambu Studio")
                .join("bambu-studio.exe"),
        )
    }
    if let Some(path) = std::env::var_os("PATH") {
        paths.extend(
            std::env::split_paths(&path)
                .filter(|p| p.is_absolute())
                .map(|p| {
                    p.join(if cfg!(windows) {
                        "bambu-studio.exe"
                    } else {
                        "bambu-studio"
                    })
                }),
        )
    }
    paths.into_iter().find(|p| p.is_file())
}
pub fn launch(exe: &Path, model: &Path) -> io::Result<()> {
    if !exe.is_absolute() || !exe.is_file() {
        return Err(invalid(
            "Bitte den vollständigen Pfad zum installierten Bambu Studio angeben",
        ));
    }
    Command::new(exe).arg(model.canonicalize()?).spawn()?;
    Ok(())
}
