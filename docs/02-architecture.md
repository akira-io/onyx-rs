# Architecture

## Module layout

```
src/
├── lib.rs              re-exports every public module
├── osinfo/             platform detection
├── paths/              per-app config/data/cache/log dirs
├── files/              open / reveal in file manager
├── shell/              resolve CLI binaries
├── clipboard/          system clipboard read/write
├── notify/             desktop notifications
└── keyring/            credential store
docs/
└── modules/<module>.md
```

Each module is self-contained: its own error enum, its own platform fan-out, its own tests. The only module any other module is allowed to depend on is `osinfo`.

## Dependency direction

```
clipboard ─┐
notify    ─┤
keyring   ─┼──► osinfo
files     ─┤
paths     ─┤
shell     ─┘
```

`osinfo` has no internal dependencies. Every other module imports `osinfo::Platform` for branching. No module imports any sibling. If a feature needs primitives from two modules, the consumer composes them; `onyx` itself does not.

This keeps the crate trivially composable. A consumer that needs only `paths` does not pull in clipboard or keyring code.

## Platform fan-out

- `runtime::OS` (Rust's `std::env::consts::OS` and `cfg!(target_os = "...")`) is referenced **only inside `osinfo`**.
- Every other module branches on `Platform::current()` returning a typed enum.
- New backends live next to the function that fans out, not in a separate `_unix.rs` / `_windows.rs` file (except where Rust's `#[cfg(target_os = ...)]` syntax forces it).

## SOLID applied

- **Single responsibility** — each module solves one problem. `clipboard` does not load files, `files` does not resolve binaries.
- **Open/closed** — to support a new platform backend, add a branch inside the module; do not edit existing branches.
- **Liskov** — every backend behind a module returns the same `Result` shape. A caller never needs to know which backend ran.
- **Interface segregation** — traits, if any, are tiny and behaviour-focused. Consumers depend on exactly the functions they use.
- **Dependency inversion** — modules depend on the contract exposed by `osinfo`, not on the OS directly.

## Error contract

Every module exposes its own error enum derived via `thiserror::Error`. Variants are package-prefixed in their `#[error("...")]` strings so a log line tells you which module failed:

```
clipboard: no supported backend available
shell: binary not found
keyring: secret not found
```

Wrapped backend errors include the action (`"get"`, `"write"`) and backend name (`"security"`, `"secret-tool"`, `"powershell"`) so a failure points straight at the OS call that failed.

## Testing strategy

- Pure logic (path resolution, validation, escaping) has direct unit tests.
- OS-facing functions have round-trip tests that exercise the real backend when available and skip cleanly when none is.
- No std mocking. No background processes left running. Tests that share a global resource (clipboard, keychain) serialize through a `Mutex` to avoid parallel-test races.

## Versioning

SemVer. Public API of any module is the contract; breaking it requires a major bump (`0.X` while pre-1.0). New modules and new variants on existing enums are additive — minor bump.

## Mirror with onyx (Go)

This crate tracks the Go [`onyx`](https://github.com/akira-io/onyx) module surface one-to-one. When a feature lands in one repo it is proposed for the other, with names translated idiomatically:

- `paths.For(app)` ↔ `paths::for_app(app)`
- `shell.NewResolver().Lookup(x).Resolve()` ↔ `shell::Resolver::new().lookup(x).resolve()`
- `ErrBinaryNotFound` ↔ `ShellError::BinaryNotFound`

Asymmetries (anything that uses Go channels or Rust async, for instance) are documented in both READMEs under a "Language differences" note rather than silently kept on one side.
