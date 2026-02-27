# api-client

HTTP clients for Star Citizen external data sources.

## Modules

| File             | Purpose                                          |
|------------------|--------------------------------------------------|
| `uex.rs`         | UEX Corp API -- commodities, prices, terminals, trade routes |
| `sc_api.rs`      | starcitizen-api.com -- starmap, systems, stations |
| `fleetyards.rs`  | FleetYards.net -- ship specs, fuel, cargo capacity |
| `error.rs`       | Shared `ApiError` enum (thiserror)               |

## External APIs

| API            | Base URL                                    | Auth            |
|----------------|---------------------------------------------|-----------------|
| UEX Corp 2.0   | `https://uexcorp.space/api/2.0`            | None (public)   |
| SC API         | `https://api.starcitizen-api.com`           | API key in path |
| FleetYards v1  | `https://api.fleetyards.net/v1`            | None (public)   |

Key endpoints consumed:
- UEX: `/commodities`, `/commodities_prices`, `/commodities_prices_all`, `/terminals`
- SC API: `/{key}/cache/starmap/star-system`, `/{key}/cache/starmap/systems`, `/{key}/cache/starmap/search`
- FleetYards: `/models` (paginated, 100/page, safety limit 20 pages)

## Caching

- **FleetYards** has file-based cache via `FleetYardsClient::with_cache(path)`.
  Saves `ships.json` with version + unix timestamp. Cache expires after 24 hours (86400s).
  `refresh_cache()` forces re-fetch. `get_ships()` checks cache before hitting API.
- **UEX / SC API** have no built-in caching -- callers must handle this.

## Error Handling

All clients return `Result<T, ApiError>`. Error variants:
- `Request` -- HTTP failure (from reqwest)
- `Parse` -- JSON deserialization failure
- `Api { status, message }` -- non-success HTTP status
- `NotFound` -- 404
- `RateLimited { retry_after_secs: 60 }` -- 429

## Testing

All tests use `mockito` for HTTP mocking. Pattern:

```rust
let mut server = Server::new_async().await;
let mock = server.mock("GET", "/endpoint").with_status(200)
    .with_body(r#"{"data": [...]}"#).create_async().await;
let client = UexClient::new_with_base_url(&server.url());
// ... call client method ...
mock.assert_async().await;
```

- UEX: `new_with_base_url()` accepts mock server URL directly
- SC API: tests use a `TestScApiClient` struct (no base URL override on prod client)
- FleetYards: tests use `TestFleetYardsClient` struct; cache tests use `tempfile::TempDir`

Run tests:
```bash
cargo test -p api-client
```

## Conventions

- All async methods are `#[instrument(skip(self))]` for tracing
- Serde structs use `#[serde(default)]` liberally for optional API fields
- UEX booleans come as integers -- custom `deserialize_bool_from_int` handles both
- `TradeRoute` calculation lives in `UexClient::get_trade_routes()` (joins prices + terminals)
- `dashmap` is listed as a dependency but not currently used in source

## Watch Out For

- SC API key is embedded in the URL path, not as a header
- FleetYards pagination has a 20-page safety limit (breaks infinite loops)
- `get_trade_routes()` fetches terminals + prices concurrently via `tokio::try_join!`
- Test files are large (uex_tests: 988 lines, fleetyards_tests: 631 lines)
