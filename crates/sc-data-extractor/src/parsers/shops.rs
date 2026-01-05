//! Shop inventory JSON parser

use crate::error::Result;
use crate::models::shops::ShopInventory;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Parser for shop inventory JSON files
pub struct ShopsParser {
    shops_dir: PathBuf,
}

impl ShopsParser {
    /// Creates a new shops parser
    ///
    /// # Arguments
    /// * `sclogistics_path` - Path to the SCLogistics repository root
    pub fn new(sclogistics_path: impl AsRef<Path>) -> Self {
        let shops_dir = sclogistics_path
            .as_ref()
            .join("Shops")
            .join("shopinventories");
        Self { shops_dir }
    }

    /// Parses all shop inventory JSON files
    pub fn parse_all(&self) -> Result<Vec<ShopInventory>> {
        let mut inventories = Vec::new();

        for entry in WalkDir::new(&self.shops_dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
            })
        {
            match self.parse_file(entry.path()) {
                Ok(inventory) => inventories.push(inventory),
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", entry.path().display(), e);
                }
            }
        }

        Ok(inventories)
    }

    /// Parses a single shop inventory JSON file
    fn parse_file(&self, path: &Path) -> Result<ShopInventory> {
        let content = fs::read_to_string(path)?;
        let inventory: ShopInventory = serde_json::from_str(&content)?;
        Ok(inventory)
    }

    /// Extracts shop name from filename
    pub fn extract_shop_name(path: &Path) -> Option<String> {
        path.file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.strip_prefix("Inv_").unwrap_or(s).to_string())
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shop_json() {
        let json = r#"{
            "ShopID":"4a269932-183e-362d-aacc-188e7debbea9",
            "Collection":{
                "Inventory":[
                    {
                        "ID":{"ID":["6f93f3be-2d21-4dc8-acfb-aa6eb4386c51"]},
                        "BuyPrice":0.0,
                        "SellPrice":1.36,
                        "CurrentInventory":0.0,
                        "MaxInventory":20000000.0,
                        "RentalOfferings":[]
                    }
                ]
            }
        }"#;

        let inventory: ShopInventory = serde_json::from_str(json).expect("Failed to parse JSON");
        assert_eq!(inventory.shop_id, "4a269932-183e-362d-aacc-188e7debbea9");
        assert_eq!(inventory.collection.inventory.len(), 1);
    }

    #[test]
    fn test_extract_shop_name() {
        let path = Path::new("Inv_Admin_Area18.json");
        assert_eq!(
            ShopsParser::extract_shop_name(path),
            Some("Admin_Area18".to_string())
        );
    }
}
