//! Target intelligence and prediction for interdiction planning.
//!
//! Analyzes trade data to predict hauler routes and identify high-value targets.

mod ships;
mod targets;
mod wikelo;

#[cfg(test)]
mod ships_tests;

pub use ships::{CargoShip, LootEstimate, ShipRegistry, ShipRole};
pub use targets::{
    CommodityValue, HotRoute, InterdictionHotspot, RouteLeg, ShipFrequency, TargetAnalyzer,
    TargetPrediction, TradeActivity, TradeRun, TrafficDirection,
};
pub use wikelo::{
    all_items, AcquisitionMethod, ContractCategory, ContractRequirement, ContractReward,
    CurrencyType, DataConfidence, ExchangeRate, ItemCategory, ItemSource, RewardType, SourceFlag,
    SourceLocation, SystemFlag, WikieloContract, WikieloIntel, WikieloItem, WikieloItemSummary,
    WikieloRegistry,
};
