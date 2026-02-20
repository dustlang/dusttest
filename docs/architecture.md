# Architecture

## Workspace Layout

- `dusttest/Cargo.toml`: workspace manifest
- `crates/dusttest/Cargo.toml`: package manifest
- `crates/dusttest/src/lib.rs`: core harness library
- `crates/dusttest/src/main.rs`: CLI binary
- `crates/dusttest/tests/harness.rs`: integration-style library tests
- `.github/workflows/ci.yml`: CI pipeline

## Runtime Pipeline

1. Parse CLI flags.
2. Resolve config path (`dusttest.toml` by default).
3. Load config (`load_config`).
4. Handle `--list` fast path if requested.
5. Generate seeds (`generate_seeds`).
6. Execute tests (`run_tests`).
7. Format and print summary (`format_summary`).

## Responsibilities

- `main.rs`: CLI parsing, user-facing flow, printing.
- `lib.rs`: deterministic seed logic, config loading, command execution, summary generation.
