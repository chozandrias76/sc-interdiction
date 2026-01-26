//! Wikelo item source intelligence.
//!
//! Provides analysis methods for identifying locations as Wikelo item sources.
//! Wraps `WikieloRegistry` with flagging logic for target prioritization.

use std::sync::Arc;

use serde::Serialize;

use super::registry::WikieloRegistry;
use super::types::{ItemCategory, WikieloItem};

/// Analyzes Wikelo item source data for interdiction planning.
///
/// Wraps a `WikieloRegistry` and provides methods to flag locations
/// and systems based on the Wikelo items they contain.
pub struct WikieloIntel {
    registry: Arc<WikieloRegistry>,
}

impl WikieloIntel {
    /// Create a new `WikieloIntel` with the given registry.
    #[must_use]
    pub fn new(registry: Arc<WikieloRegistry>) -> Self {
        Self { registry }
    }

    /// Create `WikieloIntel` from static data.
    ///
    /// This creates a new `WikieloRegistry` from the built-in item data.
    #[must_use]
    pub fn from_static() -> Self {
        Self {
            registry: Arc::new(WikieloRegistry::new()),
        }
    }

    /// Check if a location is a Wikelo item source.
    ///
    /// Returns true if the location has any Wikelo items.
    #[must_use]
    pub fn is_wikelo_source(&self, location: &str) -> bool {
        !self.registry.items_at_location(location).is_empty()
    }

    /// Get all Wikelo items available at a location.
    #[must_use]
    pub fn items_at(&self, location: &str) -> Vec<&WikieloItem> {
        self.registry.items_at_location(location)
    }

    /// Flag a location with Wikelo source intelligence.
    ///
    /// Returns None if the location has no Wikelo items.
    /// Returns a `SourceFlag` with summary information if items are present.
    #[must_use]
    pub fn flag_location(&self, location: &str) -> Option<SourceFlag> {
        let items = self.registry.items_at_location(location);
        if items.is_empty() {
            return None;
        }

        let top_items: Vec<WikieloItemSummary> = items
            .iter()
            .take(5)
            .map(|item| WikieloItemSummary {
                name: item.name.clone(),
                category: item.category,
                estimated_value: item.estimated_value,
            })
            .collect();

        let has_high_value = items
            .iter()
            .any(|item| item.estimated_value.is_some_and(|v| v > 10_000_u64));

        Some(SourceFlag {
            location: location.to_string(),
            item_count: items.len(),
            top_items,
            has_high_value,
        })
    }

    /// Flag a system with aggregated Wikelo source intelligence.
    ///
    /// Returns None if the system has no Wikelo items.
    /// Returns a `SystemFlag` with summary information if items are present.
    #[must_use]
    pub fn flag_system(&self, system: &str) -> Option<SystemFlag> {
        let items = self.registry.items_in_system(system);
        if items.is_empty() {
            return None;
        }

        // Count unique locations in this system
        let location_count = items
            .iter()
            .flat_map(|item| item.sources.iter())
            .filter(|source| source.location.system.to_lowercase() == system.to_lowercase())
            .map(|source| source.location.name.to_lowercase())
            .collect::<std::collections::HashSet<_>>()
            .len();

        let top_items: Vec<WikieloItemSummary> = items
            .iter()
            .take(5)
            .map(|item| WikieloItemSummary {
                name: item.name.clone(),
                category: item.category,
                estimated_value: item.estimated_value,
            })
            .collect();

        let has_high_value = items
            .iter()
            .any(|item| item.estimated_value.is_some_and(|v| v > 10_000_u64));

        Some(SystemFlag {
            system: system.to_string(),
            location_count,
            item_count: items.len(),
            top_items,
            has_high_value,
        })
    }

    /// Get a reference to the underlying registry.
    #[must_use]
    pub fn registry(&self) -> &WikieloRegistry {
        &self.registry
    }
}

/// Source flag for a specific location.
#[derive(Debug, Clone, Serialize)]
pub struct SourceFlag {
    /// Location name.
    pub location: String,
    /// Number of Wikelo items at this location.
    pub item_count: usize,
    /// Top items at this location (up to 5).
    pub top_items: Vec<WikieloItemSummary>,
    /// Whether any item has estimated value > 10,000.
    pub has_high_value: bool,
}

/// System flag with aggregated location data.
#[derive(Debug, Clone, Serialize)]
pub struct SystemFlag {
    /// System name.
    pub system: String,
    /// Number of unique source locations in this system.
    pub location_count: usize,
    /// Total number of Wikelo items in this system.
    pub item_count: usize,
    /// Top items in this system (up to 5).
    pub top_items: Vec<WikieloItemSummary>,
    /// Whether any item has estimated value > 10,000.
    pub has_high_value: bool,
}

/// Summary of a Wikelo item for display purposes.
#[derive(Debug, Clone, Serialize)]
pub struct WikieloItemSummary {
    /// Item name.
    pub name: String,
    /// Item category.
    pub category: ItemCategory,
    /// Estimated value in aUEC (if known).
    pub estimated_value: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_wikelo_source_positive() {
        let intel = WikieloIntel::from_static();

        // Pyro I is a known Valakkar source
        assert!(intel.is_wikelo_source("Pyro I"));
    }

    #[test]
    fn test_is_wikelo_source_negative() {
        let intel = WikieloIntel::from_static();

        // Random station should not be a Wikelo source
        assert!(!intel.is_wikelo_source("Completely Unknown Location"));
    }

    #[test]
    fn test_items_at_returns_items() {
        let intel = WikieloIntel::from_static();

        // Pyro I should have Valakkar items
        let items = intel.items_at("Pyro I");
        assert!(!items.is_empty(), "Pyro I should have Wikelo items");

        // Check that we get Valakkar fangs
        let has_valakkar = items
            .iter()
            .any(|item| item.name.to_lowercase().contains("valakkar"));
        assert!(has_valakkar, "Pyro I should have Valakkar items");
    }

    #[test]
    fn test_flag_location_with_items() {
        let intel = WikieloIntel::from_static();

        // Pyro I is a known source
        let flag = intel.flag_location("Pyro I");
        assert!(flag.is_some(), "Pyro I should return a SourceFlag");

        let flag = flag.unwrap();
        assert_eq!(flag.location, "Pyro I");
        assert!(flag.item_count > 0);
        assert!(!flag.top_items.is_empty());
    }

    #[test]
    fn test_flag_location_empty() {
        let intel = WikieloIntel::from_static();

        // Non-existent location
        let flag = intel.flag_location("Completely Unknown Location");
        assert!(flag.is_none(), "Unknown location should return None");
    }

    #[test]
    fn test_flag_system_aggregates() {
        let intel = WikieloIntel::from_static();

        // Pyro system should have multiple locations
        let flag = intel.flag_system("Pyro");
        assert!(flag.is_some(), "Pyro should return a SystemFlag");

        let flag = flag.unwrap();
        assert_eq!(flag.system, "Pyro");
        assert!(flag.item_count > 0);
        assert!(flag.location_count > 0);
        assert!(!flag.top_items.is_empty());
    }
}
