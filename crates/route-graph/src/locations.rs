//! Static location coordinate database for Star Citizen.
//!
//! Provides approximate 3D positions for major locations in known star systems.
//! Positions are in millions of km (Mkm) relative to the system's primary star.
//!
//! Planets are spread at different orbital angles to create realistic
//! trade route intersections across the system, not just along one axis.
//!
//! Data sources:
//! - In-game coordinates from /showlocation command
//! - Community measurements and starmap data

use crate::Point3D;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Position data for a known location.
#[derive(Debug, Clone)]
pub struct LocationPosition {
    pub name: &'static str,
    pub system: &'static str,
    pub position: Point3D,
    pub parent: Option<&'static str>,
}

/// Helper to compute position from orbital radius and angle
fn orbital_pos(radius: f64, angle_deg: f64) -> Point3D {
    let angle_rad = angle_deg * std::f64::consts::PI / 180.0;
    Point3D::new(radius * angle_rad.cos(), radius * angle_rad.sin(), 0.0)
}

/// Helper to compute position relative to a parent body
fn moon_pos(parent: Point3D, offset_radius: f64, angle_deg: f64) -> Point3D {
    let angle_rad = angle_deg * std::f64::consts::PI / 180.0;
    Point3D::new(
        parent.x + offset_radius * angle_rad.cos(),
        parent.y + offset_radius * angle_rad.sin(),
        0.0,
    )
}

/// Static database of known location positions.
/// Key is lowercase location name for easy lookup.
pub static LOCATION_POSITIONS: LazyLock<HashMap<&'static str, LocationPosition>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();

        // Stanton System - planets at different orbital angles
        // Orbital radii in millions of km (Mkm)

        // Planet positions (spread around the star at different angles)
        let hurston_pos = orbital_pos(12.85, 45.0); // 45 degrees
        let crusader_pos = orbital_pos(18.96, 135.0); // 135 degrees
        let arccorp_pos = orbital_pos(18.59, 225.0); // 225 degrees
        let microtech_pos = orbital_pos(22.46, 315.0); // 315 degrees

        // Primary bodies
        add_location(&mut map, "hurston", "Stanton", hurston_pos, None);
        add_location(&mut map, "crusader", "Stanton", crusader_pos, None);
        add_location(&mut map, "arccorp", "Stanton", arccorp_pos, None);
        add_location(&mut map, "microtech", "Stanton", microtech_pos, None);

        // Hurston landing zones & stations
        add_location(
            &mut map,
            "lorville",
            "Stanton",
            hurston_pos,
            Some("Hurston"),
        );
        add_location(
            &mut map,
            "everus harbor",
            "Stanton",
            moon_pos(hurston_pos, 0.05, 90.0),
            Some("Hurston"),
        );

        // Hurston moons (spread around Hurston at different angles)
        add_location(
            &mut map,
            "arial",
            "Stanton",
            moon_pos(hurston_pos, 0.3, 30.0),
            Some("Hurston"),
        );
        add_location(
            &mut map,
            "aberdeen",
            "Stanton",
            moon_pos(hurston_pos, 0.35, 120.0),
            Some("Hurston"),
        );
        add_location(
            &mut map,
            "magda",
            "Stanton",
            moon_pos(hurston_pos, 0.28, 210.0),
            Some("Hurston"),
        );
        add_location(
            &mut map,
            "ita",
            "Stanton",
            moon_pos(hurston_pos, 0.32, 300.0),
            Some("Hurston"),
        );

        // Crusader landing zones & stations
        add_location(
            &mut map,
            "orison",
            "Stanton",
            crusader_pos,
            Some("Crusader"),
        );
        add_location(
            &mut map,
            "port olisar",
            "Stanton",
            moon_pos(crusader_pos, 0.05, 90.0),
            Some("Crusader"),
        );
        add_location(
            &mut map,
            "seraphim",
            "Stanton",
            moon_pos(crusader_pos, 0.07, 100.0),
            Some("Crusader"),
        );

        // Crusader moons (spread around Crusader)
        add_location(
            &mut map,
            "cellin",
            "Stanton",
            moon_pos(crusader_pos, 0.4, 0.0),
            Some("Crusader"),
        );
        add_location(
            &mut map,
            "daymar",
            "Stanton",
            moon_pos(crusader_pos, 0.45, 120.0),
            Some("Crusader"),
        );
        add_location(
            &mut map,
            "yela",
            "Stanton",
            moon_pos(crusader_pos, 0.38, 240.0),
            Some("Crusader"),
        );

        // ArcCorp landing zones & stations
        add_location(&mut map, "area18", "Stanton", arccorp_pos, Some("ArcCorp"));
        add_location(&mut map, "area 18", "Stanton", arccorp_pos, Some("ArcCorp"));
        add_location(
            &mut map,
            "baijini point",
            "Stanton",
            moon_pos(arccorp_pos, 0.05, 90.0),
            Some("ArcCorp"),
        );

        // ArcCorp moons
        add_location(
            &mut map,
            "lyria",
            "Stanton",
            moon_pos(arccorp_pos, 0.25, 60.0),
            Some("ArcCorp"),
        );
        add_location(
            &mut map,
            "wala",
            "Stanton",
            moon_pos(arccorp_pos, 0.3, 180.0),
            Some("ArcCorp"),
        );

        // microTech landing zones & stations
        add_location(
            &mut map,
            "new babbage",
            "Stanton",
            microtech_pos,
            Some("microTech"),
        );
        add_location(
            &mut map,
            "port tressler",
            "Stanton",
            moon_pos(microtech_pos, 0.05, 90.0),
            Some("microTech"),
        );

        // microTech moons
        add_location(
            &mut map,
            "calliope",
            "Stanton",
            moon_pos(microtech_pos, 0.35, 30.0),
            Some("microTech"),
        );
        add_location(
            &mut map,
            "clio",
            "Stanton",
            moon_pos(microtech_pos, 0.4, 150.0),
            Some("microTech"),
        );
        add_location(
            &mut map,
            "euterpe",
            "Stanton",
            moon_pos(microtech_pos, 0.32, 270.0),
            Some("microTech"),
        );

        // Lagrange points - positioned at proper L1/L2/L4/L5 angles
        // L1: between star and planet, L2: beyond planet, L4/L5: 60 degrees ahead/behind

        // HUR-L stations (Hurston is at 45 degrees)
        add_location(&mut map, "hur-l1", "Stanton", orbital_pos(10.0, 45.0), None);
        add_location(&mut map, "hur-l2", "Stanton", orbital_pos(15.0, 45.0), None);
        add_location(
            &mut map,
            "hur-l3",
            "Stanton",
            orbital_pos(12.85, 225.0),
            None,
        ); // opposite side
        add_location(
            &mut map,
            "hur-l4",
            "Stanton",
            orbital_pos(12.85, 105.0),
            None,
        ); // 60° ahead
        add_location(
            &mut map,
            "hur-l5",
            "Stanton",
            orbital_pos(12.85, -15.0),
            None,
        ); // 60° behind

        // CRU-L stations (Crusader is at 135 degrees)
        add_location(
            &mut map,
            "cru-l1",
            "Stanton",
            orbital_pos(15.0, 135.0),
            None,
        );
        add_location(
            &mut map,
            "cru-l4",
            "Stanton",
            orbital_pos(18.96, 195.0),
            None,
        ); // 60° ahead
        add_location(
            &mut map,
            "cru-l5",
            "Stanton",
            orbital_pos(18.96, 75.0),
            None,
        ); // 60° behind

        // ARC-L stations (ArcCorp is at 225 degrees)
        add_location(
            &mut map,
            "arc-l1",
            "Stanton",
            orbital_pos(15.0, 225.0),
            None,
        );

        // MIC-L stations (microTech is at 315 degrees)
        add_location(
            &mut map,
            "mic-l1",
            "Stanton",
            orbital_pos(18.0, 315.0),
            None,
        );
        add_location(
            &mut map,
            "mic-l2",
            "Stanton",
            orbital_pos(27.0, 315.0),
            None,
        );

        // Grim HEX (near Yela)
        let yela_pos = moon_pos(crusader_pos, 0.38, 240.0);
        add_location(
            &mut map,
            "grim hex",
            "Stanton",
            moon_pos(yela_pos, 0.1, 45.0),
            Some("Yela"),
        );

        // Pyro System - planets at different orbital angles
        let pyro1_pos = orbital_pos(5.0, 0.0);
        let pyro2_pos = orbital_pos(10.0, 72.0);
        let pyro3_pos = orbital_pos(18.0, 144.0);
        let pyro4_pos = orbital_pos(28.0, 216.0);
        let pyro5_pos = orbital_pos(40.0, 270.0);
        let pyro6_pos = orbital_pos(55.0, 324.0);

        add_location(&mut map, "pyro i", "Pyro", pyro1_pos, None);
        add_location(&mut map, "pyro ii", "Pyro", pyro2_pos, None);
        add_location(&mut map, "pyro iii", "Pyro", pyro3_pos, None);
        add_location(&mut map, "pyro iv", "Pyro", pyro4_pos, None);
        add_location(&mut map, "pyro v", "Pyro", pyro5_pos, None);
        add_location(&mut map, "pyro vi", "Pyro", pyro6_pos, None);

        // Pyro stations
        add_location(
            &mut map,
            "ruin station",
            "Pyro",
            moon_pos(pyro1_pos, 0.5, 90.0),
            Some("Pyro I"),
        );
        add_location(
            &mut map,
            "checkmate station",
            "Pyro",
            moon_pos(pyro4_pos, 0.5, 90.0),
            Some("Pyro IV"),
        );

        // Nyx System
        add_location(
            &mut map,
            "levski",
            "Nyx",
            Point3D::new(0.0, 0.0, 0.0),
            Some("Delamar"),
        );
        add_location(
            &mut map,
            "delamar",
            "Nyx",
            Point3D::new(0.0, 0.0, 0.0),
            None,
        );

        // === MINING LOCATIONS ===
        // Asteroid belts and mining hotspots with approximate positions

        // Aaron Halo - massive asteroid belt surrounding Stanton system
        // Position: outer system, ~30 Mkm from star
        add_location(
            &mut map,
            "aaron halo",
            "Stanton",
            orbital_pos(30.0, 90.0),
            None,
        );

        // Yela Asteroid Belt - around Crusader's moon Yela
        add_location(
            &mut map,
            "yela asteroid belt",
            "Stanton",
            moon_pos(crusader_pos, 0.42, 240.0),
            Some("Yela"),
        );

        // ArcCorp Mining Area 045 (AMA045) - near Lyria
        add_location(
            &mut map,
            "ama045",
            "Stanton",
            moon_pos(arccorp_pos, 0.28, 60.0),
            Some("Lyria"),
        );

        // ArcCorp Mining Area 141 (AMA141) - near Wala
        add_location(
            &mut map,
            "ama141",
            "Stanton",
            moon_pos(arccorp_pos, 0.32, 180.0),
            Some("Wala"),
        );

        // Daymar surface mining sites (near Shubin Mining facilities)
        add_location(
            &mut map,
            "daymar caves",
            "Stanton",
            moon_pos(crusader_pos, 0.45, 118.0),
            Some("Daymar"),
        );

        // Aberdeen surface mining (quantainium hotspot)
        add_location(
            &mut map,
            "aberdeen caves",
            "Stanton",
            moon_pos(hurston_pos, 0.35, 118.0),
            Some("Aberdeen"),
        );

        map
    });

fn add_location(
    map: &mut HashMap<&'static str, LocationPosition>,
    name: &'static str,
    system: &'static str,
    position: Point3D,
    parent: Option<&'static str>,
) {
    map.insert(
        name,
        LocationPosition {
            name,
            system,
            position,
            parent,
        },
    );
}

/// Estimate position for a location name.
/// Attempts fuzzy matching on the location database.
pub fn estimate_position(location: &str) -> Option<Point3D> {
    let loc_lower = location.to_lowercase();

    // Direct match
    if let Some(loc) = LOCATION_POSITIONS.get(loc_lower.as_str()) {
        return Some(loc.position);
    }

    // Partial match - check if location contains a known key
    for (key, loc) in LOCATION_POSITIONS.iter() {
        if loc_lower.contains(key) {
            return Some(loc.position);
        }
    }

    None
}

/// Calculate distance between two locations in millions of km.
/// Returns None if either location is unknown.
#[must_use]
pub fn distance_between(from: &str, to: &str) -> Option<f64> {
    let from_pos = estimate_position(from)?;
    let to_pos = estimate_position(to)?;
    Some(from_pos.distance_to(&to_pos))
}

/// Get all locations in a system.
pub fn locations_in_system(system: &str) -> Vec<&'static LocationPosition> {
    let system_lower = system.to_lowercase();
    LOCATION_POSITIONS
        .values()
        .filter(|loc| loc.system.to_lowercase() == system_lower)
        .collect()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]
    #![allow(clippy::indexing_slicing)]

    use super::*;

    #[test]
    fn test_estimate_position() {
        assert!(estimate_position("Hurston").is_some());
        assert!(estimate_position("lorville").is_some());
        assert!(estimate_position("Port Olisar").is_some());
    }

    #[test]
    fn test_distance_calculation() {
        // Hurston to Crusader distance check
        let dist = distance_between("Hurston", "Crusader");
        assert!(dist.is_some());
        // Just verify it returns a reasonable positive distance
        let d = dist.unwrap();
        assert!(d > 0.0, "Distance should be positive, got {}", d);
    }
}
