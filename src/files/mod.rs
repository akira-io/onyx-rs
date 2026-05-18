//! Open files and URLs with the user's default application, and reveal
//! a path in the platform file manager.

use std::path::Path;
use std::process::Command;

use crate::osinfo::Platform;

#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("files: path is required")]
    PathRequired,
    #[error("files: unsupported platform: {0}")]
    UnsupportedPlatform(&'static str),
    #[error("files: command failed: {0}")]
    Command(#[from] std::io::Error),
}

pub fn open_path(path: impl AsRef<Path>) -> Result<(), FileError> {
    let p = path.as_ref();
    if p.as_os_str().is_empty() {
        return Err(FileError::PathRequired);
    }
    run_open(p)
}

pub fn open_url(url: &str) -> Result<(), FileError> {
    if url.is_empty() {
        return Err(FileError::PathRequired);
    }
    run_open(url.as_ref())
}

pub fn reveal_in_file_manager(path: impl AsRef<Path>) -> Result<(), FileError> {
    let p = path.as_ref();
    if p.as_os_str().is_empty() {
        return Err(FileError::PathRequired);
    }
    let platform = Platform::current();
    if platform.is_darwin() {
        return spawn("open", &["-R", &p.to_string_lossy()]);
    }
    if platform.is_linux() {
        let dir = p.parent().unwrap_or(p);
        return spawn("xdg-open", &[&dir.to_string_lossy()]);
    }
    if platform.is_windows() {
        let arg = format!("/select,{}", p.to_string_lossy());
        return spawn("explorer", &[&arg]);
    }
    Err(FileError::UnsupportedPlatform(platform.as_str()))
}

fn run_open(target: &Path) -> Result<(), FileError> {
    let platform = Platform::current();
    let s = target.to_string_lossy();
    if platform.is_darwin() {
        return spawn("open", &[&s]);
    }
    if platform.is_linux() {
        return spawn("xdg-open", &[&s]);
    }
    if platform.is_windows() {
        return spawn("cmd", &["/c", "start", "", &s]);
    }
    Err(FileError::UnsupportedPlatform(platform.as_str()))
}

fn spawn(bin: &str, args: &[&str]) -> Result<(), FileError> {
    Command::new(bin).args(args).spawn()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_path_errors() {
        assert!(matches!(open_path(""), Err(FileError::PathRequired)));
        assert!(matches!(
            reveal_in_file_manager(""),
            Err(FileError::PathRequired)
        ));
    }

    #[test]
    fn empty_url_errors() {
        assert!(matches!(open_url(""), Err(FileError::PathRequired)));
    }
}
