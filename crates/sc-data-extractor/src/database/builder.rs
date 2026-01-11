//! Database builder for populating from parsed data using Diesel ORM

use diesel::prelude::*;

use crate::database::connection::Database;
use crate::database::models::{NewLocation, NewQuantumRoute, NewShop, NewShopItem};
use crate::database::schema::raw::{locations, quantum_routes, shop_items, shops};
use crate::error::Result;
use crate::models::shops::ShopInventory;
use crate::models::starmap::StarmapLocation;

/// Builder for populating database from parsed data
pub struct DatabaseBuilder {
    db: Database,
}

impl DatabaseBuilder {
    /// Creates a new database builder
    #[must_use]
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Inserts starmap locations into the raw schema.
    ///
    /// # Errors
    ///
    /// Returns an error if database insertion fails.
    pub fn insert_locations(&mut self, starmap_locations: &[StarmapLocation]) -> Result<usize> {
        let mut conn = self.db.get_connection()?;
        let mut count = 0;

        for loc in starmap_locations {
            let qt = loc.quantum_travel.as_ref();
            let new_location = NewLocation {
                id: &loc.id,
                name: &loc.name,
                parent_id: loc.parent.as_deref(),
                location_type: &loc.location_type,
                nav_icon: loc.nav_icon.as_deref(),
                affiliation: loc.affiliation.as_deref(),
                description: loc.description.as_deref(),
                is_scannable: loc.is_scannable,
                hide_in_starmap: loc.hide_in_starmap,
                obstruction_radius: qt.map(|q| q.obstruction_radius),
                arrival_radius: qt.map(|q| q.arrival_radius),
                arrival_point_offset: qt.map(|q| q.arrival_point_detection_offset),
                adoption_radius: qt.map(|q| q.adoption_radius),
            };

            diesel::insert_into(locations::table)
                .values(&new_location)
                .on_conflict(locations::id)
                .do_update()
                .set((
                    locations::name.eq(&new_location.name),
                    locations::parent_id.eq(&new_location.parent_id),
                    locations::location_type.eq(&new_location.location_type),
                    locations::nav_icon.eq(&new_location.nav_icon),
                    locations::affiliation.eq(&new_location.affiliation),
                    locations::description.eq(&new_location.description),
                    locations::is_scannable.eq(new_location.is_scannable),
                    locations::hide_in_starmap.eq(new_location.hide_in_starmap),
                    locations::obstruction_radius.eq(new_location.obstruction_radius),
                    locations::arrival_radius.eq(new_location.arrival_radius),
                    locations::arrival_point_offset.eq(new_location.arrival_point_offset),
                    locations::adoption_radius.eq(new_location.adoption_radius),
                ))
                .execute(&mut *conn)?;

            count += 1;
        }

        Ok(count)
    }

    /// Inserts shop inventories into the raw schema.
    ///
    /// # Errors
    ///
    /// Returns an error if database insertion fails.
    pub fn insert_shops(&mut self, inventories: &[ShopInventory]) -> Result<usize> {
        let mut conn = self.db.get_connection()?;
        let mut shop_count = 0;

        for inv in inventories {
            for shop_id in inv.shop_id.split(',') {
                let shop_id = shop_id.trim();
                let shop_name = format!("Shop_{shop_id}");

                let new_shop = NewShop {
                    shop_id,
                    location_id: None,
                    shop_name: &shop_name,
                };

                diesel::insert_into(shops::table)
                    .values(&new_shop)
                    .on_conflict(shops::shop_id)
                    .do_nothing()
                    .execute(&mut *conn)?;

                shop_count += 1;

                for item in &inv.collection.inventory {
                    if let Some(item_id) = item.id.id.first() {
                        let new_item = NewShopItem {
                            shop_id,
                            item_id,
                            buy_price: item.buy_price,
                            sell_price: item.sell_price,
                            max_inventory: item.max_inventory,
                        };

                        diesel::insert_into(shop_items::table)
                            .values(&new_item)
                            .on_conflict((shop_items::shop_id, shop_items::item_id))
                            .do_update()
                            .set((
                                shop_items::buy_price.eq(new_item.buy_price),
                                shop_items::sell_price.eq(new_item.sell_price),
                                shop_items::max_inventory.eq(new_item.max_inventory),
                            ))
                            .execute(&mut *conn)?;
                    }
                }
            }
        }

        Ok(shop_count)
    }

    /// Inserts quantum routes into the raw schema.
    ///
    /// # Errors
    ///
    /// Returns an error if database insertion fails.
    pub fn insert_quantum_routes(
        &mut self,
        routes: &[(String, String, Option<f64>)],
    ) -> Result<usize> {
        let mut conn = self.db.get_connection()?;
        let mut count = 0;

        for (from_loc, to_loc, distance) in routes {
            let new_route = NewQuantumRoute {
                from_location: from_loc,
                to_location: to_loc,
                distance: *distance,
            };

            diesel::insert_into(quantum_routes::table)
                .values(&new_route)
                .on_conflict((quantum_routes::from_location, quantum_routes::to_location))
                .do_update()
                .set(quantum_routes::distance.eq(new_route.distance))
                .execute(&mut *conn)?;

            count += 1;
        }

        Ok(count)
    }

    /// Consumes the builder and returns the database
    #[must_use]
    pub fn build(self) -> Database {
        self.db
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::print_stderr)]
mod tests {
    use super::*;
    use crate::models::shops::*;
    use crate::models::starmap::*;
    use dotenvy::dotenv;

    fn setup_test_db() -> Option<Database> {
        dotenv().ok();
        Database::from_env().ok()
    }

    #[test]
    #[ignore = "requires PostgreSQL database"]
    fn test_insert_locations() {
        let Some(db) = setup_test_db() else {
            eprintln!("Skipping test: no database connection");
            return;
        };
        let mut builder = DatabaseBuilder::new(db);

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
    #[ignore = "requires PostgreSQL database"]
    fn test_insert_shops() {
        let Some(db) = setup_test_db() else {
            eprintln!("Skipping test: no database connection");
            return;
        };
        let mut builder = DatabaseBuilder::new(db);

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
