# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-14)

**Core value:** Identify where valuable targets are and what they're likely carrying
**Current focus:** v1.1 Demand Modeling — Phase 10 (Demand Registry)

## Current Position

Phase: 10 of 12 (Demand Registry)
Plan: 1 of 1 in current phase
Status: Phase 10 complete
Last activity: 2026-02-02 — Completed 10-01-PLAN.md

Progress: ████░░░░░░ ~40% (4 of ~6+ plans in v1.1)

## Performance Metrics

**Velocity:**
- Total plans completed: 31
- Average duration: 10 min
- Total execution time: 4.37 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Wikelo Data Model | 2 | 14 min | 7 min |
| 2. Item Source Research | 1 | 15 min | 15 min |
| 2.1. Game Data Extraction | 3 | 72 min | 24 min |
| 3. Wikelo Data Module | 2 | 32 min | 16 min |
| 4. Source Intel Integration | 3 | 25 min | 8 min |
| 5. TUI Wikelo Views | 3 | 23 min | 8 min |
| 5.1. TUI Snapshot Tests | 4 | 24 min | 6 min |
| 5.2. Coverage to 80% | 8 | 47 min | 6 min |
| 6. Testing & Polish | 2 | 10 min | 5 min |
| 7. Demand Research | 1 | — | — |
| 8. Demand Data Model | 1 | 5 min | 5 min |
| 9. Contract Data Population | 1 | 3 min | 3 min |
| 10. Demand Registry | 1 | 7 min | 7 min |

**Recent Trend:**
- Last 5 plans: 6m, 4m, 5m, 3m, 7m
- Trend: → (stable, fast)

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
- ContractCategory defaults to Equipment; DataConfidence uses repr(u8) for ordering
- CurrencyType::Other(String) for extensibility; all new WikieloContract fields use serde(default)
- Unknown contract quantities use 1 as placeholder; ATLS contracts categorized as Equipment
- ContractRegistry reuses normalize_location via pub(super); exported from crate root like WikieloRegistry

### Roadmap Evolution

- Phase 5.1 inserted after Phase 5: TUI Snapshot Test Coverage (URGENT) - discovered test gaps during 05-01 execution
- Phase 5.2 inserted after Phase 5.1: Coverage to 80% - CI coverage threshold blocking pushes (61.74% < 80%)
- Milestone v1.1 created: Demand Modeling, 6 phases (Phase 7-12)

### Deferred Issues

None yet.

### Blockers/Concerns

None.

## Session Continuity

Last session: 2026-02-02
Stopped at: Completed 10-01-PLAN.md (Phase 10 complete)
Resume file: `.planning/phases/10-demand-registry/10-01-SUMMARY.md`

### Critical Context for Next Session

**Phase 10 (Demand Registry) complete.** ContractRegistry with 5 bidirectional HashMap indexes, 10 query methods, 10 unit tests. 194 total tests passing.

Key additions: contract_registry.rs with ContractRegistry struct providing item→contract and location→contract lookups. contracts_requiring_item() is the key method for Phase 11 demand scoring.

**Next:** Phase 11 (Demand Intel Integration) — add demand scoring to TargetAnalyzer, flag ships heading TO demand locations.
