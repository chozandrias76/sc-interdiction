//! Wikelo registry - manages item data with bidirectional lookups.

use std::collections::HashMap;

use super::types::{ItemCategory, WikieloItem};

/// Repository of Wikelo items with bidirectional indexes.
#[derive(Clone)]
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
    /// Create registry with all known Wikelo items from static data.
    #[must_use]
    pub fn new() -> Self {
        Self::from_items(super::items::all_items())
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

    // ==================== Confidence Filtering ====================

    /// Get items with high confidence (reliability >= 4).
    ///
    /// These items have verified sources and can be relied upon for planning.
    #[must_use]
    pub fn high_confidence_items(&self) -> Vec<&WikieloItem> {
        self.items
            .iter()
            .filter(|item| item.sources.iter().any(|source| source.reliability >= 4))
            .collect()
    }

    /// Get items that need validation (reliability <= 2).
    ///
    /// These items have uncertain sources and should be verified in-game.
    #[must_use]
    pub fn needs_validation(&self) -> Vec<&WikieloItem> {
        self.items
            .iter()
            .filter(|item| item.sources.iter().all(|source| source.reliability <= 2))
            .collect()
    }
}

impl Default for WikieloRegistry {
    fn default() -> Self {
        Self::new()
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
    use crate::wikelo::types::{AcquisitionMethod, ItemSource, SourceLocation};

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
        // Use from_items with empty vec to test empty registry behavior
        let registry = WikieloRegistry::from_items(vec![]);

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

    #[test]
    fn test_new_loads_static_data() {
        let registry = WikieloRegistry::new();

        // new() should load all 31 items from static data
        assert_eq!(registry.item_count(), 31);

        // Should be able to look up known items
        assert!(registry.get("wikelo_favor").is_some());
        assert!(registry.get("irradiated_valakkar_fang_apex").is_some());
    }

    // ==================== Integration Tests with Real Data ====================

    #[test]
    fn test_registry_loads_all_items() {
        let registry = WikieloRegistry::new();
        assert_eq!(
            registry.item_count(),
            31,
            "Registry should load exactly 31 items from static data"
        );
    }

    #[test]
    fn test_valakkar_items_in_pyro() {
        let registry = WikieloRegistry::new();
        let pyro_items = registry.items_in_system("Pyro");

        // Valakkar items should be in Pyro
        let valakkar_ids = [
            "irradiated_valakkar_fang_juvenile",
            "irradiated_valakkar_fang_adult",
            "irradiated_valakkar_fang_apex",
            "irradiated_valakkar_pearl",
        ];

        for id in valakkar_ids {
            let found = pyro_items.iter().any(|item| item.id == id);
            assert!(found, "Item '{}' should be in Pyro system", id);
        }
    }

    #[test]
    fn test_creature_parts_category() {
        let registry = WikieloRegistry::new();
        let creature_parts = registry.items_by_category(ItemCategory::CreaturePart);

        // Should have 10 creature parts
        assert_eq!(
            creature_parts.len(),
            10,
            "Expected 10 creature parts, got {}",
            creature_parts.len()
        );

        // Verify known creature parts are present
        let ids: Vec<&str> = creature_parts.iter().map(|i| i.id.as_str()).collect();
        assert!(ids.contains(&"irradiated_valakkar_fang_apex"));
        assert!(ids.contains(&"tundra_kopion_horn"));
        assert!(ids.contains(&"yormandi_eye"));
        assert!(ids.contains(&"quasi_grazer_tongue"));
    }

    #[test]
    fn test_stanton_locations() {
        let registry = WikieloRegistry::new();
        let systems = registry.all_systems();

        // Stanton should be present
        assert!(
            systems.contains(&"stanton"),
            "Stanton should be in the registry's systems"
        );

        // Verify items in Stanton
        let stanton_items = registry.items_in_system("Stanton");
        assert!(!stanton_items.is_empty(), "Stanton should have items");

        // Specific items should be in Stanton
        let has_tundra_kopion = stanton_items.iter().any(|i| i.id == "tundra_kopion_horn");
        assert!(has_tundra_kopion, "Tundra Kopion Horn should be in Stanton");
    }

    #[test]
    fn test_high_confidence_count() {
        let registry = WikieloRegistry::new();
        let high_conf = registry.high_confidence_items();

        // Based on DATA-READY.md: 12 high (4-5) + 10 medium (3) items have at least one high source
        // High confidence items should be 12-22 (items with at least one source >= 4)
        assert!(
            high_conf.len() >= 12 && high_conf.len() <= 22,
            "Expected 12-22 high confidence items, got {}",
            high_conf.len()
        );
    }

    #[test]
    fn test_needs_validation_count() {
        let registry = WikieloRegistry::new();
        let needs_val = registry.needs_validation();

        // Based on DATA-READY.md: 9 low confidence items (reliability 1-2)
        assert_eq!(
            needs_val.len(),
            9,
            "Expected 9 items needing validation, got {}",
            needs_val.len()
        );

        // Known low-confidence items
        let ids: Vec<&str> = needs_val.iter().map(|i| i.id.as_str()).collect();
        assert!(ids.contains(&"irradiated_kopion_horn"));
        assert!(ids.contains(&"carinite_pure"));
        assert!(ids.contains(&"dchs_05_comp_board"));
    }

    #[test]
    fn test_specific_item_lookup() {
        let registry = WikieloRegistry::new();

        // Test wikelo_favor lookup
        let favor = registry.get("wikelo_favor");
        assert!(favor.is_some(), "wikelo_favor should exist");

        let favor = favor.unwrap();
        assert_eq!(favor.name, "Wikelo Favor");
        assert_eq!(favor.category, ItemCategory::MissionCurrency);
        assert!(
            !favor.sources.is_empty(),
            "wikelo_favor should have sources"
        );

        // Verify source details
        let source = &favor.sources[0];
        assert_eq!(source.location.system, "Stanton");
        assert_eq!(source.reliability, 5);
    }
}
