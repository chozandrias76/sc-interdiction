# Roadmap: SC Interdiction — Wikelo Intel

## Overview

Add Wikelo item source intelligence to the interdiction planning tool. Players collecting items for Wikelo contracts are high-value targets - knowing where items come from lets us flag ships leaving those locations. This builds on the existing trade route analysis with a new intelligence vector based on collectible item sources.

## Domain Expertise

None

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

- [x] **Phase 1: Wikelo Data Model** - Define data structures for items, sources, contracts
- [x] **Phase 2: Item Source Research** - Research and compile item→source mappings from wiki
- [ ] **Phase 2.1: Game Data Extraction** - INSERTED: Extract authoritative data from Data.p4k
- [x] **Phase 3: Wikelo Data Module** - Create crate with static Wikelo item/source data
- [x] **Phase 4: Source Intel Integration** - Integrate source flagging into intel crate
- [x] **Phase 5: TUI Wikelo Views** - Add Wikelo intel display to dashboard
- [x] **Phase 5.1: TUI Snapshot Test Coverage** - INSERTED: Comprehensive visual regression tests for Wikelo features
- [ ] **Phase 5.2: Coverage to 80%** - INSERTED: Reach CI coverage threshold
- [ ] **Phase 6: Testing & Polish** - Tests, edge cases, documentation

## Phase Details

### Phase 1: Wikelo Data Model
**Goal**: Define Rust types for Wikelo items, source locations, and contracts
**Depends on**: Nothing (first phase)
**Research**: Unlikely (internal data modeling)
**Plans**: 2 plans

Plans:
- [x] 01-01: Define core types (WikieloItem, ItemSource, ItemCategory)
- [x] 01-02: Define contract types (WikieloContract, ContractRequirement)

### Phase 2: Item Source Research
**Goal**: Research and document where each Wikelo input item comes from
**Depends on**: Phase 1 (need types to structure findings)
**Research**: Complete (02-RESEARCH.md created via /gsd:research-phase)
**Research topics**: starcitizen.tools item pages, creature spawn locations, loot tables, mining locations in Pyro/Stanton
**Plans**: 1 plan (consolidated after research completion)

Plans:
- [x] 02-01: Verify research and create Phase 3-ready data reference

### Phase 2.1: Game Data Extraction (INSERTED)
**Goal**: Build pipeline to extract authoritative item/contract data from Data.p4k
**Depends on**: Phase 2 (understanding what data we need)
**Research**: Complete (02.1-RESEARCH.md)
**Research topics**: Data.p4k format, DataForge DCB parsing, scdatatools, extraction caching
**Plans**: 3 plans

Plans:
- [x] 02.1-01: Set up scdatatools and extract Game2.dcb + global.ini
- [x] 02.1-02: Build DataForge exploration CLI to find Wikelo contracts
- [x] 02.1-03: Create extraction pipeline with caching for sc-data-extractor

### Phase 3: Wikelo Data Module
**Goal**: Create wikelo-data crate with static compiled item/source data
**Depends on**: Phase 2.1 (need authoritative data from game files)
**Research**: Unlikely (standard crate creation)
**Plans**: 2 plans
**Status**: Complete

Plans:
- [x] 03-01: Create crate structure with item registry
- [x] 03-02: Populate static data from research, add lookup functions

### Phase 4: Source Intel Integration
**Goal**: Integrate Wikelo source flagging into intel crate's target analysis
**Depends on**: Phase 3 (need data module)
**Research**: Unlikely (extends existing patterns in crates/intel)
**Plans**: 3 plans

Plans:
- [x] 04-01: Add WikieloIntel trait/struct to intel crate
- [x] 04-02: Integrate source location flagging into TargetAnalyzer
- [x] 04-03: Add Wikelo scoring to route/target calculations

### Phase 5: TUI Wikelo Views
**Goal**: Display Wikelo intel in existing TUI dashboard views
**Depends on**: Phase 4 (need intel integration)
**Research**: Unlikely (follows existing TUI patterns)
**Plans**: 3 plans

Plans:
- [x] 05-01: Add Wikelo column/indicator to targets view
- [x] 05-02: Add source location highlighting to map view
- [x] 05-03: Add Wikelo detail panel or hotspot enhancement

### Phase 5.1: TUI Snapshot Test Coverage (INSERTED)
**Goal**: Add comprehensive visual regression and interaction tests for TUI
**Depends on**: Phase 5 (need Wikelo TUI features to test)
**Research**: Unlikely (standard testing)
**Plans**: 4 plans

Plans:
- [x] 05.1-01: Add test fixtures with wikelo_flag data for realistic snapshots
- [x] 05.1-02: Add Wikelo detail panel expanded state snapshots
- [x] 05.1-03: Add key handler unit tests for navigation and toggles
- [x] 05.1-04: Add map and hotspot view snapshots

### Phase 5.2: Coverage to 80% (INSERTED)
**Goal**: Reach 80% code coverage threshold for CI
**Depends on**: Phase 5.1 (TUI snapshot tests provide foundation)
**Research**: Unlikely (standard testing)
**Plans**: 6 plans

Current coverage: 54.30%
Target: 80%

Key gaps addressed:
- `spatial.rs`: 27/210 → comprehensive unit tests (Plans 01, 03)
- `fuel.rs`: 23/98 → calculation tests (Plan 01)
- `ships/registry.rs`: 14/62 → lookup tests (Plan 02)
- `wikelo/contracts.rs`: 0/16 → helper tests (Plan 02)
- `ships/types.rs`: 27/47 → method tests (Plan 04)
- `graph.rs`: 56/77 → graph operation tests (Plan 05)
- `refinery.rs`: 25/41 → index tests (Plan 05)
- `uex.rs`: 48/108 → mock API tests (Plan 06)

Plans:
- [ ] 05.2-01: route-graph spatial.rs + fuel.rs unit tests
- [ ] 05.2-02: intel ships/registry.rs + wikelo/contracts.rs unit tests
- [ ] 05.2-03: route-graph spatial helper + intersection tests
- [ ] 05.2-04: intel ships/types.rs + wikelo/types.rs unit tests
- [ ] 05.2-05: route-graph graph.rs + refinery.rs unit tests
- [ ] 05.2-06: api-client get_trade_routes mock tests

### Phase 6: Testing & Polish
**Goal**: Comprehensive tests, edge cases, and documentation
**Depends on**: Phase 5.2 (need coverage threshold met)
**Research**: Unlikely (standard testing)
**Plans**: 2 plans

Plans:
- [ ] 06-01: Unit tests for data module and intel integration
- [ ] 06-02: Integration tests, edge cases, inline documentation

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 2.1 → 3 → 4 → 5 → 5.1 → 5.2 → 6

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Wikelo Data Model | 2/2 | Complete | 2026-01-15 |
| 2. Item Source Research | 1/1 | Complete | 2026-01-15 |
| 2.1. Game Data Extraction | 3/3 | Complete | 2026-01-17 |
| 3. Wikelo Data Module | 2/2 | Complete | 2026-01-18 |
| 4. Source Intel Integration | 3/3 | Complete | 2026-01-20 |
| 5. TUI Wikelo Views | 3/3 | Complete | 2026-01-21 |
| 5.1. TUI Snapshot Test Coverage | 4/4 | Complete | 2026-01-22 |
| 5.2. Coverage to 80% | 0/6 | Planned | - |
| 5.2. Coverage to 80% | 0/? | Not started | - |
| 6. Testing & Polish | 0/2 | Not started | - |
