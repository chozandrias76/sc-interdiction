# intel

Target analysis, ship estimation, and Wikelo intelligence for interdiction planning.

## Modules

### targets.rs
Core analysis engine. `TargetAnalyzer` wraps a `UexClient` + `ShipRegistry` and provides:

- `get_hot_routes(limit)` -- profitable routes sorted by estimated haul value
- `predict_targets_at(location)` -- arriving/departing traffic predictions with cargo estimates
- `find_interdiction_points(graph, top_n, cross_system)` -- chokepoints via `route_graph`
- `get_trade_runs(limit)` -- round-trip routes with outbound + optional return legs
- `get_interdiction_hotspots(limit)` -- locations ranked by aggregate cargo value

All methods are async and call `uex.get_trade_routes().await`. Builder pattern:
`TargetAnalyzer::new(uex, registry).with_wikelo(wikelo_intel)`.

Key helper fns: `calculate_risk_score` (profit/SCU/cargo heuristic, capped at 100),
`calculate_wikelo_score` (base 20 + 10/high-value item + 5/item, capped at 100),
`extract_system` (parses terminal name parenthesized system suffix).

### ships/ (mod.rs, types.rs, registry.rs, enrichment.rs)
Cargo ship database. Only `production_status: "flight-ready"` ships from FleetYards API.

- `ShipRegistry` -- builds from API data with fallback (Aurora CL); `estimate_for_route`
  picks best ship by cargo capacity. `from_api_ships(vec![])` creates minimal fallback.
- `CargoShip` -- cargo_scu, threat_level, fuel capacity, QT drive size, `ShipRole` enum
- `LootEstimate` -- `calculate(cargo_value, ship, destruction_level)` models recoverable
  cargo + salvage value at varying destruction percentages (0-100%)
- `ShipRole` -- Cargo, Combat, Mining, Salvage, Transport, Exploration, Support.
  Each role has a component value multiplier (0.7x Cargo to 1.4x Combat).

### wikelo/ (mod.rs, intel.rs, items.rs, types.rs, contracts.rs, registry.rs)
Wikelo item source intelligence. Tracks valuable items and their locations.

- `WikieloIntel` -- `from_static()` loads hardcoded item data; `flag_location(name)`
  returns `SourceFlag` if location is a known item source (e.g., Pyro I -> Valakkar)
- `SourceFlag` -- location name, item_count, top_items with estimated values
- `WikieloRegistry` -- item catalog with acquisition methods, exchange rates
- Contract types: `WikieloContract`, `ContractRequirement`, `ContractReward`

## Tests

```bash
cargo test -p intel
```

### Unit tests (src/targets_tests.rs, src/ships_tests.rs)
Inline `#[cfg(test)]` modules covering:
- System name extraction from terminal strings (parenthesized suffixes)
- Risk score calculation (profit + SCU + large cargo bonus)
- Wikelo scoring (base points, high-value bonuses, non-source locations)
- `LocationAggregator` accumulation, sorting, avg threat, top-5 truncation
- `LootEstimate` destruction levels, clamping, ship-size salvage scaling
- `ShipRole` multipliers, manufacturer interactions, crew-size effects
- Struct serialization round-trips via `serde_json`

### Integration tests (tests/target_analyzer_integration.rs)
Uses **mockito** to mock UEX API endpoints (`/terminals`, `/commodities_prices_all`).
Pattern: `mockito::Server::new_async().await` -> mock GET endpoints -> create
`UexClient::new_with_base_url(&server.url())`. The `ServerGuard` must stay alive
for the duration of each test.

Fixture data in `tests/fixtures.rs` provides 3 commodities (Laranite, Aluminum, Gold)
across 4 terminals (Port Olisar, Area18, Lorville, Ruin Station) spanning Stanton + Pyro.

Integration tests cover: hot routes ordering, target predictions at Port Olisar,
trade run structure validation, hotspot ranking with Wikelo-absent assertions.

## Gotchas

- `targets.rs` is 826 lines (over 500-line limit) -- `LocationAggregator` could extract
- All `TargetAnalyzer` methods hit the network via `UexClient`; always mock in tests
- Wikelo data is currently static (`from_static()`); no live API source yet
- `ShipRegistry::from_api_ships(vec![])` creates a fallback Aurora CL -- used in all tests
