//! Route graph and pathfinding for Star Citizen.
//!
//! Builds a graph of stations/locations and calculates quantum travel routes.
//! Used to identify chokepoints and optimal interdiction positions.

mod chokepoint;
mod graph;
mod locations;
mod spatial;

pub use chokepoint::{find_chokepoints, Chokepoint, InterdictPosition, RoutePair};
pub use graph::{Edge, Node, NodeType, RouteGraph};
pub use locations::{
    distance_between, estimate_position, locations_in_system, LocationPosition, LOCATION_POSITIONS,
};
pub use spatial::{
    find_route_intersections, AltJumpInstruction, IndexedHotspot, IntersectingRoute,
    JumpInstruction, NearbyHotspot, Point3D, RouteIntersection, RouteSegment, SpatialIndex,
};
