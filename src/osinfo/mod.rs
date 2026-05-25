//! Single source of truth for platform identity.
//!
//! Every other module in `onyx` asks `osinfo` instead of switching on
//! `std::env::consts::OS` directly. Keeps cross-cutting platform
//! knowledge in one place.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Platform {
    identifier: &'static str,
}

impl Platform {
    pub fn current() -> Self {
        Self {
            identifier: std::env::consts::OS,
        }
    }

    pub fn is_darwin(self) -> bool {
        self.identifier == "macos"
    }

    pub fn is_linux(self) -> bool {
        self.identifier == "linux"
    }

    pub fn is_windows(self) -> bool {
        self.identifier == "windows"
    }

    pub fn as_str(self) -> &'static str {
        self.identifier
    }
}

pub fn executable_extension() -> &'static str {
    if Platform::current().is_windows() {
        ".exe"
    } else {
        ""
    }
}

/// hostname returns the operating system host name, when it can be determined.
///
/// Returns `None` if the host name is unavailable or not valid UTF-8, leaving
/// the fallback choice to the caller. Backed by the `gethostname` crate because
/// the standard library exposes no portable host-name API.
pub fn hostname() -> Option<String> {
    gethostname::gethostname().into_string().ok()
}

/// device_name returns a human-friendly name for the current machine, the name a
/// user recognizes in system settings, when it can be determined.
///
/// Falls back to [`hostname`] when no friendlier source is available, so it
/// returns `None` only when neither a friendly name nor the host name is
/// readable.
pub fn device_name() -> Option<String> {
    let platform = Platform::current();
    if platform.is_darwin() {
        if let Some(name) = command_output("scutil", &["--get", "ComputerName"]) {
            return Some(name);
        }
    }
    if platform.is_windows() {
        if let Some(name) = env_name("COMPUTERNAME") {
            return Some(name);
        }
    }
    if platform.is_linux() {
        if let Some(name) = command_output("hostnamectl", &["--pretty"]) {
            return Some(name);
        }
    }
    hostname()
}

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = std::process::Command::new(program)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    non_empty(String::from_utf8(output.stdout).ok()?)
}

fn env_name(key: &str) -> Option<String> {
    non_empty(std::env::var(key).ok()?)
}

fn non_empty(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_matches_os_constant() {
        assert_eq!(Platform::current().as_str(), std::env::consts::OS);
    }

    #[test]
    fn at_most_one_predicate_holds() {
        let p = Platform::current();
        let count = [p.is_darwin(), p.is_linux(), p.is_windows()]
            .into_iter()
            .filter(|x| *x)
            .count();
        assert!(
            count <= 1,
            "expected at most one predicate true, got {}",
            count
        );
    }

    #[test]
    fn executable_extension_matches_platform() {
        let got = executable_extension();
        if Platform::current().is_windows() {
            assert_eq!(got, ".exe");
        } else {
            assert_eq!(got, "");
        }
    }

    #[test]
    fn hostname_is_non_empty_when_present() {
        if let Some(name) = hostname() {
            assert!(!name.is_empty(), "hostname should not be an empty string");
        }
    }

    #[test]
    fn device_name_falls_back_to_hostname() {
        match device_name() {
            Some(name) => assert!(!name.is_empty(), "device name should not be empty"),
            None => assert!(
                hostname().is_none(),
                "device name should fall back to hostname"
            ),
        }
    }
}
