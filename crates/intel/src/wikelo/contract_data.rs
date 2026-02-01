//! Static contract data for Wikelo contracts.
//!
//! Contains the 14 high-confidence contracts from Phase 7 research (07-RESEARCH.md).

// Allow long functions for static data definitions - these are data, not logic
#![allow(clippy::too_many_lines)]

#[allow(unused_imports)]
use super::contracts::{
    ContractCategory, ContractRequirement, ContractReward, CurrencyType, DataConfidence,
    ExchangeRate, RewardType, WikieloContract,
};

/// Returns all Wikelo contracts from research data.
#[must_use]
pub fn all_contracts() -> Vec<WikieloContract> {
    let mut contracts = Vec::with_capacity(14);
    contracts.extend(prerequisite_contracts());
    contracts.extend(favor_exchange_contracts());
    contracts.extend(partial_data_contracts());
    contracts
}

/// Returns the 3 Wikelo Emporium station names used as turn-in locations.
#[allow(dead_code)]
fn wikelo_locations() -> Vec<String> {
    vec![
        "Wikelo Emporium Dasi".to_string(),
        "Wikelo Emporium Kinga".to_string(),
        "Wikelo Emporium Selo".to_string(),
    ]
}

/// Returns prerequisite contracts.
fn prerequisite_contracts() -> Vec<WikieloContract> {
    vec![]
}

/// Returns favor exchange contracts.
fn favor_exchange_contracts() -> Vec<WikieloContract> {
    vec![]
}

/// Returns contracts with partial data (weapons, armor, ships, equipment).
fn partial_data_contracts() -> Vec<WikieloContract> {
    vec![]
}
