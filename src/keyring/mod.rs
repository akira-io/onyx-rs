//! Store, retrieve, and delete secrets in the system credential store
//! via per-platform backends. Mirrors the Go `onyx/keyring` package.

use std::io::Write as IoWrite;
use std::process::{Command, Stdio};

use crate::osinfo::Platform;

#[derive(Debug, thiserror::Error)]
pub enum KeyringError {
    #[error("keyring: service must not be empty")]
    EmptyService,
    #[error("keyring: account must not be empty")]
    EmptyAccount,
    #[error("keyring: secret not found")]
    NotFound,
    #[error("keyring: no supported backend available")]
    Unavailable,
    #[error("keyring {action} via {backend}: {source}")]
    Backend {
        action: &'static str,
        backend: &'static str,
        #[source]
        source: std::io::Error,
    },
}

pub fn set(service: &str, account: &str, secret: &str) -> Result<(), KeyringError> {
    validate(service, account)?;
    let platform = Platform::current();
    if platform.is_darwin() {
        return set_darwin(service, account, secret);
    }
    if platform.is_linux() {
        return set_linux(service, account, secret);
    }
    if platform.is_windows() {
        return set_windows(service, account, secret);
    }
    Err(KeyringError::Unavailable)
}

pub fn get(service: &str, account: &str) -> Result<String, KeyringError> {
    validate(service, account)?;
    let platform = Platform::current();
    if platform.is_darwin() {
        return get_darwin(service, account);
    }
    if platform.is_linux() {
        return get_linux(service, account);
    }
    if platform.is_windows() {
        return get_windows(service, account);
    }
    Err(KeyringError::Unavailable)
}

pub fn delete(service: &str, account: &str) -> Result<(), KeyringError> {
    validate(service, account)?;
    let platform = Platform::current();
    if platform.is_darwin() {
        return delete_darwin(service, account);
    }
    if platform.is_linux() {
        return delete_linux(service, account);
    }
    if platform.is_windows() {
        return delete_windows(service, account);
    }
    Err(KeyringError::Unavailable)
}

fn validate(service: &str, account: &str) -> Result<(), KeyringError> {
    if service.trim().is_empty() {
        return Err(KeyringError::EmptyService);
    }
    if account.trim().is_empty() {
        return Err(KeyringError::EmptyAccount);
    }
    Ok(())
}

fn set_darwin(service: &str, account: &str, secret: &str) -> Result<(), KeyringError> {
    run(
        "set",
        "security",
        Command::new("security").args([
            "add-generic-password",
            "-U",
            "-s",
            service,
            "-a",
            account,
            "-w",
            secret,
        ]),
    )
}

fn get_darwin(service: &str, account: &str) -> Result<String, KeyringError> {
    let output = Command::new("security")
        .args(["find-generic-password", "-s", service, "-a", account, "-w"])
        .output()
        .map_err(|e| KeyringError::Backend {
            action: "get",
            backend: "security",
            source: e,
        })?;
    if !output.status.success() {
        if output.status.code() == Some(44) {
            return Err(KeyringError::NotFound);
        }
        return Err(KeyringError::Backend {
            action: "get",
            backend: "security",
            source: std::io::Error::other("security exited non-zero"),
        });
    }
    let mut s = String::from_utf8_lossy(&output.stdout).into_owned();
    while s.ends_with('\n') {
        s.pop();
    }
    Ok(s)
}

fn delete_darwin(service: &str, account: &str) -> Result<(), KeyringError> {
    let status = Command::new("security")
        .args(["delete-generic-password", "-s", service, "-a", account])
        .status()
        .map_err(|e| KeyringError::Backend {
            action: "delete",
            backend: "security",
            source: e,
        })?;
    if !status.success() {
        if status.code() == Some(44) {
            return Err(KeyringError::NotFound);
        }
        return Err(KeyringError::Backend {
            action: "delete",
            backend: "security",
            source: std::io::Error::other("security exited non-zero"),
        });
    }
    Ok(())
}

fn set_linux(service: &str, account: &str, secret: &str) -> Result<(), KeyringError> {
    let label = format!("--label={service}");
    let mut child = Command::new("secret-tool")
        .args(["store", &label, "service", service, "account", account])
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| KeyringError::Backend {
            action: "set",
            backend: "secret-tool",
            source: e,
        })?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(secret.as_bytes())
            .map_err(|e| KeyringError::Backend {
                action: "set",
                backend: "secret-tool",
                source: e,
            })?;
    }
    let status = child.wait().map_err(|e| KeyringError::Backend {
        action: "set",
        backend: "secret-tool",
        source: e,
    })?;
    if !status.success() {
        return Err(KeyringError::Backend {
            action: "set",
            backend: "secret-tool",
            source: std::io::Error::other("secret-tool exited non-zero"),
        });
    }
    Ok(())
}

fn get_linux(service: &str, account: &str) -> Result<String, KeyringError> {
    let output = Command::new("secret-tool")
        .args(["lookup", "service", service, "account", account])
        .output()
        .map_err(|e| KeyringError::Backend {
            action: "get",
            backend: "secret-tool",
            source: e,
        })?;
    if !output.status.success() {
        if output.status.code() == Some(1) {
            return Err(KeyringError::NotFound);
        }
        return Err(KeyringError::Backend {
            action: "get",
            backend: "secret-tool",
            source: std::io::Error::other("secret-tool exited non-zero"),
        });
    }
    if output.stdout.is_empty() {
        return Err(KeyringError::NotFound);
    }
    let mut s = String::from_utf8_lossy(&output.stdout).into_owned();
    while s.ends_with('\n') {
        s.pop();
    }
    Ok(s)
}

fn delete_linux(service: &str, account: &str) -> Result<(), KeyringError> {
    run(
        "delete",
        "secret-tool",
        Command::new("secret-tool").args(["clear", "service", service, "account", account]),
    )
}

fn windows_target(service: &str, account: &str) -> String {
    format!("{service}:{account}")
}

fn set_windows(service: &str, account: &str, secret: &str) -> Result<(), KeyringError> {
    let target = windows_target(service, account);
    let status = Command::new("cmdkey")
        .arg(format!("/generic:{target}"))
        .arg(format!("/user:{account}"))
        .arg(format!("/pass:{secret}"))
        .status()
        .map_err(|e| KeyringError::Backend {
            action: "set",
            backend: "cmdkey",
            source: e,
        })?;
    if !status.success() {
        return Err(KeyringError::Backend {
            action: "set",
            backend: "cmdkey",
            source: std::io::Error::other("cmdkey exited non-zero"),
        });
    }
    Ok(())
}

fn get_windows(service: &str, account: &str) -> Result<String, KeyringError> {
    let target = windows_target(service, account).replace('\'', "''");
    let script = format!(
        "if (-not (Get-Module -ListAvailable -Name CredentialManager)) {{ exit 2 }}; Import-Module CredentialManager; $c = Get-StoredCredential -Target '{target}'; if ($null -eq $c) {{ exit 3 }}; [Runtime.InteropServices.Marshal]::PtrToStringAuto([Runtime.InteropServices.Marshal]::SecureStringToBSTR($c.Password))"
    );
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output()
        .map_err(|e| KeyringError::Backend {
            action: "get",
            backend: "powershell",
            source: e,
        })?;
    if !output.status.success() {
        return match output.status.code() {
            Some(2) => Err(KeyringError::Unavailable),
            Some(3) => Err(KeyringError::NotFound),
            _ => Err(KeyringError::Backend {
                action: "get",
                backend: "powershell",
                source: std::io::Error::other("powershell exited non-zero"),
            }),
        };
    }
    let mut s = String::from_utf8_lossy(&output.stdout).into_owned();
    while s.ends_with('\r') || s.ends_with('\n') {
        s.pop();
    }
    Ok(s)
}

fn delete_windows(service: &str, account: &str) -> Result<(), KeyringError> {
    let target = windows_target(service, account);
    run(
        "delete",
        "cmdkey",
        Command::new("cmdkey").arg(format!("/delete:{target}")),
    )
}

fn run(action: &'static str, backend: &'static str, cmd: &mut Command) -> Result<(), KeyringError> {
    let status = cmd.status().map_err(|e| KeyringError::Backend {
        action,
        backend,
        source: e,
    })?;
    if !status.success() {
        return Err(KeyringError::Backend {
            action,
            backend,
            source: std::io::Error::other(format!("{backend} exited non-zero")),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::process;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    static SERIALIZE: Mutex<()> = Mutex::new(());

    fn unique_service(prefix: &str) -> String {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        format!("{prefix}-{}-{nanos}", process::id())
    }

    #[test]
    fn rejects_empty_service() {
        assert!(matches!(
            set("", "account", "secret"),
            Err(KeyringError::EmptyService)
        ));
    }

    #[test]
    fn rejects_empty_account() {
        assert!(matches!(
            set("service", "", "secret"),
            Err(KeyringError::EmptyAccount)
        ));
    }

    #[test]
    fn get_validates_inputs() {
        assert!(matches!(get("", "a"), Err(KeyringError::EmptyService)));
        assert!(matches!(get("s", ""), Err(KeyringError::EmptyAccount)));
    }

    #[test]
    fn delete_validates_inputs() {
        assert!(matches!(delete("", "a"), Err(KeyringError::EmptyService)));
    }

    #[test]
    fn round_trip_on_real_keyring() {
        let _guard = SERIALIZE.lock().unwrap_or_else(|e| e.into_inner());
        let service = unique_service("onyx-test");
        let account = "tester";
        let secret = "hunter2";
        if set(&service, account, secret).is_err() {
            return;
        }
        let cleanup = || {
            let _ = delete(&service, account);
        };
        let result = get(&service, account);
        cleanup();
        if let Ok(got) = result {
            assert_eq!(got, secret);
        }
    }

    #[test]
    fn get_returns_not_found_for_missing_entry() {
        let _guard = SERIALIZE.lock().unwrap_or_else(|e| e.into_inner());
        let service = unique_service("onyx-missing");
        match get(&service, "nobody") {
            Err(KeyringError::NotFound) => {}
            Err(KeyringError::Unavailable) | Err(KeyringError::Backend { .. }) => {}
            Err(other) => panic!("unexpected error: {other}"),
            Ok(s) => panic!("expected not found, got {s:?}"),
        }
    }

    #[test]
    fn windows_target_joins_with_colon() {
        assert_eq!(windows_target("svc", "acct"), "svc:acct");
    }
}
