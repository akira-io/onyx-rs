# Conventions

Every module in `onyx` follows the rules below. They are non-negotiable. If a contribution breaks one of these rules without justification, it is rejected.

## Naming

### Modules

- Lowercase, single word, no underscores or camelCase.
- Singular when the module is about a single concept (`shell`).
- Plural when the module primarily yields collections or grouped resources (`paths`, `files`).
- Never prefix with `onyx` — the crate root already provides that context.

### Items

- **Be verbose. Be explicit. Never abbreviate.**
  - `resolve_executable`, never `rslv_exec`.
  - `reveal_in_file_manager`, never `show_file`.
- Functions are `snake_case`, start with a verb in the imperative mood.
  - `open_path`, `write_secret`, `lookup_path`.
- Types are `PascalCase`. `Resolver`, `Platform`, `ShellError`.
- Constructors return a configured value, named after the value or the domain.
  - `Resolver::new()`, `paths::for_app("hyperion")`.
- Predicate functions start with `is_`, `has_`, or `can_`.
  - `is_available`, `has_extension`, `can_write`.

### Errors

- One typed error enum per module, derived via `thiserror::Error`.
- Variants are `PascalCase`, package-prefixed in the `#[error("...")]` string (e.g. `"shell: binary not found"`).
- Distinguish input validation, not-found, unavailable backend, and wrapped backend errors as separate variants.
- Never `panic!` for recoverable failures. Never `unwrap()` / `expect()` in library code.

## Function design

- **Single responsibility.** A function does one thing the name describes.
- **`Result<T, ModuleError>` over `Option` for fallible operations.** Reserve `Option` for genuinely optional values.
- **No boolean flag parameters.** Branching on flags means two functions wearing one name. Split them.
- **No `Box<dyn Trait>` in public signatures unless dispatch is genuinely dynamic.** Prefer generics or concrete enums.
- **Builders for 3+ parameters.** A function that needs many options exposes a builder so call sites are readable.
- **`PathBuf` / `&Path` for paths.** Never raw `String`. `OsStr`/`OsString` at the OS boundary.

## Comments

- **Code is the documentation.** If the name does not explain the function, the name is wrong.
- **No inline `//` narration.** Comments explaining *what* the next line does are forbidden.
- **`///` doc comments on every public item** — one or two sentences describing intent, not implementation. `//!` at the top of each module describes the module.
- **`// TODO(@handle):` is allowed**, one line, with the author's GitHub handle.

## Documentation

- Every module has a markdown file in `docs/modules/<module>.md`.
- The file covers: purpose, public API, examples, error catalog, dependencies, related modules.
- `README.md` is the adoption hook — long-form text belongs in `docs/`.

## SOLID, DRY, KISS

- **Single responsibility** — one module = one concern. No "utils" grab bag.
- **Open/closed** — extend by adding new types or implementations, not by editing existing ones.
- **Liskov** — traits are tiny, behavior is consistent across implementations.
- **Interface segregation** — prefer many small traits. A consumer should depend only on what it uses.
- **Dependency inversion** — modules depend on contracts in `onyx`, not on transitive third-party crates.
- **DRY** — if the same OS switch appears twice, it lives in `osinfo` or a shared helper.
- **KISS** — the simplest correct API wins. Macros, unsafe, and async are last resorts.

## Testing

- Every public function has a test.
- Tests are `snake_case`, scenario-first: `resolve_fails_when_nothing_matches`.
- Unit tests live in `#[cfg(test)] mod tests { ... }` at the bottom of the module. Integration tests in `tests/`.
- No mocking std. Use `tempfile::tempdir()` and the real filesystem.
- OS-facing tests skip cleanly when no backend is reachable (return early on `Unavailable`, etc.). The suite stays green on minimal CI.

## Lint gate

Before opening a PR, all three pass:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
```

CI enforces the same gate on every push and pull request.

## Module layout

```
src/
├── lib.rs              re-exports every public module
└── <module>/
    └── mod.rs          public API + #[cfg(test)] mod tests
docs/
└── modules/<module>.md
```

For small modules a flat `src/<module>.rs` is acceptable; promote to `<module>/mod.rs` when the file exceeds ~200 lines or grows internal submodules. No `util.rs`, `helpers.rs`, or `common.rs`.
