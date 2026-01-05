//! Route graph and pathfinding for Star Citizen.
//!
//! Builds a graph of stations/locations and calculates quantum travel routes.
//! Used to identify chokepoints and optimal interdiction positions.

mod chokepoint;
pub mod fuel;
mod graph;
mod locations;
pub mod mining;
pub mod refinery;
mod spatial;

pub use chokepoint::{find_chokepoints, Chokepoint, InterdictPosition, RoutePair};
pub use fuel::{
    calculate_qt_fuel_consumption, calculate_refuel_cost, calculate_route_refuel_cost,
    can_complete_route, efficiency_for_size, find_route_with_refueling, max_range_mkm,
    FuelStationIndex, QtDriveEfficiency, Waypoint, HYDROGEN_FUEL_PRICE_PER_UNIT,
    QT_DRIVE_EFFICIENCY, QUANTUM_FUEL_PRICE_PER_UNIT,
};
pub use graph::{Edge, Node, NodeType, RouteGraph};
pub use locations::{
    distance_between, estimate_position, locations_in_system, LocationPosition, LOCATION_POSITIONS,
};
pub use mining::{
    nearest_mining_site, sites_with_resource, MiningSite, ResourceType, MINING_SITES,
};
pub use refinery::{Refinery, RefineryIndex, RefineryMethod, REFINERY_METHODS};
pub use spatial::{
    find_route_intersections, AltJumpInstruction, IndexedHotspot, IntersectingRoute,
    JumpInstruction, NearbyHotspot, Point3D, RouteIntersection, RouteSegment, SpatialIndex,
};
