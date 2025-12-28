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
    /// Ship combat capability (0 = defenseless, 10 = dangerous).
    pub threat_level: u8,
    /// Typical value of the ship itself.
    pub ship_value_uec: u64,
}

/// Common cargo ships in Star Citizen.
pub static CARGO_SHIPS: &[CargoShip] = &[
    // Small haulers
    CargoShip {
        name: "Aurora CL",
        manufacturer: "RSI",
        cargo_scu: 6,
        crew_size: 1,
        threat_level: 1,
        ship_value_uec: 45_000,
    },
    CargoShip {
        name: "Avenger Titan",
        manufacturer: "Aegis",
        cargo_scu: 8,
        crew_size: 1,
        threat_level: 4,
        ship_value_uec: 85_000,
    },
    CargoShip {
        name: "Nomad",
        manufacturer: "Consolidated Outland",
        cargo_scu: 24,
        crew_size: 1,
        threat_level: 2,
        ship_value_uec: 95_000,
    },
    CargoShip {
        name: "Cutlass Black",
        manufacturer: "Drake",
        cargo_scu: 46,
        crew_size: 2,
        threat_level: 5,
        ship_value_uec: 150_000,
    },
    // Medium haulers
    CargoShip {
        name: "Freelancer",
        manufacturer: "MISC",
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 4,
        ship_value_uec: 180_000,
    },
    CargoShip {
        name: "Freelancer MAX",
        manufacturer: "MISC",
        cargo_scu: 120,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 220_000,
    },
    CargoShip {
        name: "Constellation Taurus",
        manufacturer: "RSI",
        cargo_scu: 174,
        crew_size: 2,
        threat_level: 5,
        ship_value_uec: 350_000,
    },
    CargoShip {
        name: "Constellation Andromeda",
        manufacturer: "RSI",
        cargo_scu: 96,
        crew_size: 4,
        threat_level: 7,
        ship_value_uec: 400_000,
    },
    // Large haulers
    CargoShip {
        name: "Caterpillar",
        manufacturer: "Drake",
        cargo_scu: 576,
        crew_size: 4,
        threat_level: 4,
        ship_value_uec: 600_000,
    },
    CargoShip {
        name: "C2 Hercules",
        manufacturer: "Crusader",
        cargo_scu: 696,
        crew_size: 2,
        threat_level: 5,
        ship_value_uec: 800_000,
    },
    CargoShip {
        name: "Hull C",
        manufacturer: "MISC",
        cargo_scu: 4608,
        crew_size: 3,
        threat_level: 1,
        ship_value_uec: 1_200_000,
    },
    // Industrial/Mining (sometimes haul refined)
    CargoShip {
        name: "RAFT",
        manufacturer: "MISC",
        cargo_scu: 96,
        crew_size: 1,
        threat_level: 1,
        ship_value_uec: 150_000,
    },
    CargoShip {
        name: "MOLE",
        manufacturer: "ARGO",
        cargo_scu: 96,
        crew_size: 4,
        threat_level: 1,
        ship_value_uec: 500_000,
    },
];

/// Estimate likely ship for a trade route based on cargo volume.
pub fn estimate_ship_for_route(route: &TradeRoute) -> CargoShip {
    let scu_needed = route.max_profitable_scu();

    // Find smallest ship that can carry the cargo
    let mut suitable: Vec<_> = CARGO_SHIPS
        .iter()
        .filter(|s| s.cargo_scu as f64 >= scu_needed * 0.5)
        .cloned()
        .collect();

    // Sort by cargo capacity
    suitable.sort_by_key(|s| s.cargo_scu);

    // Return the most likely ship (smallest that fits, but not too small)
    suitable.first().cloned().unwrap_or_else(|| CargoShip {
        name: "Unknown",
        manufacturer: "Unknown",
        cargo_scu: scu_needed as u32,
        crew_size: 1,
        threat_level: 3,
        ship_value_uec: 100_000,
    })
}

/// Get ship by name.
pub fn get_ship(name: &str) -> Option<&'static CargoShip> {
    CARGO_SHIPS.iter().find(|s| s.name.eq_ignore_ascii_case(name))
}

/// Calculate potential loot value from a target.
pub fn estimate_loot_value(ship: &CargoShip, cargo_value: f64) -> LootEstimate {
    LootEstimate {
        cargo_value,
        // Assume 10-30% of cargo can be looted before destruction
        recoverable_cargo: cargo_value * 0.2,
        // Ship components/salvage value
        salvage_value: ship.ship_value_uec as f64 * 0.1,
        total: cargo_value * 0.2 + ship.ship_value_uec as f64 * 0.1,
    }
}

/// Estimated loot from a successful interdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootEstimate {
    pub cargo_value: f64,
    pub recoverable_cargo: f64,
    pub salvage_value: f64,
    pub total: f64,
}
