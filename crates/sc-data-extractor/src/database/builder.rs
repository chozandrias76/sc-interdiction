//! Database builder for populating from parsed data

use crate::database::schema::Database;
use crate::error::Result;
use crate::models::shops::ShopInventory;
use crate::models::starmap::StarmapLocation;
use rusqlite::params;

/// Builder for populating database from parsed data
pub struct DatabaseBuilder {
    db: Database,
}

impl DatabaseBuilder {
    /// Creates a new database builder
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Initializes the database schema.
    ///
    /// # Errors
    ///
    /// Returns an error if schema creation fails.
    pub fn init_schema(&self) -> Result<()> {
        self.db.init_schema()
    }

    /// Inserts starmap locations into the database.
    ///
    /// # Errors
    ///
    /// Returns an error if database insertion fails.
    pub fn insert_locations(&mut self, locations: &[StarmapLocation]) -> Result<usize> {
        let mut count = 0;
        let conn = self.db.connection_mut();

        let tx = conn.transaction()?;

        for loc in locations {
            let qt = loc.quantum_travel.as_ref();
            tx.execute(
                r#"
                INSERT OR REPLACE INTO locations (
                    id, name, parent_id, location_type, nav_icon, affiliation,
                    description, is_scannable, hide_in_starmap,
                    obstruction_radius, arrival_radius, arrival_point_offset, adoption_radius
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
                "#,
                params![
                    loc.id,
                    loc.name,
                    loc.parent,
                    loc.location_type,
                    loc.nav_icon,
                    loc.affiliation,
                    loc.description,
                    loc.is_scannable as i32,
                    loc.hide_in_starmap as i32,
                    qt.map(|q| q.obstruction_radius),
                    qt.map(|q| q.arrival_radius),
                    qt.map(|q| q.arrival_point_detection_offset),
                    qt.map(|q| q.adoption_radius),
                ],
            )?;
            count += 1;
        }

        tx.commit()?;

        Ok(count)
    }

    /// Inserts shop inventories into the database.
    ///
    /// # Errors
    ///
    /// Returns an error if database insertion fails.
    pub fn insert_shops(&mut self, inventories: &[ShopInventory]) -> Result<usize> {
        let mut shop_count = 0;
        let mut item_count = 0;
        let conn = self.db.connection_mut();

        let tx = conn.transaction()?;

        for inv in inventories {
            for shop_id in inv.shop_id.split(',') {
                let shop_id = shop_id.trim();
                tx.execute(
                    "INSERT OR IGNORE INTO shops (shop_id, shop_name) VALUES (?1, ?2)",
                    params![shop_id, format!("Shop_{shop_id}")],
                )?;
                shop_count += 1;

                for item in &inv.collection.inventory {
                    if let Some(item_id) = item.id.id.first() {
                        tx.execute(
                            r#"
                            INSERT OR REPLACE INTO shop_items 
                                (shop_id, item_id, buy_price, sell_price, max_inventory)
                            VALUES (?1, ?2, ?3, ?4, ?5)
                            "#,
                            params![
                                shop_id,
                                item_id,
                                item.buy_price,
                                item.sell_price,
                                item.max_inventory
                            ],
                        )?;
                        item_count += 1;
                    }
                }
            }
        }

        tx.commit()?;

        // Note: caller is responsible for logging/displaying this information
        let _ = item_count; // Suppress unused warning
        Ok(shop_count)
    }

    /// Consumes the builder and returns the database
    pub fn build(self) -> Database {
        self.db
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::models::shops::*;
    use crate::models::starmap::*;

    #[test]
    fn test_insert_locations() {
        let db = Database::new_in_memory().expect("Failed to create DB");
        let mut builder = DatabaseBuilder::new(db);
        builder.init_schema().expect("Failed to init schema");

        let locations = vec![StarmapLocation {
            id: "test-id-123".to_string(),
            name: "@Test_Location".to_string(),
            parent: None,
            location_type: "planet".to_string(),
            nav_icon: Some("Planet".to_string()),
            affiliation: None,
            description: None,
            is_scannable: true,
            hide_in_starmap: false,
            quantum_travel: Some(QuantumTravelData {
                obstruction_radius: 100.0,
                arrival_radius: 5000.0,
                arrival_point_detection_offset: 1000.0,
                adoption_radius: 0.0,
            }),
            amenities: vec![],
        }];

        let count = builder
            .insert_locations(&locations)
            .expect("Failed to insert locations");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_insert_shops() {
        let db = Database::new_in_memory().expect("Failed to create DB");
        let mut builder = DatabaseBuilder::new(db);
        builder.init_schema().expect("Failed to init schema");

        let inventories = vec![ShopInventory {
            shop_id: "shop-123".to_string(),
            collection: InventoryCollection {
                inventory: vec![InventoryItem {
                    id: ItemId {
                        id: vec!["item-456".to_string()],
                    },
                    buy_price: 10.0,
                    sell_price: 15.0,
                    current_inventory: 100.0,
                    max_inventory: 500.0,
                }],
            },
        }];

        let count = builder
            .insert_shops(&inventories)
            .expect("Failed to insert shops");
        assert!(count > 0);
    }
}
