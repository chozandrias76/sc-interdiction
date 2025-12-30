//! Tests for UEX API client.

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
