//! Target intelligence and prediction for interdiction planning.
//!
//! Analyzes trade data to predict hauler routes and identify high-value targets.

mod ships;
mod targets;

#[cfg(test)]
mod ships_tests;

pub use ships::{CargoShip, LootEstimate, ShipRole, CARGO_SHIPS};
pub use targets::{
    CommodityValue, HotRoute, InterdictionHotspot, RouteLeg, ShipFrequency, TargetAnalyzer,
    TargetPrediction, TradeActivity, TradeRun, TrafficDirection,
};
