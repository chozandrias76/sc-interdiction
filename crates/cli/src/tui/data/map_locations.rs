//! Map location data for different star systems.

use super::super::types::{MapLocation, MapLocationType};
use std::f64::consts::PI;

/// Infer system name from a location string.
pub fn infer_system(location: &str) -> String {
    let loc_lower = location.to_lowercase();

    // Stanton system locations
    let stanton_locations = [
        "hurston",
        "crusader",
        "arccorp",
        "microtech",
        "lorville",
        "orison",
        "area18",
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

/// Build static map locations for a system.
/// Uses angular positions to spread planets around the star for visualization.
pub fn build_map_locations(system: &str) -> Vec<MapLocation> {
    let mut locations = Vec::new();

    match system {
        "Stanton" => build_stanton_locations(&mut locations),
        "Pyro" => build_pyro_locations(&mut locations),
        _ => {}
    }

    locations
}

fn build_stanton_locations(locations: &mut Vec<MapLocation>) {
    // Star at center
    locations.push(MapLocation {
        name: "Stanton".to_string(),
        angle: 0.0,
        orbital_radius: 0.0,
        loc_type: MapLocationType::Star,
        parent: None,
    });

    // Planets - spread around the star at different angles
    add_stanton_planets(locations);
    add_hurston_moons(locations);
    add_crusader_moons(locations);
    add_arccorp_moons(locations);
    add_microtech_moons(locations);
    add_stanton_stations(locations);
}

fn add_stanton_planets(locations: &mut Vec<MapLocation>) {
    // Hurston: ~45 degrees (inner)
    locations.push(MapLocation {
        name: "Hurston".to_string(),
        angle: PI * 0.25,
        orbital_radius: 12.85,
        loc_type: MapLocationType::Planet,
        parent: None,
    });

    // Crusader: ~135 degrees
    locations.push(MapLocation {
        name: "Crusader".to_string(),
        angle: PI * 0.75,
        orbital_radius: 18.96,
        loc_type: MapLocationType::Planet,
        parent: None,
    });

    // ArcCorp: ~225 degrees
    locations.push(MapLocation {
        name: "ArcCorp".to_string(),
        angle: PI * 1.25,
        orbital_radius: 18.59,
        loc_type: MapLocationType::Planet,
        parent: None,
    });

    // microTech: ~315 degrees (outer)
    locations.push(MapLocation {
        name: "microTech".to_string(),
        angle: PI * 1.75,
        orbital_radius: 22.46,
        loc_type: MapLocationType::Planet,
        parent: None,
    });
}

fn add_hurston_moons(locations: &mut Vec<MapLocation>) {
    for (i, name) in ["Arial", "Aberdeen", "Magda", "Ita"].iter().enumerate() {
        locations.push(MapLocation {
            name: (*name).to_string(),
            angle: PI * 0.25 + (i as f64 * PI * 0.5),
            orbital_radius: 1.5,
            loc_type: MapLocationType::Moon,
            parent: Some("Hurston".to_string()),
        });
    }
}

fn add_crusader_moons(locations: &mut Vec<MapLocation>) {
    for (i, name) in ["Cellin", "Daymar", "Yela"].iter().enumerate() {
        locations.push(MapLocation {
            name: (*name).to_string(),
            angle: PI * 0.75 + (i as f64 * PI * 0.67),
            orbital_radius: 1.8,
            loc_type: MapLocationType::Moon,
            parent: Some("Crusader".to_string()),
        });
    }
}

fn add_arccorp_moons(locations: &mut Vec<MapLocation>) {
    for (i, name) in ["Lyria", "Wala"].iter().enumerate() {
        locations.push(MapLocation {
            name: (*name).to_string(),
            angle: PI * 1.25 + (i as f64 * PI),
            orbital_radius: 1.5,
            loc_type: MapLocationType::Moon,
            parent: Some("ArcCorp".to_string()),
        });
    }
}

fn add_microtech_moons(locations: &mut Vec<MapLocation>) {
    for (i, name) in ["Calliope", "Clio", "Euterpe"].iter().enumerate() {
        locations.push(MapLocation {
            name: (*name).to_string(),
            angle: PI * 1.75 + (i as f64 * PI * 0.67),
            orbital_radius: 1.8,
            loc_type: MapLocationType::Moon,
            parent: Some("microTech".to_string()),
        });
    }
}

fn add_stanton_stations(locations: &mut Vec<MapLocation>) {
    locations.push(MapLocation {
        name: "Port Olisar".to_string(),
        angle: PI * 0.75 + 0.1,
        orbital_radius: 0.5,
        loc_type: MapLocationType::Station,
        parent: Some("Crusader".to_string()),
    });
    locations.push(MapLocation {
        name: "Everus Harbor".to_string(),
        angle: PI * 0.25 + 0.1,
        orbital_radius: 0.5,
        loc_type: MapLocationType::Station,
        parent: Some("Hurston".to_string()),
    });
    locations.push(MapLocation {
        name: "Baijini Point".to_string(),
        angle: PI * 1.25 + 0.1,
        orbital_radius: 0.5,
        loc_type: MapLocationType::Station,
        parent: Some("ArcCorp".to_string()),
    });
    locations.push(MapLocation {
        name: "Port Tressler".to_string(),
        angle: PI * 1.75 + 0.1,
        orbital_radius: 0.5,
        loc_type: MapLocationType::Station,
        parent: Some("microTech".to_string()),
    });
    locations.push(MapLocation {
        name: "Grim HEX".to_string(),
        angle: PI * 0.75 + PI * 0.67 * 2.0 + 0.2,
        orbital_radius: 0.3,
        loc_type: MapLocationType::Station,
        parent: Some("Yela".to_string()),
    });
}

fn build_pyro_locations(locations: &mut Vec<MapLocation>) {
    // Pyro star
    locations.push(MapLocation {
        name: "Pyro".to_string(),
        angle: 0.0,
        orbital_radius: 0.0,
        loc_type: MapLocationType::Star,
        parent: None,
    });

    // Planets spread around the star
    let pyro_planets = [
        ("Pyro I", 5.0, 0.0),
        ("Pyro II", 10.0, PI * 0.4),
        ("Pyro III", 18.0, PI * 0.8),
        ("Pyro IV", 28.0, PI * 1.2),
        ("Pyro V", 40.0, PI * 1.5),
        ("Pyro VI", 55.0, PI * 1.8),
    ];

    for (name, radius, angle) in pyro_planets {
        locations.push(MapLocation {
            name: name.to_string(),
            angle,
            orbital_radius: radius,
            loc_type: MapLocationType::Planet,
            parent: None,
        });
    }

    // Stations
    locations.push(MapLocation {
        name: "Ruin Station".to_string(),
        angle: 0.1,
        orbital_radius: 0.5,
        loc_type: MapLocationType::Station,
        parent: Some("Pyro I".to_string()),
    });
    locations.push(MapLocation {
        name: "Checkmate".to_string(),
        angle: PI * 1.2 + 0.1,
        orbital_radius: 0.5,
        loc_type: MapLocationType::Station,
        parent: Some("Pyro IV".to_string()),
    });
}
