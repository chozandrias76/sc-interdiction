//! Mock response fixtures for integration tests.
//!
//! Provides helper functions that generate mock JSON responses matching
//! the UEX API schema for testing composite API methods.

/// Returns a mock JSON response for the `/commodities_prices_all` endpoint.
///
/// Contains 3 commodities across 4 terminals with price differentials
/// that create profitable trade routes:
/// - Laranite: Buy at Port Olisar, sell at Area18 (profit: 5.0/SCU)
/// - Aluminum: Buy at Lorville, sell at Port Olisar (profit: 0.25/SCU)
/// - Gold: Buy at Ruin Station (Pyro), sell at Port Olisar (profit: 10.0/SCU)
pub fn mock_commodities_prices_all_response() -> &'static str {
    r#"{
        "status": "ok",
        "code": 200,
        "data": [
            {
                "id": 1,
                "id_commodity": 100,
                "id_terminal": 1,
                "commodity_name": "Laranite",
                "commodity_code": "LARA",
                "terminal_name": "Port Olisar",
                "price_buy": 25.0,
                "price_sell": 0.0,
                "scu_buy": 500.0,
                "scu_sell_stock": 0.0
            },
            {
                "id": 2,
                "id_commodity": 100,
                "id_terminal": 2,
                "commodity_name": "Laranite",
                "commodity_code": "LARA",
                "terminal_name": "Area18",
                "price_buy": 0.0,
                "price_sell": 30.0,
                "scu_buy": 0.0,
                "scu_sell_stock": 200.0
            },
            {
                "id": 3,
                "id_commodity": 101,
                "id_terminal": 3,
                "commodity_name": "Aluminum",
                "commodity_code": "ALUM",
                "terminal_name": "Lorville",
                "price_buy": 1.25,
                "price_sell": 0.0,
                "scu_buy": 800.0,
                "scu_sell_stock": 0.0
            },
            {
                "id": 4,
                "id_commodity": 101,
                "id_terminal": 1,
                "commodity_name": "Aluminum",
                "commodity_code": "ALUM",
                "terminal_name": "Port Olisar",
                "price_buy": 0.0,
                "price_sell": 1.50,
                "scu_buy": 0.0,
                "scu_sell_stock": 400.0
            },
            {
                "id": 5,
                "id_commodity": 102,
                "id_terminal": 4,
                "commodity_name": "Gold",
                "commodity_code": "GOLD",
                "terminal_name": "Ruin Station",
                "price_buy": 5.0,
                "price_sell": 0.0,
                "scu_buy": 300.0,
                "scu_sell_stock": 0.0
            },
            {
                "id": 6,
                "id_commodity": 102,
                "id_terminal": 1,
                "commodity_name": "Gold",
                "commodity_code": "GOLD",
                "terminal_name": "Port Olisar",
                "price_buy": 0.0,
                "price_sell": 15.0,
                "scu_buy": 0.0,
                "scu_sell_stock": 150.0
            },
            {
                "id": 7,
                "id_commodity": 100,
                "id_terminal": 3,
                "commodity_name": "Laranite",
                "commodity_code": "LARA",
                "terminal_name": "Lorville",
                "price_buy": 0.0,
                "price_sell": 28.0,
                "scu_buy": 0.0,
                "scu_sell_stock": 100.0
            }
        ]
    }"#
}

/// Returns a mock JSON response for the `/terminals` endpoint.
///
/// Contains 4 terminals across Stanton and Pyro systems:
/// - Port Olisar (Stanton) - ID 1
/// - Area18 (Stanton > ArcCorp) - ID 2
/// - Lorville (Stanton > Hurston) - ID 3
/// - Ruin Station (Pyro) - ID 4
pub fn mock_terminals_response() -> &'static str {
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
                "code": "A18",
                "name": "Area18",
                "star_system_name": "Stanton",
                "planet_name": "ArcCorp",
                "city_name": "Area18",
                "is_refuel": 1,
                "is_refinery": 0
            },
            {
                "id": 3,
                "code": "LOR",
                "name": "Lorville",
                "star_system_name": "Stanton",
                "planet_name": "Hurston",
                "city_name": "Lorville",
                "is_refuel": 1,
                "is_refinery": 1
            },
            {
                "id": 4,
                "code": "RUIN",
                "name": "Ruin Station",
                "star_system_name": "Pyro",
                "is_refuel": 1,
                "is_refinery": 0
            }
        ]
    }"#
}
