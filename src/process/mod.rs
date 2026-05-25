//! Launches and relaunches the host application across platforms, hiding the
//! per-OS command needed to open a fresh instance.

use std::process::Command;

use thiserror::Error;

use crate::osinfo::Platform;
use crate::shell::{Resolver, ShellError};

/// Errors returned by [`relaunch`].
#[derive(Debug, Error)]
pub enum ProcessError {
    /// The application path was empty.
    #[error("process: application path required")]
    EmptyPath,
    /// The platform launcher (`open`) could not be located.
    #[error("process: locate launcher: {0}")]
    Locate(#[from] ShellError),
    /// The new process could not be started.
    #[error("process: relaunch: {0}")]
    Spawn(#[from] std::io::Error),
}

/// relaunch starts a fresh instance of the application at the given path. The
/// caller quits the current process afterwards. On macOS the path is the
/// `.app` bundle, on Windows the executable, and on Linux the binary.
pub fn relaunch(application_path: &str) -> Result<(), ProcessError> {
    if application_path.is_empty() {
        return Err(ProcessError::EmptyPath);
    }
    let platform = Platform::current();
    if platform.is_darwin() {
        let opener = Resolver::new()
            .lookups(["open", "/usr/bin/open"])
            .resolve()?;
        Command::new(opener)
            .args(["-n", application_path])
            .spawn()?;
        return Ok(());
    }
    if platform.is_windows() {
        Command::new("cmd")
            .args(["/c", "start", "", application_path])
            .spawn()?;
        return Ok(());
    }
    Command::new(application_path).spawn()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_path() {
        assert!(matches!(relaunch(""), Err(ProcessError::EmptyPath)));
    }
}
