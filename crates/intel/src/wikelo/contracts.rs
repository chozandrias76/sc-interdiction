//! Wikelo contract types for trade contracts and rewards.

use serde::{Deserialize, Serialize};

/// Categorizes contracts for filtering and grouping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractCategory {
    /// Must be completed before accessing other contracts.
    Prerequisite,
    /// Exchanges one currency or favor type for another.
    FavorExchange,
    /// Rewards a weapon upon completion.
    Weapon,
    /// Rewards armor or clothing upon completion.
    Armor,
    /// Rewards a ship or vehicle upon completion.
    Ship,
    /// Rewards equipment, consumables, or other items.
    Equipment,
}

/// Tracks how reliable contract data is.
///
/// Higher values indicate greater confidence. Uses `repr(u8)` so variants
/// can be compared with `<` / `>` operators via the derived `Ord` impl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum DataConfidence {
    /// Data inferred from the contract name only.
    Inferred = 1,
    /// Some fields confirmed but gaps remain.
    Partial = 2,
    /// All fields confirmed from a wiki or community source.
    Confirmed = 3,
    /// Verified in-game by a player.
    Verified = 4,
    /// Sourced from official game data or API.
    Authoritative = 5,
}

/// Models the different currencies in the Star Citizen economy.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurrencyType {
    /// Alpha United Earth Credits — the primary in-game currency.
    Auec,
    /// MG Scrip — earned from MicroTech Guardian missions.
    MgScrip,
    /// Council Scrip — earned from CDF missions.
    CouncilScrip,
    /// Wikelo Favor — earned by completing Wikelo contracts.
    WikieloFavor,
    /// Polaris Bit — alternate currency from Polaris system vendors.
    PolarisBit,
    /// Carinite — rare mineral used as currency in some exchanges.
    Carinite,
    /// Any other currency not yet modeled.
    Other(String),
}

/// Models a currency conversion rate between two currency types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    /// The currency being exchanged from.
    pub input_currency: CurrencyType,
    /// Quantity of input currency required.
    pub input_quantity: u32,
    /// The currency received.
    pub output_currency: CurrencyType,
    /// Quantity of output currency received.
    pub output_quantity: u32,
}

impl ExchangeRate {
    /// Calculate the conversion rate (output per input unit).
    #[must_use]
    pub fn rate(&self) -> f64 {
        f64::from(self.output_quantity) / f64::from(self.input_quantity)
    }
}

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

/// A Wikelo trade contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikieloContract {
    /// Unique identifier for this contract
    pub id: String,
    /// Display name of the contract
    pub name: String,
    /// Items required to complete this contract
    pub requirements: Vec<ContractRequirement>,
    /// Rewards for completing this contract
    pub rewards: Vec<ContractReward>,
    /// Whether this contract is repeatable
    pub repeatable: bool,
    /// Optional description/flavor text
    pub description: Option<String>,
}

impl WikieloContract {
    /// Calculate total estimated reward value.
    #[must_use]
    pub fn total_reward_value(&self) -> u64 {
        self.rewards.iter().filter_map(|r| r.estimated_value).sum()
    }

    /// Get all unique item IDs required by this contract.
    #[must_use]
    pub fn required_item_ids(&self) -> Vec<&str> {
        self.requirements
            .iter()
            .map(|r| r.item_id.as_str())
            .collect()
    }

    /// Check if this contract requires a specific item.
    #[must_use]
    pub fn requires_item(&self, item_id: &str) -> bool {
        self.requirements.iter().any(|r| r.item_id == item_id)
    }

    /// Get quantity required of a specific item.
    #[must_use]
    pub fn quantity_required(&self, item_id: &str) -> u32 {
        self.requirements
            .iter()
            .filter(|r| r.item_id == item_id)
            .map(|r| r.quantity)
            .sum()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// Create a mock contract requirement.
    fn mock_requirement(item_id: &str, qty: u32) -> ContractRequirement {
        ContractRequirement::new(item_id, qty)
    }

    /// Create a mock contract reward.
    fn mock_reward(name: &str, value: Option<u64>) -> ContractReward {
        let mut reward = ContractReward::new(name, RewardType::Other);
        if let Some(v) = value {
            reward = reward.with_value(v);
        }
        reward
    }

    /// Create a mock contract with specified requirements and rewards.
    fn mock_contract(
        requirements: Vec<ContractRequirement>,
        rewards: Vec<ContractReward>,
    ) -> WikieloContract {
        WikieloContract {
            id: "test-contract".to_string(),
            name: "Test Contract".to_string(),
            requirements,
            rewards,
            repeatable: false,
            description: None,
        }
    }

    // ContractRequirement tests

    #[test]
    fn test_requirement_new() {
        let req = ContractRequirement::new("item-123", 50);
        assert_eq!(req.item_id, "item-123");
        assert_eq!(req.quantity, 50);
    }

    // ContractReward tests

    #[test]
    fn test_reward_new() {
        let reward = ContractReward::new("Test Weapon", RewardType::Weapon);
        assert_eq!(reward.name, "Test Weapon");
        assert_eq!(reward.reward_type, RewardType::Weapon);
        assert!(reward.estimated_value.is_none());
        assert!(reward.description.is_none());
    }

    #[test]
    fn test_reward_with_value() {
        let reward = ContractReward::new("Expensive Item", RewardType::Armor).with_value(50_000);
        assert_eq!(reward.name, "Expensive Item");
        assert_eq!(reward.estimated_value, Some(50_000));
    }

    // WikieloContract tests

    #[test]
    fn test_total_reward_value_sum() {
        let rewards = vec![
            mock_reward("Reward A", Some(1000)),
            mock_reward("Reward B", Some(2500)),
            mock_reward("Reward C", Some(500)),
        ];
        let contract = mock_contract(vec![], rewards);

        assert_eq!(contract.total_reward_value(), 4000);
    }

    #[test]
    fn test_total_reward_value_none_skipped() {
        let rewards = vec![
            mock_reward("Has Value", Some(1000)),
            mock_reward("No Value", None),
            mock_reward("Also Has Value", Some(500)),
        ];
        let contract = mock_contract(vec![], rewards);

        // Only sums rewards with values, skips None
        assert_eq!(contract.total_reward_value(), 1500);
    }

    #[test]
    fn test_required_item_ids_unique() {
        let requirements = vec![
            mock_requirement("item-a", 10),
            mock_requirement("item-b", 20),
            mock_requirement("item-c", 30),
        ];
        let contract = mock_contract(requirements, vec![]);

        let ids = contract.required_item_ids();
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&"item-a"));
        assert!(ids.contains(&"item-b"));
        assert!(ids.contains(&"item-c"));
    }

    #[test]
    fn test_requires_item_true() {
        let requirements = vec![
            mock_requirement("gold-ore", 100),
            mock_requirement("silver-ore", 50),
        ];
        let contract = mock_contract(requirements, vec![]);

        assert!(contract.requires_item("gold-ore"));
        assert!(contract.requires_item("silver-ore"));
    }

    #[test]
    fn test_requires_item_false() {
        let requirements = vec![mock_requirement("gold-ore", 100)];
        let contract = mock_contract(requirements, vec![]);

        assert!(!contract.requires_item("copper-ore"));
        assert!(!contract.requires_item("nonexistent"));
    }

    #[test]
    fn test_quantity_required_single() {
        let requirements = vec![mock_requirement("titanium", 75)];
        let contract = mock_contract(requirements, vec![]);

        assert_eq!(contract.quantity_required("titanium"), 75);
    }

    #[test]
    fn test_quantity_required_multiple() {
        // Same item appearing multiple times in requirements
        let requirements = vec![
            mock_requirement("laranite", 50),
            mock_requirement("laranite", 30),
            mock_requirement("other-item", 100),
        ];
        let contract = mock_contract(requirements, vec![]);

        // Should sum quantities for the same item
        assert_eq!(contract.quantity_required("laranite"), 80);
    }

    #[test]
    fn test_quantity_required_not_found() {
        let requirements = vec![mock_requirement("gold-ore", 100)];
        let contract = mock_contract(requirements, vec![]);

        assert_eq!(contract.quantity_required("unknown-item"), 0);
    }
}
