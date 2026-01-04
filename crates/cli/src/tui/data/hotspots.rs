//! Hotspot calculation from trade routes.

use intel::HotRoute;
use route_graph::{find_route_intersections, RouteIntersection, RouteSegment};

/// Load hotspots from routes by finding intersections.
pub fn load_hotspots(routes: &[HotRoute]) -> Vec<RouteIntersection> {
    // Convert hot routes to route segments for intersection analysis
    let segments: Vec<RouteSegment> = routes
        .iter()
        .filter_map(|route| {
            let origin_pos = route_graph::estimate_position(&route.origin)?;
            let dest_pos = route_graph::estimate_position(&route.destination)?;

            Some(RouteSegment {
                origin: origin_pos,
                destination: dest_pos,
                origin_name: route.origin.clone(),
                destination_name: route.destination.clone(),
                origin_system: extract_system_from_location(&route.origin),
                destination_system: extract_system_from_location(&route.destination),
                cargo_value: route.estimated_haul_value,
                commodity: route.commodity.clone(),
                ship_name: route.likely_ship.name.to_string(),
                threat_level: route.likely_ship.threat_level,
            })
        })
        .collect();

    // Find intersections with reasonable proximity (2 Mkm = 2 million km)
    let mut intersections = find_route_intersections(&segments, 2.0, 2);

    // Sort by route count (traffic) for display
    intersections.sort_by(|a, b| b.route_pair_count.cmp(&a.route_pair_count));

    // Take top 20 hotspots
    intersections.truncate(20);

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
