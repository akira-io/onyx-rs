//! Show desktop notifications via per-platform backends. Mirrors the
//! Go `onyx/notify` package.

use std::process::Command;

use crate::osinfo::Platform;

#[derive(Debug, thiserror::Error)]
pub enum NotifyError {
    #[error("notify: title must not be empty")]
    EmptyTitle,
    #[error("notify: no supported backend available")]
    Unavailable,
    #[error("notify via {backend}: {source}")]
    Backend {
        backend: &'static str,
        #[source]
        source: std::io::Error,
    },
}

pub fn show(title: &str, body: &str) -> Result<(), NotifyError> {
    if title.trim().is_empty() {
        return Err(NotifyError::EmptyTitle);
    }
    let platform = Platform::current();
    if platform.is_darwin() {
        return show_darwin(title, body);
    }
    if platform.is_linux() {
        return show_linux(title, body);
    }
    if platform.is_windows() {
        return show_windows(title, body);
    }
    Err(NotifyError::Unavailable)
}

fn show_darwin(title: &str, body: &str) -> Result<(), NotifyError> {
    let script = format!(
        "display notification {} with title {}",
        quote_applescript(body),
        quote_applescript(title),
    );
    let status = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .status()
        .map_err(|e| NotifyError::Backend {
            backend: "osascript",
            source: e,
        })?;
    if !status.success() {
        return Err(NotifyError::Backend {
            backend: "osascript",
            source: std::io::Error::other("osascript exited non-zero"),
        });
    }
    Ok(())
}

fn show_linux(title: &str, body: &str) -> Result<(), NotifyError> {
    let status = Command::new("notify-send")
        .arg(title)
        .arg(body)
        .status()
        .map_err(|e| NotifyError::Backend {
            backend: "notify-send",
            source: e,
        })?;
    if !status.success() {
        return Err(NotifyError::Backend {
            backend: "notify-send",
            source: std::io::Error::other("notify-send exited non-zero"),
        });
    }
    Ok(())
}

fn show_windows(title: &str, body: &str) -> Result<(), NotifyError> {
    let burnt = format!(
        "if (Get-Module -ListAvailable -Name BurntToast) {{ Import-Module BurntToast; New-BurntToastNotification -Text {}, {}; exit 0 }} else {{ exit 1 }}",
        quote_powershell(title),
        quote_powershell(body),
    );
    if Command::new("powershell")
        .args(["-NoProfile", "-Command", &burnt])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        return Ok(());
    }
    let msg = format!("{title}\n{body}");
    if Command::new("msg")
        .arg("*")
        .arg(msg)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        return Ok(());
    }
    Err(NotifyError::Unavailable)
}

fn quote_applescript(s: &str) -> String {
    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

fn quote_powershell(s: &str) -> String {
    let escaped = s.replace('\'', "''");
    format!("'{escaped}'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_title() {
        assert!(matches!(show("", "body"), Err(NotifyError::EmptyTitle)));
        assert!(matches!(show("   ", "body"), Err(NotifyError::EmptyTitle)));
    }

    #[test]
    fn applescript_escapes_quotes_and_backslashes() {
        let got = quote_applescript(r#"he said "hi" \\path"#);
        assert_eq!(got, r#""he said \"hi\" \\\\path""#);
    }

    #[test]
    fn powershell_doubles_single_quotes() {
        assert_eq!(quote_powershell("it's fine"), "'it''s fine'");
    }

    #[test]
    fn show_does_not_panic_on_best_effort() {
        let _ = show("onyx-test", "body");
    }
}
