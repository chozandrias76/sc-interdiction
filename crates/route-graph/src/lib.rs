//! Route graph and pathfinding for Star Citizen.
//!
//! Builds a graph of stations/locations and calculates quantum travel routes.
//! Used to identify chokepoints and optimal interdiction positions.

mod chokepoint;
pub mod fuel;
mod graph;
mod locations;
mod spatial;

pub use chokepoint::{find_chokepoints, Chokepoint, InterdictPosition, RoutePair};
pub use fuel::{
    calculate_qt_fuel_consumption, can_complete_route, efficiency_for_size, max_range_mkm,
    QtDriveEfficiency, QT_DRIVE_EFFICIENCY,
};
pub use graph::{Edge, Node, NodeType, RouteGraph};
pub use locations::{
    distance_between, estimate_position, get_system, locations_in_system, same_system,
    LocationPosition, LOCATION_POSITIONS,
};
pub use spatial::{
    find_route_intersections, AltJumpInstruction, IndexedHotspot, IntersectingRoute,
    JumpInstruction, NearbyHotspot, Point3D, RouteIntersection, RouteSegment, SpatialIndex,
};
