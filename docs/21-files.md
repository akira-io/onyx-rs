# files

Open files / URLs with the user's default application and reveal a path inside the platform file manager.

```rust
use onyx::files;
```

## API

| Symbol | Kind | Summary |
|--------|------|---------|
| `open_path(path: impl AsRef<Path>) -> Result<(), FileError>` | fn | Open `path` with the OS default app. |
| `open_url(url: &str) -> Result<(), FileError>` | fn | Open `url` in the default browser. |
| `reveal_in_file_manager(path: impl AsRef<Path>) -> Result<(), FileError>` | fn | Highlight `path` inside Finder / Explorer / Files. |
| `FileError::PathRequired` | variant | Empty path or URL. |
| `FileError::UnsupportedPlatform(&'static str)` | variant | Running on a platform `onyx` does not target. |
| `FileError::Command(std::io::Error)` | variant | Spawn failed. |

## Platform backends

| Platform | `open_path` / `open_url` | `reveal_in_file_manager` |
|----------|--------------------------|--------------------------|
| macOS | `open <target>` | `open -R <path>` |
| Linux | `xdg-open <target>` | `xdg-open <parent-dir>` |
| Windows | `cmd /c start "" <target>` | `explorer /select,<path>` |

Linux does not have a portable "highlight this file" command, so `reveal_in_file_manager` falls back to opening the parent directory. On macOS the `-R` flag tells Finder to select the file in its enclosing folder. On Windows, `/select` ensures Explorer opens with the file highlighted.

## Examples

```rust
use onyx::files;

files::open_path("/Users/me/Downloads/report.pdf")?;
files::open_url("https://akira.foundation")?;
files::reveal_in_file_manager("/Users/me/Downloads/report.pdf")?;
# Ok::<(), onyx::files::FileError>(())
```

Wails / Tauri integration:

```rust
#[tauri::command]
fn reveal(path: String) -> Result<(), String> {
    onyx::files::reveal_in_file_manager(&path).map_err(|e| e.to_string())
}
```

## Behaviour

- The spawned process is detached — `onyx` does not `wait()` on it. The OS default app launches in the background.
- An empty path or URL returns `PathRequired` immediately (no spawn).
- On unsupported platforms (BSD, Solaris, …), `UnsupportedPlatform` is returned. The `&'static str` is `osinfo::Platform::current().as_str()`.

## Errors

- `PathRequired` — caller passed an empty path or empty URL.
- `UnsupportedPlatform(name)` — `osinfo::Platform::current()` returned a value other than `macos`/`linux`/`windows`.
- `Command(io::Error)` — `Command::new(...).spawn()` failed. The wrapped error carries the underlying OS reason.

The exit code of the spawned helper is **not** checked — the helper has already detached by the time `spawn()` returns. The user sees the system app open; failures past that point are surfaced by the OS, not by `onyx`.

## Dependencies

- `osinfo` — `Platform::current()` selects the per-OS code path.

## Related modules

- [`paths`](25-paths.md) — choose where to write the file you are about to reveal.
- [`shell`](26-shell.md) — resolve a custom binary if you need to override the default app.

## Cross-crate parity

Mirrors the Go crate's `files` package one-to-one: same commands, same arguments, same parent-directory fallback on Linux.

---

Navigation: [← Clipboard](20-clipboard.md) · **Files** · [Keyring →](22-keyring.md)
