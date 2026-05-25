//! Reports the operating system's current color scheme so applications can
//! match the native light or dark theme.

use std::process::Command;

use crate::osinfo::Platform;

/// is_dark reports whether the operating system is using a dark color scheme.
/// Best-effort: returns `false` when the preference cannot be determined.
pub fn is_dark() -> bool {
    let platform = Platform::current();
    if platform.is_darwin() {
        return darwin_is_dark();
    }
    if platform.is_windows() {
        return windows_is_dark();
    }
    if platform.is_linux() {
        return linux_is_dark();
    }
    false
}

fn darwin_is_dark() -> bool {
    output_contains(
        Command::new("defaults").args(["read", "-g", "AppleInterfaceStyle"]),
        "dark",
    )
}

fn windows_is_dark() -> bool {
    output_contains(
        Command::new("reg").args([
            "query",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "/v",
            "AppsUseLightTheme",
        ]),
        "0x0",
    )
}

fn linux_is_dark() -> bool {
    if gsettings_contains("color-scheme") {
        return true;
    }
    gsettings_contains("gtk-theme")
}

fn gsettings_contains(key: &str) -> bool {
    output_contains(
        Command::new("gsettings").args(["get", "org.gnome.desktop.interface", key]),
        "dark",
    )
}

fn output_contains(command: &mut Command, needle: &str) -> bool {
    command
        .output()
        .map(|out| {
            String::from_utf8_lossy(&out.stdout)
                .to_lowercase()
                .contains(needle)
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_dark_is_stable_within_a_run() {
        assert_eq!(is_dark(), is_dark());
    }
}
