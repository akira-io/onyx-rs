# onyx

Cross-platform Rust toolkit for building desktop applications without rewriting OS-specific glue every time. Sister crate to [`akira-io/onyx`](https://github.com/akira-io/onyx) (Go), with a mirrored module surface so a team can split a desktop app between a Rust core and Go services and reach for the same primitives on both sides.

## Why

```rust
// Without onyx
if cfg!(target_os = "macos") {
    return Some(home.join("Library").join("Application Support").join(app));
}
if cfg!(target_os = "linux") {
    if let Some(xdg) = env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(xdg).join(app));
    }
    return Some(home.join(".config").join(app));
}
if cfg!(target_os = "windows") {
    if let Some(v) = env::var_os("APPDATA") {
        return Some(PathBuf::from(v).join(app));
    }
    return Some(home.join("AppData").join("Roaming").join(app));
}
```

```rust
// With onyx
let config = paths::for_app("Hyperion").config()?;
```

Same behaviour on macOS, Linux, and Windows. No `cfg!(target_os = ...)` switches, no hand-rolled XDG logic, no per-OS shell invocations leaked into application code.

## Install

```toml
[dependencies]
onyx-rs = "0.1"
```

Then in your code:

```rust
use onyx::{files, paths, shell};
```

Requires Rust 1.75 or later.

## Quickstart

```rust
use onyx::{files, paths, shell};

fn main() -> anyhow::Result<()> {
    let app = paths::for_app("Hyperion");
    let config = app.config()?;
    files::reveal_in_file_manager(&config)?;

    let claude = shell::Resolver::new()
        .lookup("claude")
        .lookup("/opt/homebrew/bin/claude")
        .resolve()?;
    println!("claude at: {}", claude.display());
    Ok(())
}
```

## Modules

| Module | Purpose |
| --- | --- |
| [paths](./docs/modules/paths.md) | Configuration, data, cache, and log directories per platform. |
| [files](./docs/modules/files.md) | Open paths and URLs, reveal in file manager. |
| [shell](./docs/modules/shell.md) | Resolve CLI binaries via PATH lookup with explicit fallback paths. |
| [clipboard](./docs/modules/clipboard.md) | Read and write the system clipboard as plain text. |
| [notify](./docs/modules/notify.md) | Show desktop notifications. |
| [keyring](./docs/modules/keyring.md) | Store, retrieve, and delete secrets in the system credential store. |
| [osinfo](./docs/modules/osinfo.md) | Typed runtime platform detection. |

## Documentation

The full reference lives in [`docs/`](./docs/):

- [docs/00-overview.md](./docs/00-overview.md) — what onyx is and is not.
- [docs/01-conventions.md](./docs/01-conventions.md) — naming, function design, documentation rules every module follows.
- [docs/02-architecture.md](./docs/02-architecture.md) — module layout and the SOLID/DRY/KISS principles that drive it.
- [docs/modules/](./docs/modules/) — per-module reference.

## Platforms

| OS | Status |
| --- | --- |
| macOS | Fully tested by the maintainer. Primary development target. |
| Linux | Compiled and unit-tested in CI. Not exercised against real desktop environments by the maintainer. |
| Windows | Compiled and unit-tested in CI. Not exercised against real desktop environments by the maintainer. |

CI runs on all three platforms, but a green pipeline only proves the code compiles and the platform-agnostic tests pass. The Linux and Windows backends (notification daemons, clipboard helpers, credential managers, file managers) are not verified end-to-end against live systems by the maintainer. Report issues with reproduction steps and we will work through them.

## Testing

```sh
cargo test --all
```

Run `cargo fmt --all` and `cargo clippy --all-targets -- -D warnings` before opening a PR. The full suite runs on macOS, Linux, and Windows in CI. Tests that exercise OS facilities (notifications, clipboard, keychain) bail out cleanly when no backend is reachable, so the suite stays green even on minimal CI images.

## Contributing

Pull requests welcome. Before opening one:

1. Read [docs/01-conventions.md](./docs/01-conventions.md). PRs that break the conventions get rejected without further review.
2. Read [docs/02-architecture.md](./docs/02-architecture.md) for the rules that govern where new code goes.
3. Add tests for every public change. Touch [CHANGELOG.md](./CHANGELOG.md) under `## [Unreleased]`.
4. Use conventional commits (`feat:`, `fix:`, `refactor:`, `docs:`, `chore:`). The changelog workflow groups bullets by prefix.

For Go consumers, see the sister module [`akira-io/onyx`](https://github.com/akira-io/onyx).

## Prior art

`onyx` is a study project. These crates solve overlapping problems and have more battle-test. If you want a dependency you can lean on, reach for them first:

- Paths: [`dirs`](https://crates.io/crates/dirs), [`directories`](https://crates.io/crates/directories).
- Clipboard: [`arboard`](https://crates.io/crates/arboard).
- Notifications: [`notify-rust`](https://crates.io/crates/notify-rust).
- Keyring: [`keyring`](https://crates.io/crates/keyring).
- Binary resolution: [`which`](https://crates.io/crates/which).

## License

MIT. See [LICENSE](./LICENSE).
