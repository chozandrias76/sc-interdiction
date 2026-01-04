//! Cargo ship data for target estimation.
//!
//! Only ships with `production_status: "flight-ready"` from the FleetYards API
//! are included in the registry. This ensures that only ships available in the
//! PTU (Public Test Universe) are used for route estimation and target analysis.

mod enrichment;
mod registry;
mod types;

pub use registry::ShipRegistry;
pub use types::{CargoShip, LootEstimate, ShipRole};

// Legacy compatibility - these are now ShipRegistry methods
// Use registry.estimate_for_route() instead
