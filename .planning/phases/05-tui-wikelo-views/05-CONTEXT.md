# Phase 5: TUI Wikelo Views - Context

**Gathered:** 2026-01-21
**Status:** Ready for planning

<vision>
## How This Should Work

Wikelo intel should be **prominent** in the existing TUI — dedicated columns and sections, not just subtle indicators. The TUI surfaces all the Wikelo intelligence gathered in Phase 4 so interdictors can make decisions quickly.

**Targets view:** A Wikelo score column for quick scanning, plus detailed item info when selecting or expanding a target. Score tells you at a glance how valuable a target is from a Wikelo perspective; drill in for specifics.

**Map view:** Wikelo source locations are highlighted by default (visual indicator on existing markers). Hovering shows which Wikelo items come from that location via tooltip. A filter/toggle mode lets you isolate just Wikelo sources, hiding other noise.

**Hotspots view:** Same pattern as targets — Wikelo score column for quick scanning, expandable breakdown showing which items are likely at each hotspot.

**Item details:** When drilling into any Wikelo info (on target, location, or hotspot), show full item cards with name, category, source type, and value/price indicators where available.

</vision>

<essential>
## What Must Be Nailed

- **Spatial awareness** — Knowing which locations on the map produce Wikelo items. This is positioning intel — where to wait for targets.
- **Detail on demand** — Being able to drill into exactly what Wikelo items a target/location/hotspot involves. Full item cards, not just names.

The quick-scan score column is useful but secondary to these two priorities.

</essential>

<boundaries>
## What's Out of Scope

- **Live price fetching** — Not implementing new UEX API calls for Wikelo pricing. Use existing UexClient data where available.
- Phase 5 is about surfacing existing intel in the TUI, not gathering new data.

</boundaries>

<specifics>
## Specific Ideas

- Tooltip on hover for map locations (shows which Wikelo items come from there)
- Full item cards when drilling in: name, category, source type, price if available
- Use existing `api-client` crate's `UexClient` for price data where commodities map to Wikelo items
- Filter/toggle on map to show only Wikelo source locations
- Score + expandable breakdown pattern for both targets and hotspots views

</specifics>

<notes>
## Additional Context

Pricing comes from existing UEX integration in `crates/api-client/src/uex.rs`. The UexClient already provides commodity prices — Phase 5 should display these for Wikelo items where the mapping exists.

Key files from Phase 4:
- WikieloIntel: `crates/intel/src/wikelo/intel.rs`
- WikieloRegistry: `crates/intel/src/wikelo/registry.rs`
- TargetAnalyzer: `crates/intel/src/targets.rs` (has wikelo_score, wikelo_potential)
- Wikelo types: `crates/intel/src/wikelo/types.rs`

</notes>

---

*Phase: 05-tui-wikelo-views*
*Context gathered: 2026-01-21*
