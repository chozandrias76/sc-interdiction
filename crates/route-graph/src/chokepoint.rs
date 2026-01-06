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
