//! Map location data sourced from database and route-graph coordinates.
//!
//! This module provides map locations by:
//! 1. Querying the database for location metadata (name, type, parent)
//! 2. Looking up coordinates from route-graph's static position database
//! 3. Falling back to static data if database is unavailable

use super::super::types::{MapLocation, MapLocationType};
use route_graph::{estimate_position, locations_in_system, LOCATION_POSITIONS};
use sc_data_extractor::database::Database;

/// Infer system name from a location string.
pub fn infer_system(location: &str) -> String {
    let loc_lower = location.to_lowercase();

    // Check route-graph positions for system info
    if let Some(pos) = LOCATION_POSITIONS.get(loc_lower.as_str()) {
        return pos.system.to_string();
    }

    // Stanton system locations
    let stanton_locations = [
        "hurston",
        "crusader",
        "arccorp",
        "microtech",
        "lorville",
        "orison",
        "area18",
        "area 18",
        "new babbage",
        "port olisar",
        "everus",
        "baijini",
        "tressler",
        "grim hex",
        "levski",
        "hur-l",
        "cru-l",
        "arc-l",
        "mic-l",
        "arial",
        "aberdeen",
        "magda",
        "ita",
        "cellin",
        "daymar",
        "yela",
        "lyria",
        "wala",
        "calliope",
        "clio",
        "euterpe",
    ];

    for loc in stanton_locations {
        if loc_lower.contains(loc) {
            return "Stanton".to_string();
        }
    }

    // Pyro system locations
    let pyro_locations = ["pyro", "ruin station", "checkmate"];
    for loc in pyro_locations {
        if loc_lower.contains(loc) {
            return "Pyro".to_string();
        }
    }

    "Stanton".to_string() // Default
}

/// Build map locations from database, with coordinate lookup from route-graph.
///
/// Falls back to static data if database is unavailable.
pub fn build_map_locations(system: &str) -> Vec<MapLocation> {
    // Try to load from database first
    if let Ok(db) = Database::from_env() {
        if let Ok(db_locations) = db.query_map_locations_for_system(system) {
            let mut locations = Vec::new();

            for db_loc in db_locations {
                // Look up coordinates from route-graph
                let name_lower = db_loc.display_name.to_lowercase();
                if let Some(pos) = estimate_position(&name_lower) {
                    let loc_type = match db_loc.nav_icon.as_str() {
                        "Star" => MapLocationType::Star,
                        "Planet" => MapLocationType::Planet,
                        "Moon" => MapLocationType::Moon,
                        "Station" | "LandingZone" => MapLocationType::Station,
                        _ => continue, // Skip unknown types
                    };

                    locations.push(MapLocation {
                        name: db_loc.display_name,
                        x: pos.x,
                        y: pos.y,
                        loc_type,
                        parent: db_loc.parent_display_name,
                    });
                }
            }

            if !locations.is_empty() {
                // Add the star at center if not already present
                if !locations
                    .iter()
                    .any(|l| l.loc_type == MapLocationType::Star)
                {
                    locations.insert(
                        0,
                        MapLocation {
                            name: system.to_string(),
                            x: 0.0,
                            y: 0.0,
                            loc_type: MapLocationType::Star,
                            parent: None,
                        },
                    );
                }
                return locations;
            }
        }
    }

    // Fall back to static data
    build_static_map_locations(system)
}

/// Build static map locations (fallback when database unavailable).
fn build_static_map_locations(system: &str) -> Vec<MapLocation> {
    let mut locations = Vec::new();

    // Get all locations from route-graph for this system
    let system_locations = locations_in_system(system);

    for pos in system_locations {
        let loc_type = infer_location_type(pos.name, pos.parent);

        locations.push(MapLocation {
            name: pos.name.to_string(),
            x: pos.position.x,
            y: pos.position.y,
            loc_type,
            parent: pos.parent.map(String::from),
        });
    }

    // Add star at center if not present
    if !locations
        .iter()
        .any(|l| l.loc_type == MapLocationType::Star)
    {
        locations.insert(
            0,
            MapLocation {
                name: system.to_string(),
                x: 0.0,
                y: 0.0,
                loc_type: MapLocationType::Star,
                parent: None,
            },
        );
    }

    locations
}

/// Infer location type from name and parent.
fn infer_location_type(name: &str, parent: Option<&str>) -> MapLocationType {
    let name_lower = name.to_lowercase();

    // Stars have no parent and are at origin
    if parent.is_none() {
        // Check if it's a planet (has orbital radius)
        if let Some(pos) = LOCATION_POSITIONS.get(name_lower.as_str()) {
            if pos.position.x.abs() < 0.001 && pos.position.y.abs() < 0.001 {
                return MapLocationType::Star;
            }
        }
        return MapLocationType::Planet;
    }

    // Check for station indicators
    let station_keywords = [
        "station", "harbor", "point", "olisar", "tressler", "hex", "kareah", "covalex", "seraphim",
        "-l1", "-l2", "-l3", "-l4", "-l5",
    ];
    for keyword in station_keywords {
        if name_lower.contains(keyword) {
            return MapLocationType::Station;
        }
    }

    // Check for landing zone indicators
    let landing_keywords = [
        "lorville",
        "orison",
        "area 18",
        "area18",
        "new babbage",
        "levski",
    ];
    for keyword in landing_keywords {
        if name_lower.contains(keyword) {
            return MapLocationType::Station; // Landing zones render as stations
        }
    }

    // If parent is a planet, it's likely a moon
    if let Some(p) = parent {
        let parent_lower = p.to_lowercase();
        let planets = [
            "hurston",
            "crusader",
            "arccorp",
            "microtech",
            "pyro i",
            "pyro ii",
            "pyro iii",
            "pyro iv",
            "pyro v",
            "pyro vi",
        ];
        for planet in planets {
            if parent_lower.contains(planet) {
                return MapLocationType::Moon;
            }
        }
    }

    MapLocationType::Moon // Default to moon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_system() {
        assert_eq!(infer_system("Hurston"), "Stanton");
        assert_eq!(infer_system("Pyro III"), "Pyro");
        assert_eq!(infer_system("Port Olisar"), "Stanton");
    }

    #[test]
    fn test_static_locations() {
        let locations = build_static_map_locations("Stanton");
        assert!(!locations.is_empty());

        // Should have a star
        assert!(locations
            .iter()
            .any(|l| l.loc_type == MapLocationType::Star));

        // Should have planets
        assert!(locations
            .iter()
            .any(|l| l.loc_type == MapLocationType::Planet));
    }
}
