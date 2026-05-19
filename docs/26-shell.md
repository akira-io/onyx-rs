# shell

Locate command-line executables. Each `lookup` target is treated as a `PATH` name when it has no separators, or as a file path otherwise.

```rust
use onyx::shell::{Resolver, ShellError};
```

## API

| Symbol | Kind | Summary |
|--------|------|---------|
| `Resolver` | struct | `Clone + Default` builder collecting lookup targets. |
| `Resolver::new()` | fn | Empty resolver. |
| `Resolver::lookup(self, target: impl Into<String>) -> Self` | method | Append one target. |
| `Resolver::lookups<I, S>(self, targets: I) -> Self` | method | Append many. |
| `Resolver::resolve() -> Result<PathBuf, ShellError>` | method | Try each target in order; first matching executable wins. |
| `ShellError::BinaryNotFound` | variant | Resolver could not find any of the targets. |
| `list_npm_global_bin_dirs() -> Vec<PathBuf>` | fn | Candidate npm global bin directories. |
| `list_user_local_bin_dirs() -> Vec<PathBuf>` | fn | `~/.local/bin`, `~/bin`. |
| `list_system_bin_dirs() -> Vec<PathBuf>` | fn | Platform-specific system bin directories. |
| `list_windows_application_dirs(app: &str) -> Vec<PathBuf>` | fn | `LOCALAPPDATA\Programs\<app>`, `ProgramFiles\<app>`, `ProgramFiles(x86)\<app>`. |

## Lookup semantics

`Resolver` accepts two kinds of targets:

- **Bare name** (`"claude"`, `"node"`) — searched on `PATH`. Same algorithm as `which`.
- **Path-like** (`"./bin/foo"`, `"/opt/homebrew/bin/claude"`, `"C:\Program Files\app.exe"`) — checked directly. The substring rules:
  - Contains `/` or `\` → path-like.
  - Has a Windows drive letter (`X:`) → path-like.

Targets are tried in the order they were appended. First one that resolves to an existing file wins. Empty inputs are skipped silently.

`is_executable_file` only checks `metadata.is_file()` — the crate does not look at the executable bit. Most desktop apps embed a sidecar with a known name; the file existence check is sufficient.

## Examples

```rust
use onyx::shell::Resolver;

let claude = Resolver::new()
    .lookup("claude")                                    // PATH
    .lookup("/opt/homebrew/bin/claude")                  // explicit
    .lookup("/Applications/Claude.app/Contents/MacOS/claude")
    .resolve()?;
# Ok::<(), onyx::shell::ShellError>(())
```

Cross-platform binary lookup using `executable_extension`:

```rust
use onyx::{osinfo::executable_extension, shell::Resolver};

let name = format!("hyperion{}", executable_extension());
let bin = Resolver::new()
    .lookup(name)
    .lookups([
        "/usr/local/bin/hyperion",
        "/opt/homebrew/bin/hyperion",
    ])
    .resolve()?;
# Ok::<(), onyx::shell::ShellError>(())
```

Building a candidate list dynamically:

```rust
use onyx::shell::{list_user_local_bin_dirs, list_system_bin_dirs, Resolver};

let candidates = list_user_local_bin_dirs()
    .into_iter()
    .chain(list_system_bin_dirs())
    .map(|d| d.join("hyperion").to_string_lossy().into_owned());

let bin = Resolver::new().lookups(candidates).resolve()?;
# Ok::<(), onyx::shell::ShellError>(())
```

## Bin-dir helpers

| Helper | Returns |
|--------|---------|
| `list_npm_global_bin_dirs()` | `~/.npm-global/bin`, `~/.local/share/npm/bin` on Unix; `%APPDATA%\npm` on Windows. |
| `list_user_local_bin_dirs()` | `~/.local/bin`, `~/bin` on Unix. Empty on Windows. |
| `list_system_bin_dirs()` | `/usr/local/bin`, `/opt/homebrew/bin`, `/usr/bin` on macOS; `/usr/local/bin`, `/usr/bin` on Linux. Empty on Windows. |
| `list_windows_application_dirs(app)` | `LOCALAPPDATA\Programs\<app>`, `ProgramFiles\<app>`, `ProgramFiles(x86)\<app>`. Empty on non-Windows or when `app` is empty. |

The helpers consult environment variables (`HOME`, `APPDATA`, `LOCALAPPDATA`, `ProgramFiles`, `ProgramFiles(x86)`). Missing vars yield an empty vector — the helper never errors.

## Behaviour

- `Resolver::lookup("")` is a no-op (empty strings are filtered out before storage).
- `Resolver::resolve()` returns `BinaryNotFound` when no target matches. Always check the result.
- The crate does not spawn the resolved binary — that is the caller's job. Use `std::process::Command::new(resolved_path)`.
- Path-like targets that exist but are not regular files (e.g. directories) are skipped. A future version may add an `executable` check on Unix; for now, the metadata file-check is enough for shipped sidecars.

## Errors

- `BinaryNotFound` — no target resolved. The variant carries no payload — log the targets you tried at the call site if you need diagnostics.

## Dependencies

- `osinfo` — `Platform::current()` drives the OS-specific bin-dir lists.
- `std::env::var_os` for `HOME`, `APPDATA`, `LOCALAPPDATA`, `ProgramFiles`, `ProgramFiles(x86)`, `PATH`.

## Related modules

- [`files`](21-files.md) — open the binary's installer or repository page if missing.
- [`osinfo`](24-osinfo.md) — `executable_extension()` for `<name>.exe` on Windows.

## Cross-crate parity

Mirrors the Go crate's `shell` package one-to-one: same `Resolver` builder shape, same path-like heuristic, same priority order for bin-dir helpers.

---

Navigation: [← Paths](25-paths.md) · **Shell** · [Index ↑](00-index.md)
