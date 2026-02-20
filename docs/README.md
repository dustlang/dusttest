# dusttest Documentation

This directory contains complete Markdown documentation for `dusttest`.

## Documentation Index

- `getting_started.md`: setup, build, and first run.
- `architecture.md`: workspace and module layout.
- `cli_reference.md`: command-line contract and option behavior.
- `config_reference.md`: `dusttest.toml` format and examples.
- `seed_model.md`: deterministic seed generation and ensemble behavior.
- `execution_model.md`: command execution flow and result collection.
- `library_api.md`: public Rust API reference.
- `error_handling.md`: failure modes and reporting semantics.
- `platform_notes.md`: shell/runtime assumptions and portability notes.
- `testing_and_ci.md`: test suite and CI workflow.

## Scope

`dusttest` is a deterministic command harness. It reads test definitions from TOML, executes each test command across one or more seeds, and prints pass/fail summaries.

It does not compile Dust source directly and does not own test semantics. It runs external commands and interprets process exit codes.
