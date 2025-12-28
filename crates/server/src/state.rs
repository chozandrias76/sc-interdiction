//! Application state shared across handlers.

use api_client::{ScApiClient, UexClient};
use intel::TargetAnalyzer;
use route_graph::RouteGraph;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    pub sc_api: ScApiClient,
    pub uex: UexClient,
    pub analyzer: Arc<TargetAnalyzer>,
    pub graph: Arc<RwLock<RouteGraph>>,
}

impl AppState {
    /// Create new application state.
    pub fn new(sc_api_key: impl Into<String>) -> Self {
        let sc_api = ScApiClient::new(sc_api_key);
        let uex = UexClient::new();
        let analyzer = Arc::new(TargetAnalyzer::new(uex.clone()));
        let graph = Arc::new(RwLock::new(RouteGraph::new()));

        Self {
            sc_api,
            uex,
            analyzer,
            graph,
        }
    }

    /// Initialize the route graph with data from APIs.
    pub async fn init_graph(&self) -> eyre::Result<()> {
        let terminals = self.uex.get_terminals().await?;

        let mut graph = self.graph.write().await;

        for terminal in &terminals {
            graph.add_terminal(terminal);
        }

        // Connect terminals within each system
        let systems: std::collections::HashSet<_> = terminals
            .iter()
            .map(|t| t.star_system_name.clone())
            .collect();

        for system in systems {
            graph.connect_system(&system);
        }

        tracing::info!(
            "Initialized graph with {} nodes and {} edges",
            graph.node_count(),
            graph.edge_count()
        );

        Ok(())
    }
}
