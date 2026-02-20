# Seed Model

Source: `generate_seeds` in `crates/dusttest/src/lib.rs`

## Function Signature

```rust
pub fn generate_seeds(base_seed: Option<u64>, count: usize) -> (u64, Vec<u64>)
```

## Rules

1. If `count == 0`:
   - returns `(base_seed.unwrap_or(0), vec![])`.

2. If `base_seed` is provided:
   - base is exactly that value.
   - seed list is `base + i` for `i in [0, count)` using wrapping addition.

3. If `base_seed` is omitted:
   - base is generated with `StdRng::from_entropy().gen::<u64>()`.
   - seed list is still derived as `base + i`.

## Determinism

Given a fixed base seed and count, generated sequence is deterministic.

Example:

- base `100`, count `5` -> `[100, 101, 102, 103, 104]`
