# Phase 9: Contract Data Population - Context

**Gathered:** 2026-02-09
**Status:** Ready for planning

<vision>
## How This Should Work

Contract data lives in TOML files that get loaded at runtime or build time. The 43 Wikelo contracts are defined in a structured, human-editable format that's easy to update when game patches change things.

This is a data population phase — take the types from Phase 8 and fill them with actual contract data. The TOML approach keeps the data separate from code, making it maintainable and easy to audit.

</vision>

<essential>
## What Must Be Nailed

- **Complete coverage** - All 43 contracts populated, even if some fields need DataConfidence markers for incomplete data
- **TOML data format** - Human-readable, Rust-idiomatic, easy to edit manually

</essential>

<boundaries>
## What's Out of Scope

- Registry/indexing work — that's Phase 10 (ContractRegistry with bidirectional lookups)
- The TOML files just hold raw data; Phase 10 builds the index structures

</boundaries>

<specifics>
## Specific Ideas

- Research to fill gaps: 29 of 43 contracts have incomplete requirement data from 07-RESEARCH.md. This phase includes wiki scraping or additional research to complete the data rather than leaving fields empty.
- Use DataConfidence enum from Phase 8 to mark any fields that remain uncertain after research

</specifics>

<notes>
## Additional Context

Phase 8 defined: ContractCategory, DataConfidence, CurrencyType, ExchangeRate. WikieloContract was extended with 7 fields. RewardType extended with 5 variants. These are the types we're populating.

The 07-RESEARCH.md contains the initial contract research. Additional wiki scraping may be needed to fill the 29 contracts with incomplete requirement data.

</notes>

---

*Phase: 09-contract-data-population*
*Context gathered: 2026-02-09*
