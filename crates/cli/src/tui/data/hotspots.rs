//! Hotspot calculation from trade routes.

use intel::HotRoute;
use route_graph::{find_route_intersections, RouteIntersection, RouteSegment};

/// Load hotspots from routes by finding intersections.
/// Returns ALL hotspots for later filtering by proximity to location.
pub fn load_hotspots(routes: &[HotRoute]) -> Vec<RouteIntersection> {
    // Convert hot routes to route segments for intersection analysis
    let segments: Vec<RouteSegment> = routes
        .iter()
        .filter_map(|route| {
            let origin_pos = route_graph::estimate_position(&route.origin)?;
            let dest_pos = route_graph::estimate_position(&route.destination)?;

            // Use the system fields from HotRoute if available, otherwise extract
            let origin_system = route
                .origin_system
                .clone()
                .or_else(|| extract_system_from_location(&route.origin));
            let destination_system = route
                .destination_system
                .clone()
                .or_else(|| extract_system_from_location(&route.destination));

            Some(RouteSegment {
                origin: origin_pos,
                destination: dest_pos,
                origin_name: route.origin.clone(),
                destination_name: route.destination.clone(),
                origin_system,
                destination_system,
                cargo_value: route.estimated_haul_value,
                commodity: route.commodity.clone(),
                ship_name: route.likely_ship.name.to_string(),
                threat_level: route.likely_ship.threat_level,
            })
        })
        .collect();

    // Find intersections with reasonable proximity (2 Mkm = 2 million km)
    // Minimum of 1 route pair to show single high-value routes
    let mut intersections = find_route_intersections(&segments, 2.0, 1);

    // Sort by route count (traffic) for display
    intersections.sort_by(|a, b| b.route_pair_count.cmp(&a.route_pair_count));

    // Return top 200 hotspots for location-based filtering
    intersections.truncate(200);

    intersections
}

/// Extract system name from location string.
/// Format: "Terminal Name (System > Planet > ...)"
fn extract_system_from_location(location: &str) -> Option<String> {
    if let Some(start) = location.find('(') {
        if let Some(end) = location.find('>') {
            let system = location[start + 1..end].trim();
            if !system.is_empty() {
                return Some(system.to_string());
            }
        }
    }
    None
}

/// Filter hotspots to those relevant for a specific location/system.
/// Returns hotspots sorted by proximity to the location.
/// By default, filters OUT cross-system hotspots unless allow_cross_system is true.
#[allow(dead_code)]
pub fn filter_hotspots_for_location(
    hotspots: &[RouteIntersection],
    location: &str,
) -> Vec<RouteIntersection> {
    filter_hotspots_for_location_with_cross_system(hotspots, location, false)
}

/// Filter hotspots with optional cross-system routes.
///
/// TODO: This feature and its related methods need to be updated to have a better
/// source of truth that doesn't require partial string matching anywhere. There is
/// a lot wrong with this module but it's worth it for the first pass.
pub fn filter_hotspots_for_location_with_cross_system(
    hotspots: &[RouteIntersection],
    location: &str,
    allow_cross_system: bool,
) -> Vec<RouteIntersection> {
    let loc_lower = location.to_lowercase();

    // Get coordinates of the target location
    let location_coords = route_graph::estimate_position(location);

    // Determine what system we're filtering for
    let filter_system = if loc_lower == "stanton" || loc_lower == "pyro" || loc_lower == "nyx" {
        Some(location.to_string())
    } else {
        // Infer system from location (e.g., "Crusader" -> "Stanton")
        Some(infer_system_from_location(location))
    };

    let mut filtered: Vec<(RouteIntersection, f64)> = hotspots
        .iter()
        .filter_map(|hotspot| {
            // Filter OUT cross-system hotspots by default
            if !allow_cross_system && hotspot.is_cross_system {
                return None;
            }

            // Filter by system - hotspot must be in the same system
            if let Some(ref sys) = filter_system {
                if !hotspot.system.eq_ignore_ascii_case(sys) {
                    return None;
                }
            }

            // Calculate distance to location if we have coordinates
            let distance = if let Some(loc_pos) = location_coords {
                hotspot.position.distance_to(&loc_pos)
            } else {
                // No coordinates - just use traffic score as priority
                -(hotspot.route_pair_count as f64)
            };

            Some((hotspot.clone(), distance))
        })
        .collect();

    // Sort by distance (closest first) or by traffic if no coords
    filtered.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    // Return top 100 closest hotspots
    filtered.truncate(100);
    filtered.into_iter().map(|(h, _)| h).collect()
}

/// Infer system name from a location string.
fn infer_system_from_location(location: &str) -> String {
    // PRIMARY: Parse system from UEX route format "(SystemName > ...)"
    if let Some(start) = location.find('(') {
        if let Some(end) = location.find('>') {
            let system_part = location[start + 1..end].trim();
            if !system_part.is_empty() {
                return system_part.to_string();
            }
        }
    }

    // FALLBACK: Keyword matching for simple location names
    let loc_lower = location.to_lowercase();

    // Partial Stanton location examples - we don't want to hard code Stanton locations.
    // We will have a better way to get these names later.
    let partial_stanton_location_examples = [
        "hurston",
        "crusader",
        "arccorp",
        "microtech",
        "lorville",
        "orison",
        "area18",
        "new babbage",
        "everus",
        "baijini",
        "tressler",
        "seraphim",
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

    for loc in partial_stanton_location_examples {
        if loc_lower.contains(loc) {
            return "Stanton".to_string();
        }
    }

    // Pyro system locations
    let pyro_locations = ["pyro", "monox", "vatra", "bloom", "terminus"];
    for loc in pyro_locations {
        if loc_lower.contains(loc) {
            return "Pyro".to_string();
        }
    }

    // Nyx system
    let nyx_locations = ["levski", "nyx", "delamar"];
    for loc in nyx_locations {
        if loc_lower.contains(loc) {
            return "Nyx".to_string();
        }
    }

    "Stanton".to_string() // Default
}
