# Phase 3: Wikelo Data Module - Context

**Gathered:** 2026-01-15
**Status:** Ready for planning

<vision>
## How This Should Work

A central registry struct that holds all Wikelo item and source data, created at startup and passed to components that need it. No global singletons — explicit dependency passing keeps it clean and testable.

The registry supports flexible queries: look up sources for a specific item, find all items available at a location, or browse by category (creature drops, mining, missions, etc.). Both directions matter — "where can I get X" and "what's at Y" are equally important for interdiction intelligence.

</vision>

<essential>
## What Must Be Nailed

- **Bidirectional lookups** — Query item→sources AND location→items efficiently
- **Confidence tracking** — Include all data but flag confidence levels so consumers can decide how to handle uncertain sources
- **Follow existing patterns** — Match whatever conventions exist in the other crates (trade-data, intel, etc.)
- **Balanced priorities** — Fast lookups, easy to update when game changes, strong type safety. No single priority dominates.

</essential>

<boundaries>
## What's Out of Scope

- Pricing/value data — No UEX prices or value calculations in this module
- Contract tracking — This is item/source data only, not contract progress or requirements logic

</boundaries>

<specifics>
## Specific Ideas

- Registry pattern with explicit dependency injection (not global singleton or lazy static)
- Support all query patterns: single item lookup, batch by location/system, category browsing
- Follow standard Rust crate conventions for module organization and type relationships

</specifics>

<notes>
## Additional Context

Phase 2 research flagged 9 items as low-confidence due to conflicting wiki sources or outdated patch data. These should be included in the data but marked with their confidence level so consumers can make informed decisions.

The bidirectional lookup requirement stems from interdiction use cases: sometimes you're watching a location and want to know what targets might be carrying, sometimes you know what cargo you want and need to know where to intercept.

</notes>

---

*Phase: 03-wikelo-data-module*
*Context gathered: 2026-01-15*
