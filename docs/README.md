# dusttest Documentation

This directory contains Markdown documentation for `dusttest`.

## Documentation Index

- `getting_started.md`: setup, build, and first run.
- `architecture.md`: workspace and module layout.
- `cli_reference.md`: command-line contract and option behavior.
- `config_reference.md`: `dusttest.toml` format and examples.
- `seed_model.md`: deterministic seed generation and ensemble behavior.
- `execution_model.md`: command execution flow and result collection.
- `library_api.md`: legacy API notes from pre-migration Rust implementation.
- `error_handling.md`: failure modes and reporting semantics.
- `platform_notes.md`: shell/runtime assumptions and portability notes.
- `testing_and_ci.md`: test strategy and CI workflow.

## Scope

`dusttest` now ships as a Dust-native top-level grammar profile (`src/main.ds`).
Historical docs that reference the retired Rust crate layout are retained for roadmap context.
