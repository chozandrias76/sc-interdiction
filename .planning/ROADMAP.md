# Roadmap: SC Interdiction — Wikelo Intel

## Overview

Add Wikelo item source intelligence to the interdiction planning tool. Players collecting items for Wikelo contracts are high-value targets - knowing where items come from lets us flag ships leaving those locations. This builds on the existing trade route analysis with a new intelligence vector based on collectible item sources.

## Domain Expertise

None

## Milestones

- ✅ **v1.0 Wikelo Intel** - Phases 1-6 (shipped 2026-01-27)
- 🚧 **v1.1 Demand Modeling** - Phases 7-12 (in progress)

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

<details>
<summary>✅ v1.0 Wikelo Intel (Phases 1-6) - SHIPPED 2026-01-27</summary>

- [x] **Phase 1: Wikelo Data Model** - Define data structures for items, sources, contracts
- [x] **Phase 2: Item Source Research** - Research and compile item→source mappings from wiki
- [x] **Phase 2.1: Game Data Extraction** - INSERTED: Extract authoritative data from Data.p4k
- [x] **Phase 3: Wikelo Data Module** - Create crate with static Wikelo item/source data
- [x] **Phase 4: Source Intel Integration** - Integrate source flagging into intel crate
- [x] **Phase 5: TUI Wikelo Views** - Add Wikelo intel display to dashboard
- [x] **Phase 5.1: TUI Snapshot Test Coverage** - INSERTED: Comprehensive visual regression tests
- [x] **Phase 5.2: Coverage to 80%** - INSERTED: Reach CI coverage threshold
- [x] **Phase 6: Testing & Polish** - Tests, edge cases, documentation

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
**Plans**: 8 plans

Plans:
- [x] 05.2-01: route-graph spatial.rs + fuel.rs unit tests
- [x] 05.2-02: intel ships/registry.rs + wikelo/contracts.rs unit tests
- [x] 05.2-03: route-graph spatial helper + intersection tests
- [x] 05.2-04: intel ships/types.rs + wikelo/types.rs unit tests
- [x] 05.2-05: route-graph graph.rs + refinery.rs unit tests
- [x] 05.2-06: api-client get_trade_routes mock tests
- [x] 05.2-07: intel targets.rs LocationAggregator + helper tests
- [x] 05.2-08: api-client sc_api.rs mock tests + remaining gaps

### Phase 6: Testing & Polish
**Goal**: Comprehensive tests, edge cases, and documentation
**Depends on**: Phase 5.2 (need coverage threshold met)
**Research**: Unlikely (standard testing)
**Plans**: 2 plans

Plans:
- [x] 06-01: TargetAnalyzer integration tests with mock fixtures
- [x] 06-02: Integration tests, edge cases, inline documentation

</details>

### 🚧 v1.1 Demand Modeling (In Progress)

**Milestone Goal:** Add contract/demand tracking to complement existing item sources. Model the full item economy: where items come from (supply, already done) AND who wants them (demand, new).

- [x] **Phase 7: Demand Research** - Research Wikelo contracts and other demand sources
- [x] **Phase 8: Demand Data Model** - Extend types for demand/contracts
- [x] **Phase 9: Contract Data Population** - Populate Wikelo contract instances
- [x] **Phase 10: Demand Registry** - Create contract registry with indexing
- [ ] **Phase 11: Demand Intel Integration** - Add demand scoring to TargetAnalyzer
- [ ] **Phase 12: CLI/TUI Demand Views** - Add item lookup and demand display

#### Phase 7: Demand Research
**Goal**: Research Wikelo contracts, MG/Council systems, and other demand sources
**Depends on**: v1.0 complete (builds on existing item data)
**Research**: Likely (external data gathering)
**Research topics**: wikelotrades.com contracts, mission giver requirements, faction reward systems, exchange rates (50 MG Scrip → 1 Favor)
**Plans**: TBD

Plans:
- [x] 07-01: Research and document all 43 Wikelo contracts and demand ecosystem

#### Phase 8: Demand Data Model
**Goal**: Extend type system for demand sources and contracts
**Depends on**: Phase 7 (need research results)
**Research**: Unlikely (internal modeling)
**Plans**: TBD

Plans:
- [x] 08-01: Add demand-side types and extend WikieloContract/RewardType

#### Phase 9: Contract Data Population
**Goal**: Populate WikieloContract instances with researched data
**Depends on**: Phase 8 (need types defined)
**Research**: Unlikely (data entry from research)
**Plans**: TBD

Plans:
- [x] 09-01: Create contract_data module and populate 14 high-confidence contracts

#### Phase 10: Demand Registry
**Goal**: Create ContractRegistry with bidirectional indexing (item→contracts, contract→items)
**Depends on**: Phase 9 (need contract data)
**Research**: Unlikely (follows WikieloRegistry pattern)
**Plans**: TBD

Plans:
- [x] 10-01: Create ContractRegistry with bidirectional indexes and unit tests

#### Phase 11: Demand Intel Integration
**Goal**: Add demand scoring to TargetAnalyzer - flag ships heading TO demand locations
**Depends on**: Phase 10 (need registry)
**Research**: Unlikely (extends existing patterns)
**Plans**: TBD

Plans:
- [ ] 11-01: TBD

#### Phase 12: CLI/TUI Demand Views
**Goal**: Add item reverse lookup CLI and demand display in TUI
**Depends on**: Phase 11 (need intel integration)
**Research**: Unlikely (follows existing patterns)
**Plans**: TBD

Plans:
- [ ] 12-01: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 2.1 → 3 → 4 → 5 → 5.1 → 5.2 → 6 → 7 → ...

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Wikelo Data Model | v1.0 | 2/2 | Complete | 2026-01-15 |
| 2. Item Source Research | v1.0 | 1/1 | Complete | 2026-01-15 |
| 2.1. Game Data Extraction | v1.0 | 3/3 | Complete | 2026-01-17 |
| 3. Wikelo Data Module | v1.0 | 2/2 | Complete | 2026-01-18 |
| 4. Source Intel Integration | v1.0 | 3/3 | Complete | 2026-01-20 |
| 5. TUI Wikelo Views | v1.0 | 3/3 | Complete | 2026-01-21 |
| 5.1. TUI Snapshot Test Coverage | v1.0 | 4/4 | Complete | 2026-01-22 |
| 5.2. Coverage to 80% | v1.0 | 8/8 | Complete | 2026-01-23 |
| 6. Testing & Polish | v1.0 | 2/2 | Complete | 2026-01-27 |
| 7. Demand Research | v1.1 | 1/1 | Complete | 2026-01-29 |
| 8. Demand Data Model | v1.1 | 1/1 | Complete | 2026-02-01 |
| 9. Contract Data Population | v1.1 | 1/1 | Complete | 2026-02-01 |
| 10. Demand Registry | v1.1 | 1/1 | Complete | 2026-02-02 |
| 11. Demand Intel Integration | v1.1 | 0/? | Not started | - |
| 12. CLI/TUI Demand Views | v1.1 | 0/? | Not started | - |
