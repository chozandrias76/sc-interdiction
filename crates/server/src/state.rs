//! Application state shared across handlers.

use api_client::{ScApiClient, UexClient};
use intel::{ShipRegistry, TargetAnalyzer};
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
    pub registry: Arc<ShipRegistry>,
}

impl AppState {
    /// Create new application state.
    ///
    /// # Errors
    ///
    /// Returns error if ship registry fails to load.
    pub async fn new(sc_api_key: impl Into<String>) -> eyre::Result<Self> {
        let sc_api = ScApiClient::new(sc_api_key);
        let uex = UexClient::new();
        let registry = ShipRegistry::load()
            .await
            .map_err(|e| eyre::eyre!("Failed to load ship registry: {}", e))?;
        let registry = Arc::new(registry);
        let analyzer = Arc::new(TargetAnalyzer::new(uex.clone(), Arc::clone(&registry)));
        let graph = Arc::new(RwLock::new(RouteGraph::new()));

        Ok(Self {
            sc_api,
            uex,
            analyzer,
            graph,
            registry,
        })
    }

    /// Initialize the route graph with data from APIs.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn init_graph(&self) -> eyre::Result<()> {
        let terminals = self.uex.get_terminals().await?;

        let mut graph = self.graph.write().await;

        for terminal in &terminals {
            graph.add_terminal(terminal);
        }

        // Connect terminals within each system
        let systems: std::collections::HashSet<_> = terminals
            .iter()
            .filter_map(|t| t.star_system_name.clone())
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
