//! API route handlers.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tracing::error;

use crate::state::AppState;

/// Create the API router with all routes.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health))
        // Route analysis
        .route("/api/routes/hot", get(get_hot_routes))
        .route("/api/routes/chokepoints", get(get_chokepoints))
        // Intel
        .route("/api/intel/targets/:location", get(get_targets_at))
        .route("/api/intel/ships", get(get_ships))
        // Data
        .route("/api/data/systems", get(get_systems))
        .route("/api/data/terminals", get(get_terminals))
        .route("/api/data/commodities", get(get_commodities))
        // State and middleware
        .with_state(state)
        .layer(CorsLayer::permissive())
}

// ============================================================================
// Handlers
// ============================================================================

async fn health() -> &'static str {
    "ok"
}

#[derive(Debug, Deserialize)]
struct HotRoutesQuery {
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    20
}

async fn get_hot_routes(
    State(state): State<AppState>,
    Query(query): Query<HotRoutesQuery>,
) -> impl IntoResponse {
    match state.analyzer.get_hot_routes(query.limit).await {
        Ok(routes) => Json(routes).into_response(),
        Err(e) => {
            error!("Failed to get hot routes: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
struct ChokepointsQuery {
    #[serde(default = "default_chokepoint_limit")]
    limit: usize,
}

fn default_chokepoint_limit() -> usize {
    10
}

async fn get_chokepoints(
    State(state): State<AppState>,
    Query(query): Query<ChokepointsQuery>,
) -> impl IntoResponse {
    let graph = state.graph.read().await;

    match state
        .analyzer
        .find_interdiction_points(&graph, query.limit)
        .await
    {
        Ok(chokepoints) => Json(chokepoints).into_response(),
        Err(e) => {
            error!("Failed to find chokepoints: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn get_targets_at(
    State(state): State<AppState>,
    Path(location): Path<String>,
) -> impl IntoResponse {
    match state.analyzer.predict_targets_at(&location).await {
        Ok(targets) => Json(targets).into_response(),
        Err(e) => {
            error!("Failed to predict targets: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn get_ships() -> Json<&'static [intel::CargoShip]> {
    Json(intel::CARGO_SHIPS)
}

async fn get_systems(State(state): State<AppState>) -> impl IntoResponse {
    match state.sc_api.get_systems().await {
        Ok(systems) => Json(systems).into_response(),
        Err(e) => {
            error!("Failed to get systems: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
struct TerminalsQuery {
    system: Option<String>,
}

async fn get_terminals(
    State(state): State<AppState>,
    Query(query): Query<TerminalsQuery>,
) -> impl IntoResponse {
    let result = match &query.system {
        Some(system) => state.uex.get_terminals_in_system(system).await,
        None => state.uex.get_terminals().await,
    };

    match result {
        Ok(terminals) => Json(terminals).into_response(),
        Err(e) => {
            error!("Failed to get terminals: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn get_commodities(State(state): State<AppState>) -> impl IntoResponse {
    match state.uex.get_commodities().await {
        Ok(commodities) => Json(commodities).into_response(),
        Err(e) => {
            error!("Failed to get commodities: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
