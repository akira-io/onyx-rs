//! Per-application platform paths (config, data, cache, logs).
//!
//! Follows macOS conventions (`~/Library/...`), XDG Base Directory on
//! Linux, and Known Folders on Windows.

use std::path::PathBuf;

use crate::osinfo::Platform;

#[derive(Debug, thiserror::Error)]
pub enum PathError {
    #[error("paths: application name is required")]
    MissingApplicationName,
    #[error("paths: home directory unavailable")]
    HomeUnavailable,
    #[error("paths: environment variable {0} not set")]
    EnvNotSet(&'static str),
}

pub struct AppPaths {
    name: String,
    platform: Platform,
}

pub fn for_app(application_name: impl Into<String>) -> AppPaths {
    AppPaths {
        name: application_name.into(),
        platform: Platform::current(),
    }
}

impl AppPaths {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn config(&self) -> Result<PathBuf, PathError> {
        self.require_name()?;
        if self.platform.is_darwin() {
            return join_under_home(["Library", "Application Support", &self.name]);
        }
        if self.platform.is_linux() {
            return resolve_xdg("XDG_CONFIG_HOME", ".config", &self.name);
        }
        if self.platform.is_windows() {
            return join_under_env("APPDATA", &self.name);
        }
        fallback_config(&self.name)
    }

    pub fn data(&self) -> Result<PathBuf, PathError> {
        self.require_name()?;
        if self.platform.is_darwin() {
            return join_under_home(["Library", "Application Support", &self.name]);
        }
        if self.platform.is_linux() {
            return resolve_xdg("XDG_DATA_HOME", ".local/share", &self.name);
        }
        if self.platform.is_windows() {
            return join_under_env("APPDATA", &self.name);
        }
        fallback_config(&self.name)
    }

    pub fn cache(&self) -> Result<PathBuf, PathError> {
        self.require_name()?;
        if self.platform.is_darwin() {
            return join_under_home(["Library", "Caches", &self.name]);
        }
        if self.platform.is_linux() {
            return resolve_xdg("XDG_CACHE_HOME", ".cache", &self.name);
        }
        if self.platform.is_windows() {
            let mut p = env_path("LOCALAPPDATA")?;
            p.push(&self.name);
            p.push("Cache");
            return Ok(p);
        }
        fallback_cache(&self.name)
    }

    pub fn logs(&self) -> Result<PathBuf, PathError> {
        self.require_name()?;
        if self.platform.is_darwin() {
            return join_under_home(["Library", "Logs", &self.name]);
        }
        if self.platform.is_linux() {
            let mut suffix = String::from(&*self.name);
            suffix.push_str("/logs");
            return resolve_xdg("XDG_STATE_HOME", ".local/state", &suffix);
        }
        if self.platform.is_windows() {
            let mut p = env_path("LOCALAPPDATA")?;
            p.push(&self.name);
            p.push("Logs");
            return Ok(p);
        }
        let mut name = String::from(&*self.name);
        name.push_str("/logs");
        fallback_cache(&name)
    }

    fn require_name(&self) -> Result<(), PathError> {
        if self.name.is_empty() {
            return Err(PathError::MissingApplicationName);
        }
        Ok(())
    }
}

fn join_under_home<I, S>(segments: I) -> Result<PathBuf, PathError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let home = home_dir()?;
    let mut out = home;
    for s in segments {
        out.push(s.as_ref());
    }
    Ok(out)
}

fn join_under_env(env: &'static str, suffix: &str) -> Result<PathBuf, PathError> {
    let mut p = env_path(env)?;
    p.push(suffix);
    Ok(p)
}

fn env_path(env: &'static str) -> Result<PathBuf, PathError> {
    std::env::var_os(env)
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .ok_or(PathError::EnvNotSet(env))
}

fn resolve_xdg(
    env: &'static str,
    fallback_subdir: &str,
    suffix: &str,
) -> Result<PathBuf, PathError> {
    if let Some(explicit) = std::env::var_os(env) {
        if !explicit.is_empty() {
            let mut p = PathBuf::from(explicit);
            p.push(suffix);
            return Ok(p);
        }
    }
    let mut p = home_dir()?;
    p.push(fallback_subdir);
    p.push(suffix);
    Ok(p)
}

fn fallback_config(suffix: &str) -> Result<PathBuf, PathError> {
    let mut p = home_dir()?;
    p.push(".config");
    p.push(suffix);
    Ok(p)
}

fn fallback_cache(suffix: &str) -> Result<PathBuf, PathError> {
    let mut p = home_dir()?;
    p.push(".cache");
    p.push(suffix);
    Ok(p)
}

fn home_dir() -> Result<PathBuf, PathError> {
    let key = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    std::env::var_os(key)
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .ok_or(PathError::HomeUnavailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_name_errors() {
        let app = for_app("");
        assert!(matches!(
            app.config(),
            Err(PathError::MissingApplicationName)
        ));
    }

    #[test]
    fn config_returns_path_when_named() {
        let app = for_app("hyperion-test");
        let p = app.config().expect("config path");
        assert!(p.to_string_lossy().contains("hyperion-test"));
    }
}
