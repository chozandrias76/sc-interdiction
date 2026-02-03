---
phase: 12-cli-tui-demand-views
plan: 02
type: summary
subsystem: tui
tags:
  - tui
  - demand
  - display
provides:
  - demand-column-targets-tui
  - demand-column-routes-tui
  - demand-detail-panel
affects:
  - crates/cli/src/tui/views/targets.rs
  - crates/cli/src/tui/views/routes.rs
  - crates/intel/src/lib.rs
  - crates/intel/src/wikelo/mod.rs
duration: PT5M
---

# Plan 12-02 Summary: Add Demand Columns to TUI Views

**One-liner:** Demand columns in targets/routes TUI with detail panel and updated snapshots.

## Completed Tasks

### Task 1: Add Demand column to targets view
**Commit:** `14c6fdc feat(12-02): add demand column to targets TUI view`

- Added "Demand" column header with Yellow color
- Added demand indicator showing contract count with Cyan color scheme
- High-value demand (★{count}) in Cyan, regular ({count}) in LightCyan, none (-) in DarkGray
- Created `render_demand_detail()` function for expanded detail panel
- Split detail panel horizontally when both Wikelo and Demand flags present
- Exported `DemandFlag` and `DemandContractSummary` from intel crate
- Refactored `render_targets()` into smaller helper functions to address clippy warnings
- Updated TUI snapshots for new column layout

**Column order:** Dir | Ship | Cargo | Destination | Wikelo | Demand | Value | Threat

### Task 2: Add Demand column to routes view
**Commit:** `afbfbc2 feat(12-02): add demand column to routes TUI view`

- Added "Demand" column header with Yellow color
- Added demand score indicator with color thresholds:
  - Score > 50: "★{score}" in Cyan (high demand)
  - Score > 20: "{score}" in LightCyan (medium demand)
  - Score > 0: "{score}" in DarkGray (low demand)
  - No demand: "-" in DarkGray
- Updated widths array to include new column (Length(7))
- Updated TUI snapshots for new column layout

**Column order:** Commodity | Origin | Destination | Profit/SCU | Haul Value | Wikelo | Demand | Ship

### Task 3: Update snapshot tests
Snapshots were updated and included with Task 1 and Task 2 commits. All 55 TUI tests pass.

## Files Modified

- `crates/cli/src/tui/views/targets.rs` - Added demand column and detail panel
- `crates/cli/src/tui/views/routes.rs` - Added demand score column
- `crates/intel/src/lib.rs` - Exported DemandFlag and DemandContractSummary
- `crates/intel/src/wikelo/mod.rs` - Exported DemandContractSummary
- `crates/cli/src/tui/snapshots/*.snap` - Updated 10 snapshot files

## Color Scheme

| Data Type | High Value | Regular | None |
|-----------|-----------|---------|------|
| Wikelo Source | Yellow (★) | Green | DarkGray |
| Demand | Cyan (★) | LightCyan | DarkGray |

This color scheme ensures clear visual distinction between supply-side (Wikelo/source) and demand-side (contract turn-in) intelligence.

## Verification

- [x] `cargo build --package sc-interdiction` succeeds
- [x] `cargo clippy --package sc-interdiction` passes (pre-existing warnings only)
- [x] `cargo test --package sc-interdiction` passes (55 tests)
- [x] Targets view shows Demand column with contract counts
- [x] Routes view shows Demand score column
- [x] Detail panel can show demand contract info
- [x] Color scheme distinguishes demand (cyan) from source (yellow/green)

## Duration

Start: 2026-02-03T05:16:14Z
End: 2026-02-03T05:21:13Z
Duration: ~5 minutes
