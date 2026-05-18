# notify

Show desktop notifications. Per-platform backends, only stdlib + `thiserror`.

## API

| Symbol | Kind | Summary |
| --- | --- | --- |
| `show(title: &str, body: &str) -> Result<(), NotifyError>` | fn | Displays a notification. Title must be non-empty. |
| `NotifyError::EmptyTitle` | variant | Title is empty or whitespace-only. |
| `NotifyError::Unavailable` | variant | No supported backend is reachable. |
| `NotifyError::Backend { backend, source }` | variant | A specific backend failed. |

## Platform behavior

| Platform | Backend |
| --- | --- |
| macOS | `osascript -e 'display notification ...'`. Always present. |
| Linux | `notify-send` (from `libnotify-bin`). |
| Windows | PowerShell `BurntToast` module. Falls back to `msg.exe`. |

Linux requires `libnotify-bin`. Windows toasts require the `BurntToast` PowerShell module (`Install-Module BurntToast`); without it the call falls back to `msg.exe`.

## Examples

```rust
use onyx::notify;

notify::show("Hyperion", "Export finished.")?;
# Ok::<(), onyx::notify::NotifyError>(())
```

## Errors

- `NotifyError::EmptyTitle` — title cannot be empty.
- `NotifyError::Unavailable` — every backend failed.
- `NotifyError::Backend` — backend located but the call itself failed.

## Dependencies

- `osinfo` for platform detection.

## Related modules

- `files` — opening files with the default app.
- `clipboard` — clipboard read/write.
