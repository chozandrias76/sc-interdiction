# External Integrations

**Analysis Date:** 2026-01-11

## APIs & External Services

**Star Citizen API (starcitizen-api.com):**
- Purpose: Starmap data, system info, station details
- SDK/Client: `ScApiClient` in `crates/api-client/src/sc_api.rs`
- Auth: API key via `SC_API_KEY` env var
- Base URL: `https://api.starcitizen-api.com`
- Endpoints used:
  - `/{api_key}/cache/starmap/star-system?code={code}` - Get system by code
  - `/{api_key}/cache/starmap/systems` - Get all systems
  - `/{api_key}/cache/starmap/search?name={query}` - Search starmap objects
- Status: Verified in `docs/DATA_SOURCES.md`

**UEX Corporation API (uexcorp.space):**
- Purpose: Commodity prices, trade routes, terminals
- SDK/Client: `UexClient` in `crates/api-client/src/uex.rs`
- Auth: None required (public API)
- Base URL: `https://uexcorp.space/api/2.0`
- Endpoints used:
  - `/commodities` - Get all commodities
  - `/commodities_prices?code={code}` - Get commodity prices at terminals
  - `/terminals` - Get all trade terminals
  - `/trade_routes` - Trade route data
- Caching: In-memory via Dashmap
- Testing: Mockito for HTTP mocking
- Status: Verified in `docs/DATA_SOURCES.md`

**FleetYards API (fleetyards.net):**
- Purpose: Ship specifications, cargo capacity, fuel tanks
- SDK/Client: `FleetYardsClient` in `crates/api-client/src/fleetyards.rs`
- Auth: None required (public API)
- Base URL: `https://api.fleetyards.net/v1`
- Endpoints used:
  - `/models?page={page}&perPage=100` - Get ship models with pagination
- Caching: File-based with version tracking
- Rate Limiting: Handles 429 responses with retry_after
- Status: Verified in `docs/DATA_SOURCES.md`

## Data Storage

**Databases:**
- PostgreSQL 16 (Alpine) - Primary data store
- Connection: `DATABASE_URL` env var
- Client: Diesel ORM 2.2 with r2d2 connection pooling
- Used by: `crates/data-viewer/`, `crates/sc-data-extractor/`, `crates/sc-logistics-importer/`

**Database Schema:**
- Migrations: Diesel migrations in `crates/sc-data-extractor/migrations/`
- Architecture: Medallion (raw → silver → gold)
- Schemas:
  - `raw` - Extracted game data
  - `staging` - Views for transformation
  - `silver` - Cleaned tables
  - `gold` - Analytics views

**File Storage:**
- FleetYards ship cache: Local file with version tracking
- SCLogistics data: External repository (`$SCLOGISTICS_PATH`)

**Caching:**
- In-memory: Dashmap 6.0 for API response caching
- No external cache (Redis not used)

## Authentication & Identity

**Auth Provider:**
- None - No user authentication required
- API keys for external services only

**API Keys:**
- `SC_API_KEY` - Star Citizen API (optional, some endpoints)
- No OAuth integrations

## Monitoring & Observability

**Error Tracking:**
- None - No external error tracking service
- Errors logged via tracing

**Analytics:**
- None - No analytics service

**Logs:**
- Tracing framework with env-filter
- Stdout logging only
- No external log aggregation

## CI/CD & Deployment

**Hosting:**
- Local development only
- No cloud deployment configured

**CI Pipeline:**
- Pre-commit hooks for linting/testing
- Coverage check (80% minimum)
- No external CI service configured

## Environment Configuration

**Development:**
- Required: `DATABASE_URL` (PostgreSQL connection)
- Optional: `SCLOGISTICS_PATH` (game data location)
- Optional: `SC_API_KEY` (Star Citizen API access)
- Secrets: `.env` file (gitignored), `.env.example` template

**Docker Services:**
- PostgreSQL 16 - `docker-compose.yml`
- dbt 1.7.0 - Data transformation container
- Volumes: `pgdata` for database persistence

**Database Setup:**
```bash
make db-up       # Start PostgreSQL container
make db-migrate  # Run Diesel migrations
make db-import   # Import SCLogistics data
make dbt-all     # Run dbt transformations
```

## Data Import Pipeline

**SCLogistics Repository:**
- Source: External repository (configure via `$SCLOGISTICS_PATH`)
- Data files:
  - `starmap/` - XML starmap data
  - `Shops/shopinventories/` - JSON shop inventory
- Processing: `crates/sc-data-extractor/build.rs` (compile-time)
- Import: `sc-logistics-importer` CLI tool

**dbt Transformations:**
- Profile: `dbt/profiles.yml`
- Project: `dbt/dbt_project.yml`
- Models: `dbt/models/{staging,silver,gold}/`
- Run: `make dbt-run` or via Docker

## Webhooks & Callbacks

**Incoming:**
- None

**Outgoing:**
- None

---

*Integration audit: 2026-01-11*
*Update when adding/removing external services*
