# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-14)

**Core value:** Identify where valuable targets are and what they're likely carrying
**Current focus:** Phase 4 — Source Intel Integration

## Current Position

Phase: 4 of 7 (Source Intel Integration)
Plan: 2 of 3 in current phase
Status: In progress
Last activity: 2026-01-20 — Completed 04-02-PLAN.md

Progress: ████████░░ 77% (10 of 13 plans complete)

## Performance Metrics

**Velocity:**
- Total plans completed: 10
- Average duration: 15 min
- Total execution time: 2.5 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Wikelo Data Model | 2 | 14 min | 7 min |
| 2. Item Source Research | 1 | 15 min | 15 min |
| 2.1. Game Data Extraction | 3 | 72 min | 24 min |
| 3. Wikelo Data Module | 2 | 32 min | 16 min |
| 4. Source Intel Integration | 2 | 15 min | 7.5 min |

**Recent Trend:**
- Last 5 plans: 35m, 15m, 17m, 10m, 5m
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

### Deferred Issues

None yet.

### Blockers/Concerns

None.

## Session Continuity

Last session: 2026-01-20
Stopped at: Completed 04-02-PLAN.md (TargetAnalyzer Wikelo integration)
Resume file: .planning/phases/04-source-intel-integration/04-03-PLAN.md

### Critical Context for Next Session

**Phase 4 IN PROGRESS.** 2 of 3 plans complete.

1. **04-01:** ✓ WikieloIntel struct with source flagging methods
2. **04-02:** ✓ TargetAnalyzer with WikieloIntel, TargetPrediction.wikelo_flag
3. **04-03:** Add Wikelo scoring to HotRoute and InterdictionHotspot

**Key Files:**
- WikieloIntel: `crates/intel/src/wikelo/intel.rs`
- WikieloRegistry: `crates/intel/src/wikelo/registry.rs`
- TargetAnalyzer: `crates/intel/src/targets.rs`
- Wikelo types: `crates/intel/src/wikelo/types.rs`
