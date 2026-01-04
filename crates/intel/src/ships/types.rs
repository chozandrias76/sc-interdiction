//! Cargo ship data for target estimation.

use serde::{Deserialize, Serialize};

/// Ship role indicating what types of cargo it can carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipRole {
    /// Standard cargo ships - can carry refined materials only
    Cargo,
    /// Combat vessels - fighters and military ships
    Combat,
    /// Can extract ore AND carry raw ore to refineries
    Mining,
    /// Can salvage and carry RMC/CMATs to refineries
    Salvage,
    /// Cargo transport and hauling
    Transport,
    /// Exploration and scanning vessels
    Exploration,
    /// Support ships - refueling, repair, medical
    Support,
}

/// A cargo ship with relevant stats.
#[derive(Debug, Clone, Serialize)]
pub struct CargoShip {
    pub name: String,
    pub manufacturer: String,
    pub cargo_scu: u32,
    pub crew_size: u8,
    /// Ship combat difficulty for interdictors (1-10).
    /// 1 = easy kill (no weapons, slow, fragile)
    /// 5 = moderate (some weapons, tanky, or fast)
    /// 10 = very difficult (heavy weapons, fighter escort, etc.)
    pub threat_level: u8,
    /// Typical value of the ship itself (purchase price).
    pub ship_value_uec: u64,
    /// Whether this ship requires a station with external freight elevators (Hull series).
    pub requires_freight_elevator: bool,
    /// Quantum fuel tank capacity (units).
    pub quantum_fuel_capacity: f64,
    /// Hydrogen fuel tank capacity (units).
    pub hydrogen_fuel_capacity: f64,
    /// Quantum drive size class (1=S1/small, 2=S2/medium, 3=S3/large).
    pub qt_drive_size: u8,
    /// Ship role (Cargo, Mining, or Salvage).
    pub role: ShipRole,
    /// Mining/salvage capacity in SCU (for mining and salvage ships).
    pub mining_capacity_scu: Option<u32>,
    /// Ship mass in kilograms (for salvage yield calculation).
    pub mass_kg: Option<f64>,
}

impl CargoShip {
    /// Calculate salvage value breakdown for this ship.
    ///
    /// Returns (component_value, hull_salvage_min, hull_salvage_max, total_min, total_max)
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
    pub fn interdiction_value(&self, cargo_value: f64) -> f64 {
        let threat_factor = self.threat_level.max(1) as f64;
        let crew_factor = 1.0 + (self.crew_size.saturating_sub(1) as f64 * 0.2); // Each extra crew adds 20% difficulty
        cargo_value / (threat_factor * crew_factor)
    }

    /// Get the quantum drive efficiency for this ship.
    pub fn qt_drive_efficiency(&self) -> Option<&'static route_graph::QtDriveEfficiency> {
        route_graph::efficiency_for_size(self.qt_drive_size)
    }

    /// Check if this ship can complete a route of given distance (Mkm).
    ///
    /// Returns (`can_complete`, `fuel_required`, `fuel_remaining`).
    pub fn can_complete_route(&self, distance_mkm: f64) -> (bool, f64, f64) {
        if let Some(efficiency) = self.qt_drive_efficiency() {
            route_graph::can_complete_route(distance_mkm, self.quantum_fuel_capacity, efficiency)
        } else {
            (false, 0.0, 0.0)
        }
    }

    /// Calculate maximum quantum travel range in Mkm.
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
    /// Total value of cargo carried by target
    pub cargo_value: f64,
    /// Estimated value of recoverable cargo after interdiction
    pub recoverable_cargo: f64,
    /// Estimated value of salvageable ship components
    pub salvage_value: f64,
    /// Total estimated loot value
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
    pub fn calculate_disable(cargo_value: f64, ship: &CargoShip) -> Self {
        Self::calculate(cargo_value, ship, 0.0)
    }

    /// Calculate loot estimation assuming moderate destruction.
    pub fn calculate_moderate(cargo_value: f64, ship: &CargoShip) -> Self {
        Self::calculate(cargo_value, ship, 0.5)
    }

    /// Calculate loot estimation assuming complete destruction.
    pub fn calculate_destroy(cargo_value: f64, ship: &CargoShip) -> Self {
        Self::calculate(cargo_value, ship, 1.0)
    }
}

/// Estimate the value of a cargo ship based on its characteristics.
///
/// This is a rough approximation based on cargo capacity, role, and typical ship prices.
#[allow(dead_code)]
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
