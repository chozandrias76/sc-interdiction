# Phase 2: Item Source Research - Research

**Researched:** 2026-01-15
**Verified:** 2026-01-15 (Task 02-01)
**Domain:** Star Citizen wiki data extraction for Wikelo item sources
**Confidence:** HIGH

<research_summary>
## Summary

Researched the Star Citizen ecosystem for Wikelo item sources. This phase involves extracting item source location data from the Star Citizen wiki (starcitizen.tools) and community resources. The data model from Phase 1 is already in place - this phase populates it with actual game data.

Key finding: The wiki and community tools (wikelotrades.com, finder.cstone.space, regolith.rocks) provide comprehensive item source data. Most items have well-documented spawn locations, but some rare items (medals, artifact fragments) have less precise location data requiring "hostile outpost" or "mission loot" as general sources.

**Primary recommendation:** Use starcitizen.tools as primary source, cross-reference with finder.cstone.space for precise location data. Structure data by roadmap plan breakdown (biological, mining, mission/loot).
</research_summary>

<item_source_findings>
## Item Source Data Collected

### Plan 02-01: Creature/Biological Items

| Item | Source Location | System | Method | Reliability | Notes |
|------|----------------|--------|--------|-------------|-------|
| **Irradiated Valakkar Fang (Juvenile/Adult/Apex)** | Monox, Pyro I, Daymar | Pyro/Stanton | Hunting | 4 | Native to Leir III; invasive on Monox/Pyro I/Daymar; sizes: Juvenile ~5m, Adult ~15m, Apex ~300m |
| **Irradiated Valakkar Pearl** | Monox, Pyro I, Daymar | Pyro/Stanton | Hunting | 4 | Same as fangs; pearl-like body parts harvested |
| **Tundra Kopion Horn** | microTech (equator/Green Zones) | Stanton | Hunting | 4 | Elevated grasslands/tundras; packs of 8-18; OM coords community-sourced |
| **Irradiated Kopion Horn** | Unknown | Unknown | Hunting | 2 | Likely variant on Pyro; needs validation |
| **Yormandi Eye** | ASD Onyx Facility (cave system) | Stanton | Combat | 5 | Underground cave encounter; antibody-rich eyes valued by researchers |
| **Yormandi Tongue** | ASD Onyx Facility (cave system) | Stanton | Combat | 5 | Same source as eyes; prized for culinary use |
| **Quasi Grazer Tongue** | Terra III (Quasi), terraformed planets | Terra/Stanton | Hunting | 3 | Native to Quasi on Terra III; "space cow" found on most terraformed worlds |
| **Quasi Grazer Egg (various)** | Terra III, other planets | Terra/Stanton | Hunting | 3 | Desert, Boreal, Grassland variants on different biomes |

### Plan 02-02: Mining/Material Items

| Item | Source Location | System | Method | Reliability | Notes |
|------|----------------|--------|--------|-------------|-------|
| **Carinite** | Aberdeen, Daymar (Hathor caves) | Stanton | Mining | 5 | VERIFIED: Caves exposed by Hathor orbital laser platforms; deep-red mineral; used in advanced processors; Size 5 commodity (16 SCU) |
| **Carinite (Pure)** | Aberdeen, Daymar | Stanton | Mining | 2 | NOT VERIFIED: Wiki does not mention pure variant; needs gameplay validation |
| **Quantanium** | Various asteroid fields | Stanton/Pyro | Mining | 4 | Ship mining; volatile; exchange rate needs validation |
| **Jaclium (Ore)** | ARC-L1, ARC-L2, asteroid fields | Stanton | Mining | 3 | Rare metal for strong alloys; asteroid or ground deposits |
| **Saldynium (Ore)** | Asteroid fields | Stanton | Mining | 3 | Rare; high-powered laser production; specific environmental formation |
| **Copper** | Various | Stanton/Pyro | Mining | 5 | Common ore |
| **Tungsten** | Various | Stanton/Pyro | Mining | 5 | Common ore |
| **Corundum** | Various | Stanton/Pyro | Mining | 4 | Gem mining |
| **Atlasium** | Various | Stanton/Pyro | Mining | 3 | Rare ore |
| **Janalite** | Various | Stanton/Pyro | Mining | 3 | Rare ore |

### Plan 02-03: Mission/Loot/Equipment Items

| Item | Source Location | System | Method | Reliability | Notes |
|------|----------------|--------|--------|-------------|-------|
| **MG Scrip** | Mercenary Guild contracts | Stanton/Pyro | Mission | 5 | VERIFIED: Awarded for MG contract completion; traded for Wikelo Favors; specific mission amounts need gameplay validation |
| **Council Scrip** | Unknown source | Stanton/Pyro | Combat | 2 | PARTIAL: Wiki confirms Wikelo trade but source unclear; Ace Pilot drops unconfirmed; needs gameplay validation |
| **Polaris Bit** | Wikelo contracts (Quantanium delivery) | Stanton | Mission | 4 | VERIFIED: Earned via Wikelo Quantanium delivery contracts; required for Polaris ship purchase; exchange rate unconfirmed |
| **ASD Secure Drive** | ASD Onyx Facility (Jorrit dossier missions) | Stanton | Mission | 5 | VERIFIED: Investigation contracts (Power Usage, Energy Anomaly, Security, Seismic Data); delivered to home location |
| **DCHS-05 Orbital Positioning Comp-Board** | Unknown | Unknown | Mission/Loot | 1 | LOW CONFIDENCE: Likely mission reward or hostile outpost loot; needs gameplay validation |
| **Ace Interceptor Helmet** | Ace Pilot loot drops | Stanton/Pyro | Combat | 2 | LOW CONFIDENCE: Same source as Council Scrip; needs gameplay validation |
| **Tevarin War Service Marker (Pristine)** | Hostile outposts (small square boxes) | Stanton | Salvage | 2 | LOW CONFIDENCE: Rare collectible; random loot; needs gameplay validation |
| **Government Cartography Agency Medal (Pristine)** | Hostile outposts | Stanton | Salvage | 2 | LOW CONFIDENCE: Rare collectible; random loot; needs gameplay validation |
| **UEE 6th Platoon Medal (Pristine)** | Hostile outposts | Stanton | Salvage | 2 | LOW CONFIDENCE: Rare collectible; random loot; needs gameplay validation |
| **Large Artifact Fragment (Pristine)** | Abandoned outposts (Hurston) | Stanton | Salvage | 2 | LOW CONFIDENCE: Hadasian artifact; rare spawn; needs gameplay validation |
| **Vanduul Plating** | Vanduul tech smugglers mission | Stanton | Combat | 3 | Combat loot from Vanduul encounters |
| **Vanduul Metal** | Vanduul tech smugglers mission | Stanton | Combat | 3 | Same source as plating |
| **Wikelo Favor** | Wikelo contracts | Stanton | Mission | 5 | VERIFIED: Primary Wikelo currency; MG Scrip trades for Favors at Wikelo Emporiums |
</item_source_findings>

<data_sources>
## Data Sources for Research

### Primary Sources (HIGH confidence)

| Source | URL | Use For |
|--------|-----|---------|
| Star Citizen Wiki | https://starcitizen.tools | Item descriptions, creature info, location names |
| Wikelo main page | https://starcitizen.tools/Wikelo | Contract list, item requirements |
| Individual item pages | https://starcitizen.tools/{ItemName} | Detailed item info |

### Community Tools (MEDIUM-HIGH confidence)

| Tool | URL | Use For |
|------|-----|---------|
| Wikelo Trades | https://wikelotrades.com | Contract tracking, item requirements |
| Universal Item Finder | https://finder.cstone.space | Precise spawn locations |
| Regolith | https://regolith.rocks | Mining locations (crowd-sourced) |
| UEX Corp | https://uexcorp.space | Trade data, market prices |

### Research Methods for Each Plan

**02-01 (Creature Items):**
- Creature wiki pages (Valakkar, Kopion, Yormandi, Quasi Grazer)
- Spawn location guides from community hub
- finder.cstone.space for precise OM coordinates

**02-02 (Mining Items):**
- Mineral wiki pages
- regolith.rocks for current mining location data
- Hathor mining facility documentation

**02-03 (Mission/Loot Items):**
- Mission wiki pages (MG contracts, Hackrow Agency)
- Loot table documentation (community-sourced)
- finder.cstone.space for hostile outpost locations
</data_sources>

<location_mappings>
## Location Name Mappings

For integration with existing route/terminal system:

### Creature Hunting Locations
| Wiki Name | Suggested Terminal Name | System |
|-----------|------------------------|--------|
| Pyro I / Monox | `pyro_i_surface` | Pyro |
| microTech OM-5/OM-13-18 | `microtech_surface` | Stanton |
| ASD Onyx Facility Site B | `asd_onyx_site_b` | Stanton |
| Aberdeen (Hathor caves) | `aberdeen_hathor` | Stanton |
| Daymar (Hathor caves) | `daymar_hathor` | Stanton |

### Mining Locations
| Wiki Name | Suggested Terminal Name | System |
|-----------|------------------------|--------|
| Aberdeen | `aberdeen_surface` | Stanton |
| Daymar | `daymar_surface` | Stanton |
| ARC-L1 | `arc_l1` | Stanton |
| ARC-L2 | `arc_l2` | Stanton |

### Mission/Loot Locations
| Wiki Name | Suggested Terminal Name | System |
|-----------|------------------------|--------|
| Wikelo Emporium Kinga | `wikelo_kinga` | Stanton |
| Wikelo Emporium Dasi | `wikelo_dasi` | Stanton |
| Wikelo Emporium Selo | `wikelo_selo` | Stanton |
| Hostile outposts (various) | `hostile_outpost_*` | Stanton |
</location_mappings>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Wiki Data Staleness
**What goes wrong:** Wiki data may be outdated if patch changed spawn locations
**Why it happens:** Game updates frequently, wiki volunteers may lag
**How to avoid:** Cross-reference with community hub posts dated after latest patch; note patch version in source data
**Warning signs:** Location names don't match in-game, items not found at documented locations

### Pitfall 2: Location Specificity Variance
**What goes wrong:** Some items have precise locations (OM markers), others are vague ("hostile outposts")
**Why it happens:** Different items have different spawn mechanisms (fixed vs random)
**How to avoid:** Accept varying reliability levels; flag low-specificity sources appropriately
**Warning signs:** Reliability rating doesn't match actual findability

### Pitfall 3: Pyro vs Stanton System Confusion
**What goes wrong:** Same creature (Valakkar) appears in multiple systems with different variants
**Why it happens:** Game added Pyro; some items are system-specific
**How to avoid:** Always note system in source location; distinguish Irradiated (Pyro) vs standard variants
**Warning signs:** Items listed without system context

### Pitfall 4: Missing Rare Item Sources
**What goes wrong:** Can't find definitive sources for rare items (medals, artifacts)
**Why it happens:** Random loot tables, community hasn't fully documented
**How to avoid:** Use general categories ("hostile outposts") with low reliability; mark as needing validation
**Warning signs:** No community consensus on source location
</common_pitfalls>

<contract_categories>
## Wikelo Contract Categories (for reference)

From wiki research, contracts fall into these categories:

| Category | Example Contract | Key Items | Primary Sources |
|----------|-----------------|-----------|-----------------|
| **Currencies** | Trade Merc Scrip for Favors | MG Scrip, Council Scrip | MG missions, Ace Pilots |
| **Weapons** | Yormandi Gun | Yormandi parts, creature parts | Yormandi boss, hunting |
| **Armor** | Walk in danger. Look good | Valakkar parts, Scrip | Pyro hunting, missions |
| **Ships** | Now make Polaris | Polaris Bit, Carinite, drives | Mining, missions, combat |
| **Vehicles** | Make ATLS shoot | Various components | Mixed sources |

**High-Value Contracts (Ship rewards) require:**
- Large quantities of Polaris Bits (Quantanium mining)
- Rare creature parts (Apex Valakkar)
- Mission currencies (high volume)
- Multiple DCHS-05 boards
- Rare collectibles (medals, artifacts)
</contract_categories>

<data_structure_recommendations>
## Data Structure Recommendations

### Item ID Convention
Use snake_case with specificity:
- `irradiated_valakkar_fang_apex`
- `irradiated_valakkar_fang_adult`
- `irradiated_valakkar_fang_juvenile`
- `tundra_kopion_horn`
- `carinite`
- `carinite_pure`
- `mg_scrip`

### Reliability Scale (already in types.rs)
1 = Unverified/rumored
2 = Community reported, sparse data
3 = Multiple community reports
4 = Wiki documented with some specificity
5 = Definitive location with coordinates

### Source Location Granularity
- Surface locations: `{planet}_surface` or `{moon}_surface`
- Specific facilities: `{facility_name}`
- Orbital markers: Include in notes field (e.g., "Near OM-5")
- Space stations: Full station name
</data_structure_recommendations>

<open_questions>
## Open Questions

1. **DCHS-05 Orbital Positioning Comp-Board source**
   - What we know: Required for high-value contracts
   - What's unclear: Specific mission or loot source
   - Recommendation: Mark as low reliability, general "mission_reward" source

2. **Irradiated Kopion Horn vs standard Kopion Horn**
   - What we know: Irradiated variant exists for Valakkar
   - What's unclear: Whether Irradiated Kopion is separate item or same as Tundra Kopion
   - Recommendation: Research during implementation; may be Pyro variant

3. **Hostile outpost precise locations**
   - What we know: Rare items spawn in small square boxes at hostile outposts
   - What's unclear: Which specific outposts have which items
   - Recommendation: Use finder.cstone.space during implementation for specific coordinates
</open_questions>

<sources>
## Sources

### Primary (HIGH confidence)
- https://starcitizen.tools/Wikelo - Contract list, requirements overview
- https://starcitizen.tools/Valakkar - Creature info, Pyro locations
- https://starcitizen.tools/Tundra_kopion - microTech spawn data
- https://starcitizen.tools/Yormandi - ASD facility boss info
- https://starcitizen.tools/Carinite - Mining locations (Aberdeen/Daymar)
- https://starcitizen.tools/MG_Scrip - Mission reward info
- https://starcitizen.tools/Council_Scrip - Ace Pilot loot info
- https://starcitizen.tools/Polaris_Bit - Quantanium trade info

### Secondary (MEDIUM confidence)
- https://wikelotrades.com - Contract tracking (community tool)
- https://finder.cstone.space - Item spawn locations (crowd-sourced)
- https://regolith.rocks - Mining location data (crowd-sourced)

### Tertiary (LOW confidence - needs validation)
- Community hub posts for precise spawn coordinates
- Reddit/Spectrum posts for rare item locations
</sources>

<metadata>
## Metadata

**Research scope:**
- Core data: Wiki item source documentation
- Ecosystem: Community tracking tools
- Patterns: Data extraction from semi-structured wiki pages
- Pitfalls: Data staleness, location specificity variance

**Confidence breakdown:**
- Creature items: HIGH - well-documented on wiki
- Mining items: HIGH - wiki + regolith.rocks
- Mission currencies: HIGH - wiki with mission details
- Rare loot items: MEDIUM - less precise documentation
- Equipment sources: MEDIUM - some gaps in wiki coverage

**Research date:** 2026-01-15
**Valid until:** 2026-02-15 (30 days - game data changes with patches)
</metadata>

---

*Phase: 02-item-source-research*
*Research completed: 2026-01-15*
*Ready for planning: yes*
