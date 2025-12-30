//! Integration tests for TargetAnalyzer
//!
//! These tests require complex API mocking and are better suited as integration tests.
//! They test the full behavior of TargetAnalyzer methods that depend on UexClient.

#![allow(unused_imports)]
#![allow(dead_code)]

use api_client::{TradeRoute, UexClient};
use intel::TargetAnalyzer;
use mockito::Server;

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

// NOTE: These tests are currently disabled because they require proper mocking
// of the composite get_trade_routes() method, which calls multiple endpoints.
//
// To properly implement these, we would need to:
// 1. Mock both /commodities_prices_all and /terminals endpoints
// 2. Provide realistic response data that matches the API schema
// 3. Handle the complex data transformation logic
//
// For now, the unit tests in targets_tests.rs provide good coverage of the
// business logic (helper functions, calculations, parsing).

#[tokio::test]
#[ignore = "Requires implementation of composite API mocking"]
async fn test_get_hot_routes_integration() {
    // This would test get_hot_routes() with full API mocking
    // Implementation pending
}

#[tokio::test]
#[ignore = "Requires implementation of composite API mocking"]
async fn test_predict_targets_at_integration() {
    // This would test predict_targets_at() with full API mocking
    // Implementation pending
}

#[tokio::test]
#[ignore = "Requires implementation of composite API mocking"]
async fn test_get_trade_runs_integration() {
    // This would test get_trade_runs() with full API mocking
    // Implementation pending
}

#[tokio::test]
#[ignore = "Requires implementation of composite API mocking"]
async fn test_get_interdiction_hotspots_integration() {
    // This would test get_interdiction_hotspots() with full API mocking
    // Implementation pending
}
