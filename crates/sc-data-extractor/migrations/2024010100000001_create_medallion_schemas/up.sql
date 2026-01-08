-- Create medallion architecture schemas
-- Raw schema: managed by Diesel (tables)
-- Silver/Gold schemas: managed by dbt (models/views)

CREATE SCHEMA IF NOT EXISTS raw;
CREATE SCHEMA IF NOT EXISTS silver;
CREATE SCHEMA IF NOT EXISTS gold;
CREATE SCHEMA IF NOT EXISTS staging;

-- ============================================================================
-- RAW SCHEMA: Direct ingest from source files (Diesel-managed)
-- ============================================================================

CREATE TABLE raw.locations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id TEXT,
    location_type TEXT NOT NULL,
    nav_icon TEXT,
    affiliation TEXT,
    description TEXT,
    is_scannable BOOLEAN NOT NULL DEFAULT false,
    hide_in_starmap BOOLEAN NOT NULL DEFAULT false,
    obstruction_radius DOUBLE PRECISION,
    arrival_radius DOUBLE PRECISION,
    arrival_point_offset DOUBLE PRECISION,
    adoption_radius DOUBLE PRECISION,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_raw_locations_parent ON raw.locations(parent_id);
CREATE INDEX idx_raw_locations_type ON raw.locations(location_type);

CREATE TABLE raw.shops (
    shop_id TEXT PRIMARY KEY,
    location_id TEXT,
    shop_name TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_raw_shops_location ON raw.shops(location_id);

CREATE TABLE raw.shop_items (
    id SERIAL PRIMARY KEY,
    shop_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    buy_price DOUBLE PRECISION NOT NULL,
    sell_price DOUBLE PRECISION NOT NULL,
    max_inventory DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(shop_id, item_id)
);

CREATE INDEX idx_raw_shop_items_shop ON raw.shop_items(shop_id);
CREATE INDEX idx_raw_shop_items_item ON raw.shop_items(item_id);

CREATE TABLE raw.quantum_routes (
    id SERIAL PRIMARY KEY,
    from_location TEXT NOT NULL,
    to_location TEXT NOT NULL,
    distance DOUBLE PRECISION,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(from_location, to_location)
);

CREATE INDEX idx_raw_routes_from ON raw.quantum_routes(from_location);
CREATE INDEX idx_raw_routes_to ON raw.quantum_routes(to_location);

-- Add updated_at triggers for raw tables
SELECT diesel_manage_updated_at('raw.locations');
SELECT diesel_manage_updated_at('raw.shops');
SELECT diesel_manage_updated_at('raw.shop_items');
SELECT diesel_manage_updated_at('raw.quantum_routes');

-- Note: Silver and Gold schemas are created by dbt models
-- Run 'make dbt-all' after migrations to build silver/gold layers
