//! Auto-generated schema types from `SCLogistics` data
//!
//! This module contains Rust structs generated at compile time by scanning
//! the `SCLogistics` repository. The schema is inferred from actual XML and JSON
//! files, ensuring the types always match the source data format.
//!
//! # Environment Variable
//!
//! Set `SCLOGISTICS_PATH` to point to your `SCLogistics` repository clone.
//! If not set, a minimal default schema will be used.
//!
//! # Generated Types
//!
//! - `StarmapLocation` - Location data from starmap XML files (43 fields)
//! - `QuantumTravelData` - Quantum travel parameters (5 fields)
//! - `ShopInventory` - Shop data from JSON files
//! - `InventoryCollection`, `InventoryItem`, `ItemId` - Nested shop types

// Include the generated schema from build.rs
include!(concat!(env!("OUT_DIR"), "/generated_schema.rs"));

// Include schema metadata
mod schema_info {
    include!(concat!(env!("OUT_DIR"), "/schema_info.rs"));
}

pub use schema_info::*;

#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_schema_exists() {
        // Verify the generated types exist and have expected fields
        let _location = StarmapLocation::default();
        let _quantum = QuantumTravelData::default();

        // Check field counts are reasonable (not just the minimal default)
        assert!(
            STARMAP_FIELD_COUNT > 2,
            "Expected more than 2 starmap fields, got {}. Is SCLOGISTICS_PATH set?",
            STARMAP_FIELD_COUNT
        );
    }

    #[test]
    fn test_starmap_default() {
        // Test that the generated struct can be default-constructed
        let location = StarmapLocation::default();
        assert!(location.name.is_none());
        assert!(location.ref_field.is_none());
    }

    #[test]
    fn test_shop_default() {
        // Test that the generated struct can be default-constructed
        let shop = ShopInventory::default();
        assert!(shop.shop_id.is_none());
        assert!(shop.collection.is_none());
    }
}
