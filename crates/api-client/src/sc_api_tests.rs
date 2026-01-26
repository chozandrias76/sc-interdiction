//! Tests for SC API client.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::indexing_slicing)]

use crate::sc_api::{extract_stations, CelestialObject, StarSystem, StarmapObject};
use crate::{ApiError, Result};
use mockito::Server;
use reqwest::Client;

// =============================================================================
// extract_stations unit tests
// =============================================================================

#[test]
fn test_extract_stations_empty_system() {
    let system = StarSystem {
        id: "sys-1".to_string(),
        code: "STANTON".to_string(),
        name: "Stanton".to_string(),
        description: String::new(),
        affiliation: vec![],
        celestial_objects: vec![],
    };

    let stations = extract_stations(&system);
    assert!(stations.is_empty());
}

#[test]
fn test_extract_stations_single_station() {
    let system = StarSystem {
        id: "sys-1".to_string(),
        code: "STANTON".to_string(),
        name: "Stanton".to_string(),
        description: String::new(),
        affiliation: vec![],
        celestial_objects: vec![CelestialObject {
            id: "obj-1".to_string(),
            code: "PO".to_string(),
            name: "Port Olisar".to_string(),
            object_type: "STATION".to_string(),
            designation: String::new(),
            children: vec![],
        }],
    };

    let stations = extract_stations(&system);
    assert_eq!(stations.len(), 1);
    assert_eq!(stations[0].name, "Port Olisar");
    assert_eq!(stations[0].station_type, "STATION");
    assert_eq!(stations[0].system_code, "STANTON");
}

#[test]
fn test_extract_stations_nested_landing_zones() {
    let system = StarSystem {
        id: "sys-1".to_string(),
        code: "STANTON".to_string(),
        name: "Stanton".to_string(),
        description: String::new(),
        affiliation: vec![],
        celestial_objects: vec![CelestialObject {
            id: "planet-1".to_string(),
            code: "HURSTON".to_string(),
            name: "Hurston".to_string(),
            object_type: "PLANET".to_string(),
            designation: String::new(),
            children: vec![CelestialObject {
                id: "lz-1".to_string(),
                code: "LOR".to_string(),
                name: "Lorville".to_string(),
                object_type: "LANDING_ZONE".to_string(),
                designation: String::new(),
                children: vec![],
            }],
        }],
    };

    let stations = extract_stations(&system);
    assert_eq!(stations.len(), 1);
    assert_eq!(stations[0].name, "Lorville");
    assert_eq!(stations[0].station_type, "LANDING_ZONE");
    assert_eq!(stations[0].parent_name, "Hurston");
}

#[test]
fn test_extract_stations_filters_non_stations() {
    let system = StarSystem {
        id: "sys-1".to_string(),
        code: "STANTON".to_string(),
        name: "Stanton".to_string(),
        description: String::new(),
        affiliation: vec![],
        celestial_objects: vec![
            CelestialObject {
                id: "planet-1".to_string(),
                code: "CRUSADER".to_string(),
                name: "Crusader".to_string(),
                object_type: "PLANET".to_string(),
                designation: String::new(),
                children: vec![],
            },
            CelestialObject {
                id: "moon-1".to_string(),
                code: "YELA".to_string(),
                name: "Yela".to_string(),
                object_type: "MOON".to_string(),
                designation: String::new(),
                children: vec![],
            },
            CelestialObject {
                id: "station-1".to_string(),
                code: "PO".to_string(),
                name: "Port Olisar".to_string(),
                object_type: "STATION".to_string(),
                designation: String::new(),
                children: vec![],
            },
        ],
    };

    let stations = extract_stations(&system);
    // Should only include STATION, not PLANET or MOON
    assert_eq!(stations.len(), 1);
    assert_eq!(stations[0].name, "Port Olisar");
}

#[test]
fn test_extract_stations_multiple_types() {
    let system = StarSystem {
        id: "sys-1".to_string(),
        code: "STANTON".to_string(),
        name: "Stanton".to_string(),
        description: String::new(),
        affiliation: vec![],
        celestial_objects: vec![
            CelestialObject {
                id: "s1".to_string(),
                code: "PO".to_string(),
                name: "Port Olisar".to_string(),
                object_type: "STATION".to_string(),
                designation: String::new(),
                children: vec![],
            },
            CelestialObject {
                id: "lz1".to_string(),
                code: "A18".to_string(),
                name: "Area 18".to_string(),
                object_type: "LANDING_ZONE".to_string(),
                designation: String::new(),
                children: vec![],
            },
            CelestialObject {
                id: "op1".to_string(),
                code: "OP".to_string(),
                name: "Outpost Alpha".to_string(),
                object_type: "OUTPOST".to_string(),
                designation: String::new(),
                children: vec![],
            },
            CelestialObject {
                id: "c1".to_string(),
                code: "LOR".to_string(),
                name: "Lorville".to_string(),
                object_type: "CITY".to_string(),
                designation: String::new(),
                children: vec![],
            },
            CelestialObject {
                id: "set1".to_string(),
                code: "SET".to_string(),
                name: "Small Settlement".to_string(),
                object_type: "SETTLEMENT".to_string(),
                designation: String::new(),
                children: vec![],
            },
        ],
    };

    let stations = extract_stations(&system);
    // All 5 types should be included
    assert_eq!(stations.len(), 5);

    let types: Vec<&str> = stations.iter().map(|s| s.station_type.as_str()).collect();
    assert!(types.contains(&"STATION"));
    assert!(types.contains(&"LANDING_ZONE"));
    assert!(types.contains(&"OUTPOST"));
    assert!(types.contains(&"CITY"));
    assert!(types.contains(&"SETTLEMENT"));
}

#[test]
fn test_extract_stations_parent_name_propagation() {
    let system = StarSystem {
        id: "sys-1".to_string(),
        code: "STANTON".to_string(),
        name: "Stanton".to_string(),
        description: String::new(),
        affiliation: vec![],
        celestial_objects: vec![CelestialObject {
            id: "planet-1".to_string(),
            code: "ARCCORP".to_string(),
            name: "ArcCorp".to_string(),
            object_type: "PLANET".to_string(),
            designation: String::new(),
            children: vec![
                CelestialObject {
                    id: "lz-1".to_string(),
                    code: "A18".to_string(),
                    name: "Area 18".to_string(),
                    object_type: "LANDING_ZONE".to_string(),
                    designation: String::new(),
                    children: vec![],
                },
                CelestialObject {
                    id: "moon-1".to_string(),
                    code: "LYRIA".to_string(),
                    name: "Lyria".to_string(),
                    object_type: "MOON".to_string(),
                    designation: String::new(),
                    children: vec![CelestialObject {
                        id: "op-1".to_string(),
                        code: "LOP".to_string(),
                        name: "Lyria Outpost".to_string(),
                        object_type: "OUTPOST".to_string(),
                        designation: String::new(),
                        children: vec![],
                    }],
                },
            ],
        }],
    };

    let stations = extract_stations(&system);
    assert_eq!(stations.len(), 2);

    // Area 18 should have parent "ArcCorp"
    let area18 = stations.iter().find(|s| s.name == "Area 18").unwrap();
    assert_eq!(area18.parent_name, "ArcCorp");

    // Lyria Outpost should have parent "ArcCorp" (first planet, not moon)
    let outpost = stations.iter().find(|s| s.name == "Lyria Outpost").unwrap();
    assert_eq!(outpost.parent_name, "ArcCorp");
}

// =============================================================================
// Mock API tests using TestScApiClient
// =============================================================================

/// Test client that allows base URL override for mocking
struct TestScApiClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl TestScApiClient {
    fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            api_key: "test-key".to_string(),
        }
    }

    async fn get_system(&self, code: &str) -> Result<StarSystem> {
        let url = format!(
            "{}/{}/cache/starmap/star-system?code={}",
            self.base_url, self.api_key, code
        );
        self.get_json(&url).await
    }

    async fn get_systems(&self) -> Result<Vec<StarSystem>> {
        let url = format!("{}/{}/cache/starmap/systems", self.base_url, self.api_key);

        #[derive(serde::Deserialize)]
        struct SystemsResponse {
            data: Vec<StarSystem>,
        }

        let response: SystemsResponse = self.get_json(&url).await?;
        Ok(response.data)
    }

    async fn search_starmap(&self, query: &str) -> Result<Vec<StarmapObject>> {
        let url = format!(
            "{}/{}/cache/starmap/search?name={}",
            self.base_url,
            self.api_key,
            urlencoding::encode(query)
        );

        #[derive(serde::Deserialize)]
        struct SearchResponse {
            data: Vec<StarmapObject>,
        }

        let response: SearchResponse = self.get_json(&url).await?;
        Ok(response.data)
    }

    async fn get_json<T: for<'de> serde::Deserialize<'de>>(&self, url: &str) -> Result<T> {
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

#[tokio::test]
async fn test_get_system_success() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/test-key/cache/starmap/star-system?code=STANTON")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "id": "sys-1",
            "code": "STANTON",
            "name": "Stanton",
            "description": "A corporate system",
            "affiliation": [],
            "celestial_objects": [
                {
                    "id": "obj-1",
                    "code": "PO",
                    "name": "Port Olisar",
                    "type": "STATION",
                    "designation": "",
                    "children": []
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = TestScApiClient::new(&server.url());
    let system = client.get_system("STANTON").await.unwrap();

    mock.assert_async().await;
    assert_eq!(system.code, "STANTON");
    assert_eq!(system.name, "Stanton");
    assert_eq!(system.celestial_objects.len(), 1);
    assert_eq!(system.celestial_objects[0].name, "Port Olisar");
}

#[tokio::test]
async fn test_get_systems_success() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/test-key/cache/starmap/systems")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "data": [
                {
                    "id": "sys-1",
                    "code": "STANTON",
                    "name": "Stanton",
                    "description": "",
                    "affiliation": [],
                    "celestial_objects": []
                },
                {
                    "id": "sys-2",
                    "code": "PYRO",
                    "name": "Pyro",
                    "description": "",
                    "affiliation": [],
                    "celestial_objects": []
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = TestScApiClient::new(&server.url());
    let systems = client.get_systems().await.unwrap();

    mock.assert_async().await;
    assert_eq!(systems.len(), 2);
    assert_eq!(systems[0].code, "STANTON");
    assert_eq!(systems[1].code, "PYRO");
}

#[tokio::test]
async fn test_search_starmap_success() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/test-key/cache/starmap/search?name=Port")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "data": [
                {
                    "id": "obj-1",
                    "code": "PO",
                    "name": "Port Olisar",
                    "type": "STATION"
                },
                {
                    "id": "obj-2",
                    "code": "PT",
                    "name": "Port Tressler",
                    "type": "STATION"
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let client = TestScApiClient::new(&server.url());
    let results = client.search_starmap("Port").await.unwrap();

    mock.assert_async().await;
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].name, "Port Olisar");
    assert_eq!(results[1].name, "Port Tressler");
}

#[tokio::test]
async fn test_api_error_404() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/test-key/cache/starmap/star-system?code=INVALID")
        .with_status(404)
        .with_body("Not Found")
        .create_async()
        .await;

    let client = TestScApiClient::new(&server.url());
    let result = client.get_system("INVALID").await;

    mock.assert_async().await;
    assert!(result.is_err());
    match result.unwrap_err() {
        ApiError::NotFound(url) => {
            assert!(url.contains("INVALID"));
        }
        other => panic!("Expected NotFound error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_api_error_429() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/test-key/cache/starmap/systems")
        .with_status(429)
        .with_body("Rate Limited")
        .create_async()
        .await;

    let client = TestScApiClient::new(&server.url());
    let result = client.get_systems().await;

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
async fn test_api_error_500() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/test-key/cache/starmap/systems")
        .with_status(500)
        .with_body("Internal Server Error")
        .create_async()
        .await;

    let client = TestScApiClient::new(&server.url());
    let result = client.get_systems().await;

    mock.assert_async().await;
    assert!(result.is_err());
    match result.unwrap_err() {
        ApiError::Api { status, message } => {
            assert_eq!(status, 500);
            assert!(message.contains("Internal Server Error"));
        }
        other => panic!("Expected Api error, got: {:?}", other),
    }
}
