# osinfo

Single source of truth for platform identity. Every other module asks `osinfo` instead of branching on `std::env::consts::OS` directly.

```rust
use onyx::osinfo::{Platform, executable_extension};
```

## API

| Symbol | Kind | Summary |
|--------|------|---------|
| `Platform` | struct | `Copy + Clone + Eq + Hash` value wrapping the OS identifier. |
| `Platform::current()` | fn | Returns the host platform. |
| `Platform::is_darwin()` | method | macOS predicate. |
| `Platform::is_linux()` | method | Linux predicate. |
| `Platform::is_windows()` | method | Windows predicate. |
| `Platform::as_str()` | method | `&'static str` — `"macos" \| "linux" \| "windows" \| <other std::env::consts::OS>`. |
| `executable_extension()` | fn | `".exe"` on Windows, `""` otherwise. |

`Platform` derives `Debug, Clone, Copy, PartialEq, Eq`. The internal `identifier: &'static str` is borrowed straight from `std::env::consts::OS`.

## Why

Spreading `if cfg!(target_os = "macos")` throughout the codebase makes adding new targets a search-and-replace job. Concentrating that logic here means a new platform target adds one match arm in `osinfo` and propagates everywhere.

Predicates over string matches also kill the typo-class of bugs: `Platform::current().is_dawin()` would not compile, whereas `cfg!(target_os = "dawin")` silently compiles and never matches.

## Examples

```rust
use onyx::osinfo::{Platform, executable_extension};

let p = Platform::current();
println!("{} (exe ext: {:?})", p.as_str(), executable_extension());

if p.is_darwin() {
    use_macos_keychain();
} else if p.is_linux() {
    use_secret_service();
} else if p.is_windows() {
    use_credential_manager();
}
```

Use `Platform::current()` once and pass it around when you need many predicates in a tight loop — `current()` is cheap (one field read) but explicit threading is clearer:

```rust
fn pick_backend(p: Platform) -> &'static str {
    if p.is_darwin()  { "security" }
    else if p.is_linux()   { "secret-tool" }
    else if p.is_windows() { "cmdkey" }
    else { "unsupported" }
}
```

## executable_extension

Helper for building binary file names that work across platforms:

```rust
let target = format!("hyperion{}", onyx::osinfo::executable_extension());
// "hyperion" on macOS/Linux, "hyperion.exe" on Windows
```

Use this when constructing paths to bundled binaries — e.g. embedding a sidecar via `include_bytes!` and writing it to disk.

## Behaviour

- `Platform::current()` always succeeds — `std::env::consts::OS` is a compile-time constant; there is no runtime call.
- Predicates return `false` on any unsupported OS rather than panicking. The `as_str()` value exposes whatever `std::env::consts::OS` reports (e.g. `"freebsd"`, `"haiku"`), so consumers can branch:
  ```rust
  if !p.is_darwin() && !p.is_linux() && !p.is_windows() {
      return Err(MyError::UnsupportedOs(p.as_str()));
  }
  ```

## Errors

`osinfo` has no error type. Every function returns a plain value.

## Dependencies

None (uses `std::env::consts::OS` only).

## Related modules

Every other module depends on `osinfo`. When adding a new module, ask `Platform::current()` instead of touching `cfg!(target_os = ...)` outside the test gates.

## Cross-crate parity

Mirrors the Go crate's `osinfo` package: same predicate names (`is_darwin` ↔ `IsDarwin`), same `executable_extension` helper, same one-source-of-truth principle.

---

Navigation: [← Notify](23-notify.md) · **OS info** · [Paths →](25-paths.md)
