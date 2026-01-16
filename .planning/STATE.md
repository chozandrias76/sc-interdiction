# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-14)

**Core value:** Identify where valuable targets are and what they're likely carrying
**Current focus:** Phase 3 — Wikelo Data Module

## Current Position

Phase: 3 of 6 (Wikelo Data Module) — BLOCKED
Plan: Research complete, planning blocked
Status: Awaiting Phase 2.1 insertion (Game Data Extraction)
Last activity: 2026-01-15 — Phase 3 research discovered Data.p4k requirement

Progress: ███░░░░░░░ 23% (paused for roadmap update)

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

Last session: 2026-01-15
Stopped at: Phase 3 research — discovered Data.p4k requirement
Resume file: .planning/phases/03-wikelo-data-module/03-RESEARCH.md

### Critical Context for Next Session

**Discovery:** Star Citizen's `Data.p4k` (150GB archive) contains authoritative game data:
- `Data\Game2.dcb` (294MB) — DataForge database with items, contracts, missions
- `Data\Localization\english\global.ini` (9.5MB) — English text strings

**Action Required:**
1. Update ROADMAP.md to insert Phase 2.1 (Game Data Extraction Pipeline)
2. Extract Game2.dcb and global.ini using 7z (now installed in WSL)
3. Convert .dcb to XML using unforge or explore native Rust options
4. Build CLI viewer for exploring DataForge structure
5. Find Wikelo contract definitions in extracted data
6. Then proceed with Phase 3 using authoritative data

**Key Files:**
- Game data: `/mnt/c/Star Citizen/StarCitizen/LIVE/Data.p4k`
- Research: `.planning/phases/03-wikelo-data-module/03-RESEARCH.md`
- Extraction tools: [unp4k](https://github.com/dolkensp/unp4k), [StarBreaker](https://github.com/diogotr7/StarBreaker)

**Crates to Update:**
- `sc-data-extractor` — needs Data.p4k support (currently uses SCLogistics)
- `data-viewer` — new CLI for exploring extracted data
