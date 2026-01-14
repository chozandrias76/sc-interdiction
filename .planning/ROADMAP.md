# Roadmap: SC Interdiction — Wikelo Intel

## Overview

Add Wikelo item source intelligence to the interdiction planning tool. Players collecting items for Wikelo contracts are high-value targets - knowing where items come from lets us flag ships leaving those locations. This builds on the existing trade route analysis with a new intelligence vector based on collectible item sources.

## Domain Expertise

None

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

- [ ] **Phase 1: Wikelo Data Model** - Define data structures for items, sources, contracts
- [ ] **Phase 2: Item Source Research** - Research and compile item→source mappings from wiki
- [ ] **Phase 3: Wikelo Data Module** - Create crate with static Wikelo item/source data
- [ ] **Phase 4: Source Intel Integration** - Integrate source flagging into intel crate
- [ ] **Phase 5: TUI Wikelo Views** - Add Wikelo intel display to dashboard
- [ ] **Phase 6: Testing & Polish** - Tests, edge cases, documentation

## Phase Details

### Phase 1: Wikelo Data Model
**Goal**: Define Rust types for Wikelo items, source locations, and contracts
**Depends on**: Nothing (first phase)
**Research**: Unlikely (internal data modeling)
**Plans**: 2 plans

Plans:
- [ ] 01-01: Define core types (WikieloItem, ItemSource, ItemCategory)
- [ ] 01-02: Define contract types (WikieloContract, ContractRequirement)

### Phase 2: Item Source Research
**Goal**: Research and document where each Wikelo input item comes from
**Depends on**: Phase 1 (need types to structure findings)
**Research**: Likely (external wiki research)
**Research topics**: starcitizen.tools item pages, creature spawn locations, loot tables, mining locations in Pyro/Stanton
**Plans**: 3 plans

Plans:
- [ ] 02-01: Research creature/biological items (Valakkar, Kopion, Yormandi, Grazer)
- [ ] 02-02: Research mining/material items (Carinite, ores, SCU commodities)
- [ ] 02-03: Research mission/loot items (Scrip, medals, drives, artifacts)

### Phase 3: Wikelo Data Module
**Goal**: Create wikelo-data crate with static compiled item/source data
**Depends on**: Phase 2 (need researched data to compile)
**Research**: Unlikely (standard crate creation)
**Plans**: 2 plans

Plans:
- [ ] 03-01: Create crate structure with item registry
- [ ] 03-02: Populate static data from research, add lookup functions

### Phase 4: Source Intel Integration
**Goal**: Integrate Wikelo source flagging into intel crate's target analysis
**Depends on**: Phase 3 (need data module)
**Research**: Unlikely (extends existing patterns in crates/intel)
**Plans**: 3 plans

Plans:
- [ ] 04-01: Add WikieloIntel trait/struct to intel crate
- [ ] 04-02: Integrate source location flagging into TargetAnalyzer
- [ ] 04-03: Add Wikelo scoring to route/target calculations

### Phase 5: TUI Wikelo Views
**Goal**: Display Wikelo intel in existing TUI dashboard views
**Depends on**: Phase 4 (need intel integration)
**Research**: Unlikely (follows existing TUI patterns)
**Plans**: 3 plans

Plans:
- [ ] 05-01: Add Wikelo column/indicator to targets view
- [ ] 05-02: Add source location highlighting to map view
- [ ] 05-03: Add Wikelo detail panel or hotspot enhancement

### Phase 6: Testing & Polish
**Goal**: Comprehensive tests, edge cases, and documentation
**Depends on**: Phase 5 (need all features implemented)
**Research**: Unlikely (standard testing)
**Plans**: 2 plans

Plans:
- [ ] 06-01: Unit tests for data module and intel integration
- [ ] 06-02: Integration tests, edge cases, inline documentation

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Wikelo Data Model | 0/2 | Not started | - |
| 2. Item Source Research | 0/3 | Not started | - |
| 3. Wikelo Data Module | 0/2 | Not started | - |
| 4. Source Intel Integration | 0/3 | Not started | - |
| 5. TUI Wikelo Views | 0/3 | Not started | - |
| 6. Testing & Polish | 0/2 | Not started | - |
