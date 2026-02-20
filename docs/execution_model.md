# Execution Model

Source: `run_tests` and `format_summary` in `crates/dusttest/src/lib.rs`

## Test Execution

For each test and each seed:

1. Start command as `sh -c <test.cmd>`.
2. Set environment variable `DUST_SEED=<seed>`.
3. Capture both stdout and stderr.
4. Mark success if exit status is success (`status.success()`).
5. Otherwise mark failure and record failing seed.

## Verbose Output Collection

When `verbose = true`:

- Combined output string is recorded per seed.
- stderr is appended after stdout when non-empty.
- Spawn errors are recorded as `error invoking command: <error>`.

When `verbose = false`:

- per-seed output bodies are not stored.

## Spawn Failure Behavior

If process spawn or execution fails:

- seed is treated as failure,
- failure is included in `failing_seeds`.

## Summary Formatting

`format_summary(results, base_seed)` prints:

- header with test count and base seed,
- one line per test:
  - `pass on X/Y seeds` when no failures,
  - `pass on X/Y seeds (failures on seeds: [...])` when failures occur,
  - `SKIPPED (no seeds)` when total runs is zero,
- final line:
  - `All tests passed!` if all non-skipped tests passed,
  - `Some tests failed.` otherwise.
