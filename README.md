# Dusttest – deterministic test harness with seeded ensembles

`dusttest` is a deterministic test harness for the **Dust programming
language**.  It provides a command‑line runner that reads a test
configuration, executes a suite of programs or commands under a
controlled random seed, and reports pass/fail results.  The harness can
run each test multiple times with different seeds (a *seeded
ensemble*), ensuring reproducible behaviour and robust detection of
non‑deterministic bugs.

Determinism is a core goal.  Property‑based testing frameworks like
fast‑check emphasise that tests should launch with a precise seed
so that the same set of random values is generated on each run【157165867946773†L116-L120】.
Similarly, when dealing with concurrent code, reproducing thread
interleavings requires a deterministic scheduler that is driven by a
seed【548468680421956†L118-L129】.  `dusttest` uses seeds to control the random
values or scheduling of the programs it runs.  You can specify a
single seed via `--seed` or ask for an ensemble of seeds via
`--ensemble`, producing multiple independent runs to capture rare
failures.

## Features

* **Deterministic seeds** – by default the harness selects a random
  seed; use `--seed N` to control it.  Seeds are passed to tests via
  environment variables (see below).
* **Seeded ensembles** – specify `--ensemble <n>` to run each test
  multiple times with distinct seeds.  The seeds are derived from the
  base seed and the index of the run, ensuring reproducible
  sequences across machines and over time.
* **Test configuration** – define tests in a `dusttest.toml` file at
  the root of your project.  Each test consists of a name and a shell
  command.  Commands are executed with a `DUST_SEED` environment
  variable set to the current seed.  The harness interprets exit code
  0 as success and anything else as failure.
* **Summary reporting** – after all runs, `dusttest` prints a summary
  showing how many seeds passed for each test and lists any failing
  seeds.  Use `--verbose` for per‑run logs.
* **Extensible** – the harness can be extended to spawn the Dust
  compiler on `.dust` files directly, or to integrate deterministic
  schedulers for concurrent DPL code.  The foundation is designed to
  be language‑agnostic: it simply runs commands under different
  seeds.

## Usage

Place a `dusttest.toml` file in your project directory defining your
tests.  For example:

```toml
[[test]]
name = "formatting"
cmd = "cargo run --quiet --bin dustfmt tests/fixtures/example1.dust"

[[test]]
name = "docs"
cmd = "cargo run --quiet --bin dustdoc --html tests/fixtures/example2.dust"
```

Each `cmd` is run in a shell with the environment variable `DUST_SEED`
set to the chosen seed.  Your programs should read this variable to
seed their random number generators or schedulers.  When using
`--ensemble`, the harness will execute each test multiple times and
record any failing seeds.

Run the harness from the project root:

```
$ dusttest [OPTIONS]

Options:
    --config <FILE>    Path to a custom configuration file (defaults to
                       `dusttest.toml`).
    --seed <INT>       Use a specific seed instead of a random one.
    --ensemble <N>     Run each test N times with different seeds.
    --verbose          Show per‑run output.
    --list             List tests without running them.
    -h, --help         Display this help.
```

### Example

To run all tests once with a fixed seed:

```
$ dusttest --seed 42

Running 2 tests with seed 42
Test formatting: PASS
Test docs: PASS
All tests passed!
```

To run each test ten times, deriving seeds from an initial seed:

```
$ dusttest --ensemble 10 --seed 1234

Running 2 tests with ensemble size 10 starting from seed 1234
Test formatting: pass on 10 seeds
Test docs: pass on 9/10 seeds (failures on seeds: 1237)
```

## Implementation notes

`dusttest` is implemented in Rust.  It uses the `toml` crate to parse
configuration files, `rand` to generate deterministic seeds for each
ensemble, and `serde` for data structures.  The harness spawns
external commands using `std::process::Command`, passing the seed via
the `DUST_SEED` environment variable.  Exit codes and captured
outputs determine test success.

When you run `cargo test` in this crate, a suite of unit tests under
`tests/` validates seed handling and failure reporting.  This harness
is a starting point; future versions may integrate directly with the
Dust compiler to auto‑discover and run test functions within DPL
modules, apply deterministic thread schedulers【548468680421956†L118-L129】,
or support property‑based testing with shrinking【157165867946773†L116-L120】.

## License

Licensed under the **Dust Open Source License (DOSL)**.  See
`LICENSE` for details.