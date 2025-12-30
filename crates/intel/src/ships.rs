//! Cargo ship data for target estimation.

use api_client::TradeRoute;
use serde::{Deserialize, Serialize};

/// A cargo ship with relevant stats.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct CargoShip {
    pub name: &'static str,
    pub manufacturer: &'static str,
    pub cargo_scu: u32,
    pub crew_size: u8,
    /// Ship combat difficulty for interdictors (1-10).
    /// 1 = easy kill (no weapons, slow, fragile)
    /// 5 = moderate (some weapons, tanky, or fast)
    /// 10 = very difficult (heavy weapons, fighter escort, etc.)
    pub threat_level: u8,
    /// Typical value of the ship itself.
    pub ship_value_uec: u64,
    /// Whether this ship requires a station with external freight elevators (Hull series).
    pub requires_freight_elevator: bool,
    /// Quantum fuel tank capacity (units).
    pub quantum_fuel_capacity: f64,
    /// Hydrogen fuel tank capacity (units).
    pub hydrogen_fuel_capacity: f64,
    /// Quantum drive size class (1=S1/small, 2=S2/medium, 3=S3/large).
    pub qt_drive_size: u8,
}

impl CargoShip {
    /// Calculate interdiction value score for this ship carrying given cargo value.
    /// Higher = more attractive target (better value-to-risk ratio).
    /// Formula: cargo_value / (threat_level * crew_factor)
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
    /// Returns (can_complete, fuel_required, fuel_remaining).
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

/// Common cargo ships in Star Citizen.
/// Threat levels are calibrated for interdictor perspective:
/// - How hard is this ship to kill?
/// - How likely is it to fight back effectively?
/// - Can it escape easily?
pub static CARGO_SHIPS: &[CargoShip] = &[
    // Small haulers - easy targets, low cargo
    // S1 drives: ~583-1250 QT fuel, ~100-300 H fuel
    CargoShip {
        name: "Aurora CL",
        manufacturer: "RSI",
        cargo_scu: 6,
        crew_size: 1,
        threat_level: 1,
        ship_value_uec: 45_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 583.0,
        hydrogen_fuel_capacity: 105.0,
        qt_drive_size: 1,
    },
    CargoShip {
        name: "Avenger Titan",
        manufacturer: "Aegis",
        cargo_scu: 8,
        crew_size: 1,
        threat_level: 4,
        ship_value_uec: 85_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 700.0,
        hydrogen_fuel_capacity: 140.0,
        qt_drive_size: 1,
    },
    CargoShip {
        name: "Nomad",
        manufacturer: "Consolidated Outland",
        cargo_scu: 24,
        crew_size: 1,
        threat_level: 2,
        ship_value_uec: 95_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 200.0,
        qt_drive_size: 1,
    },
    CargoShip {
        name: "Cutlass Black",
        manufacturer: "Drake",
        cargo_scu: 46,
        crew_size: 2,
        threat_level: 5,
        ship_value_uec: 150_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2500.0,
        hydrogen_fuel_capacity: 450.0,
        qt_drive_size: 2,
    },
    // Medium haulers
    // S2 drives: ~2500-5000 QT fuel, ~400-800 H fuel
    CargoShip {
        name: "Freelancer",
        manufacturer: "MISC",
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 4,
        ship_value_uec: 180_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2500.0,
        hydrogen_fuel_capacity: 500.0,
        qt_drive_size: 2,
    },
    CargoShip {
        name: "Freelancer MAX",
        manufacturer: "MISC",
        cargo_scu: 120,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 220_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 3500.0,
        hydrogen_fuel_capacity: 600.0,
        qt_drive_size: 2,
    },
    CargoShip {
        name: "Constellation Taurus",
        manufacturer: "RSI",
        cargo_scu: 174,
        crew_size: 2,
        threat_level: 4,
        ship_value_uec: 350_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 5000.0,
        hydrogen_fuel_capacity: 800.0,
        qt_drive_size: 2,
    },
    CargoShip {
        name: "Constellation Andromeda",
        manufacturer: "RSI",
        cargo_scu: 96,
        crew_size: 4,
        threat_level: 7,
        ship_value_uec: 400_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 5000.0,
        hydrogen_fuel_capacity: 800.0,
        qt_drive_size: 2,
    },
    // Large haulers - high value, variable threat
    // S3 drives: ~10000-20000 QT fuel, ~1500-2500 H fuel
    CargoShip {
        name: "Caterpillar",
        manufacturer: "Drake",
        cargo_scu: 576,
        crew_size: 4,
        threat_level: 3,
        ship_value_uec: 600_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 10000.0,
        hydrogen_fuel_capacity: 1800.0,
        qt_drive_size: 3,
    },
    CargoShip {
        name: "C2 Hercules",
        manufacturer: "Crusader",
        cargo_scu: 696,
        crew_size: 2,
        threat_level: 2,
        ship_value_uec: 800_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 10000.0,
        hydrogen_fuel_capacity: 2500.0,
        qt_drive_size: 3,
    },
    CargoShip {
        name: "Hull C",
        manufacturer: "MISC",
        cargo_scu: 4608,
        crew_size: 3,
        threat_level: 1,
        ship_value_uec: 1_200_000,
        requires_freight_elevator: true,
        quantum_fuel_capacity: 20000.0,
        hydrogen_fuel_capacity: 2000.0,
        qt_drive_size: 3,
    },
    // Industrial/Mining (sometimes haul refined)
    CargoShip {
        name: "RAFT",
        manufacturer: "MISC",
        cargo_scu: 96,
        crew_size: 1,
        threat_level: 1,
        ship_value_uec: 150_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2500.0,
        hydrogen_fuel_capacity: 400.0,
        qt_drive_size: 2,
    },
    CargoShip {
        name: "MOLE",
        manufacturer: "ARGO",
        cargo_scu: 96,
        crew_size: 4,
        threat_level: 1,
        ship_value_uec: 500_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 3500.0,
        hydrogen_fuel_capacity: 600.0,
        qt_drive_size: 2,
    },
];

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
fn has_freight_elevator(terminal_name: &str) -> bool {
    let name_lower = terminal_name.to_lowercase();
    FREIGHT_ELEVATOR_LOCATIONS
        .iter()
        .any(|loc| name_lower.contains(loc))
}

/// Estimate likely ship for a trade route based on cargo volume and docking restrictions.
pub fn estimate_ship_for_route(route: &TradeRoute) -> CargoShip {
    let scu_needed = route.max_profitable_scu();

    // Check if both endpoints support freight elevator ships
    let supports_freight_elevator = has_freight_elevator(&route.terminal_origin_name)
        && has_freight_elevator(&route.terminal_destination_name);

    // Get all ships that can dock at these terminals, sorted by capacity
    let mut dockable: Vec<_> = CARGO_SHIPS
        .iter()
        .filter(|s| !s.requires_freight_elevator || supports_freight_elevator)
        .collect();
    dockable.sort_by_key(|s| s.cargo_scu);

    // Find smallest ship that can carry the full cargo
    if let Some(ship) = dockable.iter().find(|s| s.cargo_scu as f64 >= scu_needed) {
        return (*ship).clone();
    }

    // If no ship can carry the full cargo, use the largest available ship
    // (traders will still use the best ship they have)
    if let Some(largest) = dockable.last() {
        return (*largest).clone();
    }

    // Fallback only if no ships at all (shouldn't happen)
    CargoShip {
        name: "Unknown",
        manufacturer: "Unknown",
        cargo_scu: scu_needed as u32,
        crew_size: 1,
        threat_level: 3,
        ship_value_uec: 100_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2500.0,
        hydrogen_fuel_capacity: 500.0,
        qt_drive_size: 2,
    }
}

/// Estimate a ship that can service multiple routes (for round-trips).
/// Returns the smallest ship that can handle all routes and dock at all terminals.
pub fn estimate_ship_for_routes(routes: &[&TradeRoute]) -> CargoShip {
    if routes.is_empty() {
        return CargoShip {
            name: "Unknown",
            manufacturer: "Unknown",
            cargo_scu: 0,
            crew_size: 1,
            threat_level: 3,
            ship_value_uec: 100_000,
            requires_freight_elevator: false,
            quantum_fuel_capacity: 2500.0,
            hydrogen_fuel_capacity: 500.0,
            qt_drive_size: 2,
        };
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
    let mut dockable: Vec<_> = CARGO_SHIPS
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

    CargoShip {
        name: "Unknown",
        manufacturer: "Unknown",
        cargo_scu: max_scu_needed as u32,
        crew_size: 1,
        threat_level: 3,
        ship_value_uec: 100_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2500.0,
        hydrogen_fuel_capacity: 500.0,
        qt_drive_size: 2,
    }
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
    /// - Cargo recovery rate: 70% for disable (destruction_level = 0.0), decreases with destruction
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
