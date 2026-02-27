# dbt -- Agent Instructions

## Overview

dbt (data build tool) transforms raw Postgres tables into analytics-ready
layers using a medallion architecture. Runs inside Docker via
`ghcr.io/dbt-labs/dbt-postgres:1.7.0`.

## Medallion Layers

| Layer   | Dir              | Schema    | Materialized | Purpose                        |
|---------|------------------|-----------|--------------|--------------------------------|
| Staging | `models/staging` | staging   | view         | Light renames, type casts      |
| Silver  | `models/silver`  | silver    | table        | Cleaned, deduplicated entities |
| Gold    | `models/gold`    | gold      | view         | Aggregates, ready for queries  |

Seeds (CSV reference data) land in the **silver** schema.

## Directory Structure

```
dbt/
  models/
    staging/     stg_*.sql      -- raw source wrappers
    silver/      silver_*.sql   -- cleaned tables
    gold/        gold_*.sql     -- analytical views
  seeds/         *.csv          -- static reference data
  macros/                       -- reusable Jinja helpers (empty)
  tests/                        -- custom data tests (empty)
  profiles.yml                  -- connection config (env vars)
  dbt_project.yml               -- project settings
```

### Exclude from indexing / search

- Compiled SQL artifacts (generated) should be excluded from indexing/search.
- Vendored dbt packages (installed dependencies) should be excluded from indexing/search.

## Commands

All dbt commands run through Docker Compose with the `dbt` profile.
The DB service must be healthy first.

```bash
# Start Postgres (if not running)
docker compose up -d db

# Run all models (staging -> silver -> gold)
docker compose --profile dbt run --rm dbt run

# Run tests (schema + data quality)
docker compose --profile dbt run --rm dbt test

# Load seed CSVs into silver schema
docker compose --profile dbt run --rm dbt seed

# Full pipeline: seed + run + test
docker compose --profile dbt run --rm dbt build

# Run a single model
docker compose --profile dbt run --rm dbt run --select gold_locations

# Debug connection
docker compose --profile dbt run --rm dbt debug
```

## Connection

Profile `sc_interdiction` connects to Postgres using env vars
(`DBT_HOST`, `DBT_PORT`, `DBT_USER`, `DBT_PASSWORD`, `DBT_DBNAME`).
Docker Compose sets these automatically. For local runs, defaults
point to `localhost:5432` with user/pass `sc/sc`.

## Conventions

- Staging models prefix: `stg_`
- Silver models prefix: `silver_`
- Gold models prefix: `gold_`
- Each layer has `schema.yml` for column docs, tests, and seed definitions
