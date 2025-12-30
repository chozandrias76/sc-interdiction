//! Type definitions for the TUI.
//!
//! This module contains enums and structs used throughout the TUI,
//! separated from the application state for clarity.

/// Current view in the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    /// Target intel dashboard.
    Targets,
    /// Hot trade routes.
    Routes,
    /// System map visualization.
    Map,
    /// Help screen.
    Help,
}

/// Sort column for targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetSort {
    Value,
    Threat,
    Ship,
    Commodity,
}

/// Sort column for routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteSort {
    Profit,
    Value,
    Commodity,
}

/// A celestial body or location for map display.
#[derive(Debug, Clone)]
pub struct MapLocation {
    pub name: String,
    /// Angle from center in radians (for 2D map display).
    pub angle: f64,
    /// Distance from center (or parent) for orbital display.
    pub orbital_radius: f64,
    pub loc_type: MapLocationType,
    pub parent: Option<String>,
}

/// Type of location for map rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapLocationType {
    Star,
    Planet,
    Moon,
    Station,
}
