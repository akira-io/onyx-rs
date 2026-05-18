# onyx

Cross-platform Rust toolkit for building desktop applications without rewriting OS-specific glue every time.

Sister crate to [`github.com/akira-io/onyx`](https://github.com/akira-io/onyx) (Go). Mirrors the same module surface so a team can split a desktop app between a Rust core and Go services and reach for the same primitives on both sides.

### Without onyx

```rust
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn app_config_dir(app: &str) -> Option<PathBuf> {
    let home = env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" })?;
    let home = PathBuf::from(home);
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
    None
}

fn reveal_in_file_manager(path: &str) -> std::io::Result<()> {
    if cfg!(target_os = "macos") {
        return Command::new("open").arg("-R").arg(path).spawn().map(|_| ());
    }
    if cfg!(target_os = "linux") {
        return Command::new("xdg-open").arg(path).spawn().map(|_| ());
    }
    if cfg!(target_os = "windows") {
        return Command::new("explorer")
            .arg(format!("/select,{}", path))
            .spawn()
            .map(|_| ());
    }
    Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "platform"))
}
```

### With onyx

```rust
use onyx::{files, paths, shell};

let app = paths::for_app("Hyperion");
let config = app.config()?;
files::reveal_in_file_manager(&config)?;

let claude = shell::Resolver::new()
    .lookup("claude")
    .lookup("/opt/homebrew/bin/claude")
    .resolve()?;
```

Same behavior on macOS, Linux, and Windows. No `cfg!(target_os = ...)` switches, no hand-rolled XDG logic, no per-OS shell invocations leaked into application code.

## Modules

| Module | Purpose |
| --- | --- |
| `osinfo` | Platform detection (`Platform::current`, `is_darwin`, `is_linux`, `is_windows`, `executable_extension`). |
| `paths` | Per-app `config`, `data`, `cache`, `logs` directories with XDG / Library / AppData resolution. |
| `files` | Open paths/URLs with the default application, reveal a path in the file manager. |
| `shell` | Resolve CLI binaries via PATH first, then well-known per-platform install directories. |
| `clipboard` | Read and write the system clipboard as plain text. |
| `notify` | Show desktop notifications. |

## Status

`v0.1.0`. Feature parity with `onyx` (Go) at `v1.0.2`. API stable within a minor version pre-1.0.

## Design notes

### `shell::Resolver`: one verb, two cases

The Resolver started with two separate concepts: a name list for `PATH` lookups and a fallback path list for explicit files to try when `PATH` missed. Resolution exposed a `ResolutionSource` enum (`Path`, `Fallback`, `Unknown`) so callers could see how the binary was found.

The split asked callers to classify each input upfront. The classification is mechanical: if the string has a path separator (`/`, `\\`) or a Windows drive prefix (`C:`), it is a path; otherwise it is a name. The source tag was rarely inspected.

`Resolver` now collapses everything to a single ordered list of targets. `lookup` accepts both. `resolve` returns the absolute `PathBuf` of the first target that resolves, or `ShellError::BinaryNotFound`. Callers that genuinely need to know how a binary was located inspect the returned path themselves.

## Publishing

This crate publishes to crates.io via [Trusted Publishing](https://crates.io/docs/trusted-publishing). The `publish.yml` workflow exchanges a GitHub OIDC token for a short-lived crates.io token at publish time. No long-lived API tokens are stored in repo secrets.

### Bootstrap (one-time, for the very first release)

Trusted Publishing requires the crate to exist on crates.io before it can be configured. To bootstrap:

1. Generate a one-time API token at <https://crates.io/me> with `publish-new` scope.
2. Run locally:
   ```sh
   cargo publish --token <one-time-token>
   ```
   This claims the `onyx-rs` crate name and ships v0.1.0.
3. On <https://crates.io/crates/onyx-rs/settings>, open **Trusted Publishing** and click **Add**:
   - Publisher: GitHub
   - Repository: `akira-io/onyx-rs`
   - Workflow: `publish.yml`
   - Environment: `release`
4. Revoke the one-time API token. Future releases trigger the `publish.yml` workflow on every `vX.Y.Z` tag push and no token is needed.
5. In this repo's GitHub Settings, create an Environment named `release` (optionally with required reviewers as a pre-publish gate).

### Cutting a release

1. Bump `Cargo.toml` `version`.
2. Promote the `## [Unreleased]` block in `CHANGELOG.md` to a dated `## [X.Y.Z]` block.
3. Commit with `chore(release): vX.Y.Z`.
4. Tag and push:
   ```sh
   git tag vX.Y.Z
   git push origin main vX.Y.Z
   ```
5. `publish.yml` verifies the tag matches `Cargo.toml`, runs tests, authenticates with crates.io via OIDC, publishes, and creates a GitHub Release.

## Installation

```toml
[dependencies]
onyx-rs = "0.1"
```

Crate publishes as `onyx-rs` on crates.io (the `onyx` name was already taken). The library is still imported as `onyx`:

```rust
use onyx::{files, paths, shell};
```

## Platforms

macOS, Linux, Windows. Builds and tests run on all three in CI.

## License

MIT.
