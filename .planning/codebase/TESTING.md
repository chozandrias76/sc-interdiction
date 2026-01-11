# Testing Patterns

**Analysis Date:** 2026-01-11

## Test Framework

**Runner:**
- Built-in Rust test framework (`#[test]`, `#[tokio::test]`)
- No external test runner

**Assertion Library:**
- Built-in `assert!`, `assert_eq!`, `assert_ne!`
- Pattern matching with `matches!` macro

**Run Commands:**
```bash
make test                    # Run all tests
make test-cli               # Run CLI crate tests
make test-pkg PKG=intel     # Run specific package tests
cargo test -- --nocapture   # With output
```

## Test File Organization

**Location:**
- Unit tests: Co-located with source (`#[cfg(test)] mod tests`)
- Complex tests: Separate file (`*_tests.rs` alongside module)
- Integration tests: `crates/{crate}/tests/*.rs`

**Naming:**
- Unit tests: `module.rs` contains `#[cfg(test)] mod tests { ... }`
- Separate tests: `module_tests.rs` (e.g., `ships_tests.rs`)
- Integration: `feature_integration.rs`

**Structure:**
```
crates/intel/src/
├── lib.rs              # Module exports
├── targets.rs          # Source code with inline tests
├── targets_tests.rs    # Separate test file (complex tests)
├── ships/
│   ├── mod.rs
│   ├── types.rs        # Has #[cfg(test)] mod tests
│   ├── registry.rs     # Has #[cfg(test)] mod tests
│   └── enrichment.rs   # Has #[cfg(test)] mod tests
└── ships_tests.rs      # Complex ship tests (591 lines)
```

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::assertions_on_constants)]
mod tests {
    use super::*;

    #[test]
    fn test_function_success_case() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_function_error_case() {
        // Test error conditions
    }
}
```

**Patterns:**
- Arrange/Act/Assert structure (not always explicit)
- One logical assertion per test (multiple `assert!` OK)
- `#[allow(clippy::unwrap_used)]` at module level for tests
- Descriptive test names: `test_<what>_<scenario>`

## Mocking

**Framework:**
- Mockito 1.2 - HTTP request mocking (`crates/api-client/`, `crates/intel/`, `crates/server/`)

**Patterns:**
```rust
// From crates/intel/tests/target_analyzer_integration.rs
use mockito::{Server, Mock};

#[tokio::test]
async fn test_with_mock_api() {
    let mut server = Server::new_async().await;

    let mock = server.mock("GET", "/commodities")
        .with_status(200)
        .with_body(json_response)
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let result = client.get_commodities().await;

    mock.assert();
    assert!(result.is_ok());
}
```

**What to Mock:**
- External HTTP APIs (UEX, FleetYards, StarCitizen)
- Database connections (when testing without DB)

**What NOT to Mock:**
- Internal pure functions
- Route graph calculations
- Ship registry lookups

## Fixtures and Factories

**Test Data:**
```rust
// Factory function pattern (from tests)
fn mock_trade_route() -> TradeRoute {
    TradeRoute {
        commodity_code: "AGRI".to_string(),
        origin_code: "LORVL".to_string(),
        destination_code: "AREA18".to_string(),
        profit_per_unit: 10.0,
        scu_origin: 100.0,
        // ...
    }
}

// Multiple variants
fn mock_trade_route_with_profit(profit: f64) -> TradeRoute {
    TradeRoute {
        profit_per_unit: profit,
        ..mock_trade_route()
    }
}
```

**Location:**
- Factory functions: Defined in test file near usage
- Shared fixtures: `tests/fixtures/` (if needed)
- JSON fixtures: Inline strings or separate `.json` files

## Coverage

**Requirements:**
- Minimum: 80% line coverage (enforced)
- Tool: cargo-tarpaulin

**Configuration:**
- Exclusions: Test files, config files

**Commands:**
```bash
make coverage          # Generate coverage report
make coverage-html     # Generate HTML report (target/tarpaulin/)
make coverage-check    # Check >= 80% threshold
```

**Current Status (from TESTING.md):**
| Crate | Tests | Coverage |
|-------|-------|----------|
| api-client | 9 | TBD |
| intel | 13 | TBD |
| route-graph | 11 | TBD |
| cli | 13 | TBD |
| server | ~5 | TBD |
| **Total** | **~50** | **>=80%** |

## Test Types

**Unit Tests:**
- Scope: Single function in isolation
- Mocking: None or minimal
- Speed: Fast (<100ms per test)
- Examples: `crates/route-graph/src/fuel.rs` tests

**Integration Tests:**
- Scope: Multiple modules together
- Mocking: External APIs only
- Location: `crates/{crate}/tests/`
- Examples: `crates/intel/tests/target_analyzer_integration.rs`

**Snapshot Tests:**
- Framework: Insta 1.45.1
- Purpose: TUI rendering verification
- Location: `crates/cli/src/tui/snapshots/`
- Update: `cargo insta review`

## Common Patterns

**Async Testing:**
```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

**Error Testing:**
```rust
#[test]
fn test_invalid_input_returns_error() {
    let result = function_with_bad_input(None);
    assert!(result.is_err());
    assert!(matches!(result, Err(MyError::InvalidInput)));
}
```

**Boundary Testing:**
```rust
#[test]
fn test_boundary_conditions() {
    assert_eq!(function(0), expected_for_zero);
    assert_eq!(function(u8::MAX), expected_for_max);
    assert!(function(u8::MAX).checked_add(1).is_none());
}
```

**Float Comparison:**
```rust
#[test]
fn test_float_calculation() {
    let result = calculate_something();
    let expected = 42.5;
    let epsilon = 0.001;
    assert!((result - expected).abs() < epsilon);
}
```

## CI/CD Integration

**Pre-commit Hook:**
1. Format check (`cargo fmt --check`)
2. Clippy lint (`cargo clippy -- -D warnings`)
3. All tests (`cargo test`)
4. Commit size warning (>500 lines)

**Make Targets:**
```makefile
test:           cargo test --workspace
test-cli:       cargo test -p cli
coverage:       cargo tarpaulin --out Html
coverage-check: # Enforces 80% minimum
```

---

*Testing analysis: 2026-01-11*
*Update when test patterns change*
