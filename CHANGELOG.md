# Changelog - dusttest (DPL Testing Framework)

All notable changes to dusttest are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-02-12 (DPL v0.2)

### Added

- **DPL v0.2 Compliance**: Full support for v0.2 specification
- Full test framework for K Regime v0.2
- Unit test support with `#[test]` attribute
- Integration test support
- Benchmark support with `#[bench]` attribute
- Property-based testing support
- Memory safety testing (bounds checking, leak detection)
- Test organization in `tests/` directory
- Automatic test discovery
- Test reporting and coverage

### Changed

- Enhanced test runner for new K Regime features
- Improved assertion messages
- Better error reporting for test failures

### Fixed

- Memory safety issues in test harness
- Thread safety in concurrent tests

## [0.1.0] - 2026-02-12

### Added

- Initial testing framework
- Basic test discovery
- Simple assertion support
- Example test files

### Known Issues

- Limited to v0.1 emit-only testing

---

Copyright © 2026 Dust LLC