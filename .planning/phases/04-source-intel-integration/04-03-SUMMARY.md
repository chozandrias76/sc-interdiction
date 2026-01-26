---
phase: 04-source-intel-integration
plan: 03
subsystem: intel
tags: [target-analysis, wikelo, route-scoring, hotspots]

# Dependency graph
requires:
  - phase: 04-02
    provides: WikieloIntel integrated into TargetAnalyzer
provides:
  - HotRoute with wikelo_score and wikelo_items fields
  - InterdictionHotspot with wikelo_potential and wikelo_items fields
  - calculate_wikelo_score() helper function
  - Wikelo scoring for route and hotspot prioritization
affects: [05-01, 06-01]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Reusable calculate_wikelo_score() helper for all Wikelo scoring
    - Scoring formula: 20 base + 10/high-value + 5/item (capped at 100)

key-files:
  created: []
  modified:
    - crates/intel/src/targets.rs
    - crates/intel/src/targets_tests.rs
    - crates/cli/src/tui/ui.rs

key-decisions:
  - "Same scoring formula for both HotRoute and InterdictionHotspot"
  - "Score is Option<f64> (None = no WikieloIntel, Some(0) = not a source)"

patterns-established:
  - "Wikelo scoring returns (Option<f64>, Vec<String>) for score and items"

issues-created: []

# Metrics
duration: 8min
completed: 2026-01-21
---

# Phase 4 Plan 3: Wikelo Route and Hotspot Scoring Summary

**HotRoute and InterdictionHotspot enhanced with Wikelo scoring to prioritize locations with collectible items**

## Performance

- **Duration:** 8 min
- **Started:** 2026-01-21T00:32:00Z
- **Completed:** 2026-01-21T00:40:32Z
- **Tasks:** 3
- **Files modified:** 3

## Accomplishments

- Added `wikelo_score` and `wikelo_items` to HotRoute for route origin scoring
- Added `wikelo_potential` and `wikelo_items` to InterdictionHotspot for location scoring
- Created reusable `calculate_wikelo_score()` helper with documented scoring formula
- Added 4 new tests (66 total in intel crate now)
- Phase 4 complete: Wikelo source flagging fully integrated into intel crate

## Task Commits

Each task was committed atomically:

1. **Task 1: Add wikelo fields to HotRoute** - `6ec661d` (feat)
2. **Task 2: Add wikelo fields to InterdictionHotspot** - `8d4838c` (feat)
3. **Task 3: Add tests for Wikelo scoring** - `e9a14a7` (test)

**Plan metadata:** (this commit) (docs: complete plan)

## Files Created/Modified

- `crates/intel/src/targets.rs` - HotRoute and InterdictionHotspot with Wikelo scoring, calculate_wikelo_score() helper
- `crates/intel/src/targets_tests.rs` - 4 new tests for Wikelo scoring
- `crates/cli/src/tui/ui.rs` - Updated test data for new struct fields

## Decisions Made

- Reused same scoring formula for both HotRoute and InterdictionHotspot (consistency)
- Score is Option<f64> to distinguish "no WikieloIntel" (None) from "not a Wikelo source" (Some(0.0))

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None

## Next Phase Readiness

- Phase 4 complete: Wikelo source intelligence fully integrated into intel crate
- Ready for Phase 5 (TUI Wikelo Views) or Phase 6 (API endpoints)
- HotRoute and InterdictionHotspot now carry Wikelo scoring for prioritization

---
*Phase: 04-source-intel-integration*
*Completed: 2026-01-21*
