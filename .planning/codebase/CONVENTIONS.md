# Coding Conventions

**Analysis Date:** 2026-01-11

## Naming Patterns

**Files:**
- snake_case for all Rust files: `target_analyzer.rs`, `hot_routes.rs`
- *_tests.rs for separate test files alongside source
- mod.rs for module directory exports

**Functions:**
- snake_case for all functions: `get_hot_routes()`, `find_by_name()`
- No special prefix for async functions
- `handle_*` for event handlers: `handle_key()`, `handle_tick()`

**Variables:**
- snake_case for variables: `trade_routes`, `ship_registry`
- SCREAMING_SNAKE_CASE for constants: `COVERAGE_THRESHOLD`, `QT_DRIVE_EFFICIENCY`
- No underscore prefix for private members

**Types:**
- PascalCase for structs/enums: `CargoShip`, `TargetAnalyzer`, `ScrollState`
- No I prefix for traits: `TradeDataSource`, not `ITradeDataSource`
- PascalCase for enum variants: `View::Targets`, `NodeType::Station`

## Code Style

**Formatting:**
- rustfmt (standard Rust formatting)
- 4-space indentation (Rust default)
- 100 character line limit (soft)
- Run: `make fmt` or `cargo fmt`

**Linting:**
- Clippy with workspace lints (`Cargo.toml` lines 60-100)
- Strict rules: `unsafe_code = "deny"`, `unused = "deny"`
- Quality warnings: `unwrap_used`, `expect_used`, `panic`, `cognitive_complexity`
- Run: `make clippy` or `cargo lint`
- Config: `clippy.toml` (cognitive_complexity_threshold = 15)

## Import Organization

**Order:**
1. Standard library (`std::*`)
2. External crates (`serde`, `tokio`, `petgraph`)
3. Workspace crates (`api_client`, `intel`, `route_graph`)
4. Local modules (`super::*`, `crate::*`)

**Grouping:**
- Blank line between groups
- No sorting enforced (follow existing patterns)
- Use braces for multiple imports: `use std::{collections::HashMap, sync::Arc};`

**Path Aliases:**
- No path aliases defined
- Use full crate names: `api_client::UexClient`

## Error Handling

**Patterns:**
- Return `Result<T, E>` from fallible functions
- Custom errors via thiserror: `#[derive(Debug, Error)]`
- Context via eyre for CLI commands
- Avoid bare `unwrap()` - use `expect()` with message or `?`

**Error Types:**
- Per-crate error types: `GraphError`, `ApiError`
- When to throw: Invalid input, missing data, network failures
- When to return: Expected failures (no path found, item not found)

**Workspace Lints:**
```toml
# From Cargo.toml
unwrap_used = "warn"
expect_used = "warn"
panic = "warn"
```

## Logging

**Framework:**
- Tracing with tracing-subscriber
- Levels: trace, debug, info, warn, error

**Patterns:**
- Structured logging: `tracing::info!(count = ships.len(), "Loaded ships")`
- Log at service boundaries, not in utilities
- Error logging before returning errors
- No console.log equivalent - use tracing macros

## Comments

**When to Comment:**
- Explain why, not what: `// Retry 3 times because API has transient failures`
- Document business rules and domain logic
- Explain non-obvious algorithms or workarounds
- STATUS markers for unverified data: `/// **STATUS: ESTIMATED VALUES**`

**Doc Comments:**
- `//!` for module-level documentation
- `///` for public API documentation
- Include `# Examples` for complex functions
- Required tags: `# Panics`, `# Errors` when applicable

**Example from `crates/route-graph/src/fuel.rs`:**
```rust
//! Quantum fuel consumption calculations.
//!
//! Provides functions to calculate quantum travel fuel consumption
//! based on distance and ship quantum drive efficiency.

/// Get the default efficiency for a quantum drive size (1-3).
///
/// Returns None if size is out of range.
#[must_use]
pub fn efficiency_for_size(size: u8) -> Option<&'static QtDriveEfficiency> {
```

**TODO Comments:**
- Format: `// TODO: description`
- For unverified: `// STATUS: Gameplay assumption - needs verification`

## Function Design

**Size:**
- Keep under 50 lines (soft limit)
- `#[allow(clippy::too_many_lines)]` for complex rendering functions
- Extract helpers for complex logic

**Parameters:**
- Max 5-7 parameters (enforced by `.code-quality.toml`)
- Use options struct for many parameters
- Destructure in parameter list when helpful

**Return Values:**
- Explicit return types (no elision for public functions)
- `#[must_use]` for pure functions returning values
- Return early for guard clauses

## Module Design

**Exports:**
- Named exports via `pub use` in `lib.rs`
- Re-export public types from submodules
- Keep internal helpers private (no `pub`)

**Example from `crates/intel/src/lib.rs`:**
```rust
mod ships;
mod targets;

pub use ships::{CargoShip, ShipRegistry, ShipRole};
pub use targets::{HotRoute, TargetAnalyzer, TargetPrediction};
```

**Barrel Files:**
- `mod.rs` re-exports public API from submodules
- Avoid deep nesting (max 2-3 levels)
- No circular dependencies

## Rust-Specific Patterns

**Attributes:**
- `#[must_use]` on pure functions
- `#[allow(clippy::*)]` with justification comment
- `#[cfg(test)]` for test modules

**Lifetimes:**
- Elide when possible
- Explicit only when compiler requires

**Async:**
- Use `async fn` with `.await`
- Prefer `tokio::spawn` for concurrent tasks
- No blocking in async contexts

---

*Convention analysis: 2026-01-11*
*Update when patterns change*
