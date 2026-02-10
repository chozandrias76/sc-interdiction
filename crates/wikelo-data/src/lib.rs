//! Wikelo item data registry and lookups.
//!
//! Provides a registry for Wikelo items with bidirectional lookups
//! (item→sources and location→items) for integration with interdiction planning.

pub mod contracts;
pub mod items;
mod registry;

pub use contracts::{all_contracts, get_contract};
pub use items::all_items;
pub use registry::WikieloRegistry;
