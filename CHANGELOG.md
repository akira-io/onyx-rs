# Changelog

All notable changes to `onyx` (Rust crate) are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0] - 2026-05-25

### Added

- Add device_name for a human-friendly machine name.

## [1.1.0] - 2026-05-25

### Added

- Add hostname for best-effort OS host name.
- Add login_path and enriched_environ for login-shell PATH recovery.
- Add get_or_create for a stable per-application machine identifier.
- Add is_dark for OS color scheme detection.
- Add relaunch to start a fresh application instance.

### Documentation

- Restructure docs/ to tens-block numbering.
- List new modules in index and changelog.

## [0.1.0] - 2026-05-19

### Added

- Onyx-rs v0.1.0 — Rust port of onyx.
- Add clipboard module.
- Add notify module.
- Add keyring module.

### Changed

- Drop fallback, switch publish to trusted publishing.

### Documentation

- Explain Resolver simplification and source removal.
- Rewrite for adoption and add docs/ tree.
- Add Prior art section.
- Remove empty Unreleased section before re-tag.

### Fixed

- Use Windows.Forms clipboard on Windows.


