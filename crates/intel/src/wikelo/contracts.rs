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
