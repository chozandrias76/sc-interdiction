//! Integration tests for `TargetAnalyzer`
//!
//! These tests use mockito to mock the UEX API endpoints and test the full
//! behavior of `TargetAnalyzer` methods that depend on `UexClient`.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::indexing_slicing)]

mod fixtures;

use api_client::{TradeRoute, UexClient};
use intel::{ShipRegistry, TargetAnalyzer, TrafficDirection};
use mockito::ServerGuard;
use std::sync::Arc;

/// Helper to create a mock trade route for testing
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
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

/// Create a test ship registry with minimal fallback data.
fn test_registry() -> Arc<ShipRegistry> {
    // Use fallback registry which creates a minimal Aurora CL ship
    Arc::new(ShipRegistry::from_api_ships(vec![]).unwrap())
}

/// Setup mock server with terminals and commodities_prices_all endpoints.
///
/// Returns the server (to keep mocks alive) and a configured UexClient.
/// The server must be kept alive for the duration of the test.
async fn setup_mock_server() -> (ServerGuard, UexClient) {
    let mut server = mockito::Server::new_async().await;

    // Mock terminals endpoint
    server
        .mock("GET", "/terminals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(fixtures::mock_terminals_response())
        .create_async()
        .await;

    // Mock commodities_prices_all endpoint
    server
        .mock("GET", "/commodities_prices_all")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(fixtures::mock_commodities_prices_all_response())
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    (server, client)
}

#[tokio::test]
async fn test_get_hot_routes_integration() {
    let (_server, client) = setup_mock_server().await;
    let registry = test_registry();
    let analyzer = TargetAnalyzer::new(client, registry);

    let routes = analyzer.get_hot_routes(10).await.unwrap();

    // Should have profitable routes from our fixtures
    assert!(!routes.is_empty(), "Expected at least one hot route");

    // Routes should be sorted by estimated haul value (descending)
    for window in routes.windows(2) {
        assert!(
            window[0].estimated_haul_value >= window[1].estimated_haul_value,
            "Routes should be sorted by estimated haul value descending"
        );
    }

    // Check that routes contain expected commodities from fixtures
    let commodity_names: Vec<&str> = routes.iter().map(|r| r.commodity.as_str()).collect();
    // Our fixtures have Laranite, Aluminum, and Gold as profitable routes
    assert!(
        commodity_names.contains(&"Laranite")
            || commodity_names.contains(&"Aluminum")
            || commodity_names.contains(&"Gold"),
        "Expected routes to contain commodities from fixtures"
    );

    // Each route should have positive profit
    for route in &routes {
        assert!(
            route.profit_per_scu > 0.0,
            "Hot routes should have positive profit"
        );
        assert!(
            route.estimated_haul_value > 0.0,
            "Hot routes should have positive estimated haul value"
        );
    }
}

#[tokio::test]
async fn test_predict_targets_at_integration() {
    let (_server, client) = setup_mock_server().await;
    let registry = test_registry();
    let analyzer = TargetAnalyzer::new(client, registry);

    // Predict targets at Port Olisar (appears in multiple routes in our fixtures)
    let predictions = analyzer.predict_targets_at("Port Olisar").await.unwrap();

    // Should have predictions for both arriving and departing traffic
    assert!(
        !predictions.is_empty(),
        "Expected predictions for Port Olisar"
    );

    // Check that predictions have valid directions
    let has_arriving = predictions
        .iter()
        .any(|p| p.direction == TrafficDirection::Arriving);
    let has_departing = predictions
        .iter()
        .any(|p| p.direction == TrafficDirection::Departing);

    // Port Olisar is both a buy and sell location in our fixtures
    // Laranite: bought at Port Olisar (departing), sold elsewhere
    // Aluminum: sold at Port Olisar (arriving from Lorville)
    // Gold: sold at Port Olisar (arriving from Ruin Station)
    assert!(
        has_arriving || has_departing,
        "Expected at least one arriving or departing prediction"
    );

    // Validate prediction structure
    for prediction in &predictions {
        assert!(
            !prediction.commodity.is_empty(),
            "Prediction should have a commodity"
        );
        assert!(
            prediction.estimated_cargo_value > 0.0,
            "Prediction should have positive cargo value"
        );
        assert!(
            !prediction.destination.is_empty(),
            "Prediction should have a destination"
        );
    }
}

#[tokio::test]
async fn test_get_trade_runs_integration() {
    let (_server, client) = setup_mock_server().await;
    let registry = test_registry();
    let analyzer = TargetAnalyzer::new(client, registry);

    let trade_runs = analyzer.get_trade_runs(5).await.unwrap();

    // Should have trade runs from our profitable routes
    assert!(!trade_runs.is_empty(), "Expected at least one trade run");

    // Validate trade run structure
    for run in &trade_runs {
        // Outbound leg should be populated
        assert!(
            !run.outbound.commodity.is_empty(),
            "Outbound leg should have a commodity"
        );
        assert!(
            !run.outbound.origin.is_empty(),
            "Outbound leg should have an origin"
        );
        assert!(
            !run.outbound.destination.is_empty(),
            "Outbound leg should have a destination"
        );
        assert!(
            run.outbound.profit_per_scu > 0.0,
            "Outbound leg should have positive profit"
        );

        // Return leg is optional but if present should be valid
        if let Some(return_leg) = &run.return_leg {
            assert!(
                !return_leg.commodity.is_empty(),
                "Return leg should have a commodity"
            );
            assert!(
                return_leg.profit_per_scu > 0.0,
                "Return leg should have positive profit"
            );
        }

        // Total profit should be positive
        assert!(
            run.total_profit > 0.0,
            "Total profit should be positive: {}",
            run.total_profit
        );

        // has_return_cargo should match presence of return_leg
        assert_eq!(
            run.has_return_cargo,
            run.return_leg.is_some(),
            "has_return_cargo should match return_leg presence"
        );
    }

    // Trade runs should be sorted by total profit descending
    for window in trade_runs.windows(2) {
        assert!(
            window[0].total_profit >= window[1].total_profit,
            "Trade runs should be sorted by total profit descending"
        );
    }
}

#[tokio::test]
async fn test_get_interdiction_hotspots_integration() {
    let (_server, client) = setup_mock_server().await;
    let registry = test_registry();
    let analyzer = TargetAnalyzer::new(client, registry);

    let hotspots = analyzer.get_interdiction_hotspots(3).await.unwrap();

    // Should have hotspots from our fixtures
    assert!(!hotspots.is_empty(), "Expected at least one hotspot");

    // Port Olisar should be a hotspot since it appears in multiple routes
    let port_olisar = hotspots.iter().find(|h| h.location.contains("Port Olisar"));
    assert!(
        port_olisar.is_some(),
        "Port Olisar should be identified as a hotspot"
    );

    // Validate hotspot structure
    for hotspot in &hotspots {
        assert!(
            !hotspot.location.is_empty(),
            "Hotspot should have a location"
        );
        assert!(
            hotspot.route_count > 0,
            "Hotspot should have at least one route"
        );
        assert!(
            hotspot.total_cargo_value > 0.0,
            "Hotspot should have positive cargo value"
        );
        assert!(
            !hotspot.top_commodities.is_empty(),
            "Hotspot should have top commodities"
        );
        assert!(
            !hotspot.likely_ships.is_empty(),
            "Hotspot should have likely ships"
        );
        assert!(
            !hotspot.suggested_position.is_empty(),
            "Hotspot should have a suggested position"
        );

        // wikelo_potential should be None since we didn't configure WikieloIntel
        assert!(
            hotspot.wikelo_potential.is_none(),
            "wikelo_potential should be None without WikieloIntel"
        );
        assert!(
            hotspot.wikelo_items.is_empty(),
            "wikelo_items should be empty without WikieloIntel"
        );
    }

    // Hotspots should be sorted by total cargo value descending
    for window in hotspots.windows(2) {
        assert!(
            window[0].total_cargo_value >= window[1].total_cargo_value,
            "Hotspots should be sorted by total cargo value descending"
        );
    }
}
