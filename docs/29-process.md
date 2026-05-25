# process

Launches and relaunches the host application across platforms, hiding the per-OS command needed to open a fresh instance.

```rust
use onyx::process;
```

## API

| Symbol | Kind | Summary |
|--------|------|---------|
| `relaunch(application_path: &str) -> Result<(), ProcessError>` | fn | Starts a fresh instance of the application at the given path. |
| `ProcessError` | enum | `EmptyPath`, `Locate`, `Spawn`. |

## Platform behavior

| OS | Command |
|----|---------|
| macOS | `open -n <app>.app` (resolved via [shell](./26-shell.md)). |
| Windows | `cmd /c start "" <exe>`. |
| Linux | exec the binary directly. |

## Behaviour

`relaunch` only starts the new instance; the caller quits the current process afterwards (typically right after staging an update). The new process is detached.

## Errors

- `ProcessError::EmptyPath` when no path is supplied.
- `ProcessError::Locate` when the launcher cannot be found.
- `ProcessError::Spawn` when the process fails to start.

## Dependencies

- [osinfo](./24-osinfo.md) to select the platform command.
- [shell](./26-shell.md) to resolve `open` on macOS.

## Cross-crate parity

Mirrors the Go package's `Relaunch`.

---

Navigation: [← Appearance](28-appearance.md) · **Process** · [Index →](00-index.md)
