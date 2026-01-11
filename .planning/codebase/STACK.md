# Technology Stack

**Analysis Date:** 2026-01-11

## Languages

**Primary:**
- Rust (Edition 2021) - All application code (`Cargo.toml`)

**Secondary:**
- SQL - dbt models and migrations (`dbt/models/`, `crates/sc-data-extractor/migrations/`)
- YAML - Configuration files (`docker-compose.yml`, `dbt/profiles.yml`)

## Runtime

**Environment:**
- Rust stable (Edition 2021)
- Async runtime: Tokio 1.0 with full features
- Platform: WSL2 Linux (Windows Subsystem for Linux)

**Package Manager:**
- Cargo (workspace-based)
- Lockfile: `Cargo.lock` present (versioning: 4)

## Frameworks

**Core:**
- Axum 0.7 - REST API server (`crates/server/src/routes.rs`)
- Ratatui 0.29 - TUI rendering (`crates/cli/src/tui/`)
- Crossterm 0.28 - Terminal manipulation

**Testing:**
- Built-in `#[test]` framework
- Mockito 1.2 - HTTP mocking
- Insta 1.45.1 - Snapshot testing (`crates/cli/`)
- cargo-tarpaulin - Code coverage (80% minimum)

**Build/Dev:**
- Make - Development task automation (`Makefile`)
- Clippy - Linting with strict workspace rules
- rustfmt - Code formatting

## Key Dependencies

**Critical:**
- Diesel 2.2 - PostgreSQL ORM with r2d2 connection pooling (`crates/data-viewer/src/db.rs`)
- Reqwest 0.12 - Async HTTP client with JSON + rustls-tls (`crates/api-client/`)
- Petgraph 0.6 - Graph data structures for route analysis (`crates/route-graph/`)
- Clap 4.0 - CLI argument parsing with derive + env features (`crates/cli/src/main.rs`)

**Infrastructure:**
- Tokio 1.0 - Async runtime (`crates/server/`, `crates/cli/`)
- Tower 0.5 / Tower-HTTP 0.6 - HTTP middleware with CORS
- Dashmap 6.0 - Concurrent hashmap for caching (`crates/api-client/`)
- Serde 1.0 / Serde JSON 1.0 - Serialization

**Error Handling:**
- Thiserror 1.0 - Error type derivation
- Eyre 0.6 - Error handling and reporting

**Logging:**
- Tracing 0.1 - Structured logging
- Tracing-Subscriber 0.3 - Log formatting with env-filter

## Configuration

**Environment:**
- `.env` files via dotenvy 0.15
- Required: `DATABASE_URL` - PostgreSQL connection string
- Optional: `SCLOGISTICS_PATH` - Path to game data repository
- Optional: `SC_API_KEY` - Star Citizen API key

**Build:**
- `Cargo.toml` - Workspace manifest with shared dependencies
- `.cargo/config.toml` - Build optimization (LTO, codegen-units)
- `clippy.toml` - Cognitive complexity threshold: 15

## Platform Requirements

**Development:**
- Any platform with Rust toolchain
- Docker for PostgreSQL (`docker-compose.yml`)
- Optional: mold/lld linker for faster builds

**Production:**
- Linux (tested on WSL2)
- PostgreSQL 16 (Alpine)
- dbt 1.7.0 for data transformations

---

*Stack analysis: 2026-01-11*
*Update after major dependency changes*
