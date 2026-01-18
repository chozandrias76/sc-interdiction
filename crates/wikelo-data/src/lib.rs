//! Wikelo item data registry and lookups.
//!
//! Provides a registry for Wikelo items with bidirectional lookups
//! (item→sources and location→items) for integration with interdiction planning.

mod registry;

pub use registry::WikieloRegistry;
