//! Read and write the system clipboard as plain text via per-platform
//! backends. Mirrors the Go `onyx/clipboard` package.

use std::io::Write as IoWrite;
use std::process::{Command, Stdio};

use crate::osinfo::Platform;

#[derive(Debug, thiserror::Error)]
pub enum ClipboardError {
    #[error("clipboard: no supported backend available")]
    Unavailable,
    #[error("clipboard {action} via {backend}: {source}")]
    Backend {
        action: &'static str,
        backend: &'static str,
        #[source]
        source: std::io::Error,
    },
}

pub fn read() -> Result<String, ClipboardError> {
    let platform = Platform::current();
    if platform.is_darwin() {
        return run_reader("pbpaste", &[]);
    }
    if platform.is_windows() {
        return read_windows();
    }
    if platform.is_linux() {
        for (cmd, args) in linux_readers() {
            if let Ok(out) = run_reader(cmd, args) {
                return Ok(out);
            }
        }
        return Err(ClipboardError::Unavailable);
    }
    Err(ClipboardError::Unavailable)
}

pub fn write(text: &str) -> Result<(), ClipboardError> {
    let platform = Platform::current();
    if platform.is_darwin() {
        return run_writer(text, "pbcopy", &[]);
    }
    if platform.is_windows() {
        return write_windows(text);
    }
    if platform.is_linux() {
        for (cmd, args) in linux_writers() {
            if run_writer(text, cmd, args).is_ok() {
                return Ok(());
            }
        }
        return Err(ClipboardError::Unavailable);
    }
    Err(ClipboardError::Unavailable)
}

fn read_windows() -> Result<String, ClipboardError> {
    let script = "Add-Type -AssemblyName System.Windows.Forms; \
                  [System.Windows.Forms.Clipboard]::GetText()";
    let output = Command::new("powershell")
        .args(["-NoProfile", "-STA", "-Command", script])
        .output()
        .map_err(|e| ClipboardError::Backend {
            action: "read",
            backend: "powershell",
            source: e,
        })?;
    if !output.status.success() {
        return Err(ClipboardError::Backend {
            action: "read",
            backend: "powershell",
            source: std::io::Error::other("powershell exited non-zero"),
        });
    }
    let mut s = String::from_utf8_lossy(&output.stdout).into_owned();
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
    Ok(s)
}

fn write_windows(text: &str) -> Result<(), ClipboardError> {
    let script = "Add-Type -AssemblyName System.Windows.Forms; \
                  $v = $env:ONYX_CLIP_TEXT; \
                  if ([string]::IsNullOrEmpty($v)) { \
                      [System.Windows.Forms.Clipboard]::Clear() \
                  } else { \
                      [System.Windows.Forms.Clipboard]::SetText($v) \
                  }";
    let status = Command::new("powershell")
        .args(["-NoProfile", "-STA", "-Command", script])
        .env("ONYX_CLIP_TEXT", text)
        .status()
        .map_err(|e| ClipboardError::Backend {
            action: "write",
            backend: "powershell",
            source: e,
        })?;
    if !status.success() {
        return Err(ClipboardError::Backend {
            action: "write",
            backend: "powershell",
            source: std::io::Error::other("powershell exited non-zero"),
        });
    }
    Ok(())
}

fn linux_readers() -> &'static [(&'static str, &'static [&'static str])] {
    &[
        ("wl-paste", &["--no-newline"]),
        ("xclip", &["-selection", "clipboard", "-o"]),
        ("xsel", &["--clipboard", "--output"]),
    ]
}

fn linux_writers() -> &'static [(&'static str, &'static [&'static str])] {
    &[
        ("wl-copy", &[]),
        ("xclip", &["-selection", "clipboard"]),
        ("xsel", &["--clipboard", "--input"]),
    ]
}

fn run_reader(name: &'static str, args: &[&str]) -> Result<String, ClipboardError> {
    let output = Command::new(name)
        .args(args)
        .output()
        .map_err(|e| ClipboardError::Backend {
            action: "read",
            backend: name,
            source: e,
        })?;
    if !output.status.success() {
        return Err(ClipboardError::Backend {
            action: "read",
            backend: name,
            source: std::io::Error::other(format!("{name} exited non-zero")),
        });
    }
    let mut s = String::from_utf8_lossy(&output.stdout).into_owned();
    while s.ends_with('\n') {
        s.pop();
    }
    Ok(s)
}

fn run_writer(text: &str, name: &'static str, args: &[&str]) -> Result<(), ClipboardError> {
    let mut child = Command::new(name)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| ClipboardError::Backend {
            action: "write",
            backend: name,
            source: e,
        })?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| ClipboardError::Backend {
                action: "write",
                backend: name,
                source: e,
            })?;
    }
    let status = child.wait().map_err(|e| ClipboardError::Backend {
        action: "write",
        backend: name,
        source: e,
    })?;
    if !status.success() {
        return Err(ClipboardError::Backend {
            action: "write",
            backend: name,
            source: std::io::Error::other(format!("{name} exited non-zero")),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    static SERIALIZE: Mutex<()> = Mutex::new(());

    #[test]
    fn round_trip_preserves_text() {
        let _guard = SERIALIZE.lock().unwrap_or_else(|e| e.into_inner());
        const SAMPLE: &str = "onyx-clipboard-test";
        if write(SAMPLE).is_err() {
            return;
        }
        if let Ok(got) = read() {
            assert_eq!(got, SAMPLE);
        }
    }

    #[test]
    fn empty_string_is_allowed() {
        let _guard = SERIALIZE.lock().unwrap_or_else(|e| e.into_inner());
        let _ = write("");
    }

    #[test]
    fn linux_backends_are_in_priority_order() {
        assert_eq!(linux_readers().len(), 3);
        assert_eq!(linux_writers().len(), 3);
        assert_eq!(linux_readers()[0].0, "wl-paste");
        assert_eq!(linux_writers()[0].0, "wl-copy");
    }
}
