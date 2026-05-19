# keyring

Store, retrieve, and delete secrets in the system credential store via per-platform backends.

```rust
use onyx::keyring;
```

## API

| Symbol | Kind | Summary |
|--------|------|---------|
| `set(service: &str, account: &str, secret: &str) -> Result<(), KeyringError>` | fn | Store or overwrite a secret. |
| `get(service: &str, account: &str) -> Result<String, KeyringError>` | fn | Read the stored secret. |
| `delete(service: &str, account: &str) -> Result<(), KeyringError>` | fn | Remove the entry. |
| `KeyringError::EmptyService` | variant | Service string empty after trim. |
| `KeyringError::EmptyAccount` | variant | Account string empty after trim. |
| `KeyringError::NotFound` | variant | Entry is not in the credential store. |
| `KeyringError::Unavailable` | variant | No supported backend reachable. |
| `KeyringError::Backend { action, backend, source }` | variant | Backend located but failed. |

`action` is `"set" | "get" | "delete"`. `backend` is `"security" | "secret-tool" | "cmdkey" | "powershell"`.

## Platform backends

| Platform | Backend |
|----------|---------|
| macOS | `security add-generic-password / find-generic-password / delete-generic-password` |
| Linux | `secret-tool store / lookup / clear` (libsecret over D-Bus) |
| Windows | `cmdkey /generic` for write/delete, PowerShell + `CredentialManager` for read |

The Linux backend requires a running Secret Service implementation (`gnome-keyring`, `kwallet5`, `KeePassXC` with the Secret Service plugin). On headless CI, install `gnome-keyring` and start a session daemon — or accept that `get`/`set` will return `Unavailable`.

The Windows read path uses PowerShell because `cmdkey` does not print stored passwords. The script imports `CredentialManager` (a PowerShell Gallery module) — when the module is absent, the call returns `Unavailable` with exit code `2`.

## Examples

```rust
use onyx::keyring;

keyring::set("io.akira.unified-dev", "kid@example.com", "hunter2")?;

match keyring::get("io.akira.unified-dev", "kid@example.com") {
    Ok(secret) => use_token(&secret),
    Err(onyx::keyring::KeyringError::NotFound) => prompt_login(),
    Err(err) => return Err(err.into()),
}

keyring::delete("io.akira.unified-dev", "kid@example.com")?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Service / account convention:

- `service` — application identifier, typically the bundle id (`io.akira.unified-dev`).
- `account` — customer identifier within that app (email, user id, or `"default"`).

This matches the macOS `kSecAttrService` / `kSecAttrAccount` pair and the Linux `secret-tool` `service` / `account` attribute schema.

## Behaviour

- Validation runs **before** any IPC. Empty service or account returns `EmptyService` / `EmptyAccount` without touching the OS keychain.
- The macOS backend uses `add-generic-password -U` so a second `set` overwrites the previous secret without prompting.
- Linux backend writes the secret to `secret-tool`'s stdin (not as a CLI argument) so it does not appear in process listings.
- Windows backend joins `service:account` into the credential target name — `delete` and `get` reconstruct the same join.
- Trailing newlines from the backend output are stripped on `get`.

## Errors

- `EmptyService` / `EmptyAccount` — caller passed empty (or whitespace-only) values.
- `NotFound` — backend reported the entry is missing. macOS uses exit code 44; Linux uses 1; Windows uses PowerShell exit code 3.
- `Unavailable` — no backend reachable (unsupported OS, or Windows without the `CredentialManager` PowerShell module).
- `Backend { action, backend, source }` — backend was located but the call failed. Inspect `source` for the underlying IO error.

## Security notes

- `onyx` does not zero the returned `String`. Wrap with `secrecy::SecretString` (or your own) at the call site if you need stronger guarantees.
- Secrets are passed as CLI arguments on macOS and Windows (`security -w` and `cmdkey /pass`). On a system you do not control, this may be visible in `ps`. The Linux backend writes via stdin to avoid this.
- The Windows backend invokes PowerShell with `-NoProfile` to skip user-customised profile scripts.

## Dependencies

- `osinfo` — `Platform::current()` selects the per-OS code path.

## Related modules

- [`paths`](25-paths.md) — choose a stable location to mirror non-secret config alongside the keychain entry.

## Cross-crate parity

Mirrors the Go crate's `keyring` package one-to-one: same backend choice, same exit-code parsing, same stdin handling.

---

Navigation: [← Files](21-files.md) · **Keyring** · [Notify →](23-notify.md)
