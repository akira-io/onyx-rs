# Installation

## Add to Cargo.toml

```toml
[dependencies]
onyx-rs = "0.2"
```

The crate name on crates.io is `onyx-rs`. The library name is `onyx`:

```rust
use onyx::{clipboard, files, keyring, notify, paths, shell, osinfo};
```

## MSRV

`rust-version = "1.75"`. The crate compiles on stable.

## Features

```toml
[features]
default = []
```

No optional features at this stage. Every module is in the default build. The runtime dependency surface is just `thiserror` — every backend call goes through `std::process::Command` (no FFI, no transitive C deps).

## OS support

| Module | macOS | Linux | Windows |
|--------|-------|-------|---------|
| `clipboard` | pbcopy / pbpaste | wl-clipboard / xclip / xsel | PowerShell + WinForms |
| `files` | open | xdg-open | cmd `start` |
| `keyring` | `security` | `secret-tool` | `cmdkey` + PowerShell + `CredentialManager` |
| `notify` | osascript | notify-send | BurntToast / `msg` |
| `osinfo` | always | always | always |
| `paths` | `~/Library/...` | XDG Base Directory | Known Folders |
| `shell` | `PATH` lookup | `PATH` lookup | `PATH` lookup + Program Files heuristics |

Modules that shell out report a typed error variant when no backend is reachable (`ClipboardError::Unavailable`, `KeyringError::Unavailable`, `NotifyError::Unavailable`) — callers can branch and surface install instructions.

## Workspace consumption

```toml
[workspace.dependencies]
onyx-rs = { version = "0.2" }

[dependencies]
onyx-rs = { workspace = true }
```

## Verify

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

The unit suite skips cleanly when a backend is missing (returns early on `Unavailable`), so the suite stays green on minimal CI runners.

---

Navigation: [← Index](00-index.md) · **Installation** · [Quickstart →](02-quickstart.md)
