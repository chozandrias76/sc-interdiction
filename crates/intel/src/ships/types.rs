//! Cargo ship data for target estimation.

use serde::{Deserialize, Serialize};

/// Ship role indicating primary function and cargo capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipRole {
    /// Standard commodity haulers - carry refined materials and trade goods
    Cargo,
    /// Combat vessels - fighters and military ships
    Combat,
    /// Can extract ore AND carry raw ore to refineries
    Mining,
    /// Can salvage and carry RMC/CMATs to refineries
    Salvage,
    /// Personnel/vehicle transport - passenger ships, dropships, troop carriers
    Transport,
    /// Exploration and scanning vessels
    Exploration,
    /// Support ships - refueling, repair, medical
    Support,
}

/// A cargo ship with relevant stats for interdiction analysis.
#[derive(Debug, Clone, Serialize)]
pub struct CargoShip {
    /// Ship name (e.g., "Caterpillar").
    pub name: String,
    /// Ship manufacturer (e.g., "Drake").
    pub manufacturer: String,
    /// Cargo capacity in SCU.
    pub cargo_scu: u32,
    /// Maximum crew size.
    pub crew_size: u8,
    /// Ship combat difficulty for interdictors (1-10).
    /// 1 = easy kill (no weapons, slow, fragile)
    /// 5 = moderate (some weapons, tanky, or fast)
    /// 10 = very difficult (heavy weapons, fighter escort, etc.)
    pub threat_level: u8,
    /// Typical value of the ship itself (purchase price in aUEC).
    pub ship_value_uec: u64,
    /// Whether this ship requires a station with external freight elevators (Hull series).
    pub requires_freight_elevator: bool,
    /// Quantum fuel tank capacity (units).
    pub quantum_fuel_capacity: f64,
    /// Hydrogen fuel tank capacity (units).
    pub hydrogen_fuel_capacity: f64,
    /// Quantum drive size class (1=S1/small, 2=S2/medium, 3=S3/large).
    pub qt_drive_size: u8,
    /// Ship role (Cargo, Combat, Mining, Salvage, Transport, Exploration, or Support).
    pub role: ShipRole,
    /// Mining/salvage capacity in SCU (for mining and salvage ships).
    pub mining_capacity_scu: Option<u32>,
    /// Ship mass in kilograms (for salvage yield calculation).
    pub mass_kg: Option<f64>,
}

impl CargoShip {
    /// Calculate salvage value breakdown for this ship.
    ///
    /// Returns (`component_value`, `hull_salvage_min`, `hull_salvage_max`, `total_min`, `total_max`)
    #[must_use]
    pub fn salvage_value(&self, cm_price_per_scu: f64) -> SalvageValue {
        // Improved component value estimation
        let component_value = self.estimate_component_value();

        // Hull salvage: based on ship mass
        let (hull_min, hull_max) = if let Some(mass) = self.mass_kg {
            // RMC yield: ~1 SCU per 100kg of ship mass
            let rmc_scu = mass / 100.0;

            // Refining: 50% (worst) to 80% (best) yield
            let cm_min_scu = rmc_scu * 0.50;
            let cm_max_scu = rmc_scu * 0.80;

            // Market value
            let hull_min = (cm_min_scu * cm_price_per_scu) as u64;
            let hull_max = (cm_max_scu * cm_price_per_scu) as u64;
            (hull_min, hull_max)
        } else {
            // Fallback: estimate from cargo capacity
            let estimated_mass = (self.cargo_scu as f64) * 1000.0; // ~1000kg per SCU
            let rmc_scu = estimated_mass / 100.0;
            let cm_min_scu = rmc_scu * 0.50;
            let cm_max_scu = rmc_scu * 0.80;
            let hull_min = (cm_min_scu * cm_price_per_scu) as u64;
            let hull_max = (cm_max_scu * cm_price_per_scu) as u64;
            (hull_min, hull_max)
        };

        SalvageValue {
            component_value,
            hull_salvage_min: hull_min,
            hull_salvage_max: hull_max,
            total_min: component_value + hull_min,
            total_max: component_value + hull_max,
        }
    }

    /// Estimate component value based on ship characteristics.
    ///
    /// Uses ship size (QT drive), manufacturer quality, role, and crew size
    /// to estimate the salvage value of stock components.
    fn estimate_component_value(&self) -> u64 {
        // Base component value by QT drive size (proxy for ship size class)
        let base = match self.qt_drive_size {
            1 => 5_000,  // Small ships: S1 components
            2 => 18_000, // Medium ships: S2 components
            3 => 50_000, // Large ships: S3 components
            _ => 15_000, // Fallback
        };

        // Manufacturer quality multiplier
        let mfr_mult = match self.manufacturer.as_str() {
            "Drake" => 0.6,                          // Budget components
            "Greycat" => 0.7,                        // Budget industrial
            "MISC" | "Consolidated Outland" => 0.85, // Mid-tier
            "Argo" => 0.9,                           // Industrial
            "Anvil" | "Aegis" => 1.2,                // Military-grade
            "Origin" => 1.5,                         // Premium luxury
            _ => 1.0,                                // Standard (RSI, Crusader, etc.)
        };

        // Role multiplier (affects stock loadout quality)
        let role_mult = match self.role {
            ShipRole::Cargo => 0.7,       // Barebones cargo haulers
            ShipRole::Transport => 0.8,   // Transport haulers
            ShipRole::Mining => 1.3,      // Industrial-grade equipment
            ShipRole::Salvage => 1.2,     // Specialized salvage gear
            ShipRole::Combat => 1.4,      // Military-grade weapons and systems
            ShipRole::Exploration => 1.1, // Advanced scanners and systems
            ShipRole::Support => 1.0,     // Standard support equipment
        };

        // Crew size multiplier (more crew = more life support, more components)
        let crew_mult = 1.0 + (self.crew_size as f64 * 0.05).min(0.3);

        (base as f64 * mfr_mult * role_mult * crew_mult) as u64
    }
}

/// Salvage value breakdown for an interdicted ship.
#[derive(Debug, Clone, Serialize)]
pub struct SalvageValue {
    /// Estimated value of salvageable components.
    pub component_value: u64,
    /// Minimum hull salvage value (worst refinery yield, 50%).
    pub hull_salvage_min: u64,
    /// Maximum hull salvage value (best refinery yield, 80%).
    pub hull_salvage_max: u64,
    /// Total minimum interdiction value.
    pub total_min: u64,
    /// Total maximum interdiction value.
    pub total_max: u64,
}

impl CargoShip {
    /// Calculate interdiction value score for this ship carrying given cargo value.
    /// Higher = more attractive target (better value-to-risk ratio).
    /// Formula: `cargo_value` / (`threat_level` * `crew_factor`)
    #[must_use]
    pub fn interdiction_value(&self, cargo_value: f64) -> f64 {
        let threat_factor = self.threat_level.max(1) as f64;
        let crew_factor = 1.0 + (self.crew_size.saturating_sub(1) as f64 * 0.2); // Each extra crew adds 20% difficulty
        cargo_value / (threat_factor * crew_factor)
    }

    /// Get the quantum drive efficiency for this ship.
    #[must_use]
    pub fn qt_drive_efficiency(&self) -> Option<&'static route_graph::QtDriveEfficiency> {
        route_graph::efficiency_for_size(self.qt_drive_size)
    }

    /// Check if this ship can complete a route of given distance (Mkm).
    ///
    /// Returns (`can_complete`, `fuel_required`, `fuel_remaining`).
    #[must_use]
    pub fn can_complete_route(&self, distance_mkm: f64) -> (bool, f64, f64) {
        if let Some(efficiency) = self.qt_drive_efficiency() {
            route_graph::can_complete_route(distance_mkm, self.quantum_fuel_capacity, efficiency)
        } else {
            (false, 0.0, 0.0)
        }
    }

    /// Calculate maximum quantum travel range in Mkm.
    #[must_use]
    pub fn max_range_mkm(&self) -> f64 {
        if let Some(efficiency) = self.qt_drive_efficiency() {
            route_graph::max_range_mkm(self.quantum_fuel_capacity, efficiency)
        } else {
            0.0
        }
    }
}

/// Locations known to have freight elevator facilities for large ships like Hull C.
/// This includes orbital stations and some landing zones with external pads.
static FREIGHT_ELEVATOR_LOCATIONS: &[&str] = &[
    // Stanton orbital stations
    "station", // Generic - catches most orbital stations
    "everus harbor",
    "port tressler",
    "baijini point",
    "seraphim",
    // Lagrange stations (ARC-L1 through L5, HUR-L1, etc.)
    "arc-l",
    "hur-l",
    "cru-l",
    "mic-l",
    // Nyx
    "levski", // Levski has external freight elevators
    "stanton gateway",
    // Pyro stations
    "checkmate",
    "endgame",
    "gaslight",
    "ruin",
    // Grim Hex
    "grim hex",
];

/// Check if a terminal name indicates it has freight elevator facilities.
/// Hull C and similar large ships can only dock at locations with external freight elevators.
pub(super) fn has_freight_elevator(terminal_name: &str) -> bool {
    let name_lower = terminal_name.to_lowercase();
    FREIGHT_ELEVATOR_LOCATIONS
        .iter()
        .any(|loc| name_lower.contains(loc))
}

/// Estimated loot from a successful interdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootEstimate {
    /// Total value of cargo carried by target (aUEC).
    pub cargo_value: f64,
    /// Estimated value of recoverable cargo after interdiction (aUEC).
    pub recoverable_cargo: f64,
    /// Estimated value of salvageable ship components (aUEC).
    pub salvage_value: f64,
    /// Total estimated loot value (cargo + salvage, in aUEC).
    pub total: f64,
}

impl LootEstimate {
    /// Calculate loot estimation for a target ship carrying cargo.
    ///
    /// # Arguments
    /// * `cargo_value` - Total value of cargo on board
    /// * `ship` - The target ship
    /// * `destruction_level` - How destructive the interdiction method (0.0 = disable, 1.0 = destroy)
    ///
    /// # Returns
    /// A loot estimate with cargo recovery and salvage value calculations.
    ///
    /// # Assumptions
    /// - Cargo recovery rate: 70% for disable (`destruction_level` = 0.0), decreases with destruction
    /// - Salvage value: ~5-15% of ship price based on ship size and destruction level
    /// - Larger ships have more salvageable components but are harder to fully recover
    #[must_use]
    pub fn calculate(cargo_value: f64, ship: &CargoShip, destruction_level: f64) -> Self {
        // Clamp destruction level to [0.0, 1.0]
        let destruction = destruction_level.clamp(0.0, 1.0);

        // Cargo recovery rate decreases with destruction
        // At 0% destruction (disable): 70% recovery
        // At 50% destruction: 40% recovery
        // At 100% destruction (fully destroyed): 10% recovery
        let base_recovery_rate = 0.70;
        let cargo_recovery_rate = base_recovery_rate * (1.0 - (destruction * 0.857)); // ~10% at full destruction
        let recoverable_cargo = cargo_value * cargo_recovery_rate;

        // Salvage value based on ship size and destruction
        // Base salvage rate: 5% for small ships, 10% for medium, 15% for large
        // Increases slightly with destruction (more exposed components)
        let base_salvage_rate = match ship.cargo_scu {
            scu if scu < 100 => 0.05, // Small ships (Aurora, Avenger)
            scu if scu < 300 => 0.10, // Medium ships (Freelancer, Cutlass, Constellation)
            _ => 0.15,                // Large ships (Caterpillar, C2, Hull series)
        };

        // Salvage rate increases slightly with destruction (up to 1.5x at full destruction)
        let destruction_multiplier = 1.0 + (destruction * 0.5);
        let salvage_rate = base_salvage_rate * destruction_multiplier;

        // Estimate ship value based on size and role (rough approximation)
        let estimated_ship_value = estimate_ship_value(ship);
        let salvage_value = estimated_ship_value * salvage_rate;

        let total = recoverable_cargo + salvage_value;

        Self {
            cargo_value,
            recoverable_cargo,
            salvage_value,
            total,
        }
    }

    /// Calculate loot estimation assuming non-destructive interdiction (disable).
    #[must_use]
    pub fn calculate_disable(cargo_value: f64, ship: &CargoShip) -> Self {
        Self::calculate(cargo_value, ship, 0.0)
    }

    /// Calculate loot estimation assuming moderate destruction.
    #[must_use]
    pub fn calculate_moderate(cargo_value: f64, ship: &CargoShip) -> Self {
        Self::calculate(cargo_value, ship, 0.5)
    }

    /// Calculate loot estimation assuming complete destruction.
    #[must_use]
    pub fn calculate_destroy(cargo_value: f64, ship: &CargoShip) -> Self {
        Self::calculate(cargo_value, ship, 1.0)
    }
}

/// Estimate the value of a cargo ship based on its characteristics.
///
/// This is a rough approximation based on cargo capacity, role, and typical ship prices.
fn estimate_ship_value(ship: &CargoShip) -> f64 {
    // Base value on cargo capacity (aUEC per SCU)
    let base_value_per_scu = match ship.cargo_scu {
        scu if scu < 50 => 15_000.0,  // Small ships: ~750k-1M
        scu if scu < 100 => 12_000.0, // Small-medium: ~1-1.2M
        scu if scu < 200 => 10_000.0, // Medium: ~1.5-2M
        scu if scu < 400 => 8_000.0,  // Large: ~2.5-3.2M
        _ => 6_000.0,                 // Very large: ~3.6M+
    };

    let base_value = ship.cargo_scu as f64 * base_value_per_scu;

    // Adjust for ship role and capabilities
    let role_multiplier = if ship.requires_freight_elevator {
        1.3 // Ships with freight elevators tend to be more expensive (C2, M2, etc.)
    } else {
        1.0
    };

    base_value * role_multiplier
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a test cargo ship with configurable parameters.
    fn test_cargo_ship() -> CargoShip {
        CargoShip {
            name: "Test Hauler".to_string(),
            manufacturer: "RSI".to_string(),
            cargo_scu: 100,
            crew_size: 2,
            threat_level: 3,
            ship_value_uec: 1_500_000,
            requires_freight_elevator: false,
            quantum_fuel_capacity: 1000.0,
            hydrogen_fuel_capacity: 5000.0,
            qt_drive_size: 2,
            role: ShipRole::Cargo,
            mining_capacity_scu: None,
            mass_kg: Some(50_000.0),
        }
    }

    // ========== CargoShip::salvage_value tests ==========

    #[test]
    fn salvage_value_with_known_mass() {
        let ship = test_cargo_ship();
        let cm_price = 100.0; // 100 aUEC per SCU of construction materials

        let salvage = ship.salvage_value(cm_price);

        // mass_kg = 50,000
        // RMC yield = 50,000 / 100 = 500 SCU
        // min CM = 500 * 0.50 = 250 SCU => 25,000 aUEC
        // max CM = 500 * 0.80 = 400 SCU => 40,000 aUEC
        assert_eq!(salvage.hull_salvage_min, 25_000);
        assert_eq!(salvage.hull_salvage_max, 40_000);

        // Component value comes from estimate_component_value
        assert!(salvage.component_value > 0);

        // Total = component + hull
        assert_eq!(
            salvage.total_min,
            salvage.component_value + salvage.hull_salvage_min
        );
        assert_eq!(
            salvage.total_max,
            salvage.component_value + salvage.hull_salvage_max
        );
    }

    #[test]
    fn salvage_value_without_mass_uses_fallback() {
        let mut ship = test_cargo_ship();
        ship.mass_kg = None;
        ship.cargo_scu = 50;
        let cm_price = 100.0;

        let salvage = ship.salvage_value(cm_price);

        // Fallback: estimated_mass = cargo_scu * 1000 = 50,000 kg
        // RMC yield = 50,000 / 100 = 500 SCU
        // min CM = 500 * 0.50 = 250 SCU => 25,000 aUEC
        // max CM = 500 * 0.80 = 400 SCU => 40,000 aUEC
        assert_eq!(salvage.hull_salvage_min, 25_000);
        assert_eq!(salvage.hull_salvage_max, 40_000);
    }

    // ========== CargoShip::estimate_component_value tests ==========

    #[test]
    fn estimate_component_value_varies_by_qt_drive_size() {
        let mut small = test_cargo_ship();
        small.qt_drive_size = 1;

        let mut medium = test_cargo_ship();
        medium.qt_drive_size = 2;

        let mut large = test_cargo_ship();
        large.qt_drive_size = 3;

        // Same manufacturer/role/crew, different QT size
        let small_val = small.salvage_value(100.0).component_value;
        let medium_val = medium.salvage_value(100.0).component_value;
        let large_val = large.salvage_value(100.0).component_value;

        assert!(
            small_val < medium_val,
            "S1 should have lower component value than S2"
        );
        assert!(
            medium_val < large_val,
            "S2 should have lower component value than S3"
        );
    }

    #[test]
    fn estimate_component_value_varies_by_manufacturer() {
        let mut drake = test_cargo_ship();
        drake.manufacturer = "Drake".to_string();

        let mut origin = test_cargo_ship();
        origin.manufacturer = "Origin".to_string();

        let drake_val = drake.salvage_value(100.0).component_value;
        let origin_val = origin.salvage_value(100.0).component_value;

        assert!(
            drake_val < origin_val,
            "Drake (budget) should have lower value than Origin (premium)"
        );
    }

    #[test]
    fn estimate_component_value_varies_by_role() {
        let mut cargo = test_cargo_ship();
        cargo.role = ShipRole::Cargo;

        let mut combat = test_cargo_ship();
        combat.role = ShipRole::Combat;

        let cargo_val = cargo.salvage_value(100.0).component_value;
        let combat_val = combat.salvage_value(100.0).component_value;

        assert!(
            cargo_val < combat_val,
            "Cargo ships should have lower component value than combat ships"
        );
    }

    // ========== CargoShip::interdiction_value tests ==========

    #[test]
    fn interdiction_value_basic_calculation() {
        let ship = test_cargo_ship();
        let cargo_value = 100_000.0;

        let iv = ship.interdiction_value(cargo_value);

        // threat_factor = 3 (threat_level)
        // crew_factor = 1.0 + (2 - 1) * 0.2 = 1.2
        // iv = 100,000 / (3 * 1.2) = 100,000 / 3.6 = 27,777.77...
        let expected = 100_000.0 / (3.0 * 1.2);
        assert!((iv - expected).abs() < 0.01);
    }

    #[test]
    fn interdiction_value_higher_threat_reduces_value() {
        let mut low_threat = test_cargo_ship();
        low_threat.threat_level = 1;

        let mut high_threat = test_cargo_ship();
        high_threat.threat_level = 10;

        let cargo_value = 100_000.0;
        let low_iv = low_threat.interdiction_value(cargo_value);
        let high_iv = high_threat.interdiction_value(cargo_value);

        assert!(
            low_iv > high_iv,
            "Lower threat ships should have higher interdiction value"
        );
    }

    #[test]
    fn interdiction_value_more_crew_reduces_value() {
        let mut solo = test_cargo_ship();
        solo.crew_size = 1;

        let mut crewed = test_cargo_ship();
        crewed.crew_size = 5;

        let cargo_value = 100_000.0;
        let solo_iv = solo.interdiction_value(cargo_value);
        let crewed_iv = crewed.interdiction_value(cargo_value);

        assert!(
            solo_iv > crewed_iv,
            "Solo ships should have higher interdiction value than crewed ships"
        );
    }

    #[test]
    fn interdiction_value_zero_threat_uses_minimum() {
        let mut ship = test_cargo_ship();
        ship.threat_level = 0; // Edge case

        let iv = ship.interdiction_value(100_000.0);

        // Should use max(1) to avoid division by zero
        assert!(iv.is_finite());
        assert!(iv > 0.0);
    }

    // ========== CargoShip::can_complete_route tests ==========

    #[test]
    fn can_complete_route_returns_false_for_invalid_qt_size() {
        let mut ship = test_cargo_ship();
        ship.qt_drive_size = 99; // Invalid

        let (can_complete, fuel_required, fuel_remaining) = ship.can_complete_route(100.0);

        assert!(!can_complete);
        assert_eq!(fuel_required, 0.0);
        assert_eq!(fuel_remaining, 0.0);
    }

    #[test]
    fn can_complete_route_succeeds_for_short_distance() {
        let ship = test_cargo_ship();

        // Short distance should be completable
        let (can_complete, fuel_required, fuel_remaining) = ship.can_complete_route(10.0);

        assert!(can_complete, "Should complete short 10 Mkm route");
        assert!(fuel_required > 0.0);
        assert!(fuel_remaining >= 0.0);
    }

    // ========== CargoShip::max_range_mkm tests ==========

    #[test]
    fn max_range_mkm_returns_zero_for_invalid_qt_size() {
        let mut ship = test_cargo_ship();
        ship.qt_drive_size = 99;

        assert_eq!(ship.max_range_mkm(), 0.0);
    }

    #[test]
    fn max_range_mkm_larger_tank_means_longer_range() {
        let mut small_tank = test_cargo_ship();
        small_tank.quantum_fuel_capacity = 500.0;

        let mut large_tank = test_cargo_ship();
        large_tank.quantum_fuel_capacity = 2000.0;

        let small_range = small_tank.max_range_mkm();
        let large_range = large_tank.max_range_mkm();

        assert!(
            large_range > small_range,
            "Larger fuel tank should provide longer range"
        );
    }

    // ========== has_freight_elevator tests ==========

    #[test]
    fn has_freight_elevator_station_keyword() {
        assert!(has_freight_elevator("Everus Harbor Station"));
        assert!(has_freight_elevator("CRU-L1 Station"));
    }

    #[test]
    fn has_freight_elevator_arc_l_locations() {
        assert!(has_freight_elevator("ARC-L1 Wide Forest Station"));
        assert!(has_freight_elevator("ARC-L2"));
        assert!(has_freight_elevator("arc-l5")); // lowercase
    }

    #[test]
    fn has_freight_elevator_hur_l_locations() {
        assert!(has_freight_elevator("HUR-L1 Green Glade Station"));
        assert!(has_freight_elevator("HUR-L2"));
    }

    #[test]
    fn has_freight_elevator_surface_locations_without_elevator() {
        // Surface locations generally don't have freight elevators
        assert!(!has_freight_elevator("Lorville"));
        assert!(!has_freight_elevator("Area18"));
        assert!(!has_freight_elevator("New Babbage"));
    }

    #[test]
    fn has_freight_elevator_case_insensitive() {
        assert!(has_freight_elevator("EVERUS HARBOR"));
        assert!(has_freight_elevator("Everus Harbor"));
        assert!(has_freight_elevator("everus harbor"));
        assert!(has_freight_elevator("GRIM HEX"));
        assert!(has_freight_elevator("grim hex"));
    }

    #[test]
    fn has_freight_elevator_pyro_stations() {
        assert!(has_freight_elevator("Checkmate Station"));
        assert!(has_freight_elevator("Ruin Station"));
        assert!(has_freight_elevator("Gaslight"));
    }

    // ========== LootEstimate tests ==========

    #[test]
    fn loot_estimate_calculate_disable() {
        let ship = test_cargo_ship();
        let cargo_value = 100_000.0;

        let loot = LootEstimate::calculate_disable(cargo_value, &ship);

        assert_eq!(loot.cargo_value, cargo_value);
        // At 0% destruction: 70% recovery
        assert!((loot.recoverable_cargo - 70_000.0).abs() < 1.0);
        assert!(loot.salvage_value > 0.0);
        assert!((loot.total - (loot.recoverable_cargo + loot.salvage_value)).abs() < 0.01);
    }

    #[test]
    fn loot_estimate_calculate_moderate() {
        let ship = test_cargo_ship();
        let cargo_value = 100_000.0;

        let loot = LootEstimate::calculate_moderate(cargo_value, &ship);

        // At 50% destruction: ~40% recovery
        // base_recovery * (1 - 0.5 * 0.857) = 0.70 * 0.5715 = ~0.40
        assert!(loot.recoverable_cargo < 70_000.0); // Less than disable
        assert!(loot.recoverable_cargo > 10_000.0); // More than destroy
    }

    #[test]
    fn loot_estimate_calculate_destroy() {
        let ship = test_cargo_ship();
        let cargo_value = 100_000.0;

        let loot = LootEstimate::calculate_destroy(cargo_value, &ship);

        // At 100% destruction: ~10% recovery
        // base_recovery * (1 - 1.0 * 0.857) = 0.70 * 0.143 = ~0.10
        assert!(loot.recoverable_cargo < 15_000.0);
    }

    #[test]
    fn loot_estimate_destruction_clamps_to_valid_range() {
        let ship = test_cargo_ship();
        let cargo_value = 100_000.0;

        // Over 1.0 should clamp
        let loot_over = LootEstimate::calculate(cargo_value, &ship, 2.0);
        let loot_max = LootEstimate::calculate(cargo_value, &ship, 1.0);

        assert!((loot_over.recoverable_cargo - loot_max.recoverable_cargo).abs() < 0.01);

        // Under 0.0 should clamp
        let loot_under = LootEstimate::calculate(cargo_value, &ship, -1.0);
        let loot_min = LootEstimate::calculate(cargo_value, &ship, 0.0);

        assert!((loot_under.recoverable_cargo - loot_min.recoverable_cargo).abs() < 0.01);
    }

    #[test]
    fn loot_estimate_larger_ships_have_higher_salvage() {
        let mut small_ship = test_cargo_ship();
        small_ship.cargo_scu = 30; // Small

        let mut large_ship = test_cargo_ship();
        large_ship.cargo_scu = 500; // Large

        let cargo_value = 100_000.0;
        let small_loot = LootEstimate::calculate_disable(cargo_value, &small_ship);
        let large_loot = LootEstimate::calculate_disable(cargo_value, &large_ship);

        assert!(
            large_loot.salvage_value > small_loot.salvage_value,
            "Larger ships should have higher salvage value"
        );
    }

    #[test]
    fn loot_estimate_destruction_increases_salvage_rate() {
        let ship = test_cargo_ship();
        let cargo_value = 100_000.0;

        let loot_disable = LootEstimate::calculate_disable(cargo_value, &ship);
        let loot_destroy = LootEstimate::calculate_destroy(cargo_value, &ship);

        // Salvage rate increases with destruction (up to 1.5x)
        assert!(
            loot_destroy.salvage_value > loot_disable.salvage_value,
            "Destruction should increase salvage value"
        );
    }

    // ========== Additional coverage tests ==========

    #[test]
    fn ship_role_equality() {
        assert_eq!(ShipRole::Cargo, ShipRole::Cargo);
        assert_ne!(ShipRole::Cargo, ShipRole::Combat);
        assert_ne!(ShipRole::Mining, ShipRole::Salvage);
    }

    #[test]
    fn ship_role_debug() {
        let role = ShipRole::Mining;
        let debug_str = format!("{:?}", role);
        assert!(debug_str.contains("Mining"));
    }

    #[test]
    fn cargo_ship_debug() {
        let ship = test_cargo_ship();
        let debug_str = format!("{:?}", ship);
        assert!(debug_str.contains("Test Hauler"));
        assert!(debug_str.contains("RSI"));
    }

    #[test]
    fn cargo_ship_clone() {
        let ship = test_cargo_ship();
        let cloned = ship.clone();
        assert_eq!(cloned.name, ship.name);
        assert_eq!(cloned.cargo_scu, ship.cargo_scu);
    }

    #[test]
    fn salvage_value_debug() {
        let ship = test_cargo_ship();
        let salvage = ship.salvage_value(100.0);
        let debug_str = format!("{:?}", salvage);
        assert!(debug_str.contains("SalvageValue"));
    }

    #[test]
    fn salvage_value_clone() {
        let ship = test_cargo_ship();
        let salvage = ship.salvage_value(100.0);
        let cloned = salvage.clone();
        assert_eq!(cloned.component_value, salvage.component_value);
    }

    #[test]
    fn loot_estimate_debug() {
        let ship = test_cargo_ship();
        let loot = LootEstimate::calculate_disable(100_000.0, &ship);
        let debug_str = format!("{:?}", loot);
        assert!(debug_str.contains("LootEstimate"));
    }

    #[test]
    fn loot_estimate_clone() {
        let ship = test_cargo_ship();
        let loot = LootEstimate::calculate_disable(100_000.0, &ship);
        let cloned = loot.clone();
        assert_eq!(cloned.cargo_value, loot.cargo_value);
    }

    #[test]
    fn estimate_component_value_greycat_manufacturer() {
        let mut ship = test_cargo_ship();
        ship.manufacturer = "Greycat".to_string();
        let val = ship.salvage_value(100.0).component_value;
        assert!(val > 0);
    }

    #[test]
    fn estimate_component_value_misc_manufacturer() {
        let mut ship = test_cargo_ship();
        ship.manufacturer = "MISC".to_string();
        let val = ship.salvage_value(100.0).component_value;
        assert!(val > 0);
    }

    #[test]
    fn estimate_component_value_argo_manufacturer() {
        let mut ship = test_cargo_ship();
        ship.manufacturer = "Argo".to_string();
        let val = ship.salvage_value(100.0).component_value;
        assert!(val > 0);
    }

    #[test]
    fn estimate_component_value_anvil_manufacturer() {
        let mut ship = test_cargo_ship();
        ship.manufacturer = "Anvil".to_string();
        let val_anvil = ship.salvage_value(100.0).component_value;

        ship.manufacturer = "Aegis".to_string();
        let val_aegis = ship.salvage_value(100.0).component_value;

        // Both military-grade should have same multiplier
        assert_eq!(val_anvil, val_aegis);
    }

    #[test]
    fn estimate_component_value_all_roles() {
        let mut ship = test_cargo_ship();

        for role in [
            ShipRole::Cargo,
            ShipRole::Combat,
            ShipRole::Mining,
            ShipRole::Salvage,
            ShipRole::Transport,
            ShipRole::Exploration,
            ShipRole::Support,
        ] {
            ship.role = role;
            let val = ship.salvage_value(100.0).component_value;
            assert!(
                val > 0,
                "Role {:?} should have positive component value",
                role
            );
        }
    }

    #[test]
    fn estimate_component_value_fallback_qt_size() {
        let mut ship = test_cargo_ship();
        ship.qt_drive_size = 99; // Invalid, should use fallback

        let val = ship.salvage_value(100.0).component_value;
        assert!(val > 0, "Should use fallback base value");
    }

    #[test]
    fn estimate_ship_value_freight_elevator_multiplier() {
        let mut regular = test_cargo_ship();
        regular.requires_freight_elevator = false;

        let mut freight = test_cargo_ship();
        freight.requires_freight_elevator = true;

        let regular_val = estimate_ship_value(&regular);
        let freight_val = estimate_ship_value(&freight);

        assert!(
            freight_val > regular_val,
            "Ships with freight elevator should have higher estimated value"
        );
    }

    #[test]
    fn estimate_ship_value_by_cargo_size() {
        let mut tiny = test_cargo_ship();
        tiny.cargo_scu = 20;

        let mut small = test_cargo_ship();
        small.cargo_scu = 80;

        let mut medium = test_cargo_ship();
        medium.cargo_scu = 150;

        let mut large = test_cargo_ship();
        large.cargo_scu = 350;

        let mut huge = test_cargo_ship();
        huge.cargo_scu = 600;

        // Larger ships should have higher value (even with lower price per SCU)
        let tiny_val = estimate_ship_value(&tiny);
        let huge_val = estimate_ship_value(&huge);

        assert!(huge_val > tiny_val);
    }

    #[test]
    fn qt_drive_efficiency_returns_some_for_valid_sizes() {
        let mut ship = test_cargo_ship();

        for size in 1..=3 {
            ship.qt_drive_size = size;
            assert!(
                ship.qt_drive_efficiency().is_some(),
                "Size {} should return efficiency",
                size
            );
        }
    }

    #[test]
    fn qt_drive_efficiency_returns_none_for_invalid_sizes() {
        let mut ship = test_cargo_ship();

        ship.qt_drive_size = 0;
        assert!(ship.qt_drive_efficiency().is_none());

        ship.qt_drive_size = 4;
        assert!(ship.qt_drive_efficiency().is_none());
    }
}
