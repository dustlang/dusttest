# Platform Notes

## Shell Dependency

Current executor runs every test command through:

```text
sh -c <cmd>
```

This means:

- A POSIX shell named `sh` must be available.
- Command syntax should be shell-compatible.

## Environment Contract

For each run, harness sets:

- `DUST_SEED=<u64>`

Test commands should consume this variable when deterministic behavior is required.

## Windows Considerations

On Windows without an `sh` provider (MSYS2, Git Bash, WSL integration, etc.), test execution will fail at spawn time.

Recommended future enhancement:

- choose shell backend by platform (`sh` on Unix, `cmd` or PowerShell option on Windows),
- or execute commands directly without shell when possible.
