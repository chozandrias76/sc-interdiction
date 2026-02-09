# Phase 9: Contract Data Population - Research

**Researched:** 2026-02-09
**Domain:** TOML data files for Rust static game data + Wikelo contract scraping
**Confidence:** HIGH

<research_summary>
## Summary

Researched two domains: (1) Rust TOML data file patterns for static game data, and (2) complete Wikelo contract requirements from starcitizen.tools wiki.

**TOML approach:** Use `include_str!()` macro to embed TOML files at compile time, then parse with `toml` crate and serde. This keeps data human-editable while compiling into the binary for zero-runtime I/O.

**Contract data:** All 43 Wikelo contracts have been scraped from the wiki with complete requirements. The prior 07-RESEARCH.md had 29/43 contracts with incomplete data; this research fills those gaps. Several new items were discovered that aren't in the current item registry.

**Primary recommendation:** Create `contracts.toml` with all 43 contracts using the existing WikieloContract struct. Use `include_str!()` + `toml::from_str()` for compile-time embedding. Add ~15 missing items to the item registry.
</research_summary>

---

<standard_stack>
## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| toml | 0.8.x | TOML parsing/serialization | De facto Rust TOML library |
| serde | 1.0 | Serialization framework | Already in project; required for toml |

### Build-Time vs Runtime

| Approach | Pros | Cons | When to Use |
|----------|------|------|-------------|
| `include_str!()` + parse | Compiled into binary, no I/O, immutable | Parse at init, slightly larger binary | Static game data that rarely changes |
| `fs::read_to_string()` | Hot-reload possible, external files | Requires file I/O, path management | User configs, frequently updated data |
| Build script codegen | Pre-parsed at compile time | Complex build, harder to debug | Large data, performance critical |

**Chosen approach:** `include_str!()` + runtime parse once at initialization.

**Installation:**
```bash
# Already have serde in project
cargo add toml  # if not present
```
</standard_stack>

<architecture_patterns>
## Architecture Patterns

### Recommended Data Organization
```
crates/wikelo-data/
├── src/
│   ├── lib.rs
│   ├── items.rs          # Existing: all_items() static data
│   ├── contracts.rs      # NEW: all_contracts() from TOML
│   └── registry.rs       # Existing: WikieloRegistry
└── data/
    └── contracts.toml    # NEW: Human-editable contract data
```

### Pattern 1: TOML Embedded Data
**What:** Embed TOML at compile time, parse once at first access
**When to use:** Static game data that changes with game patches
**Example:**
```rust
// Source: toml docs + include_str! pattern
use once_cell::sync::Lazy;
use intel::wikelo::contracts::WikieloContract;

const CONTRACTS_TOML: &str = include_str!("../data/contracts.toml");

static CONTRACTS: Lazy<Vec<WikieloContract>> = Lazy::new(|| {
    #[derive(Deserialize)]
    struct ContractsFile {
        contracts: Vec<WikieloContract>,
    }
    let file: ContractsFile = toml::from_str(CONTRACTS_TOML)
        .expect("contracts.toml parse error");
    file.contracts
});

pub fn all_contracts() -> &'static [WikieloContract] {
    &CONTRACTS
}
```

### Pattern 2: Inline Static Data (Current Pattern)
**What:** Define data as Rust code in `fn all_items() -> Vec<T>`
**When to use:** When data is simple or needs compile-time validation
**Current use:** `items.rs` uses this pattern

### Pattern 3: TOML Schema with Serde Defaults
**What:** Use `#[serde(default)]` for optional fields
**When to use:** When some contracts have incomplete data
**Example:**
```toml
[[contracts]]
id = "yormandi_gun"
name = "Yormandi Gun"
category = "Weapon"
confidence = "Confirmed"  # from wiki
repeatable = true
available = true
# requirements and rewards as nested arrays
```

### Anti-Patterns to Avoid
- **Parsing TOML on every access:** Parse once with Lazy or OnceCell
- **Hand-rolling TOML syntax:** Use serde derive, don't parse manually
- **Mixing approaches:** Pick one (TOML or inline Rust) per data type
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| TOML parsing | Custom parser | `toml` crate | Edge cases in escaping, multiline strings |
| Lazy initialization | Manual static mut | `once_cell::sync::Lazy` | Thread safety, initialization order |
| Enum serialization | String matching | `#[serde(rename_all)]` | Less error-prone, handles kebab-case |
| Optional fields | Default values in code | `#[serde(default)]` | Cleaner TOML, explicit fallbacks |

**Key insight:** The WikieloContract struct already has `#[serde(default)]` on many fields (from Phase 8), making it TOML-ready. No type changes needed.
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: TOML Array of Tables Syntax
**What goes wrong:** Using `[contracts]` instead of `[[contracts]]`
**Why it happens:** TOML syntax for arrays of tables is non-obvious
**How to avoid:** Always use `[[table_name]]` for Vec<T>
**Warning signs:** "duplicate key" errors, single item instead of list

### Pitfall 2: Enum Serialization Mismatch
**What goes wrong:** TOML has `category = "weapon"` but enum is `Weapon`
**Why it happens:** Case sensitivity in serde
**How to avoid:** Use `#[serde(rename_all = "PascalCase")]` or match exactly
**Warning signs:** "unknown variant" deserialization errors

### Pitfall 3: Missing Items in Registry
**What goes wrong:** Contract references item_id not in WikieloRegistry
**Why it happens:** New items discovered during contract research
**How to avoid:** Cross-reference contract requirements with item registry before finalizing
**Warning signs:** Runtime panics on item lookups, empty results

### Pitfall 4: Empty Requirements Arrays
**What goes wrong:** TOML with `requirements = []` parses but breaks logic
**Why it happens:** Some contracts have TBD requirements in wiki
**How to avoid:** Mark confidence as Inferred, add validation for non-empty requirements
**Warning signs:** Contracts with zero requirements, division by zero in scoring
</common_pitfalls>

<contract_data>
## Complete Contract Data (All 43)

### Prerequisite Contract (1)

| ID | Name | Requirements | Reward | Confidence |
|----|------|-------------|--------|------------|
| new_to_system | New to System | 1x Vestal Water, 3x Tundra Kopion Horn | Access to all Wikelo contracts | Confirmed |

### Favor Exchange Contracts (5)

| ID | Name | Requirements | Reward | Confidence |
|----|------|-------------|--------|------------|
| turn_things_to_favor | Turn Things to Favor | 50x Carinite | 1x Wikelo Favor | Confirmed |
| trade_council_scrip | Trade Council Scrip for Favors? | 50x Council Scrip | 1x Wikelo Favor | Confirmed |
| need_mining_things | Need mining things. Clever things to trade. | 5x Sabir | TBA | Partial |
| trade_worm_parts | Trade Worm Parts for Favors? | 15x Irradiated Valakkar Pearl | 1x Wikelo Favor | Confirmed |
| very_hungry | Very Hungry | 1x Fried Seanut with Sauce, 1x Smoltz (Bottle) | aUEC (TBA) | Partial |

**Note:** Wiki shows "Turn Things to Favor" requires 50x Carinite (not MG Scrip as in prior research). The MG Scrip -> Favor exchange may be a different contract or game change.

### Weapon Contracts (15)

| ID | Name | Requirements | Reward | Confidence |
|----|------|-------------|--------|------------|
| yormandi_gun | Yormandi Gun | 1x Fresnel Energy LMG, 6x Yormandi Eye, 3x Yormandi Tongue | 1x Fresnel "Yormandi" LMG | Confirmed |
| fix_up_coda_gun | Fix up Coda gun | 1x Coda Pistol, 5x Ace Interceptor Helmet, 1x Tevarin War Service Marker (Pristine), 30x MG Scrip | 1x Coda "Ascension" Pistol | Confirmed |
| f55_look_better | F55 Look Better | 1x F55 LMG, 4x Yormandi Eye, 2x Yormandi Tongue, 20x Carinite | 1x F55 "Mark I" LMG | Confirmed |
| fun_kopion_skull_gun | Fun Kopion Skull Gun | 40x Jaclium, 20x Carinite, 30x Saldynium, 1x Parallax Energy Assault Rifle, 35x Tundra Kopion Horn | 1x Parallax "Fun Kopion Skull" Energy Assault Rifle | Confirmed |
| fun_kopion_tooth_gun | Fun Kopion Tooth Gun | 40x Jaclium, 20x Carinite, 30x Saldynium, 1x Parallax Energy Assault Rifle, 35x Tundra Kopion Horn | 1x Parallax "Fun Kopion Tooth" Energy Assault Rifle | Confirmed |
| fun_military_skull_gun | Fun Military Skull Gun | 40x Jaclium, 20x Carinite, 30x Saldynium, 1x Parallax Energy Assault Rifle, 35x Irradiated Valakkar Fang (Juvenile) | 1x Parallax "Fun Military Skull" Energy Assault Rifle | Confirmed |
| fun_military_tooth_gun | Fun Military Tooth Gun | 40x Jaclium, 20x Carinite, 30x Saldynium, 1x Parallax Energy Assault Rifle, 35x Irradiated Valakkar Fang (Juvenile) | 1x Parallax "Fun Military Tooth" Energy Assault Rifle | Confirmed |
| heavy_volt_zapper | Heavy Volt Zapper | 1x Fresnel Energy LMG, 5x ASD Secure Drive, 5x RCMBNT-XTL-1, 5x RCMBNT-XTL-2, 5x RCMBNT-XTL-3 | 1x Fresnel "Deepwater" Energy LMG | Confirmed |
| make_gun_sandy | Make gun sandy | 1x Quartz Energy SMG, 1x Council Scrip, 5x Ace Interceptor Helmet, 10x Advocacy Badge (Replica) | 1x Quartz "Hunter Camo" Energy SMG | Confirmed |
| make_volt_shotgun_angrier | Make VOLT shotgun angrier | 3x Irradiated Valakkar Fang (Adult), 1x Irradiated Valakkar Fang (Juvenile), 1x Wikelo Favor, 1x Prism Laser Shotgun | 1x Prism "Irradiated" Laser Shotgun | Confirmed |
| prettify_karna_gun | Prettify Karna Gun | 1x Karna Rifle, 10x Irradiated Valakkar Fang (Adult), 10x Irradiated Valakkar Fang (Juvenile), 10x Irradiated Kopion Horn, 1x Irradiated Valakkar Pearl (Grade AAA) | 1x Karna "Ascension" Rifle | Confirmed |
| snow_snipe | Snow Snipe | 1x Zenith Laser Sniper Rifle, 10x ASD Secure Drive | 1x Zenith "Snow Camo" Laser Sniper Rifle | Confirmed |
| volt_gun_more_navy | Volt gun more Navy-like | 1x Quartz Energy SMG, 30x MG Scrip, 5x Ace Interceptor Helmet, 50x Grassland Quasi Grazer Egg | 1x Quartz "Cobalt Camo" Energy SMG | Confirmed |
| zappy_gun_more_woodlike | Zappy gun more woodlike | 3x Quartz Energy SMG, 30x MG Scrip, 5x Ace Interceptor Helmet | 1x Quartz "Jungle Camo" Energy SMG | Confirmed |
| need_ore_will_give_guns | Need Ore. Will give Guns. | 10x Carinite, 20x Jaclium | Random: Scourge/Animus/GP-33 MOD "Quite Useful" | Confirmed |

### Armor Contracts (9)

| ID | Name | Requirements | Reward | Confidence |
|----|------|-------------|--------|------------|
| armor_with_horn_and_string | Armor with horn and string | 1x Carinite (Pure), 1x Antium Armor Core, 1x Antium Armor Helmet, 1x Antium Armor Legs, 1x Antium Armor Arms | Random: Ana Armor Endro piece | Confirmed |
| hide_snow_suit | Hide Snow Suit | Full Geist Armor ASD Edition set (5 pieces), 10x ASD Secure Drive | Full Geist Armor Snow Camo set | Confirmed |
| look_at_desert | Look at desert but don't see you | 1x Wikelo Favor, 3x Ace Interceptor Helmet, 15x Irradiated Valakkar Fang (Juvenile) | Random: DCP Armor Hunter Camo piece | Confirmed |
| make_glowy_armor | Make glowy armor | 3x Irradiated Valakkar Pearl, 1x Irradiated Valakkar Fang (Apex), 3x Ana Armor Arms/Legs/Core/Helmet Endro | Full Bokto Armor set | Confirmed |
| make_space_navy_armor | Make space navy armor | 1x Wikelo Favor, 3x Ace Interceptor Helmet, 3x UNE Unification War Medal (Damaged) | Random: DCP Armor Cobalt Camo piece | Confirmed |
| test_armor | Test Armor | Full Palatino set (5 pieces), 20x Yormandi Eye, 10x Yormandi Tongue | Full Palatino Mark 1 set | Confirmed |
| walk_in_danger_look_good | Walk in danger. Look good | 1x Novikov "Ascension" Helmet, 1x Novikov Exploration Suit, 30x MG Scrip, 10x Irradiated Valakkar Fang (Adult), 20x Irradiated Valakkar Fang (Juvenile), 1x Irradiated Valakkar Pearl (Grade AAA) | Novikov "Ascension" set | Confirmed |
| want_armor_look_like_tree | Want armor look like tree? | 3x Wikelo Favor, 3x Ace Interceptor Helmet, 1x Grassland Quasi Grazer Egg | Random: DCP Armor Jungle Camo piece | Confirmed |
| xian_xanthule_suit_better | Xi'an Xanthule Suit made better | 1x Xanthule Helmet, 1x Xanthule Suit, 15x Ace Interceptor Helmet, 1x Tevarin War Service Marker (Pristine), 20x MG Scrip | Xanthule Ascension set | Confirmed |

### Ship/Vehicle Contracts (9)

| ID | Name | Requirements | Reward | Confidence |
|----|------|-------------|--------|------------|
| adventure_a_aventure | Adventure a A-Venture | Full Venture set, 30x MG Scrip, 10x Saldynium, 10x Jaclium, 1x Carinite (Pure) | Full Venture Ascension set | Confirmed |
| have_good_ships_for_trade | Have Good Ships for trade | 10x Wikelo Favor | Random: Intrepid/Scorpius/C1 Spirit/Fortune/Pulse | Confirmed |
| make_atls_shoot | Make ATLS shoot | 3x Argo ATLS, 1x NN-13 Cannon, 3x Irradiated Valakkar Pearl, 1x Irradiated Valakkar Fang (Apex) | 1x ATLS IKTI | Confirmed |
| make_jumpy_atls_shoot | Make jumpy ATLS shoot | 1x Argo ATLS GEO, 1x Argo ATLS IKTI, 2x NN-13 Cannon, 1x Wikelo Favor, 1x DCHS-05 Board | 1x ATLS GEO IKTI | Confirmed |
| now_make_polaris | Now make Polaris. Short Time Deal. | 50x Wikelo Favor, 15x Polaris Bit, 10x DCHS-05 Board, 20x Carinite, 20x Irradiated Valakkar Fang (Apex), 15x Ace Interceptor Helmet, 25x Irradiated Valakkar Pearl (Grade AAA), 15x UEE 6th Platoon Medal (Pristine), 15x Carinite (Pure), 15x ASD Secure Drive, 9x RCMBNT boards | 1x Polaris Wikelo Special | Confirmed |
| turn_atls_suit_pretty | Turn ATLS suit pretty | 3x Wikelo Favor, 1x Argo ATLS GEO | 1x ATLS GEO "Orange Line" | Confirmed |
| very_very_nice_ship | Very very nice ship for trade | 30x Wikelo Favor, 10x Carinite, 15x Jaclium, 20x Saldynium, 30x Ace Interceptor Helmet | Random: F8C Lightning/Constellation Taurus | Confirmed |
| want_big_ship | Want big ship? But not too big? | 16x Wikelo Favor, 15x Ace Interceptor Helmet | Random: F7C Hornet Mk II/Guardian/Sabre/Zeus Mk II | Confirmed |
| want_polaris | Want Polaris? Need something special. | 24x Quantanium (SCU) | 1x Polaris Bit | Confirmed |

### Equipment Contracts (4)

| ID | Name | Requirements | Reward | Confidence |
|----|------|-------------|--------|------------|
| trade_for_useful_ship_parts | Trade for useful ship parts | 1x CF-337 Panther Repeater, 3x Ace Interceptor Helmet | Random: FR-66/JS-300/VK-00/Glacier | Confirmed |
| trade_for_very_useful_ship_parts | Trade for very useful ship parts | 5x Ace Interceptor Helmet, 2x CF-337 Panther Repeater | Random: FR-76/JS-400/XL-1/Avalanche | Confirmed |
| want_super_useful_ship_parts | Want super useful ship parts? | 10x Ace Interceptor Helmet, 1x Wikelo Favor, 3x CF-337 Panther Repeater | Random: FR-86/JS-500/TS-2/Blizzard | Confirmed |
| want_better_eyes | Want Better Eyes | 1x Wikelo Favor, 2x Yormandi Eye, 1x Yormandi Tongue | 1x XDL "Mark I" Monocular Rangefinder | Confirmed |

</contract_data>

<missing_items>
## Items to Add to Registry

The following items appear in contract requirements but are NOT in the current 31-item registry:

### New Items Needed (Priority Order)

| Item ID | Item Name | Category | Source/Notes |
|---------|-----------|----------|--------------|
| vestal_water | Vestal Water | Commodity | Purchasable; required for New to System |
| jaclium | Jaclium | MinedMaterial | Ore; multiple contracts |
| saldynium | Saldynium | MinedMaterial | Ore; multiple contracts |
| sabir | Sabir | Equipment | Mining tech from Hathor sites |
| advocacy_badge_replica | Advocacy Badge (Replica) | CombatLoot | Make gun sandy contract |
| une_unification_war_medal_damaged | UNE Unification War Medal (Damaged) | CombatLoot | Make space navy armor |
| grassland_quasi_grazer_egg | Grassland Quasi Grazer Egg | CreaturePart | Specific variant |
| irradiated_valakkar_pearl_aaa | Irradiated Valakkar Pearl (Grade AAA) | CreaturePart | High-grade variant |
| uee_6th_platoon_medal_pristine | UEE 6th Platoon Medal (Pristine) | CombatLoot | Polaris contract |
| rcmbnt_pwl_1 | RCMBNT-PWL-1 | Equipment | Polaris contract board |
| rcmbnt_pwl_2 | RCMBNT-PWL-2 | Equipment | Polaris contract board |
| rcmbnt_pwl_3 | RCMBNT-PWL-3 | Equipment | Polaris contract board |
| rcmbnt_rgl_1 | RCMBNT-RGL-1 | Equipment | Polaris contract board |
| rcmbnt_rgl_2 | RCMBNT-RGL-2 | Equipment | Polaris contract board |
| rcmbnt_rgl_3 | RCMBNT-RGL-3 | Equipment | Polaris contract board |
| rcmbnt_xtl_1 | RCMBNT-XTL-1 | Equipment | Heavy Volt Zapper + Polaris |
| rcmbnt_xtl_2 | RCMBNT-XTL-2 | Equipment | Heavy Volt Zapper + Polaris |
| rcmbnt_xtl_3 | RCMBNT-XTL-3 | Equipment | Heavy Volt Zapper + Polaris |
| fried_seanut_with_sauce | Fried Seanut with Sauce | Consumable | Very Hungry contract |
| smoltz_bottle | Smoltz (Bottle) | Consumable | Very Hungry contract |

### Weapon Base Items (For Contract Requirements)
| Item ID | Item Name | Category | Notes |
|---------|-----------|----------|-------|
| fresnel_energy_lmg | Fresnel Energy LMG | Equipment | Base weapon for Yormandi/Heavy Volt |
| coda_pistol | Coda Pistol | Equipment | Fix up Coda gun |
| f55_lmg | F55 LMG | Equipment | F55 Look Better |
| parallax_energy_assault_rifle | Parallax Energy Assault Rifle | Equipment | Kopion/Military guns |
| quartz_energy_smg | Quartz Energy SMG | Equipment | Multiple camo contracts |
| prism_laser_shotgun | Prism Laser Shotgun | Equipment | VOLT shotgun angrier |
| karna_rifle | Karna Rifle | Equipment | Prettify Karna |
| zenith_laser_sniper_rifle | Zenith Laser Sniper Rifle | Equipment | Snow Snipe |
| cf337_panther_repeater | CF-337 Panther Repeater | Equipment | Ship parts contracts |
| nn13_cannon | NN-13 Cannon | Equipment | ATLS shoot contracts |

### Armor Base Items (For Contract Requirements)
| Item ID | Item Name | Category | Notes |
|---------|-----------|----------|-------|
| antium_armor_set | Antium Armor (Core/Helmet/Legs/Arms) | Equipment | Armor with horn |
| geist_armor_asd_set | Geist Armor ASD Edition (5 pieces) | Equipment | Hide Snow Suit |
| ana_armor_endro_set | Ana Armor Endro (4 pieces) | Equipment | Make glowy armor |
| palatino_set | Palatino (5 pieces) | Equipment | Test Armor |
| novikov_ascension_set | Novikov "Ascension" (Helmet/Suit) | Equipment | Walk in danger |
| xanthule_set | Xanthule (Helmet/Suit) | Equipment | Xi'an suit |
| venture_set | Venture (4 pieces) | Equipment | Adventure a A-Venture |

### Vehicle Items
| Item ID | Item Name | Category | Notes |
|---------|-----------|----------|-------|
| argo_atls | Argo ATLS | Vehicle | Make ATLS shoot |
| argo_atls_geo | Argo ATLS GEO | Vehicle | Make jumpy ATLS |
| argo_atls_ikti | Argo ATLS IKTI | Vehicle | Make jumpy ATLS (input) |

**Total new items needed:** ~40-50 items

**Recommendation:** Add items incrementally during Phase 9 as contracts are populated. Group by category for maintainability.
</missing_items>

<code_examples>
## Code Examples

### TOML Contract Format
```toml
# data/contracts.toml
# Source: Pattern from toml crate docs + project conventions

[[contracts]]
id = "new_to_system"
name = "New to System"
category = "Prerequisite"
confidence = "Confirmed"
repeatable = false
available = true
limited_time = false
prerequisites = []
turn_in_locations = ["Wikelo Emporium Kinga", "Wikelo Emporium Dasi", "Wikelo Emporium Selo"]

[[contracts.requirements]]
item_id = "vestal_water"
quantity = 1

[[contracts.requirements]]
item_id = "tundra_kopion_horn"
quantity = 3

[[contracts.rewards]]
name = "Access to Wikelo Contracts"
reward_type = "Access"

[[contracts]]
id = "yormandi_gun"
name = "Yormandi Gun"
category = "Weapon"
confidence = "Confirmed"
repeatable = true
available = true
prerequisites = ["new_to_system"]
turn_in_locations = ["Wikelo Emporium Kinga", "Wikelo Emporium Dasi", "Wikelo Emporium Selo"]

[[contracts.requirements]]
item_id = "fresnel_energy_lmg"
quantity = 1

[[contracts.requirements]]
item_id = "yormandi_eye"
quantity = 6

[[contracts.requirements]]
item_id = "yormandi_tongue"
quantity = 3

[[contracts.rewards]]
name = "Fresnel 'Yormandi' LMG"
reward_type = "Weapon"
```

### Loading TOML in Rust
```rust
// Source: toml crate docs + once_cell pattern
// crates/wikelo-data/src/contracts.rs

use once_cell::sync::Lazy;
use intel::wikelo::contracts::WikieloContract;
use serde::Deserialize;

const CONTRACTS_TOML: &str = include_str!("../data/contracts.toml");

#[derive(Deserialize)]
struct ContractsFile {
    contracts: Vec<WikieloContract>,
}

static CONTRACTS: Lazy<Vec<WikieloContract>> = Lazy::new(|| {
    let file: ContractsFile = toml::from_str(CONTRACTS_TOML)
        .expect("Failed to parse contracts.toml - check TOML syntax");
    file.contracts
});

/// Returns all Wikelo contracts.
#[must_use]
pub fn all_contracts() -> &'static [WikieloContract] {
    &CONTRACTS
}

/// Find a contract by ID.
#[must_use]
pub fn get_contract(id: &str) -> Option<&'static WikieloContract> {
    CONTRACTS.iter().find(|c| c.id == id)
}
```

### Validation Test
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_contracts_parse() {
        // This will panic if TOML is malformed
        let contracts = all_contracts();
        assert_eq!(contracts.len(), 43, "Expected 43 contracts");
    }

    #[test]
    fn test_all_contracts_have_unique_ids() {
        let contracts = all_contracts();
        let mut ids: Vec<_> = contracts.iter().map(|c| &c.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), contracts.len(), "Duplicate contract IDs found");
    }

    #[test]
    fn test_prerequisite_contract_exists() {
        let contract = get_contract("new_to_system");
        assert!(contract.is_some(), "New to System prerequisite missing");
    }
}
```
</code_examples>

<sota_updates>
## State of the Art (2025-2026)

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| lazy_static! macro | once_cell::sync::Lazy | 2023+ | once_cell is now in std (1.70+), cleaner API |
| toml 0.5 | toml 0.8 | 2023 | Better error messages, TOML 1.0 compliance |
| Manual TOML construction | serde_toml macro | 2024 | Compile-time TOML in some cases |

**New tools/patterns to consider:**
- **std::sync::LazyLock:** Rust 1.80+ has LazyLock in std, can replace once_cell in future
- **TOML 1.0:** Current toml crate fully supports TOML 1.0 spec

**Deprecated/outdated:**
- **lazy_static!:** Works but once_cell/LazyLock preferred
- **toml 0.5:** Upgrade to 0.8 for better errors
</sota_updates>

<open_questions>
## Open Questions

1. **MG Scrip Exchange Discrepancy**
   - What we know: Wiki shows "Turn Things to Favor" requires 50x Carinite, not MG Scrip
   - What's unclear: Was there a game change? Is there a separate MG Scrip exchange?
   - Recommendation: Add both possible exchanges, mark MG Scrip one as Partial confidence

2. **"Need mining things" Contract Status**
   - What we know: Wiki shows status "Currently unavailable" and reward "TBA"
   - What's unclear: Is this a future contract or bugged?
   - Recommendation: Include with confidence=Partial, available=false

3. **Item Source Locations for New Items**
   - What we know: ~40 new items need to be added
   - What's unclear: Source locations for many (especially equipment/weapons)
   - Recommendation: Add items with minimal source data, mark low confidence
</open_questions>

<sources>
## Sources

### Primary (HIGH confidence)
- starcitizen.tools individual contract pages (all 43 contracts fetched)
- docs.rs/toml - TOML crate documentation
- serde.rs - Serde derive documentation

### Secondary (MEDIUM confidence)
- perfcode.com Rust TOML guide - verified against toml docs
- Rust Cookbook - Build Tools section for include_str! pattern

### Tertiary (for validation)
- Project codebase: crates/intel/src/wikelo/contracts.rs (existing types)
- Project codebase: crates/wikelo-data/src/items.rs (existing pattern)
</sources>

<metadata>
## Metadata

**Research scope:**
- Core technology: TOML parsing with serde in Rust
- Ecosystem: toml crate, once_cell for lazy init
- Patterns: Embedded data with include_str!, TOML array of tables
- Pitfalls: TOML syntax, enum serialization, missing items

**Confidence breakdown:**
- TOML approach: HIGH - well-documented, project already uses serde
- Contract data: HIGH - all 43 scraped from official wiki
- New items list: MEDIUM - extracted from requirements, sources incomplete
- Code examples: HIGH - from official docs and project patterns

**Research date:** 2026-02-09
**Valid until:** 2026-03-09 (30 days - game may patch contract data)
</metadata>

---

*Phase: 09-contract-data-population*
*Research completed: 2026-02-09*
*Ready for planning: YES*
