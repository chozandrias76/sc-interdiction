---
phase: 05-tui-wikelo-views
plan: 03
subsystem: ui
tags: [tui, ratatui, wikelo, routes, hotspots]

# Dependency graph
requires:
  - phase: 05-02
    provides: WikieloIntel in App, hotspot detail Wikelo display
provides:
  - Wikelo score column in routes table
  - Wikelo item breakdown in hotspot detail panels
affects: [testing, phase-6]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Collect items from route origins for hotspot Wikelo data

key-files:
  created: []
  modified:
    - crates/cli/src/tui/views/routes.rs
    - crates/cli/src/tui/views/map.rs

key-decisions:
  - "Use route origins (not jump destination) for hotspot Wikelo items"
  - "High-value threshold: >10k aUEC estimated value"

patterns-established:
  - "HashSet for deduplicating items across multiple routes"

issues-created: []

# Metrics
duration: 5 min
completed: 2026-01-21
---

# Phase 5 Plan 3: Routes and Hotspot Wikelo Enhancement Summary

**Wikelo score column in routes table with color coding, hotspot panels show item breakdown from all intersecting route origins**

## Performance

- **Duration:** 5 min
- **Started:** 2026-01-21T22:45:35Z
- **Completed:** 2026-01-21T22:50:27Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- Routes table now displays Wikelo score column (0-100) between Haul Value and Ship
- Color coding: magenta star (>50), light magenta (>20), gray (<20 or none)
- Hotspot compact view shows up to 5 Wikelo items from all intersecting routes' origins
- Hotspot expanded view shows up to 3 Wikelo items with high-value star indicator
- Phase 5 complete: Wikelo intelligence fully integrated into all TUI views

## Task Commits

Each task was committed atomically:

1. **Task 1: Add Wikelo column to routes view** - `3db98ed` (feat)
2. **Task 2: Enhance hotspot details with Wikelo breakdown** - `a78f6c1` (feat)

**Plan metadata:** (this commit)

## Files Created/Modified

- `crates/cli/src/tui/views/routes.rs` - Added Wikelo column header and score display with color coding
- `crates/cli/src/tui/views/map.rs` - Enhanced hotspot panels with Wikelo item breakdown from route origins
- `crates/cli/src/tui/snapshots/sc_interdiction__tui__ui__tests__render_routes_view.snap` - Updated snapshot

## Decisions Made

- Use route origins (cargo sources) rather than jump destination for hotspot Wikelo items
- High-value detection uses same threshold as WikieloIntel (>10k aUEC estimated value)
- Star indicator for high-value items consistent with map ring coloring

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

- Phase 5 complete: Wikelo intelligence visible in targets view, map view, routes view, and hotspot details
- Ready for Phase 5.1 (TUI Snapshot Test Coverage)

---
*Phase: 05-tui-wikelo-views*
*Completed: 2026-01-21*
