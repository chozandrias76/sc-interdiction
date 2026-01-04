# Quick Start: SCLogistics Data Import

## Setup (One-Time)

```bash
# 1. Clone the SCLogistics repository
cd /home/choza/projects
git clone https://gitlab.com/painlabs/SCLogistics.git

# 2. Build the importer tool
cd sc-interdiction
cargo build --release --package sc-logistics-importer
```

## Import Game Data

```bash
# Import all data (starmap + shops)
cargo run --release --package sc-logistics-importer -- \
  --sclogistics-path /home/choza/projects/SCLogistics \
  all

# Check what was imported
cargo run --release --package sc-logistics-importer -- stats
```

Expected output:
```
Database Statistics:
  Locations: 1915
  Shops: 108
  Shop Items: 12148
```

## Using the Database in Your Code

### Query Locations

```rust
use rusqlite::{Connection, Result};

fn query_locations() -> Result<()> {
    let conn = Connection::open("data/sc-game-data.db")?;
    
    let mut stmt = conn.prepare(
        "SELECT id, name, location_type, arrival_radius 
         FROM locations 
         WHERE arrival_radius > 0 
         ORDER BY name"
    )?;
    
    let locations = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, f64>(3)?,
        ))
    })?;
    
    for location in locations {
        let (id, name, type_, radius) = location?;
        println!("{}: {} ({}, radius: {})", id, name, type_, radius);
    }
    
    Ok(())
}
```

### Query Shop Pricing

```rust
fn query_commodity_prices(commodity_id: &str) -> Result<Vec<(String, f64, f64)>> {
    let conn = Connection::open("data/sc-game-data.db")?;
    
    let mut stmt = conn.prepare(
        "SELECT s.shop_name, si.buy_price, si.sell_price
         FROM shop_items si
         JOIN shops s ON si.shop_id = s.shop_id
         WHERE si.item_id = ?1
         ORDER BY si.sell_price DESC"
    )?;
    
    let prices = stmt.query_map([commodity_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, f64>(2)?,
        ))
    })?;
    
    Ok(prices.collect::<Result<Vec<_>>>()?)
}
```

### Build Location Graph

```rust
use petgraph::graph::Graph;

fn build_location_graph() -> Result<Graph<String, f64>> {
    let conn = Connection::open("data/sc-game-data.db")?;
    let mut graph = Graph::new();
    
    // Add nodes for each location
    let mut stmt = conn.prepare("SELECT id, name FROM locations")?;
    let locations: Vec<(String, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<_>>()?;
    
    for (id, name) in locations {
        graph.add_node(name);
    }
    
    // TODO: Add edges based on quantum travel routes
    
    Ok(graph)
}
```

## Update Data

When SCLogistics repository is updated:

```bash
# 1. Pull latest game data
cd /home/choza/projects/SCLogistics
git pull

# 2. Re-import (overwrites existing database)
cd /home/choza/projects/sc-interdiction
cargo run --release --package sc-logistics-importer -- all
```

## Database Files

- **Location:** `data/sc-game-data.db`
- **Size:** ~4.3 MB
- **Format:** SQLite 3
- **Gitignored:** Yes (excluded from version control)

## Available Tables

```sql
-- Starmap locations with quantum travel data
SELECT * FROM locations;

-- Trading shops and terminals
SELECT * FROM shops;

-- Commodity inventory and pricing
SELECT * FROM shop_items;

-- Quantum travel routes (reserved for future use)
SELECT * FROM quantum_routes;
```

## Useful Queries

### Find all stations in a system

```sql
SELECT name, nav_icon 
FROM locations 
WHERE parent IN (
  SELECT id FROM locations WHERE name LIKE '%Stanton%'
)
AND nav_icon IN ('Station', 'Outpost');
```

### Find best sell price for an item

```sql
SELECT s.shop_name, si.sell_price, si.max_inventory
FROM shop_items si
JOIN shops s ON si.shop_id = s.shop_id
WHERE si.item_id = 'your-commodity-uuid'
  AND si.sell_price > 0
ORDER BY si.sell_price DESC
LIMIT 10;
```

### Count locations by type

```sql
SELECT location_type, COUNT(*) as count
FROM locations
GROUP BY location_type
ORDER BY count DESC;
```

## Adding to Other Crates

To use `sc-data-extractor` as a dependency:

```toml
[dependencies]
sc-data-extractor = { path = "../sc-data-extractor" }
rusqlite = { version = "0.32", features = ["bundled"] }
```

Then in your code:

```rust
use sc_data_extractor::database::Database;

let db = Database::new("data/sc-game-data.db")?;
// Query as needed
```

## Troubleshooting

**Database doesn't exist:**
```bash
cargo run --release --package sc-logistics-importer -- all
```

**Old data:**
```bash
rm data/sc-game-data.db
cargo run --release --package sc-logistics-importer -- all
```

**SCLogistics path error:**
```bash
# Check the path exists
ls -la /home/choza/projects/SCLogistics

# Or specify custom path
cargo run --release --package sc-logistics-importer -- \
  --sclogistics-path /custom/path/to/SCLogistics \
  all
```
