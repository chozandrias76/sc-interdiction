# Demand Modeling Research Plan

## Objective

Research contract/demand systems in Star Citizen to model the full item economy:
- **Supply** (sources) - already modeled in wikelo-data
- **Demand** (contracts/NPCs who want items) - needs research

## Research Areas

### 1. Internal Codebase Audit

**What exists:**
- `WikieloContract`, `ContractRequirement`, `ContractReward` types (empty)
- 31 items with sources in wikelo-data
- `ItemCategory` enum: CreaturePart, MinedMaterial, MissionCurrency, CombatLoot, Equipment, Commodity

**Gaps to fill:**
- Contract instances (Wikelo trades)
- Other demand sources (mission givers, factions)
- Exchange rates (50 MG Scrip → 1 Wikelo Favor)

### 2. External Data Sources

#### Wikelo Trades (Primary)
- **wikelotrades.com** - Community tracker
  - What items Wikelo accepts
  - What rewards are available
  - Exchange rates and quantities
  - Contract requirements

#### Mission/Contract Systems
- **starcitizen.tools/wiki** - Mission documentation
  - Mission givers and their requirements
  - Faction contracts (MG, Council, etc.)
  - Delivery missions requiring specific items

- **scunpacked-data** - Game files
  - Check for mission/contract definitions
  - Reward structures in game data

#### Faction Systems
- Mercenary Guild contracts → MG Scrip rewards
- Civilian Defense Force → Council Scrip
- Other factions with item-based economies

### 3. Data Model Questions

1. **Is an item always both tradeable AND a potential reward?**
   - Can you trade Kopion Horns for something AND get them as a reward?

2. **Exchange rates**
   - Fixed or variable?
   - 50 MG Scrip = 1 Favor (known)
   - What other exchange chains exist?

3. **Contract repeatability**
   - One-time vs repeatable
   - Cooldowns?
   - Availability conditions?

4. **Location constraints**
   - Where to turn in items (Wikelo Emporium stations)
   - Mission giver locations

## Research Outputs

### Phase 1: Wikelo Contract Data
- [ ] Scrape/document all Wikelo contracts from wikelotrades.com
- [ ] Map: Item → Quantity → Reward for each contract
- [ ] Identify Wikelo Emporium locations (turn-in points)

### Phase 2: Other Demand Sources
- [ ] Document MG contract structure (missions → scrip → rewards)
- [ ] Document Council/CDF contracts
- [ ] Identify other NPCs/systems that accept items

### Phase 3: Game Data Validation
- [ ] Check scunpacked-data for contract definitions
- [ ] Cross-reference wiki data with game files
- [ ] Flag low-confidence data

## Proposed Data Model

```rust
// Generalized demand source (not just Wikelo)
pub struct ItemDemand {
    pub item_id: String,
    pub consumer: DemandSource,
    pub quantity_required: u32,
    pub reward: Option<Reward>,
    pub turn_in_location: String,
    pub repeatable: bool,
}

pub enum DemandSource {
    Wikelo { contract_id: String },
    MercenaryGuild { mission_type: String },
    CivilianDefense { mission_type: String },
    TradeTerminal { terminal_id: String },
    Other { name: String },
}

pub struct Reward {
    pub name: String,
    pub reward_type: RewardType,
    pub estimated_value: Option<u64>,
}
```

## Success Criteria

Research is complete when we can answer:
1. "Who wants Kopion Horns and what do they give for them?"
2. "What's the full chain from MG mission → MG Scrip → Wikelo Favor → Reward?"
3. "Which locations are demand hotspots (turn-in points)?"
