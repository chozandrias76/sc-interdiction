# Testing Guide

This document describes the testing strategy and coverage requirements for sc-interdiction.

## Test Coverage Requirements

**Minimum coverage**: 80%

All code changes must maintain or improve test coverage. Coverage is measured using `cargo-tarpaulin`.

## Running Tests

### Run all tests
```bash
make test
# or
cargo test --all
```

### Run tests for a specific crate
```bash
cargo test -p api-client
cargo test -p intel
cargo test -p route-graph
cargo test -p sc-interdiction
```

## Test Coverage

### Prerequisites

Install system dependencies:
```bash
# Ubuntu/Debian/WSL
sudo apt install pkg-config libssl-dev

# macOS
brew install pkg-config openssl
```

Install `cargo-tarpaulin`:
```bash
make install-coverage-tools
# or
cargo install cargo-tarpaulin
```

### Generate Coverage Report

**Terminal output:**
```bash
make coverage
```

**HTML report:**
```bash
make coverage-html
# Opens target/tarpaulin/tarpaulin-report.html
```

**Check if coverage meets 80% threshold:**
```bash
make coverage-check
```

This command will:
- Run tests with coverage
- Display current coverage percentage
- Exit with error code 1 if below 80%
- Exit with code 0 if >= 80%

### CI/CD Integration

Add to your CI pipeline:
```yaml
- name: Check test coverage
  run: make coverage-check
```

## Test Organization

### Unit Tests

- **api-client**: HTTP client mocking, API response parsing
- **intel**: Business logic, calculations, data validation
- **route-graph**: Graph algorithms, pathfinding, fuel calculations
- **cli**: TUI rendering, event handling (snapshot tests)

### Test File Naming

- Production code: `src/module.rs`
- Tests: `src/module_tests.rs` or inline `#[cfg(test)] mod tests { ... }`

### Writing Tests

**Avoid `.unwrap()` in tests** - Clippy will warn on this. Use `?` or `.expect()` with meaningful messages:
```rust
#[test]
fn test_example() {
    let result = some_function().expect("function should succeed");
    assert_eq!(result, expected);
}
```

**Test naming convention:**
```rust
#[test]
fn test_<what_is_being_tested>_<scenario>() {
    // Arrange
    let input = setup();

    // Act
    let result = function_under_test(input);

    // Assert
    assert_eq!(result, expected);
}
```

## Current Test Statistics

| Crate | Tests | Coverage |
|-------|-------|----------|
| api-client | 9 | TBD |
| intel | 13 | TBD |
| route-graph | 11 | TBD |
| cli | 13 | TBD |
| server | 0 | 0% |
| **Total** | **46** | **TBD** |

Run `make coverage` to see current coverage percentages.

## Test Coverage Goals

- [x] api-client: Add HTTP mocking tests for UexClient
- [x] intel: Add tests for LootEstimate and CargoShip methods
- [ ] intel: Add tests for TargetAnalyzer
- [ ] api-client: Add tests for FleetYardsClient
- [ ] server: Add route handler tests
- [ ] Achieve 80%+ overall coverage

## Troubleshooting

**Tarpaulin compilation fails:**
- Ensure `pkg-config` and `libssl-dev` are installed
- Try: `sudo apt install build-essential pkg-config libssl-dev`

**Tests timeout:**
- Increase timeout in Cargo.toml test profile
- Or run with: `cargo test -- --test-threads=1`

**Coverage seems incorrect:**
- Ensure `--skip-clean` is used to preserve test artifacts
- Check that all crates are included in workspace

## References

- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [Rust testing best practices](https://doc.rust-lang.org/book/ch11-00-testing.html)
