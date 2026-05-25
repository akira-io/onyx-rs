# appearance

Reports the operating system's current color scheme so applications can match the native light or dark theme.

```rust
use onyx::appearance;
```

## API

| Symbol | Kind | Summary |
|--------|------|---------|
| `is_dark() -> bool` | fn | True when the OS is using a dark color scheme. |

## Platform behavior

| OS | Source |
|----|--------|
| macOS | `defaults read -g AppleInterfaceStyle` contains `Dark`. |
| Windows | Registry `AppsUseLightTheme` under `Themes\Personalize` is `0x0`. |
| Linux | `gsettings` `color-scheme` (then `gtk-theme`) contains `dark`. |

## Behaviour

Best-effort. When the preference cannot be read, `is_dark()` returns `false` (light). The value can change at runtime when the user switches themes, so cache it only for a single render.

## Errors

`appearance` has no error type. `is_dark()` returns a plain `bool`.

## Dependencies

- [osinfo](./24-osinfo.md) to select the platform backend.

## Cross-crate parity

Mirrors the Go package's `IsDark`.

---

Navigation: [← Machine ID](27-machineid.md) · **Appearance** · [Process →](29-process.md)
