//! Core types for Wikelo item and source tracking.

use serde::{Deserialize, Serialize};

/// Categories of items that Wikelo accepts, based on acquisition method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemCategory {
    /// Creature parts from hunting (Valakkar, Kopion, Yormandi, Grazer).
    CreaturePart,
    /// Mined materials (Carinite, Quantanium, ores).
    MinedMaterial,
    /// Mission reward currencies (MG Scrip, Council Scrip).
    MissionCurrency,
    /// Combat loot from Vanduul or other enemies.
    CombatLoot,
    /// Equipment and components (drives, boards).
    Equipment,
    /// Trade commodities (SCU goods).
    Commodity,
}

/// A location where Wikelo items can be acquired.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Location name (should match terminal naming for route integration).
    /// Examples: "Lazarus Transport Centers", "ARC-L1", "Pyro I"
    pub name: String,
    /// Star system containing this location.
    pub system: String,
    /// Brief description of what happens here.
    pub description: Option<String>,
}

/// How an item is obtained at a source location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AcquisitionMethod {
    /// Hunt creatures at this location.
    Hunting,
    /// Mine resources at this location.
    Mining,
    /// Complete missions that reward this item.
    Mission,
    /// Loot from combat encounters.
    Combat,
    /// Purchase from shops/kiosks.
    Purchase,
    /// Salvage from wrecks.
    Salvage,
}

/// A source for a specific item - where and how to get it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSource {
    /// The location where this item can be acquired.
    pub location: SourceLocation,
    /// How the item is acquired at this location.
    pub method: AcquisitionMethod,
    /// Relative abundance/reliability (1-5, 5 = very common/reliable).
    pub reliability: u8,
    /// Notes about acquisition (spawn times, requirements, etc.).
    pub notes: Option<String>,
}

/// An item that Wikelo accepts in trade contracts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikieloItem {
    /// Unique identifier for the item (e.g., `irradiated_valakkar_fang_apex`).
    pub id: String,
    /// Display name (e.g., "Irradiated Valakkar Fang (Apex)").
    pub name: String,
    /// Category of item.
    pub category: ItemCategory,
    /// Known source locations for this item.
    pub sources: Vec<ItemSource>,
    /// Estimated market value in aUEC (if known).
    pub estimated_value: Option<u64>,
    /// Whether this item is stackable in inventory.
    pub stackable: bool,
    /// Size in SCU if applicable (for commodities).
    pub scu_per_unit: Option<f64>,
}

impl WikieloItem {
    /// Get the primary source location (highest reliability).
    #[must_use]
    pub fn primary_source(&self) -> Option<&ItemSource> {
        self.sources.iter().max_by_key(|s| s.reliability)
    }

    /// Get all systems where this item can be found.
    #[must_use]
    pub fn source_systems(&self) -> Vec<&str> {
        self.sources
            .iter()
            .map(|s| s.location.system.as_str())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a test item source with specified location and reliability.
    fn test_source(name: &str, system: &str, reliability: u8) -> ItemSource {
        ItemSource {
            location: SourceLocation {
                name: name.to_string(),
                system: system.to_string(),
                description: None,
            },
            method: AcquisitionMethod::Hunting,
            reliability,
            notes: None,
        }
    }

    /// Create a test WikieloItem with specified sources.
    fn test_item(sources: Vec<ItemSource>) -> WikieloItem {
        WikieloItem {
            id: "test_item".to_string(),
            name: "Test Item".to_string(),
            category: ItemCategory::CreaturePart,
            sources,
            estimated_value: Some(10_000),
            stackable: true,
            scu_per_unit: None,
        }
    }

    // ========== WikieloItem::primary_source tests ==========

    #[test]
    fn primary_source_returns_none_for_empty_sources() {
        let item = test_item(vec![]);
        assert!(item.primary_source().is_none());
    }

    #[test]
    fn primary_source_returns_single_source() {
        let source = test_source("Pyro I", "Pyro", 3);
        let item = test_item(vec![source]);

        let primary = item.primary_source();
        assert!(primary.is_some());
        assert_eq!(primary.unwrap().location.name, "Pyro I");
    }

    #[test]
    fn primary_source_returns_highest_reliability() {
        let low = test_source("Low Spawn", "Stanton", 1);
        let high = test_source("High Spawn", "Pyro", 5);
        let medium = test_source("Medium Spawn", "Nyx", 3);

        let item = test_item(vec![low, high, medium]);

        let primary = item.primary_source();
        assert!(primary.is_some());
        assert_eq!(primary.unwrap().location.name, "High Spawn");
        assert_eq!(primary.unwrap().reliability, 5);
    }

    #[test]
    fn primary_source_with_equal_reliability_returns_one() {
        let source1 = test_source("Location A", "Stanton", 3);
        let source2 = test_source("Location B", "Pyro", 3);

        let item = test_item(vec![source1, source2]);

        // Should return one of them (max_by_key is stable but we just verify it returns something)
        let primary = item.primary_source();
        assert!(primary.is_some());
        assert_eq!(primary.unwrap().reliability, 3);
    }

    // ========== WikieloItem::source_systems tests ==========

    #[test]
    fn source_systems_returns_empty_for_no_sources() {
        let item = test_item(vec![]);
        assert!(item.source_systems().is_empty());
    }

    #[test]
    fn source_systems_returns_single_system() {
        let source = test_source("Location", "Pyro", 3);
        let item = test_item(vec![source]);

        let systems = item.source_systems();
        assert_eq!(systems.len(), 1);
        assert!(systems.contains(&"Pyro"));
    }

    #[test]
    fn source_systems_deduplicates_same_system() {
        let source1 = test_source("Location A", "Pyro", 3);
        let source2 = test_source("Location B", "Pyro", 4);
        let source3 = test_source("Location C", "Pyro", 2);

        let item = test_item(vec![source1, source2, source3]);

        let systems = item.source_systems();
        assert_eq!(systems.len(), 1);
        assert!(systems.contains(&"Pyro"));
    }

    #[test]
    fn source_systems_returns_multiple_unique_systems() {
        let stanton = test_source("Crusader", "Stanton", 2);
        let pyro = test_source("Pyro I", "Pyro", 4);
        let nyx = test_source("Levski", "Nyx", 3);

        let item = test_item(vec![stanton, pyro, nyx]);

        let systems = item.source_systems();
        assert_eq!(systems.len(), 3);
        assert!(systems.contains(&"Stanton"));
        assert!(systems.contains(&"Pyro"));
        assert!(systems.contains(&"Nyx"));
    }

    #[test]
    fn source_systems_mixed_unique_and_duplicate() {
        let stanton1 = test_source("ArcCorp", "Stanton", 2);
        let stanton2 = test_source("Crusader", "Stanton", 3);
        let pyro = test_source("Pyro I", "Pyro", 4);

        let item = test_item(vec![stanton1, stanton2, pyro]);

        let systems = item.source_systems();
        assert_eq!(systems.len(), 2);
        assert!(systems.contains(&"Stanton"));
        assert!(systems.contains(&"Pyro"));
    }
}
