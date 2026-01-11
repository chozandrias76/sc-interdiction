//! Database query functions for retrieving processed data

use diesel::prelude::*;
use diesel::sql_types::Text;

use super::connection::Database;
use crate::error::Result;

/// A location suitable for map display
#[derive(Debug, Clone)]
pub struct MapLocation {
    /// Display name (human-readable)
    pub display_name: String,
    /// Location type from `nav_icon` (Star, Planet, Moon, Station, `LandingZone`)
    pub nav_icon: String,
    /// Parent display name (if any)
    pub parent_display_name: Option<String>,
    /// System name (inferred from parent chain)
    pub system: String,
}

/// Raw result from database query
#[derive(QueryableByName)]
struct MapLocationRow {
    #[diesel(sql_type = Text)]
    display_name: String,
    #[diesel(sql_type = Text)]
    nav_icon: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<Text>)]
    parent_display_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<Text>)]
    system_name: Option<String>,
}

impl Database {
    /// Query map locations from the gold schema.
    ///
    /// Returns locations with display names, types, and parent relationships.
    /// Only includes locations visible on starmap (Star, Planet, Moon, Station, `LandingZone`).
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub fn query_map_locations(&self) -> Result<Vec<MapLocation>> {
        let mut conn = self.get_connection()?;

        let rows: Vec<MapLocationRow> = diesel::sql_query(
            r#"
            WITH RECURSIVE location_hierarchy AS (
                -- Base case: stars (root locations with Star nav_icon)
                SELECT
                    id,
                    display_name,
                    nav_icon,
                    parent_id,
                    display_name as system_name,
                    0 as depth
                FROM gold.locations
                WHERE nav_icon = 'Star'
                  AND display_name IS NOT NULL

                UNION ALL

                -- Recursive case: children of known locations
                SELECT
                    l.id,
                    l.display_name,
                    l.nav_icon,
                    l.parent_id,
                    lh.system_name,
                    lh.depth + 1
                FROM gold.locations l
                INNER JOIN location_hierarchy lh ON l.parent_id = lh.id
                WHERE l.display_name IS NOT NULL
                  AND l.nav_icon IN ('Planet', 'Moon', 'Station', 'LandingZone')
                  AND l.hide_in_starmap = false
            )
            SELECT
                lh.display_name,
                lh.nav_icon,
                p.display_name as parent_display_name,
                lh.system_name
            FROM location_hierarchy lh
            LEFT JOIN gold.locations p ON lh.parent_id = p.id
            ORDER BY
                lh.system_name,
                CASE lh.nav_icon
                    WHEN 'Star' THEN 1
                    WHEN 'Planet' THEN 2
                    WHEN 'Moon' THEN 3
                    WHEN 'LandingZone' THEN 4
                    WHEN 'Station' THEN 5
                    ELSE 6
                END,
                lh.display_name
            "#,
        )
        .load(&mut *conn)?;

        Ok(rows
            .into_iter()
            .map(|row| MapLocation {
                display_name: row.display_name,
                nav_icon: row.nav_icon,
                parent_display_name: row.parent_display_name,
                system: row.system_name.unwrap_or_else(|| "Unknown".to_string()),
            })
            .collect())
    }

    /// Query map locations for a specific system.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub fn query_map_locations_for_system(&self, system: &str) -> Result<Vec<MapLocation>> {
        let all = self.query_map_locations()?;
        Ok(all
            .into_iter()
            .filter(|loc| loc.system.eq_ignore_ascii_case(system))
            .collect())
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[test]
    #[ignore = "requires PostgreSQL database"]
    fn test_query_map_locations() {
        dotenv().ok();
        let db = Database::from_env().expect("Failed to create database");
        let locations = db.query_map_locations().expect("Failed to query locations");

        // Should have some locations
        assert!(!locations.is_empty());

        // Should have Stanton and Pyro
        let systems: std::collections::HashSet<_> =
            locations.iter().map(|l| l.system.as_str()).collect();
        assert!(systems.contains("Stanton"));

        // Check structure
        for loc in &locations {
            assert!(!loc.display_name.is_empty());
            assert!(!loc.nav_icon.is_empty());
        }
    }
}
