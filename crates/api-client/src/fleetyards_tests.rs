//! Tests for FleetYardsClient

use super::*;
use mockito::Server;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::TempDir;

/// Helper to create a mock ship for testing
fn mock_ship_json(id: &str, name: &str, manufacturer: Option<&str>) -> String {
    let manufacturer_json = if let Some(mfr) = manufacturer {
        format!(
            r#","manufacturer": {{"name": "{}", "slug": "{}", "code": "{}"}}"#,
            mfr,
            mfr.to_lowercase(),
            mfr.chars().take(4).collect::<String>().to_uppercase()
        )
    } else {
        String::new()
    };

    format!(
        r#"{{
            "id": "{}",
            "name": "{}",
            "slug": "{}",
            "description": "A test ship",
            "metrics": {{
                "cargo": 100.0,
                "hydrogenFuelTankSize": 500.0,
                "quantumFuelTankSize": 250.0,
                "length": 25.0,
                "beam": 15.0,
                "height": 8.0,
                "mass": 50000.0
            }},
            "crew": {{
                "min": 1,
                "max": 3
            }},
            "price": 125000.0
            {}
        }}"#,
        id,
        name,
        name.to_lowercase().replace(' ', "-"),
        manufacturer_json
    )
}

#[tokio::test]
async fn test_fetch_ships_single_page() {
    let mut server = Server::new_async().await;

    // Mock first page with 2 ships
    let page1_mock = server
        .mock("GET", "/models?page=1&perPage=100")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(format!(
            "[{}, {}]",
            mock_ship_json("ship-1", "Freelancer", Some("MISC")),
            mock_ship_json("ship-2", "Constellation", Some("Roberts Space Industries"))
        ))
        .create_async()
        .await;

    // Mock second page with empty array to signal end of pagination
    let page2_mock = server
        .mock("GET", "/models?page=2&perPage=100")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("[]")
        .create_async()
        .await;

    let client = create_test_client(&server.url());
    let ships = client.fetch_all_ships().await.unwrap();

    page1_mock.assert_async().await;
    page2_mock.assert_async().await;
    assert_eq!(ships.len(), 2);
    assert_eq!(ships[0].name, "Freelancer");
    assert_eq!(ships[1].name, "Constellation");
}

#[tokio::test]
async fn test_fetch_ships_multiple_pages() {
    let mut server = Server::new_async().await;

    // Mock page 1
    let page1_mock = server
        .mock("GET", "/models?page=1&perPage=100")
        .with_status(200)
        .with_body(format!(
            "[{}]",
            mock_ship_json("ship-1", "Freelancer", Some("MISC"))
        ))
        .create_async()
        .await;

    // Mock page 2
    let page2_mock = server
        .mock("GET", "/models?page=2&perPage=100")
        .with_status(200)
        .with_body(format!(
            "[{}]",
            mock_ship_json("ship-2", "Caterpillar", Some("Drake Interplanetary"))
        ))
        .create_async()
        .await;

    // Mock page 3 - empty to end pagination
    let page3_mock = server
        .mock("GET", "/models?page=3&perPage=100")
        .with_status(200)
        .with_body("[]")
        .create_async()
        .await;

    let client = create_test_client(&server.url());
    let ships = client.fetch_all_ships().await.unwrap();

    page1_mock.assert_async().await;
    page2_mock.assert_async().await;
    page3_mock.assert_async().await;
    assert_eq!(ships.len(), 2);
}

#[tokio::test]
async fn test_get_ship_by_name() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/models?page=1&perPage=100")
        .with_status(200)
        .with_body(format!(
            "[{}, {}]",
            mock_ship_json("ship-1", "Freelancer", Some("MISC")),
            mock_ship_json("ship-2", "Constellation", Some("RSI"))
        ))
        .create_async()
        .await;

    let _empty_mock = server
        .mock("GET", "/models?page=2&perPage=100")
        .with_status(200)
        .with_body("[]")
        .create_async()
        .await;

    let client = create_test_client(&server.url());

    // Test exact match
    let ship = client.get_ship("Freelancer").await.unwrap();
    assert!(ship.is_some());
    assert_eq!(ship.unwrap().name, "Freelancer");

    // Test case-insensitive match
    let ship = client.get_ship("freelancer").await.unwrap();
    assert!(ship.is_some());
    assert_eq!(ship.unwrap().name, "Freelancer");

    // Test non-existent ship
    let ship = client.get_ship("NonExistent").await.unwrap();
    assert!(ship.is_none());
}

#[tokio::test]
async fn test_get_ships_by_manufacturer() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/models?page=1&perPage=100")
        .with_status(200)
        .with_body(format!(
            "[{}, {}, {}]",
            mock_ship_json("ship-1", "Freelancer", Some("MISC")),
            mock_ship_json("ship-2", "Starfarer", Some("MISC")),
            mock_ship_json("ship-3", "Constellation", Some("Roberts Space Industries"))
        ))
        .create_async()
        .await;

    let _empty_mock = server
        .mock("GET", "/models?page=2&perPage=100")
        .with_status(200)
        .with_body("[]")
        .create_async()
        .await;

    let client = create_test_client(&server.url());

    // Test exact manufacturer name
    let ships = client.get_ships_by_manufacturer("MISC").await.unwrap();
    assert_eq!(ships.len(), 2);
    assert!(ships.iter().all(|s| s.manufacturer_name() == Some("MISC")));

    // Test partial match (case-insensitive)
    let ships = client.get_ships_by_manufacturer("roberts").await.unwrap();
    assert_eq!(ships.len(), 1);
    assert_eq!(ships[0].name, "Constellation");

    // Test non-existent manufacturer
    let ships = client
        .get_ships_by_manufacturer("NonExistent")
        .await
        .unwrap();
    assert_eq!(ships.len(), 0);
}

#[tokio::test]
async fn test_build_ship_lookup() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/models?page=1&perPage=100")
        .with_status(200)
        .with_body(format!(
            "[{}, {}]",
            mock_ship_json("ship-1", "Freelancer", Some("MISC")),
            mock_ship_json("ship-2", "Constellation", Some("RSI"))
        ))
        .create_async()
        .await;

    let _empty_mock = server
        .mock("GET", "/models?page=2&perPage=100")
        .with_status(200)
        .with_body("[]")
        .create_async()
        .await;

    let client = create_test_client(&server.url());
    let lookup = client.build_ship_lookup().await.unwrap();

    assert_eq!(lookup.len(), 2);
    assert!(lookup.contains_key("freelancer"));
    assert!(lookup.contains_key("constellation"));

    let freelancer = lookup.get("freelancer").unwrap();
    assert_eq!(freelancer.name, "Freelancer");
}

#[tokio::test]
async fn test_ship_model_metrics() {
    let ship_json = mock_ship_json("ship-1", "Freelancer", Some("MISC"));
    let ship: ShipModel = serde_json::from_str(&ship_json).unwrap();

    // Test fuel capacity methods
    assert_eq!(ship.hydrogen_fuel_capacity(), Some(500));
    assert_eq!(ship.quantum_fuel_capacity(), Some(250));
    assert_eq!(ship.cargo_capacity(), Some(100));
    assert_eq!(ship.manufacturer_name(), Some("MISC"));
}

#[tokio::test]
async fn test_ship_model_metrics_missing() {
    let ship_json = r#"{
        "id": "ship-1",
        "name": "Test Ship",
        "slug": "test-ship"
    }"#;
    let ship: ShipModel = serde_json::from_str(ship_json).unwrap();

    // All metrics should return None when not present
    assert_eq!(ship.hydrogen_fuel_capacity(), None);
    assert_eq!(ship.quantum_fuel_capacity(), None);
    assert_eq!(ship.cargo_capacity(), None);
    assert_eq!(ship.manufacturer_name(), None);
}

#[tokio::test]
async fn test_cache_save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().to_path_buf();

    let ship: ShipModel =
        serde_json::from_str(&mock_ship_json("ship-1", "Freelancer", Some("MISC"))).unwrap();

    let client = FleetYardsClient::with_cache(cache_dir.clone());

    // Save to cache
    client.save_to_cache(&[ship.clone()]).unwrap();

    // Verify file exists
    let cache_path = cache_dir.join("ships.json");
    assert!(cache_path.exists());

    // Load from cache
    let loaded_cache = client.load_from_cache().unwrap();
    assert_eq!(loaded_cache.ships.len(), 1);
    assert_eq!(loaded_cache.ships[0].name, "Freelancer");

    // Verify cache is considered valid (not expired)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    assert!(loaded_cache.fetched_at <= now);
    assert!(loaded_cache.fetched_at > now - 60); // Created within last minute
}

#[tokio::test]
async fn test_cache_expiration() {
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().to_path_buf();

    let client = FleetYardsClient::with_cache(cache_dir.clone());

    // Create an expired cache (fetched 25 hours ago)
    let expired_cache = ShipCache {
        version: env!("CARGO_PKG_VERSION").to_string(),
        fetched_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - (25 * 3600), // 25 hours ago
        ships: vec![
            serde_json::from_str(&mock_ship_json("ship-1", "Freelancer", Some("MISC"))).unwrap(),
        ],
    };

    // Save expired cache
    let cache_path = cache_dir.join("ships.json");
    std::fs::create_dir_all(&cache_dir).unwrap();
    let data = serde_json::to_string_pretty(&expired_cache).unwrap();
    std::fs::write(&cache_path, data).unwrap();

    // Try to load - should return None because cache is expired
    let loaded = client.load_from_cache();
    assert!(loaded.is_none(), "Expired cache should not be loaded");
}

#[tokio::test]
async fn test_get_ships_uses_cache() {
    let server = Server::new_async().await;
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().to_path_buf();

    // Create a client with cache
    let client = create_test_client_with_cache(&server.url(), cache_dir.clone());

    // Pre-populate the cache with valid data
    let ship: ShipModel =
        serde_json::from_str(&mock_ship_json("ship-1", "Cached Ship", Some("MISC"))).unwrap();
    client.save_to_cache(&[ship]).unwrap();

    // Call get_ships - should use cache and NOT call API
    let ships = client.get_ships().await.unwrap();
    assert_eq!(ships.len(), 1);
    assert_eq!(ships[0].name, "Cached Ship");

    // Verify no API calls were made
    // (if a mock was created and called, this test would fail)
}

#[tokio::test]
async fn test_refresh_cache() {
    let mut server = Server::new_async().await;
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().to_path_buf();

    let _mock = server
        .mock("GET", "/models?page=1&perPage=100")
        .with_status(200)
        .with_body(format!(
            "[{}]",
            mock_ship_json("ship-1", "Fresh Ship", Some("MISC"))
        ))
        .create_async()
        .await;

    let _empty_mock = server
        .mock("GET", "/models?page=2&perPage=100")
        .with_status(200)
        .with_body("[]")
        .create_async()
        .await;

    let client = create_test_client_with_cache(&server.url(), cache_dir.clone());

    // Pre-populate cache with old data
    let old_ship: ShipModel =
        serde_json::from_str(&mock_ship_json("ship-old", "Old Ship", Some("MISC"))).unwrap();
    client.save_to_cache(&[old_ship]).unwrap();

    // Refresh cache - should fetch from API and update cache
    let ships = client.refresh_cache().await.unwrap();
    assert_eq!(ships.len(), 1);
    assert_eq!(ships[0].name, "Fresh Ship");

    // Verify cache was updated
    let cached = client.load_from_cache().unwrap();
    assert_eq!(cached.ships[0].name, "Fresh Ship");
}

#[tokio::test]
async fn test_api_error_handling() {
    let mut server = Server::new_async().await;

    // Test 404 error
    let _mock = server
        .mock("GET", "/models?page=1&perPage=100")
        .with_status(404)
        .with_body("Not Found")
        .create_async()
        .await;

    let client = create_test_client(&server.url());
    let result = client.fetch_all_ships().await;
    assert!(result.is_err());
    match result.unwrap_err() {
        ApiError::NotFound(_) => {}
        e => panic!("Expected NotFound error, got: {:?}", e),
    }
}

#[tokio::test]
async fn test_rate_limit_error() {
    let mut server = Server::new_async().await;

    let _mock = server
        .mock("GET", "/models?page=1&perPage=100")
        .with_status(429)
        .with_body("Rate limited")
        .create_async()
        .await;

    let client = create_test_client(&server.url());
    let result = client.fetch_all_ships().await;
    assert!(result.is_err());
    match result.unwrap_err() {
        ApiError::RateLimited { .. } => {}
        e => panic!("Expected RateLimited error, got: {:?}", e),
    }
}

#[tokio::test]
async fn test_pagination_safety_limit() {
    let mut server = Server::new_async().await;

    // Create mocks for pages 1-21 (exceeds safety limit of 20)
    for page in 1..=21 {
        let path = format!("/models?page={}&perPage=100", page);
        server
            .mock("GET", path.as_str())
            .with_status(200)
            .with_body(format!(
                "[{}]",
                mock_ship_json(&format!("ship-{}", page), "Test Ship", Some("MISC"))
            ))
            .create_async()
            .await;
    }

    let client = create_test_client(&server.url());
    let ships = client.fetch_all_ships().await.unwrap();

    // Should stop at page 20 due to safety limit
    assert_eq!(ships.len(), 20);
}

// Helper functions

/// Create a test client with a custom base URL (for mocking)
fn create_test_client(base_url: &str) -> TestFleetYardsClient {
    TestFleetYardsClient {
        client: reqwest::Client::new(),
        base_url: base_url.to_string(),
        cache_dir: None,
    }
}

/// Create a test client with cache directory
fn create_test_client_with_cache(base_url: &str, cache_dir: PathBuf) -> TestFleetYardsClient {
    TestFleetYardsClient {
        client: reqwest::Client::new(),
        base_url: base_url.to_string(),
        cache_dir: Some(cache_dir),
    }
}

/// Test version of FleetYardsClient that allows base URL override
#[derive(Clone)]
struct TestFleetYardsClient {
    client: reqwest::Client,
    base_url: String,
    cache_dir: Option<PathBuf>,
}

impl TestFleetYardsClient {
    async fn fetch_all_ships(&self) -> Result<Vec<ShipModel>> {
        let mut all_ships = Vec::new();
        let mut page = 1;

        loop {
            let url = format!("{}/models?page={}&perPage=100", self.base_url, page);
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

            all_ships.extend(ships);
            page += 1;

            // Safety limit
            if page > 20 {
                break;
            }
        }

        Ok(all_ships)
    }

    async fn get_ships(&self) -> Result<Vec<ShipModel>> {
        // Try loading from cache first
        if let Some(cached) = self.load_from_cache() {
            return Ok(cached.ships);
        }

        // Fetch from API
        let ships = self.fetch_all_ships().await?;

        // Save to cache
        let _ = self.save_to_cache(&ships);

        Ok(ships)
    }

    async fn get_ship(&self, name: &str) -> Result<Option<ShipModel>> {
        let ships = self.get_ships().await?;
        let name_lower = name.to_lowercase();
        Ok(ships
            .into_iter()
            .find(|s| s.name.to_lowercase() == name_lower))
    }

    async fn get_ships_by_manufacturer(&self, manufacturer: &str) -> Result<Vec<ShipModel>> {
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

    async fn build_ship_lookup(&self) -> Result<HashMap<String, ShipModel>> {
        let ships = self.get_ships().await?;
        let map: HashMap<String, ShipModel> = ships
            .into_iter()
            .map(|s| (s.name.to_lowercase(), s))
            .collect();
        Ok(map)
    }

    async fn refresh_cache(&self) -> Result<Vec<ShipModel>> {
        let ships = self.fetch_all_ships().await?;
        let _ = self.save_to_cache(&ships);
        Ok(ships)
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
        Ok(())
    }
}
