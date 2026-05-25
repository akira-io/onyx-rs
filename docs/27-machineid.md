# machineid

A stable, per-application identifier for the current machine, persisted in the system keyring so it survives restarts and reinstalls of the application data.

```rust
use onyx::machineid;
```

## API

| Symbol | Kind | Summary |
|--------|------|---------|
| `get_or_create(application: &str) -> Result<String, MachineIdError>` | fn | Returns the machine identifier, creating one on first use. |
| `MachineIdError` | enum | `EmptyApplication`, `Keyring`. |

## Why

Desktop apps need a durable device identifier for licensing, telemetry, and device management. `machineid` keeps it in the OS keyring under the application's namespace, so it is stable and outside the app's own data directory.

## Behaviour

- First call generates a UUID v4 and stores it.
- Subsequent calls return the stored value unchanged.
- Scoped to `application`; two applications get independent identifiers.

## Errors

- `MachineIdError::EmptyApplication` when no application name is supplied.
- `MachineIdError::Keyring` when the credential store is unavailable.

## Dependencies

- [keyring](./22-keyring.md) for persistence; `uuid` for generation.

## Cross-crate parity

Mirrors the Go package's `GetOrCreate`.

---

Navigation: [← Shell](26-shell.md) · **Machine ID** · [Appearance →](28-appearance.md)
