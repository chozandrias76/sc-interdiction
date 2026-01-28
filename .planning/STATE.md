# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-14)

**Core value:** Identify where valuable targets are and what they're likely carrying
**Current focus:** v1.1 Demand Modeling — Phase 7 (Demand Research)

## Current Position

Phase: 7 of 12 (Demand Research)
Plan: Not started
Status: Ready to plan
Last activity: 2026-01-27 — Milestone v1.1 created

Progress: ░░░░░░░░░░ 0% (0 of ? plans in v1.1)

## Performance Metrics

**Velocity:**
- Total plans completed: 28
- Average duration: 10 min
- Total execution time: 4.1 hours

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

**Recent Trend:**
- Last 5 plans: 8m, 5m, 5m, 6m, 4m
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

### Roadmap Evolution

- Phase 5.1 inserted after Phase 5: TUI Snapshot Test Coverage (URGENT) - discovered test gaps during 05-01 execution
- Phase 5.2 inserted after Phase 5.1: Coverage to 80% - CI coverage threshold blocking pushes (61.74% < 80%)
- Milestone v1.1 created: Demand Modeling, 6 phases (Phase 7-12)

### Deferred Issues

None yet.

### Blockers/Concerns

None.

## Session Continuity

Last session: 2026-01-27
Stopped at: Milestone v1.1 Demand Modeling created
Resume file: None

### Critical Context for Next Session

**v1.1 Demand Modeling milestone created.** 6 phases (7-12).

Research plan: `.planning/research/demand-modeling-research.md`

**Phase 7 (Demand Research)** needs `/gsd:research-phase 7` first to gather:
- Wikelo contracts from wikelotrades.com
- MG/Council faction systems
- Exchange rates (50 MG Scrip → 1 Favor)
- Other demand sources

**Next:** Run `/gsd:research-phase 7` to begin demand research.
