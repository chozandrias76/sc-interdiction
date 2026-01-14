# SC Interdiction

## What This Is

A Star Citizen interdiction planning tool that analyzes trade routes, predicts high-value targets at specific locations, and helps pirates identify profitable opportunities. Features a TUI dashboard for interactive analysis, REST API for integrations, and PostgreSQL data pipeline for game data.

## Core Value

**Identify where valuable targets are and what they're likely carrying** - so interdictors can position at the right place with the right intel.

## Requirements

### Validated

- Trade route analysis (hot routes, profit calculation) — existing
- Target prediction at locations (arriving/departing classification) — existing
- Ship registry with cargo capacity and fuel estimation — existing
- TUI dashboard with map, targets, and hotspots views — existing
- REST API server with route and analysis endpoints — existing
- PostgreSQL medallion architecture (raw/silver/gold) data pipeline — existing
- External API integration (UEX, FleetYards, SC API) — existing
- Chokepoint analysis for route intersections — existing

### Active

- [ ] Wikelo item source mapping (which locations produce which Wikelo contract items)
- [ ] Source location flagging (flag ships leaving locations that produce high-value Wikelo items)
- [ ] TUI integration for Wikelo intel (show Wikelo-relevant data in existing views)
- [ ] Static item→source data (offline-first, no live lookups required)

### Out of Scope

- Live UEX price lookup for Wikelo items — focus on location intel first, pricing can come later
- Full contract tracking — don't need to track which specific Wikelo contract a player is working toward
- Automated alerts/notifications — manual analysis via TUI is sufficient for v1

## Context

**Wikelo System:**
Wikelo is a Banu trader who accepts specific items in exchange for rewards (weapons, armor, ships, currency). Players collect items from various sources and bring them to Wikelo Emporium stations. This creates two interdiction opportunities:

1. **Source intel**: Anyone leaving a location that produces Wikelo items is likely carrying valuable cargo (e.g., Lazarus Transport Centers in Pyro → Irradiated Valakkar parts)

2. **Destination intel**: Routes TO Wikelo stations are high-value because players are confirmed carrying collectible items

**Key Wikelo Input Items:**
- Creature parts: Irradiated Valakkar Fang/Pearl, Kopion Horn, Yormandi Eye/Tongue
- Mining materials: Carinite, Quantanium, Jaclium, Saldynium
- Mission rewards: MG Scrip, Council Scrip, various medals and badges
- Combat loot: Vanduul Plating/Metal, artifact fragments
- Equipment: ASD Secure Drive, DCHS-05 Comp-Board

**Data Sources:**
- wikelotrades.com — community tracker with all contracts and requirements
- starcitizen.tools — wiki with item source locations
- Existing UEX/FleetYards APIs — for trade route and ship data

## Constraints

- **Offline-first**: Item source mapping must be static/compiled, not require live lookups
- **TUI integration**: Must integrate into existing dashboard, not be a separate tool
- **Existing stack**: Use current Rust crates and architecture patterns

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Source location flagging as core feature | Most actionable intel - know where to wait for targets | — Pending |
| Static item→source mapping | Reliability over freshness; game data changes slowly | — Pending |
| Skip live pricing | Simplifies v1; location intel is more valuable than exact prices | — Pending |

---
*Last updated: 2026-01-14 after initialization*
