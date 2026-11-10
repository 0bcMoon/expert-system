# Repository Guidelines

## Project Structure & Module Organization

This repository is an early-stage Rust 2024 expert system with no external dependencies. `src/main.rs` runs the lexer against the root-level `input.txt` and prints tokens or an error. `src/lib.rs` declares modules and re-exports common types.

- `src/lexer.rs`: token definitions, file reading, tokenization, and lexer errors.
- `src/span.rs`: source-position spans.
- `src/parser.rs` and `src/error.rs`: currently empty placeholders for future implementation.
- `input.txt`: sample rules, initial facts, and queries; treat it as example input, not a passing test fixture.
- `target/`: generated Cargo output, ignored by Git.

## Build, Test, and Development Commands

Run commands from the repository root using a toolchain that supports Rust edition 2024:

- `cargo build`: compile the library and executable.
- `cargo check`: check compilation without producing an executable.
- `cargo run`: tokenize `input.txt`; the filename is currently hardcoded.
- `cargo test`: run unit, integration, and documentation tests.
- `cargo fmt --check`: check standard Rust formatting.
- `cargo clippy --all-targets`: inspect compiler and lint diagnostics.

## Coding Style & Naming Conventions

Use four-space indentation and standard `rustfmt` formatting. Name modules, functions, and variables in `snake_case`; types and enum variants use `PascalCase`. Keep lexer behavior in its module and expose shared public types through `src/lib.rs`. Preserve useful line and column information when reporting malformed input, and propagate I/O failures through `Result`.

## Testing Guidelines

No tests or coverage threshold are currently defined. Add unit tests in a module-local `#[cfg(test)] mod tests` block; place public API integration tests under `tests/`. Use descriptive names such as `rejects_unexpected_character`. Cover valid operators, malformed expressions, end-of-input handling, and source positions when changing tokenization. Run `cargo test` before submitting changes.

## Commit & Pull Request Guidelines

The repository has no commits yet, so no established commit convention exists. Use concise, imperative subjects such as `Add lexer operator tests`. Keep changes focused. Pull requests should describe the behavior changed, link relevant issues, and list validation commands and results. Include sample input and expected tokens or diagnostics for lexer or parser changes.
