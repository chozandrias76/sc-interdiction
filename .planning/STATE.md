# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-14)

**Core value:** Identify where valuable targets are and what they're likely carrying
**Current focus:** Phase 3 — Wikelo Data Module

## Current Position

Phase: 3 of 7 (Wikelo Data Module)
Plan: 1 of 2 in current phase
Status: In progress
Last activity: 2026-01-18 — Completed 03-01-PLAN.md

Progress: █████░░░░░ 54% (7 of 13 plans complete)

## Performance Metrics

**Velocity:**
- Total plans completed: 7
- Average duration: 17 min
- Total execution time: 1.9 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Wikelo Data Model | 2 | 14 min | 7 min |
| 2. Item Source Research | 1 | 15 min | 15 min |
| 2.1. Game Data Extraction | 3 | 72 min | 24 min |
| 3. Wikelo Data Module | 1 | 15 min | 15 min |

**Recent Trend:**
- Last 5 plans: 15m, 15m, 12m, 35m, 15m
- Trend: → (stable)

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
- **NEW:** Import types from intel crate; wikelo-data depends on intel for types
- **NEW:** Normalized key matching (lowercase, collapsed whitespace) for flexible lookups

### Deferred Issues

None yet.

### Blockers/Concerns

None.

## Session Continuity

Last session: 2026-01-18
Stopped at: Completed 03-01-PLAN.md
Resume file: .planning/phases/03-wikelo-data-module/03-01-SUMMARY.md

### Critical Context for Next Session

**Phase 3 in progress.** Plan 03-01 complete, plan 03-02 next.

1. **WikieloRegistry** — Bidirectional indexes (by_id, by_location, by_system, by_category)
2. **Pattern established** — Follows ShipRegistry pattern with normalize() for flexible matching
3. **Next step** — Plan 03-02 adds static data from research and from_static() constructor

**Key Files:**
- WikieloRegistry: `crates/wikelo-data/src/registry.rs`
- Types (imported): `crates/intel/src/wikelo/types.rs`
- Research data: `.planning/phases/02-item-source-research/02-DATA-REFERENCE.md`
