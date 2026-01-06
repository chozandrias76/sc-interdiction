//! Ship registry - manages the fleet of available ships from API data.

use api_client::{FleetYardsClient, ShipModel};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{info, warn};

use super::enrichment::from_api_ship;
use super::types::has_freight_elevator;
use super::CargoShip;

/// Repository of ships loaded from API with local enrichment.
#[derive(Clone)]
pub struct ShipRegistry {
    ships: Vec<CargoShip>,
    by_name: HashMap<String, usize>,
}

impl ShipRegistry {
    /// Load ships from API (with caching).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request or cache operations fail.
    pub async fn load() -> anyhow::Result<Self> {
        let cache_dir = PathBuf::from("data/cache");
        std::fs::create_dir_all(&cache_dir)?;

        let client = FleetYardsClient::with_cache(cache_dir);
        let api_ships = client.get_ships().await?;

        info!("Loaded {} ships from API", api_ships.len());

        Self::from_api_ships(api_ships)
    }

    /// Create registry from API ship models.
    ///
    /// # Errors
    ///
    /// This function is infallible but returns Result for API consistency.
    #[allow(clippy::cognitive_complexity)]
    pub fn from_api_ships(api_ships: Vec<ShipModel>) -> anyhow::Result<Self> {
        let mut ships = Vec::new();
        let mut by_name = HashMap::new();

        for api_ship in api_ships {
            if let Some(cargo_ship) = from_api_ship(&api_ship) {
                let name_key = normalize_ship_name(&cargo_ship.name);
                by_name.insert(name_key, ships.len());
                ships.push(cargo_ship);
            }
        }

        info!("Converted {} cargo-capable ships", ships.len());

        if ships.is_empty() {
            warn!("No cargo ships loaded - using fallback data");
            return Ok(Self::fallback());
        }

        Ok(Self { ships, by_name })
    }

    /// Get all ships in the registry.
    #[must_use]
    pub fn all_ships(&self) -> &[CargoShip] {
        &self.ships
    }

    /// Find a ship by name (case-insensitive, flexible matching).
    #[must_use]
    pub fn find_by_name(&self, name: &str) -> Option<&CargoShip> {
        let normalized = normalize_ship_name(name);
        let idx = self.by_name.get(&normalized)?;
        self.ships.get(*idx)
    }

    /// Find ships with at least the given cargo capacity.
    #[must_use]
    pub fn find_by_min_cargo(&self, min_scu: u32) -> Vec<&CargoShip> {
        self.ships
            .iter()
            .filter(|s| s.cargo_scu >= min_scu)
            .collect()
    }

    /// Find the smallest ship that can carry the given cargo.
    #[must_use]
    pub fn smallest_for_cargo(&self, scu: u32) -> Option<&CargoShip> {
        self.ships
            .iter()
            .filter(|s| s.cargo_scu >= scu)
            .min_by_key(|s| s.cargo_scu)
    }

    /// Estimate likely ship for a trade route based on cargo volume and docking restrictions.
    #[must_use]
    pub fn estimate_for_route(&self, route: &api_client::TradeRoute) -> CargoShip {
        let scu_needed = route.max_profitable_scu();

        // Check if both endpoints support freight elevator ships
        let supports_freight_elevator = has_freight_elevator(&route.terminal_origin_name)
            && has_freight_elevator(&route.terminal_destination_name);

        // Get all ships that can dock at these terminals, sorted by capacity
        let mut dockable: Vec<_> = self
            .ships
            .iter()
            .filter(|s| !s.requires_freight_elevator || supports_freight_elevator)
            .collect();
        dockable.sort_by_key(|s| s.cargo_scu);

        // Find smallest ship that can carry the full cargo
        if let Some(ship) = dockable.iter().find(|s| s.cargo_scu as f64 >= scu_needed) {
            return (*ship).clone();
        }

        // If no ship can carry the full cargo, use the largest available ship
        if let Some(largest) = dockable.last() {
            return (*largest).clone();
        }

        // Fallback
        self.create_fallback_ship(scu_needed as u32)
    }

    /// Estimate a ship that can service multiple routes (for round-trips).
    #[must_use]
    pub fn estimate_for_routes(&self, routes: &[&api_client::TradeRoute]) -> CargoShip {
        if routes.is_empty() {
            return self.create_fallback_ship(0);
        }

        // Find the maximum SCU needed across all routes
        let max_scu_needed = routes
            .iter()
            .map(|r| r.max_profitable_scu())
            .fold(0.0_f64, |a, b| a.max(b));

        // Check if ALL terminals across all routes support freight elevators
        let all_support_freight_elevator = routes.iter().all(|r| {
            has_freight_elevator(&r.terminal_origin_name)
                && has_freight_elevator(&r.terminal_destination_name)
        });

        // Get all ships that can dock at all terminals, sorted by capacity
        let mut dockable: Vec<_> = self
            .ships
            .iter()
            .filter(|s| !s.requires_freight_elevator || all_support_freight_elevator)
            .collect();
        dockable.sort_by_key(|s| s.cargo_scu);

        // Find smallest ship that can carry the max cargo
        if let Some(ship) = dockable
            .iter()
            .find(|s| s.cargo_scu as f64 >= max_scu_needed)
        {
            return (*ship).clone();
        }

        // If no ship can carry the full cargo, use the largest available ship
        if let Some(largest) = dockable.last() {
            return (*largest).clone();
        }

        self.create_fallback_ship(max_scu_needed as u32)
    }

    /// Create a fallback ship when no suitable ship is found.
    fn create_fallback_ship(&self, cargo_scu: u32) -> CargoShip {
        CargoShip {
            name: "Unknown".to_string(),
            manufacturer: "Unknown".to_string(),
            cargo_scu,
            crew_size: 1,
            threat_level: 3,
            ship_value_uec: 100_000,
            requires_freight_elevator: false,
            quantum_fuel_capacity: 2500.0,
            hydrogen_fuel_capacity: 500.0,
            qt_drive_size: 2,
            role: super::ShipRole::Cargo,
            mining_capacity_scu: None,
            mass_kg: None,
        }
    }

    /// Create a minimal fallback registry if API fails.
    fn fallback() -> Self {
        warn!("Using minimal fallback ship registry");

        // Just Aurora CL as fallback
        let fallback_ship = CargoShip {
            name: "Aurora CL".to_string(),
            manufacturer: "RSI".to_string(),
            cargo_scu: 6,
            crew_size: 1,
            threat_level: 1,
            ship_value_uec: 45_000,
            requires_freight_elevator: false,
            quantum_fuel_capacity: 583.0,
            hydrogen_fuel_capacity: 105.0,
            qt_drive_size: 1,
            role: super::ShipRole::Cargo,
            mining_capacity_scu: None,
            mass_kg: Some(25_778.0), // Aurora CL mass
        };

        let mut by_name = HashMap::new();
        // Index by full name and base name
        by_name.insert(normalize_ship_name("Aurora CL"), 0);
        by_name.insert(normalize_ship_name("Aurora"), 0);

        Self {
            ships: vec![fallback_ship],
            by_name,
        }
    }
}

/// Normalize ship name for matching.
///
/// Examples:
/// - "Cutlass Black" -> "cutlass black"
/// - "300i" -> "300i"
/// - "C2  Hercules" -> "c2 hercules"
fn normalize_ship_name(name: &str) -> String {
    name.to_lowercase()
        .replace(['-', '_'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_ship_name() {
        assert_eq!(normalize_ship_name("Cutlass Black"), "cutlass black");
        assert_eq!(normalize_ship_name("C2  Hercules"), "c2 hercules");
        assert_eq!(normalize_ship_name("300i"), "300i");
        assert_eq!(normalize_ship_name("Hull-C"), "hull c");
    }

    #[test]
    fn test_fallback_registry() {
        let registry = ShipRegistry::fallback();
        assert_eq!(registry.all_ships().len(), 1);
        assert!(registry.find_by_name("aurora").is_some());
        assert!(registry.find_by_name("Aurora CL").is_some());
    }
}
