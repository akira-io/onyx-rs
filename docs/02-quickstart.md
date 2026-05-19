# Quickstart

One short snippet per module. Every call returns `Result<_, ModuleError>` — handle or `?`-propagate.

## Clipboard

```rust
use onyx::clipboard;

clipboard::write("onyx is here")?;
let echo = clipboard::read()?;
println!("{echo}");
# Ok::<(), onyx::clipboard::ClipboardError>(())
```

## Files

```rust
use onyx::files;

files::open_path("/Users/me/Downloads/report.pdf")?;
files::open_url("https://akira.foundation")?;
files::reveal_in_file_manager("/Users/me/Downloads/report.pdf")?;
# Ok::<(), onyx::files::FileError>(())
```

## Keyring

```rust
use onyx::keyring;

keyring::set("io.akira.app", "kid@example.com", "hunter2")?;
let secret = keyring::get("io.akira.app", "kid@example.com")?;
keyring::delete("io.akira.app", "kid@example.com")?;
# Ok::<(), onyx::keyring::KeyringError>(())
```

## Notify

```rust
use onyx::notify;

notify::show("Build complete", "Spectra v0.9.0 is ready to install.")?;
# Ok::<(), onyx::notify::NotifyError>(())
```

## OS info

```rust
use onyx::osinfo::{Platform, executable_extension};

let p = Platform::current();
println!("{} (exe ext: {})", p.as_str(), executable_extension());
if p.is_darwin() { /* ... */ }
```

## Paths

```rust
use onyx::paths;

let app = paths::for_app("hyperion");
let config = app.config()?;     // ~/Library/Application Support/hyperion on macOS
let data   = app.data()?;
let cache  = app.cache()?;
let logs   = app.logs()?;
# Ok::<(), onyx::paths::PathError>(())
```

## Shell

```rust
use onyx::shell::Resolver;

let claude = Resolver::new()
    .lookup("claude")
    .lookup("/opt/homebrew/bin/claude")
    .resolve()?;
println!("found at {}", claude.display());
# Ok::<(), onyx::shell::ShellError>(())
```

## Combining modules

```rust
use onyx::{files, paths};

let logs = paths::for_app("hyperion").logs()?;
std::fs::create_dir_all(&logs)?;
let path = logs.join("today.log");
std::fs::write(&path, "boot ok\n")?;
files::reveal_in_file_manager(&path)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

`osinfo::Platform` is the only place the crate inspects `std::env::consts::OS` — every other module asks `Platform::current()` instead of switching directly. Reuse the same primitive in your app.

## What to read next

- [03-architecture](03-architecture.md) — module layout and design principles
- [04-conventions](04-conventions.md) — naming, error, doc rules every module follows
- Module reference under [20-clipboard](20-clipboard.md) → [26-shell](26-shell.md)

---

Navigation: [← Installation](01-installation.md) · **Quickstart** · [Architecture →](03-architecture.md)
