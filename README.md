# Expert System

A Rust 2024 expert system in development. Currently, the program reads a sample
file and tokenizes it. The parser and inference engine are planned for later.

## Getting started

Use a Rust toolchain that supports edition 2024. Run these commands from the
repository root:

```sh
cargo check  # Check that the code compiles
cargo run   # Read examples/input.txt and print tokens or a lexer error
cargo test  # Run all tests
```

The sample input demonstrates the language; it is not a passing test fixture.
The lexer is still under development and may report an error for this file.

## Project layout

- `src/main.rs`: executable entry point and output handling.
- `src/lib.rs`: module declarations and public exports.
- `src/lexer/mod.rs`: lexer, token types, and lexer errors.
- `src/lexer/tests.rs`: lexer unit tests with beginner-friendly comments.
- `src/span.rs`: source-position spans.
- `examples/input.txt`: sample rules, facts, and queries.

`mod.rs` is the entry point for a Rust module stored in a directory.
Future parser and engine modules can follow the same layout as the lexer.

## Adding tests

Copy one of the examples in `src/lexer/tests.rs`, give it a descriptive name,
and change the input and expected result. Run only lexer tests with:

```sh
cargo test lexer
```

Keep component unit tests beside their code. Once the parser and engine exist,
use a root `tests/` directory for tests that exercise the complete pipeline.
