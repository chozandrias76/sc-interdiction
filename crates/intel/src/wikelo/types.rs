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
