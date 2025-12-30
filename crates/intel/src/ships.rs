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
    /// Quantum fuel tank capacity in units.
    pub quantum_fuel_capacity: f64,
    /// Hydrogen fuel tank capacity in units.
    pub hydrogen_fuel_capacity: f64,
    /// Quantum drive size class (1-3) for efficiency lookup.
    pub qt_drive_size: u8,
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

    /// Get the quantum drive efficiency for this ship's drive size.
    pub fn qt_drive_efficiency(&self) -> Option<&'static route_graph::QtDriveEfficiency> {
        route_graph::efficiency_for_size(self.qt_drive_size)
    }

    /// Check if this ship can complete a route of given distance.
    /// Returns (`can_complete`, `fuel_required`, `fuel_remaining`)
    pub fn can_complete_route(&self, distance_mkm: f64) -> (bool, f64, f64) {
        if let Some(efficiency) = self.qt_drive_efficiency() {
            route_graph::can_complete_route(distance_mkm, self.quantum_fuel_capacity, efficiency)
        } else {
            (false, 0.0, 0.0)
        }
    }

    /// Calculate the maximum range this ship can travel on a full tank.
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
    // Small haulers - easy targets, low cargo (Size 1 QT drives)
    CargoShip {
        name: "Aurora CL",
        manufacturer: "RSI",
        cargo_scu: 6,
        crew_size: 1,
        threat_level: 1, // Paper thin, no weapons, can't run
        ship_value_uec: 45_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 583.0,
        hydrogen_fuel_capacity: 95_000.0,
        qt_drive_size: 1,
    },
    CargoShip {
        name: "Avenger Titan",
        manufacturer: "Aegis",
        cargo_scu: 8,
        crew_size: 1,
        threat_level: 4, // Actually has teeth - nose gun hurts
        ship_value_uec: 85_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 787.0,
        hydrogen_fuel_capacity: 105_000.0,
        qt_drive_size: 1,
    },
    CargoShip {
        name: "Nomad",
        manufacturer: "Consolidated Outland",
        cargo_scu: 24,
        crew_size: 1,
        threat_level: 2, // Weak weapons, slow, but tanky shields
        ship_value_uec: 95_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 120_000.0,
        qt_drive_size: 1,
    },
    CargoShip {
        name: "Cutlass Black",
        manufacturer: "Drake",
        cargo_scu: 46,
        crew_size: 2,
        threat_level: 5, // Turret + pilot guns, common pirate ship
        ship_value_uec: 150_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2500.0,
        hydrogen_fuel_capacity: 220_000.0,
        qt_drive_size: 2,
    },
    // Medium haulers (Size 2 QT drives)
    CargoShip {
        name: "Freelancer",
        manufacturer: "MISC",
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 4, // Tanky, rear turret, but blind spots
        ship_value_uec: 180_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 3500.0,
        hydrogen_fuel_capacity: 320_000.0,
        qt_drive_size: 2,
    },
    CargoShip {
        name: "Freelancer MAX",
        manufacturer: "MISC",
        cargo_scu: 120,
        crew_size: 2,
        threat_level: 3, // Traded guns for cargo - weaker than base Freelancer
        ship_value_uec: 220_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 4000.0,
        hydrogen_fuel_capacity: 360_000.0,
        qt_drive_size: 2,
    },
    CargoShip {
        name: "Constellation Taurus",
        manufacturer: "RSI",
        cargo_scu: 174,
        crew_size: 2,
        threat_level: 4, // No snub fighter, reduced turrets vs Andromeda
        ship_value_uec: 350_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 5000.0,
        hydrogen_fuel_capacity: 400_000.0,
        qt_drive_size: 2,
    },
    CargoShip {
        name: "Constellation Andromeda",
        manufacturer: "RSI",
        cargo_scu: 96,
        crew_size: 4,
        threat_level: 7, // Snub fighter + 4 turrets + missiles = dangerous
        ship_value_uec: 400_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 5000.0,
        hydrogen_fuel_capacity: 400_000.0,
        qt_drive_size: 2,
    },
    // Large haulers - high value, variable threat (Size 3 QT drives)
    CargoShip {
        name: "Caterpillar",
        manufacturer: "Drake",
        cargo_scu: 576,
        crew_size: 4,
        threat_level: 3, // Big slow barn, turrets but poor coverage, easy to kill
        ship_value_uec: 600_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 12_000.0,
        hydrogen_fuel_capacity: 800_000.0,
        qt_drive_size: 3,
    },
    CargoShip {
        name: "C2 Hercules",
        manufacturer: "Crusader",
        cargo_scu: 696,
        crew_size: 2,
        threat_level: 2, // Flying warehouse - tanky HP but weak weapons, slow
        ship_value_uec: 800_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 15_000.0,
        hydrogen_fuel_capacity: 1_200_000.0,
        qt_drive_size: 3,
    },
    CargoShip {
        name: "Hull C",
        manufacturer: "MISC",
        cargo_scu: 4608,
        crew_size: 3,
        threat_level: 1, // Completely defenseless when loaded, can't even run
        ship_value_uec: 1_200_000,
        requires_freight_elevator: true,
        quantum_fuel_capacity: 18_000.0,
        hydrogen_fuel_capacity: 1_000_000.0,
        qt_drive_size: 3,
    },
    // Industrial/Mining (sometimes haul refined) (Size 2 QT drives)
    CargoShip {
        name: "RAFT",
        manufacturer: "MISC",
        cargo_scu: 96,
        crew_size: 1,
        threat_level: 1, // No weapons at all
        ship_value_uec: 150_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2800.0,
        hydrogen_fuel_capacity: 260_000.0,
        qt_drive_size: 2,
    },
    CargoShip {
        name: "MOLE",
        manufacturer: "ARGO",
        cargo_scu: 96,
        crew_size: 4,
        threat_level: 1, // Mining lasers only, sitting duck
        ship_value_uec: 500_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 3000.0,
        hydrogen_fuel_capacity: 380_000.0,
        qt_drive_size: 2,
    },
];

/// Locations known to have freight elevator facilities for large ships like Hull C.
/// This includes orbital stations and some landing zones with external pads.
static FREIGHT_ELEVATOR_LOCATIONS: &[&str] = &[
    // Stanton orbital stations
    "station",           // Generic - catches most orbital stations
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
    "levski",            // Levski has external freight elevators
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
        hydrogen_fuel_capacity: 220_000.0,
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
            hydrogen_fuel_capacity: 220_000.0,
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
    if let Some(ship) = dockable.iter().find(|s| s.cargo_scu as f64 >= max_scu_needed) {
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
        hydrogen_fuel_capacity: 220_000.0,
        qt_drive_size: 2,
    }
}


/// Estimated loot from a successful interdiction.
/// TODO: Implement loot estimation based on cargo value and ship destruction
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootEstimate {
    pub cargo_value: f64,
    pub recoverable_cargo: f64,
    pub salvage_value: f64,
    pub total: f64,
}
