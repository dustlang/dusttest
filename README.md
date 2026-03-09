# dusttest

`dusttest` is now expressed in the currently supported top-level Dust grammar profile:

- single executable unit: `src/main.ds`
- entrypoint form: `K main { ... }`

This profile is designed to build with the current `dust` compiler parser without forge/proc namespace syntax.

## Build

From `dust/` compiler workspace:

```bash
CARGO_HOME=/tmp/cargo-home cargo run -p dust -- build ../dusttest/src --out ../dusttest/target/dust/dusttest
```

## Notes

- The previous Rust crate layout (`Cargo.toml` + `crates/dusttest`) was retired for Dust-native migration.
- `dusttest.toml` remains as an example harness configuration artifact for future runtime restoration.
