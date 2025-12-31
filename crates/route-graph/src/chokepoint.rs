//! Chokepoint detection for interdiction planning.
//!
//! Identifies locations where multiple trade routes converge,
//! making them ideal for interdiction.

use crate::graph::{Node, RouteGraph};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A chokepoint where multiple routes converge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chokepoint {
    /// The node that serves as the chokepoint.
    pub node: Node,
    /// Number of routes passing through this point.
    pub route_count: usize,
    /// Total estimated daily traffic (based on route profitability).
    pub traffic_score: f64,
    /// List of route pairs that pass through here.
    pub routes: Vec<RoutePair>,
    /// Suggested interdiction position (offset from node).
    pub suggested_position: InterdictPosition,
}

/// A pair of origin/destination that forms a route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutePair {
    pub origin: String,
    pub destination: String,
    pub profit_per_scu: f64,
}

/// Suggested position for interdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterdictPosition {
    /// Description of where to position.
    pub description: String,
    /// Distance from the node (km).
    pub distance_km: f64,
    /// Direction hint.
    pub direction: String,
}

/// Find chokepoints in the route graph.
///
/// Analyzes which nodes are crossed by the most profitable trade routes.
#[must_use]
pub fn find_chokepoints(
    graph: &RouteGraph,
    trade_routes: &[(String, String, f64)], // (origin, dest, profit_per_scu)
) -> Vec<Chokepoint> {
    // Count how many routes pass through each node
    let mut node_traffic: HashMap<String, Vec<RoutePair>> = HashMap::new();

    for (origin, dest, profit) in trade_routes {
        // For now, we assume direct routes (origin -> dest)
        // In reality, we'd calculate intermediate waypoints

        // Add traffic to origin and destination
        let route = RoutePair {
            origin: origin.clone(),
            destination: dest.clone(),
            profit_per_scu: *profit,
        };

        node_traffic
            .entry(origin.clone())
            .or_default()
            .push(route.clone());

        node_traffic.entry(dest.clone()).or_default().push(route);
    }

    // Convert to chokepoints, sorted by traffic
    let mut chokepoints: Vec<Chokepoint> = node_traffic
        .into_iter()
        .filter_map(|(code, routes)| {
            let node = graph.get_node(&code)?.clone();
            let route_count = routes.len();
            let traffic_score: f64 = routes.iter().map(|r| r.profit_per_scu).sum();

            Some(Chokepoint {
                suggested_position: suggest_interdict_position(&node),
                node,
                route_count,
                traffic_score,
                routes,
            })
        })
        .collect();

    // Sort by traffic score (highest first)
    chokepoints.sort_by(|a, b| {
        b.traffic_score
            .partial_cmp(&a.traffic_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    chokepoints
}

/// Suggest an interdiction position near a node.
fn suggest_interdict_position(node: &Node) -> InterdictPosition {
    // Interdiction works best at specific distances from QT destinations
    // Mantis range is ~20km, so position 100-200km out on approach

    let (distance, direction) = match node.node_type {
        crate::graph::NodeType::Station => {
            // Stations are in orbit - interdict on approach from planet
            (150.0, format!("Between {} and station", node.parent_body))
        }
        crate::graph::NodeType::Outpost => {
            // Outposts on surface - interdict in low orbit above
            (100.0, "Low orbit above outpost".to_string())
        }
        crate::graph::NodeType::LandingZone | crate::graph::NodeType::City => {
            // Major landing zones have heavy traffic corridors
            (200.0, "Main approach corridor".to_string())
        }
        crate::graph::NodeType::OrbitalMarker => {
            // OM points - great natural chokepoints
            (50.0, "Near OM marker".to_string())
        }
    };

    InterdictPosition {
        description: format!(
            "Position {}km from {} on {}",
            distance, node.name, direction
        ),
        distance_km: distance,
        direction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{NodeType, RouteGraph};

    fn create_test_graph() -> RouteGraph {
        let mut graph = RouteGraph::new();

        // Create test terminal data
        let terminal1 = api_client::Terminal {
            id: 1,
            name: Some("Port Olisar".to_string()),
            code: Some("PO".to_string()),
            nickname: None,
            terminal_type: Some("STATION".to_string()),
            star_system_name: Some("Stanton".to_string()),
            planet_name: Some("Crusader".to_string()),
            moon_name: None,
            space_station_name: None,
            outpost_name: None,
            city_name: None,
            has_freight_elevator: false,
            has_loading_dock: false,
            has_docking_port: false,
            is_refuel: true,
            is_refinery: false,
        };

        let terminal2 = api_client::Terminal {
            id: 2,
            name: Some("Area18".to_string()),
            code: Some("A18".to_string()),
            nickname: None,
            terminal_type: Some("CITY".to_string()),
            star_system_name: Some("Stanton".to_string()),
            planet_name: Some("ArcCorp".to_string()),
            moon_name: None,
            space_station_name: None,
            outpost_name: None,
            city_name: None,
            has_freight_elevator: false,
            has_loading_dock: false,
            has_docking_port: false,
            is_refuel: false,
            is_refinery: false,
        };

        graph.add_terminal(&terminal1);
        graph.add_terminal(&terminal2);

        graph
    }

    #[test]
    fn test_find_chokepoints_empty() {
        let graph = create_test_graph();
        let trade_routes = vec![];

        let chokepoints = find_chokepoints(&graph, &trade_routes);

        assert_eq!(chokepoints.len(), 0);
    }

    #[test]
    fn test_find_chokepoints_single_route() {
        let graph = create_test_graph();
        let trade_routes = vec![("PO".to_string(), "A18".to_string(), 100.0)];

        let chokepoints = find_chokepoints(&graph, &trade_routes);

        // Should have 2 chokepoints (one for each terminal)
        assert_eq!(chokepoints.len(), 2);
    }

    #[test]
    fn test_find_chokepoints_sorted_by_traffic() {
        let graph = create_test_graph();
        let trade_routes = vec![
            ("PO".to_string(), "A18".to_string(), 50.0),
            ("PO".to_string(), "A18".to_string(), 100.0),
            ("A18".to_string(), "PO".to_string(), 25.0),
        ];

        let chokepoints = find_chokepoints(&graph, &trade_routes);

        // First chokepoint should have highest traffic score
        assert!(chokepoints[0].traffic_score >= chokepoints[1].traffic_score);
    }

    #[test]
    fn test_find_chokepoints_route_count() {
        let graph = create_test_graph();
        let trade_routes = vec![
            ("PO".to_string(), "A18".to_string(), 50.0),
            ("PO".to_string(), "A18".to_string(), 100.0),
        ];

        let chokepoints = find_chokepoints(&graph, &trade_routes);

        // PO should have 2 routes
        let po_chokepoint = chokepoints.iter().find(|c| c.node.id == "1").unwrap();
        assert_eq!(po_chokepoint.route_count, 2);
    }

    #[test]
    fn test_suggest_interdict_position_station() {
        let node = Node {
            id: "1".to_string(),
            name: "Test Station".to_string(),
            node_type: NodeType::Station,
            system: "Stanton".to_string(),
            parent_body: "Crusader".to_string(),
            coords: None,
            is_fuel_station: true,
        };

        let position = suggest_interdict_position(&node);

        assert_eq!(position.distance_km, 150.0);
        assert!(position.description.contains("Test Station"));
    }

    #[test]
    fn test_suggest_interdict_position_outpost() {
        let node = Node {
            id: "2".to_string(),
            name: "Test Outpost".to_string(),
            node_type: NodeType::Outpost,
            system: "Stanton".to_string(),
            parent_body: "Daymar".to_string(),
            coords: None,
            is_fuel_station: false,
        };

        let position = suggest_interdict_position(&node);

        assert_eq!(position.distance_km, 100.0);
        assert_eq!(position.direction, "Low orbit above outpost");
    }

    #[test]
    fn test_suggest_interdict_position_city() {
        let node = Node {
            id: "3".to_string(),
            name: "Area18".to_string(),
            node_type: NodeType::City,
            system: "Stanton".to_string(),
            parent_body: "ArcCorp".to_string(),
            coords: None,
            is_fuel_station: false,
        };

        let position = suggest_interdict_position(&node);

        assert_eq!(position.distance_km, 200.0);
        assert_eq!(position.direction, "Main approach corridor");
    }

    #[test]
    fn test_suggest_interdict_position_orbital_marker() {
        let node = Node {
            id: "4".to_string(),
            name: "CRU-L1".to_string(),
            node_type: NodeType::OrbitalMarker,
            system: "Stanton".to_string(),
            parent_body: "Crusader".to_string(),
            coords: None,
            is_fuel_station: false,
        };

        let position = suggest_interdict_position(&node);

        assert_eq!(position.distance_km, 50.0);
        assert_eq!(position.direction, "Near OM marker");
    }
}
