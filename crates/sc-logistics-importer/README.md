# SC Logistics Importer

Command-line tool for importing Star Citizen game data from the SCLogistics repository into SQLite databases.

## Installation

From the workspace root:

```bash
cargo build --release --package sc-logistics-importer
```

The binary will be at `target/release/sc-logistics-importer`.

## Prerequisites

Clone the SCLogistics repository:

```bash
cd /path/to/projects
git clone https://gitlab.com/painlabs/SCLogistics.git
```

## Usage

### Import All Data

```bash
sc-logistics-importer --sclogistics-path /path/to/SCLogistics all
```

This will:
1. Parse all starmap XML files (~1,900 locations)
2. Parse all shop inventory JSON files (~87 shops)
3. Create a SQLite database at `data/sc-game-data.db`

### Import Only Starmap Data

```bash
sc-logistics-importer --sclogistics-path /path/to/SCLogistics starmap
```

### Import Only Shop Data

```bash
sc-logistics-importer --sclogistics-path /path/to/SCLogistics shops
```

### Show Database Statistics

```bash
sc-logistics-importer stats
```

Example output:
```
Database Statistics:
  Locations: 1915
  Shops: 108
  Shop Items: 12148
```

### Custom Output Path

```bash
sc-logistics-importer \
  --sclogistics-path /path/to/SCLogistics \
  --output /custom/path/database.db \
  all
```

## Performance

The import process is highly optimized:
- Uses SQLite transactions for batch inserts
- Processes ~1,900 locations + 12,000 items in ~1 second
- Creates a 4.3 MB database

## What Gets Imported

### From Starmap XML Files
- Location IDs, names, and descriptions
- Parent-child relationships (planets, moons, stations)
- Location types and navigation icons
- Quantum travel parameters (arrival/obstruction radii)
- Faction affiliations

### From Shop Inventory JSON Files
- Shop IDs and item inventories
- Buy and sell prices for commodities
- Maximum inventory capacities
- Current stock levels (snapshot data)

## Database Schema

The created SQLite database contains:
- `locations` - Starmap locations with quantum travel data
- `shops` - Trading terminals and shops
- `shop_items` - Items available at each shop with pricing
- `quantum_routes` - (Future) Quantum travel route graph

All tables include appropriate indexes for efficient querying.

## Example Workflow

```bash
# 1. Clone SCLogistics (one-time setup)
git clone https://gitlab.com/painlabs/SCLogistics.git

# 2. Build the importer
cargo build --release --package sc-logistics-importer

# 3. Import all data
./target/release/sc-logistics-importer \
  --sclogistics-path ../SCLogistics \
  all

# 4. Check the results
./target/release/sc-logistics-importer stats
```

## Updating Data

To refresh the database with updated game data:

```bash
# Pull latest changes from SCLogistics
cd /path/to/SCLogistics
git pull

# Re-run the importer (it will replace existing data)
sc-logistics-importer --sclogistics-path /path/to/SCLogistics all
```

## License

MIT
