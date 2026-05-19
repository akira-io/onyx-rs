# Architecture

## Module layout

```
src/
├── lib.rs              re-exports every public module
├── clipboard/mod.rs    pbcopy/pbpaste, wl-copy/xclip/xsel, PowerShell
├── files/mod.rs        open, xdg-open, cmd start
├── keyring/mod.rs      security, secret-tool, cmdkey + CredentialManager
├── notify/mod.rs       osascript, notify-send, BurntToast / msg
├── osinfo/mod.rs       Platform + executable_extension
├── paths/mod.rs        AppPaths { config, data, cache, logs }
└── shell/mod.rs        Resolver + bin-dir lookup helpers
```

For small modules a flat `src/<module>.rs` is acceptable; promote to `<module>/mod.rs` when the file exceeds ~200 lines or grows internal submodules. No `util.rs`, `helpers.rs`, or `common.rs` — every helper belongs to a domain module.

## Dependency direction

```
   osinfo      ◄────── single source of truth for OS branching
    ▲
    │
   ┌┴──────────────────────────────────────┐
   │                                       │
   clipboard  files  keyring  notify       paths   shell
```

`osinfo::Platform::current()` is the only path that touches `std::env::consts::OS`. Every other module asks `osinfo` instead of switching directly. When you add a new platform target, the change set converges on `osinfo` first.

`paths` and `shell` also lean on `std::env::var_os` for environment lookups (`HOME`, `XDG_*`, `APPDATA`, `LOCALAPPDATA`, `PATH`). These are scoped per-module — no helper layer in between.

## Trust model

`onyx` runs in the same process as the application. It does not sandbox itself, does not validate inputs beyond shape (empty path, empty service, etc.), and does not redact secrets. Wrap secrets with your own zeroizing crate at the call site if you need stronger guarantees — `keyring::get` returns a `String`.

The crate is library code: no global mutex, no spawned background threads, no environment mutation. It is safe to call from any async runtime (the calls block — wrap in `spawn_blocking` for hot paths).

## Backend selection

Modules that shell out follow the same pattern:

1. Ask `Platform::current()`.
2. Branch into a per-OS implementation.
3. Linux paths fall through a priority-ordered backend list (`wl-clipboard` → `xclip` → `xsel`, BurntToast → `msg`, etc.). First successful one wins; the `Unavailable` variant is reached only when every option fails.

The backend list lives inline in the module that owns it — no global registry, no plugin mechanism. Adding a new backend means editing one function and one variant comment.

## Error model

One typed error enum per module, derived via `thiserror::Error`. Conventions:

- Variants are `PascalCase`.
- `#[error("...")]` strings are prefixed with the module name (`"clipboard: …"`, `"keyring: …"`).
- Input validation, not-found, unavailable backend, and wrapped IO errors are separate variants.
- Backends report `Backend { action, backend, source: io::Error }` so call sites can log which binary failed.

```rust
pub enum ClipboardError {
    Unavailable,
    Backend { action: &'static str, backend: &'static str, source: io::Error },
}
```

Callers branch on the variant, not the `Display` string.

## Async + concurrency

All public functions are synchronous. The shell-out calls are blocking by design — desktop primitives are not hot-path code. Wrap with `tokio::task::spawn_blocking` (or your runtime's equivalent) inside a long-running task if needed.

Modules are stateless and `Send + Sync`. `Platform` is `Copy`. `Resolver` is `Clone + Default`. No global mutex, no per-module init.

## SOLID / DRY / KISS

- **Single responsibility** — one module, one concern.
- **Open/closed** — extend by adding a new module (`mod foo`) and a new error type; do not edit unrelated modules.
- **Liskov** — `Platform` is one shape; `Resolver` is one shape. No trait hierarchies.
- **Interface segregation** — the crate exports free functions where the contract is small (`clipboard::read`), and typed values where state is required (`Resolver`, `AppPaths`).
- **Dependency inversion** — modules depend on the `osinfo` contract, not on `std::env::consts::OS`.
- **DRY** — every OS switch lives in `osinfo` or the module that owns the backend list.
- **KISS** — the simplest correct API wins. Macros, unsafe, and async are last resorts.

## Cross-crate parity

The Go crate at [`akira-io/onyx`](https://github.com/akira-io/onyx) ships the same module names with idiomatic Go shapes. Behavioural parity is the goal — same backends, same priority order, same error categories. The two crates diverge only where the language idiom requires (`Result` vs sentinel error, `&str` vs `string`, etc.).

---

Navigation: [← Quickstart](02-quickstart.md) · **Architecture** · [Conventions →](04-conventions.md)
