# CLI Reference

Source: `crates/dusttest/src/main.rs`

## Usage

```text
dusttest [OPTIONS]
```

## Options

- `--config <FILE>`
  - Path to configuration file.
  - Default: `dusttest.toml`.

- `--seed <INT>`
  - Sets deterministic base seed.
  - Must parse as `u64`.

- `--ensemble <N>`
  - Number of seed runs per test.
  - Must parse as `usize`.
  - Default: `1`.

- `--list`
  - Lists tests and commands from config without executing.

- `--verbose`
  - Prints per-test and per-seed captured output.

- `-h`, `--help`
  - Prints help text and exits.

## Behavior Notes

- Unknown arguments are ignored by the current parser loop unless they match known options.
- Missing argument after `--config`, `--seed`, or `--ensemble` prints an error and exits.
- Invalid integer values for `--seed` or `--ensemble` print an error and exit.

## Exit Semantics

Current CLI returns from `main` on error paths without explicit non-zero process exit codes.

Operationally:

- Success paths print summary and return.
- Error paths print to stderr and return.
