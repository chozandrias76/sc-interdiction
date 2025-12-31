//! Station graph construction and pathfinding.

use api_client::{Station, Terminal};
use ordered_float::OrderedFloat;
use petgraph::algo::astar;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors from route graph operations.
#[derive(Debug, Error)]
pub enum GraphError {
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("No path exists between {from} and {to}")]
    NoPath { from: String, to: String },
}

pub type Result<T> = std::result::Result<T, GraphError>;

/// A node in the route graph (station/terminal).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub system: String,
    pub parent_body: String,
    /// Coordinates if available (x, y, z).
    pub coords: Option<(f64, f64, f64)>,
    /// Whether this location offers refueling services.
    pub is_fuel_station: bool,
}

/// Type of location node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Station,
    Outpost,
    LandingZone,
    City,
    OrbitalMarker,
}

impl NodeType {
    /// Parse a node type from a string.
    #[must_use]
    pub fn parse(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "STATION" => Self::Station,
            "OUTPOST" => Self::Outpost,
            "LANDING_ZONE" => Self::LandingZone,
            "CITY" => Self::City,
            _ => Self::OrbitalMarker,
        }
    }
}

/// An edge in the route graph (quantum travel path).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// Distance in km (estimated).
    pub distance: f64,
    /// Estimated travel time in seconds (based on avg QT speed).
    pub travel_time: f64,
    /// Whether this route crosses planetary bodies (potential obstruction).
    pub has_obstruction: bool,
}

/// Graph of stations connected by quantum travel routes.
pub struct RouteGraph {
    graph: DiGraph<Node, Edge>,
    node_indices: HashMap<String, NodeIndex>,
}

impl RouteGraph {
    /// Create an empty route graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }

    /// Add a node from a Station.
    pub fn add_station(&mut self, station: &Station) -> NodeIndex {
        if let Some(&idx) = self.node_indices.get(&station.code) {
            return idx;
        }

        let node = Node {
            id: station.id.clone(),
            name: station.name.clone(),
            node_type: NodeType::parse(&station.station_type),
            system: station.system_code.clone(),
            parent_body: station.parent_name.clone(),
            coords: None,
            is_fuel_station: false, // Station data doesn't include refuel info
        };

        let idx = self.graph.add_node(node);
        self.node_indices.insert(station.code.clone(), idx);
        idx
    }

    /// Add a node from a Terminal.
    pub fn add_terminal(&mut self, terminal: &Terminal) -> NodeIndex {
        let code = terminal.code.clone().unwrap_or_default();
        if let Some(&idx) = self.node_indices.get(&code) {
            return idx;
        }

        let node = Node {
            id: terminal.id.to_string(),
            name: terminal.name.clone().unwrap_or_default(),
            node_type: NodeType::parse(&terminal.terminal_type.clone().unwrap_or_default()),
            system: terminal.star_system_name.clone().unwrap_or_default(),
            parent_body: terminal
                .moon_name
                .clone()
                .filter(|m| !m.is_empty())
                .or_else(|| terminal.planet_name.clone())
                .unwrap_or_default(),
            coords: None,
            is_fuel_station: terminal.is_refuel,
        };

        let idx = self.graph.add_node(node);
        self.node_indices.insert(code, idx);
        idx
    }

    /// Connect two nodes with a route.
    ///
    /// # Errors
    ///
    /// Returns an error if either node code is not found in the graph.
    pub fn connect(&mut self, from_code: &str, to_code: &str, distance: f64) -> Result<()> {
        let from_idx = self
            .node_indices
            .get(from_code)
            .copied()
            .ok_or_else(|| GraphError::NodeNotFound(from_code.to_string()))?;

        let to_idx = self
            .node_indices
            .get(to_code)
            .copied()
            .ok_or_else(|| GraphError::NodeNotFound(to_code.to_string()))?;

        // Estimate travel time: avg QT speed ~0.2c for S1 drive
        // 0.2c = ~60,000 km/s, plus spool/calibration time
        let travel_time = (distance / 60_000.0) + 10.0; // +10s for spool

        let edge = Edge {
            distance,
            travel_time,
            has_obstruction: false,
        };

        self.graph.add_edge(from_idx, to_idx, edge.clone());
        self.graph.add_edge(to_idx, from_idx, edge); // Bidirectional

        Ok(())
    }

    /// Connect all nodes within the same system (full mesh within system).
    #[allow(clippy::indexing_slicing)] // Loop bounds guarantee valid indices
    pub fn connect_system(&mut self, system: &str) {
        let system_nodes: Vec<_> = self
            .node_indices
            .iter()
            .filter_map(|(code, &idx)| {
                let node = &self.graph[idx];
                if node.system == system {
                    Some((code.clone(), idx))
                } else {
                    None
                }
            })
            .collect();

        // Connect every pair
        for i in 0..system_nodes.len() {
            for j in (i + 1)..system_nodes.len() {
                let (_, idx_a) = &system_nodes[i];
                let (_, idx_b) = &system_nodes[j];

                let node_a = &self.graph[*idx_a];
                let node_b = &self.graph[*idx_b];

                // Calculate distance using actual coordinates when available
                let distance = match (node_a.coords, node_b.coords) {
                    (Some((x1, y1, z1)), Some((x2, y2, z2))) => {
                        // Calculate Euclidean distance in 3D space
                        let dx = x2 - x1;
                        let dy = y2 - y1;
                        let dz = z2 - z1;
                        (dx * dx + dy * dy + dz * dz).sqrt()
                    }
                    _ => {
                        // Fall back to default estimate when coordinates not available
                        500_000.0 // ~500k km default
                    }
                };

                let edge = Edge {
                    distance,
                    travel_time: (distance / 60_000.0) + 10.0,
                    has_obstruction: false,
                };

                self.graph.add_edge(*idx_a, *idx_b, edge.clone());
                self.graph.add_edge(*idx_b, *idx_a, edge);
            }
        }
    }

    /// Find shortest path between two nodes.
    ///
    /// # Errors
    ///
    /// Returns an error if either node code is not found or no path exists.
    pub fn find_path(&self, from_code: &str, to_code: &str) -> Result<Vec<String>> {
        let from_idx = self
            .node_indices
            .get(from_code)
            .copied()
            .ok_or_else(|| GraphError::NodeNotFound(from_code.to_string()))?;

        let to_idx = self
            .node_indices
            .get(to_code)
            .copied()
            .ok_or_else(|| GraphError::NodeNotFound(to_code.to_string()))?;

        // Use A* algorithm to find the shortest path
        // The heuristic is 0 (making it equivalent to Dijkstra but with path reconstruction)
        let result = astar(
            &self.graph,
            from_idx,
            |idx| idx == to_idx,
            |e| OrderedFloat(e.weight().travel_time),
            |_| OrderedFloat(0.0), // Zero heuristic = Dijkstra with path
        );

        match result {
            Some((_cost, path)) => {
                // Convert node indices to node codes
                let codes: Vec<String> = path
                    .into_iter()
                    .map(|idx| self.graph[idx].id.clone())
                    .collect();
                Ok(codes)
            }
            None => Err(GraphError::NoPath {
                from: from_code.to_string(),
                to: to_code.to_string(),
            }),
        }
    }

    /// Get all nodes in the graph.
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.graph.node_weights()
    }

    /// Get node by code.
    #[must_use]
    pub fn get_node(&self, code: &str) -> Option<&Node> {
        self.node_indices.get(code).map(|&idx| &self.graph[idx])
    }

    /// Get number of connections for a node (degree).
    #[must_use]
    pub fn node_degree(&self, code: &str) -> usize {
        self.node_indices
            .get(code)
            .map(|&idx| self.graph.neighbors(idx).count())
            .unwrap_or(0)
    }

    /// Get all edges from a node.
    #[must_use]
    pub fn edges_from(&self, code: &str) -> Vec<(&Node, &Edge)> {
        let Some(&idx) = self.node_indices.get(code) else {
            return Vec::new();
        };

        self.graph
            .neighbors(idx)
            .filter_map(|neighbor_idx| {
                let edge_idx = self.graph.find_edge(idx, neighbor_idx)?;
                Some((&self.graph[neighbor_idx], &self.graph[edge_idx]))
            })
            .collect()
    }

    /// Total number of nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Total number of edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

impl Default for RouteGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_terminal(
        id: i64,
        name: &str,
        code: &str,
        terminal_type: &str,
        system: &str,
        is_refuel: bool,
    ) -> Terminal {
        Terminal {
            id,
            name: Some(name.to_string()),
            code: Some(code.to_string()),
            nickname: None,
            terminal_type: Some(terminal_type.to_string()),
            star_system_name: Some(system.to_string()),
            planet_name: Some("TestPlanet".to_string()),
            moon_name: None,
            space_station_name: None,
            outpost_name: None,
            city_name: None,
            has_freight_elevator: false,
            has_loading_dock: false,
            has_docking_port: false,
            is_refuel,
            is_refinery: false,
        }
    }

    #[test]
    fn test_new_graph_is_empty() {
        let graph = RouteGraph::new();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_add_terminal() {
        let mut graph = RouteGraph::new();
        let terminal = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);

        graph.add_terminal(&terminal);

        assert_eq!(graph.node_count(), 1);
        let node = graph.get_node("PO").unwrap();
        assert_eq!(node.name, "Port Olisar");
        assert_eq!(node.system, "Stanton");
        assert!(node.is_fuel_station);
    }

    #[test]
    fn test_add_terminal_idempotent() {
        let mut graph = RouteGraph::new();
        let terminal = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);

        graph.add_terminal(&terminal);
        graph.add_terminal(&terminal);

        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn test_node_type_parse() {
        assert_eq!(NodeType::parse("STATION"), NodeType::Station);
        assert_eq!(NodeType::parse("OUTPOST"), NodeType::Outpost);
        assert_eq!(NodeType::parse("LANDING_ZONE"), NodeType::LandingZone);
        assert_eq!(NodeType::parse("CITY"), NodeType::City);
        assert_eq!(NodeType::parse("UNKNOWN"), NodeType::OrbitalMarker);
    }

    #[test]
    fn test_connect_nodes() {
        let mut graph = RouteGraph::new();
        let terminal1 = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);
        let terminal2 = create_test_terminal(2, "Area18", "A18", "CITY", "Stanton", false);

        graph.add_terminal(&terminal1);
        graph.add_terminal(&terminal2);

        let result = graph.connect("PO", "A18", 1000.0);
        assert!(result.is_ok());

        // Should create bidirectional edge
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_connect_nonexistent_node() {
        let mut graph = RouteGraph::new();
        let result = graph.connect("INVALID", "ALSO_INVALID", 1000.0);

        assert!(result.is_err());
        match result {
            Err(GraphError::NodeNotFound(code)) => assert_eq!(code, "INVALID"),
            _ => panic!("Expected NodeNotFound error"),
        }
    }

    #[test]
    fn test_connect_system() {
        let mut graph = RouteGraph::new();
        let terminal1 = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);
        let terminal2 = create_test_terminal(2, "Area18", "A18", "CITY", "Stanton", false);
        let terminal3 =
            create_test_terminal(3, "Lorville", "LOR", "CITY", "Stanton", false);

        graph.add_terminal(&terminal1);
        graph.add_terminal(&terminal2);
        graph.add_terminal(&terminal3);

        graph.connect_system("Stanton");

        // 3 nodes should create 6 edges (3 pairs × 2 directions)
        assert_eq!(graph.edge_count(), 6);
    }

    #[test]
    fn test_connect_system_with_different_systems() {
        let mut graph = RouteGraph::new();
        let terminal1 = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);
        let terminal2 = create_test_terminal(2, "Pyro Station", "PYRO", "STATION", "Pyro", false);

        graph.add_terminal(&terminal1);
        graph.add_terminal(&terminal2);

        graph.connect_system("Stanton");

        // Only Stanton nodes should be connected (0 edges since only 1 Stanton node)
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_find_path() {
        let mut graph = RouteGraph::new();
        let terminal1 = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);
        let terminal2 = create_test_terminal(2, "Area18", "A18", "CITY", "Stanton", false);

        graph.add_terminal(&terminal1);
        graph.add_terminal(&terminal2);
        graph.connect("PO", "A18", 1000.0).unwrap();

        let path = graph.find_path("PO", "A18").unwrap();

        assert_eq!(path.len(), 2);
        assert_eq!(path[0], "1");
        assert_eq!(path[1], "2");
    }

    #[test]
    fn test_find_path_no_connection() {
        let mut graph = RouteGraph::new();
        let terminal1 = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);
        let terminal2 = create_test_terminal(2, "Area18", "A18", "CITY", "Stanton", false);

        graph.add_terminal(&terminal1);
        graph.add_terminal(&terminal2);

        let result = graph.find_path("PO", "A18");

        assert!(result.is_err());
        match result {
            Err(GraphError::NoPath { from, to }) => {
                assert_eq!(from, "PO");
                assert_eq!(to, "A18");
            }
            _ => panic!("Expected NoPath error"),
        }
    }

    #[test]
    fn test_get_node() {
        let mut graph = RouteGraph::new();
        let terminal = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);

        graph.add_terminal(&terminal);

        let node = graph.get_node("PO");
        assert!(node.is_some());
        assert_eq!(node.unwrap().name, "Port Olisar");
    }

    #[test]
    fn test_get_node_not_found() {
        let graph = RouteGraph::new();
        let node = graph.get_node("INVALID");
        assert!(node.is_none());
    }

    #[test]
    fn test_node_degree() {
        let mut graph = RouteGraph::new();
        let terminal1 = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);
        let terminal2 = create_test_terminal(2, "Area18", "A18", "CITY", "Stanton", false);
        let terminal3 =
            create_test_terminal(3, "Lorville", "LOR", "CITY", "Stanton", false);

        graph.add_terminal(&terminal1);
        graph.add_terminal(&terminal2);
        graph.add_terminal(&terminal3);

        graph.connect_system("Stanton");

        // Each node should be connected to 2 others
        assert_eq!(graph.node_degree("PO"), 2);
        assert_eq!(graph.node_degree("A18"), 2);
        assert_eq!(graph.node_degree("LOR"), 2);
    }

    #[test]
    fn test_edges_from() {
        let mut graph = RouteGraph::new();
        let terminal1 = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);
        let terminal2 = create_test_terminal(2, "Area18", "A18", "CITY", "Stanton", false);

        graph.add_terminal(&terminal1);
        graph.add_terminal(&terminal2);
        graph.connect("PO", "A18", 1000.0).unwrap();

        let edges = graph.edges_from("PO");

        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].0.name, "Area18");
        assert_eq!(edges[0].1.distance, 1000.0);
    }

    #[test]
    fn test_edges_from_nonexistent() {
        let graph = RouteGraph::new();
        let edges = graph.edges_from("INVALID");
        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn test_nodes_iterator() {
        let mut graph = RouteGraph::new();
        let terminal1 = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);
        let terminal2 = create_test_terminal(2, "Area18", "A18", "CITY", "Stanton", false);

        graph.add_terminal(&terminal1);
        graph.add_terminal(&terminal2);

        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn test_travel_time_calculation() {
        let mut graph = RouteGraph::new();
        let terminal1 = create_test_terminal(1, "Port Olisar", "PO", "STATION", "Stanton", true);
        let terminal2 = create_test_terminal(2, "Area18", "A18", "CITY", "Stanton", false);

        graph.add_terminal(&terminal1);
        graph.add_terminal(&terminal2);
        graph.connect("PO", "A18", 6000000.0).unwrap(); // 6M km

        let edges = graph.edges_from("PO");

        // Travel time should be (6000000 / 60000) + 10 = 110 seconds
        assert!((edges[0].1.travel_time - 110.0).abs() < 0.01);
    }
}
