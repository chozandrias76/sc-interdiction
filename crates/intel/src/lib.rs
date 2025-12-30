//! Target intelligence and prediction for interdiction planning.
//!
//! Analyzes trade data to predict hauler routes and identify high-value targets.

mod ships;
mod targets;

pub use ships::{CargoShip, CARGO_SHIPS};
pub use targets::{
    CommodityValue, HotRoute, InterdictionHotspot, RouteLeg, ShipFrequency, TargetAnalyzer,
    TargetPrediction, TradeActivity, TradeRun, TrafficDirection,
};
