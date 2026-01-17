# DataForge Exploration Findings

**Adapted from scunpacked-data JSON structure (not Game2.json)**

The original plan assumed a single `Game2.json` file from scdatatools extraction. Since scdatatools is broken, we used the `scunpacked-data` repository which provides pre-extracted JSON files.

## Data Source

**Location:** `extracted/scunpacked-data/`

**Main Files:**
| File | Size | Records | Description |
|------|------|---------|-------------|
| items.json | 47.3 MB | 18,592 | All game items with basic metadata |
| ships.json | 1.4 MB | 281 | Ship definitions with specs |
| labels.json | 10.3 MB | ~50K+ | Localization strings (key-value dict) |
| manufacturers.json | 115.5 KB | 879 | Manufacturer definitions |
| fps-items.json | 19.7 MB | 4,567 | FPS weapon/equipment items |
| ship-items.json | 11.3 MB | 5,590 | Ship components |

**Subdirectories (detailed individual files):**
- `items/` - 18,592 individual item JSON files with full raw data
- `ships/` - 562 ship files (includes `-raw.json` variants with full details)
- `factions/` - 135 faction tint palette definitions

## Wikelo Data Discovery

### Overview

Wikelo is a Banu artisan/merchant NPC who runs the "Wikelo Emporium" - three stations in the Stanton system where players can exchange "Wikelo Favors" (mission currency) for collector ships and items.

### Wikelo-Related Records Found

**Search Results: 44 matches for "wikelo"**

#### Items (8 Wikelo items in items.json)

| ClassName | Type | Name | Description |
|-----------|------|------|-------------|
| `Carryable_1H_CY_banu_favour_Wikelo` | Misc.Harvestable | Wikelo Favor | Mission currency token - "favors indicate successful completion of work done for Wikelo" |
| `Carryable_1H_CY_banu_favour_Wikelo_special` | Misc | Polaris Bit | Special favor variant |
| `PlayerDeco_Static_Hologram_Wikelo_1_a` | Usable.Cargo | Wikelo Emporium Bust | Hologram flair item |
| `Flair_Hologram_Wikelo_1_a` | Usable | (placeholder) | Hologram flair |
| `dmc_jacket_13_02_01` | Char_Clothing_Torso_1 | Wikelo Emporium Jacket | Clothing item |
| `apar_special_ballistic_01_mat01` | WeaponPersonal | Scourge "Quite Useful" Railgun | Wikelo special weapon |
| `apar_special_ballistic_02_mat01` | WeaponPersonal | Animus "Quite Useful" Missile Launcher | Wikelo special weapon |
| `behr_glauncher_ballistic_01_mat01` | WeaponPersonal | GP-33 MOD "Quite Useful" Grenade Launcher | Wikelo special weapon |

#### Collector Ships (26+ Wikelo ships in ships.json)

Ships with "Wikelo Special" naming pattern:

| ClassName | Name |
|-----------|------|
| RSI_Polaris_Collector_Military | RSI Polaris Wikelo Special |
| RSI_Scorpius_Stealth | RSI Scorpius Wikelo Sneak Special |
| ANVL_Hornet_F7_Mk2_Collector_Mod | Anvil F7 Hornet Mk Wikelo |
| ANVL_Lightning_F8C_Collector_Military | Anvil F8C Lightning Wikelo War Special |
| ANVL_Lightning_F8C_Collector_Stealth | Anvil F8C Lightning Wikelo Sneak Special |
| MRAI_Guardian_MX_Collector_Military | Mirai Guardian MX Wikelo War Special |
| CRUS_Starlifter_A2_Collector_Military | Crusader A2 Hercules Starlifter Wikelo War Special |
| AEGS_Sabre_Firebird_Collector_Milt | Aegis Sabre Firebird Wikelo War Special |
| ... and 18+ more |

**Ship Naming Patterns:**
- `*_Collector_Military` - War variants
- `*_Collector_Stealth` - Sneak variants
- `*_Collector_Indust` - Work variants
- `*_Collector_Competition` - Racing/competition variants

#### Faction Palettes (3 files in factions/)

- `faction_wikelo_a.json` - Brown/tan tint palette
- `faction_wikelo_b.json` - Gray/dark tint palette
- `faction_wikelo_c.json` - Dark/gold accent palette

These define the visual theming for Wikelo-branded items.

### Wikelo Emporium Locations

From labels.json:

| Station | Name |
|---------|------|
| Collector_Station_01 | Wikelo Emporium Dasi Station |
| Collector_Station_02 | Wikelo Emporium Selo Station |
| Collector_Station_03 | Wikelo Emporium Kinga Station |

**Station Description:**
> "Come to Wikelo Emporium for fine made things. Always looking for useful things to buy too..."

### Localization Keys

Key patterns in labels.json:
- `PU_Collector_BAN_M_CIV_ATC_*` - ATC voice lines for Wikelo stations
- `TheCollector_Coin_Name/Desc` - Wikelo Favor item localization
- `item_*Wikelo*` - Item descriptions

## Data Structure

### items.json Entry Structure

```json
{
  "className": "Carryable_1H_CY_banu_favour_Wikelo",
  "reference": "3b1cf59f-1e6b-4a91-9edb-f8c5ddf791ae",
  "itemName": "carryable_1h_cy_banu_favour_wikelo",
  "type": "Misc",
  "subType": "Harvestable",
  "size": 1,
  "grade": 1,
  "name": "Wikelo Favor",
  "tags": "",
  "stdItem": {
    "UUID": "...",
    "ClassName": "...",
    "Size": 1,
    "Grade": 1,
    "Type": "Misc.Harvestable",
    "Name": "Wikelo Favor",
    "Description": "..."
  }
}
```

### Individual Item Files (items/*.json)

Much more detailed with:
- `Raw.Entity` - Full entity definition with components
- `Raw.Entity.Components.SAttachableComponentParams` - Type, tags, localization
- `Item` - Simplified item data (same as items.json entry)

### ships.json Entry Structure

```json
{
  "UUID": "...",
  "ClassName": "RSI_Polaris_Collector_Military",
  "Name": "RSI Polaris Wikelo Special",
  "Description": "...",
  "Career": "Combat",
  "Size": 5,
  "Cargo": 576,
  "Crew": 12,
  "Insurance": { "ExpeditedCost": ..., "ClaimTime": ... },
  "Propulsion": { ... },
  "Weapons": [ ... ]
}
```

## Key Item Types

From `dataforge-explorer types`:

| Type | Count | Relevance |
|------|-------|-----------|
| Cargo | 1,554 | Trade goods |
| Misc | 1,486 | Includes Wikelo Favor |
| Usable | 1,392 | Includes flair items |
| WeaponPersonal | 390 | Includes Wikelo weapons |
| Paints | 938 | Ship liveries |

## Mission Data Gap

**Important Finding:** Mission/contract definitions are NOT in the scunpacked-data repository.

- No `missions.json` or `contracts.json` found
- No mission givers or mission chains
- No quest requirements or reward definitions

The scunpacked-data focuses on items, ships, and equipment - not gameplay systems like missions.

**Implications for Phase 3:**
- We can extract Wikelo items and ships
- We can identify Wikelo-branded content
- We CANNOT extract mission requirements or contract details
- Mission data may need to come from wiki scraping or different data sources

## Extraction Path for Phase 3

### What We CAN Extract:

1. **Wikelo Favor Item** - `Carryable_1H_CY_banu_favour_Wikelo`
   - UUID for reference
   - Description confirming it's mission currency

2. **Collector Ships** - All `*_Collector_*` ships
   - Names, specs, loadouts
   - Can identify as Wikelo Collection rewards

3. **Wikelo Stations** - From labels
   - Station names and descriptions

4. **Special Weapons** - "Quite Useful" variants
   - Linked to Wikelo through descriptions

### What We CANNOT Extract:

1. Mission definitions (which missions grant favors)
2. Shop inventory (what items cost in favors)
3. Mission chains or prerequisites
4. Reputation requirements

## Recommendations

1. **Use scunpacked-data for item/ship metadata** - It's well-structured and comprehensive

2. **Supplement with wiki scraping** - For mission requirements, costs, and prerequisites

3. **Build extraction pipeline** that:
   - Loads items.json and ships.json
   - Filters by Wikelo/Collector patterns
   - Outputs structured data for our use case

4. **Consider caching** - items.json is 47MB, loading time is ~2-3 seconds

## Tools Created

- `crates/dataforge-explorer/` - CLI for exploring scunpacked-data
  - `search <pattern>` - Find records by name/content
  - `types` - List unique types with counts
  - `show <name>` - Display full record
  - `dump-types <type>` - Dump all records of a type
  - `files` - List available data files
  - `stats` - Show record counts
