# Configuration Reference

Source types: `TestSpec`, `Config` in `crates/dusttest/src/lib.rs`

## File Format

The config is TOML with repeated `[[test]]` tables.

```toml
[[test]]
name = "formatting"
cmd = "cargo run --quiet --bin dustfmt tests/fixtures/example1.dust"

[[test]]
name = "docs"
cmd = "cargo run --quiet --bin dustdoc --html tests/fixtures/example2.dust"
```

## Schema

### `[[test]]`

- `name` (string, required)
  - Display name in summary output.

- `cmd` (string, required)
  - Shell command executed via `sh -c`.

## Empty Test Set

If config loads but contains no tests, CLI prints:

`No tests defined in <config_file>`

and exits without running anything.

## List Mode

`--list` prints each test in this format:

`<name>: <cmd>`
