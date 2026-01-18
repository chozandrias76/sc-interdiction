//! Wikelo registry - manages item data with bidirectional lookups.

use std::collections::HashMap;

// Import types from intel crate - do NOT redefine
use intel::{ItemCategory, WikieloItem};

/// Repository of Wikelo items with bidirectional indexes.
#[derive(Clone, Default)]
#[allow(dead_code)] // Fields used in Task 2 lookup methods
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
