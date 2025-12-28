//! Client for UEX Corporation API.
//!
//! Provides access to commodity prices, trade terminals, and market data.

use crate::{ApiError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::instrument;

const BASE_URL: &str = "https://uexcorp.space/api/2.0";

/// Client for the UEX API.
#[derive(Clone)]
pub struct UexClient {
    client: Client,
}

impl UexClient {
    /// Create a new UEX client.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Get all commodities.
    ///
    /// See: <https://uexcorp.space/api/documentation/id/get_commodities/>
    #[instrument(skip(self))]
    pub async fn get_commodities(&self) -> Result<Vec<Commodity>> {
        let url = format!("{}/commodities", BASE_URL);
        let response: UexResponse<Vec<Commodity>> = self.get_json(&url).await?;
        Ok(response.data)
    }

    /// Get commodity prices at all terminals.
    ///
    /// See: <https://uexcorp.space/api/documentation/id/get_commodities_prices/>
    #[instrument(skip(self))]
    pub async fn get_commodity_prices(&self, commodity_code: &str) -> Result<Vec<CommodityPrice>> {
        let url = format!("{}/commodities_prices?code={}", BASE_URL, commodity_code);
        let response: UexResponse<Vec<CommodityPrice>> = self.get_json(&url).await?;
        Ok(response.data)
    }

    /// Get all terminals (trade locations).
    ///
    /// See: <https://uexcorp.space/api/documentation/id/get_terminals/>
    #[instrument(skip(self))]
    pub async fn get_terminals(&self) -> Result<Vec<Terminal>> {
        let url = format!("{}/terminals", BASE_URL);
        let response: UexResponse<Vec<Terminal>> = self.get_json(&url).await?;
        Ok(response.data)
    }

    /// Get terminals in a specific system.
    ///
    /// See: <https://uexcorp.space/api/documentation/id/get_terminals/>
    #[instrument(skip(self))]
    pub async fn get_terminals_in_system(&self, system: &str) -> Result<Vec<Terminal>> {
        let url = format!(
            "{}/terminals?star_system_name={}",
            BASE_URL,
            urlencoding::encode(system)
        );
        let response: UexResponse<Vec<Terminal>> = self.get_json(&url).await?;
        Ok(response.data)
    }

    /// Get all commodity prices across all terminals.
    ///
    /// See: <https://uexcorp.space/api/documentation/id/get_commodities_prices_all/>
    #[instrument(skip(self))]
    pub async fn get_all_commodity_prices(&self) -> Result<Vec<CommodityPriceAll>> {
        let url = format!("{}/commodities_prices_all", BASE_URL);
        let response: UexResponse<Vec<CommodityPriceAll>> = self.get_json(&url).await?;
        Ok(response.data)
    }

    /// Get profitable trade routes by calculating from price data.
    ///
    /// Routes are calculated by finding profitable buy/sell combinations
    /// across all terminals using data from the commodities_prices_all endpoint.
    /// Terminal location info is fetched from the terminals endpoint to enable
    /// location-based searches (e.g., "Crusader", "Hurston").
    ///
    /// See: <https://uexcorp.space/api/documentation/id/get_commodities_prices_all/>
    /// See: <https://uexcorp.space/api/documentation/id/get_terminals/>
    #[instrument(skip(self))]
    pub async fn get_trade_routes(&self) -> Result<Vec<TradeRoute>> {
        // Fetch both prices and terminals concurrently
        let (prices, terminals) = tokio::try_join!(
            self.get_all_commodity_prices(),
            self.get_terminals()
        )?;

        // Build terminal lookup map (id -> full name with location)
        let terminal_names: std::collections::HashMap<i64, String> = terminals
            .into_iter()
            .map(|t| {
                let full_name = t.full_name();
                (t.id, full_name)
            })
            .collect();

        // Group prices by commodity
        let mut by_commodity: std::collections::HashMap<i64, Vec<&CommodityPriceAll>> =
            std::collections::HashMap::new();

        for price in &prices {
            by_commodity
                .entry(price.id_commodity)
                .or_default()
                .push(price);
        }

        let mut routes = Vec::new();

        // Find profitable routes: buy low at origin, sell high at destination
        for (commodity_id, commodity_prices) in &by_commodity {
            // Find terminals that buy (have stock to sell to players)
            let buy_terminals: Vec<_> = commodity_prices
                .iter()
                .filter(|p| p.price_buy > 0.0 && p.scu_buy > 0.0)
                .collect();

            // Find terminals that sell (buy from players)
            let sell_terminals: Vec<_> = commodity_prices
                .iter()
                .filter(|p| p.price_sell > 0.0)
                .collect();

            // Calculate all profitable combinations
            for origin in &buy_terminals {
                for dest in &sell_terminals {
                    if origin.id_terminal == dest.id_terminal {
                        continue;
                    }

                    let profit = dest.price_sell - origin.price_buy;
                    if profit > 0.0 {
                        // Use full terminal name with location if available
                        let origin_name = terminal_names
                            .get(&origin.id_terminal)
                            .cloned()
                            .unwrap_or_else(|| origin.terminal_name.clone());
                        let dest_name = terminal_names
                            .get(&dest.id_terminal)
                            .cloned()
                            .unwrap_or_else(|| dest.terminal_name.clone());

                        routes.push(TradeRoute {
                            id_commodity: *commodity_id,
                            commodity_name: origin.commodity_name.clone(),
                            commodity_code: String::new(),
                            id_terminal_origin: origin.id_terminal,
                            terminal_origin_name: origin_name,
                            id_terminal_destination: dest.id_terminal,
                            terminal_destination_name: dest_name,
                            price_origin: origin.price_buy,
                            price_destination: dest.price_sell,
                            profit_per_unit: profit,
                            scu_origin: origin.scu_buy,
                            scu_destination: dest.scu_sell_stock,
                        });
                    }
                }
            }
        }

        // Sort by profit per unit descending
        routes.sort_by(|a, b| b.profit_per_unit.partial_cmp(&a.profit_per_unit).unwrap_or(std::cmp::Ordering::Equal));

        Ok(routes)
    }

    async fn get_json<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<T> {
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();

            return Err(match status {
                404 => ApiError::NotFound(url.to_string()),
                429 => ApiError::RateLimited {
                    retry_after_secs: 60,
                },
                _ => ApiError::Api { status, message },
            });
        }

        let body = response.text().await?;
        let parsed: T = serde_json::from_str(&body).map_err(|e| {
            eprintln!("Failed to parse JSON response from {}", url);
            eprintln!("Response body (first 500 chars): {}", &body.chars().take(500).collect::<String>());
            eprintln!("Parse error: {}", e);
            e
        })?;
        Ok(parsed)
    }
}

impl Default for UexClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct UexResponse<T> {
    data: T,
}

/// A tradeable commodity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commodity {
    pub id: i64,
    pub id_parent: Option<i64>,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub is_illegal: bool,
    #[serde(default)]
    pub is_raw: bool,
    #[serde(default)]
    pub is_harvestable: bool,
}

/// Price information for a commodity at a terminal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityPrice {
    pub id_commodity: i64,
    pub id_terminal: i64,
    #[serde(default)]
    pub terminal_name: String,
    #[serde(default)]
    pub terminal_code: String,
    #[serde(default)]
    pub price_buy: f64,
    #[serde(default)]
    pub price_sell: f64,
    #[serde(default)]
    pub scu_buy: f64,
    #[serde(default)]
    pub scu_sell: f64,
    #[serde(default)]
    pub status_buy: i32,
    #[serde(default)]
    pub status_sell: i32,
}

impl CommodityPrice {
    /// Returns true if this terminal buys the commodity.
    pub fn can_buy(&self) -> bool {
        self.price_buy > 0.0 && self.scu_buy > 0.0
    }

    /// Returns true if this terminal sells the commodity.
    pub fn can_sell(&self) -> bool {
        self.price_sell > 0.0 && self.scu_sell > 0.0
    }
}

/// Price information for a commodity at a terminal (from commodities_prices_all endpoint).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityPriceAll {
    pub id: i64,
    pub id_commodity: i64,
    pub id_terminal: i64,
    #[serde(default)]
    pub price_buy: f64,
    #[serde(default)]
    pub price_sell: f64,
    #[serde(default)]
    pub scu_buy: f64,
    #[serde(default)]
    pub scu_sell_stock: f64,
    #[serde(default)]
    pub commodity_name: String,
    #[serde(default)]
    pub terminal_name: String,
}

/// A trade terminal (location where you can buy/sell).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terminal {
    pub id: i64,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub star_system_name: Option<String>,
    #[serde(default)]
    pub planet_name: Option<String>,
    #[serde(default)]
    pub moon_name: Option<String>,
    #[serde(default)]
    pub space_station_name: Option<String>,
    #[serde(default)]
    pub outpost_name: Option<String>,
    #[serde(default)]
    pub city_name: Option<String>,
    #[serde(default, rename = "type")]
    pub terminal_type: Option<String>,
    #[serde(default, deserialize_with = "deserialize_bool_from_int")]
    pub has_freight_elevator: bool,
    #[serde(default, deserialize_with = "deserialize_bool_from_int")]
    pub has_loading_dock: bool,
    #[serde(default, deserialize_with = "deserialize_bool_from_int")]
    pub has_docking_port: bool,
    #[serde(default, deserialize_with = "deserialize_bool_from_int")]
    pub is_refuel: bool,
    #[serde(default, deserialize_with = "deserialize_bool_from_int")]
    pub is_refinery: bool,
}

/// Deserialize a boolean from an integer (0 = false, non-zero = true).
fn deserialize_bool_from_int<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrInt {
        Bool(bool),
        Int(i64),
    }

    match BoolOrInt::deserialize(deserializer)? {
        BoolOrInt::Bool(b) => Ok(b),
        BoolOrInt::Int(i) => Ok(i != 0),
    }
}

impl Terminal {
    /// Get a human-readable location string.
    pub fn location_string(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref s) = self.star_system_name {
            if !s.is_empty() {
                parts.push(s.clone());
            }
        }
        if let Some(ref p) = self.planet_name {
            if !p.is_empty() {
                parts.push(p.clone());
            }
        }
        if let Some(ref m) = self.moon_name {
            if !m.is_empty() {
                parts.push(m.clone());
            }
        }
        if let Some(ref s) = self.space_station_name {
            if !s.is_empty() {
                parts.push(s.clone());
            }
        }
        if let Some(ref c) = self.city_name {
            if !c.is_empty() {
                parts.push(c.clone());
            }
        }

        parts.join(" > ")
    }

    /// Get a full display name including location.
    pub fn full_name(&self) -> String {
        let name = self.name.clone().unwrap_or_default();
        let location = self.location_string();
        if location.is_empty() {
            name
        } else {
            format!("{} ({})", name, location)
        }
    }
}

/// A trade route showing profit opportunity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRoute {
    pub id_commodity: i64,
    #[serde(default)]
    pub commodity_name: String,
    #[serde(default)]
    pub commodity_code: String,
    pub id_terminal_origin: i64,
    #[serde(default)]
    pub terminal_origin_name: String,
    pub id_terminal_destination: i64,
    #[serde(default)]
    pub terminal_destination_name: String,
    #[serde(default)]
    pub price_origin: f64,
    #[serde(default)]
    pub price_destination: f64,
    #[serde(default)]
    pub profit_per_unit: f64,
    #[serde(default)]
    pub scu_origin: f64,
    #[serde(default)]
    pub scu_destination: f64,
}

impl TradeRoute {
    /// Calculate profit for a given cargo capacity.
    pub fn profit_for_scu(&self, scu: f64) -> f64 {
        let available = self.scu_origin.min(self.scu_destination).min(scu);
        available * self.profit_per_unit
    }

    /// Calculate maximum profitable cargo.
    pub fn max_profitable_scu(&self) -> f64 {
        self.scu_origin.min(self.scu_destination)
    }
}
