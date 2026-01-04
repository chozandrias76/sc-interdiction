//! Database schema and queries

use crate::error::Result;
use rusqlite::Connection;

/// SQLite database for extracted game data
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Creates a new in-memory database
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Ok(Self { conn })
    }

    /// Creates a new database at the specified path
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    /// Initializes the database schema
    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS locations (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                parent_id TEXT,
                location_type TEXT NOT NULL,
                nav_icon TEXT,
                affiliation TEXT,
                description TEXT,
                is_scannable INTEGER NOT NULL,
                hide_in_starmap INTEGER NOT NULL,
                obstruction_radius REAL,
                arrival_radius REAL,
                arrival_point_offset REAL,
                adoption_radius REAL
            );

            CREATE INDEX IF NOT EXISTS idx_locations_parent ON locations(parent_id);
            CREATE INDEX IF NOT EXISTS idx_locations_type ON locations(location_type);

            CREATE TABLE IF NOT EXISTS shops (
                shop_id TEXT PRIMARY KEY,
                location_id TEXT,
                shop_name TEXT NOT NULL,
                FOREIGN KEY (location_id) REFERENCES locations(id)
            );

            CREATE INDEX IF NOT EXISTS idx_shops_location ON shops(location_id);

            CREATE TABLE IF NOT EXISTS shop_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                shop_id TEXT NOT NULL,
                item_id TEXT NOT NULL,
                buy_price REAL NOT NULL,
                sell_price REAL NOT NULL,
                max_inventory REAL NOT NULL,
                FOREIGN KEY (shop_id) REFERENCES shops(shop_id),
                UNIQUE(shop_id, item_id)
            );

            CREATE INDEX IF NOT EXISTS idx_shop_items_shop ON shop_items(shop_id);
            CREATE INDEX IF NOT EXISTS idx_shop_items_item ON shop_items(item_id);

            CREATE TABLE IF NOT EXISTS quantum_routes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                from_location TEXT NOT NULL,
                to_location TEXT NOT NULL,
                distance REAL,
                FOREIGN KEY (from_location) REFERENCES locations(id),
                FOREIGN KEY (to_location) REFERENCES locations(id),
                UNIQUE(from_location, to_location)
            );

            CREATE INDEX IF NOT EXISTS idx_routes_from ON quantum_routes(from_location);
            CREATE INDEX IF NOT EXISTS idx_routes_to ON quantum_routes(to_location);
            "#,
        )?;

        Ok(())
    }

    /// Returns the underlying connection for custom queries
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Returns a mutable reference to the underlying connection
    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_schema() {
        let db = Database::new_in_memory().expect("Failed to create database");
        db.init_schema().expect("Failed to initialize schema");

        let table_count: i32 = db
            .connection()
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
                [],
                |row| row.get(0),
            )
            .expect("Failed to query tables");

        assert!(table_count >= 4);
    }
}
