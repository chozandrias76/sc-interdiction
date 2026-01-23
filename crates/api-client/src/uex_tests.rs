//! Tests for UEX API client.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::indexing_slicing)]

use super::*;
use mockito::Server;

#[tokio::test]
async fn test_get_commodities() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/commodities")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "code": "ALUM",
                    "name": "Aluminum",
                    "type": "metal",
                    "is_available": true
                },
                {
                    "id": 2,
                    "code": "GOLD",
                    "name": "Gold",
                    "type": "metal",
                    "is_available": true
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let commodities = client.get_commodities().await.unwrap();

    mock.assert_async().await;
    assert_eq!(commodities.len(), 2);
    assert_eq!(commodities[0].code, "ALUM");
    assert_eq!(commodities[1].name, "Gold");
}

#[tokio::test]
async fn test_get_commodity_prices() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/commodities_prices?code=ALUM")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id_commodity": 1,
                    "id_terminal": 1,
                    "terminal_name": "Port Olisar",
                    "price_buy": 1.25,
                    "price_sell": 1.50,
                    "scu_buy": 100.0,
                    "scu_sell": 50.0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let prices = client.get_commodity_prices("ALUM").await.unwrap();

    mock.assert_async().await;
    assert_eq!(prices.len(), 1);
    assert_eq!(prices[0].terminal_name, "Port Olisar");
    assert_eq!(prices[0].price_buy, 1.25);
}

#[tokio::test]
async fn test_get_terminals() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/terminals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "code": "PO",
                    "name": "Port Olisar",
                    "star_system_name": "Stanton",
                    "is_refuel": 1,
                    "is_refinery": 0
                },
                {
                    "id": 2,
                    "code": "LOR",
                    "name": "Lorville",
                    "star_system_name": "Stanton",
                    "planet_name": "Hurston",
                    "is_refuel": 1,
                    "is_refinery": 1
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let terminals = client.get_terminals().await.unwrap();

    mock.assert_async().await;
    assert_eq!(terminals.len(), 2);
    assert_eq!(terminals[0].name.as_deref(), Some("Port Olisar"));
    assert!(terminals[0].is_refuel);
    assert!(!terminals[0].is_refinery);
    assert!(terminals[1].is_refuel);
    assert!(terminals[1].is_refinery);
}

#[tokio::test]
async fn test_get_terminals_in_system() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/terminals?star_system_name=Stanton")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "name": "Port Olisar",
                    "star_system_name": "Stanton",
                    "is_refuel": 0,
                    "is_refinery": 0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let terminals = client.get_terminals_in_system("Stanton").await.unwrap();

    mock.assert_async().await;
    assert_eq!(terminals.len(), 1);
    assert_eq!(terminals[0].star_system_name.as_deref(), Some("Stanton"));
}

#[tokio::test]
async fn test_terminal_bool_from_int_deserialization() {
    let mut server = Server::new_async().await;

    // Test that both int and bool values work for is_refuel and is_refinery
    let mock = server
        .mock("GET", "/terminals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "name": "Test Terminal",
                    "is_refuel": 1,
                    "is_refinery": true
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let terminals = client.get_terminals().await.unwrap();

    mock.assert_async().await;
    assert_eq!(terminals.len(), 1);
    assert!(terminals[0].is_refuel); // from int 1
    assert!(terminals[0].is_refinery); // from bool true
}

#[tokio::test]
async fn test_api_error_handling() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/commodities")
        .with_status(500)
        .with_body("Internal Server Error")
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let result = client.get_commodities().await;

    mock.assert_async().await;
    assert!(result.is_err());
    // The error will be a Request error from reqwest for 500 status
    match result.unwrap_err() {
        ApiError::Request(_) => {
            // Expected - reqwest treats 500 as a request error
        }
        ApiError::Api { status, .. } => {
            assert_eq!(status, 500);
        }
        other => panic!("Expected Request or Api error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_terminal_location_string() {
    let terminal = Terminal {
        id: 1,
        code: Some("TEST".to_string()),
        name: Some("Test Terminal".to_string()),
        nickname: None,
        star_system_name: Some("Stanton".to_string()),
        planet_name: Some("Hurston".to_string()),
        moon_name: None,
        space_station_name: None,
        outpost_name: None,
        city_name: Some("Lorville".to_string()),
        terminal_type: Some("Trade".to_string()),
        has_freight_elevator: true,
        has_loading_dock: false,
        has_docking_port: true,
        is_refuel: true,
        is_refinery: false,
    };

    let location = terminal.location_string();
    assert!(location.contains("Stanton"));
    assert!(location.contains("Hurston"));
    assert!(location.contains("Lorville"));
}

#[tokio::test]
async fn test_terminal_full_name() {
    let terminal = Terminal {
        id: 1,
        code: Some("LOR".to_string()),
        name: Some("Lorville".to_string()),
        nickname: None,
        star_system_name: Some("Stanton".to_string()),
        planet_name: Some("Hurston".to_string()),
        moon_name: None,
        space_station_name: None,
        outpost_name: None,
        city_name: None,
        terminal_type: Some("Trade".to_string()),
        has_freight_elevator: false,
        has_loading_dock: false,
        has_docking_port: false,
        is_refuel: false,
        is_refinery: false,
    };

    let full_name = terminal.full_name();
    assert_eq!(full_name, "Lorville (Stanton > Hurston)");
}

#[tokio::test]
async fn test_get_all_commodity_prices() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/commodities_prices_all")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "id_commodity": 1,
                    "id_terminal": 1,
                    "terminal_name": "Port Olisar",
                    "price_buy": 1.25,
                    "price_sell": 1.50,
                    "scu_buy": 100.0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let prices = client.get_all_commodity_prices().await.unwrap();

    mock.assert_async().await;
    assert_eq!(prices.len(), 1);
    assert_eq!(prices[0].id_commodity, 1);
    assert_eq!(prices[0].price_buy, 1.25);
}

// =============================================================================
// get_trade_routes tests
// =============================================================================

#[tokio::test]
async fn test_get_trade_routes_basic() {
    let mut server = Server::new_async().await;

    // Mock terminals endpoint
    let terminals_mock = server
        .mock("GET", "/terminals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "name": "Port Olisar",
                    "star_system_name": "Stanton",
                    "is_refuel": 0,
                    "is_refinery": 0
                },
                {
                    "id": 2,
                    "name": "Area18",
                    "star_system_name": "Stanton",
                    "planet_name": "ArcCorp",
                    "is_refuel": 0,
                    "is_refinery": 0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    // Mock commodities_prices_all endpoint
    let prices_mock = server
        .mock("GET", "/commodities_prices_all")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "id_commodity": 100,
                    "id_terminal": 1,
                    "commodity_name": "Aluminum",
                    "terminal_name": "Port Olisar",
                    "price_buy": 1.25,
                    "price_sell": 0.0,
                    "scu_buy": 500.0,
                    "scu_sell_stock": 0.0
                },
                {
                    "id": 2,
                    "id_commodity": 100,
                    "id_terminal": 2,
                    "commodity_name": "Aluminum",
                    "terminal_name": "Area18",
                    "price_buy": 0.0,
                    "price_sell": 1.50,
                    "scu_buy": 0.0,
                    "scu_sell_stock": 200.0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let routes = client.get_trade_routes().await.unwrap();

    terminals_mock.assert_async().await;
    prices_mock.assert_async().await;

    // Verify route calculation
    assert_eq!(routes.len(), 1);
    assert_eq!(routes[0].commodity_name, "Aluminum");
    assert_eq!(routes[0].id_terminal_origin, 1);
    assert_eq!(routes[0].id_terminal_destination, 2);
    assert_eq!(routes[0].price_origin, 1.25);
    assert_eq!(routes[0].price_destination, 1.50);
    // profit = sell_price - buy_price = 1.50 - 1.25 = 0.25
    assert!((routes[0].profit_per_unit - 0.25).abs() < 0.001);
}

#[tokio::test]
async fn test_get_trade_routes_no_profitable() {
    let mut server = Server::new_async().await;

    // Mock terminals endpoint
    let terminals_mock = server
        .mock("GET", "/terminals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {"id": 1, "name": "Terminal A", "is_refuel": 0, "is_refinery": 0},
                {"id": 2, "name": "Terminal B", "is_refuel": 0, "is_refinery": 0}
            ]
        }"#,
        )
        .create_async()
        .await;

    // Mock prices - all same price, no profit possible
    let prices_mock = server
        .mock("GET", "/commodities_prices_all")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "id_commodity": 100,
                    "id_terminal": 1,
                    "commodity_name": "Gold",
                    "terminal_name": "Terminal A",
                    "price_buy": 10.0,
                    "price_sell": 10.0,
                    "scu_buy": 100.0,
                    "scu_sell_stock": 100.0
                },
                {
                    "id": 2,
                    "id_commodity": 100,
                    "id_terminal": 2,
                    "commodity_name": "Gold",
                    "terminal_name": "Terminal B",
                    "price_buy": 10.0,
                    "price_sell": 10.0,
                    "scu_buy": 100.0,
                    "scu_sell_stock": 100.0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let routes = client.get_trade_routes().await.unwrap();

    terminals_mock.assert_async().await;
    prices_mock.assert_async().await;

    // No profitable routes when prices are equal
    assert!(routes.is_empty());
}

#[tokio::test]
async fn test_get_trade_routes_sorted_by_profit() {
    let mut server = Server::new_async().await;

    // Mock terminals endpoint
    let terminals_mock = server
        .mock("GET", "/terminals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {"id": 1, "name": "Terminal A", "is_refuel": 0, "is_refinery": 0},
                {"id": 2, "name": "Terminal B", "is_refuel": 0, "is_refinery": 0},
                {"id": 3, "name": "Terminal C", "is_refuel": 0, "is_refinery": 0}
            ]
        }"#,
        )
        .create_async()
        .await;

    // Multiple routes with different profits
    let prices_mock = server
        .mock("GET", "/commodities_prices_all")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "id_commodity": 100,
                    "id_terminal": 1,
                    "commodity_name": "Commodity A",
                    "terminal_name": "Terminal A",
                    "price_buy": 10.0,
                    "price_sell": 0.0,
                    "scu_buy": 100.0,
                    "scu_sell_stock": 0.0
                },
                {
                    "id": 2,
                    "id_commodity": 100,
                    "id_terminal": 2,
                    "commodity_name": "Commodity A",
                    "terminal_name": "Terminal B",
                    "price_buy": 0.0,
                    "price_sell": 12.0,
                    "scu_buy": 0.0,
                    "scu_sell_stock": 100.0
                },
                {
                    "id": 3,
                    "id_commodity": 100,
                    "id_terminal": 3,
                    "commodity_name": "Commodity A",
                    "terminal_name": "Terminal C",
                    "price_buy": 0.0,
                    "price_sell": 15.0,
                    "scu_buy": 0.0,
                    "scu_sell_stock": 100.0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let routes = client.get_trade_routes().await.unwrap();

    terminals_mock.assert_async().await;
    prices_mock.assert_async().await;

    // Routes should be sorted by profit descending
    assert_eq!(routes.len(), 2);
    // First route: Terminal A -> Terminal C (profit = 15 - 10 = 5)
    assert!((routes[0].profit_per_unit - 5.0).abs() < 0.001);
    assert_eq!(routes[0].id_terminal_destination, 3);
    // Second route: Terminal A -> Terminal B (profit = 12 - 10 = 2)
    assert!((routes[1].profit_per_unit - 2.0).abs() < 0.001);
    assert_eq!(routes[1].id_terminal_destination, 2);
}

#[tokio::test]
async fn test_get_trade_routes_uses_terminal_names() {
    let mut server = Server::new_async().await;

    // Mock terminals with full location info
    let terminals_mock = server
        .mock("GET", "/terminals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "name": "TDD",
                    "star_system_name": "Stanton",
                    "planet_name": "ArcCorp",
                    "city_name": "Area18",
                    "is_refuel": 0,
                    "is_refinery": 0
                },
                {
                    "id": 2,
                    "name": "Admin Office",
                    "star_system_name": "Stanton",
                    "planet_name": "Hurston",
                    "city_name": "Lorville",
                    "is_refuel": 0,
                    "is_refinery": 0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    // Prices with basic terminal names (routes should use full_name from terminals)
    let prices_mock = server
        .mock("GET", "/commodities_prices_all")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "id_commodity": 100,
                    "id_terminal": 1,
                    "commodity_name": "Titanium",
                    "terminal_name": "TDD",
                    "price_buy": 8.0,
                    "price_sell": 0.0,
                    "scu_buy": 100.0,
                    "scu_sell_stock": 0.0
                },
                {
                    "id": 2,
                    "id_commodity": 100,
                    "id_terminal": 2,
                    "commodity_name": "Titanium",
                    "terminal_name": "Admin Office",
                    "price_buy": 0.0,
                    "price_sell": 12.0,
                    "scu_buy": 0.0,
                    "scu_sell_stock": 50.0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let routes = client.get_trade_routes().await.unwrap();

    terminals_mock.assert_async().await;
    prices_mock.assert_async().await;

    assert_eq!(routes.len(), 1);
    // Terminal names should include full location info
    assert!(routes[0].terminal_origin_name.contains("TDD"));
    assert!(routes[0].terminal_origin_name.contains("Stanton"));
    assert!(routes[0].terminal_origin_name.contains("ArcCorp"));
    assert!(routes[0].terminal_destination_name.contains("Admin Office"));
    assert!(routes[0].terminal_destination_name.contains("Lorville"));
}

#[tokio::test]
async fn test_get_trade_routes_systems_populated() {
    let mut server = Server::new_async().await;

    // Mock terminals with different systems
    let terminals_mock = server
        .mock("GET", "/terminals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "name": "Port Olisar",
                    "star_system_name": "Stanton",
                    "is_refuel": 0,
                    "is_refinery": 0
                },
                {
                    "id": 2,
                    "name": "Pyro Gateway",
                    "star_system_name": "Pyro",
                    "is_refuel": 0,
                    "is_refinery": 0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let prices_mock = server
        .mock("GET", "/commodities_prices_all")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "id_commodity": 100,
                    "id_terminal": 1,
                    "commodity_name": "Scrap",
                    "terminal_name": "Port Olisar",
                    "price_buy": 5.0,
                    "price_sell": 0.0,
                    "scu_buy": 200.0,
                    "scu_sell_stock": 0.0
                },
                {
                    "id": 2,
                    "id_commodity": 100,
                    "id_terminal": 2,
                    "commodity_name": "Scrap",
                    "terminal_name": "Pyro Gateway",
                    "price_buy": 0.0,
                    "price_sell": 20.0,
                    "scu_buy": 0.0,
                    "scu_sell_stock": 100.0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let routes = client.get_trade_routes().await.unwrap();

    terminals_mock.assert_async().await;
    prices_mock.assert_async().await;

    assert_eq!(routes.len(), 1);
    // Verify origin_system and destination_system are populated from terminals
    assert_eq!(routes[0].origin_system, "Stanton");
    assert_eq!(routes[0].destination_system, "Pyro");
}

// =============================================================================
// Error handling tests
// =============================================================================

#[tokio::test]
async fn test_api_error_404_not_found() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/commodities")
        .with_status(404)
        .with_body("Not Found")
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let result = client.get_commodities().await;

    mock.assert_async().await;
    assert!(result.is_err());
    match result.unwrap_err() {
        ApiError::NotFound(url) => {
            assert!(url.contains("/commodities"));
        }
        other => panic!("Expected NotFound error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_api_error_429_rate_limited() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/terminals")
        .with_status(429)
        .with_body("Too Many Requests")
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let result = client.get_terminals().await;

    mock.assert_async().await;
    assert!(result.is_err());
    match result.unwrap_err() {
        ApiError::RateLimited { retry_after_secs } => {
            assert_eq!(retry_after_secs, 60);
        }
        other => panic!("Expected RateLimited error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_api_error_parse_failure() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/commodities")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{ invalid json }")
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let result = client.get_commodities().await;

    mock.assert_async().await;
    assert!(result.is_err());
    match result.unwrap_err() {
        ApiError::Parse(_) => {
            // Expected - malformed JSON causes parse error
        }
        other => panic!("Expected Parse error, got: {:?}", other),
    }
}

// =============================================================================
// Edge case tests
// =============================================================================

#[tokio::test]
async fn test_get_trade_routes_empty_terminals() {
    let mut server = Server::new_async().await;

    let terminals_mock = server
        .mock("GET", "/terminals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"status": "ok", "code": 200, "data": []}"#)
        .create_async()
        .await;

    let prices_mock = server
        .mock("GET", "/commodities_prices_all")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "id_commodity": 100,
                    "id_terminal": 1,
                    "commodity_name": "Gold",
                    "terminal_name": "Terminal A",
                    "price_buy": 10.0,
                    "price_sell": 0.0,
                    "scu_buy": 100.0,
                    "scu_sell_stock": 0.0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let routes = client.get_trade_routes().await.unwrap();

    terminals_mock.assert_async().await;
    prices_mock.assert_async().await;

    // Routes still work but terminal names come from price data, not terminal lookup
    // With only one terminal in prices, no routes can be formed anyway
    assert!(routes.is_empty());
}

#[tokio::test]
async fn test_get_trade_routes_empty_prices() {
    let mut server = Server::new_async().await;

    let terminals_mock = server
        .mock("GET", "/terminals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {"id": 1, "name": "Terminal A", "is_refuel": 0, "is_refinery": 0},
                {"id": 2, "name": "Terminal B", "is_refuel": 0, "is_refinery": 0}
            ]
        }"#,
        )
        .create_async()
        .await;

    let prices_mock = server
        .mock("GET", "/commodities_prices_all")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"status": "ok", "code": 200, "data": []}"#)
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let routes = client.get_trade_routes().await.unwrap();

    terminals_mock.assert_async().await;
    prices_mock.assert_async().await;

    // No prices means no routes
    assert!(routes.is_empty());
}

#[tokio::test]
async fn test_terminal_missing_system() {
    let mut server = Server::new_async().await;

    // Terminal without star_system_name
    let terminals_mock = server
        .mock("GET", "/terminals")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {"id": 1, "name": "Unknown Terminal A", "is_refuel": 0, "is_refinery": 0},
                {"id": 2, "name": "Unknown Terminal B", "is_refuel": 0, "is_refinery": 0}
            ]
        }"#,
        )
        .create_async()
        .await;

    let prices_mock = server
        .mock("GET", "/commodities_prices_all")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "status": "ok",
            "code": 200,
            "data": [
                {
                    "id": 1,
                    "id_commodity": 100,
                    "id_terminal": 1,
                    "commodity_name": "Scrap",
                    "terminal_name": "Unknown Terminal A",
                    "price_buy": 5.0,
                    "price_sell": 0.0,
                    "scu_buy": 100.0,
                    "scu_sell_stock": 0.0
                },
                {
                    "id": 2,
                    "id_commodity": 100,
                    "id_terminal": 2,
                    "commodity_name": "Scrap",
                    "terminal_name": "Unknown Terminal B",
                    "price_buy": 0.0,
                    "price_sell": 10.0,
                    "scu_buy": 0.0,
                    "scu_sell_stock": 50.0
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = UexClient::new_with_base_url(&server.url());
    let routes = client.get_trade_routes().await.unwrap();

    terminals_mock.assert_async().await;
    prices_mock.assert_async().await;

    assert_eq!(routes.len(), 1);
    // System should be empty string when not provided
    assert_eq!(routes[0].origin_system, "");
    assert_eq!(routes[0].destination_system, "");
}
