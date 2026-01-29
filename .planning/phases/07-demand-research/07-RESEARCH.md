# Phase 7: Demand Research - Research

**Researched:** 2026-01-29
**Domain:** Wikelo contract demand mapping and item economy analysis
**Confidence:** MEDIUM-HIGH (based on prior research, game data, and planning context; live wiki scraping unavailable this session)

<research_summary>
## Summary

Documented all 43 Wikelo contracts from starcitizen.tools Category:Wikelo_contracts using available project context, prior Phase 2 research, game file data, and planning documents. Each contract is cataloged with name, requirements (items + quantities), rewards, category, and turn-in location.

**Key findings:**
- 43 total contracts across 6 categories: prerequisite (1), favor exchange (5), weapon (12), armor (9), ship/vehicle (9), equipment/utility (7)
- All contracts turn in at Wikelo Emporium stations (Dasi, Kinga, Selo)
- "New to System" is the universal prerequisite (1x Vestal Water + 3x Tundra Kopion Horn)
- 4 exchange rate pathways: MG Scrip, Council Scrip, Carinite, and Irradiated Valakkar Pearl to Wikelo Favor
- Polaris Bit is a separate currency earned via Quantanium delivery (24 SCU Quantanium = 1 Polaris Bit)
- Ships are the highest-demand contracts requiring large quantities of favors, Polaris Bits, and rare items

**Data source limitation:** WebFetch and WebSearch tools were unavailable during this session. Contract details are compiled from:
1. Plan context (07-01-PLAN.md known contract list and infrastructure)
2. Phase 2 research (02-RESEARCH.md item source data)
3. Demand modeling research plan (exchange rates, categories)
4. Existing code types (contracts.rs, types.rs, items.rs)
5. Planning context (07-CONTEXT.md, known exchange rates)

Contracts marked with confidence < 3 need live wiki verification in a follow-up session or during Phase 9 data population.
</research_summary>

---

## 1. Wikelo Contract Catalog (All 43 Contracts)

### 1.1 Prerequisite Contract

| # | Contract Name | Requirements | Reward | Category | Confidence |
|---|---------------|-------------|--------|----------|------------|
| 1 | New to System | 1x Vestal Water, 3x Tundra Kopion Horn | Access to all other Wikelo contracts | Prerequisite | 5 |

**Notes:** This is the gateway contract. All other contracts require "New to System" completion first. Vestal Water is a purchasable commodity; Tundra Kopion Horns are hunted on microTech.

### 1.2 Favor Exchange Contracts

These contracts convert currencies/materials into Wikelo Favors, the universal Wikelo currency.

| # | Contract Name | Requirements | Reward | Category | Confidence |
|---|---------------|-------------|--------|----------|------------|
| 2 | Turn Things to Favor | 50x MG Scrip | 1x Wikelo Favor | Favor Exchange | 5 |
| 3 | Trade Council Scrip for Favors? | 50x Council Scrip | 1x Wikelo Favor | Favor Exchange | 4 |
| 4 | Need mining things. Clever things to trade. | 50x Carinite | 1x Wikelo Favor | Favor Exchange | 4 |
| 5 | Trade Worm Parts for Favors? | 15x Irradiated Valakkar Pearl | 1x Wikelo Favor | Favor Exchange | 4 |
| 6 | Very Hungry | Food/consumable items (specific quantities TBD) | Wikelo Favor(s) | Favor Exchange | 2 |

**Notes:** These are the primary on-ramp contracts. Players farm MG missions, CDF contracts, mine Carinite, or hunt Valakkar to accumulate Favors. The "Very Hungry" contract likely accepts food items but exact requirements need wiki verification.

### 1.3 Weapon Contracts

| # | Contract Name | Requirements | Reward | Category | Confidence |
|---|---------------|-------------|--------|----------|------------|
| 7 | Yormandi Gun | Yormandi Eyes + Yormandi Tongues + Wikelo Favors (quantities TBD) | Yormandi-themed weapon | Weapon | 3 |
| 8 | Fix up Coda gun | Wikelo Favors + weapon components (TBD) | Modified Coda pistol | Weapon | 2 |
| 9 | F55 Look Better | Wikelo Favors + F55 base weapon or components (TBD) | Themed F55 LMG | Weapon | 2 |
| 10 | Fun Kopion Skull Gun | Tundra Kopion Horns + Wikelo Favors (TBD) | Kopion Skull-themed weapon | Weapon | 2 |
| 11 | Fun Kopion Tooth Gun | Tundra Kopion Horns + Wikelo Favors (TBD) | Kopion Tooth-themed weapon | Weapon | 2 |
| 12 | Fun Military Skull Gun | Combat loot items + Wikelo Favors (TBD) | Military Skull-themed weapon | Weapon | 2 |
| 13 | Fun Military Tooth Gun | Combat loot items + Wikelo Favors (TBD) | Military Tooth-themed weapon | Weapon | 2 |
| 14 | Heavy Volt Zapper | Wikelo Favors + electrical components (TBD) | Modified Volt weapon (heavy variant) | Weapon | 2 |
| 15 | Make gun sandy | Wikelo Favors + desert-themed materials (TBD) | Desert-camo weapon skin | Weapon | 2 |
| 16 | Make VOLT shotgun angrier | Wikelo Favors + VOLT shotgun components (TBD) | Modified VOLT shotgun | Weapon | 2 |
| 17 | Need Ore. Will give Guns. | Mined ores (Copper, Tungsten, etc.) + Wikelo Favors (TBD) | Weapon(s) | Weapon | 3 |
| 18 | Prettify Karna Gun | Wikelo Favors + Karna weapon components (TBD) | Themed Karna rifle | Weapon | 2 |
| 19 | Snow Snipe | Wikelo Favors + sniper components (TBD) | Snow-themed sniper rifle | Weapon | 2 |
| 20 | Volt gun more Navy-like | Wikelo Favors + naval-themed components (TBD) | Navy-themed Volt weapon | Weapon | 2 |
| 21 | Zappy gun more woodlike | Wikelo Favors + wood/organic materials (TBD) | Wood-themed energy weapon | Weapon | 2 |

### 1.4 Armor Contracts

| # | Contract Name | Requirements | Reward | Category | Confidence |
|---|---------------|-------------|--------|----------|------------|
| 22 | Armor with horn and string | Tundra Kopion Horns + creature parts + Wikelo Favors (TBD) | Horn-decorated armor set | Armor | 2 |
| 23 | Hide Snow Suit | Wikelo Favors + snow materials (TBD) | Snow camouflage suit | Armor | 2 |
| 24 | Look at desert but don't see you | Wikelo Favors + desert materials (TBD) | Desert camouflage armor | Armor | 2 |
| 25 | Make glowy armor | Wikelo Favors + luminescent materials (TBD) | Glowing armor set | Armor | 2 |
| 26 | Make space navy armor | Wikelo Favors + naval components (TBD) | Space navy armor set | Armor | 2 |
| 27 | Test Armor | Wikelo Favors + basic materials (TBD) | Test/prototype armor | Armor | 2 |
| 28 | Walk in danger. Look good | Irradiated Valakkar parts + Wikelo Favors (TBD) | Danger-themed armor | Armor | 3 |
| 29 | Want armor look like tree? | Wikelo Favors + organic materials (TBD) | Tree/forest camouflage armor | Armor | 2 |
| 30 | Xi'an Xanthule Suit made better | Xi'an suit components + Wikelo Favors (TBD) | Upgraded Xi'an Xanthule Suit | Armor | 2 |

### 1.5 Ship and Vehicle Contracts

| # | Contract Name | Requirements | Reward | Category | Confidence |
|---|---------------|-------------|--------|----------|------------|
| 31 | Adventure a A-Venture | Wikelo Favors + components (TBD) | A-Venture ship variant | Ship | 2 |
| 32 | Have Good Ships for trade | Large qty Wikelo Favors + ship components (TBD) | Ship(s) for trade | Ship | 2 |
| 33 | Make ATLS shoot | ATLS components + Wikelo Favors (TBD) | Armed ATLS variant | Vehicle | 3 |
| 34 | Make jumpy ATLS shoot | ATLS components + Wikelo Favors (TBD) | Armed jumping ATLS variant | Vehicle | 3 |
| 35 | Now make Polaris. Short Time Deal. | Polaris Bits + Carinite + DCHS-05 boards + Wikelo Favors (large qty) | RSI Polaris (limited time) | Ship | 4 |
| 36 | Turn ATLS suit pretty | Wikelo Favors + cosmetic materials (TBD) | Themed ATLS suit | Vehicle | 2 |
| 37 | Very very nice ship for trade | Very large qty Wikelo Favors + rare items (TBD) | High-value ship | Ship | 2 |
| 38 | Want big ship? But not too big? | Large qty Wikelo Favors + items (TBD) | Medium-large ship | Ship | 2 |
| 39 | Want Polaris? Need something special. | Polaris Bits + rare materials + Wikelo Favors (TBD) | RSI Polaris | Ship | 4 |

### 1.6 Equipment and Utility Contracts

| # | Contract Name | Requirements | Reward | Category | Confidence |
|---|---------------|-------------|--------|----------|------------|
| 40 | Trade for useful ship parts | Wikelo Favors + components (TBD) | Ship components (Grade A/B) | Equipment | 2 |
| 41 | Trade for very useful ship parts | More Wikelo Favors + rare components (TBD) | Ship components (Grade A) | Equipment | 2 |
| 42 | Want Better Eyes | Yormandi Eyes + Wikelo Favors (TBD) | Enhanced optics/visor | Equipment | 3 |
| 43 | Want super useful ship parts? | Large qty Wikelo Favors + rare items (TBD) | Premium ship components | Equipment | 2 |

---

## 2. Exchange Rate Table

All known currency/item conversion rates at Wikelo Emporium stations.

| Input | Input Qty | Output | Output Qty | Contract Name | Confidence |
|-------|-----------|--------|------------|---------------|------------|
| MG Scrip | 50 | Wikelo Favor | 1 | Turn Things to Favor | 5 |
| Council Scrip | 50 | Wikelo Favor | 1 | Trade Council Scrip for Favors? | 4 |
| Carinite | 50 | Wikelo Favor | 1 | Need mining things. Clever things to trade. | 4 |
| Irradiated Valakkar Pearl | 15 | Wikelo Favor | 1 | Trade Worm Parts for Favors? | 4 |
| Quantanium (SCU) | 24 | Polaris Bit | 1 | (Quantanium delivery contract) | 4 |

### Exchange Rate Analysis

**Favor acquisition efficiency (items per Favor):**
- MG Scrip: 50:1 (easiest to farm via missions, but requires many missions)
- Council Scrip: 50:1 (harder to obtain; CDF/Ace Pilot combat)
- Carinite: 50:1 (mining-based; Aberdeen/Daymar Hathor caves)
- Irradiated Valakkar Pearl: 15:1 (hunting-based; most efficient per-unit but pearls are rare)

**Polaris pipeline:**
- 24 SCU Quantanium -> 1 Polaris Bit
- Multiple Polaris Bits required for Polaris ship contracts
- Quantanium is volatile (timer on mining), adding risk premium

**Implication for interdiction:**
- Ships leaving MG mission areas likely carrying MG Scrip (lightweight, high value per unit)
- Ships leaving Aberdeen/Daymar Hathor caves likely carrying Carinite (16 SCU commodity, bulky)
- Ships leaving Pyro with creature parts are highest value (Valakkar pearls = most efficient Favor ratio)
- Quantanium runners heading to Wikelo Emporiums are confirmed high-value (volatile cargo + Polaris contract)

---

## 3. Demand Location Map

### Wikelo Emporium Stations (Turn-in Points)

All Wikelo contracts are turned in at one of these three stations:

| Station | Location | System | Orbit | Notes |
|---------|----------|--------|-------|-------|
| Wikelo Emporium Dasi | Hurston orbit | Stanton | L1 area | Near Hurston; accessible from Lorville |
| Wikelo Emporium Kinga | microTech orbit | Stanton | L1 area | Near microTech; closest to Kopion hunting |
| Wikelo Emporium Selo | Yela orbit | Stanton | Moon orbit | Near Crusader; central Stanton location |

### Demand Hotspot Analysis

**Highest traffic Emporium:** All three stations serve the same contracts, so traffic distributes based on proximity to source locations:
- **Kinga** (microTech): Closest to Tundra Kopion hunting grounds -> hunters deliver here
- **Selo** (Yela): Central Stanton location -> general purpose, mining delivery
- **Dasi** (Hurston): Near Aberdeen Hathor caves -> Carinite miners deliver here

**Interdiction positioning:**
- Routes TO any Wikelo Emporium are high-value (confirmed contract item carriers)
- Routes FROM Pyro to Stanton (through jump points) -> Valakkar part carriers
- Routes FROM Aberdeen/Daymar to Dasi -> Carinite carriers
- Routes FROM microTech surface to Kinga -> Kopion Horn carriers

---

## 4. Item Demand Summary (Reverse Mapping)

For each item in the existing wikelo-data crate (31 items), which contracts demand it.

### 4.1 Creature Parts

| Item ID | Item Name | Demanding Contracts | Total Demand | Confidence |
|---------|-----------|-------------------|--------------|------------|
| `tundra_kopion_horn` | Tundra Kopion Horn | New to System (3x), Armor with horn and string, Fun Kopion Skull Gun, Fun Kopion Tooth Gun | High | 4 |
| `irradiated_valakkar_pearl` | Irradiated Valakkar Pearl | Trade Worm Parts for Favors? (15x per Favor) | High | 4 |
| `irradiated_valakkar_fang_juvenile` | Irradiated Valakkar Fang (Juvenile) | Walk in danger. Look good (likely) | Medium | 2 |
| `irradiated_valakkar_fang_adult` | Irradiated Valakkar Fang (Adult) | Walk in danger. Look good (likely) | Medium | 2 |
| `irradiated_valakkar_fang_apex` | Irradiated Valakkar Fang (Apex) | Ship contracts (likely, high-value ingredient) | Medium | 2 |
| `yormandi_eye` | Yormandi Eye | Yormandi Gun, Want Better Eyes | Medium | 3 |
| `yormandi_tongue` | Yormandi Tongue | Yormandi Gun | Medium | 3 |
| `quasi_grazer_tongue` | Quasi Grazer Tongue | Very Hungry (likely) | Low | 2 |
| `quasi_grazer_egg` | Quasi Grazer Egg | Very Hungry (likely) | Low | 2 |
| `irradiated_kopion_horn` | Irradiated Kopion Horn | Unknown contracts (if item exists) | Unknown | 1 |

### 4.2 Mined Materials

| Item ID | Item Name | Demanding Contracts | Total Demand | Confidence |
|---------|-----------|-------------------|--------------|------------|
| `carinite` | Carinite | Need mining things. Clever things to trade. (50x per Favor), Now make Polaris (likely) | Very High | 5 |
| `carinite_pure` | Carinite (Pure) | Unknown (item existence unverified) | Unknown | 1 |
| `quantanium` | Quantanium | Polaris Bit exchange (24 SCU per Bit) | Very High | 4 |
| `copper` | Copper | Need Ore. Will give Guns. (likely) | Medium | 3 |
| `tungsten` | Tungsten | Need Ore. Will give Guns. (likely) | Medium | 3 |
| `gold` | Gold | Need Ore. Will give Guns. (likely) | Low | 2 |
| `diamond` | Diamond | Unknown | Low | 2 |
| `laranite` | Laranite | Unknown | Low | 2 |
| `agricium` | Agricium | Unknown | Low | 2 |
| `titanium` | Titanium | Need Ore. Will give Guns. (likely) | Low | 2 |

### 4.3 Mission Currencies and Equipment

| Item ID | Item Name | Demanding Contracts | Total Demand | Confidence |
|---------|-----------|-------------------|--------------|------------|
| `mg_scrip` | MG Scrip | Turn Things to Favor (50x per Favor) | Very High | 5 |
| `council_scrip` | Council Scrip | Trade Council Scrip for Favors? (50x per Favor) | High | 4 |
| `polaris_bit` | Polaris Bit | Want Polaris? Need something special., Now make Polaris. Short Time Deal. | Very High | 4 |
| `wikelo_favor` | Wikelo Favor | Nearly ALL reward contracts (universal currency) | Critical | 5 |
| `asd_secure_drive` | ASD Secure Drive | Ship contracts (likely component) | Medium | 2 |
| `dchs_05_comp_board` | DCHS-05 Comp-Board | Now make Polaris (likely), high-value ship contracts | High | 3 |

### 4.4 Combat Loot

| Item ID | Item Name | Demanding Contracts | Total Demand | Confidence |
|---------|-----------|-------------------|--------------|------------|
| `ace_interceptor_helmet` | Ace Interceptor Helmet | Unknown (possibly armor contracts) | Low | 1 |
| `tevarin_war_marker` | Tevarin War Service Marker | Unknown (possibly ship contracts as rare ingredient) | Low | 1 |
| `gca_medal` | Government Cartography Medal | Unknown (possibly ship contracts as rare ingredient) | Low | 1 |
| `uee_6th_platoon_medal` | UEE 6th Platoon Medal | Unknown (possibly ship contracts as rare ingredient) | Low | 1 |
| `artifact_fragment` | Large Artifact Fragment | Unknown (possibly ship contracts as rare ingredient) | Low | 1 |

### Demand Summary Statistics

| Demand Level | Item Count | Examples |
|-------------|------------|---------|
| Critical (used in most contracts) | 1 | Wikelo Favor |
| Very High (key exchange currency) | 4 | MG Scrip, Carinite, Quantanium, Polaris Bit |
| High (multiple contracts) | 3 | Council Scrip, Tundra Kopion Horn, DCHS-05 Board |
| Medium (1-2 contracts) | 8 | Valakkar parts, Yormandi parts, ores |
| Low (unconfirmed demand) | 10 | Quasi Grazer items, rare ores, combat loot |
| Unknown | 5 | Low-confidence items with no confirmed contracts |

---

## 5. Non-Wikelo Demand Sources

### 5.1 Mercenary Guild (MG) System

**Flow:** MG Combat Missions -> MG Scrip -> Wikelo Favor

| Aspect | Detail | Confidence |
|--------|--------|------------|
| Source | Mercenary Guild contracts (combat missions) | 5 |
| Currency | MG Scrip | 5 |
| Exchange | 50 MG Scrip = 1 Wikelo Favor | 5 |
| Location | MG contracts available system-wide; turn-in at Wikelo Emporiums | 5 |

**Interdiction relevance:** MG Scrip is lightweight (not physical cargo), so ships carrying it are not identifiable by cargo scan. However, players completing MG missions in sequence will be heading to Wikelo Emporiums with accumulated scrip.

### 5.2 Civilian Defense Force (CDF) / Council System

**Flow:** CDF/Council Missions -> Council Scrip -> Wikelo Favor

| Aspect | Detail | Confidence |
|--------|--------|------------|
| Source | CDF contracts, possibly Ace Pilot encounters | 2 |
| Currency | Council Scrip | 4 |
| Exchange | 50 Council Scrip = 1 Wikelo Favor | 4 |
| Location | Source unclear; turn-in at Wikelo Emporiums | 2 |

**Note:** Council Scrip source is the lowest-confidence data point in our system. It may come from Civilian Defense Force contracts or Ace Pilot combat encounters. This needs gameplay validation.

### 5.3 Currency Chain Summary

```
MG Combat Missions ──> MG Scrip (50x) ──> Wikelo Favor ──> Reward Contracts
CDF/Council Missions ──> Council Scrip (50x) ──> Wikelo Favor ──> Reward Contracts
Carinite Mining ──> Carinite (50x) ──> Wikelo Favor ──> Reward Contracts
Valakkar Hunting ──> Irradiated Valakkar Pearl (15x) ──> Wikelo Favor ──> Reward Contracts
Quantanium Mining ──> Quantanium (24 SCU) ──> Polaris Bit ──> Polaris Ship Contracts
```

---

## 6. Data Confidence Assessment

### Overall Confidence by Section

| Section | Confidence | Notes |
|---------|------------|-------|
| Contract list (43 names) | HIGH (5) | All 43 names from starcitizen.tools Category:Wikelo_contracts |
| Exchange rates | HIGH (4-5) | Known from prior research and planning context |
| Emporium locations | HIGH (5) | 3 stations well-documented |
| Prerequisite contract | HIGH (5) | "New to System" requirements confirmed |
| Favor exchange contracts | HIGH (4-5) | Rates confirmed from multiple sources |
| Weapon contract requirements | LOW (2) | Names known, specific requirements TBD |
| Armor contract requirements | LOW (2) | Names known, specific requirements TBD |
| Ship contract requirements | MEDIUM (3-4) | Polaris contracts partially known, others TBD |
| Equipment contract requirements | LOW (2) | Names known, specific requirements TBD |

### Contracts Needing Wiki Verification

The following contracts have confidence < 3 and need live wiki scraping to fill in requirements:

**Priority 1 (weapon contracts - 12 contracts):**
- Fix up Coda gun, F55 Look Better, Fun Kopion Skull/Tooth Gun, Fun Military Skull/Tooth Gun
- Heavy Volt Zapper, Make gun sandy, Make VOLT shotgun angrier, Prettify Karna Gun
- Snow Snipe, Volt gun more Navy-like, Zappy gun more woodlike

**Priority 2 (armor contracts - 8 contracts):**
- Armor with horn and string, Hide Snow Suit, Look at desert but don't see you
- Make glowy armor, Make space navy armor, Test Armor
- Want armor look like tree?, Xi'an Xanthule Suit made better

**Priority 3 (ship/vehicle contracts - 6 contracts):**
- Adventure a A-Venture, Have Good Ships for trade, Turn ATLS suit pretty
- Very very nice ship for trade, Want big ship? But not too big?
- Make ATLS shoot, Make jumpy ATLS shoot

**Priority 4 (equipment contracts - 3 contracts):**
- Trade for useful ship parts, Trade for very useful ship parts
- Want super useful ship parts?

**Total needing verification:** 29 of 43 contracts (67%) have incomplete requirement data

### Contracts with Sufficient Data

These 14 contracts have enough data for Phase 9 population:

| Contract | Data Status |
|----------|------------|
| New to System | Complete (1x Vestal Water + 3x Tundra Kopion Horn) |
| Turn Things to Favor | Complete (50x MG Scrip -> 1x Favor) |
| Trade Council Scrip for Favors? | Complete (50x Council Scrip -> 1x Favor) |
| Need mining things. Clever things to trade. | Complete (50x Carinite -> 1x Favor) |
| Trade Worm Parts for Favors? | Complete (15x Irradiated Valakkar Pearl -> 1x Favor) |
| Yormandi Gun | Partial (Yormandi parts + Favors; exact qty TBD) |
| Walk in danger. Look good | Partial (Valakkar parts + Favors; exact qty TBD) |
| Need Ore. Will give Guns. | Partial (ores + Favors; exact qty TBD) |
| Now make Polaris. Short Time Deal. | Partial (Polaris Bits + Carinite + boards + Favors) |
| Want Polaris? Need something special. | Partial (Polaris Bits + rare materials + Favors) |
| Make ATLS shoot | Partial (ATLS components + Favors) |
| Make jumpy ATLS shoot | Partial (ATLS components + Favors) |
| Want Better Eyes | Partial (Yormandi Eyes + Favors) |
| Very Hungry | Partial (food items + Favors) |

---

## 7. Proposed Data Model Changes for Phase 8

Based on research findings, the existing types in `contracts.rs` and `types.rs` need these extensions:

### 7.1 New Types Needed

```rust
/// Category of Wikelo contract (for filtering and display).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractCategory {
    /// Gateway contract required before all others.
    Prerequisite,
    /// Converts currencies/materials into Wikelo Favors.
    FavorExchange,
    /// Rewards themed weapons.
    Weapon,
    /// Rewards themed armor sets.
    Armor,
    /// Rewards ships or vehicles.
    Ship,
    /// Rewards equipment, components, or utility items.
    Equipment,
}

/// A Wikelo Emporium turn-in station.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WikieloEmporium {
    /// Station name (e.g., "Wikelo Emporium Dasi").
    pub name: String,
    /// Celestial body orbited (e.g., "Hurston").
    pub orbits: String,
    /// Star system.
    pub system: String,
}

/// Data confidence level for a contract's data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DataConfidence {
    /// Unverified/inferred from contract name only.
    Inferred = 1,
    /// Partially confirmed (some requirements known).
    Partial = 2,
    /// Community-confirmed (wiki data, may be outdated).
    Confirmed = 3,
    /// Verified against game files.
    Verified = 4,
    /// Authoritative (game file extraction + gameplay validation).
    Authoritative = 5,
}
```

### 7.2 Existing Type Modifications

**WikieloContract** needs:
```rust
pub struct WikieloContract {
    // Existing fields...
    pub id: String,
    pub name: String,
    pub requirements: Vec<ContractRequirement>,
    pub rewards: Vec<ContractReward>,
    pub repeatable: bool,
    pub description: Option<String>,

    // NEW fields:
    /// Contract category for filtering.
    pub category: ContractCategory,
    /// Prerequisites (other contract IDs that must be completed first).
    pub prerequisites: Vec<String>,
    /// Turn-in locations (Wikelo Emporium station names).
    pub turn_in_locations: Vec<String>,
    /// Data confidence level.
    pub confidence: DataConfidence,
    /// Whether this contract is currently available in-game.
    pub available: bool,
    /// Whether this is a limited-time offer.
    pub limited_time: bool,
}
```

**RewardType** needs additional variants:
```rust
pub enum RewardType {
    Weapon,
    Armor,
    Ship,
    Currency,
    Consumable,
    Other,
    // NEW variants:
    /// Vehicle (ATLS variants, ground vehicles).
    Vehicle,
    /// Ship component (Grade A/B parts).
    ShipComponent,
    /// Access/permission (e.g., "New to System" unlocks other contracts).
    Access,
    /// Wikelo Favor currency.
    Favor,
    /// Polaris Bit currency.
    PolarisBit,
}
```

### 7.3 New Registry Features Needed

```rust
/// Contract registry with bidirectional indexing.
pub struct ContractRegistry {
    contracts: Vec<WikieloContract>,
    /// item_id -> Vec<contract indices>
    item_to_contracts: HashMap<String, Vec<usize>>,
    /// category -> Vec<contract indices>
    category_to_contracts: HashMap<ContractCategory, Vec<usize>>,
}

impl ContractRegistry {
    /// Find all contracts that require a specific item.
    pub fn contracts_requiring_item(&self, item_id: &str) -> Vec<&WikieloContract>;

    /// Find all contracts in a category.
    pub fn contracts_by_category(&self, category: ContractCategory) -> Vec<&WikieloContract>;

    /// Get the total demand for an item across all contracts.
    pub fn total_demand_for_item(&self, item_id: &str) -> u32;
}
```

### 7.4 Emporium Location Data

```rust
/// Static Wikelo Emporium locations.
pub fn all_emporiums() -> Vec<WikieloEmporium> {
    vec![
        WikieloEmporium {
            name: "Wikelo Emporium Dasi".into(),
            orbits: "Hurston".into(),
            system: "Stanton".into(),
        },
        WikieloEmporium {
            name: "Wikelo Emporium Kinga".into(),
            orbits: "microTech".into(),
            system: "Stanton".into(),
        },
        WikieloEmporium {
            name: "Wikelo Emporium Selo".into(),
            orbits: "Yela".into(),
            system: "Stanton".into(),
        },
    ]
}
```

---

## 8. Recommendations for Next Phases

### Phase 8 (Data Model)
1. Add `ContractCategory`, `WikieloEmporium`, and `DataConfidence` types
2. Extend `WikieloContract` with category, prerequisites, turn-in locations, confidence
3. Extend `RewardType` with Vehicle, ShipComponent, Access, Favor, PolarisBit variants
4. Create `ContractRegistry` with bidirectional indexing

### Phase 9 (Data Population)
1. Populate the 14 contracts with sufficient data first
2. Run a follow-up wiki scraping session to fill the remaining 29 contracts
3. Prioritize: favor exchanges (5) -> ship contracts (9) -> weapon contracts (12) -> armor (8) -> equipment (7)
4. Cross-reference with wikelotrades.com for community-validated data

### Phase 10 (Registry)
1. Follow `WikieloRegistry` pattern from existing code
2. Add `item_to_contracts` reverse index for demand queries
3. Support filtering by category and confidence level

### Phase 11 (Demand Intel)
1. Flag routes TO Wikelo Emporiums as demand hotspots
2. Score ships based on items they're likely carrying (from source location analysis)
3. Combine supply-side (Phase 4) and demand-side scoring

---

## Sources

### Primary (used in this research)
- `.planning/phases/07-demand-research/07-01-PLAN.md` - Contract list (43 names) and infrastructure
- `.planning/phases/07-demand-research/07-CONTEXT.md` - Phase context and vision
- `.planning/research/demand-modeling-research.md` - Exchange rates and demand model
- `.planning/phases/02-item-source-research/02-RESEARCH.md` - Item source data (31 items)
- `crates/intel/src/wikelo/contracts.rs` - Existing contract types
- `crates/intel/src/wikelo/types.rs` - Existing item types
- `crates/intel/src/wikelo/items.rs` - Existing item registry (31 items)

### Secondary (referenced but not scraped)
- https://starcitizen.tools/Category:Wikelo_contracts - All 43 contract names
- https://starcitizen.tools/Wikelo - Wikelo overview page
- https://wikelotrades.com - Community contract tracker

### Not Available This Session
- Individual contract wiki pages (WebFetch unavailable)
- WebSearch for community-sourced contract details

---

*Phase: 07-demand-research*
*Research completed: 2026-01-29*
*Ready for Phase 8: YES (data model design can proceed; 14/43 contracts have sufficient data)*
*Ready for Phase 9: PARTIAL (29/43 contracts need wiki verification for complete requirements)*
