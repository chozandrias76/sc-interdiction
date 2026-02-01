//! Static contract data for Wikelo contracts.
//!
//! Contains the 14 high-confidence contracts from Phase 7 research (07-RESEARCH.md).

// Allow long functions for static data definitions - these are data, not logic
#![allow(clippy::too_many_lines)]

use super::contracts::{
    ContractCategory, ContractRequirement, ContractReward, CurrencyType, DataConfidence,
    ExchangeRate, RewardType, WikieloContract,
};

/// Returns all 14 high-confidence Wikelo contracts from research data.
#[must_use]
pub fn all_contracts() -> Vec<WikieloContract> {
    let mut contracts = Vec::with_capacity(14);
    contracts.extend(prerequisite_contracts());
    contracts.extend(favor_exchange_contracts());
    contracts.extend(partial_data_contracts());
    contracts
}

/// Returns the 3 Wikelo Emporium station names used as turn-in locations.
fn wikelo_locations() -> Vec<String> {
    vec![
        "Wikelo Emporium Dasi".to_string(),
        "Wikelo Emporium Kinga".to_string(),
        "Wikelo Emporium Selo".to_string(),
    ]
}

/// Returns the 1 prerequisite contract.
fn prerequisite_contracts() -> Vec<WikieloContract> {
    vec![WikieloContract {
        id: "new_to_system".to_string(),
        name: "New to System".to_string(),
        category: ContractCategory::Prerequisite,
        requirements: vec![
            ContractRequirement::new("vestal_water", 1),
            ContractRequirement::new("tundra_kopion_horn", 3),
        ],
        rewards: vec![ContractReward::new(
            "Access to all Wikelo contracts",
            RewardType::Access,
        )],
        repeatable: false,
        description: Some(
            "Gateway contract. Must be completed before accessing other Wikelo contracts."
                .to_string(),
        ),
        prerequisites: vec![],
        turn_in_locations: wikelo_locations(),
        confidence: DataConfidence::Confirmed,
        available: true,
        limited_time: false,
        exchange_rate: None,
    }]
}

/// Returns the 5 favor exchange contracts.
fn favor_exchange_contracts() -> Vec<WikieloContract> {
    vec![
        WikieloContract {
            id: "turn_things_to_favor".to_string(),
            name: "Turn Things to Favor".to_string(),
            category: ContractCategory::FavorExchange,
            requirements: vec![ContractRequirement::new("mg_scrip", 50)],
            rewards: vec![ContractReward::new("Wikelo Favor", RewardType::Favor)],
            repeatable: true,
            description: Some("Exchange MG Scrip for Wikelo Favors.".to_string()),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Confirmed,
            available: true,
            limited_time: false,
            exchange_rate: Some(ExchangeRate {
                input_currency: CurrencyType::MgScrip,
                input_quantity: 50,
                output_currency: CurrencyType::WikieloFavor,
                output_quantity: 1,
            }),
        },
        WikieloContract {
            id: "trade_council_scrip_for_favors".to_string(),
            name: "Trade Council Scrip for Favors?".to_string(),
            category: ContractCategory::FavorExchange,
            requirements: vec![ContractRequirement::new("council_scrip", 50)],
            rewards: vec![ContractReward::new("Wikelo Favor", RewardType::Favor)],
            repeatable: true,
            description: Some("Exchange Council Scrip for Wikelo Favors.".to_string()),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Partial,
            available: true,
            limited_time: false,
            exchange_rate: Some(ExchangeRate {
                input_currency: CurrencyType::CouncilScrip,
                input_quantity: 50,
                output_currency: CurrencyType::WikieloFavor,
                output_quantity: 1,
            }),
        },
        WikieloContract {
            id: "need_mining_things".to_string(),
            name: "Need mining things. Clever things to trade.".to_string(),
            category: ContractCategory::FavorExchange,
            requirements: vec![ContractRequirement::new("carinite", 50)],
            rewards: vec![ContractReward::new("Wikelo Favor", RewardType::Favor)],
            repeatable: true,
            description: Some("Exchange Carinite for Wikelo Favors.".to_string()),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Partial,
            available: true,
            limited_time: false,
            exchange_rate: Some(ExchangeRate {
                input_currency: CurrencyType::Carinite,
                input_quantity: 50,
                output_currency: CurrencyType::WikieloFavor,
                output_quantity: 1,
            }),
        },
        WikieloContract {
            id: "trade_worm_parts_for_favors".to_string(),
            name: "Trade Worm Parts for Favors?".to_string(),
            category: ContractCategory::FavorExchange,
            requirements: vec![ContractRequirement::new("irradiated_valakkar_pearl", 15)],
            rewards: vec![ContractReward::new("Wikelo Favor", RewardType::Favor)],
            repeatable: true,
            description: Some("Exchange Irradiated Valakkar Pearls for Wikelo Favors.".to_string()),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Partial,
            available: true,
            limited_time: false,
            exchange_rate: Some(ExchangeRate {
                input_currency: CurrencyType::Other("Irradiated Valakkar Pearl".to_string()),
                input_quantity: 15,
                output_currency: CurrencyType::WikieloFavor,
                output_quantity: 1,
            }),
        },
        WikieloContract {
            id: "very_hungry".to_string(),
            name: "Very Hungry".to_string(),
            category: ContractCategory::FavorExchange,
            requirements: vec![],
            rewards: vec![ContractReward::new("Wikelo Favor", RewardType::Favor)],
            repeatable: true,
            description: Some(
                "Exchange food/consumable items for Wikelo Favors. Exact requirements TBD."
                    .to_string(),
            ),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Inferred,
            available: true,
            limited_time: false,
            exchange_rate: None,
        },
    ]
}

/// Returns the 8 contracts with partial data (weapons, armor, ships, equipment).
fn partial_data_contracts() -> Vec<WikieloContract> {
    vec![
        // Weapon contracts
        WikieloContract {
            id: "yormandi_gun".to_string(),
            name: "Yormandi Gun".to_string(),
            category: ContractCategory::Weapon,
            requirements: vec![
                ContractRequirement::new("yormandi_eye", 1),
                ContractRequirement::new("yormandi_tongue", 1),
                ContractRequirement::new("wikelo_favor", 1),
            ],
            rewards: vec![ContractReward::new(
                "Yormandi-themed weapon",
                RewardType::Weapon,
            )],
            repeatable: false,
            description: Some(
                "Trade Yormandi parts and Favors for a Yormandi-themed weapon.".to_string(),
            ),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Partial,
            available: true,
            limited_time: false,
            exchange_rate: None,
        },
        WikieloContract {
            id: "need_ore_will_give_guns".to_string(),
            name: "Need Ore. Will give Guns.".to_string(),
            category: ContractCategory::Weapon,
            requirements: vec![
                ContractRequirement::new("copper", 1),
                ContractRequirement::new("tungsten", 1),
                ContractRequirement::new("wikelo_favor", 1),
            ],
            rewards: vec![ContractReward::new("Weapon", RewardType::Weapon)],
            repeatable: false,
            description: Some("Trade mined ores and Favors for a weapon.".to_string()),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Partial,
            available: true,
            limited_time: false,
            exchange_rate: None,
        },
        // Armor contract
        WikieloContract {
            id: "walk_in_danger_look_good".to_string(),
            name: "Walk in danger. Look good".to_string(),
            category: ContractCategory::Armor,
            requirements: vec![
                ContractRequirement::new("irradiated_valakkar_fang_juvenile", 1),
                ContractRequirement::new("irradiated_valakkar_fang_adult", 1),
                ContractRequirement::new("wikelo_favor", 1),
            ],
            rewards: vec![ContractReward::new(
                "Danger-themed armor",
                RewardType::Armor,
            )],
            repeatable: false,
            description: Some(
                "Trade Irradiated Valakkar parts and Favors for themed armor.".to_string(),
            ),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Partial,
            available: true,
            limited_time: false,
            exchange_rate: None,
        },
        // Ship contracts
        WikieloContract {
            id: "now_make_polaris_limited".to_string(),
            name: "Now make Polaris. Short Time Deal.".to_string(),
            category: ContractCategory::Ship,
            requirements: vec![
                ContractRequirement::new("polaris_bit", 1),
                ContractRequirement::new("carinite", 1),
                ContractRequirement::new("dchs_05_comp_board", 1),
                ContractRequirement::new("wikelo_favor", 1),
            ],
            rewards: vec![ContractReward::new("RSI Polaris", RewardType::Ship)],
            repeatable: false,
            description: Some(
                "Limited-time Polaris ship contract. Requires Polaris Bits, Carinite, and DCHS-05 boards."
                    .to_string(),
            ),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Partial,
            available: true,
            limited_time: true,
            exchange_rate: None,
        },
        WikieloContract {
            id: "want_polaris_need_special".to_string(),
            name: "Want Polaris? Need something special.".to_string(),
            category: ContractCategory::Ship,
            requirements: vec![
                ContractRequirement::new("polaris_bit", 1),
                ContractRequirement::new("wikelo_favor", 1),
            ],
            rewards: vec![ContractReward::new("RSI Polaris", RewardType::Ship)],
            repeatable: false,
            description: Some(
                "Standard Polaris contract. Requires Polaris Bits and rare materials.".to_string(),
            ),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Partial,
            available: true,
            limited_time: false,
            exchange_rate: None,
        },
        // Vehicle contracts
        WikieloContract {
            id: "make_atls_shoot".to_string(),
            name: "Make ATLS shoot".to_string(),
            category: ContractCategory::Equipment,
            requirements: vec![ContractRequirement::new("wikelo_favor", 1)],
            rewards: vec![ContractReward::new("Armed ATLS variant", RewardType::Vehicle)],
            repeatable: false,
            description: Some(
                "Trade Favors and ATLS components for an armed ATLS variant.".to_string(),
            ),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Partial,
            available: true,
            limited_time: false,
            exchange_rate: None,
        },
        WikieloContract {
            id: "make_jumpy_atls_shoot".to_string(),
            name: "Make jumpy ATLS shoot".to_string(),
            category: ContractCategory::Equipment,
            requirements: vec![ContractRequirement::new("wikelo_favor", 1)],
            rewards: vec![ContractReward::new(
                "Armed jumping ATLS variant",
                RewardType::Vehicle,
            )],
            repeatable: false,
            description: Some(
                "Trade Favors and ATLS components for an armed jumping ATLS variant.".to_string(),
            ),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Partial,
            available: true,
            limited_time: false,
            exchange_rate: None,
        },
        // Equipment contract
        WikieloContract {
            id: "want_better_eyes".to_string(),
            name: "Want Better Eyes".to_string(),
            category: ContractCategory::Equipment,
            requirements: vec![
                ContractRequirement::new("yormandi_eye", 1),
                ContractRequirement::new("wikelo_favor", 1),
            ],
            rewards: vec![ContractReward::new(
                "Enhanced optics/visor",
                RewardType::Other,
            )],
            repeatable: false,
            description: Some("Trade Yormandi Eyes and Favors for enhanced optics.".to_string()),
            prerequisites: vec!["new_to_system".to_string()],
            turn_in_locations: wikelo_locations(),
            confidence: DataConfidence::Partial,
            available: true,
            limited_time: false,
            exchange_rate: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_contracts_returns_14() {
        let contracts = all_contracts();
        assert_eq!(
            contracts.len(),
            14,
            "Expected 14 contracts, got {}",
            contracts.len()
        );
    }

    #[test]
    fn test_all_contracts_have_unique_ids() {
        let contracts = all_contracts();
        let mut ids: Vec<&str> = contracts.iter().map(|c| c.id.as_str()).collect();
        ids.sort();
        let original_len = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), original_len, "Found duplicate contract IDs");
    }

    #[test]
    fn test_prerequisite_count() {
        let contracts = all_contracts();
        let prereqs: Vec<_> = contracts
            .iter()
            .filter(|c| c.category == ContractCategory::Prerequisite)
            .collect();
        assert_eq!(prereqs.len(), 1, "Expected 1 prerequisite contract");
    }

    #[test]
    fn test_favor_exchange_count() {
        let contracts = all_contracts();
        let exchanges: Vec<_> = contracts
            .iter()
            .filter(|c| c.category == ContractCategory::FavorExchange)
            .collect();
        assert_eq!(exchanges.len(), 5, "Expected 5 favor exchange contracts");
    }

    #[test]
    fn test_new_to_system_has_no_prerequisites() {
        let contracts = all_contracts();
        let nts = contracts
            .iter()
            .find(|c| c.id == "new_to_system")
            .expect("new_to_system not found");
        assert!(
            nts.prerequisites.is_empty(),
            "New to System should have no prerequisites"
        );
    }

    #[test]
    fn test_non_prerequisite_contracts_require_new_to_system() {
        let contracts = all_contracts();
        for contract in &contracts {
            if contract.category != ContractCategory::Prerequisite {
                assert!(
                    contract
                        .prerequisites
                        .contains(&"new_to_system".to_string()),
                    "Contract '{}' should require new_to_system",
                    contract.id
                );
            }
        }
    }

    #[test]
    fn test_all_contracts_have_turn_in_locations() {
        let contracts = all_contracts();
        for contract in &contracts {
            assert_eq!(
                contract.turn_in_locations.len(),
                3,
                "Contract '{}' should have 3 turn-in locations",
                contract.id
            );
            assert!(contract
                .turn_in_locations
                .iter()
                .any(|l| l.contains("Dasi")));
            assert!(contract
                .turn_in_locations
                .iter()
                .any(|l| l.contains("Kinga")));
            assert!(contract
                .turn_in_locations
                .iter()
                .any(|l| l.contains("Selo")));
        }
    }

    #[test]
    fn test_favor_exchange_rates() {
        let contracts = all_contracts();
        let exchanges: Vec<_> = contracts
            .iter()
            .filter(|c| c.category == ContractCategory::FavorExchange && c.exchange_rate.is_some())
            .collect();
        assert!(
            exchanges.len() >= 4,
            "Expected at least 4 contracts with exchange rates"
        );

        // Verify MG Scrip rate: 50 -> 1
        let mg_scrip = contracts
            .iter()
            .find(|c| c.id == "turn_things_to_favor")
            .expect("turn_things_to_favor not found");
        let rate = mg_scrip
            .exchange_rate
            .as_ref()
            .expect("should have exchange rate");
        assert_eq!(rate.input_quantity, 50);
        assert_eq!(rate.output_quantity, 1);
    }

    #[test]
    fn test_polaris_limited_time_flag() {
        let contracts = all_contracts();
        let polaris_limited = contracts
            .iter()
            .find(|c| c.id == "now_make_polaris_limited")
            .expect("now_make_polaris_limited not found");
        assert!(
            polaris_limited.limited_time,
            "Polaris limited deal should be limited_time"
        );

        let polaris_standard = contracts
            .iter()
            .find(|c| c.id == "want_polaris_need_special")
            .expect("want_polaris_need_special not found");
        assert!(
            !polaris_standard.limited_time,
            "Standard Polaris should not be limited_time"
        );
    }

    #[test]
    fn test_all_contracts_available() {
        let contracts = all_contracts();
        for contract in &contracts {
            assert!(
                contract.available,
                "Contract '{}' should be available",
                contract.id
            );
        }
    }

    #[test]
    fn test_new_to_system_requirements() {
        let contracts = all_contracts();
        let nts = contracts
            .iter()
            .find(|c| c.id == "new_to_system")
            .expect("new_to_system not found");
        assert!(nts.requires_item("vestal_water"));
        assert!(nts.requires_item("tundra_kopion_horn"));
        assert_eq!(nts.quantity_required("tundra_kopion_horn"), 3);
    }

    #[test]
    fn test_wikelo_locations_helper() {
        let locations = wikelo_locations();
        assert_eq!(locations.len(), 3);
        assert!(locations.contains(&"Wikelo Emporium Dasi".to_string()));
        assert!(locations.contains(&"Wikelo Emporium Kinga".to_string()));
        assert!(locations.contains(&"Wikelo Emporium Selo".to_string()));
    }

    #[test]
    fn test_confidence_levels() {
        let contracts = all_contracts();
        // New to System and Turn Things to Favor should be Confirmed
        let nts = contracts
            .iter()
            .find(|c| c.id == "new_to_system")
            .expect("new_to_system");
        assert_eq!(nts.confidence, DataConfidence::Confirmed);

        // Very Hungry should be Inferred (lowest confidence)
        let hungry = contracts
            .iter()
            .find(|c| c.id == "very_hungry")
            .expect("very_hungry");
        assert_eq!(hungry.confidence, DataConfidence::Inferred);

        // Partial data contracts should be Partial
        let yormandi = contracts
            .iter()
            .find(|c| c.id == "yormandi_gun")
            .expect("yormandi_gun");
        assert_eq!(yormandi.confidence, DataConfidence::Partial);
    }
}
