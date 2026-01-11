# Codebase Structure

**Analysis Date:** 2026-01-11

## Directory Layout

```
sc-interdiction/
├── crates/                    # Workspace crates (8 members)
│   ├── api-client/           # External API clients
│   ├── cli/                  # Main CLI with TUI
│   ├── data-viewer/          # PostgreSQL browser TUI
│   ├── intel/                # Target analysis engine
│   ├── route-graph/          # Spatial routing & pathfinding
│   ├── sc-data-extractor/    # Game data extraction
│   ├── sc-logistics-importer/# Data import CLI
│   └── server/               # REST API server
├── dbt/                       # dbt data transformation
│   ├── models/               # SQL models (staging/silver/gold)
│   ├── seeds/                # Static seed data
│   └── profiles.yml          # Database connection
├── data/                      # Data files
├── docs/                      # Documentation
├── scripts/                   # Utility scripts
├── .cargo/                    # Cargo configuration
├── .planning/                 # GSD planning files
├── Cargo.toml                # Workspace manifest
├── Makefile                  # Development tasks
└── docker-compose.yml        # Dev environment
```

## Directory Purposes

**crates/api-client/**
- Purpose: Unified client for external data sources
- Contains: HTTP clients for UEX, FleetYards, StarCitizen APIs
- Key files: `src/lib.rs`, `src/uex.rs`, `src/fleetyards.rs`, `src/sc_api.rs`
- Subdirectories: None (flat structure)

**crates/cli/**
- Purpose: Main CLI application with TUI dashboard
- Contains: Command handlers, TUI views/handlers
- Key files: `src/main.rs` (744 lines - all CLI commands)
- Subdirectories: `src/tui/` (views, handlers, data, widgets)

**crates/cli/src/tui/**
- Purpose: Interactive TUI dashboard
- Contains: App state machine, views, event handlers
- Key files: `app.rs`, `ui.rs`, `event.rs`, `types.rs`
- Subdirectories: `views/`, `handlers/`, `data/`

**crates/data-viewer/**
- Purpose: Browse PostgreSQL medallion-architecture data
- Contains: Database browser TUI with Diesel queries
- Key files: `src/main.rs`, `src/app.rs`, `src/db.rs`
- Subdirectories: `src/views/`

**crates/intel/**
- Purpose: Target analysis and prediction engine
- Contains: TargetAnalyzer, ShipRegistry, enrichment logic
- Key files: `src/lib.rs`, `src/targets.rs` (676 lines)
- Subdirectories: `src/ships/` (types, registry, enrichment)

**crates/route-graph/**
- Purpose: Spatial calculation, pathfinding, chokepoints
- Contains: RouteGraph, SpatialIndex, fuel calculations
- Key files: `src/graph.rs`, `src/spatial.rs` (1116 lines), `src/fuel.rs`
- Subdirectories: None (flat structure)

**crates/sc-data-extractor/**
- Purpose: Parse Star Citizen game data, generate schemas
- Contains: Parsers, builders, compile-time code generation
- Key files: `src/lib.rs`, `build.rs` (738 lines)
- Subdirectories: `src/database/`, `src/parsers/`, `src/models/`, `migrations/`

**crates/server/**
- Purpose: REST API server with Axum
- Contains: Route handlers, shared state
- Key files: `src/lib.rs`, `src/routes.rs` (227 lines), `src/state.rs`
- Subdirectories: `tests/`, `data/`

**dbt/**
- Purpose: SQL-based data transformation pipeline
- Contains: Models organized by medallion layer
- Key files: `profiles.yml`, `dbt_project.yml`
- Subdirectories: `models/staging/`, `models/silver/`, `models/gold/`, `seeds/`

## Key File Locations

**Entry Points:**
- `crates/cli/src/main.rs` - Main CLI entry (sc-interdiction)
- `crates/data-viewer/src/main.rs` - Data viewer TUI
- `crates/sc-logistics-importer/src/main.rs` - Data import tool
- `crates/server/src/lib.rs` - REST API server

**Configuration:**
- `Cargo.toml` - Workspace manifest and shared dependencies
- `.cargo/config.toml` - Build optimization settings
- `clippy.toml` - Linting thresholds
- `docker-compose.yml` - Development PostgreSQL
- `dbt/profiles.yml` - Database connection for dbt

**Core Logic:**
- `crates/intel/src/targets.rs` - Target prediction algorithms
- `crates/intel/src/ships/registry.rs` - Ship database
- `crates/route-graph/src/graph.rs` - Route graph structure
- `crates/route-graph/src/spatial.rs` - Spatial indexing
- `crates/api-client/src/uex.rs` - UEX API client

**Testing:**
- `crates/intel/src/ships_tests.rs` - Ship tests (591 lines)
- `crates/intel/src/targets_tests.rs` - Target analysis tests
- `crates/server/tests/routes_test.rs` - API integration tests
- `crates/cli/src/tui/snapshots/` - TUI snapshot tests

**Documentation:**
- `README.md` - Project overview
- `docs/DATA_SOURCES.md` - External API documentation
- `CONTRIBUTING.md` - Development guidelines
- `TESTING.md` - Test conventions

## Naming Conventions

**Files:**
- snake_case.rs for all Rust source files
- mod.rs for module directory exports
- *_tests.rs for separate test files (e.g., `ships_tests.rs`)

**Directories:**
- snake_case for all directories
- Plural for collections: `views/`, `handlers/`, `ships/`
- Singular for single-purpose: `database/`, `src/`

**Special Patterns:**
- `lib.rs` - Library crate entry point with public exports
- `main.rs` - Binary crate entry point
- `build.rs` - Compile-time code generation

## Where to Add New Code

**New Feature:**
- Primary code: `crates/{relevant-crate}/src/`
- Tests: Same file (`#[cfg(test)]`) or `*_tests.rs`
- Integration tests: `crates/{crate}/tests/`

**New CLI Command:**
- Handler: `crates/cli/src/main.rs` (add to Commands enum)
- If complex: Extract to `crates/cli/src/commands/{name}.rs`

**New TUI View:**
- Implementation: `crates/cli/src/tui/views/{name}.rs`
- Register: `crates/cli/src/tui/views/mod.rs`
- Handler: `crates/cli/src/tui/handlers/`

**New API Client:**
- Implementation: `crates/api-client/src/{service}.rs`
- Export: `crates/api-client/src/lib.rs`

**New REST Endpoint:**
- Handler: `crates/server/src/routes.rs`
- State: `crates/server/src/state.rs` (if new dependencies)

**New Analysis Algorithm:**
- Implementation: `crates/intel/src/{name}.rs` or `crates/route-graph/src/`
- Export: Crate's `lib.rs`

## Special Directories

**.planning/**
- Purpose: GSD planning and state files
- Source: Created by /gsd commands
- Committed: Yes (tracking project state)

**dbt/target/**
- Purpose: dbt compiled artifacts
- Source: Generated by dbt run
- Committed: No (in .gitignore)

**data/**
- Purpose: Static data files for server
- Source: Manually maintained
- Committed: Yes

---

*Structure analysis: 2026-01-11*
*Update when directory structure changes*
