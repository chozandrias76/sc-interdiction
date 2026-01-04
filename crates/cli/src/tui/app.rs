//! Application state for the TUI.

use api_client::UexClient;
use eyre::Result;
use intel::{HotRoute, TargetAnalyzer, TargetPrediction, TrafficDirection};
use route_graph::RouteIntersection;

use super::data::hotspots::load_hotspots;
use super::data::map_locations::{build_map_locations, infer_system};
use super::text::ScrollState;
use super::types::{MapLocation, RouteSort, TargetSort, View};

/// Application state.
pub struct App {
    /// Current view.
    pub view: View,
    /// Location being analyzed.
    pub location: String,
    /// Target predictions.
    pub targets: Vec<TargetPrediction>,
    /// Hot trade routes.
    pub routes: Vec<HotRoute>,
    /// Interdiction hotspots for map view.
    pub hotspots: Vec<RouteIntersection>,
    /// Static locations for map view.
    pub map_locations: Vec<MapLocation>,
    /// Current system for map view.
    pub map_system: String,
    /// Selected hotspot index in map view.
    pub map_selected: usize,
    /// Map zoom level (1.0 = default, smaller = zoomed out).
    pub map_zoom: f64,
    /// Number of hotspots to display on map (0 = all).
    pub hotspot_limit: usize,
    /// Selected index in current list.
    pub selected: usize,
    /// Filter: show only inbound.
    pub filter_inbound: bool,
    /// Filter: show only outbound.
    pub filter_outbound: bool,
    /// Minimum threat level filter.
    pub min_threat: u8,
    /// Target sort column.
    pub target_sort: TargetSort,
    /// Route sort column.
    pub route_sort: RouteSort,
    /// Sort ascending.
    pub sort_asc: bool,
    /// Loading state.
    pub loading: bool,
    /// Error message.
    pub error: Option<String>,
    /// Status message.
    pub status: String,
    /// Scroll state for text display.
    pub scroll: ScrollState,
    /// Detail view expanded (for map view).
    #[allow(dead_code)]
    pub detail_expanded: bool,
    /// Selected route index in expanded detail view.
    #[allow(dead_code)]
    pub detail_selected: usize,
}

impl App {
    /// Create a new app and load initial data.
    pub async fn new(location: Option<String>) -> Result<Self> {
        let location = location.unwrap_or_else(|| "Crusader".to_string());
        let uex = UexClient::new();
        let registry = intel::ShipRegistry::load()
            .await
            .map_err(|e| eyre::eyre!("Failed to load ship registry: {}", e))?;
        let analyzer = TargetAnalyzer::new(uex.clone(), registry);

        // Determine system from location
        let map_system = infer_system(&location);
        let map_locations = build_map_locations(&map_system);

        let mut app = Self {
            view: View::Targets,
            location: location.clone(),
            targets: Vec::new(),
            routes: Vec::new(),
            hotspots: Vec::new(),
            map_locations,
            map_system,
            map_selected: 0,
            map_zoom: 1.0,
            hotspot_limit: 1, // Start showing just the top hotspot
            selected: 0,
            filter_inbound: false,
            filter_outbound: false,
            min_threat: 0,
            target_sort: TargetSort::Value,
            route_sort: RouteSort::Profit,
            sort_asc: false,
            loading: true,
            error: None,
            status: "Loading data...".to_string(),
            scroll: ScrollState::new(),
            detail_expanded: false,
            detail_selected: 0,
        };

        // Load data
        match analyzer.predict_targets_at(&location).await {
            Ok(targets) => {
                app.targets = targets;
                app.status = format!("Loaded {} targets for {}", app.targets.len(), location);
            }
            Err(e) => {
                app.error = Some(format!("Failed to load targets: {}", e));
            }
        }

        match analyzer.get_hot_routes(100).await {
            Ok(routes) => {
                app.routes = routes;
            }
            Err(e) => {
                if app.error.is_none() {
                    app.error = Some(format!("Failed to load routes: {}", e));
                }
            }
        }

        // Load hotspots for map view - we'll fetch intersections via route analysis
        app.hotspots = load_hotspots(&app.routes);

        app.loading = false;
        app.sort_targets();
        app.sort_routes();

        Ok(app)
    }

    /// Handle tick events.
    pub fn on_tick(&mut self) {
        self.scroll.on_tick();
    }

    /// Handle key events. Returns true if the app should exit.

    /// Get the number of visible hotspots (limited by `hotspot_limit`).
    pub fn visible_hotspot_count(&self) -> usize {
        self.hotspot_limit.min(self.hotspots.len())
    }

    /// Get visible hotspots iterator (limited by `hotspot_limit`).
    pub fn visible_hotspots(&self) -> impl Iterator<Item = &RouteIntersection> {
        self.hotspots.iter().take(self.hotspot_limit)
    }

    /// Get filtered targets iterator.
    pub fn filtered_targets(&self) -> impl Iterator<Item = &TargetPrediction> {
        self.targets.iter().filter(|t| {
            let direction_ok = match (self.filter_inbound, self.filter_outbound) {
                (true, false) => t.direction == TrafficDirection::Arriving,
                (false, true) => t.direction == TrafficDirection::Departing,
                _ => true,
            };
            let threat_ok = t.likely_ship.threat_level >= self.min_threat;
            direction_ok && threat_ok
        })
    }
}
