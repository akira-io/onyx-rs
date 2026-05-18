//! Locate command-line executables: PATH first, then well-known
//! install directories.

use std::path::{Path, PathBuf};

use crate::osinfo::Platform;

#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("shell: binary not found")]
    BinaryNotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionSource {
    Unknown,
    Path,
    Candidate,
}

impl ResolutionSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Path => "path",
            Self::Candidate => "candidate",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedExecutable {
    absolute_path: PathBuf,
    source: ResolutionSource,
}

impl ResolvedExecutable {
    pub fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    pub fn source(&self) -> ResolutionSource {
        self.source
    }
}

#[derive(Debug, Default, Clone)]
pub struct Candidates {
    names: Vec<String>,
    candidates: Vec<PathBuf>,
}

impl Candidates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        if !name.is_empty() {
            self.names.push(name);
        }
        self
    }

    pub fn with_candidate(mut self, path: impl Into<PathBuf>) -> Self {
        let p = path.into();
        if !p.as_os_str().is_empty() {
            self.candidates.push(p);
        }
        self
    }

    pub fn with_candidates<I, P>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<PathBuf>,
    {
        for p in paths {
            self = self.with_candidate(p);
        }
        self
    }

    pub fn resolve(&self) -> Result<ResolvedExecutable, ShellError> {
        for name in &self.names {
            if let Some(found) = look_path(name) {
                return Ok(ResolvedExecutable {
                    absolute_path: found,
                    source: ResolutionSource::Path,
                });
            }
        }
        for candidate in &self.candidates {
            if is_executable_file(candidate) {
                return Ok(ResolvedExecutable {
                    absolute_path: candidate.clone(),
                    source: ResolutionSource::Candidate,
                });
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
        let result = Candidates::new()
            .with_name("definitely-not-a-real-binary-xyz")
            .with_candidate("/definitely/not/a/path/binary")
            .resolve();
        assert!(matches!(result, Err(ShellError::BinaryNotFound)));
    }

    #[test]
    fn resolve_finds_explicit_candidate_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let bin = dir.path().join("fakebin");
        std::fs::write(&bin, "#!/bin/sh\nexit 0\n").expect("write");
        let result = Candidates::new()
            .with_name("definitely-not-a-real-binary-xyz")
            .with_candidate(&bin)
            .resolve()
            .expect("resolve");
        assert_eq!(result.absolute_path(), bin.as_path());
        assert_eq!(result.source(), ResolutionSource::Candidate);
    }

    #[test]
    fn ignores_empty_inputs() {
        let result = Candidates::new().with_name("").with_candidate("").resolve();
        assert!(matches!(result, Err(ShellError::BinaryNotFound)));
    }
}
