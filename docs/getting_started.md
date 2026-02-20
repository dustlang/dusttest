# Getting Started

## Prerequisites

- Rust stable toolchain
- Cargo
- A POSIX shell named `sh` available on `PATH` (required by current executor)

## Build

From the `dusttest` workspace root:

```bash
cargo build --workspace
```

## Run Tests

```bash
cargo test --workspace --verbose
```

## Run the Harness

From `dusttest/crates/dusttest` (or provide `--config` path):

```bash
cargo run -- --config dusttest.toml
```

Run with a fixed seed:

```bash
cargo run -- --config dusttest.toml --seed 42
```

Run an ensemble of 10 seeds:

```bash
cargo run -- --config dusttest.toml --seed 1234 --ensemble 10
```

List tests only:

```bash
cargo run -- --config dusttest.toml --list
```

Verbose output:

```bash
cargo run -- --config dusttest.toml --verbose
```
