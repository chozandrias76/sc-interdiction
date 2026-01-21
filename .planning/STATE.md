# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-14)

**Core value:** Identify where valuable targets are and what they're likely carrying
**Current focus:** Phase 5 — TUI Wikelo Views

## Current Position

Phase: 5 of 7 (TUI Wikelo Views)
Plan: 2 of 3 in current phase
Status: In progress
Last activity: 2026-01-21 — Completed 05-02-PLAN.md

Progress: █████████░ 93% (13 of 14 plans complete)

## Performance Metrics

**Velocity:**
- Total plans completed: 13
- Average duration: 13.5 min
- Total execution time: 2.9 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Wikelo Data Model | 2 | 14 min | 7 min |
| 2. Item Source Research | 1 | 15 min | 15 min |
| 2.1. Game Data Extraction | 3 | 72 min | 24 min |
| 3. Wikelo Data Module | 2 | 32 min | 16 min |
| 4. Source Intel Integration | 3 | 25 min | 8 min |
| 5. TUI Wikelo Views | 2 | 18 min | 9 min |

**Recent Trend:**
- Last 5 plans: 10m, 5m, 10m, 12m, 6m
- Trend: ↓ (faster)

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- Source location flagging chosen as core feature (most actionable intel)
- Static item→source mapping (offline-first, reliability over freshness)
- Skip live UEX pricing for v1
- Valakkar locations include Daymar (Stanton), not just Pyro
- Quasi Grazer native to Terra III, not microTech
- 9 low-confidence items flagged for gameplay validation before treating as reliable
- Data.p4k is authoritative source; wiki research is starting point only
- Need Phase 2.1 to build game data extraction pipeline before Phase 3
- scdatatools broken; using scunpacked-data repo for game data instead
- Mission data NOT in scunpacked-data; Phase 3 needs wiki scraping for contract details
- Using in-memory lazy caching (no disk serialization needed for ~50MB data)
- LocalizationStore supports both labels.json and global.ini formats
- Import types from intel crate; wikelo-data depends on intel for types
- Normalized key matching (lowercase, collapsed whitespace) for flexible lookups
- **NEW:** Moved registry/items from wikelo-data to intel to resolve cyclic dependency
- Only flag departing targets (arriving have cargo already on ship, source flagging not useful)

### Roadmap Evolution

- Phase 5.1 inserted after Phase 5: TUI Snapshot Test Coverage (URGENT) - discovered test gaps during 05-01 execution

### Deferred Issues

None yet.

### Blockers/Concerns

None.

## Session Continuity

Last session: 2026-01-21
Stopped at: Completed 05-02-PLAN.md
Resume file: None (ready for /gsd:execute-plan 05-03)

### Critical Context for Next Session

**Phase 5 IN PROGRESS.** 2 of 3 plans done.

1. **05-01:** ✓ Wikelo column and detail panel in targets view
2. **05-02:** ✓ Map view source highlighting (magenta rings, 'w' filter, hotspot items)
3. **05-03:** Pending — Hotspot/detail panel enhancement

**Key Files:**
- Map view: `crates/cli/src/tui/views/map.rs` (Wikelo ring rendering, filter, hotspot items)
- App state: `crates/cli/src/tui/app.rs` (wikelo_intel, wikelo_filter fields)
- Key handlers: `crates/cli/src/tui/handlers/keys.rs` ('w' toggle)
