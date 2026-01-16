# Phase 3: Wikelo Data Module - Research

**Researched:** 2026-01-15
**Domain:** Rust static data registry crate + Game data extraction
**Confidence:** MEDIUM (registry pattern HIGH, data sourcing needs work)

<critical_discovery>
## Critical Discovery: Data Sourcing

**Problem:** Phase 2 wiki research produced 31 items with 9 flagged as low-confidence. The fundamental issue is that wiki data is community-curated, potentially stale, and can't be trusted as authoritative.

**Solution Found:** Star Citizen's game client contains authoritative data in `Data.p4k`:

| File | Size | Contents |
|------|------|----------|
| `Data\Game2.dcb` | 294MB | DataForge database — items, contracts, missions, locations |
| `Data\Localization\english\global.ini` | 9.5MB | English text strings for all game content |

**Extraction Tools:**
- [unp4k](https://github.com/dolkensp/unp4k) — Extracts files from Data.p4k (encrypted zip with ZSTD)
- [unforge](https://github.com/dolkensp/unp4k) — Converts Game2.dcb (DataForge) to readable XML
- `7z` — Can list/extract files from Data.p4k (with some path format quirks)

**Wikelo-Specific Files Found:**
- Wikelo hologram 3D assets (`Data\Objects\props\reward\hologram\wikelo_hologram_*.cgfm`)
- Wikelo ship skins (`*_wikelo_a.mtl` for various ships)
- Banu Collector building assets (the Wikelo Emporium location)
- Contract definitions likely in Game2.dcb DataForge database

**Implications for Roadmap:**
1. Phase 2 wiki research is a starting point, not the final data source
2. Need new phase(s) to build proper game data extraction pipeline
3. sc-data-extractor crate needs updating to handle Data.p4k directly
4. A CLI viewer for exploring extracted data would help Claude understand structure

**Recommended Roadmap Changes:**
- Insert phase: "Game Data Extraction Pipeline" before Phase 3
- Update sc-data-extractor to extract from Data.p4k (not just SCLogistics)
- Build CLI viewer for exploring Game2.dcb structure
- Phase 3 then consumes extracted data instead of hardcoded wiki data
</critical_discovery>

<research_summary>
## Summary

Two findings:

**1. Registry Pattern (HIGH confidence):** The existing `ShipRegistry` pattern in `crates/intel/src/ships/registry.rs` is the exact template for the data module. Standard `HashMap` with runtime construction matches the project's explicit dependency injection preference.

**2. Data Sourcing (NEEDS WORK):** Wiki-scraped data is unreliable. The authoritative source is `Data.p4k` containing `Game2.dcb` (DataForge database) and `global.ini` (localization). This requires tooling updates before Phase 3 can produce reliable data.

**Primary recommendation:** Before implementing the registry, build/update the data extraction pipeline to source from game files. The registry pattern is straightforward once we have reliable data.
</research_summary>

<standard_stack>
## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| std::collections::HashMap | std | Item and location lookups | Already used in ShipRegistry; sufficient for 31 items |
| serde | 1.0 (workspace) | Serialization | Already in workspace; enables JSON export |
| thiserror | 1.0 (workspace) | Error types | Already in workspace; consistent error handling |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| tracing | 0.1 (workspace) | Logging | Already in workspace; for registry load logging |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| HashMap | phf::Map | Compile-time perfect hashing; overkill for 31 items, adds build complexity |
| Runtime construction | lazy_static | Deprecated; std::sync::LazyLock preferred, but context doc wants explicit DI anyway |
| Two HashMaps | BTreeMap | Ordered iteration; not needed for lookups |

**Installation:**
```toml
# No new dependencies - use workspace dependencies
[dependencies]
serde.workspace = true
thiserror.workspace = true
tracing.workspace = true
```
</standard_stack>

<architecture_patterns>
## Architecture Patterns

### Recommended Project Structure
```
crates/wikelo-data/
├── Cargo.toml
└── src/
    ├── lib.rs           # Re-exports, crate docs
    ├── registry.rs      # WikieloRegistry struct (main entry point)
    ├── items.rs         # Static item definitions (const arrays)
    └── error.rs         # Optional: error types if needed
```

### Pattern 1: Registry Pattern (from ShipRegistry)
**What:** Central struct holding all data with HashMap indexes for fast lookup
**When to use:** Any static data needing bidirectional queries
**Example:**
```rust
// Source: crates/intel/src/ships/registry.rs (existing pattern)
pub struct WikieloRegistry {
    items: Vec<WikieloItem>,
    by_id: HashMap<String, usize>,        // item_id -> index
    by_location: HashMap<String, Vec<usize>>, // location_name -> item indexes
    by_system: HashMap<String, Vec<usize>>,   // system -> item indexes
    by_category: HashMap<ItemCategory, Vec<usize>>,
}

impl WikieloRegistry {
    /// Create registry from static item definitions.
    pub fn new() -> Self {
        Self::from_items(ALL_ITEMS.to_vec())
    }

    /// Create from provided items (enables testing).
    pub fn from_items(items: Vec<WikieloItem>) -> Self {
        let mut by_id = HashMap::new();
        let mut by_location = HashMap::new();
        // ... build indexes
    }
}
```

### Pattern 2: Static Item Definitions
**What:** Const arrays of item structs, not inline construction
**When to use:** Large amounts of static data that shouldn't be rebuilt
**Example:**
```rust
// items.rs - const definitions
pub const ALL_ITEMS: &[WikieloItem] = &[
    WikieloItem {
        id: "irradiated_valakkar_fang_juvenile",
        name: "Irradiated Valakkar Fang (Juvenile)",
        category: ItemCategory::CreaturePart,
        sources: &[VALAKKAR_JUVENILE_SOURCE],
        // ...
    },
    // ... more items
];

// Separate const for sources enables reuse
const VALAKKAR_JUVENILE_SOURCE: ItemSource = ItemSource {
    location: SourceLocation {
        name: "Monox",
        system: "Pyro",
        description: Some("Invasive species location"),
    },
    method: AcquisitionMethod::Hunting,
    reliability: 4,
    notes: Some("Native to Leir III; also found on Daymar"),
};
```

### Pattern 3: Bidirectional Lookup with Shared Indexes
**What:** Multiple HashMap indexes pointing to the same underlying Vec
**When to use:** When same data needs querying multiple ways
**Example:**
```rust
impl WikieloRegistry {
    /// Get item by ID.
    pub fn get(&self, id: &str) -> Option<&WikieloItem> {
        self.by_id.get(id).map(|&idx| &self.items[idx])
    }

    /// Get all items available at a location.
    pub fn items_at_location(&self, location: &str) -> Vec<&WikieloItem> {
        self.by_location
            .get(location)
            .map(|indexes| indexes.iter().map(|&i| &self.items[i]).collect())
            .unwrap_or_default()
    }

    /// Get all items in a system.
    pub fn items_in_system(&self, system: &str) -> Vec<&WikieloItem> {
        self.by_system
            .get(system)
            .map(|indexes| indexes.iter().map(|&i| &self.items[i]).collect())
            .unwrap_or_default()
    }
}
```

### Anti-Patterns to Avoid
- **Global singleton with lazy_static/OnceCell:** Context doc explicitly prefers dependency injection
- **phf for small datasets:** Adds build complexity; HashMap is O(1) anyway for 31 items
- **Rebuilding indexes on every query:** Build once at construction, query many times
- **String-based location matching without normalization:** Use consistent naming from types.rs
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Perfect hash maps | Custom hashing | std HashMap | 31 items; overhead not worth it |
| Global registry | Manual unsafe static | Explicit DI (pass registry to consumers) | Context doc preference; testability |
| Location normalization | Custom string munging | Match existing terminal naming from route-graph | Consistency with existing codebase |
| Serialization | Manual JSON building | serde derive | Already in workspace; battle-tested |

**Key insight:** This is a commodity problem with clear solutions. The codebase already has the exact pattern needed (ShipRegistry). The main risk is over-engineering — resist the urge to add complexity.
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Location Name Mismatches
**What goes wrong:** Item locations don't match terminal names in route-graph, breaking integration
**Why it happens:** Phase 2 research used wiki names; route-graph uses different naming
**How to avoid:** Cross-reference location names with existing terminal data before finalizing
**Warning signs:** Location lookups return empty when they shouldn't

### Pitfall 2: Confidence Level Ignored
**What goes wrong:** Low-confidence items (reliability 1-2) treated same as verified data
**Why it happens:** Easy to forget confidence metadata exists
**How to avoid:** Include confidence in API responses; document which items need validation
**Warning signs:** Intel based on unverified sources

### Pitfall 3: Redundant Type Definitions
**What goes wrong:** Defining new types that duplicate Phase 1 types in intel crate
**Why it happens:** Not realizing types already exist
**How to avoid:** Import types from intel::wikelo, don't redefine
**Warning signs:** Compilation errors about conflicting types; unnecessary dependencies

### Pitfall 4: Over-Engineering the Registry
**What goes wrong:** Adding features not needed yet (filters, sorting, pagination)
**Why it happens:** Anticipating future needs
**How to avoid:** YAGNI — implement exactly what Phase 3 plans specify
**Warning signs:** Code that isn't called by any consumer
</common_pitfalls>

<code_examples>
## Code Examples

Verified patterns from the existing codebase:

### Registry Construction (from ShipRegistry)
```rust
// Source: crates/intel/src/ships/registry.rs:43-63
pub fn from_api_ships(api_ships: Vec<ShipModel>) -> anyhow::Result<Self> {
    let mut ships = Vec::new();
    let mut by_name = HashMap::new();

    for api_ship in api_ships {
        if let Some(cargo_ship) = from_api_ship(&api_ship) {
            let name_key = normalize_ship_name(&cargo_ship.name);
            by_name.insert(name_key, ships.len());
            ships.push(cargo_ship);
        }
    }

    Ok(Self { ships, by_name })
}
```

### Lookup Methods (from ShipRegistry)
```rust
// Source: crates/intel/src/ships/registry.rs:72-78
pub fn find_by_name(&self, name: &str) -> Option<&CargoShip> {
    let normalized = normalize_ship_name(name);
    let idx = self.by_name.get(&normalized)?;
    self.ships.get(*idx)
}
```

### Workspace Crate Setup (from intel)
```toml
# Source: crates/intel/Cargo.toml
[package]
name = "intel"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
thiserror.workspace = true
serde.workspace = true
# ...

[lints]
workspace = true
```
</code_examples>

<sota_updates>
## State of the Art (2025-2026)

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| lazy_static! macro | std::sync::LazyLock | Rust 1.80 (2024) | lazy_static deprecated; use std when lazy init needed |
| once_cell crate | std::sync::OnceLock | Rust 1.70 (2023) | Prefer std; once_cell only for MSRV < 1.70 |
| phf for all static maps | HashMap for small datasets | Always true | phf has niche use; not worth complexity for < 1000 items |

**New tools/patterns to consider:**
- None — this domain is stable and well-understood

**Deprecated/outdated:**
- **lazy_static:** Use std::sync::LazyLock or explicit construction instead
- **once_cell:** Mostly superseded by std; only needed for features not yet in std
</sota_updates>

<open_questions>
## Open Questions

Things that couldn't be fully resolved:

1. **Game2.dcb Structure**
   - What we know: DataForge binary database, ~294MB, contains game definitions
   - What's unclear: Exact schema for Wikelo contracts, item→source mappings
   - Recommendation: Extract and explore with unforge; build CLI viewer for navigation

2. **Localization Key Mapping**
   - What we know: global.ini contains English strings keyed by IDs
   - What's unclear: How to correlate localization keys with DataForge record IDs
   - Recommendation: Extract both files; analyze key patterns

3. **Data.p4k Extraction Method**
   - What we know: 7z can list/extract; unp4k handles encryption/ZSTD natively
   - What's unclear: Whether sc-data-extractor should use 7z, native Rust, or shell out to unp4k
   - Recommendation: Start with 7z (available in WSL); consider native implementation later

4. **Location naming consistency**
   - What we know: Route-graph uses terminal names; wiki uses location names; game files use internal IDs
   - What's unclear: How game file location IDs map to human-readable names
   - Recommendation: Build mapping from DataForge + localization; cross-reference with route-graph

5. **CLI Viewer Scope**
   - What we know: A viewer would help Claude navigate the data structure
   - What's unclear: What queries/views are most valuable
   - Recommendation: Start with tree view of DataForge records; add search by type/name
</open_questions>

<sources>
## Sources

### Primary (HIGH confidence)
- crates/intel/src/ships/registry.rs — ShipRegistry pattern to replicate
- crates/intel/src/wikelo/types.rs — Phase 1 types already defined
- .planning/phases/02-item-source-research/02-DATA-READY.md — 31 items ready for implementation

### Secondary (MEDIUM confidence)
- [rust-phf crate](https://github.com/rust-phf/rust-phf) — verified not needed for this scale
- [lazy_static deprecation](https://github.com/rust-lang-nursery/lazy-static.rs/issues/214) — verified, prefer std

### Tertiary (LOW confidence - needs validation)
- None — all findings verified against existing codebase
</sources>

<metadata>
## Metadata

**Research scope:**
- Core technology: Rust std library (HashMap, Vec)
- Ecosystem: No external dependencies needed beyond workspace
- Patterns: Registry pattern from ShipRegistry
- Pitfalls: Location naming, over-engineering

**Confidence breakdown:**
- Standard stack: HIGH — uses existing workspace dependencies only
- Architecture: HIGH — exact pattern exists in codebase (ShipRegistry)
- Pitfalls: HIGH — based on existing code review
- Code examples: HIGH — taken directly from existing codebase

**Research date:** 2026-01-15
**Valid until:** Indefinitely (standard Rust patterns, no external dependencies)
</metadata>

<next_steps>
## Recommended Next Steps

### Immediate: Update Roadmap

Insert a new phase before Phase 3:

**Phase 2.1: Game Data Extraction Pipeline** (INSERTED)
- Goal: Build tooling to extract and explore Data.p4k contents
- Plans:
  - 02.1-01: Extract Game2.dcb and global.ini from Data.p4k (using 7z or unp4k)
  - 02.1-02: Convert Game2.dcb to readable format (XML/JSON via unforge or native parser)
  - 02.1-03: Build CLI viewer for exploring DataForge structure
  - 02.1-04: Locate Wikelo contract definitions and item→source mappings

### Crate Updates Needed

**sc-data-extractor:**
- Currently: Parses SCLogistics repository (community-curated XML/JSON)
- Needed: Extract from Data.p4k directly
- Options:
  1. Shell out to `7z` for extraction (quick, available)
  2. Shell out to `unp4k.exe` via Wine (handles encryption natively)
  3. Native Rust p4k reader (most work, best long-term)

**data-viewer (new or updated):**
- Purpose: CLI tool for Claude to explore extracted game data
- Features:
  - List record types in DataForge
  - Search by name/type/ID
  - Show record details with localized names
  - Export specific records to JSON

### Data Files Location

```
/mnt/c/Star Citizen/StarCitizen/LIVE/Data.p4k
├── Data/Game2.dcb           # DataForge database (294MB)
├── Data/Localization/english/global.ini  # English strings (9.5MB)
└── ... (150GB of assets, mostly not needed)
```

### Session Handoff Notes

For the next session:
1. This research doc captures the Data.p4k discovery
2. Roadmap needs Phase 2.1 inserted
3. Start by extracting Game2.dcb and global.ini
4. Use unforge to convert .dcb to XML for exploration
5. Find Wikelo-specific records in the converted XML
6. Build CLI viewer based on what's discovered

### Reference Links

- [unp4k releases](https://github.com/dolkensp/unp4k/releases/tag/v3.13.66)
- [tradein.space source](https://bitbucket.org/Cpt_BA/tradeinspace/src/master/) — example of Data.p4k consumer
- [scunpacked](https://github.com/richardthombs/scunpacked) — archived but shows data structure patterns
- [StarBreaker](https://github.com/diogotr7/StarBreaker) — Rust toolkit for SC game files
</next_steps>

---

*Phase: 03-wikelo-data-module*
*Research completed: 2026-01-15*
*Ready for planning: BLOCKED — needs Phase 2.1 (Data Extraction) first*
