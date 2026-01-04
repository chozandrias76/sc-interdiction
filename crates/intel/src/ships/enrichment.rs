//! Ship data enrichment - adding game logic to API data.

use super::{CargoShip, ShipRole};
use api_client::ShipModel;

/// Estimate threat level for interdiction based on ship characteristics.
///
/// Threat level (1-10):
/// - 1 = Easy kill (no weapons, slow, fragile)
/// - 5 = Moderate (some weapons, tanky, or fast)
/// - 10 = Very difficult (heavy weapons, escorts, etc.)
///
/// TODO: Build a more robust system based on ship stats and module potential.
/// Stock modules can be looked up somehow, and we will figure that out, and then
/// we can look at the potential for best modules, and what that means later.
pub fn estimate_threat_level(ship: &ShipModel) -> u8 {
    let name_lower = ship.name.to_lowercase();

    // Known combat ships - high threat
    if name_lower.contains("hornet")
        || name_lower.contains("sabre")
        || name_lower.contains("vanguard")
        || name_lower.contains("gladius")
        || name_lower.contains("arrow")
        || name_lower.contains("buccaneer")
    {
        return 8;
    }

    // Multi-crew combat ships
    if name_lower.contains("hammerhead")
        || name_lower.contains("perseus")
        || name_lower.contains("redeemer")
    {
        return 10;
    }

    // Armed haulers - moderate threat
    if name_lower.contains("cutlass")
        || name_lower.contains("freelancer")
        || name_lower.contains("constellation")
    {
        return if name_lower.contains("andromeda") {
            7
        } else {
            4
        };
    }

    // Unarmed or lightly armed haulers
    if name_lower.contains("hull") || name_lower.contains("c2") || name_lower.contains("m2") {
        return 2;
    }

    // Small ships - generally low threat
    if let Some(crew) = ship.crew.as_ref() {
        if crew.max.unwrap_or(1) == 1 {
            return 2;
        }
    }

    // Default moderate
    3
}

/// Classify ship role based on type and capabilities.
///
/// TODO: We will want a better way to infer this without string matching.
/// Use stats about the ship to infer if it's mining, salvage, or something
/// else not in the enum yet.
pub fn classify_role(ship: &ShipModel) -> ShipRole {
    let name_lower = ship.name.to_lowercase();
    let classification = ship.classification.as_ref().map(|s| s.to_lowercase());

    // Mining ships
    if name_lower.contains("prospector")
        || name_lower.contains("mole")
        || name_lower.contains("golem")
        || classification
            .as_deref()
            .is_some_and(|c| c.contains("mining"))
    {
        return ShipRole::Mining;
    }

    // Salvage ships
    if name_lower.contains("reclaimer")
        || name_lower.contains("vulture")
        || classification
            .as_deref()
            .is_some_and(|c| c.contains("salvage"))
    {
        return ShipRole::Salvage;
    }

    // Everything else is cargo (default)
    ShipRole::Cargo
}

/// Estimate quantum drive size class based on ship size/classification.
///
/// Returns:
/// - 1 = Small (S1)
/// - 2 = Medium (S2)
/// - 3 = Large (S3)
pub fn estimate_qt_drive_size(ship: &ShipModel) -> u8 {
    // Use ship size classification if available
    if let Some(size) = &ship.size {
        let size_lower = size.to_lowercase();
        return match size_lower.as_str() {
            "vehicle" | "snub" | "small" => 1,
            "large" | "capital" => 3,
            _ => 2, // Medium or unknown
        };
    }

    // Fallback to cargo capacity estimation
    let cargo = ship.cargo_capacity().unwrap_or(0);
    match cargo {
        0..=50 => 1,
        51..=200 => 2,
        _ => 3,
    }
}

/// Check if ship requires freight elevator based on design.
pub fn requires_freight_elevator(ship: &ShipModel) -> bool {
    let name_lower = ship.name.to_lowercase();

    // Hull series external cargo
    name_lower.contains("hull c") || name_lower.contains("hull d") || name_lower.contains("hull e")
}

/// Estimate fuel capacities from ship size if not in API.
///
/// TODO: This implementation sucks, but does return the expected type.
/// Get this information from the real source or fail if it's not possible.
pub fn estimate_fuel_capacity(ship: &ShipModel) -> (f64, f64) {
    let cargo_scu = ship.cargo_capacity().unwrap_or(0);

    // If API provides it, use that
    let quantum_fuel = if let Some(fuel) = ship.quantum_fuel_capacity() {
        fuel as f64
    } else {
        // Estimate based on size
        match cargo_scu {
            0..=20 => 700.0,
            21..=50 => 1250.0,
            51..=150 => 2500.0,
            151..=300 => 5000.0,
            301..=700 => 10000.0,
            _ => 15000.0,
        }
    };

    let hydrogen_fuel = if let Some(fuel) = ship.hydrogen_fuel_capacity() {
        fuel as f64
    } else {
        match cargo_scu {
            0..=20 => 120.0,
            21..=50 => 200.0,
            51..=150 => 450.0,
            151..=300 => 800.0,
            301..=700 => 1800.0,
            _ => 2500.0,
        }
    };

    (quantum_fuel, hydrogen_fuel)
}

/// Convert API ship model to internal cargo ship representation.
///
/// Only converts ships that are:
/// - Flight-ready (`production_status` == "flight-ready")
/// - Have cargo capacity > 0 SCU
///
/// Returns `None` for ships that don't meet these criteria.
pub fn from_api_ship(ship: &ShipModel) -> Option<CargoShip> {
    // Only include flight-ready ships (live in PTU)
    if ship.production_status.as_deref() != Some("flight-ready") {
        return None;
    }

    let cargo_scu = ship.cargo_capacity()? as u32;

    // Skip ships with no cargo capacity
    if cargo_scu == 0 {
        return None;
    }

    let (quantum_fuel, hydrogen_fuel) = estimate_fuel_capacity(ship);
    let crew_max = ship.crew.as_ref()?.max.unwrap_or(1);

    // Get ship mass from metrics
    let mass_kg = ship.metrics.as_ref().and_then(|m| m.mass);

    Some(CargoShip {
        name: ship.name.clone(),
        manufacturer: ship.manufacturer_name().unwrap_or("Unknown").to_string(),
        cargo_scu,
        crew_size: crew_max.min(255) as u8,
        threat_level: estimate_threat_level(ship),
        ship_value_uec: ship.price.unwrap_or(ship.pledge_price.unwrap_or(100000.0)) as u64,
        requires_freight_elevator: requires_freight_elevator(ship),
        quantum_fuel_capacity: quantum_fuel,
        hydrogen_fuel_capacity: hydrogen_fuel,
        qt_drive_size: estimate_qt_drive_size(ship),
        role: classify_role(ship),
        mining_capacity_scu: if classify_role(ship) != ShipRole::Cargo {
            Some(cargo_scu)
        } else {
            None
        },
        mass_kg,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_ship(name: &str) -> ShipModel {
        ShipModel {
            id: "test-id".to_string(),
            name: name.to_string(),
            slug: name.to_lowercase(),
            description: None,
            manufacturer: None,
            metrics: None,
            crew: None,
            speeds: None,
            focus: None,
            production_status: Some("flight-ready".to_string()),
            classification: None,
            size: None,
            price: None,
            pledge_price: None,
            rsi_id: None,
        }
    }

    #[test]
    fn test_threat_level_estimation() {
        let mut ship = create_test_ship("Cutlass Black");
        assert_eq!(estimate_threat_level(&ship), 4);

        ship.name = "Hull C".to_string();
        assert_eq!(estimate_threat_level(&ship), 2);

        ship.name = "Hammerhead".to_string();
        assert_eq!(estimate_threat_level(&ship), 10);
    }

    #[test]
    fn test_role_classification() {
        let mut ship = create_test_ship("Prospector");
        ship.classification = None;
        assert_eq!(classify_role(&ship), ShipRole::Mining);

        ship.name = "Freelancer MAX".to_string();
        assert_eq!(classify_role(&ship), ShipRole::Cargo);

        ship.name = "Vulture".to_string();
        assert_eq!(classify_role(&ship), ShipRole::Salvage);
    }

    #[test]
    fn test_from_api_ship_filters_non_flight_ready() {
        let mut ship = create_test_ship("Test Ship");
        ship.production_status = Some("in-concept".to_string());
        ship.metrics = Some(api_client::ShipMetrics {
            cargo: Some(100.0),
            beam: None,
            height: None,
            length: None,
            mass: None,
            size: None,
            hydrogen_fuel_tank_size: None,
            quantum_fuel_tank_size: None,
        });
        ship.crew = Some(api_client::CrewInfo {
            min: Some(1),
            max: Some(2),
        });

        // Non-flight-ready ships should be filtered out
        assert!(from_api_ship(&ship).is_none());

        // Flight-ready ships should be included
        ship.production_status = Some("flight-ready".to_string());
        assert!(from_api_ship(&ship).is_some());
    }
}
