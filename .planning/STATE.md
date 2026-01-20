# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-14)

**Core value:** Identify where valuable targets are and what they're likely carrying
**Current focus:** Phase 4 — Source Intel Integration

## Current Position

Phase: 4 of 7 (Source Intel Integration)
Plan: 0 of 3 in current phase
Status: Planning complete, ready for execution
Last activity: 2026-01-19 — Created 04-01, 04-02, 04-03 PLAN.md

Progress: ██████░░░░ 62% (8 of 13 plans complete)

## Performance Metrics

**Velocity:**
- Total plans completed: 8
- Average duration: 17 min
- Total execution time: 2.2 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Wikelo Data Model | 2 | 14 min | 7 min |
| 2. Item Source Research | 1 | 15 min | 15 min |
| 2.1. Game Data Extraction | 3 | 72 min | 24 min |
| 3. Wikelo Data Module | 2 | 32 min | 16 min |

**Recent Trend:**
- Last 5 plans: 15m, 12m, 35m, 15m, 17m
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

Last session: 2026-01-19
Stopped at: Created Phase 4 plans (04-01, 04-02, 04-03)
Resume file: .planning/phases/04-source-intel-integration/04-01-PLAN.md

### Critical Context for Next Session

**Phase 4 PLANNED.** 3 plans ready for execution.

1. **04-01:** Create WikieloIntel struct with source flagging methods
2. **04-02:** Integrate WikieloIntel into TargetAnalyzer
3. **04-03:** Add Wikelo scoring to HotRoute and InterdictionHotspot

**Key Files:**
- WikieloRegistry: `crates/wikelo-data/src/registry.rs`
- TargetAnalyzer: `crates/intel/src/targets.rs`
- Wikelo types: `crates/intel/src/wikelo/types.rs`
