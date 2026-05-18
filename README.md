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

let claude = shell::Candidates::new()
    .with_name("claude")
    .with_candidate("/opt/homebrew/bin/claude")
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

## Status

`v0.1.0` — feature parity with `onyx` (Go) at `v1.0.0`. API stable within a minor version pre-1.0.

## Installation

```toml
[dependencies]
onyx = "0.1"
```

## Platforms

macOS, Linux, Windows. Builds and tests run on all three in CI.

## License

MIT.
