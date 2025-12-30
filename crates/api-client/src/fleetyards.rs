//! Client for FleetYards.net API.
//!
//! Provides access to ship specifications including fuel capacity and quantum drive data.
//! Data is cached locally with version tracking for offline use and reduced API calls.

use crate::{ApiError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{info, instrument, warn};

const BASE_URL: &str = "https://api.fleetyards.net/v1";

/// Client for the `FleetYards` API.
#[derive(Clone)]
pub struct FleetYardsClient {
    client: Client,
    cache_dir: Option<PathBuf>,
}

impl FleetYardsClient {
    /// Create a new `FleetYards` client.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            cache_dir: None,
        }
    }

    /// Create a new `FleetYards` client with local caching.
    pub fn with_cache(cache_dir: PathBuf) -> Self {
        Self {
            client: Client::new(),
            cache_dir: Some(cache_dir),
        }
    }

    /// Get all ship models from the API.
    #[instrument(skip(self))]
    pub async fn get_ships(&self) -> Result<Vec<ShipModel>> {
        // Try loading from cache first
        if let Some(cached) = self.load_from_cache() {
            info!("Loaded {} ships from cache", cached.ships.len());
            return Ok(cached.ships);
        }

        // Fetch from API
        let ships = self.fetch_all_ships().await?;

        // Save to cache
        if let Err(e) = self.save_to_cache(&ships) {
            warn!("Failed to save ship cache: {}", e);
        }

        Ok(ships)
    }

    /// Fetch all ships with pagination.
    async fn fetch_all_ships(&self) -> Result<Vec<ShipModel>> {
        let mut all_ships = Vec::new();
        let mut page = 1;

        loop {
            let url = format!("{}/models?page={}&perPage=100", BASE_URL, page);
            let response = self.client.get(&url).send().await?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let message = response.text().await.unwrap_or_default();
                return Err(match status {
                    404 => ApiError::NotFound(url),
                    429 => ApiError::RateLimited {
                        retry_after_secs: 60,
                    },
                    _ => ApiError::Api { status, message },
                });
            }

            let ships: Vec<ShipModel> = response.json().await?;
            if ships.is_empty() {
                break;
            }

            info!("Fetched page {} with {} ships", page, ships.len());
            all_ships.extend(ships);
            page += 1;

            // Safety limit
            if page > 20 {
                break;
            }
        }

        Ok(all_ships)
    }

    /// Get a ship by name (case-insensitive).
    #[instrument(skip(self))]
    pub async fn get_ship(&self, name: &str) -> Result<Option<ShipModel>> {
        let ships = self.get_ships().await?;
        let name_lower = name.to_lowercase();
        Ok(ships
            .into_iter()
            .find(|s| s.name.to_lowercase() == name_lower))
    }

    /// Get ships by manufacturer.
    #[instrument(skip(self))]
    pub async fn get_ships_by_manufacturer(&self, manufacturer: &str) -> Result<Vec<ShipModel>> {
        let ships = self.get_ships().await?;
        let mfr_lower = manufacturer.to_lowercase();
        Ok(ships
            .into_iter()
            .filter(|s| {
                s.manufacturer
                    .as_ref()
                    .is_some_and(|m| m.name.to_lowercase().contains(&mfr_lower))
            })
            .collect())
    }

    /// Build a lookup map from ship name to ship data.
    pub async fn build_ship_lookup(&self) -> Result<HashMap<String, ShipModel>> {
        let ships = self.get_ships().await?;
        let map: HashMap<String, ShipModel> = ships
            .into_iter()
            .map(|s| (s.name.to_lowercase(), s))
            .collect();
        Ok(map)
    }

    fn cache_path(&self) -> Option<PathBuf> {
        self.cache_dir.as_ref().map(|d| d.join("ships.json"))
    }

    fn load_from_cache(&self) -> Option<ShipCache> {
        let path = self.cache_path()?;
        if !path.exists() {
            return None;
        }

        let data = std::fs::read_to_string(&path).ok()?;
        let cache: ShipCache = serde_json::from_str(&data).ok()?;

        // Check if cache is still valid (24 hours)
        let age = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_secs()
            - cache.fetched_at;

        if age > 86400 {
            return None; // Cache expired
        }

        Some(cache)
    }

    fn save_to_cache(&self, ships: &[ShipModel]) -> std::io::Result<()> {
        let Some(path) = self.cache_path() else {
            return Ok(());
        };

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let cache = ShipCache {
            version: env!("CARGO_PKG_VERSION").to_string(),
            fetched_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                .as_secs(),
            ships: ships.to_vec(),
        };

        let data = serde_json::to_string_pretty(&cache)?;
        std::fs::write(&path, data)?;
        info!("Saved {} ships to cache at {:?}", ships.len(), path);
        Ok(())
    }

    /// Force refresh the cache from the API.
    pub async fn refresh_cache(&self) -> Result<Vec<ShipModel>> {
        let ships = self.fetch_all_ships().await?;
        if let Err(e) = self.save_to_cache(&ships) {
            warn!("Failed to save ship cache: {}", e);
        }
        Ok(ships)
    }
}

impl Default for FleetYardsClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Cached ship data with versioning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipCache {
    /// Application version that created this cache.
    pub version: String,
    /// Unix timestamp when data was fetched.
    pub fetched_at: u64,
    /// Cached ship data.
    pub ships: Vec<ShipModel>,
}

/// A ship model from `FleetYards`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipModel {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub manufacturer: Option<Manufacturer>,
    #[serde(default)]
    pub metrics: Option<ShipMetrics>,
    #[serde(default)]
    pub crew: Option<CrewInfo>,
    #[serde(default)]
    pub speeds: Option<SpeedInfo>,
    #[serde(default)]
    pub focus: Option<String>,
    #[serde(default)]
    pub production_status: Option<String>,
    #[serde(default)]
    pub classification: Option<String>,
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde(default)]
    pub pledge_price: Option<f64>,
    #[serde(default)]
    pub rsi_id: Option<f64>,
}

impl ShipModel {
    /// Get hydrogen fuel tank capacity.
    pub fn hydrogen_fuel_capacity(&self) -> Option<u64> {
        self.metrics
            .as_ref()?
            .hydrogen_fuel_tank_size
            .map(|v| v as u64)
    }

    /// Get quantum fuel tank capacity.
    pub fn quantum_fuel_capacity(&self) -> Option<u64> {
        self.metrics
            .as_ref()?
            .quantum_fuel_tank_size
            .map(|v| v as u64)
    }

    /// Get cargo capacity in SCU.
    pub fn cargo_capacity(&self) -> Option<u64> {
        self.metrics.as_ref()?.cargo.map(|v| v as u64)
    }

    /// Get the manufacturer name.
    pub fn manufacturer_name(&self) -> Option<&str> {
        self.manufacturer.as_ref().map(|m| m.name.as_str())
    }
}

/// Ship manufacturer information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manufacturer {
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub code: Option<String>,
}

/// Crew size information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrewInfo {
    #[serde(default)]
    pub min: Option<u32>,
    #[serde(default)]
    pub max: Option<u32>,
}

/// Speed performance information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeedInfo {
    #[serde(default)]
    pub scm_speed: Option<f64>,
    #[serde(default)]
    pub max_speed_acceleration: Option<f64>,
    #[serde(default)]
    pub max_speed_decceleration: Option<f64>,
    #[serde(default)]
    pub scm_speed_acceleration: Option<f64>,
    #[serde(default)]
    pub scm_speed_decceleration: Option<f64>,
}

/// Ship metrics/specifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipMetrics {
    #[serde(default)]
    pub beam: Option<f64>,
    #[serde(default)]
    pub cargo: Option<f64>,
    #[serde(default)]
    pub height: Option<f64>,
    #[serde(default)]
    pub length: Option<f64>,
    #[serde(default)]
    pub mass: Option<f64>,
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub hydrogen_fuel_tank_size: Option<f64>,
    #[serde(default)]
    pub quantum_fuel_tank_size: Option<f64>,
}
