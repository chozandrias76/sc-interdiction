//! Unit tests for `TargetAnalyzer` helper functions and business logic

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::too_many_arguments)]

use super::*;
use api_client::TradeRoute;

/// Helper to create a mock trade route for testing
fn mock_trade_route(
    commodity: &str,
    commodity_code: &str,
    origin: &str,
    destination: &str,
    origin_system: &str,
    dest_system: &str,
    profit_per_unit: f64,
    scu_origin: f64,
    price_origin: f64,
) -> TradeRoute {
    TradeRoute {
        id_commodity: 1,
        commodity_name: commodity.to_string(),
        commodity_code: commodity_code.to_string(),
        id_terminal_origin: 1,
        terminal_origin_name: format!("{} Terminal ({})", origin, origin_system),
        origin_system: origin_system.to_string(),
        id_terminal_destination: 2,
        terminal_destination_name: format!("{} Terminal ({})", destination, dest_system),
        destination_system: dest_system.to_string(),
        price_origin,
        price_destination: price_origin + profit_per_unit,
        profit_per_unit,
        scu_origin,
        scu_destination: 1000.0,
    }
}

// ===== System Name Extraction Tests =====

#[test]
fn test_extract_system_basic() {
    assert_eq!(extract_system("Port Olisar (Stanton)"), "Stanton");
    assert_eq!(extract_system("Levski (Stanton)"), "Stanton");
    assert_eq!(extract_system("Ruin Station (Pyro)"), "Pyro");
}

#[test]
fn test_extract_system_with_arrow() {
    assert_eq!(extract_system("Terminal (Stanton > ArcCorp)"), "Stanton");
    assert_eq!(extract_system("Gateway (Pyro > Stanton)"), "Pyro");
}

#[test]
fn test_extract_system_complex() {
    // Nested parens - should use the last paren group with arrow
    assert_eq!(
        extract_system("Stanton Gateway (Pyro) (Pyro > Stanton Gateway)"),
        "Pyro"
    );

    // Full terminal name format
    assert_eq!(
        extract_system("Commodity Shop - Admin - ARC-L1 (Stanton > ArcCorp)"),
        "Stanton"
    );
}

#[test]
fn test_extract_system_no_system() {
    assert_eq!(extract_system("Some Location"), "Unknown");
    assert_eq!(extract_system(""), "Unknown");
}

// ===== Risk Score Calculation Tests =====

#[test]
fn test_calculate_risk_score() {
    let route = mock_trade_route(
        "Gold", "GOLD", "Origin", "Dest", "Stanton", "Stanton", 100.0, 1000.0, 50.0,
    );
    let score = calculate_risk_score(&route);

    // Profit score: (100 / 10).min(30) = 10
    // SCU score: (1000 / 100).min(20) = 10
    // Large cargo bonus: 1000 > 500, so +20
    // Total: 10 + 10 + 20 = 40
    assert_eq!(score, 40.0);
}

#[test]
fn test_calculate_risk_score_low_values() {
    let route = mock_trade_route(
        "Copper", "COPP", "Origin", "Dest", "Stanton", "Stanton", 5.0, 50.0, 10.0,
    );
    let score = calculate_risk_score(&route);

    // Low profit (5 / 10 = 0.5) + low SCU (50 / 100 = 0.5) + no large cargo bonus
    // = 0.5 + 0.5 = 1.0
    assert_eq!(score, 1.0);
}

#[test]
fn test_calculate_risk_score_capped_at_100() {
    let route = mock_trade_route(
        "Unobtanium",
        "UNO",
        "Origin",
        "Dest",
        "Stanton",
        "Stanton",
        1000.0,
        10000.0,
        500.0,
    );
    let score = calculate_risk_score(&route);

    // Profit: (1000 / 10).min(30) = 30
    // SCU: (10000 / 100).min(20) = 20
    // Large cargo: +20
    // Total: 30 + 20 + 20 = 70, then .min(100) = 70
    assert_eq!(score, 70.0);
}

#[test]
fn test_calculate_risk_score_large_cargo_threshold() {
    // Test the exact threshold (500 SCU)
    let route_at_threshold = mock_trade_route(
        "Test", "TEST", "Origin", "Dest", "Stanton", "Stanton", 10.0, 500.0, 10.0,
    );
    let score_at = calculate_risk_score(&route_at_threshold);

    let route_above_threshold = mock_trade_route(
        "Test", "TEST", "Origin", "Dest", "Stanton", "Stanton", 10.0, 501.0, 10.0,
    );
    let score_above = calculate_risk_score(&route_above_threshold);

    // At threshold should not get bonus, above threshold should
    assert!(score_above > score_at);
    // Bonus should be approximately 20 (allow for floating point precision)
    let diff = (score_above - score_at - 20.0).abs();
    assert!(
        diff < 0.01,
        "Expected difference of ~20.0, got {}",
        score_above - score_at
    );
}

// ===== Traffic Direction Tests =====

#[test]
fn test_traffic_direction_equality() {
    assert_eq!(TrafficDirection::Arriving, TrafficDirection::Arriving);
    assert_eq!(TrafficDirection::Departing, TrafficDirection::Departing);
    assert_ne!(TrafficDirection::Arriving, TrafficDirection::Departing);
}

#[test]
fn test_traffic_direction_serialization() {
    use serde_json;

    let arriving = TrafficDirection::Arriving;
    let departing = TrafficDirection::Departing;

    let arriving_json = serde_json::to_string(&arriving).unwrap();
    let departing_json = serde_json::to_string(&departing).unwrap();

    assert_eq!(arriving_json, r#""Arriving""#);
    assert_eq!(departing_json, r#""Departing""#);

    let arriving_de: TrafficDirection = serde_json::from_str(&arriving_json).unwrap();
    let departing_de: TrafficDirection = serde_json::from_str(&departing_json).unwrap();

    assert_eq!(arriving_de, TrafficDirection::Arriving);
    assert_eq!(departing_de, TrafficDirection::Departing);
}

// ===== TradeRoute Helper Tests =====

#[test]
fn test_trade_route_profit_calculation() {
    let route = mock_trade_route(
        "Gold", "GOLD", "Origin", "Dest", "Stanton", "Stanton", 10.0, 100.0, 50.0,
    );

    // Profit should be profit_per_unit * scu
    let profit_for_100 = route.profit_for_scu(100.0);
    assert_eq!(profit_for_100, 1000.0); // 10 * 100

    let profit_for_50 = route.profit_for_scu(50.0);
    assert_eq!(profit_for_50, 500.0); // 10 * 50
}

#[test]
fn test_trade_route_max_profitable_scu() {
    let route = mock_trade_route(
        "Gold", "GOLD", "Origin", "Dest", "Stanton", "Stanton", 10.0, 100.0, 50.0,
    );

    // Max profitable SCU should be min of origin and destination available
    let max_scu = route.max_profitable_scu();
    assert_eq!(max_scu, 100.0); // min(100, 1000) = 100
}

// ===== Wikelo Integration Tests =====

use crate::ships::ShipRole;
use crate::wikelo::WikieloIntel;

/// Helper to create a mock cargo ship for testing
fn mock_cargo_ship() -> crate::ships::CargoShip {
    crate::ships::CargoShip {
        name: "Test Hauler".to_string(),
        manufacturer: "RSI".to_string(),
        cargo_scu: 100,
        crew_size: 1,
        threat_level: 2,
        ship_value_uec: 1_000_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 1000.0,
        hydrogen_fuel_capacity: 500.0,
        qt_drive_size: 2,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: Some(50_000.0),
    }
}

#[test]
fn test_target_prediction_without_wikelo() {
    // Without WikieloIntel configured, wikelo_flag should always be None
    let prediction = TargetPrediction {
        direction: TrafficDirection::Departing,
        commodity: "Gold".to_string(),
        likely_ship: mock_cargo_ship(),
        estimated_cargo_value: 100_000.0,
        destination: "Pyro I".to_string(),
        wikelo_flag: None,
        demand_flag: None,
    };

    assert!(
        prediction.wikelo_flag.is_none(),
        "Without WikieloIntel, wikelo_flag should be None"
    );
}

#[test]
fn test_target_prediction_departing_to_wikelo_source() {
    // Create WikieloIntel from static data
    let wikelo = WikieloIntel::from_static();

    // Pyro I is a known Wikelo source (Valakkar)
    let flag = wikelo.flag_location("Pyro I");
    assert!(
        flag.is_some(),
        "Pyro I should be flagged as a Wikelo source"
    );

    let flag = flag.unwrap();
    assert!(flag.item_count > 0, "Pyro I should have Wikelo items");

    // Simulate a departing target to Pyro I
    let prediction = TargetPrediction {
        direction: TrafficDirection::Departing,
        commodity: "Medical Supplies".to_string(),
        likely_ship: mock_cargo_ship(),
        estimated_cargo_value: 50_000.0,
        destination: "Pyro I".to_string(),
        wikelo_flag: Some(flag),
        demand_flag: None,
    };

    assert!(
        prediction.wikelo_flag.is_some(),
        "Departing to Wikelo source should have flag"
    );
    let wikelo_flag = prediction.wikelo_flag.unwrap();
    assert_eq!(wikelo_flag.location, "Pyro I");
    assert!(!wikelo_flag.top_items.is_empty());
}

#[test]
fn test_target_prediction_arriving_no_wikelo_flag() {
    // Arriving targets should never have wikelo_flag set
    // (the cargo is already on the ship, source flagging isn't useful)
    let prediction = TargetPrediction {
        direction: TrafficDirection::Arriving,
        commodity: "Gold".to_string(),
        likely_ship: mock_cargo_ship(),
        estimated_cargo_value: 100_000.0,
        destination: "Pyro I".to_string(),
        wikelo_flag: None, // Arriving targets don't get flagged
        demand_flag: None,
    };

    assert!(
        prediction.wikelo_flag.is_none(),
        "Arriving targets should not have wikelo_flag"
    );
}

#[test]
fn test_target_prediction_departing_to_non_wikelo_location() {
    // Create WikieloIntel from static data
    let wikelo = WikieloIntel::from_static();

    // "Random Station" is not a Wikelo source
    let flag = wikelo.flag_location("Random Station That Doesn't Exist");
    assert!(flag.is_none(), "Non-Wikelo locations should not be flagged");

    // Departing target to non-Wikelo location should have no flag
    let prediction = TargetPrediction {
        direction: TrafficDirection::Departing,
        commodity: "Gold".to_string(),
        likely_ship: mock_cargo_ship(),
        estimated_cargo_value: 100_000.0,
        destination: "Random Station".to_string(),
        wikelo_flag: None,
        demand_flag: None,
    };

    assert!(
        prediction.wikelo_flag.is_none(),
        "Departing to non-Wikelo location should not have flag"
    );
}

#[test]
fn test_wikelo_flag_has_item_details() {
    // Verify that wikelo flags include useful item information
    let wikelo = WikieloIntel::from_static();

    // Pyro I is known to have Valakkar items
    let flag = wikelo
        .flag_location("Pyro I")
        .expect("Pyro I should be a Wikelo source");

    // Flag should have items
    assert!(flag.item_count > 0, "Should have at least one item");
    assert!(!flag.top_items.is_empty(), "Should have top items listed");

    // Check that top_items have names
    for item in &flag.top_items {
        assert!(!item.name.is_empty(), "Item should have a name");
    }
}

// ===== Wikelo Scoring Tests (Phase 4 Plan 3) =====

#[test]
fn test_hot_route_wikelo_score() {
    // Test that routes from Wikelo sources get non-zero wikelo_score
    use std::sync::Arc;

    let wikelo = Arc::new(WikieloIntel::from_static());

    // Pyro I is a known Wikelo source
    let (score, items) = calculate_wikelo_score(&Some(wikelo), "Pyro I");

    assert!(
        score.is_some(),
        "Should have Some score when WikieloIntel is present"
    );
    let score_val = score.unwrap();
    assert!(
        score_val > 0.0,
        "Wikelo source should have positive score, got {}",
        score_val
    );
    assert!(
        score_val <= 100.0,
        "Score should be capped at 100, got {}",
        score_val
    );
    assert!(
        !items.is_empty(),
        "Wikelo source should have item names populated"
    );

    // Verify score includes base points (20) for being a Wikelo source
    assert!(
        score_val >= 20.0,
        "Score should include base 20 points for Wikelo source, got {}",
        score_val
    );
}

#[test]
fn test_hotspot_wikelo_potential() {
    // Test that hotspots at Wikelo sources get non-zero wikelo_potential
    use std::sync::Arc;

    let wikelo = Arc::new(WikieloIntel::from_static());

    // Use calculate_wikelo_score since hotspot uses same scoring
    let (potential, items) = calculate_wikelo_score(&Some(wikelo), "Pyro I");

    assert!(
        potential.is_some(),
        "Should have Some potential when WikieloIntel is present"
    );
    let potential_val = potential.unwrap();
    assert!(
        potential_val > 0.0,
        "Wikelo location should have positive potential, got {}",
        potential_val
    );
    assert!(
        !items.is_empty(),
        "Wikelo location should have item names populated"
    );
}

#[test]
fn test_wikelo_score_without_integration() {
    // Test that wikelo scoring returns None/empty when no WikieloIntel configured

    // No WikieloIntel
    let (score, items) = calculate_wikelo_score(&None, "Pyro I");

    assert!(
        score.is_none(),
        "Score should be None when no WikieloIntel configured"
    );
    assert!(
        items.is_empty(),
        "Items should be empty when no WikieloIntel configured"
    );
}

#[test]
fn test_wikelo_score_non_wikelo_location() {
    // Test that non-Wikelo locations get zero score (not None)
    use std::sync::Arc;

    let wikelo = Arc::new(WikieloIntel::from_static());

    // Random location that isn't a Wikelo source
    let (score, items) = calculate_wikelo_score(&Some(wikelo), "Random Unknown Station");

    assert!(
        score.is_some(),
        "Should have Some score even for non-Wikelo location"
    );
    assert_eq!(
        score.unwrap(),
        0.0,
        "Non-Wikelo location should have zero score"
    );
    assert!(items.is_empty(), "Non-Wikelo location should have no items");
}

// ===== LocationAggregator Tests =====

#[test]
fn test_location_aggregator_new() {
    let agg = LocationAggregator::new("Port Olisar".to_string(), "Stanton".to_string());
    assert_eq!(agg.location, "Port Olisar");
    assert_eq!(agg.system, "Stanton");
    assert_eq!(agg.total_value, 0.0);
    assert_eq!(agg.route_count, 0);
    assert!(agg.threat_levels.is_empty());
    assert!(agg.commodity_values.is_empty());
    assert!(agg.ship_counts.is_empty());
}

#[test]
fn test_location_aggregator_add_route() {
    let mut agg = LocationAggregator::new("Port Olisar".to_string(), "Stanton".to_string());
    let ship = mock_cargo_ship();

    agg.add_route("Gold", 50_000.0, &ship);

    assert_eq!(agg.total_value, 50_000.0);
    assert_eq!(agg.route_count, 1);
    assert_eq!(agg.threat_levels.len(), 1);
    assert_eq!(agg.threat_levels[0], ship.threat_level);
    assert_eq!(agg.commodity_values.get("Gold"), Some(&50_000.0));
    assert_eq!(
        agg.ship_counts.get("Test Hauler"),
        Some(&(1, ship.threat_level))
    );
}

#[test]
fn test_location_aggregator_multiple_routes() {
    let mut agg = LocationAggregator::new("Lorville".to_string(), "Stanton".to_string());

    let ship1 = crate::ships::CargoShip {
        name: "Freelancer".to_string(),
        manufacturer: "MISC".to_string(),
        cargo_scu: 66,
        crew_size: 2,
        threat_level: 3,
        ship_value_uec: 500_000,
        requires_freight_elevator: false,
        quantum_fuel_capacity: 800.0,
        hydrogen_fuel_capacity: 400.0,
        qt_drive_size: 2,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: Some(40_000.0),
    };

    let ship2 = crate::ships::CargoShip {
        name: "Caterpillar".to_string(),
        manufacturer: "Drake".to_string(),
        cargo_scu: 576,
        crew_size: 4,
        threat_level: 5,
        ship_value_uec: 2_000_000,
        requires_freight_elevator: true,
        quantum_fuel_capacity: 2000.0,
        hydrogen_fuel_capacity: 1000.0,
        qt_drive_size: 3,
        role: ShipRole::Cargo,
        mining_capacity_scu: None,
        mass_kg: Some(200_000.0),
    };

    // Add multiple routes with different commodities and ships
    agg.add_route("Gold", 100_000.0, &ship1);
    agg.add_route("Titanium", 50_000.0, &ship2);
    agg.add_route("Gold", 75_000.0, &ship1); // Same commodity, same ship

    assert_eq!(agg.total_value, 225_000.0);
    assert_eq!(agg.route_count, 3);
    assert_eq!(agg.threat_levels, vec![3, 5, 3]);

    // Gold should have accumulated values
    assert_eq!(agg.commodity_values.get("Gold"), Some(&175_000.0));
    assert_eq!(agg.commodity_values.get("Titanium"), Some(&50_000.0));

    // Ship counts should be correct
    assert_eq!(agg.ship_counts.get("Freelancer"), Some(&(2, 3)));
    assert_eq!(agg.ship_counts.get("Caterpillar"), Some(&(1, 5)));
}

#[test]
fn test_location_aggregator_into_hotspot_empty() {
    let agg = LocationAggregator::new("Empty Station".to_string(), "Pyro".to_string());
    let hotspot = agg.into_hotspot(None, vec![], None, vec![]);

    assert_eq!(hotspot.location, "Empty Station");
    assert_eq!(hotspot.system, "Pyro");
    assert_eq!(hotspot.total_cargo_value, 0.0);
    assert_eq!(hotspot.route_count, 0);
    assert_eq!(hotspot.avg_threat_level, 5.0); // Default when empty
    assert!(hotspot.top_commodities.is_empty());
    assert!(hotspot.likely_ships.is_empty());
    assert!(hotspot.wikelo_potential.is_none());
    assert!(hotspot.wikelo_items.is_empty());
}

#[test]
fn test_location_aggregator_into_hotspot_with_data() {
    let mut agg = LocationAggregator::new("Area 18".to_string(), "Stanton".to_string());
    let ship = mock_cargo_ship();

    agg.add_route("Gold", 100_000.0, &ship);
    agg.add_route("Titanium", 50_000.0, &ship);

    let wikelo_items = vec!["Valakkar Fang".to_string()];
    let hotspot = agg.into_hotspot(Some(75.0), wikelo_items.clone(), None, vec![]);

    assert_eq!(hotspot.location, "Area 18");
    assert_eq!(hotspot.system, "Stanton");
    assert_eq!(hotspot.total_cargo_value, 150_000.0);
    assert_eq!(hotspot.route_count, 2);
    assert_eq!(hotspot.wikelo_potential, Some(75.0));
    assert_eq!(hotspot.wikelo_items, wikelo_items);

    // Top commodities should be sorted by value (Gold > Titanium)
    assert_eq!(hotspot.top_commodities.len(), 2);
    assert_eq!(hotspot.top_commodities[0].name, "Gold");
    assert_eq!(hotspot.top_commodities[0].estimated_value, 100_000.0);
    assert_eq!(hotspot.top_commodities[1].name, "Titanium");
}

#[test]
fn test_location_aggregator_avg_threat_calculation() {
    let mut agg = LocationAggregator::new("Test".to_string(), "Test".to_string());

    // Create ships with different threat levels
    let low_threat = crate::ships::CargoShip {
        threat_level: 2,
        ..mock_cargo_ship()
    };
    let high_threat = crate::ships::CargoShip {
        threat_level: 8,
        name: "Big Ship".to_string(),
        ..mock_cargo_ship()
    };

    agg.add_route("A", 1.0, &low_threat);
    agg.add_route("B", 1.0, &high_threat);
    agg.add_route("C", 1.0, &low_threat);

    let hotspot = agg.into_hotspot(None, vec![], None, vec![]);

    // Average of [2, 8, 2] = 4.0
    assert!((hotspot.avg_threat_level - 4.0).abs() < 0.01);
}

#[test]
fn test_location_aggregator_top_commodities_sorted() {
    let mut agg = LocationAggregator::new("Test".to_string(), "Test".to_string());
    let ship = mock_cargo_ship();

    // Add 6 commodities with different values
    agg.add_route("Bronze", 10.0, &ship);
    agg.add_route("Silver", 50.0, &ship);
    agg.add_route("Gold", 100.0, &ship);
    agg.add_route("Platinum", 200.0, &ship);
    agg.add_route("Diamond", 500.0, &ship);
    agg.add_route("Unobtanium", 1000.0, &ship);

    let hotspot = agg.into_hotspot(None, vec![], None, vec![]);

    // Should only have top 5, sorted by value descending
    assert_eq!(hotspot.top_commodities.len(), 5);
    assert_eq!(hotspot.top_commodities[0].name, "Unobtanium");
    assert_eq!(hotspot.top_commodities[1].name, "Diamond");
    assert_eq!(hotspot.top_commodities[2].name, "Platinum");
    assert_eq!(hotspot.top_commodities[3].name, "Gold");
    assert_eq!(hotspot.top_commodities[4].name, "Silver");
    // Bronze should be excluded (only top 5)
}

#[test]
fn test_location_aggregator_likely_ships_sorted() {
    let mut agg = LocationAggregator::new("Test".to_string(), "Test".to_string());

    // Create different ships
    let freelancer = crate::ships::CargoShip {
        name: "Freelancer".to_string(),
        threat_level: 3,
        ..mock_cargo_ship()
    };
    let caterpillar = crate::ships::CargoShip {
        name: "Caterpillar".to_string(),
        threat_level: 5,
        ..mock_cargo_ship()
    };
    let hull_c = crate::ships::CargoShip {
        name: "Hull C".to_string(),
        threat_level: 2,
        ..mock_cargo_ship()
    };

    // Add routes with different ship frequencies
    agg.add_route("A", 1.0, &freelancer);
    agg.add_route("B", 1.0, &freelancer);
    agg.add_route("C", 1.0, &freelancer); // 3 times
    agg.add_route("D", 1.0, &caterpillar);
    agg.add_route("E", 1.0, &caterpillar); // 2 times
    agg.add_route("F", 1.0, &hull_c); // 1 time

    let hotspot = agg.into_hotspot(None, vec![], None, vec![]);

    // Should be sorted by count descending
    assert_eq!(hotspot.likely_ships.len(), 3);
    assert_eq!(hotspot.likely_ships[0].ship_name, "Freelancer");
    assert_eq!(hotspot.likely_ships[0].count, 3);
    assert_eq!(hotspot.likely_ships[1].ship_name, "Caterpillar");
    assert_eq!(hotspot.likely_ships[1].count, 2);
    assert_eq!(hotspot.likely_ships[2].ship_name, "Hull C");
    assert_eq!(hotspot.likely_ships[2].count, 1);
}

// ===== Suggested Position Tests =====

#[test]
fn test_suggested_position_low_risk() {
    let mut agg = LocationAggregator::new("Test".to_string(), "Test".to_string());
    let ship = crate::ships::CargoShip {
        threat_level: 1,
        ..mock_cargo_ship()
    };

    agg.add_route("A", 1.0, &ship);
    agg.add_route("B", 1.0, &ship);

    let hotspot = agg.into_hotspot(None, vec![], None, vec![]);
    // Average threat 1.0 < 3.0
    assert_eq!(
        hotspot.suggested_position,
        "Low risk - solo interdiction viable"
    );
}

#[test]
fn test_suggested_position_medium_risk() {
    let mut agg = LocationAggregator::new("Test".to_string(), "Test".to_string());
    let ship = crate::ships::CargoShip {
        threat_level: 4,
        ..mock_cargo_ship()
    };

    agg.add_route("A", 1.0, &ship);

    let hotspot = agg.into_hotspot(None, vec![], None, vec![]);
    // Average threat 4.0 >= 3.0 and < 6.0
    assert_eq!(hotspot.suggested_position, "Medium risk - wing recommended");
}

#[test]
fn test_suggested_position_high_risk() {
    let mut agg = LocationAggregator::new("Test".to_string(), "Test".to_string());
    let ship = crate::ships::CargoShip {
        threat_level: 8,
        ..mock_cargo_ship()
    };

    agg.add_route("A", 1.0, &ship);

    let hotspot = agg.into_hotspot(None, vec![], None, vec![]);
    // Average threat 8.0 >= 6.0
    assert_eq!(
        hotspot.suggested_position,
        "High risk - multi-ship crew required"
    );
}

// ===== Struct Serialization Tests =====

#[test]
fn test_hot_route_serialization() {
    let hot_route = HotRoute {
        commodity: "Gold".to_string(),
        commodity_code: "GOLD".to_string(),
        origin: "Port Olisar".to_string(),
        destination: "Area 18".to_string(),
        origin_system: Some("Stanton".to_string()),
        destination_system: Some("Stanton".to_string()),
        profit_per_scu: 15.5,
        available_scu: 500.0,
        likely_ship: mock_cargo_ship(),
        estimated_haul_value: 7750.0,
        risk_score: 45.0,
        distance_mkm: 12.5,
        fuel_sufficient: true,
        fuel_required: 250.0,
        wikelo_score: Some(30.0),
        wikelo_items: vec!["Valakkar Fang".to_string()],
        demand_score: None,
        demand_contracts: vec![],
    };

    let json = serde_json::to_string(&hot_route).unwrap();
    assert!(json.contains("\"commodity\":\"Gold\""));
    assert!(json.contains("\"profit_per_scu\":15.5"));
    assert!(json.contains("\"fuel_sufficient\":true"));
}

#[test]
fn test_trade_run_serialization() {
    let trade_run = TradeRun {
        outbound: RouteLeg {
            commodity: "Gold".to_string(),
            origin: "Port Olisar".to_string(),
            destination: "Lorville".to_string(),
            profit_per_scu: 10.0,
            cargo_value: 50_000.0,
            distance_mkm: 8.0,
        },
        return_leg: Some(RouteLeg {
            commodity: "Titanium".to_string(),
            origin: "Lorville".to_string(),
            destination: "Port Olisar".to_string(),
            profit_per_scu: 5.0,
            cargo_value: 25_000.0,
            distance_mkm: 8.0,
        }),
        likely_ship: mock_cargo_ship(),
        total_profit: 15_000.0,
        has_return_cargo: true,
        total_distance_mkm: 16.0,
        fuel_sufficient: true,
    };

    let json = serde_json::to_string(&trade_run).unwrap();
    assert!(json.contains("\"total_profit\":15000.0"));
    assert!(json.contains("\"has_return_cargo\":true"));
}

#[test]
fn test_route_leg_fields() {
    let leg = RouteLeg {
        commodity: "Medical Supplies".to_string(),
        origin: "New Babbage".to_string(),
        destination: "Ruin Station".to_string(),
        profit_per_scu: 25.0,
        cargo_value: 125_000.0,
        distance_mkm: 45.0,
    };

    assert_eq!(leg.commodity, "Medical Supplies");
    assert_eq!(leg.origin, "New Babbage");
    assert_eq!(leg.destination, "Ruin Station");
    assert_eq!(leg.profit_per_scu, 25.0);
    assert_eq!(leg.cargo_value, 125_000.0);
    assert_eq!(leg.distance_mkm, 45.0);
}

#[test]
fn test_interdiction_hotspot_serialization() {
    let hotspot = InterdictionHotspot {
        location: "Crusader".to_string(),
        system: "Stanton".to_string(),
        total_cargo_value: 1_000_000.0,
        route_count: 50,
        avg_threat_level: 4.5,
        top_commodities: vec![CommodityValue {
            name: "Gold".to_string(),
            estimated_value: 500_000.0,
        }],
        likely_ships: vec![ShipFrequency {
            ship_name: "Freelancer".to_string(),
            count: 25,
            threat_level: 3,
        }],
        suggested_position: "Medium risk - wing recommended".to_string(),
        wikelo_potential: Some(60.0),
        wikelo_items: vec!["Kopion Horn".to_string()],
        demand_score: None,
        demand_contracts: vec![],
    };

    let json = serde_json::to_string(&hotspot).unwrap();
    assert!(json.contains("\"location\":\"Crusader\""));
    assert!(json.contains("\"route_count\":50"));
    assert!(json.contains("\"wikelo_potential\":60.0"));
}

#[test]
fn test_commodity_value_struct() {
    let cv = CommodityValue {
        name: "Laranite".to_string(),
        estimated_value: 250_000.0,
    };

    assert_eq!(cv.name, "Laranite");
    assert_eq!(cv.estimated_value, 250_000.0);

    let json = serde_json::to_string(&cv).unwrap();
    assert!(json.contains("Laranite"));
    assert!(json.contains("250000"));
}

#[test]
fn test_ship_frequency_struct() {
    let sf = ShipFrequency {
        ship_name: "Caterpillar".to_string(),
        count: 15,
        threat_level: 5,
    };

    assert_eq!(sf.ship_name, "Caterpillar");
    assert_eq!(sf.count, 15);
    assert_eq!(sf.threat_level, 5);

    let json = serde_json::to_string(&sf).unwrap();
    assert!(json.contains("Caterpillar"));
    assert!(json.contains("15"));
}

// ===== Edge Case Tests (Phase 6 Plan 2) =====

#[test]
fn test_calculate_wikelo_score_no_items() {
    // Test that a location with WikieloIntel but no items returns score 0.0
    use std::sync::Arc;

    let wikelo = Arc::new(WikieloIntel::from_static());

    // A location that exists but has no Wikelo items
    let (score, items) = calculate_wikelo_score(&Some(wikelo), "Area 18");

    assert!(
        score.is_some(),
        "Should return Some score even for location with no items"
    );
    assert_eq!(
        score.unwrap(),
        0.0,
        "Location with no Wikelo items should have score 0.0"
    );
    assert!(
        items.is_empty(),
        "Location with no Wikelo items should have empty vec"
    );
}

#[test]
fn test_extract_system_with_missing_system() {
    // Test terminal string without proper system suffix
    assert_eq!(
        extract_system("Some Terminal"),
        "Unknown",
        "Terminal without parens should return Unknown"
    );

    assert_eq!(
        extract_system("Terminal ()"),
        "",
        "Terminal with empty parens should return empty string"
    );

    assert_eq!(
        extract_system("Terminal (OnlySystem)"),
        "OnlySystem",
        "Terminal with parens but no arrow should extract content"
    );
}

#[test]
fn test_location_aggregator_into_hotspot_with_no_routes() {
    // Test that an empty LocationAggregator produces valid hotspot
    let agg = LocationAggregator::new("Empty".to_string(), "Stanton".to_string());
    let hotspot = agg.into_hotspot(None, vec![], None, vec![]);

    assert_eq!(hotspot.location, "Empty");
    assert_eq!(hotspot.route_count, 0);
    assert_eq!(hotspot.total_cargo_value, 0.0);
    // avg_threat should default to 5.0 when no routes
    assert_eq!(hotspot.avg_threat_level, 5.0);
    assert!(hotspot.top_commodities.is_empty());
    assert!(hotspot.likely_ships.is_empty());
}

#[test]
fn test_zero_profit_route_risk_score() {
    // Test risk score calculation with zero profit route
    let route = mock_trade_route(
        "Test", "TEST", "Origin", "Dest", "Stanton", "Stanton", 0.0, 100.0, 10.0,
    );
    let score = calculate_risk_score(&route);

    // Zero profit should result in low score
    // Profit: 0/10 = 0, SCU: 100/100 = 1.0, no large cargo bonus
    assert_eq!(score, 1.0);
}

#[test]
fn test_negative_profit_route_risk_score() {
    // Test risk score calculation with negative profit route
    let route = mock_trade_route(
        "Test", "TEST", "Origin", "Dest", "Stanton", "Stanton", -5.0, 100.0, 10.0,
    );
    let score = calculate_risk_score(&route);

    // Negative profit should still calculate (though such routes would be filtered upstream)
    // Profit: -5/10 = -0.5, but .min(30) caps positive only
    // SCU: 100/100 = 1.0
    // Score: -0.5 + 1.0 = 0.5
    assert!(
        score < 2.0,
        "Negative profit routes should have very low risk scores"
    );
}

// ===== Demand Scoring Tests (Phase 11 Plan 1) =====

#[test]
fn test_demand_score_none_without_wikelo() {
    // No WikieloIntel configured
    let (score, contracts) = calculate_demand_score(&None, "Wikelo Emporium Dasi");

    assert!(
        score.is_none(),
        "Score should be None when no WikieloIntel configured"
    );
    assert!(
        contracts.is_empty(),
        "Contracts should be empty when no WikieloIntel configured"
    );
}

#[test]
fn test_demand_score_zero_at_non_demand_location() {
    use std::sync::Arc;

    let wikelo = Arc::new(WikieloIntel::from_static());

    // Random location with no contracts
    let (score, contracts) = calculate_demand_score(&Some(wikelo), "Random Unknown Station");

    assert!(
        score.is_some(),
        "Should have Some score even for non-demand location"
    );
    assert_eq!(
        score.unwrap(),
        0.0,
        "Non-demand location should have zero score"
    );
    assert!(
        contracts.is_empty(),
        "Non-demand location should have no contracts"
    );
}

#[test]
fn test_demand_score_positive_at_wikelo_station() {
    use std::sync::Arc;

    let wikelo = Arc::new(WikieloIntel::from_static());

    // Wikelo Emporium Dasi is a known turn-in location
    let (score, contracts) = calculate_demand_score(&Some(wikelo), "Wikelo Emporium Dasi");

    assert!(
        score.is_some(),
        "Should have Some score when WikieloIntel is present"
    );
    let score_val = score.unwrap();
    assert!(
        score_val > 0.0,
        "Wikelo station should have positive demand score, got {}",
        score_val
    );
    assert!(
        score_val <= 100.0,
        "Score should be capped at 100, got {}",
        score_val
    );
    assert!(
        !contracts.is_empty(),
        "Wikelo station should have contract names populated"
    );

    // Score should include base 20 points for having contracts
    assert!(
        score_val >= 20.0,
        "Score should include base 20 points for contracts, got {}",
        score_val
    );
}

#[test]
fn test_demand_score_capped_at_100() {
    use std::sync::Arc;

    let wikelo = Arc::new(WikieloIntel::from_static());

    // Test all known Wikelo turn-in locations to ensure cap
    for location in &[
        "Wikelo Emporium Dasi",
        "Wikelo Emporium Kinga",
        "Wikelo Emporium Selo",
    ] {
        let (score, _) = calculate_demand_score(&Some(wikelo.clone()), location);
        if let Some(score_val) = score {
            assert!(
                score_val <= 100.0,
                "Score at {} should be capped at 100, got {}",
                location,
                score_val
            );
        }
    }
}
