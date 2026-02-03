---
subsystem: cli
tags: [cli, wikelo, item-lookup, demand]
provides: [item-cli-command, item-source-lookup, item-contract-lookup]
affects: [crates/cli]
key-files:
  - crates/cli/src/main.rs
duration: PT4M
---

# 12-01 SUMMARY: Item CLI Command with Source and Contract Lookup

Item CLI command with source/contract lookup and --list flag.

## Completed Tasks

| Task | Commit | Description |
|------|--------|-------------|
| Task 1 | 554ae1a | Add Item subcommand to CLI with source and contract lookup |
| Task 2 | 554ae1a | Add search improvements and --list flag (combined with Task 1) |

## Implementation Details

### Task 1 & 2: Item CLI Command

Added new `item` subcommand to the CLI with complete functionality:

**Command Structure:**
- `sc-interdiction item <query>` - Search items by name
- `sc-interdiction item --list` - List all known Wikelo items
- `sc-interdiction item <query> --json` - JSON output mode

**Features Implemented:**
- Case-insensitive substring matching on item names
- Displays item name, category, and estimated value
- Shows source locations with system and acquisition method
- Lists contracts requiring each item with turn-in locations
- Both text and JSON output modes
- Helpful "no matches found" messaging with tip to use --list
- Multiple match handling (lists all matches with details)

**Output Format (Text):**
```
================================================================================
 FOUND 4 ITEMS
================================================================================

Item: Irradiated Valakkar Fang (Juvenile)
Category: CreaturePart, Value: unknown

Sources:
  - Monox (Pyro) [Hunting]

Used in Contracts:
  - Walk in danger. Look good (turn-in: Wikelo Emporium Dasi, ...)
--------------------------------------------------------------------------------
```

**JSON Output Structure:**
```json
{
  "name": "Irradiated Valakkar Fang (Juvenile)",
  "category": "CreaturePart",
  "estimated_value": null,
  "sources": [{"location": "Monox", "system": "Pyro", "method": "Hunting"}],
  "contracts": [{"name": "Walk in danger. Look good", "turn_in_locations": [...]}]
}
```

## Files Modified

| File | Changes |
|------|---------|
| `crates/cli/src/main.rs` | Added Item command enum variant, handle_item() function, ItemSearchResult/SourceInfo/ContractInfo structs, print_items_list(), print_item_results(), format_number(), truncate_str() helpers |

## Verification Results

- `cargo build --package sc-interdiction` - PASSED
- `cargo clippy --package sc-interdiction` - PASSED (4 pre-existing warnings in TUI code)
- `cargo test --package sc-interdiction` - PASSED (55 tests)
- `sc-interdiction item --help` - Shows usage
- `sc-interdiction item "valakkar"` - Shows 4 matching items with sources and contracts
- `sc-interdiction item --list` - Shows all 31 known items
- `sc-interdiction item "valakkar" --json` - Outputs valid JSON
- `sc-interdiction item "xyz"` - Shows "no items found" message

## Deviations

None. Tasks 1 and 2 were implemented together in a single commit as the --list flag and helpful messaging were natural additions during the initial implementation.
