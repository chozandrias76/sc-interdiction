//! Route graph and pathfinding for Star Citizen.
//!
//! Builds a graph of stations/locations and calculates quantum travel routes.
//! Used to identify chokepoints and optimal interdiction positions.

mod graph;
mod chokepoint;
mod spatial;

pub use graph::{RouteGraph, Node, Edge, NodeType};
pub use chokepoint::{Chokepoint, RoutePair, InterdictPosition, find_chokepoints};
pub use spatial::{Point3D, SpatialIndex, IndexedHotspot, NearbyHotspot};
