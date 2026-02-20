# Library API Reference

Source: `crates/dusttest/src/lib.rs`

## Data Structures

### `TestSpec`

```rust
pub struct TestSpec {
    pub name: String,
    pub cmd: String,
}
```

Represents one configured test command.

### `Config`

```rust
pub struct Config {
    #[serde(rename = "test")]
    pub tests: Vec<TestSpec>,
}
```

Top-level deserialization target for TOML configuration.

### `TestResult`

```rust
pub struct TestResult {
    pub name: String,
    pub successes: usize,
    pub total_runs: usize,
    pub failing_seeds: Vec<u64>,
    pub outputs: Vec<(u64, String)>,
}
```

Aggregated result for one test across all seeds.

## Functions

### `load_config(path: &str) -> io::Result<Vec<TestSpec>>`

- Reads TOML from disk.
- Deserializes into `Config`.
- Returns `tests` list.

### `generate_seeds(base_seed: Option<u64>, count: usize) -> (u64, Vec<u64>)`

- Produces deterministic seed sequence.
- See `seed_model.md` for full contract.

### `run_tests(tests: &[TestSpec], seeds: &[u64], verbose: bool) -> Vec<TestResult>`

- Executes command suite under seed ensemble.
- Collects success/failure counts and failing seeds.

### `format_summary(results: &[TestResult], base_seed: u64) -> String`

- Converts result set into user-facing report text.
