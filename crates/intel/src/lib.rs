//! Target intelligence and prediction for interdiction planning.
//!
//! Analyzes trade data to predict hauler routes and identify high-value targets.

mod targets;
mod ships;

pub use targets::{
    HotRoute, TargetAnalyzer, TargetPrediction, TradeActivity, TrafficDirection,
};
pub use ships::{CargoShip, CARGO_SHIPS};
