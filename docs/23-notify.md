# notify

Show desktop notifications via per-platform backends.

```rust
use onyx::notify;
```

## API

| Symbol | Kind | Summary |
|--------|------|---------|
| `show(title: &str, body: &str) -> Result<(), NotifyError>` | fn | Display a notification with title and body. |
| `NotifyError::EmptyTitle` | variant | Title was empty (after trim). |
| `NotifyError::Unavailable` | variant | No supported backend is reachable. |
| `NotifyError::Backend { backend, source }` | variant | Backend located but failed. |

`backend` is `"osascript" | "notify-send" | "powershell"`.

## Platform backends

| Platform | Backend (priority) |
|----------|--------------------|
| macOS | `osascript -e 'display notification "<body>" with title "<title>"'` |
| Linux | `notify-send <title> <body>` |
| Windows | `BurntToast` PowerShell module first; falls back to `msg *` |

`BurntToast` is a community PowerShell module that renders proper Windows 10/11 toasts. When it is not installed, `notify` falls back to `msg *`, which displays a blocking message box — workable but visually different from a toast.

Linux requires `notify-send` (`libnotify-bin` package on Debian/Ubuntu). The Wayland and X11 notification daemons listen on the same `org.freedesktop.Notifications` D-Bus service.

## Examples

```rust
use onyx::notify;

notify::show("Build complete", "Spectra v0.9.0 is ready to install.")?;
# Ok::<(), onyx::notify::NotifyError>(())
```

Best-effort branching when the user might not have a notifier:

```rust
if let Err(onyx::notify::NotifyError::Unavailable) = onyx::notify::show("Heads up", "Update available") {
    fall_back_to_logging();
}
```

## Behaviour

- Title is trimmed and rejected if empty — empty notifications are useless and most backends silently drop them.
- Body may be empty. The notifier displays a title-only toast.
- macOS: arguments are passed through an AppleScript-safe quoter that escapes `\\` and `"` — pasting user input as a title is safe.
- Windows: arguments are passed through a PowerShell-safe quoter that doubles `'` — `it's fine` becomes `'it''s fine'`.
- `notify-send` accepts the title and body as plain arguments; no extra escaping is needed because the spawn API does not invoke a shell.

## Errors

- `EmptyTitle` — caller passed an empty (or whitespace-only) title.
- `Unavailable` — Windows with neither BurntToast nor `msg`; or running on an unsupported OS.
- `Backend { backend, source }` — backend was reached but failed (non-zero exit code or IO error).

The exit code of a failed backend is collapsed into `std::io::Error::other(...)` so callers do not need to import platform-specific status types.

## Dependencies

- `osinfo` — `Platform::current()` selects the per-OS code path.

## Related modules

- [`files`](21-files.md) — open a follow-up action target from the notification handler.
- [`shell`](26-shell.md) — detect whether `notify-send` is on `PATH` before calling.

## Cross-crate parity

Mirrors the Go crate's `notify` package one-to-one: same backends, same priority order, same quoting rules.

---

Navigation: [← Keyring](22-keyring.md) · **Notify** · [OS info →](24-osinfo.md)
