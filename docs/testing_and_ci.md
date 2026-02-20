# Testing and CI

## Automated Tests

File: `crates/dusttest/tests/harness.rs`

### `seeds_generation`

Validates deterministic sequence generation:

- input: `base=100`, `count=5`
- expected seeds: `[100, 101, 102, 103, 104]`

### `run_tests_with_dummy_script`

Creates temporary shell script that passes only for seed `1` and verifies:

- successes count,
- total run count,
- failing seed tracking.

## CI Pipeline

File: `.github/workflows/ci.yml`

CI steps:

1. checkout repository,
2. install Rust stable,
3. `cargo build --workspace --verbose`,
4. `cargo test --workspace --verbose`.

## Local Verification

From `dusttest` workspace root:

```bash
cargo build --workspace --verbose
cargo test --workspace --verbose
```
