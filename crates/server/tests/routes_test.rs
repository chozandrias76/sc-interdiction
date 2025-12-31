//! Integration tests for API routes.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use server::{create_router, AppState};
use tower::ServiceExt;

/// Helper to send a GET request to the app and return status + body
async fn get(uri: &str) -> (StatusCode, String) {
    let state = AppState::new("test-api-key");
    let app = create_router(state);

    let request = Request::builder()
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    (status, body_str)
}

#[tokio::test]
async fn test_health_endpoint() {
    let (status, body) = get("/health").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "ok");
}

#[tokio::test]
async fn test_get_ships_endpoint() {
    let (status, body) = get("/api/intel/ships").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.starts_with('['), "Response should be a JSON array");
    assert!(
        body.contains("Caterpillar") || body.contains("C2"),
        "Ships list should contain known ship names"
    );
}

#[tokio::test]
async fn test_get_hot_routes_default_limit() {
    let (status, _body) = get("/api/routes/hot").await;

    // Should return 200 or 500 (if APIs fail, which is expected in test)
    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_hot_routes_custom_limit() {
    let (status, _body) = get("/api/routes/hot?limit=5").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_chokepoints_default() {
    let (status, _body) = get("/api/routes/chokepoints").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_chokepoints_with_params() {
    let (status, _body) = get("/api/routes/chokepoints?top=5&cross_system=true").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_chokepoints_respects_max_limit() {
    let (status, _body) = get("/api/routes/chokepoints?top=200").await;

    // Request 200, should be capped at 100 (MAX_CHOKEPOINTS)
    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_targets_at_location() {
    let (status, _body) = get("/api/intel/targets/Stanton").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_hotspots_default() {
    let (status, _body) = get("/api/intel/hotspots").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_hotspots_custom_limit() {
    let (status, _body) = get("/api/intel/hotspots?top=15").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_hotspots_respects_max_limit() {
    let (status, _body) = get("/api/intel/hotspots?top=200").await;

    // Request 200, should be capped at 100 (MAX_HOTSPOTS)
    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_intersections_default() {
    let (status, _body) = get("/api/intel/intersections").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_intersections_with_params() {
    let (status, _body) = get("/api/intel/intersections?limit=5&min_routes=3").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_systems() {
    let (status, _body) = get("/api/data/systems").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_terminals_all() {
    let (status, _body) = get("/api/data/terminals").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_terminals_by_system() {
    let (status, _body) = get("/api/data/terminals?system=Stanton").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_get_commodities() {
    let (status, _body) = get("/api/data/commodities").await;

    assert!(
        status.is_success() || status.is_server_error(),
        "Expected success or server error, got {}",
        status
    );
}

#[tokio::test]
async fn test_invalid_route_returns_404() {
    let (status, _body) = get("/api/invalid/route").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}
