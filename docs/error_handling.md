# Error Handling

## Config Loading Errors

`load_config` can fail for:

- file read errors (`std::io::Error`),
- TOML parse errors mapped to `io::ErrorKind::InvalidData`.

CLI reports:

`error: unable to read config <path>: <error>`

## Argument Errors

CLI prints errors and returns for:

- missing value after `--config`, `--seed`, `--ensemble`,
- non-integer value for `--seed` or `--ensemble`.

## Runtime Command Errors

During `run_tests`, command spawn/execution errors:

- are treated as failed runs,
- add the seed to `failing_seeds`,
- include error text in `outputs` only when verbose mode is enabled.

## No-Work Conditions

- No tests in config -> message and return.
- No seeds generated (`--ensemble 0`) -> message and return.
