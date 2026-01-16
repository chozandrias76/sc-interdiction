# Phase 3: Wikelo Data Module - Research

**Researched:** 2026-01-15
**Domain:** Rust static data registry crate
**Confidence:** HIGH

<research_summary>
## Summary

Researched patterns for creating a Rust crate containing static item/source data with bidirectional lookups. This is a straightforward domain with clear patterns already established in the codebase.

Key finding: The existing `ShipRegistry` pattern in `crates/intel/src/ships/registry.rs` is the exact template for this work. No external libraries needed — standard `HashMap` with runtime construction matches the project's explicit dependency injection preference over global singletons.

**Primary recommendation:** Create `wikelo-data` crate following the `ShipRegistry` pattern exactly. Construct registry at runtime from const item definitions. No `phf`, no `lazy_static`, no global singletons.
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

1. **Location naming consistency**
   - What we know: Route-graph uses terminal names; wiki uses location names
   - What's unclear: Whether exact match is needed or fuzzy matching acceptable
   - Recommendation: Review route-graph terminal data during planning; may need normalization layer

2. **Future system expansion**
   - What we know: Current data covers Stanton, Pyro, Terra
   - What's unclear: Whether other systems will be added
   - Recommendation: Design for current needs; String-based system field handles expansion naturally
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

---

*Phase: 03-wikelo-data-module*
*Research completed: 2026-01-15*
*Ready for planning: yes*
