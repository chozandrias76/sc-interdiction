//! Wikelo contract types for trade contracts and rewards.

use serde::{Deserialize, Serialize};

/// A single requirement for a Wikelo contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractRequirement {
    /// The item required (by ID reference)
    pub item_id: String,
    /// Quantity of this item needed
    pub quantity: u32,
}

impl ContractRequirement {
    /// Create a new contract requirement.
    #[must_use]
    pub fn new(item_id: impl Into<String>, quantity: u32) -> Self {
        Self {
            item_id: item_id.into(),
            quantity,
        }
    }
}

/// Type of reward from a Wikelo contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RewardType {
    /// Weapon reward
    Weapon,
    /// Armor piece
    Armor,
    /// Ship or vehicle
    Ship,
    /// In-game currency (aUEC)
    Currency,
    /// Consumable items
    Consumable,
    /// Other/miscellaneous
    Other,
}

/// A reward from completing a Wikelo contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractReward {
    /// Name of the reward item
    pub name: String,
    /// Type of reward
    pub reward_type: RewardType,
    /// Estimated market value in aUEC
    pub estimated_value: Option<u64>,
    /// Description of the reward
    pub description: Option<String>,
}

impl ContractReward {
    /// Create a new contract reward.
    #[must_use]
    pub fn new(name: impl Into<String>, reward_type: RewardType) -> Self {
        Self {
            name: name.into(),
            reward_type,
            estimated_value: None,
            description: None,
        }
    }

    /// Set the estimated value.
    #[must_use]
    pub fn with_value(mut self, value: u64) -> Self {
        self.estimated_value = Some(value);
        self
    }
}
