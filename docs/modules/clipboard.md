# clipboard

Read and write the system clipboard as plain text. Per-platform backends, no FFI, only stdlib + `thiserror`.

## API

| Symbol | Kind | Summary |
| --- | --- | --- |
| `read() -> Result<String, ClipboardError>` | fn | Returns current clipboard text. |
| `write(text: &str) -> Result<(), ClipboardError>` | fn | Replaces clipboard text. Empty string clears. |
| `ClipboardError::Unavailable` | variant | No supported backend is reachable. |
| `ClipboardError::Backend { action, backend, source }` | variant | A specific backend failed. |

## Platform behavior

| Platform | Backend (in order tried) |
| --- | --- |
| macOS | `pbcopy` / `pbpaste` (always present). |
| Windows | PowerShell hosting `System.Windows.Forms.Clipboard` (STA). |
| Linux | `wl-copy`/`wl-paste` (Wayland) → `xclip -selection clipboard` → `xsel --clipboard`. First successful one wins. |

On Linux, install one of `wl-clipboard`, `xclip`, or `xsel` to enable.

## Examples

```rust
use onyx::clipboard;

clipboard::write("hello")?;
let text = clipboard::read()?;
println!("{text}");
# Ok::<(), onyx::clipboard::ClipboardError>(())
```

## Errors

- `ClipboardError::Unavailable` — no backend reachable (only on Linux when wl-clipboard/xclip/xsel are all missing).
- `ClipboardError::Backend` — backend located but the call itself failed (binary missing permission, display server not running, etc.).

## Dependencies

- `osinfo` for platform detection.

## Related modules

- `files` — opening files with the default app.
- `shell` — resolving CLI binaries.
