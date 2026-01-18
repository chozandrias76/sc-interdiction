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

#[cfg(test)]
mod tests {
    use super::*;
    use intel::{AcquisitionMethod, ItemSource, SourceLocation};

    /// Create a test item with the given parameters.
    fn make_test_item(
        id: &str,
        name: &str,
        category: ItemCategory,
        sources: Vec<(&str, &str)>,
    ) -> WikieloItem {
        WikieloItem {
            id: id.to_string(),
            name: name.to_string(),
            category,
            sources: sources
                .into_iter()
                .map(|(loc, sys)| ItemSource {
                    location: SourceLocation {
                        name: loc.to_string(),
                        system: sys.to_string(),
                        description: None,
                    },
                    method: AcquisitionMethod::Hunting,
                    reliability: 3,
                    notes: None,
                })
                .collect(),
            estimated_value: Some(1000),
            stackable: true,
            scu_per_unit: None,
        }
    }

    #[test]
    fn test_registry_creation() {
        let items = vec![
            make_test_item(
                "item_1",
                "Item 1",
                ItemCategory::CreaturePart,
                vec![("Pyro I", "Pyro")],
            ),
            make_test_item(
                "item_2",
                "Item 2",
                ItemCategory::MinedMaterial,
                vec![("ARC-L1", "Stanton")],
            ),
        ];

        let registry = WikieloRegistry::from_items(items);

        assert_eq!(registry.item_count(), 2);
        assert_eq!(registry.all_items().len(), 2);
    }

    #[test]
    fn test_get_by_id() {
        let items = vec![
            make_test_item(
                "valakkar_fang",
                "Valakkar Fang",
                ItemCategory::CreaturePart,
                vec![("Pyro I", "Pyro")],
            ),
            make_test_item(
                "carinite_ore",
                "Carinite Ore",
                ItemCategory::MinedMaterial,
                vec![("ARC-L1", "Stanton")],
            ),
        ];

        let registry = WikieloRegistry::from_items(items);

        // Found cases
        let item = registry.get("valakkar_fang");
        assert!(item.is_some());
        assert_eq!(item.unwrap().name, "Valakkar Fang");

        let item = registry.get("carinite_ore");
        assert!(item.is_some());
        assert_eq!(item.unwrap().name, "Carinite Ore");

        // Not found case
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_items_at_location() {
        let items = vec![
            make_test_item(
                "item_1",
                "Item 1",
                ItemCategory::CreaturePart,
                vec![("Lazarus Transport Centers", "Pyro")],
            ),
            make_test_item(
                "item_2",
                "Item 2",
                ItemCategory::MinedMaterial,
                vec![("Lazarus Transport Centers", "Pyro")],
            ),
            make_test_item(
                "item_3",
                "Item 3",
                ItemCategory::CombatLoot,
                vec![("ARC-L1", "Stanton")],
            ),
        ];

        let registry = WikieloRegistry::from_items(items);

        // Should find items at Lazarus (normalized matching)
        let at_lazarus = registry.items_at_location("Lazarus Transport Centers");
        assert_eq!(at_lazarus.len(), 2);

        // Case-insensitive matching
        let at_lazarus_lower = registry.items_at_location("lazarus transport centers");
        assert_eq!(at_lazarus_lower.len(), 2);

        // Items at ARC-L1
        let at_arc = registry.items_at_location("ARC-L1");
        assert_eq!(at_arc.len(), 1);
        assert_eq!(at_arc[0].name, "Item 3");

        // No items at unknown location
        let at_unknown = registry.items_at_location("Unknown Station");
        assert!(at_unknown.is_empty());
    }

    #[test]
    fn test_items_in_system() {
        let items = vec![
            make_test_item(
                "item_1",
                "Item 1",
                ItemCategory::CreaturePart,
                vec![("Location A", "Pyro")],
            ),
            make_test_item(
                "item_2",
                "Item 2",
                ItemCategory::MinedMaterial,
                vec![("Location B", "Pyro")],
            ),
            make_test_item(
                "item_3",
                "Item 3",
                ItemCategory::CombatLoot,
                vec![("ARC-L1", "Stanton")],
            ),
        ];

        let registry = WikieloRegistry::from_items(items);

        // Should aggregate items across locations in same system
        let in_pyro = registry.items_in_system("Pyro");
        assert_eq!(in_pyro.len(), 2);

        // Case-insensitive
        let in_pyro_lower = registry.items_in_system("pyro");
        assert_eq!(in_pyro_lower.len(), 2);

        // Items in Stanton
        let in_stanton = registry.items_in_system("Stanton");
        assert_eq!(in_stanton.len(), 1);

        // No items in unknown system
        let in_unknown = registry.items_in_system("Nyx");
        assert!(in_unknown.is_empty());
    }

    #[test]
    fn test_items_by_category() {
        let items = vec![
            make_test_item(
                "item_1",
                "Item 1",
                ItemCategory::CreaturePart,
                vec![("Loc A", "Pyro")],
            ),
            make_test_item(
                "item_2",
                "Item 2",
                ItemCategory::CreaturePart,
                vec![("Loc B", "Pyro")],
            ),
            make_test_item(
                "item_3",
                "Item 3",
                ItemCategory::MinedMaterial,
                vec![("Loc C", "Stanton")],
            ),
        ];

        let registry = WikieloRegistry::from_items(items);

        // Should find creature parts
        let creature_parts = registry.items_by_category(ItemCategory::CreaturePart);
        assert_eq!(creature_parts.len(), 2);

        // Should find mined materials
        let mined = registry.items_by_category(ItemCategory::MinedMaterial);
        assert_eq!(mined.len(), 1);
        assert_eq!(mined[0].name, "Item 3");

        // No items in unused category
        let combat = registry.items_by_category(ItemCategory::CombatLoot);
        assert!(combat.is_empty());
    }

    #[test]
    fn test_empty_registry() {
        let registry = WikieloRegistry::new();

        // All methods should handle empty registry gracefully
        assert_eq!(registry.item_count(), 0);
        assert!(registry.all_items().is_empty());
        assert!(registry.get("any_id").is_none());
        assert!(registry.items_at_location("any_location").is_empty());
        assert!(registry.items_in_system("any_system").is_empty());
        assert!(registry
            .items_by_category(ItemCategory::CreaturePart)
            .is_empty());
        assert!(registry.all_locations().is_empty());
        assert!(registry.all_systems().is_empty());
    }
}
