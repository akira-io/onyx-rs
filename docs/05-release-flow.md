# Release Flow

Releases are automated. Two pipelines drive everything from conventional commits and `cliff.toml`.

## 1. CHANGELOG refresh (every push to `main`)

The `changelog` workflow runs on every push to `main`. It uses [`git-cliff`](https://git-cliff.org) to regenerate the `## [Unreleased]` block at the top of `CHANGELOG.md` from conventional commit messages since the last `vX.Y.Z` tag.

If the regenerated block differs from what is already in the file, the workflow opens a pull request titled `chore(changelog): refresh unreleased section`. A human reviewer reads the bullets, tightens them so they sound natural rather than mechanical, and merges.

That makes `## [Unreleased]` the **human-voice changelog**: a curated, readable summary of what is coming in the next release. The bot writes a draft; a human ships the final wording.

## 2. Release (every `vX.Y.Z` tag push)

The `release` workflow runs when a tag matching `v[0-9]+.[0-9]+.[0-9]+` (or a prerelease suffix like `-rc.1`) is pushed. It:

1. Runs `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --all`.
2. Generates **technical release notes** from `git-cliff` for the commit range bounded by the new tag. These notes are exhaustive and faithful to the commit log — they are not the same as the human-voice CHANGELOG.
3. Builds a source archive (`onyx-rs-vX.Y.Z.tar.gz`).
4. Publishes a GitHub Release at the tag with the technical notes as the body and the artifacts attached.
5. Publishes to crates.io via `cargo publish` (when `CARGO_REGISTRY_TOKEN` is set on the runner).

## How the two voices coexist

| Surface | Voice | Audience |
|---------|-------|----------|
| `CHANGELOG.md` | Human, curated, succinct | Library users browsing history |
| GitHub Release body | Technical, exhaustive | Downstream maintainers diffing versions |
| Git tags + commits | Conventional commits | Tooling and bots |
| `docs.rs` | API reference | Anyone integrating the crate |

The CHANGELOG is the polished story; the GitHub Release is the audit log; `docs.rs` is the rustdoc surface.

## Cutting a release

1. Make sure `## [Unreleased]` reads cleanly. Edit prose if the bot's first draft is awkward.
2. Bump `Cargo.toml`'s `version = "X.Y.Z"`.
3. Bump the unreleased header — change `## [Unreleased]` to `## [X.Y.Z] - YYYY-MM-DD`, and start a fresh empty `## [Unreleased]` above it.
4. Commit the change with `chore(release): vX.Y.Z`.
5. Tag and push:
   ```
   git tag vX.Y.Z
   git push origin main vX.Y.Z
   ```
6. The release workflow publishes the GitHub Release and (when configured) `cargo publish`es automatically.

## SemVer

Pre-1.0:

- `0.X.Y` bumps `X` for **any** breaking change (function signature, type rename, error variant rename, behaviour change).
- `0.X.Y` bumps `Y` for new features and bug fixes.

Post-1.0 (planned):

- `X.Y.Z` follows standard SemVer — `X` for breaking, `Y` for additive, `Z` for fixes.

Feature parity with the Go crate is tracked module-by-module; behavioural divergence between the two is treated as a bug in either crate.

## Conventional commit map

`cliff.toml` groups commits into sections by prefix:

| Prefix | CHANGELOG section |
|--------|-------------------|
| `feat`, `add` | Added |
| `fix` | Fixed |
| `refactor` | Changed |
| `perf` | Improved |
| `revert` | Reverted |
| `docs` | Documentation |
| `chore(deps)` | Dependencies |
| `style`, `test`, `chore` (other), `ci` | hidden |

Anything not matching a parser rule is dropped. Use the right prefix or your change will not appear in the changelog.

---

Navigation: [← Conventions](04-conventions.md) · **Release flow** · [Clipboard →](20-clipboard.md)
