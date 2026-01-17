//! Type definitions for `DataForge` records from scunpacked-data
//!
//! These types are derived from the actual JSON structure in `items.json` and `ships.json`.
//! We use serde's flexible parsing (skip unknown fields) for resilience to game updates.

use serde::{Deserialize, Serialize};

/// A game item from `items.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameItem {
    /// Internal class name (e.g., `Carryable_1H_CY_banu_favour_Wikelo`)
    pub class_name: String,

    /// UUID reference
    pub reference: String,

    /// Lowercase item name
    pub item_name: String,

    /// Item type (e.g., Misc, `WeaponPersonal`, Cargo)
    #[serde(rename = "type")]
    pub item_type: String,

    /// Item subtype (e.g., Harvestable, Default)
    pub sub_type: Option<String>,

    /// Item size (1-5 typically)
    pub size: Option<i32>,

    /// Item grade
    pub grade: Option<i32>,

    /// Human-readable name
    pub name: Option<String>,

    /// Tags string
    pub tags: Option<String>,

    /// Standard item data (nested)
    #[serde(rename = "stdItem")]
    pub std_item: Option<StdItem>,
}

/// Standard item data nested within `GameItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StdItem {
    /// UUID
    #[serde(rename = "UUID")]
    pub uuid: Option<String>,

    /// Class name
    pub class_name: Option<String>,

    /// Size
    pub size: Option<i32>,

    /// Grade
    pub grade: Option<i32>,

    /// Width dimension
    pub width: Option<f64>,

    /// Height dimension
    pub height: Option<f64>,

    /// Length dimension
    pub length: Option<f64>,

    /// Full type string (e.g., `Misc.Harvestable`)
    #[serde(rename = "Type")]
    pub type_string: Option<String>,

    /// Human-readable name
    pub name: Option<String>,

    /// Item description
    pub description: Option<String>,
}

/// A ship from `ships.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ship {
    /// UUID
    #[serde(rename = "UUID")]
    pub uuid: String,

    /// Internal class name (e.g., `RSI_Polaris_Collector_Military`)
    pub class_name: String,

    /// Human-readable name
    pub name: Option<String>,

    /// Ship description
    pub description: Option<String>,

    /// Career focus (e.g., Combat, Transport)
    pub career: Option<String>,

    /// Ship role
    pub role: Option<String>,

    /// Ship size (1-5)
    pub size: Option<i32>,

    /// Ship width in meters
    pub width: Option<f64>,

    /// Ship length in meters
    pub length: Option<f64>,

    /// Ship height in meters
    pub height: Option<f64>,

    /// Insurance information
    pub insurance: Option<ShipInsurance>,

    /// Whether this is a vehicle
    pub is_vehicle: Option<bool>,

    /// Whether this is a gravlev vehicle
    pub is_gravlev: Option<bool>,

    /// Whether this is a spaceship
    pub is_spaceship: Option<bool>,

    /// Ship mass in kg
    pub mass: Option<f64>,

    /// Propulsion data
    pub propulsion: Option<ShipPropulsion>,

    /// Cargo capacity in SCU
    pub cargo: Option<i32>,

    /// Ship health
    pub health: Option<f64>,

    /// Crew capacity (max)
    pub crew: Option<i32>,
}

/// Ship insurance information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipInsurance {
    /// Cost to expedite claim
    pub expedited_cost: Option<i32>,

    /// Expedited claim time in minutes
    pub expedited_claim_time: Option<i32>,

    /// Standard claim time in minutes
    pub standard_claim_time: Option<i32>,
}

/// Ship propulsion data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipPropulsion {
    /// Fuel capacity
    pub fuel_capacity: Option<f64>,

    /// Fuel intake rate
    pub fuel_intake_rate: Option<f64>,

    /// Fuel usage by thruster type
    pub fuel_usage: Option<ThrusterValues>,

    /// Thrust capacity by thruster type
    pub thrust_capacity: Option<ThrusterValues>,
}

/// Values for different thruster types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThrusterValues {
    /// Main thruster
    pub main: Option<f64>,

    /// Retro thrusters
    pub retro: Option<f64>,

    /// VTOL thrusters
    pub vtol: Option<f64>,

    /// Maneuvering thrusters
    pub maneuvering: Option<f64>,
}

/// Wikelo-specific item types we care about
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WikiloItemType {
    /// Wikelo Favor currency token
    Favor,
    /// "Quite Useful" special weapon
    SpecialWeapon,
    /// Wikelo-branded clothing
    Clothing,
    /// Hologram flair items
    Flair,
    /// Other Wikelo item
    Other,
}

impl GameItem {
    /// Get the item's full type string (e.g., `Misc.Harvestable`)
    #[must_use]
    pub fn full_type(&self) -> String {
        if let Some(sub) = &self.sub_type {
            format!("{}.{}", self.item_type, sub)
        } else {
            self.item_type.clone()
        }
    }

    /// Get display name, falling back to `class_name` if no name
    #[must_use]
    pub fn display_name(&self) -> &str {
        self.name
            .as_deref()
            .filter(|n| !n.is_empty() && !n.contains("PLACEHOLDER"))
            .or_else(|| self.std_item.as_ref().and_then(|s| s.name.as_deref()))
            .unwrap_or(&self.class_name)
    }

    /// Get description from `std_item`
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.std_item
            .as_ref()
            .and_then(|s| s.description.as_deref())
            .filter(|d| !d.is_empty())
    }

    /// Classify as a Wikelo item type
    #[must_use]
    pub fn wikelo_type(&self) -> Option<WikiloItemType> {
        let name_lower = self.class_name.to_lowercase();
        let display = self.display_name().to_lowercase();

        if name_lower.contains("banu_favour_wikelo") {
            Some(WikiloItemType::Favor)
        } else if display.contains("quite useful") {
            Some(WikiloItemType::SpecialWeapon)
        } else if name_lower.contains("jacket") && name_lower.contains("wikelo") {
            Some(WikiloItemType::Clothing)
        } else if name_lower.contains("hologram") && name_lower.contains("wikelo") {
            Some(WikiloItemType::Flair)
        } else if name_lower.contains("wikelo") || display.contains("wikelo") {
            Some(WikiloItemType::Other)
        } else {
            None
        }
    }

    /// Returns true if this is a Wikelo-related item
    #[must_use]
    pub fn is_wikelo_item(&self) -> bool {
        self.wikelo_type().is_some()
    }
}

impl Ship {
    /// Get display name, falling back to `class_name` if no name
    #[must_use]
    pub fn display_name(&self) -> &str {
        self.name
            .as_deref()
            .filter(|n| !n.is_empty())
            .unwrap_or(&self.class_name)
    }

    /// Returns true if this is a Wikelo collector ship
    #[must_use]
    pub fn is_wikelo_collector(&self) -> bool {
        self.class_name.contains("Collector")
            || self.name.as_ref().is_some_and(|n| n.contains("Wikelo"))
    }

    /// Get collector variant type from name
    #[must_use]
    pub fn collector_variant(&self) -> Option<CollectorVariant> {
        let name = self.display_name();
        if name.contains("War") {
            Some(CollectorVariant::War)
        } else if name.contains("Sneak") {
            Some(CollectorVariant::Sneak)
        } else if name.contains("Work") {
            Some(CollectorVariant::Work)
        } else if name.contains("Competition") || name.contains("Racing") {
            Some(CollectorVariant::Competition)
        } else if self.class_name.contains("Collector") {
            Some(CollectorVariant::Standard)
        } else {
            None
        }
    }
}

/// Collector ship variant types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectorVariant {
    /// Standard collector variant
    Standard,
    /// War/Military variant
    War,
    /// Sneak/Stealth variant
    Sneak,
    /// Work/Industrial variant
    Work,
    /// Competition/Racing variant
    Competition,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_item_display_name() {
        let item = GameItem {
            class_name: "Test_Item".to_string(),
            reference: "uuid".to_string(),
            item_name: "test_item".to_string(),
            item_type: "Misc".to_string(),
            sub_type: None,
            size: None,
            grade: None,
            name: Some("Test Item".to_string()),
            tags: None,
            std_item: None,
        };
        assert_eq!(item.display_name(), "Test Item");
    }

    #[test]
    fn test_game_item_placeholder_fallback() {
        let item = GameItem {
            class_name: "Test_Item".to_string(),
            reference: "uuid".to_string(),
            item_name: "test_item".to_string(),
            item_type: "Misc".to_string(),
            sub_type: None,
            size: None,
            grade: None,
            name: Some("<= PLACEHOLDER =>".to_string()),
            tags: None,
            std_item: None,
        };
        assert_eq!(item.display_name(), "Test_Item");
    }

    #[test]
    fn test_wikelo_favor_detection() {
        let item = GameItem {
            class_name: "Carryable_1H_CY_banu_favour_Wikelo".to_string(),
            reference: "uuid".to_string(),
            item_name: "banu_favour_wikelo".to_string(),
            item_type: "Misc".to_string(),
            sub_type: Some("Harvestable".to_string()),
            size: Some(1),
            grade: Some(1),
            name: Some("Wikelo Favor".to_string()),
            tags: None,
            std_item: None,
        };
        assert_eq!(item.wikelo_type(), Some(WikiloItemType::Favor));
    }

    #[test]
    fn test_collector_variant_detection() {
        let ship = Ship {
            uuid: "uuid".to_string(),
            class_name: "RSI_Scorpius_Stealth".to_string(),
            name: Some("RSI Scorpius Wikelo Sneak Special".to_string()),
            description: None,
            career: None,
            role: None,
            size: None,
            width: None,
            length: None,
            height: None,
            insurance: None,
            is_vehicle: None,
            is_gravlev: None,
            is_spaceship: None,
            mass: None,
            propulsion: None,
            cargo: None,
            health: None,
            crew: None,
        };
        assert!(ship.is_wikelo_collector());
        assert_eq!(ship.collector_variant(), Some(CollectorVariant::Sneak));
    }
}
