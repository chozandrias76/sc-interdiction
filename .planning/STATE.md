# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-14)

**Core value:** Identify where valuable targets are and what they're likely carrying
**Current focus:** Phase 3 — Wikelo Data Module

## Current Position

Phase: 2.1 of 7 (Game Data Extraction) — Ready for planning
Plan: Research complete (02.1-RESEARCH.md)
Status: Phase 2.1 inserted into roadmap, ready to plan
Last activity: 2026-01-16 — Phase 2.1 research complete

Progress: ███░░░░░░░ 23% (Phase 2.1 ready for planning)

## Performance Metrics

**Velocity:**
- Total plans completed: 3
- Average duration: 9 min
- Total execution time: 0.5 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Wikelo Data Model | 2 | 14 min | 7 min |
| 2. Item Source Research | 1 | 15 min | 15 min |

**Recent Trend:**
- Last 5 plans: 6m, 8m, 15m
- Trend: ↑ (research verification heavier than type definitions)

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
- **NEW:** Data.p4k is authoritative source; wiki research is starting point only
- **NEW:** Need Phase 2.1 to build game data extraction pipeline before Phase 3

### Deferred Issues

None yet.

### Blockers/Concerns

None yet.

## Session Continuity

Last session: 2026-01-16
Stopped at: Phase 2.1 research complete
Resume file: .planning/phases/02.1-game-data-extraction/02.1-RESEARCH.md

### Critical Context for Next Session

**Phase 2.1 research is complete.** Key findings:

1. **No Rust crate exists** for p4k/dcb parsing — all tools are C#/Python
2. **Recommended approach:** Shell out to `scdatatools` (Python), parse JSON output
3. **Don't hand-roll** format parsers — complex, changes with game updates

**Ready for next action:**
- Run `/gsd:plan-phase 2.1` to create execution plan
- Plans will cover: scdatatools setup, extraction, CLI viewer, caching

**Key Files:**
- Game data: `/mnt/c/Star Citizen/StarCitizen/LIVE/Data.p4k`
- Research: `.planning/phases/02.1-game-data-extraction/02.1-RESEARCH.md`
- Tool: `pip install scdatatools` (Python, actively maintained)

**Crates to Update:**
- `sc-data-extractor` — add Data.p4k extraction via scdatatools
- `data-viewer` — new CLI for exploring extracted DataForge JSON
