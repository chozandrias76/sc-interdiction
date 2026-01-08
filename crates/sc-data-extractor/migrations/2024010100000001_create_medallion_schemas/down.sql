-- Revert medallion architecture
-- Note: dbt-managed tables/views in silver/gold/staging will be dropped with schema

-- Drop raw tables
DROP TABLE IF EXISTS raw.quantum_routes;
DROP TABLE IF EXISTS raw.shop_items;
DROP TABLE IF EXISTS raw.shops;
DROP TABLE IF EXISTS raw.locations;

-- Drop schemas (CASCADE drops all dbt-created objects)
DROP SCHEMA IF EXISTS staging CASCADE;
DROP SCHEMA IF EXISTS gold CASCADE;
DROP SCHEMA IF EXISTS silver CASCADE;
DROP SCHEMA IF EXISTS raw CASCADE;
