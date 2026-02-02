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
