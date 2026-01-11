# Architecture

**Analysis Date:** 2026-01-11

## Pattern Overview

**Overall:** Monolithic Workspace with Multiple Specialized Crates

**Key Characteristics:**
- 8 workspace crates organized by functional domain
- Clean dependency hierarchy (no circular dependencies)
- Shared async runtime (Tokio) across all crates
- Multiple entry points (CLI commands, TUI dashboard, REST API)

## Layers

```
┌────────────────────────────────────────────────────────┐
│              PRESENTATION LAYER                         │
│  ┌──────────────────┬──────────────────────────────┐   │
│  │  CLI Binary      │    TUI Dashboard             │   │
│  │  (Commands)      │    (Interactive Analysis)    │   │
│  │  `crates/cli`    │    (Ratatui-based)           │   │
│  └──────────────────┴──────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Data Viewer TUI (PostgreSQL Browser)            │   │
│  │  `crates/data-viewer`                            │   │
│  └──────────────────────────────────────────────────┘   │
├────────────────────────────────────────────────────────┤
│              SERVICE LAYER (Business Logic)             │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Target Analyzer & Ship Registry                 │   │
│  │  `crates/intel` - Prediction & Analysis          │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Route Graph & Pathfinding                       │   │
│  │  `crates/route-graph` - Spatial Calculation      │   │
│  └──────────────────────────────────────────────────┘   │
├────────────────────────────────────────────────────────┤
│              API LAYER                                   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  REST API Server                                 │   │
│  │  `crates/server` - Axum-based HTTP endpoints     │   │
│  └──────────────────────────────────────────────────┘   │
├────────────────────────────────────────────────────────┤
│              DATA ACCESS LAYER                           │
│  ┌──────────────┬──────────────────┬──────────────┐    │
│  │  API Clients │  Data Extractor  │  DB Schema   │    │
│  │  (External)  │  (Game Data)      │  (Diesel)    │    │
│  │  `api-client`│  `sc-data-*`      │              │    │
│  └──────────────┴──────────────────┴──────────────┘    │
├────────────────────────────────────────────────────────┤
│              EXTERNAL DATA SOURCES                       │
│  • StarCitizen API (starcitizen-api.com)               │
│  • UEX Market API (uexcorp.space)                       │
│  • FleetYards API (fleetyards.net)                      │
│  • SCLogistics Game Data (XML/JSON)                     │
│  • PostgreSQL Database (Medallion Architecture)         │
└────────────────────────────────────────────────────────┘
```

**Presentation Layer:**
- Purpose: User-facing interfaces (CLI, TUI, REST)
- Contains: Command handlers, views, event loops
- Depends on: Service layer for business logic
- Used by: End users

**Service Layer:**
- Purpose: Core business logic and analysis
- Contains: `TargetAnalyzer`, `ShipRegistry`, `RouteGraph`
- Location: `crates/intel/`, `crates/route-graph/`
- Depends on: Data access layer
- Used by: Presentation layer

**API Layer:**
- Purpose: HTTP REST API endpoints
- Contains: Route handlers, AppState
- Location: `crates/server/src/routes.rs`
- Depends on: Service layer
- Used by: External clients, CLI `serve` command

**Data Access Layer:**
- Purpose: External data fetching and persistence
- Contains: API clients, database operations
- Location: `crates/api-client/`, `crates/data-viewer/src/db.rs`
- Depends on: External services
- Used by: Service layer

## Data Flow

**Hot Routes Analysis (CLI Command):**
1. User runs: `sc-interdiction routes --limit 10`
2. Load ShipRegistry (cached from FleetYards)
3. UexClient → fetch_trade_routes()
4. TargetAnalyzer::get_hot_routes()
   - Filter profitable routes
   - Estimate ship and cargo capacity
   - Calculate risk scores
5. Output as table or JSON

**Target Prediction (TUI/CLI):**
1. User runs: `sc-interdiction intel "Port Olisar"`
2. Fetch trade_routes from UexClient
3. TargetAnalyzer::predict_targets_at()
   - Filter routes where location is origin/destination
   - Classify as ARRIVING/DEPARTING
   - Estimate likely ship and cargo value
4. Render as interactive table

**REST API Request:**
1. Client: `GET /api/routes/hot?limit=20`
2. Axum route handler extracts State(AppState)
3. analyzer.get_hot_routes(20)
4. JSON serialize → HTTP response

**State Management:**
- File-based: `.planning/` directory for GSD state
- In-memory: Arc<RwLock<RouteGraph>> for shared graph
- Database: PostgreSQL with Diesel ORM (medallion architecture)

## Key Abstractions

**TargetAnalyzer:**
- Purpose: Core prediction engine
- Location: `crates/intel/src/targets.rs`
- Methods: `get_hot_routes()`, `predict_targets_at()`, `find_interdiction_points()`
- Pattern: Stateless service consuming UexClient + ShipRegistry

**ShipRegistry:**
- Purpose: Ship database with cargo/fuel lookup
- Location: `crates/intel/src/ships/registry.rs`
- Methods: `find_by_name()`, `estimate_for_route()`
- Pattern: Singleton loaded once, cloned via Arc

**RouteGraph:**
- Purpose: Graph of terminals connected by quantum travel
- Location: `crates/route-graph/src/graph.rs`
- Methods: `add_terminal()`, `find_path()`, `connect_system()`
- Pattern: Wrapper around petgraph DiGraph

**AppState:**
- Purpose: Shared state container for REST API
- Location: `crates/server/src/state.rs`
- Contains: Arc'd services (analyzer, graph, registry, clients)
- Pattern: Cloneable container for Axum handlers

## Entry Points

**CLI Entry:**
- Location: `crates/cli/src/main.rs` (744 lines)
- Triggers: User runs `sc-interdiction <command>`
- Commands: serve, routes, runs, chokepoints, intel, ships, dashboard, etc.

**TUI Dashboard:**
- Location: `crates/cli/src/tui/mod.rs`
- Triggers: `sc-interdiction dashboard [--location X]`
- Event loop: crossterm events with 250ms tick rate

**Data Viewer:**
- Location: `crates/data-viewer/src/main.rs`
- Triggers: `data-viewer [--url DATABASE_URL]`
- Purpose: Browse PostgreSQL medallion-architecture data

**Import Tool:**
- Location: `crates/sc-logistics-importer/src/main.rs`
- Triggers: `sc-logistics-importer all|starmap|shops|stats`
- Purpose: Import SCLogistics game data into PostgreSQL

## Error Handling

**Strategy:** Throw errors via Result, catch at boundaries

**Patterns:**
- Custom errors via thiserror (`GraphError`, `ApiError`)
- Error context via eyre for CLI commands
- HTTP error responses with status codes in REST API
- User-friendly messages in TUI with status bar

## Cross-Cutting Concerns

**Logging:**
- Tracing framework with env-filter
- Structured logging in service layer
- Console output in CLI commands

**Caching:**
- Dashmap for in-memory API response caching
- File-based caching for FleetYards ship data
- No external cache service (Redis not used)

**Validation:**
- Input validation at API boundaries
- Type-safe parsing with serde
- Graceful fallbacks for missing data

---

*Architecture analysis: 2026-01-11*
*Update when major patterns change*
