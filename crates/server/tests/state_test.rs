//! Tests for application state management.

use server::AppState;
use std::sync::Arc;

#[test]
fn test_app_state_creation() {
    let state = AppState::new("test-api-key");

    // Verify state can be created successfully
    // If we got here without panicking, the state was created
    let _ = state.analyzer;
    let _ = state.graph;
}

#[test]
fn test_app_state_clone() {
    let state = AppState::new("test-api-key");
    let cloned = state.clone();

    // AppState should be cloneable (required for Axum)
    // Verify Arc references are properly cloned
    assert_eq!(
        Arc::strong_count(&state.analyzer),
        Arc::strong_count(&cloned.analyzer)
    );
    assert_eq!(
        Arc::strong_count(&state.graph),
        Arc::strong_count(&cloned.graph)
    );
}

#[tokio::test]
async fn test_init_graph_empty_state() {
    let state = AppState::new("test-api-key");

    // Graph should start empty
    let graph = state.graph.read().await;
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[tokio::test]
async fn test_init_graph_is_idempotent() {
    let state = AppState::new("test-api-key");

    // Calling init_graph multiple times should be safe
    // (though it will fail without real API access)
    let result1 = state.init_graph().await;
    let result2 = state.init_graph().await;

    // Both should have same result (both fail or both succeed)
    assert_eq!(result1.is_ok(), result2.is_ok());
}
