# Changelog

All notable changes to `onyx` (Rust crate) are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-05-18

### Added

- Initial Rust port of the `onyx` toolkit, sister crate to `github.com/akira-io/onyx` (Go).
- `osinfo` module with `Platform`, `Platform::current`, and `executable_extension`.
- `paths` module with `for_app(name)` and `config`, `data`, `cache`, `logs` resolvers for macOS, Linux, and Windows.
- `files` module with `open_path`, `open_url`, and `reveal_in_file_manager`.
- `shell` module with `Resolver` builder, `lookup`/`lookups` accepting names or paths (auto-detected), `resolve` returning `PathBuf`, and `list_*` helpers for npm/user/system/Windows-application install directories.
- MIT license.
