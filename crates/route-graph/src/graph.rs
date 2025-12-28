//! Station graph construction and pathfinding.

use api_client::{Station, Terminal};
use ordered_float::OrderedFloat;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::dijkstra;
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
    pub fn from_str(s: &str) -> Self {
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
            node_type: NodeType::from_str(&station.station_type),
            system: station.system_code.clone(),
            parent_body: station.parent_name.clone(),
            coords: None,
        };

        let idx = self.graph.add_node(node);
        self.node_indices.insert(station.code.clone(), idx);
        idx
    }

    /// Add a node from a Terminal.
    pub fn add_terminal(&mut self, terminal: &Terminal) -> NodeIndex {
        if let Some(&idx) = self.node_indices.get(&terminal.code) {
            return idx;
        }

        let node = Node {
            id: terminal.id.to_string(),
            name: terminal.name.clone(),
            node_type: NodeType::from_str(&terminal.terminal_type),
            system: terminal.star_system_name.clone(),
            parent_body: if !terminal.moon_name.is_empty() {
                terminal.moon_name.clone()
            } else {
                terminal.planet_name.clone()
            },
            coords: None,
        };

        let idx = self.graph.add_node(node);
        self.node_indices.insert(terminal.code.clone(), idx);
        idx
    }

    /// Connect two nodes with a route.
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

                // Estimate distance based on typical intra-system distances
                // TODO: Use actual coordinates when available
                let distance = 500_000.0; // ~500k km default

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

        let distances = dijkstra(&self.graph, from_idx, Some(to_idx), |e| {
            OrderedFloat(e.weight().travel_time)
        });

        if !distances.contains_key(&to_idx) {
            return Err(GraphError::NoPath {
                from: from_code.to_string(),
                to: to_code.to_string(),
            });
        }

        // Reconstruct path (simplified - just returns node codes for now)
        // TODO: Implement proper path reconstruction
        Ok(vec![from_code.to_string(), to_code.to_string()])
    }

    /// Get all nodes in the graph.
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.graph.node_weights()
    }

    /// Get node by code.
    pub fn get_node(&self, code: &str) -> Option<&Node> {
        self.node_indices
            .get(code)
            .map(|&idx| &self.graph[idx])
    }

    /// Get number of connections for a node (degree).
    pub fn node_degree(&self, code: &str) -> usize {
        self.node_indices
            .get(code)
            .map(|&idx| self.graph.neighbors(idx).count())
            .unwrap_or(0)
    }

    /// Get all edges from a node.
    pub fn edges_from(&self, code: &str) -> Vec<(&Node, &Edge)> {
        let Some(&idx) = self.node_indices.get(code) else {
            return Vec::new();
        };

        self.graph
            .neighbors(idx)
            .map(|neighbor_idx| {
                let edge_idx = self.graph.find_edge(idx, neighbor_idx).expect("edge exists");
                (&self.graph[neighbor_idx], &self.graph[edge_idx])
            })
            .collect()
    }

    /// Total number of nodes.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Total number of edges.
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

impl Default for RouteGraph {
    fn default() -> Self {
        Self::new()
    }
}
