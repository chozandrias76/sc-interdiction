//! Wikelo registry - manages item data with bidirectional lookups.

use std::collections::HashMap;

// Import types from intel crate - do NOT redefine
use intel::{ItemCategory, WikieloItem};

/// Repository of Wikelo items with bidirectional indexes.
#[derive(Clone, Default)]
pub struct WikieloRegistry {
    /// Canonical item storage.
    items: Vec<WikieloItem>,
    /// Item ID to index.
    by_id: HashMap<String, usize>,
    /// Location name to item indexes.
    by_location: HashMap<String, Vec<usize>>,
    /// System to item indexes.
    by_system: HashMap<String, Vec<usize>>,
    /// Category to item indexes.
    by_category: HashMap<ItemCategory, Vec<usize>>,
}

impl WikieloRegistry {
    /// Create a new empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Build registry from a list of items.
    #[must_use]
    pub fn from_items(items: Vec<WikieloItem>) -> Self {
        let mut by_id = HashMap::new();
        let mut by_location: HashMap<String, Vec<usize>> = HashMap::new();
        let mut by_system: HashMap<String, Vec<usize>> = HashMap::new();
        let mut by_category: HashMap<ItemCategory, Vec<usize>> = HashMap::new();

        for (idx, item) in items.iter().enumerate() {
            // Index by ID
            by_id.insert(item.id.clone(), idx);

            // Index by category
            by_category.entry(item.category).or_default().push(idx);

            // Index by location and system from sources
            for source in &item.sources {
                let location_key = normalize_location(&source.location.name);
                by_location.entry(location_key).or_default().push(idx);

                let system_key = normalize_location(&source.location.system);
                by_system.entry(system_key).or_default().push(idx);
            }
        }

        Self {
            items,
            by_id,
            by_location,
            by_system,
            by_category,
        }
    }

    // ==================== Item Lookups ====================

    /// Get an item by its ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&WikieloItem> {
        let idx = self.by_id.get(id)?;
        self.items.get(*idx)
    }

    /// Get all items in the registry.
    #[must_use]
    pub fn all_items(&self) -> &[WikieloItem] {
        &self.items
    }

    // ==================== Location Lookups ====================

    /// Get all items available at a specific location.
    #[must_use]
    pub fn items_at_location(&self, location: &str) -> Vec<&WikieloItem> {
        let key = normalize_location(location);
        self.by_location
            .get(&key)
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|idx| self.items.get(*idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all items available in a specific system.
    #[must_use]
    pub fn items_in_system(&self, system: &str) -> Vec<&WikieloItem> {
        let key = normalize_location(system);
        self.by_system
            .get(&key)
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|idx| self.items.get(*idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    // ==================== Category Lookups ====================

    /// Get all items in a specific category.
    #[must_use]
    pub fn items_by_category(&self, category: ItemCategory) -> Vec<&WikieloItem> {
        self.by_category
            .get(&category)
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|idx| self.items.get(*idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    // ==================== Utility Methods ====================

    /// List all unique location names in the registry.
    #[must_use]
    pub fn all_locations(&self) -> Vec<&str> {
        self.by_location.keys().map(String::as_str).collect()
    }

    /// List all unique systems in the registry.
    #[must_use]
    pub fn all_systems(&self) -> Vec<&str> {
        self.by_system.keys().map(String::as_str).collect()
    }

    /// Get the total number of items in the registry.
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

/// Normalize location/system name for matching.
///
/// Examples:
/// - "Lazarus Transport Centers" -> "lazarus transport centers"
/// - "ARC-L1" -> "arc l1"
/// - "Pyro  I" -> "pyro i"
fn normalize_location(name: &str) -> String {
    name.to_lowercase()
        .replace(['-', '_'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
