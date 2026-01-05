# SC Data Extractor

Rust library for parsing Star Citizen game data from the [SCLogistics repository](https://gitlab.com/painlabs/SCLogistics) and building SQLite databases for use in the sc-interdiction application.

## Features

- **Starmap Parser**: Extracts location data, quantum travel parameters, and navigation information from XML files
- **Shop Parser**: Parses shop inventory JSON files with commodity pricing and availability
- **SQLite Database**: Creates optimized databases with indexes for fast querying
- **Batch Processing**: Uses transactions for efficient bulk inserts

## Usage

### As a Library

```rust
use sc_data_extractor::{
    database::{Database, DatabaseBuilder},
    parsers::{StarmapParser, ShopsParser},
};

// Parse starmap locations
let parser = StarmapParser::new("/path/to/SCLogistics");
let locations = parser.parse_all()?;

// Parse shop inventories
let shops_parser = ShopsParser::new("/path/to/SCLogistics");
let inventories = shops_parser.parse_all()?;

// Build database
let db = Database::new("output.db")?;
let mut builder = DatabaseBuilder::new(db);
builder.init_schema()?;
builder.insert_locations(&locations)?;
builder.insert_shops(&inventories)?;
```

### CLI Tool

See the `sc-logistics-importer` crate for a command-line tool that uses this library.

## Database Schema

### `locations` Table
- Location ID, name, type, parent hierarchy
- Navigation icons and affiliation
- Quantum travel parameters (arrival/obstruction radii)

### `shops` Table
- Shop ID and name
- Optional location reference

### `shop_items` Table
- Item ID, buy/sell prices
- Maximum inventory capacity
- Foreign keys to shops

### `quantum_routes` Table
- From/to location pairs
- Distance (for future use)

## Data Source

This library expects the SCLogistics repository to be cloned locally. The default path is `../SCLogistics` relative to the workspace root, but this can be customized.

## Performance

- Parses 1,900+ starmap locations in ~100ms
- Processes 87 shop inventory files in ~50ms
- Database creation with full dataset completes in ~1 second

## Testing

```bash
cargo test --package sc-data-extractor
```

## License

MIT
