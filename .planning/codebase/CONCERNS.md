# Codebase Concerns

**Analysis Date:** 2026-01-11

## Tech Debt

**Dynamic SQL Construction in Data Viewer:**
- Issue: SQL queries built with `format!()` for table/column names
- Files: `crates/data-viewer/src/db.rs` (lines 54-62, 87-96, 194-197, 209-273)
- Why: Quick implementation for browsing arbitrary tables
- Impact: Potential SQL injection risk despite escaping functions
- Fix approach: Refactor to use Diesel query builder or parameterized queries

**Large Functions with Clippy Overrides:**
- Issue: Multiple functions exceed line limits with `#[allow(clippy::too_many_lines)]`
- Files:
  - `crates/cli/src/tui/views/map.rs` (506 lines, 3 overrides)
  - `crates/route-graph/src/spatial.rs` (1,116 lines)
  - `crates/cli/src/main.rs` (744 lines)
- Why: Complex TUI rendering and spatial calculations
- Impact: Hard to maintain and test
- Fix approach: Extract rendering, layout, and data preparation into separate functions

**Excessive Cloning:**
- Issue: 440+ instances of `.clone()`, `.to_string()`, `.to_owned()`
- Files: Throughout codebase, notably `crates/cli/src/tui/views/map.rs`, `crates/route-graph/src/spatial.rs`
- Why: Easier ownership handling
- Impact: Unnecessary allocations affecting performance
- Fix approach: Use references and iterators where possible

## Known Bugs

**No Critical Bugs Found**

The codebase is stable with proper error handling. Minor issues noted below.

## Security Considerations

**SQL Construction Pattern:**
- Risk: SQL injection via table/column names in data-viewer
- Files: `crates/data-viewer/src/db.rs`
- Current mitigation: `quote_identifier()` and `escape_string()` functions
- Recommendations: Migrate to Diesel query DSL for type-safe queries

**No Unsafe Code:**
- `unsafe_code = "deny"` in workspace lints
- No `unsafe` blocks found in codebase
- Good: Memory safety guaranteed by compiler

## Performance Bottlenecks

**Spatial Indexing - Linear Search:**
- Problem: `find_nearest()` collects all hotspots into vec, then sorts
- File: `crates/route-graph/src/spatial.rs` (lines 107-129)
- Measurement: O(n log n) for each query
- Cause: No spatial index (R-tree, KD-tree)
- Improvement path: Use a heap for k-nearest, or add proper spatial index

**API Response Not Streamed:**
- Problem: Large API responses loaded entirely into memory
- Files: `crates/api-client/src/fleetyards.rs`, `crates/api-client/src/uex.rs`
- Measurement: Memory spikes during initial data load
- Cause: reqwest `.json()` deserializes full response
- Improvement path: Stream large responses or add pagination limits

## Fragile Areas

**TUI Event Loop:**
- Files: `crates/cli/src/tui/event.rs`, `crates/cli/src/tui/app.rs`
- Why fragile: Complex state machine with multiple view modes
- Common failures: State inconsistency after rapid key presses
- Safe modification: Add comprehensive integration tests before changes
- Test coverage: Snapshot tests exist but limited state transition coverage

**Ship Registry Estimation:**
- Files: `crates/intel/src/ships/registry.rs` (line 42 has complexity override)
- Why fragile: Complex estimation logic with nested conditions
- Common failures: Incorrect ship matching for edge cases
- Safe modification: Extract into smaller testable functions
- Test coverage: Comprehensive tests in `ships_tests.rs`

## Scaling Limits

**In-Memory Caching:**
- Current capacity: Holds all API responses in memory
- Limit: System RAM
- Symptoms at limit: Memory exhaustion, OOM
- Scaling path: Add LRU eviction or external cache (Redis)

**Route Graph Size:**
- Current capacity: ~200 terminals (current game data)
- Limit: Petgraph handles thousands of nodes efficiently
- Symptoms at limit: Slow pathfinding on large graphs
- Scaling path: Current implementation should scale adequately

## Dependencies at Risk

**All Dependencies Current** (✓ Positive Finding)
- Tokio 1.0, Axum 0.7, Diesel 2.2, Ratatui 0.29 - all actively maintained
- No deprecated or unmaintained dependencies detected

## Missing Critical Features

**No User Authentication:**
- Problem: REST API has no auth (anyone can query)
- Current workaround: Local use only
- Blocks: Multi-user deployment, access control
- Implementation complexity: Low (add API key middleware)

**No Data Refresh Mechanism:**
- Problem: Ship/commodity data goes stale
- Current workaround: Manual re-import
- Blocks: Always-fresh predictions
- Implementation complexity: Medium (add cache expiry, background refresh)

## Test Coverage Gaps

**Data Viewer SQL Construction:**
- What's not tested: SQL query building in `crates/data-viewer/src/db.rs`
- Risk: SQL injection or syntax errors undetected
- Priority: Medium
- Difficulty: Need to mock database or use test fixtures

**TUI State Transitions:**
- What's not tested: Full state machine transitions in TUI
- Risk: UI bugs from unexpected state combinations
- Priority: Low (snapshots cover most rendering)
- Difficulty: Need integration test harness for TUI

## Unverified Data

**Quantum Fuel Efficiency Constants:**
- File: `crates/route-graph/src/fuel.rs` (lines 32-52)
- Status: `STATUS: ESTIMATED VALUES - NEEDS VERIFICATION`
- Issue: Fuel consumption rates (40.0, 80.0, 160.0 per Mkm) are estimates
- Impact: All fuel calculations and range predictions depend on these
- Recommendation: Verify with controlled in-game testing

**Position Estimation Fallbacks:**
- File: `crates/route-graph/src/spatial.rs` (lines 196+)
- Issue: Hardcoded position maps with "rough estimates for demo purposes"
- Impact: Hotspot positioning may be inaccurate
- Recommendation: Add confidence indicators for estimated vs. actual positions

**Gameplay Assumptions:**
- File: `crates/intel/src/targets.rs` (line 528)
- Status: `// STATUS: Gameplay assumption - needs verification`
- Issue: Threat calculations based on unverified mechanics
- Recommendation: Document assumptions and add validation tests

---

*Concerns audit: 2026-01-11*
*Update as issues are fixed or new ones discovered*
