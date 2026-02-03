# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-14)

**Core value:** Identify where valuable targets are and what they're likely carrying
**Current focus:** v1.1 Demand Modeling — Phase 11 (Demand Intel Integration)

## Current Position

Phase: 12 of 12 (CLI/TUI Demand Views)
Plan: 1 of 2 in current phase
Status: In progress
Last activity: 2026-02-03 — Completed 12-01-PLAN.md

Progress: ██████░░░░ ~55% (6 of ~7+ plans in v1.1)

## Performance Metrics

**Velocity:**
- Total plans completed: 33
- Average duration: 10 min
- Total execution time: 4.57 hours

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
| 11. Demand Intel Integration | 1 | 8 min | 8 min |
| 12. CLI/TUI Demand Views | 1 | 4 min | 4 min |

**Recent Trend:**
- Last 5 plans: 5m, 3m, 7m, 8m, 4m
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
- Demand scoring mirrors source scoring: 20 base + per-contract bonuses, capped at 100
- Arriving targets flagged with demand at current location; departing flagged with demand at destination

### Roadmap Evolution

- Phase 5.1 inserted after Phase 5: TUI Snapshot Test Coverage (URGENT) - discovered test gaps during 05-01 execution
- Phase 5.2 inserted after Phase 5.1: Coverage to 80% - CI coverage threshold blocking pushes (61.74% < 80%)
- Milestone v1.1 created: Demand Modeling, 6 phases (Phase 7-12)

### Deferred Issues

None yet.

### Blockers/Concerns

None.

## Session Continuity

Last session: 2026-02-03
Stopped at: Completed 12-01-PLAN.md (Plan 1 of 2 in Phase 12)
Resume file: `.planning/phases/12-cli-tui-demand-views/12-01-SUMMARY.md`

### Critical Context for Next Session

**12-01 complete.** Item CLI command added with source/contract lookup and --list flag. 31 Wikelo items searchable via `sc-interdiction item <query>`. Shows sources with locations/systems/methods, contracts requiring items with turn-in locations. Both text and JSON output modes.

Key additions: Item command in CLI with handle_item(), ItemSearchResult/SourceInfo/ContractInfo structs, case-insensitive substring search, multiple match handling, helpful "no matches" messaging.

**Next:** 12-02 (TUI Demand Views) — add demand columns to targets/routes TUI views.
