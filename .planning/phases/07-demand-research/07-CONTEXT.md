# Phase 7: Demand Research - Context

**Gathered:** 2026-01-27
**Status:** Ready for research

<vision>
## How This Should Work

Map the full item economy demand side — not just Wikelo contracts, but all NPC demand sources. For any item in the game, we should know who wants it and where to deliver it.

This complements the existing supply-side intelligence (where items come from) with demand-side intelligence (who wants items). Together, this lets us flag ships both leaving source locations AND heading to demand locations.

The research should cover:
- Wikelo contracts (item requirements and quantities)
- Mission giver requirements (Miles Eckhart, Ruto, etc.)
- Faction reward systems (MG Scrip, Council Favor, exchange rates)

</vision>

<essential>
## What Must Be Nailed

- **Item→demand mapping** — For any item, know who wants it and where to deliver it. This is the core deliverable that everything else builds on.

</essential>

<boundaries>
## What's Out of Scope

- Future factions — focus on current systems (MG, Council, Wikelo) only
- Mission lootables — items you can pick up at mission sites don't count
- Player-to-player trading — NPC demand only
- Just specified rewards — official contract/mission requirements, not incidental loot

</boundaries>

<specifics>
## Specific Ideas

- **wikelotrades.com scraping** — pull contract data directly from the fan site
- **Wiki + game files** — cross-reference starcitizen.tools with Data.p4k, same approach as Phase 2
- **UEX as a source** — they show where you can buy items for mission turn-ins

</specifics>

<notes>
## Additional Context

This builds on the Phase 2 research approach that worked well — wiki research cross-referenced with game file extraction. The difference is we're mapping demand (who wants items) instead of supply (where items come from).

Exchange rates (50 MG Scrip → 1 Favor) are useful context for understanding relative item values across faction systems.

</notes>

---

*Phase: 07-demand-research*
*Context gathered: 2026-01-27*
