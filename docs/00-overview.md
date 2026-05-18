# onyx — Overview

`onyx` is the Akira Foundation's Rust toolkit for building cross-platform desktop applications without writing the same OS-specific glue twice. Sister crate to [`akira-io/onyx`](https://github.com/akira-io/onyx) (Go), mirroring the same module surface.

The crate packages thin, opinionated wrappers around the stable Rust standard library (and direct OS calls when no safe abstraction exists), behind a single, consistent, intention-revealing API.

## Why

Every desktop application repeats the same primitives:

- Finding the user's configuration / data / cache directory.
- Opening a file with the user's default application.
- Revealing a file inside the platform's file manager.
- Resolving the absolute path of a CLI binary the user has installed.
- Copying text to the system clipboard.
- Sending a system notification.
- Reading and writing secrets in the keychain.

Across macOS, Linux, and Windows each of these has subtle differences. `onyx` makes those differences disappear behind a predictable API.

## What it is

- **Cross-platform** — single import works on macOS, Linux, Windows.
- **Idiomatic Rust** — `Result`, typed errors via `thiserror`, `PathBuf`/`&Path` for paths, no `unwrap` in library code.
- **Low-dependency** — only `thiserror` at runtime. No `cfg!(target_os)` leaks past the `osinfo` module.
- **Mirror of the Go crate** — same module names, same surface, idiomatic in each language.
- **Open source** — MIT-licensed, lives at `github.com/akira-io/onyx-rs`.

## What it is not

- It is not a UI framework. It does not draw windows, menus, or render content.
- It is not a desktop runtime. Use it from inside Tauri, egui, iced, dioxus, or a CLI.
- It is not a kitchen sink. Modules are small, focused, and independently usable.

## Reading guide

- [01-conventions.md](./01-conventions.md) — naming, function design, and documentation rules every module follows.
- [02-architecture.md](./02-architecture.md) — module layout and the SOLID/DRY/KISS principles that drive it.
- [modules/](./modules) — per-module reference.

## Status

Pre-1.0. API stable within a minor version, breaking changes only on `0.X` bumps. Feature parity with `onyx` (Go) at `v1.0.2` and tracking forward.
