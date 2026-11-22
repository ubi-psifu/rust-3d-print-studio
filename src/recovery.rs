//! Getrennte Sicherung je laufender Studio-Instanz. Nur die eigene Sitzung wird bereinigt.
use crate::{
    files::{self, Output},
    project::Project,
};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::SystemTime,
};
const FILENAME: &str = "Autosicherung.r3dp";

pub fn directory() -> io::Result<PathBuf> {
    let base = if cfg!(windows) {
        env::var_os("LOCALAPPDATA").map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        env::var_os("HOME").map(|p| PathBuf::from(p).join("Library/Application Support"))
    } else {
        env::var_os("XDG_STATE_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .or_else(|| env::var_os("HOME").map(|p| PathBuf::from(p).join(".local/state")))
    };
    Ok(base
        .filter(|p| p.is_absolute())
        .ok_or_else(|| io::Error::other("Benutzerordner fehlt"))?
        .join("Rust-3D-Print-Studio")
        .join("Sicherungen"))
}
pub fn list(root: &Path) -> io::Result<Vec<(SystemTime, PathBuf)>> {
    let mut result = Vec::new();
    let dirs = match fs::read_dir(root) {
        Ok(d) => d,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(result),
        Err(e) => return Err(e),
    };
    for dir in dirs {
        let dir = dir?;
        if !dir.file_name().to_string_lossy().starts_with("Sitzung-") || !dir.file_type()?.is_dir()
        {
            continue;
        }
        let path = dir.path().join(FILENAME);
        if let Ok(meta) = fs::symlink_metadata(&path) {
            if meta.is_file() {
                result.push((meta.modified()?, path));
            }
        }
    }
    result.sort_by(|a, b| b.cmp(a));
    Ok(result)
}
#[derive(Default)]
struct Store {
    folder: Option<PathBuf>,
}
impl Store {
    fn save(&mut self, root: &Path, p: &Project) -> io::Result<()> {
        if self.folder.is_none() {
            fs::create_dir_all(root)?;
            self.folder = Some(
                tempfile::Builder::new()
                    .prefix("Sitzung-")
                    .tempdir_in(root)?
                    .keep(),
            );
        }
        files::save(
            &self.folder.as_ref().unwrap().join(FILENAME),
            p,
            Output::Project,
            true,
        )
    }
    fn clear(&mut self) -> io::Result<()> {
        if let Some(dir) = &self.folder {
            match fs::remove_file(dir.join(FILENAME)) {
                Ok(()) => (),
                Err(e) if e.kind() == io::ErrorKind::NotFound => (),
                Err(e) => return Err(e),
            }
            fs::remove_dir(dir)?;
            self.folder = None;
        }
        Ok(())
    }
}
enum Command {
    Save(Box<Project>),
    Clear,
}
pub struct Worker {
    tx: Option<mpsc::Sender<Command>>,
    errors: mpsc::Receiver<String>,
    thread: Option<thread::JoinHandle<()>>,
    busy: std::sync::Arc<std::sync::atomic::AtomicBool>,
}
impl Default for Worker {
    fn default() -> Self {
        use std::sync::atomic::{AtomicBool, Ordering};
        let (tx, rx) = mpsc::channel();
        let (fail, errors) = mpsc::channel();
        let busy = std::sync::Arc::new(AtomicBool::new(false));
        let active = busy.clone();
        let thread = thread::spawn(move || {
            let mut store = Store::default();
            for cmd in rx {
                let result = match cmd {
                    Command::Save(p) => {
                        let r = directory().and_then(|dir| store.save(&dir, &p));
                        active.store(false, Ordering::Release);
                        r
                    }
                    Command::Clear => store.clear(),
                };
                if let Err(e) = result {
                    let _ = fail.send(format!("Autosicherung: {e}"));
                }
            }
        });
        Self {
            tx: Some(tx),
            errors,
            thread: Some(thread),
            busy,
        }
    }
}
impl Worker {
    pub fn checkpoint(&self, p: &Project) {
        use std::sync::atomic::Ordering;
        if !self.busy.swap(true, Ordering::AcqRel) {
            if let Some(tx) = &self.tx {
                let _ = tx.send(Command::Save(Box::new(p.clone())));
            }
        }
    }
    pub fn clear(&self) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(Command::Clear);
        }
    }
    pub fn error(&self) -> Option<String> {
        self.errors.try_recv().ok()
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        self.tx.take();
        if let Some(worker) = self.thread.take() {
            let _ = worker.join();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sessions_survive_restart_and_clear_independently() {
        let dir = tempfile::tempdir().unwrap();
        let mut first = Store::default();
        let mut second = Store::default();
        let mut p = Project::default();
        first.save(dir.path(), &p).unwrap();
        p.name = "Wiederherstellen".into();
        first.save(dir.path(), &p).unwrap();
        second.save(dir.path(), &p).unwrap();
        assert_eq!(list(dir.path()).unwrap().len(), 2);
        second.clear().unwrap();
        drop(first);
        let items = list(dir.path()).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(files::load(&items[0].1).unwrap().name, p.name);
    }
}
