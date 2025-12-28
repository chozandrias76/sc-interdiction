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
    #[instrument(skip(self))]
    pub async fn get_commodities(&self) -> Result<Vec<Commodity>> {
        let url = format!("{}/commodities", BASE_URL);
        let response: UexResponse<Vec<Commodity>> = self.get_json(&url).await?;
        Ok(response.data)
    }

    /// Get commodity prices at all terminals.
    #[instrument(skip(self))]
    pub async fn get_commodity_prices(&self, commodity_code: &str) -> Result<Vec<CommodityPrice>> {
        let url = format!("{}/commodities_prices?code={}", BASE_URL, commodity_code);
        let response: UexResponse<Vec<CommodityPrice>> = self.get_json(&url).await?;
        Ok(response.data)
    }

    /// Get all terminals (trade locations).
    #[instrument(skip(self))]
    pub async fn get_terminals(&self) -> Result<Vec<Terminal>> {
        let url = format!("{}/terminals", BASE_URL);
        let response: UexResponse<Vec<Terminal>> = self.get_json(&url).await?;
        Ok(response.data)
    }

    /// Get terminals in a specific system.
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

    /// Get profitable trade routes.
    #[instrument(skip(self))]
    pub async fn get_trade_routes(&self) -> Result<Vec<TradeRoute>> {
        let url = format!("{}/commodities_routes", BASE_URL);
        let response: UexResponse<Vec<TradeRoute>> = self.get_json(&url).await?;
        Ok(response.data)
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
        let parsed: T = serde_json::from_str(&body)?;
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
    pub code: String,
    pub name: String,
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
    pub terminal_name: String,
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

/// A trade terminal (location where you can buy/sell).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terminal {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub nickname: Option<String>,
    #[serde(default)]
    pub star_system_name: String,
    #[serde(default)]
    pub planet_name: String,
    #[serde(default)]
    pub moon_name: String,
    #[serde(default)]
    pub space_station_name: String,
    #[serde(default)]
    pub city_name: String,
    #[serde(rename = "type")]
    pub terminal_type: String,
}

impl Terminal {
    /// Get a human-readable location string.
    pub fn location_string(&self) -> String {
        let mut parts = vec![self.star_system_name.clone()];

        if !self.planet_name.is_empty() {
            parts.push(self.planet_name.clone());
        }
        if !self.moon_name.is_empty() {
            parts.push(self.moon_name.clone());
        }
        if !self.space_station_name.is_empty() {
            parts.push(self.space_station_name.clone());
        }
        if !self.city_name.is_empty() {
            parts.push(self.city_name.clone());
        }

        parts.join(" > ")
    }
}

/// A trade route showing profit opportunity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRoute {
    pub id_commodity: i64,
    pub commodity_name: String,
    pub commodity_code: String,
    pub id_terminal_origin: i64,
    pub terminal_origin_name: String,
    pub id_terminal_destination: i64,
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
