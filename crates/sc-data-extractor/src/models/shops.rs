//! Shop and inventory data models

use serde::{Deserialize, Serialize};

/// Shop inventory data from JSON files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopInventory {
    #[serde(rename = "ShopID")]
    pub shop_id: String,
    #[serde(rename = "Collection")]
    pub collection: InventoryCollection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryCollection {
    #[serde(rename = "Inventory")]
    pub inventory: Vec<InventoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    #[serde(rename = "ID")]
    pub id: ItemId,
    #[serde(rename = "BuyPrice")]
    pub buy_price: f64,
    #[serde(rename = "SellPrice")]
    pub sell_price: f64,
    #[serde(rename = "CurrentInventory")]
    pub current_inventory: f64,
    #[serde(rename = "MaxInventory")]
    pub max_inventory: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemId {
    #[serde(rename = "ID")]
    pub id: Vec<String>,
}

/// Processed shop data linking shops to locations
#[derive(Debug, Clone)]
pub struct ProcessedShop {
    pub shop_id: String,
    pub location_id: Option<String>,
    pub shop_name: String,
    pub items: Vec<ProcessedItem>,
}

#[derive(Debug, Clone)]
pub struct ProcessedItem {
    pub item_id: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub max_inventory: f64,
}
