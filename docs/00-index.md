# onyx — Reference

Cross-platform desktop toolkit for Rust. Sister crate to [`akira-io/onyx`](https://github.com/akira-io/onyx) (Go) — same module surface, same conventions, idiomatic in each language.

The crate packages thin, opinionated wrappers around the stable Rust standard library (and direct OS calls when no safe abstraction exists), behind a single, consistent, intention-revealing API.

## Meta

| File | Topic |
|------|-------|
| [01-installation](01-installation.md) | Add the crate, MSRV, feature flags |
| [02-quickstart](02-quickstart.md) | 60-second snippet for each module |
| [03-architecture](03-architecture.md) | Module layout, SOLID/DRY/KISS principles |
| [04-conventions](04-conventions.md) | Naming, function design, comments, errors |
| [05-release-flow](05-release-flow.md) | Versioning, branching, tagging, publishing |

## Modules

| File | Topic |
|------|-------|
| [20-clipboard](20-clipboard.md) | Read/write the system clipboard as plain text |
| [21-files](21-files.md) | Open files / URLs, reveal in file manager |
| [22-keyring](22-keyring.md) | Store, retrieve, delete secrets in the OS credential store |
| [23-notify](23-notify.md) | Desktop notifications |
| [24-osinfo](24-osinfo.md) | Platform identity — single source of truth for OS branching |
| [25-paths](25-paths.md) | Per-application config / data / cache / log directories |
| [26-shell](26-shell.md) | Locate command-line executables |

## What this crate is

- **Cross-platform** — single import works on macOS, Linux, Windows.
- **Idiomatic Rust** — `Result`, typed errors via `thiserror`, `PathBuf`/`&Path` for paths, no `unwrap` in library code.
- **Low-dependency** — only `thiserror` at runtime. No `cfg!(target_os)` leaks past `osinfo`.
- **Mirror of the Go crate** — same module names, same surface, idiomatic in each language.

## What this crate is not

- It is not a UI framework. No window, menu, or rendering surface.
- It is not a desktop runtime. Use it from Tauri, egui, iced, dioxus, or a CLI.
- It is not a kitchen sink. Modules are small, focused, and independently usable.

## Status

Pre-1.0. API stable within a minor version; breaking changes only on `0.X` bumps. Feature parity with the Go crate `onyx` at `v1.0.2` and tracking forward.

---

Navigation: [README](../README.md) · **Index** · [Installation →](01-installation.md)
