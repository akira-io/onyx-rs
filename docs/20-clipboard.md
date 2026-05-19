# clipboard

Read and write the system clipboard as plain text. Per-platform backends, no FFI, only stdlib + `thiserror`.

```rust
use onyx::clipboard;
```

## API

| Symbol | Kind | Summary |
|--------|------|---------|
| `read() -> Result<String, ClipboardError>` | fn | Returns current clipboard text. |
| `write(text: &str) -> Result<(), ClipboardError>` | fn | Replaces clipboard text. Empty string clears. |
| `ClipboardError::Unavailable` | variant | No supported backend is reachable. |
| `ClipboardError::Backend { action, backend, source }` | variant | A specific backend failed. |

`ClipboardError` derives `thiserror::Error`. `action` is `"read"` or `"write"`. `backend` is the binary name (`"pbpaste"`, `"wl-copy"`, etc.). `source` is the underlying `std::io::Error`.

## Platform backends

| Platform | Backend (priority order) |
|----------|--------------------------|
| macOS | `pbcopy` / `pbpaste` (always present) |
| Windows | PowerShell hosting `System.Windows.Forms.Clipboard` (STA) |
| Linux | `wl-copy` / `wl-paste` (Wayland) → `xclip -selection clipboard` → `xsel --clipboard`. First successful one wins. |

On Linux, install one of `wl-clipboard`, `xclip`, or `xsel` to enable. The Wayland backend is preferred because it works on both Wayland and X11 sessions when XWayland is available.

## Behaviour

- Trailing newlines and carriage returns are stripped from the clipboard contents on read — backends emit them inconsistently.
- `write("")` clears the clipboard on all platforms.
- The Windows backend passes the payload via the `ONYX_CLIP_TEXT` environment variable, not as a command-line argument, to avoid escaping issues with quotes, backticks, and newlines.

## Examples

```rust
use onyx::clipboard;

clipboard::write("hello onyx")?;
let text = clipboard::read()?;
assert_eq!(text, "hello onyx");
# Ok::<(), onyx::clipboard::ClipboardError>(())
```

Best-effort branching when the user might not have a clipboard tool installed:

```rust
match onyx::clipboard::read() {
    Ok(text) => println!("clipboard: {text}"),
    Err(onyx::clipboard::ClipboardError::Unavailable) => {
        eprintln!("install wl-clipboard, xclip, or xsel");
    }
    Err(err) => return Err(err.into()),
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Errors

- `Unavailable` — no backend reachable. Only on Linux when `wl-clipboard`/`xclip`/`xsel` are all missing, or on unsupported OS targets.
- `Backend { action, backend, source }` — backend located but the call itself failed (binary missing permission, display server not running, sandbox preventing IPC).

The exit code of a failed backend is collapsed into `std::io::Error::other(...)` so callers do not need to import platform-specific status types.

## Dependencies

- `osinfo` — `Platform::current()` selects the per-OS code path.

## Related modules

- [`files`](21-files.md) — opening files with the default app.
- [`shell`](26-shell.md) — resolving CLI binaries when you want to detect which backend is present.

## Cross-crate parity

Mirrors the Go crate's `clipboard` package one-to-one: same backend priority, same `Unavailable` semantics, same trimming behaviour.

---

Navigation: [← Release flow](05-release-flow.md) · **Clipboard** · [Files →](21-files.md)
