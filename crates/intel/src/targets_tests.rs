//! Unit tests for TargetAnalyzer helper functions and business logic

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
