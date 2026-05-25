# Changelog

All notable changes to `onyx` (Rust crate) are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.0] - 2026-05-25

### Added

- `osinfo::device_name()` returns a human-friendly machine name (macOS `ComputerName`, Windows `COMPUTERNAME`, Linux pretty hostname), falling back to `hostname()` when no friendlier source is available.

## [1.1.0] - 2026-05-25

### Added

- `osinfo::hostname()` returns the operating system host name as `Option<String>`, or `None` when it cannot be determined. Backed by the `gethostname` crate since the standard library exposes no portable host-name API.
- `machineid` module. `get_or_create(application)` returns a stable per-application identifier for the current machine, persisted in the system keyring. Error enum `MachineIdError`.
- `appearance` module. `is_dark()` reports whether the OS uses a dark color scheme via `defaults` (macOS), the registry (Windows), and `gsettings` (Linux). Best-effort: returns `false` when undetermined.
- `shell::login_path()` and `shell::enriched_environ()`. Recover the PATH from the user's login shell so GUI applications can locate user-installed tools.
- `process` module. `relaunch(application_path)` starts a fresh instance of the application (`open -n` on macOS, `start` on Windows, exec on Linux). Error enum `ProcessError`.

## [0.1.0] - 2026-05-18

### Added

- Initial Rust port of the `onyx` toolkit, sister crate to `github.com/akira-io/onyx` (Go).
- `osinfo` module with `Platform`, `Platform::current`, and `executable_extension`.
- `paths` module with `for_app(name)` and `config`, `data`, `cache`, `logs` resolvers for macOS, Linux, and Windows.
- `files` module with `open_path`, `open_url`, and `reveal_in_file_manager`.
- `shell` module with `Resolver` builder, `lookup`/`lookups` accepting names or paths (auto-detected), `resolve` returning `PathBuf`, and `list_*` helpers for npm/user/system/Windows-application install directories.
- MIT license.
