# Phase 8: Demand Data Model - Context

**Gathered:** 2026-01-29
**Status:** Ready for planning

<vision>
## How This Should Work

The demand data model should capture the full item economy — not just "contract X needs item Y" but the complete value chain. This means modeling reward structures (MG Scrip amounts, Favor conversion rates) alongside item requirements so the system can reason about item value holistically.

The model should answer two key questions:
1. **Item value ranking** — "Which items are most valuable to collect?" by factoring in how many contracts want an item, rarity, and reward value
2. **Contract optimization** — "Which contracts give the best return?" by comparing effort-to-reward ratios across contracts, factoring in item availability

This builds on the existing supply-side data (where items come from) by adding the demand side (who wants them and what they'll pay).

</vision>

<essential>
## What Must Be Nailed

- **Extensibility** — The types must not be Wikelo-specific. The demand model should be generic enough that mission givers, faction reward systems, and other demand sources can plug in later without restructuring
- **Reward modeling** — MG Scrip amounts, Favor conversion rates (50 MG Scrip → 1 Favor), and whatever else is needed to calculate actual item/contract value
- **Economy reasoning** — The type system should make it natural to compute item value rankings and contract effort-to-reward comparisons

</essential>

<boundaries>
## What's Out of Scope

- Live data or API calls — all static data, same as the supply side
- UI or display concerns — that's Phase 12
- Actual data population — define the types but don't fill them with real contract data (that's Phase 9)
- Registry/lookup structures — just types and structs, registry is Phase 10

</boundaries>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches. The existing codebase patterns (WikieloItem, ItemSource, WikieloRegistry) can inform the design but shouldn't constrain it if a better pattern serves extensibility.

</specifics>

<notes>
## Additional Context

Phase 7 research produced data on all 43 Wikelo contracts (07-RESEARCH.md), though 29/43 have incomplete requirement data (names and categories known, exact item quantities TBD). The type system should accommodate partial/incomplete contract data gracefully.

The economy has a clear value chain: items → contracts → MG Scrip → Favors. Modeling this chain explicitly will enable the value ranking and optimization queries the user wants.

</notes>

---

*Phase: 08-demand-data-model*
*Context gathered: 2026-01-29*
