# paths

Per-application platform paths — config, data, cache, logs. Follows macOS conventions (`~/Library/...`), XDG Base Directory on Linux, and Known Folders on Windows.

```rust
use onyx::paths;
```

## API

| Symbol | Kind | Summary |
|--------|------|---------|
| `for_app(name: impl Into<String>) -> AppPaths` | fn | Build a path resolver for an application. |
| `AppPaths::name() -> &str` | method | The configured application name. |
| `AppPaths::config() -> Result<PathBuf, PathError>` | method | Per-user configuration directory. |
| `AppPaths::data() -> Result<PathBuf, PathError>` | method | Per-user data directory (state shared across machines). |
| `AppPaths::cache() -> Result<PathBuf, PathError>` | method | Per-user cache directory (regenerable). |
| `AppPaths::logs() -> Result<PathBuf, PathError>` | method | Per-user log directory. |
| `PathError::MissingApplicationName` | variant | `for_app("")` was called. |
| `PathError::HomeUnavailable` | variant | `HOME` / `USERPROFILE` not set. |
| `PathError::EnvNotSet(&'static str)` | variant | Required env var not set (e.g. `APPDATA`). |

`AppPaths` carries the application name and the cached `Platform::current()`. It is `Clone` but not `Copy` (owns a `String`).

## Path table

| Method | macOS | Linux | Windows |
|--------|-------|-------|---------|
| `config()` | `~/Library/Application Support/<name>` | `$XDG_CONFIG_HOME/<name>` or `~/.config/<name>` | `%APPDATA%\<name>` |
| `data()` | `~/Library/Application Support/<name>` | `$XDG_DATA_HOME/<name>` or `~/.local/share/<name>` | `%APPDATA%\<name>` |
| `cache()` | `~/Library/Caches/<name>` | `$XDG_CACHE_HOME/<name>` or `~/.cache/<name>` | `%LOCALAPPDATA%\<name>\Cache` |
| `logs()` | `~/Library/Logs/<name>` | `$XDG_STATE_HOME/<name>/logs` or `~/.local/state/<name>/logs` | `%LOCALAPPDATA%\<name>\Logs` |

`config` and `data` collapse to the same path on macOS and Windows — those platforms do not distinguish the two. On Linux they are separate (XDG defines distinct base directories).

## Examples

```rust
use onyx::paths;

let app = paths::for_app("hyperion");
let config = app.config()?;
let data   = app.data()?;
let cache  = app.cache()?;
let logs   = app.logs()?;

std::fs::create_dir_all(&config)?;
std::fs::write(config.join("settings.json"), "{}")?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

For a long-lived app, build `AppPaths` once and clone it as needed:

```rust
let app = paths::for_app("hyperion");
let cache = app.clone().cache()?;
let logs  = app.logs()?;
```

## Behaviour

- Paths are **constructed**, not created. The caller calls `std::fs::create_dir_all(path)?` before writing.
- `for_app("")` does not fail — the empty name is captured. The error comes from the first `config()`/`data()`/etc. call, which checks `require_name()`.
- XDG resolution: when `$XDG_CONFIG_HOME` (etc.) is set and non-empty, it wins. Otherwise the fallback is rooted at `$HOME`.
- Linux logs use `$XDG_STATE_HOME` (or `~/.local/state`) per the 2021 XDG update. Older `~/.cache/<name>/logs` is not used.
- Unknown platforms fall back to `~/.config/<name>` and `~/.cache/<name>` so the call still returns a path on unusual BSDs.

## Errors

- `MissingApplicationName` — empty `name` reached a path-returning method.
- `HomeUnavailable` — `HOME` (Unix) or `USERPROFILE` (Windows) is not set or empty. Unrecoverable: the OS environment is broken.
- `EnvNotSet(name)` — a Windows path required `APPDATA` / `LOCALAPPDATA` and the env var was missing. Same kind of broken-environment error.

## Dependencies

- `osinfo` — `Platform::current()` selects the per-OS code path.
- `std::env::var_os` for HOME / USERPROFILE / XDG_* / APPDATA / LOCALAPPDATA.

## Related modules

- [`files`](21-files.md) — open the directory you just resolved.
- [`keyring`](22-keyring.md) — keep secrets in the OS keychain, non-secrets in `paths::for_app(...)`.

## Cross-crate parity

Mirrors the Go crate's `paths` package one-to-one: same XDG semantics, same macOS `~/Library/...` paths, same Windows `Known Folder` mapping, same `Logs` subdirectory convention.

---

Navigation: [← OS info](24-osinfo.md) · **Paths** · [Shell →](26-shell.md)
