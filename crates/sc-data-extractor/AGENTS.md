# SC Data Extractor - Agent Guide

## Purpose

Library crate that parses Star Citizen game data from SCLogistics (XML/JSON)
and provides Diesel-backed PostgreSQL storage with a medallion architecture
(raw -> silver -> gold schemas via dbt).

## Architecture

```
build.rs              Compile-time schema inference from SCLogistics files
src/
  parsers/
    starmap.rs        XML parser -- locations, quantum routes, nav data
    shops.rs          JSON parser -- shop inventories, commodity pricing
  database/
    schema.rs         Diesel-generated table definitions (raw.* tables)
    builder.rs        Bulk insert logic with transaction batching
    connection.rs     Postgres connection pool setup
    models.rs         Diesel Insertable/Queryable structs
    queries.rs        Read queries (MapLocation, etc.)
  dataforge/          scunpacked-data access (items, ships, labels)
  models/             Domain types (StarmapLocation, ShopInventory)
  generated.rs        Includes build.rs output via include!(concat!(env!("OUT_DIR"), ...))
  localization.rs     Game string localization store
  error.rs            Error/Result types (thiserror)
```

## Build Script (build.rs)

The build script is the most complex part of this crate. It:

1. Reads `SCLOGISTICS_PATH` env var (reruns on change)
2. Walks `starmap/` XML files -- extracts all attributes from StarMap* elements
3. Walks `Shops/shopinventories/` JSON files -- extracts all fields recursively
4. Infers Rust types from values (string/int/float/bool) with merge logic
5. Generates `generated_schema.rs`, `generated_schema.sql`, `schema_info.rs`
6. Copies outputs to `crates/sc-logistics-importer/resources/`

If `SCLOGISTICS_PATH` is unset or missing, a default minimal schema is emitted
so the crate still compiles without the SCLogistics repo present.

## Database

- PostgreSQL via Diesel ORM (not SQLite despite README references)
- Migrations in `migrations/` -- medallion schemas (raw namespace)
- Tables: `raw.locations`, `raw.quantum_routes`, `raw.shops`, `raw.shop_items`
- `diesel.toml` configures schema generation
- Connection requires `DATABASE_URL` env var (see `.env.example`)

## Key Patterns

- Parsers return domain model vecs; DatabaseBuilder inserts in transactions
- `quick-xml` for streaming XML parse; `serde_json` for JSON
- `walkdir` for recursive file discovery in both parsers and build.rs
- Optional fields determined by occurrence count vs total file count

## Testing

```bash
cargo test -p sc-data-extractor
```

Tests use `tempfile` for ephemeral databases. Parser tests need either
`SCLOGISTICS_PATH` set or they test the default schema path.

## Common Tasks

| Task                        | Where to look                    |
|-----------------------------|----------------------------------|
| Add a parsed field          | build.rs + parsers/ + models/    |
| Change DB schema            | migrations/ then diesel migration|
| Fix type inference           | build.rs `infer_type_from_value` |
| Add a new data source       | New parser module + model types  |
| Debug missing fields        | Check build warnings in cargo output |
