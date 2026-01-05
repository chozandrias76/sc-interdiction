//! Starmap data models

use serde::{Deserialize, Serialize};

/// A location in the Star Citizen universe from starmap XML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarmapLocation {
    /// Unique reference ID
    pub id: String,
    /// Display name (often a localization key like @Stanton1_...)
    pub name: String,
    /// Parent location ID (e.g., moon belongs to planet)
    pub parent: Option<String>,
    /// Location type UUID
    pub location_type: String,
    /// Navigation icon type
    pub nav_icon: Option<String>,
    /// Affiliation UUID (faction/ownership)
    pub affiliation: Option<String>,
    /// Description (localization key)
    pub description: Option<String>,
    /// Whether location is scannable
    pub is_scannable: bool,
    /// Whether to hide in starmap
    pub hide_in_starmap: bool,
    /// Quantum travel data
    pub quantum_travel: Option<QuantumTravelData>,
    /// Location amenities (services available)
    pub amenities: Vec<String>,
}

/// Quantum travel parameters for a location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumTravelData {
    /// Radius where quantum travel is obstructed
    pub obstruction_radius: f64,
    /// Radius where ships arrive from quantum travel
    pub arrival_radius: f64,
    /// Offset for arrival point detection
    pub arrival_point_detection_offset: f64,
    /// Adoption radius (auto-selection for quantum travel)
    pub adoption_radius: f64,
}

/// A processed location with coordinates and connections
#[derive(Debug, Clone)]
pub struct ProcessedLocation {
    pub id: String,
    pub name: String,
    pub location_type: String,
    pub parent: Option<String>,
    pub position: Option<Position>,
    pub quantum_links: Vec<String>,
    pub has_trading: bool,
}

/// 3D position in space
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
