//! Contract registry - manages Wikelo contracts with bidirectional lookups.

use std::collections::HashMap;

use super::contracts::{ContractCategory, DataConfidence, WikieloContract};
use super::registry::normalize_location;

/// Repository of Wikelo contracts with bidirectional indexes.
#[derive(Clone)]
pub struct ContractRegistry {
    /// Canonical contract storage.
    contracts: Vec<WikieloContract>,
    /// Contract ID to index.
    by_id: HashMap<String, usize>,
    /// Category to contract indexes.
    by_category: HashMap<ContractCategory, Vec<usize>>,
    /// Item ID to contract indexes (bidirectional: which contracts need this item).
    by_item: HashMap<String, Vec<usize>>,
    /// Turn-in location to contract indexes.
    by_location: HashMap<String, Vec<usize>>,
    /// Confidence level to contract indexes.
    by_confidence: HashMap<DataConfidence, Vec<usize>>,
}

impl ContractRegistry {
    /// Create registry with all known Wikelo contracts from static data.
    #[must_use]
    pub fn new() -> Self {
        Self::from_contracts(super::contract_data::all_contracts())
    }

    /// Build registry from a list of contracts.
    #[must_use]
    pub fn from_contracts(contracts: Vec<WikieloContract>) -> Self {
        let mut by_id = HashMap::new();
        let mut by_category: HashMap<ContractCategory, Vec<usize>> = HashMap::new();
        let mut by_item: HashMap<String, Vec<usize>> = HashMap::new();
        let mut by_location: HashMap<String, Vec<usize>> = HashMap::new();
        let mut by_confidence: HashMap<DataConfidence, Vec<usize>> = HashMap::new();

        for (idx, contract) in contracts.iter().enumerate() {
            // Index by ID
            by_id.insert(contract.id.clone(), idx);

            // Index by category
            by_category.entry(contract.category).or_default().push(idx);

            // Index by required items (bidirectional link)
            for req in &contract.requirements {
                by_item.entry(req.item_id.clone()).or_default().push(idx);
            }

            // Index by turn-in location
            for location in &contract.turn_in_locations {
                let location_key = normalize_location(location);
                by_location.entry(location_key).or_default().push(idx);
            }

            // Index by confidence
            by_confidence
                .entry(contract.confidence)
                .or_default()
                .push(idx);
        }

        Self {
            contracts,
            by_id,
            by_category,
            by_item,
            by_location,
            by_confidence,
        }
    }

    // ==================== Contract Lookups ====================

    /// Get a contract by its ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&WikieloContract> {
        let idx = self.by_id.get(id)?;
        self.contracts.get(*idx)
    }

    /// Get all contracts in the registry.
    #[must_use]
    pub fn all_contracts(&self) -> &[WikieloContract] {
        &self.contracts
    }

    // ==================== Category Lookups ====================

    /// Get all contracts in a specific category.
    #[must_use]
    pub fn by_category(&self, category: ContractCategory) -> Vec<&WikieloContract> {
        self.by_category
            .get(&category)
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|idx| self.contracts.get(*idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    // ==================== Item Lookups ====================

    /// Get all contracts that require a specific item.
    ///
    /// This is the key demand lookup: given an item ID, find which contracts
    /// need it. Used by Phase 11 to score interdiction targets.
    #[must_use]
    pub fn contracts_requiring_item(&self, item_id: &str) -> Vec<&WikieloContract> {
        self.by_item
            .get(item_id)
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|idx| self.contracts.get(*idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    // ==================== Location Lookups ====================

    /// Get all contracts available at a specific turn-in location.
    #[must_use]
    pub fn contracts_at_location(&self, location: &str) -> Vec<&WikieloContract> {
        let key = normalize_location(location);
        self.by_location
            .get(&key)
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|idx| self.contracts.get(*idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    // ==================== Confidence Lookups ====================

    /// Get all contracts with a specific confidence level.
    #[must_use]
    pub fn by_confidence(&self, confidence: DataConfidence) -> Vec<&WikieloContract> {
        self.by_confidence
            .get(&confidence)
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|idx| self.contracts.get(*idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    // ==================== Utility Methods ====================

    /// Get the total number of contracts in the registry.
    #[must_use]
    pub fn contract_count(&self) -> usize {
        self.contracts.len()
    }

    /// Get all unique item IDs required across all contracts.
    #[must_use]
    pub fn all_required_items(&self) -> Vec<&str> {
        self.by_item.keys().map(String::as_str).collect()
    }

    // ==================== Confidence Filtering ====================

    /// Get contracts with high confidence (Confirmed or above).
    #[must_use]
    pub fn high_confidence(&self) -> Vec<&WikieloContract> {
        self.contracts
            .iter()
            .filter(|c| c.confidence >= DataConfidence::Confirmed)
            .collect()
    }

    /// Get contracts that need validation (Partial or below).
    #[must_use]
    pub fn needs_validation(&self) -> Vec<&WikieloContract> {
        self.contracts
            .iter()
            .filter(|c| c.confidence <= DataConfidence::Partial)
            .collect()
    }
}

impl Default for ContractRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wikelo::contracts::{ContractRequirement, ContractReward, RewardType};

    /// Create a test contract with the given parameters.
    fn make_test_contract(
        id: &str,
        name: &str,
        category: ContractCategory,
        item_ids: Vec<&str>,
        locations: Vec<&str>,
    ) -> WikieloContract {
        WikieloContract {
            id: id.to_string(),
            name: name.to_string(),
            category,
            requirements: item_ids
                .into_iter()
                .map(|item_id| ContractRequirement::new(item_id, 1))
                .collect(),
            rewards: vec![ContractReward::new("Test Reward", RewardType::Other)],
            repeatable: false,
            description: None,
            prerequisites: vec![],
            turn_in_locations: locations.into_iter().map(String::from).collect(),
            confidence: DataConfidence::Confirmed,
            available: true,
            limited_time: false,
            exchange_rate: None,
        }
    }

    #[test]
    fn test_registry_creation() {
        let contracts = vec![
            make_test_contract(
                "contract_1",
                "Contract 1",
                ContractCategory::Weapon,
                vec!["item_a"],
                vec!["Station Alpha"],
            ),
            make_test_contract(
                "contract_2",
                "Contract 2",
                ContractCategory::Armor,
                vec!["item_b"],
                vec!["Station Beta"],
            ),
        ];

        let registry = ContractRegistry::from_contracts(contracts);

        assert_eq!(registry.contract_count(), 2);
        assert_eq!(registry.all_contracts().len(), 2);
    }

    #[test]
    fn test_get_by_id() {
        let contracts = vec![
            make_test_contract(
                "weapon_contract",
                "Weapon Contract",
                ContractCategory::Weapon,
                vec!["ore_a"],
                vec!["Station A"],
            ),
            make_test_contract(
                "armor_contract",
                "Armor Contract",
                ContractCategory::Armor,
                vec!["ore_b"],
                vec!["Station B"],
            ),
        ];

        let registry = ContractRegistry::from_contracts(contracts);

        // Found cases
        let contract = registry.get("weapon_contract");
        assert!(contract.is_some());
        assert_eq!(contract.unwrap().name, "Weapon Contract");

        let contract = registry.get("armor_contract");
        assert!(contract.is_some());
        assert_eq!(contract.unwrap().name, "Armor Contract");

        // Not found case
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_by_category() {
        let contracts = vec![
            make_test_contract(
                "c1",
                "Weapon 1",
                ContractCategory::Weapon,
                vec!["item_a"],
                vec!["Loc"],
            ),
            make_test_contract(
                "c2",
                "Weapon 2",
                ContractCategory::Weapon,
                vec!["item_b"],
                vec!["Loc"],
            ),
            make_test_contract(
                "c3",
                "Armor 1",
                ContractCategory::Armor,
                vec!["item_c"],
                vec!["Loc"],
            ),
        ];

        let registry = ContractRegistry::from_contracts(contracts);

        let weapons = registry.by_category(ContractCategory::Weapon);
        assert_eq!(weapons.len(), 2);

        let armor = registry.by_category(ContractCategory::Armor);
        assert_eq!(armor.len(), 1);
        assert_eq!(armor[0].name, "Armor 1");

        let ships = registry.by_category(ContractCategory::Ship);
        assert!(ships.is_empty());
    }

    #[test]
    fn test_contracts_requiring_item() {
        let contracts = vec![
            make_test_contract(
                "c1",
                "Contract 1",
                ContractCategory::Weapon,
                vec!["shared_item", "unique_a"],
                vec!["Loc"],
            ),
            make_test_contract(
                "c2",
                "Contract 2",
                ContractCategory::Armor,
                vec!["shared_item", "unique_b"],
                vec!["Loc"],
            ),
            make_test_contract(
                "c3",
                "Contract 3",
                ContractCategory::Ship,
                vec!["unique_c"],
                vec!["Loc"],
            ),
        ];

        let registry = ContractRegistry::from_contracts(contracts);

        // shared_item appears in two contracts
        let shared = registry.contracts_requiring_item("shared_item");
        assert_eq!(shared.len(), 2);

        // unique items appear in one contract each
        let unique_a = registry.contracts_requiring_item("unique_a");
        assert_eq!(unique_a.len(), 1);
        assert_eq!(unique_a[0].id, "c1");

        let unique_c = registry.contracts_requiring_item("unique_c");
        assert_eq!(unique_c.len(), 1);
        assert_eq!(unique_c[0].id, "c3");

        // Unknown item returns empty
        assert!(registry.contracts_requiring_item("nonexistent").is_empty());
    }

    #[test]
    fn test_contracts_at_location() {
        let contracts = vec![
            make_test_contract(
                "c1",
                "Contract 1",
                ContractCategory::Weapon,
                vec!["item_a"],
                vec!["Wikelo Emporium Dasi", "Wikelo Emporium Kinga"],
            ),
            make_test_contract(
                "c2",
                "Contract 2",
                ContractCategory::Armor,
                vec!["item_b"],
                vec!["Wikelo Emporium Dasi"],
            ),
        ];

        let registry = ContractRegistry::from_contracts(contracts);

        // Both contracts at Dasi
        let at_dasi = registry.contracts_at_location("Wikelo Emporium Dasi");
        assert_eq!(at_dasi.len(), 2);

        // Case-insensitive matching
        let at_dasi_lower = registry.contracts_at_location("wikelo emporium dasi");
        assert_eq!(at_dasi_lower.len(), 2);

        // Only one contract at Kinga
        let at_kinga = registry.contracts_at_location("Wikelo Emporium Kinga");
        assert_eq!(at_kinga.len(), 1);
        assert_eq!(at_kinga[0].id, "c1");

        // No contracts at unknown location
        assert!(registry.contracts_at_location("Unknown Station").is_empty());
    }

    #[test]
    fn test_by_confidence() {
        let mut c1 = make_test_contract(
            "c1",
            "Confirmed",
            ContractCategory::Weapon,
            vec!["item_a"],
            vec!["Loc"],
        );
        c1.confidence = DataConfidence::Confirmed;

        let mut c2 = make_test_contract(
            "c2",
            "Partial",
            ContractCategory::Armor,
            vec!["item_b"],
            vec!["Loc"],
        );
        c2.confidence = DataConfidence::Partial;

        let mut c3 = make_test_contract(
            "c3",
            "Also Partial",
            ContractCategory::Ship,
            vec!["item_c"],
            vec!["Loc"],
        );
        c3.confidence = DataConfidence::Partial;

        let registry = ContractRegistry::from_contracts(vec![c1, c2, c3]);

        let confirmed = registry.by_confidence(DataConfidence::Confirmed);
        assert_eq!(confirmed.len(), 1);
        assert_eq!(confirmed[0].id, "c1");

        let partial = registry.by_confidence(DataConfidence::Partial);
        assert_eq!(partial.len(), 2);

        let inferred = registry.by_confidence(DataConfidence::Inferred);
        assert!(inferred.is_empty());
    }

    #[test]
    fn test_empty_registry() {
        let registry = ContractRegistry::from_contracts(vec![]);

        assert_eq!(registry.contract_count(), 0);
        assert!(registry.all_contracts().is_empty());
        assert!(registry.get("any_id").is_none());
        assert!(registry.by_category(ContractCategory::Weapon).is_empty());
        assert!(registry.contracts_requiring_item("any_item").is_empty());
        assert!(registry.contracts_at_location("any_location").is_empty());
        assert!(registry.by_confidence(DataConfidence::Confirmed).is_empty());
        assert!(registry.all_required_items().is_empty());
        assert!(registry.high_confidence().is_empty());
        assert!(registry.needs_validation().is_empty());
    }

    #[test]
    fn test_new_loads_static_data() {
        let registry = ContractRegistry::new();

        // new() should load all 14 contracts from static data
        assert_eq!(registry.contract_count(), 14);

        // Should be able to look up known contracts
        assert!(registry.get("new_to_system").is_some());
        assert!(registry.get("turn_things_to_favor").is_some());
        assert!(registry.get("yormandi_gun").is_some());
    }

    #[test]
    fn test_all_required_items() {
        let contracts = vec![
            make_test_contract(
                "c1",
                "Contract 1",
                ContractCategory::Weapon,
                vec!["item_a", "item_b"],
                vec!["Loc"],
            ),
            make_test_contract(
                "c2",
                "Contract 2",
                ContractCategory::Armor,
                vec!["item_b", "item_c"],
                vec!["Loc"],
            ),
        ];

        let registry = ContractRegistry::from_contracts(contracts);
        let mut items = registry.all_required_items();
        items.sort();

        // Should return unique item IDs
        assert_eq!(items.len(), 3);
        assert_eq!(items, vec!["item_a", "item_b", "item_c"]);
    }

    #[test]
    fn test_high_confidence_and_needs_validation() {
        let registry = ContractRegistry::new();

        // From static data: 2 Confirmed contracts (new_to_system, turn_things_to_favor)
        let high = registry.high_confidence();
        assert_eq!(
            high.len(),
            2,
            "Expected 2 high confidence contracts (Confirmed+), got {}",
            high.len()
        );

        // From static data: 12 contracts with Partial or Inferred confidence
        let needs_val = registry.needs_validation();
        assert_eq!(
            needs_val.len(),
            12,
            "Expected 12 contracts needing validation (Partial or below), got {}",
            needs_val.len()
        );

        // Verify wikelo_favor is required by 8 contracts
        let favor_contracts = registry.contracts_requiring_item("wikelo_favor");
        assert_eq!(
            favor_contracts.len(),
            8,
            "Expected 8 contracts requiring wikelo_favor, got {}",
            favor_contracts.len()
        );
    }
}
