//! Static contract definitions for Wikelo contracts.
//!
//! Loads contracts from TOML at first access using lazy initialization.
//! Contains all 43 contracts from Phase 9 research (09-RESEARCH.md).

use intel::WikieloContract;
use once_cell::sync::Lazy;
use serde::Deserialize;

const CONTRACTS_TOML: &str = include_str!("../data/contracts.toml");

/// Wrapper struct for TOML root deserialization.
#[derive(Deserialize)]
struct ContractsFile {
    #[serde(default)]
    contracts: Vec<WikieloContract>,
}

static CONTRACTS: Lazy<Vec<WikieloContract>> = Lazy::new(|| {
    // SAFETY: expect is acceptable here - static initialization with compile-time embedded data.
    // If TOML parsing fails, it's a build-time data error that should panic immediately.
    #[allow(clippy::expect_used)]
    let file: ContractsFile =
        toml::from_str(CONTRACTS_TOML).expect("Failed to parse contracts.toml - check TOML syntax");
    file.contracts
});

/// Returns all Wikelo contracts.
#[must_use]
pub fn all_contracts() -> &'static [WikieloContract] {
    &CONTRACTS
}

/// Find a contract by ID.
#[must_use]
pub fn get_contract(id: &str) -> Option<&'static WikieloContract> {
    CONTRACTS.iter().find(|c| c.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use intel::{ContractCategory, DataConfidence};

    #[test]
    fn test_all_contracts_parse() {
        let contracts = all_contracts();
        assert_eq!(contracts.len(), 43, "Expected 43 contracts");
    }

    #[test]
    fn test_all_contracts_have_unique_ids() {
        let contracts = all_contracts();
        let mut ids: Vec<_> = contracts.iter().map(|c| &c.id).collect();
        ids.sort();
        let original_len = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), original_len, "Duplicate contract IDs found");
    }

    #[test]
    fn test_prerequisite_contract_exists() {
        let contract = get_contract("new_to_system");
        assert!(contract.is_some(), "New to System prerequisite missing");
        let contract = contract.unwrap();
        assert_eq!(contract.category, ContractCategory::Prerequisite);
        assert!(
            !contract.repeatable,
            "Prerequisite should not be repeatable"
        );
    }

    #[test]
    fn test_contracts_by_category_count() {
        let contracts = all_contracts();

        let prerequisite_count = contracts
            .iter()
            .filter(|c| c.category == ContractCategory::Prerequisite)
            .count();
        let favor_exchange_count = contracts
            .iter()
            .filter(|c| c.category == ContractCategory::FavorExchange)
            .count();
        let weapon_count = contracts
            .iter()
            .filter(|c| c.category == ContractCategory::Weapon)
            .count();
        let armor_count = contracts
            .iter()
            .filter(|c| c.category == ContractCategory::Armor)
            .count();
        let ship_count = contracts
            .iter()
            .filter(|c| c.category == ContractCategory::Ship)
            .count();
        let equipment_count = contracts
            .iter()
            .filter(|c| c.category == ContractCategory::Equipment)
            .count();

        assert_eq!(prerequisite_count, 1, "Expected 1 Prerequisite");
        assert_eq!(favor_exchange_count, 5, "Expected 5 FavorExchange");
        assert_eq!(weapon_count, 15, "Expected 15 Weapon");
        assert_eq!(armor_count, 9, "Expected 9 Armor");
        assert_eq!(ship_count, 9, "Expected 9 Ship");
        assert_eq!(equipment_count, 4, "Expected 4 Equipment");
    }

    #[test]
    fn test_favor_exchange_contracts_have_confirmed_confidence() {
        let contracts = all_contracts();
        let favor_contracts: Vec<_> = contracts
            .iter()
            .filter(|c| c.category == ContractCategory::FavorExchange)
            .collect();

        // Most FavorExchange contracts should be Confirmed (some are Partial)
        let confirmed_count = favor_contracts
            .iter()
            .filter(|c| c.confidence == DataConfidence::Confirmed)
            .count();

        // At least 3 should be confirmed (turn_things_to_favor, trade_council_scrip, trade_worm_parts)
        assert!(
            confirmed_count >= 3,
            "Expected at least 3 confirmed FavorExchange contracts, got {}",
            confirmed_count
        );
    }

    #[test]
    fn test_polaris_is_limited_time() {
        let contract = get_contract("now_make_polaris");
        assert!(contract.is_some(), "now_make_polaris contract missing");
        let contract = contract.unwrap();
        assert!(
            contract.limited_time,
            "now_make_polaris should be limited_time"
        );
        assert_eq!(contract.category, ContractCategory::Ship);
    }
}
