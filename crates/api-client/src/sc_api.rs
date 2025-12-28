//! Client for starcitizen-api.com.
//!
//! Provides access to starmap data including systems, objects, and routes.

use crate::{ApiError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::instrument;

const BASE_URL: &str = "https://api.starcitizen-api.com";

/// Client for the Star Citizen API.
#[derive(Clone)]
pub struct ScApiClient {
    client: Client,
    api_key: String,
}

impl ScApiClient {
    /// Create a new SC API client.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
        }
    }

    /// Fetch a star system by code (e.g., "STANTON").
    #[instrument(skip(self))]
    pub async fn get_system(&self, code: &str) -> Result<StarSystem> {
        let url = format!(
            "{}/{}/cache/starmap/star-system?code={}",
            BASE_URL, self.api_key, code
        );
        self.get_json(&url).await
    }

    /// Fetch all systems.
    #[instrument(skip(self))]
    pub async fn get_systems(&self) -> Result<Vec<StarSystem>> {
        let url = format!("{}/{}/cache/starmap/systems", BASE_URL, self.api_key);
        let response: SystemsResponse = self.get_json(&url).await?;
        Ok(response.data)
    }

    /// Search starmap objects by name.
    #[instrument(skip(self))]
    pub async fn search_starmap(&self, query: &str) -> Result<Vec<StarmapObject>> {
        let url = format!(
            "{}/{}/cache/starmap/search?name={}",
            BASE_URL,
            self.api_key,
            urlencoding::encode(query)
        );
        let response: SearchResponse = self.get_json(&url).await?;
        Ok(response.data)
    }

    /// Get all stations/landing zones in a system.
    #[instrument(skip(self))]
    pub async fn get_stations(&self, system_code: &str) -> Result<Vec<Station>> {
        let system = self.get_system(system_code).await?;
        Ok(extract_stations(&system))
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

/// A star system (e.g., Stanton, Pyro).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystem {
    pub id: String,
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub affiliation: Vec<Affiliation>,
    #[serde(default)]
    pub celestial_objects: Vec<CelestialObject>,
}

/// Political affiliation of a system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affiliation {
    pub id: String,
    pub name: String,
    pub code: String,
}

/// A celestial object (planet, moon, station, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelestialObject {
    pub id: String,
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String,
    #[serde(default)]
    pub designation: String,
    #[serde(default)]
    pub children: Vec<CelestialObject>,
}

/// A station or landing zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub id: String,
    pub code: String,
    pub name: String,
    pub station_type: String,
    pub parent_name: String,
    pub system_code: String,
}

/// Starmap search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarmapObject {
    pub id: String,
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String,
}

#[derive(Debug, Deserialize)]
struct SystemsResponse {
    data: Vec<StarSystem>,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    data: Vec<StarmapObject>,
}

/// Extract stations from a system's celestial objects.
fn extract_stations(system: &StarSystem) -> Vec<Station> {
    let mut stations = Vec::new();
    extract_stations_recursive(&system.celestial_objects, &system.code, "", &mut stations);
    stations
}

fn extract_stations_recursive(
    objects: &[CelestialObject],
    system_code: &str,
    parent_name: &str,
    stations: &mut Vec<Station>,
) {
    for obj in objects {
        let is_station = matches!(
            obj.object_type.as_str(),
            "STATION" | "LANDING_ZONE" | "OUTPOST" | "CITY" | "SETTLEMENT"
        );

        if is_station {
            stations.push(Station {
                id: obj.id.clone(),
                code: obj.code.clone(),
                name: obj.name.clone(),
                station_type: obj.object_type.clone(),
                parent_name: parent_name.to_string(),
                system_code: system_code.to_string(),
            });
        }

        let next_parent = if parent_name.is_empty() {
            &obj.name
        } else {
            parent_name
        };

        extract_stations_recursive(&obj.children, system_code, next_parent, stations);
    }
}
