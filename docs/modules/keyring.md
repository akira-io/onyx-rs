# keyring

Store, retrieve, and delete secrets in the system credential store via per-platform backends. Only stdlib + `thiserror`.

## API

| Symbol | Kind | Summary |
| --- | --- | --- |
| `set(service: &str, account: &str, secret: &str) -> Result<(), KeyringError>` | fn | Stores or updates a secret. |
| `get(service: &str, account: &str) -> Result<String, KeyringError>` | fn | Returns the stored secret, or `NotFound`. |
| `delete(service: &str, account: &str) -> Result<(), KeyringError>` | fn | Removes a stored secret. |
| `KeyringError::EmptyService`, `EmptyAccount` | variants | Input validation failures. |
| `KeyringError::NotFound` | variant | No matching entry. |
| `KeyringError::Unavailable` | variant | No supported backend reachable. |
| `KeyringError::Backend { action, backend, source }` | variant | Backend invocation failed. |

## Platform behavior

| Platform | Backend |
| --- | --- |
| macOS | `security add/find/delete-generic-password` (Keychain). |
| Linux | `secret-tool store/lookup/clear` (libsecret, Secret Service). |
| Windows | `cmdkey` for write/delete; PowerShell `CredentialManager` module for read. |

Linux requires `libsecret-tools` + a running Secret Service provider. Windows read returns `Unavailable` if the `CredentialManager` PowerShell module is not installed.

## Examples

```rust
use onyx::keyring;

keyring::set("hyperion", "github_pat", "ghp_...")?;
match keyring::get("hyperion", "github_pat") {
    Ok(secret) => println!("got {} chars", secret.len()),
    Err(keyring::KeyringError::NotFound) => println!("first run"),
    Err(e) => return Err(e),
}
keyring::delete("hyperion", "github_pat")?;
# Ok::<(), onyx::keyring::KeyringError>(())
```

## Errors

- `EmptyService` / `EmptyAccount` — input validation.
- `NotFound` — entry does not exist.
- `Unavailable` — backend missing.
- `Backend` — backend located but invocation failed (locked keychain, user cancelled prompt, etc.).

## Dependencies

- `osinfo` for platform detection.

## Related modules

- `clipboard` — clipboard read/write.
- `files` — opening files with the default app.
