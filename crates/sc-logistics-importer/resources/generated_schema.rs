// Auto-generated schema from SCLogistics data
// Generated at compile time by build.rs

use serde::{Deserialize, Serialize};

// ===== Starmap Schema =====

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuantumTravelData {
    #[serde(rename = "adoptionRadius")]
    pub adoption_radius: f64,
    #[serde(rename = "arrivalPointDetectionOffset")]
    pub arrival_point_detection_offset: i64,
    #[serde(rename = "arrivalRadius")]
    pub arrival_radius: f64,
    #[serde(rename = "obstructionRadius")]
    pub obstruction_radius: f64,
    #[serde(rename = "subPointRadiusMultiplier")]
    pub sub_point_radius_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StarmapLocation {
    #[serde(rename = "__ref")]
    #[serde(default)]
    pub ref_field: Option<String>,
    #[serde(default)]
    pub affiliation: Option<String>,
    #[serde(rename = "blockTravel")]
    #[serde(default)]
    pub block_travel: Option<i64>,
    #[serde(default)]
    pub callout1: Option<String>,
    #[serde(default)]
    pub callout2: Option<String>,
    #[serde(default)]
    pub callout3: Option<String>,
    #[serde(rename = "densityScale")]
    #[serde(default)]
    pub density_scale: Option<f64>,
    #[serde(default)]
    pub depth: Option<f64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "excludeFromLevelLoad")]
    #[serde(default)]
    pub exclude_from_level_load: Option<i64>,
    #[serde(rename = "exposeForPlayerCreatedMissions")]
    #[serde(default)]
    pub expose_for_player_created_missions: Option<i64>,
    #[serde(rename = "facingMode")]
    #[serde(default)]
    pub facing_mode: Option<String>,
    #[serde(rename = "hideInStarmap")]
    #[serde(default)]
    pub hide_in_starmap: Option<i64>,
    #[serde(rename = "hideInWorld")]
    #[serde(default)]
    pub hide_in_world: Option<i64>,
    #[serde(rename = "hideWhenInAdoptionRadius")]
    #[serde(default)]
    pub hide_when_in_adoption_radius: Option<i64>,
    #[serde(rename = "innerRadius")]
    #[serde(default)]
    pub inner_radius: Option<i64>,
    #[serde(rename = "isScannable")]
    #[serde(default)]
    pub is_scannable: Option<i64>,
    #[serde(default)]
    pub jurisdiction: Option<String>,
    #[serde(rename = "locationHierarchyTag")]
    #[serde(default)]
    pub location_hierarchy_tag: Option<String>,
    #[serde(rename = "locationImagePath")]
    #[serde(default)]
    pub location_image_path: Option<String>,
    #[serde(rename = "locationMedicalImagePath")]
    #[serde(default)]
    pub location_medical_image_path: Option<String>,
    #[serde(rename = "minimumDisplaySize")]
    #[serde(default)]
    pub minimum_display_size: Option<f64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "navIcon")]
    #[serde(default)]
    pub nav_icon: Option<String>,
    #[serde(rename = "noAutoBodyRecovery")]
    #[serde(default)]
    pub no_auto_body_recovery: Option<i64>,
    #[serde(rename = "onlyShowWhenParentSelected")]
    #[serde(default)]
    pub only_show_when_parent_selected: Option<i64>,
    #[serde(rename = "outerRadius")]
    #[serde(default)]
    pub outer_radius: Option<i64>,
    #[serde(rename = "overridePermanent")]
    #[serde(default)]
    pub override_permanent: Option<String>,
    #[serde(rename = "overrideRotationSpeed")]
    #[serde(default)]
    pub override_rotation_speed: Option<i64>,
    #[serde(rename = "overrideRotationSpeedValue")]
    #[serde(default)]
    pub override_rotation_speed_value: Option<i64>,
    #[serde(rename = "overrideShowInAllZones")]
    #[serde(default)]
    pub override_show_in_all_zones: Option<String>,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(rename = "respawnLocationType")]
    #[serde(default)]
    pub respawn_location_type: Option<String>,
    #[serde(rename = "rotationSpeed")]
    #[serde(default)]
    pub rotation_speed: Option<i64>,
    #[serde(rename = "setEntityLocationOnEnter")]
    #[serde(default)]
    pub set_entity_location_on_enter: Option<i64>,
    #[serde(rename = "showOrbitLine")]
    #[serde(default)]
    pub show_orbit_line: Option<i64>,
    #[serde(default)]
    pub size: Option<f64>,
    #[serde(rename = "sizeScale")]
    #[serde(default)]
    pub size_scale: Option<i64>,
    #[serde(rename = "starMapGeomPath")]
    #[serde(default)]
    pub star_map_geom_path: Option<String>,
    #[serde(rename = "starMapMaterialPath")]
    #[serde(default)]
    pub star_map_material_path: Option<String>,
    #[serde(rename = "starMapShapePath")]
    #[serde(default)]
    pub star_map_shape_path: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub type_field: Option<String>,
    #[serde(rename = "useHoloMaterial")]
    #[serde(default)]
    pub use_holo_material: Option<i64>,
    #[serde(default)]
    pub quantum_travel_data: Option<QuantumTravelData>,
}

// ===== Shop Schema =====

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemId {
    #[serde(rename = "ID")]
    pub id: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryItem {
    #[serde(rename = "BuyPrice")]
    pub buy_price: f64,
    #[serde(rename = "CurrentInventory")]
    pub current_inventory: f64,
    #[serde(rename = "MaxInventory")]
    pub max_inventory: f64,
    #[serde(rename = "RentalOfferings")]
    pub rental_offerings: Vec<String>,
    #[serde(rename = "SellPrice")]
    pub sell_price: f64,
    #[serde(rename = "ID")]
    #[serde(default)]
    pub id: Option<ItemId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryCollection {
    #[serde(rename = "Inventory")]
    #[serde(default)]
    pub inventory: Option<InventoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShopInventory {
    #[serde(rename = "Inventory")]
    #[serde(default)]
    pub inventory: Option<Vec<serde_json::Value>>,
    #[serde(rename = "ShopID")]
    #[serde(default)]
    pub shop_id: Option<String>,
    #[serde(rename = "Collection")]
    #[serde(default)]
    pub collection: Option<InventoryCollection>,
}
