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

/// Test that different ShipRole variants produce different component values.
#[test]
fn test_role_multipliers_affect_component_value() {
    // Create base ship configuration
    let base_ship = CargoShip {
        name: "Test Ship".to_string(),
        manufacturer: "RSI".to_string(), // 1.0x multiplier
        cargo_scu: 100,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_000_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2500.0,
        hydrogen_fuel_capacity: 500.0,
        qt_drive_size: 2, // Medium ship, 18_000 base
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: Some(100_000.0),
    };

    // Create ships with different roles but same other attributes
    let cargo_ship = CargoShip {
        role: ShipRole::Cargo,
        ..base_ship.clone()
    };
    let combat_ship = CargoShip {
        role: ShipRole::Combat,
        ..base_ship.clone()
    };
    let mining_ship = CargoShip {
        role: ShipRole::Mining,
        ..base_ship.clone()
    };
    let salvage_ship = CargoShip {
        role: ShipRole::Salvage,
        ..base_ship.clone()
    };
    let transport_ship = CargoShip {
        role: ShipRole::Transport,
        ..base_ship.clone()
    };
    let exploration_ship = CargoShip {
        role: ShipRole::Exploration,
        ..base_ship.clone()
    };
    let support_ship = CargoShip {
        role: ShipRole::Support,
        ..base_ship
    };

    // Get salvage values (which use estimate_component_value internally)
    let cm_price = 100.0;
    let cargo_salvage = cargo_ship.salvage_value(cm_price);
    let combat_salvage = combat_ship.salvage_value(cm_price);
    let mining_salvage = mining_ship.salvage_value(cm_price);
    let salvage_salvage = salvage_ship.salvage_value(cm_price);
    let transport_salvage = transport_ship.salvage_value(cm_price);
    let exploration_salvage = exploration_ship.salvage_value(cm_price);
    let support_salvage = support_ship.salvage_value(cm_price);

    // Verify role multipliers produce different component values in ascending order
    assert!(cargo_salvage.component_value < transport_salvage.component_value);
    assert!(transport_salvage.component_value < support_salvage.component_value);
    assert!(support_salvage.component_value < exploration_salvage.component_value);
    assert!(exploration_salvage.component_value < salvage_salvage.component_value);
    assert!(salvage_salvage.component_value < mining_salvage.component_value);
    assert!(mining_salvage.component_value < combat_salvage.component_value);

    // Verify Combat has approximately 2x the component value of Cargo
    let ratio = combat_salvage.component_value as f64 / cargo_salvage.component_value as f64;
    assert!((ratio - 2.0).abs() < 0.1); // Should be close to 2.0
}

/// Test that role multipliers interact correctly with manufacturer multipliers.
#[test]
fn test_role_and_manufacturer_multipliers() {
    let cm_price = 100.0;

    // Drake Cargo ship (0.6 mfr * 0.7 role = 0.42x)
    let drake_cargo = CargoShip {
        name: "Drake Cargo".to_string(),
        manufacturer: "Drake".to_string(),
        cargo_scu: 100,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_000_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2500.0,
        hydrogen_fuel_capacity: 500.0,
        qt_drive_size: 2,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: Some(100_000.0),
    };

    // Origin Combat ship (1.5 mfr * 1.4 role = 2.1x)
    let origin_combat = CargoShip {
        name: "Origin Combat".to_string(),
        manufacturer: "Origin".to_string(),
        role: ShipRole::Combat,
        ..drake_cargo.clone()
    };

    let drake_salvage = drake_cargo.salvage_value(cm_price);
    let origin_salvage = origin_combat.salvage_value(cm_price);

    // Origin Combat should have much higher component value than Drake Cargo
    // due to combined effect of manufacturer and role multipliers
    let ratio = origin_salvage.component_value as f64 / drake_salvage.component_value as f64;
    assert!((ratio - 5.0).abs() < 0.3); // Should be approximately 5x
}

/// Test that role multipliers interact correctly with crew size multipliers.
#[test]
fn test_role_and_crew_size_multipliers() {
    let cm_price = 100.0;

    // Single-crew cargo ship
    let solo_cargo = CargoShip {
        name: "Solo Cargo".to_string(),
        manufacturer: "RSI".to_string(),
        cargo_scu: 50,
        crew_size: 1,
        threat_level: 2,
        ship_value_uec: 500_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 250.0,
        qt_drive_size: 1,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: Some(50_000.0),
    };

    // Multi-crew combat ship
    let multi_combat = CargoShip {
        name: "Multi Combat".to_string(),
        crew_size: 6, // More crew = more components
        role: ShipRole::Combat,
        ..solo_cargo.clone()
    };

    let solo_salvage = solo_cargo.salvage_value(cm_price);
    let multi_salvage = multi_combat.salvage_value(cm_price);

    // Multi-crew combat should have significantly higher component value
    assert!(multi_salvage.component_value > solo_salvage.component_value);

    // The difference should be substantial due to both role and crew multipliers
    let ratio = multi_salvage.component_value as f64 / solo_salvage.component_value as f64;
    assert!(ratio > 2.0); // Combat (1.4) vs Cargo (0.7) plus crew multiplier
}

/// Test Mining role with mining_capacity_scu set.
#[test]
fn test_mining_role_characteristics() {
    let mining_ship = CargoShip {
        name: "Prospector".to_string(),
        manufacturer: "MISC".to_string(),
        cargo_scu: 32,
        crew_size: 1,
        threat_level: 2,
        ship_value_uec: 1_500_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1000.0,
        hydrogen_fuel_capacity: 200.0,
        qt_drive_size: 1,
        role: ShipRole::Mining,
        mining_capacity_scu: Some(32), // Mining ships have capacity
        mass_kg: Some(40_000.0),
    };

    let cargo_ship = CargoShip {
        role: ShipRole::Cargo,
        mining_capacity_scu: None, // Cargo ships don't
        ..mining_ship.clone()
    };

    let cm_price = 100.0;
    let mining_salvage = mining_ship.salvage_value(cm_price);
    let cargo_salvage = cargo_ship.salvage_value(cm_price);

    // Mining ship should have higher component value (1.3x vs 0.7x role multiplier)
    assert!(mining_salvage.component_value > cargo_salvage.component_value);

    // Verify mining capacity is set correctly
    assert_eq!(mining_ship.mining_capacity_scu, Some(32));
    assert_eq!(cargo_ship.mining_capacity_scu, None);
}

/// Test Exploration role characteristics.
#[test]
fn test_exploration_role_characteristics() {
    let exploration_ship = CargoShip {
        name: "Terrapin".to_string(),
        manufacturer: "Anvil".to_string(), // Military-grade (1.2x)
        cargo_scu: 6,
        crew_size: 1,
        threat_level: 5,
        ship_value_uec: 2_200_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1200.0,
        hydrogen_fuel_capacity: 250.0,
        qt_drive_size: 1,
        role: ShipRole::Exploration,
        mining_capacity_scu: None,
        mass_kg: Some(35_000.0),
    };

    let cm_price = 100.0;
    let salvage = exploration_ship.salvage_value(cm_price);

    // Component value should be in expected range for small exploration ship
    // with military-grade manufacturer
    assert!(salvage.component_value > 6000);
    assert!(salvage.component_value < 7000);
}

/// Test Support role characteristics.
#[test]
fn test_support_role_characteristics() {
    let support_ship = CargoShip {
        name: "Vulcan".to_string(),
        manufacturer: "Aegis".to_string(), // Military-grade (1.2x)
        cargo_scu: 12,
        crew_size: 3,
        threat_level: 4,
        ship_value_uec: 2_500_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2000.0,
        hydrogen_fuel_capacity: 400.0,
        qt_drive_size: 2,
        role: ShipRole::Support,
        mining_capacity_scu: None,
        mass_kg: Some(80_000.0),
    };

    let cm_price = 100.0;
    let salvage = support_ship.salvage_value(cm_price);

    // Support role has 1.0x multiplier (neutral)
    assert!(salvage.component_value > 0);

    // Component value should be reasonable for a medium support ship
    assert!(salvage.component_value > 15_000);
    assert!(salvage.component_value < 30_000);
}

// Tests for ShipRegistry would require async runtime or mock data
// They are tested via integration tests instead
