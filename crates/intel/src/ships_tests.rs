//! Tests for ship-related functionality.

use crate::ships::{estimate_ship_for_route, CargoShip, LootEstimate, CARGO_SHIPS};
use api_client::TradeRoute;

#[test]
fn test_loot_estimate_disable_scenario() {
    let ship = CargoShip {
        name: "Freelancer",
        manufacturer: "MISC",
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 288.0,
        qt_drive_size: 1,
    };

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
        name: "Caterpillar",
        manufacturer: "Drake",
        cargo_scu: 576,
        crew_size: 3,
        threat_level: 5,
        ship_value_uec: 4_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 6000.0,
        hydrogen_fuel_capacity: 1200.0,
        qt_drive_size: 2,
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
        name: "Constellation Andromeda",
        manufacturer: "RSI",
        cargo_scu: 96,
        crew_size: 4,
        threat_level: 6,
        ship_value_uec: 3_500_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 2500.0,
        hydrogen_fuel_capacity: 500.0,
        qt_drive_size: 2,
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
    let ship = CargoShip {
        name: "Cutlass Black",
        manufacturer: "Drake",
        cargo_scu: 46,
        crew_size: 2,
        threat_level: 4,
        ship_value_uec: 1_400_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 583.0,
        hydrogen_fuel_capacity: 209.0,
        qt_drive_size: 1,
    };

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
    let ship = CargoShip {
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
    };

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
    };

    // Medium ship
    let medium_ship = CargoShip {
        name: "Freelancer",
        manufacturer: "MISC",
        cargo_scu: 150,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 288.0,
        qt_drive_size: 1,
    };

    // Large ship
    let large_ship = CargoShip {
        name: "C2 Hercules",
        manufacturer: "Crusader",
        cargo_scu: 696,
        crew_size: 3,
        threat_level: 5,
        ship_value_uec: 5_000_000,
        requires_freight_elevator: true,
        quantum_fuel_capacity: 7000.0,
        hydrogen_fuel_capacity: 2000.0,
        qt_drive_size: 3,
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
    let ship = CargoShip {
        name: "Freelancer",
        manufacturer: "MISC",
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 288.0,
        qt_drive_size: 1,
    };

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
    };

    // S2 drive ship
    let s2_ship = CargoShip {
        name: "Freelancer",
        manufacturer: "MISC",
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 288.0,
        qt_drive_size: 2,
    };

    let s1_eff = s1_ship.qt_drive_efficiency();
    let s2_eff = s2_ship.qt_drive_efficiency();

    assert!(s1_eff.is_some());
    assert!(s2_eff.is_some());
}

#[test]
fn test_can_complete_route() {
    let ship = CargoShip {
        name: "Freelancer",
        manufacturer: "MISC",
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 288.0,
        qt_drive_size: 1,
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
        name: "Freelancer",
        manufacturer: "MISC",
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 1_700_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1250.0,
        hydrogen_fuel_capacity: 288.0,
        qt_drive_size: 1,
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

#[test]
fn test_estimate_ship_for_route() {
    let route = TradeRoute {
        id_commodity: 1,
        commodity_name: "Agricultural Supplies".to_string(),
        commodity_code: "AGRI".to_string(),
        id_terminal_origin: 1,
        terminal_origin_name: "Port Olisar".to_string(),
        origin_system: "Stanton".to_string(),
        id_terminal_destination: 2,
        terminal_destination_name: "Lorville".to_string(),
        destination_system: "Stanton".to_string(),
        price_origin: 1.0,
        price_destination: 1.5,
        scu_origin: 1000.0,
        scu_destination: 500.0,
        profit_per_unit: 0.5,
    };

    let ship = estimate_ship_for_route(&route);

    // Should return a valid ship
    assert!(!ship.name.is_empty());
    assert!(ship.cargo_scu > 0);

    // Ship should be able to handle the route's cargo
    assert!(ship.cargo_scu as f64 >= route.scu_origin.min(route.scu_destination));
}

#[test]
fn test_cargo_ships_list_not_empty() {
    assert!(!CARGO_SHIPS.is_empty());
    assert!(CARGO_SHIPS.len() > 10); // Should have at least 10 ships
}

#[test]
fn test_all_cargo_ships_have_valid_data() {
    for ship in CARGO_SHIPS.iter() {
        assert!(!ship.name.is_empty());
        assert!(!ship.manufacturer.is_empty());
        assert!(ship.cargo_scu > 0);
        assert!(ship.crew_size > 0);
        assert!(ship.threat_level > 0);
        assert!(ship.threat_level <= 10);
        assert!(ship.ship_value_uec > 0);
        assert!(ship.quantum_fuel_capacity > 0.0);
        assert!(ship.hydrogen_fuel_capacity > 0.0);
        assert!(ship.qt_drive_size > 0);
        assert!(ship.qt_drive_size <= 3);
    }
}
