//! Tests for ship-related functionality.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::indexing_slicing)]

use crate::ships::{CargoShip, LootEstimate, ShipRole};

/// Helper to create a test cargo ship.
fn make_test_ship(name: &str, cargo_scu: u32, threat_level: u8, ship_value_uec: u64) -> CargoShip {
    CargoShip {
        name: name.to_string(),
        manufacturer: "Test".to_string(),
        cargo_scu,
        crew_size: 2,
        threat_level,
        ship_value_uec,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 288.0,
        qt_drive_size: 1,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: None,
    }
}

#[test]
fn test_loot_estimate_disable_scenario() {
    let ship = make_test_ship("Freelancer", 66, 3, 1_700_000);

    let cargo_value = 100_000.0;
    let estimate = LootEstimate::calculate_disable(cargo_value, &ship);

    // For disable (0% destruction), expect 70% cargo recovery
    assert!((estimate.recoverable_cargo - 70_000.0).abs() < 1.0);

    // Salvage should be small for disable
    assert!(estimate.salvage_value > 0.0);
    assert!(estimate.salvage_value < estimate.recoverable_cargo);

    // Total should be sum of both
    assert!((estimate.total - (estimate.recoverable_cargo + estimate.salvage_value)).abs() < 0.01);

    // Cargo value should match input
    assert_eq!(estimate.cargo_value, cargo_value);
}

#[test]
fn test_loot_estimate_destroy_scenario() {
    let ship = CargoShip {
        name: "Caterpillar".to_string(),
        manufacturer: "Drake".to_string(),
        cargo_scu: 576,
        crew_size: 3,
        threat_level: 5,
        ship_value_uec: 4_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 6000.0,
        hydrogen_fuel_capacity: 1200.0,
        qt_drive_size: 2,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: Some(500_000.0),
    };

    let cargo_value = 200_000.0;
    let estimate = LootEstimate::calculate_destroy(cargo_value, &ship);

    // For destroy (100% destruction), expect ~10% cargo recovery
    assert!(estimate.recoverable_cargo < 25_000.0); // Less than 12.5%
    assert!(estimate.recoverable_cargo > 15_000.0); // More than 7.5%

    // Salvage should be higher for large destroyed ship
    assert!(estimate.salvage_value > 0.0);
    assert!(estimate.salvage_value > estimate.recoverable_cargo); // More salvage than cargo

    // Total should be sum
    assert!((estimate.total - (estimate.recoverable_cargo + estimate.salvage_value)).abs() < 0.01);
}

#[test]
fn test_loot_estimate_moderate_destruction() {
    let ship = CargoShip {
        name: "Constellation Andromeda".to_string(),
        manufacturer: "RSI".to_string(),
        cargo_scu: 96,
        crew_size: 4,
        threat_level: 6,
        ship_value_uec: 3_500_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2500.0,
        hydrogen_fuel_capacity: 500.0,
        qt_drive_size: 2,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: None,
    };

    let cargo_value = 150_000.0;
    let estimate = LootEstimate::calculate_moderate(cargo_value, &ship);

    // For moderate (50% destruction), expect ~40% cargo recovery
    assert!(estimate.recoverable_cargo > 50_000.0);
    assert!(estimate.recoverable_cargo < 70_000.0);

    // Both cargo and salvage should contribute meaningfully
    assert!(estimate.recoverable_cargo > 0.0);
    assert!(estimate.salvage_value > 0.0);
}

#[test]
fn test_loot_estimate_custom_destruction_level() {
    let ship = make_test_ship("Cutlass Black", 46, 4, 1_400_000);

    let cargo_value = 80_000.0;

    // Test 25% destruction
    let estimate_25 = LootEstimate::calculate(cargo_value, &ship, 0.25);

    // Test 75% destruction
    let estimate_75 = LootEstimate::calculate(cargo_value, &ship, 0.75);

    // Higher destruction should mean less cargo recovered
    assert!(estimate_25.recoverable_cargo > estimate_75.recoverable_cargo);

    // Higher destruction should mean more salvage value
    assert!(estimate_75.salvage_value > estimate_25.salvage_value);
}

#[test]
fn test_loot_estimate_destruction_level_clamping() {
    let ship = make_test_ship("Aurora CL", 6, 1, 45_000);

    let cargo_value = 10_000.0;

    // Test values outside [0.0, 1.0] range
    let estimate_negative = LootEstimate::calculate(cargo_value, &ship, -0.5);
    let estimate_above_one = LootEstimate::calculate(cargo_value, &ship, 2.0);
    let estimate_zero = LootEstimate::calculate(cargo_value, &ship, 0.0);
    let estimate_one = LootEstimate::calculate(cargo_value, &ship, 1.0);

    // Negative should clamp to 0.0
    assert!((estimate_negative.recoverable_cargo - estimate_zero.recoverable_cargo).abs() < 0.01);

    // Above 1.0 should clamp to 1.0
    assert!((estimate_above_one.recoverable_cargo - estimate_one.recoverable_cargo).abs() < 0.01);
}

#[test]
fn test_loot_estimate_ship_size_affects_salvage() {
    let cargo_value = 100_000.0;
    let destruction = 0.5;

    // Small ship
    let small_ship = CargoShip {
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
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: Some(25_000.0),
    };

    // Medium ship
    let medium_ship = CargoShip {
        name: "Freelancer".to_string(),
        manufacturer: "MISC".to_string(),
        cargo_scu: 150,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 288.0,
        qt_drive_size: 1,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: Some(100_000.0),
    };

    // Large ship
    let large_ship = CargoShip {
        name: "C2 Hercules".to_string(),
        manufacturer: "Crusader".to_string(),
        cargo_scu: 696,
        crew_size: 3,
        threat_level: 5,
        ship_value_uec: 5_000_000,
        requires_freight_elevator: true,
        quantum_fuel_capacity: 7000.0,
        hydrogen_fuel_capacity: 2000.0,
        qt_drive_size: 3,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: Some(800_000.0),
    };

    let small_estimate = LootEstimate::calculate(cargo_value, &small_ship, destruction);
    let medium_estimate = LootEstimate::calculate(cargo_value, &medium_ship, destruction);
    let large_estimate = LootEstimate::calculate(cargo_value, &large_ship, destruction);

    // Larger ships should have more salvage value
    assert!(medium_estimate.salvage_value > small_estimate.salvage_value);
    assert!(large_estimate.salvage_value > medium_estimate.salvage_value);
}

#[test]
fn test_interdiction_value_calculation() {
    let ship = make_test_ship("Freelancer", 66, 3, 1_700_000);

    let cargo_value = 100_000.0;
    let value = ship.interdiction_value(cargo_value);

    // Value should be positive
    assert!(value > 0.0);

    // Higher cargo value should mean higher interdiction value
    let higher_value = ship.interdiction_value(200_000.0);
    assert!(higher_value > value);

    // Ship with lower threat should have higher value
    let easy_ship = CargoShip {
        threat_level: 1,
        crew_size: 1,
        ..ship
    };
    let easy_value = easy_ship.interdiction_value(cargo_value);
    assert!(easy_value > value);
}

#[test]
fn test_qt_drive_efficiency() {
    // S1 drive ship
    let s1_ship = CargoShip {
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
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: None,
    };

    // S2 drive ship
    let s2_ship = CargoShip {
        name: "Freelancer".to_string(),
        manufacturer: "MISC".to_string(),
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 288.0,
        qt_drive_size: 2,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: None,
    };

    let s1_eff = s1_ship.qt_drive_efficiency();
    let s2_eff = s2_ship.qt_drive_efficiency();

    assert!(s1_eff.is_some());
    assert!(s2_eff.is_some());
}

#[test]
fn test_can_complete_route() {
    let ship = CargoShip {
        name: "Freelancer".to_string(),
        manufacturer: "MISC".to_string(),
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 288.0,
        qt_drive_size: 1,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: None,
    };

    // Short distance - should be able to complete
    let (can_short, _fuel_req, remaining) = ship.can_complete_route(10.0);
    assert!(can_short);
    assert!(remaining > 0.0);

    // Very long distance - should not be able to complete
    let (can_long, _, _) = ship.can_complete_route(10_000.0);
    // At 10,000 Mkm with 1250 QT fuel, this should fail
    assert!(!can_long);
}

#[test]
fn test_max_range() {
    let ship = CargoShip {
        name: "Freelancer".to_string(),
        manufacturer: "MISC".to_string(),
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 288.0,
        qt_drive_size: 1,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: None,
    };

    let max_range = ship.max_range_mkm();
    assert!(max_range > 0.0);

    // Ship with more fuel should have longer range
    let large_ship = CargoShip {
        quantum_fuel_capacity: 5000.0,
        ..ship
    };
    let large_range = large_ship.max_range_mkm();
    assert!(large_range > max_range);
}

// Tests for ShipRegistry would require async runtime or mock data
// They are tested via integration tests instead
