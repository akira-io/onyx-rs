//! Locate command-line executables. Each `lookup` target is treated as
//! a `PATH` name when it has no separators, or as a file path otherwise.

use std::path::{Path, PathBuf};

use crate::osinfo::Platform;

#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("shell: binary not found")]
    BinaryNotFound,
}

#[derive(Debug, Default, Clone)]
pub struct Resolver {
    targets: Vec<String>,
}

impl Resolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lookup(mut self, target: impl Into<String>) -> Self {
        let t = target.into();
        if !t.is_empty() {
            self.targets.push(t);
        }
        self
    }

    pub fn lookups<I, S>(mut self, targets: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for t in targets {
            self = self.lookup(t);
        }
        self
    }

    pub fn resolve(&self) -> Result<PathBuf, ShellError> {
        for t in &self.targets {
            if is_path_like(t) {
                let p = PathBuf::from(t);
                if is_executable_file(&p) {
                    return Ok(p);
                }
                continue;
            }
            if let Some(found) = look_path(t) {
                return Ok(found);
            }
        }
        Err(ShellError::BinaryNotFound)
    }
}

pub fn list_npm_global_bin_dirs() -> Vec<PathBuf> {
    let platform = Platform::current();
    let home = std::env::var_os("HOME").map(PathBuf::from);
    if platform.is_windows() {
        let mut out = Vec::new();
        if let Some(appdata) = std::env::var_os("APPDATA") {
            let mut p = PathBuf::from(appdata);
            p.push("npm");
            out.push(p);
        }
        return out;
    }
    if let Some(h) = home {
        return vec![
            h.join(".npm-global").join("bin"),
            h.join(".local/share/npm/bin"),
        ];
    }
    Vec::new()
}

pub fn list_user_local_bin_dirs() -> Vec<PathBuf> {
    if let Some(h) = std::env::var_os("HOME") {
        let home = PathBuf::from(h);
        return vec![home.join(".local/bin"), home.join("bin")];
    }
    Vec::new()
}

pub fn list_system_bin_dirs() -> Vec<PathBuf> {
    let platform = Platform::current();
    if platform.is_windows() {
        return Vec::new();
    }
    if platform.is_darwin() {
        return vec![
            PathBuf::from("/usr/local/bin"),
            PathBuf::from("/opt/homebrew/bin"),
            PathBuf::from("/usr/bin"),
        ];
    }
    vec![PathBuf::from("/usr/local/bin"), PathBuf::from("/usr/bin")]
}

pub fn list_windows_application_dirs(application_name: &str) -> Vec<PathBuf> {
    if !Platform::current().is_windows() || application_name.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    if let Some(v) = std::env::var_os("LOCALAPPDATA") {
        let mut p = PathBuf::from(v);
        p.push("Programs");
        p.push(application_name);
        out.push(p);
    }
    if let Some(v) = std::env::var_os("ProgramFiles") {
        let mut p = PathBuf::from(v);
        p.push(application_name);
        out.push(p);
    }
    if let Some(v) = std::env::var_os("ProgramFiles(x86)") {
        let mut p = PathBuf::from(v);
        p.push(application_name);
        out.push(p);
    }
    out
}

fn is_path_like(s: &str) -> bool {
    if s.contains('/') || s.contains('\\') {
        return true;
    }
    let bytes = s.as_bytes();
    bytes.len() >= 2 && bytes[1] == b':'
}

fn look_path(name: &str) -> Option<PathBuf> {
    let path_env = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_env) {
        let candidate = dir.join(name);
        if is_executable_file(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn is_executable_file(path: &Path) -> bool {
    match std::fs::metadata(path) {
        Ok(m) => m.is_file(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_fails_when_nothing_matches() {
        let result = Resolver::new()
            .lookup("definitely-not-a-real-binary-xyz")
            .lookup("/definitely/not/a/path/binary")
            .resolve();
        assert!(matches!(result, Err(ShellError::BinaryNotFound)));
    }

    #[test]
    fn resolve_finds_explicit_path() {
        let dir = tempfile::tempdir().expect("tempdir");
        let bin = dir.path().join("fakebin");
        std::fs::write(&bin, "#!/bin/sh\nexit 0\n").expect("write");
        let resolved = Resolver::new()
            .lookup("definitely-not-a-real-binary-xyz")
            .lookup(bin.to_string_lossy().to_string())
            .resolve()
            .expect("resolve");
        assert_eq!(resolved, bin);
    }

    #[test]
    fn ignores_empty_inputs() {
        let result = Resolver::new().lookup("").lookup("").resolve();
        assert!(matches!(result, Err(ShellError::BinaryNotFound)));
    }

    #[test]
    fn is_path_like_detects_separators() {
        assert!(!is_path_like("claude"));
        assert!(is_path_like("/opt/homebrew/bin/claude"));
        assert!(is_path_like("./bin/foo"));
        assert!(is_path_like(r"C:\Program Files\app.exe"));
        assert!(!is_path_like(""));
    }
}
