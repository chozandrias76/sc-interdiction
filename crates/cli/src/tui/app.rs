//! Application state for the TUI.

use api_client::UexClient;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use eyre::Result;
use intel::{TargetAnalyzer, TargetPrediction, HotRoute, TrafficDirection};
use route_graph::RouteIntersection;

use super::text::ScrollState;
use super::types::{MapLocation, MapLocationType, RouteSort, TargetSort, View};

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
}

impl App {
    /// Create a new app and load initial data.
    pub async fn new(location: Option<String>) -> Result<Self> {
        let location = location.unwrap_or_else(|| "Crusader".to_string());
        let uex = UexClient::new();
        let analyzer = TargetAnalyzer::new(uex.clone());

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
    pub fn on_key(&mut self, key: KeyEvent) -> bool {
        // Quit on q or Ctrl+C
        if key.code == KeyCode::Char('q')
            || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
        {
            return true;
        }

        match key.code {
            // Navigation
            KeyCode::Up | KeyCode::Char('k') => self.prev(),
            KeyCode::Down | KeyCode::Char('j') => self.next(),
            KeyCode::PageUp => self.page_up(),
            KeyCode::PageDown => self.page_down(),
            KeyCode::Home => self.home(),
            KeyCode::End => self.end(),

            // View switching
            KeyCode::Tab => self.next_view(),
            KeyCode::Char('1') => self.view = View::Targets,
            KeyCode::Char('2') => self.view = View::Routes,
            KeyCode::Char('3') => self.view = View::Map,
            KeyCode::Char('?') => self.view = View::Help,
            KeyCode::Esc => {
                if self.view == View::Help {
                    self.view = View::Targets;
                }
            }

            // Map navigation
            KeyCode::Left | KeyCode::Char('h') => {
                if self.view == View::Map && self.map_selected > 0 {
                    self.map_selected -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.view == View::Map {
                    let max = self.visible_hotspot_count().saturating_sub(1);
                    if self.map_selected < max {
                        self.map_selected += 1;
                    }
                }
            }
            // Map zoom (z/Z or [/])
            KeyCode::Char('z') | KeyCode::Char('[') => {
                if self.view == View::Map {
                    self.map_zoom = (self.map_zoom * 0.8).max(0.2); // Zoom out
                }
            }
            KeyCode::Char('Z') | KeyCode::Char(']') => {
                if self.view == View::Map {
                    self.map_zoom = (self.map_zoom * 1.25).min(3.0); // Zoom in
                }
            }
            KeyCode::Char('0') => {
                if self.view == View::Map {
                    self.map_zoom = 1.0; // Reset zoom
                }
            }
            // Hotspot limit (n/N to decrease/increase, a for all)
            KeyCode::Char('n') => {
                if self.view == View::Map && self.hotspot_limit > 1 {
                    self.hotspot_limit -= 1;
                    // Adjust selection if it's now out of bounds
                    if self.map_selected >= self.hotspot_limit {
                        self.map_selected = self.hotspot_limit.saturating_sub(1);
                    }
                }
            }
            KeyCode::Char('N') => {
                if self.view == View::Map {
                    let max = self.hotspots.len();
                    if self.hotspot_limit < max {
                        self.hotspot_limit += 1;
                    }
                }
            }
            KeyCode::Char('a') => {
                if self.view == View::Map {
                    // Toggle between showing all and showing 1
                    if self.hotspot_limit == self.hotspots.len() {
                        self.hotspot_limit = 1;
                        self.map_selected = 0;
                    } else {
                        self.hotspot_limit = self.hotspots.len();
                    }
                }
            }

            // Filtering
            KeyCode::Char('i') => {
                self.filter_inbound = !self.filter_inbound;
                if self.filter_inbound {
                    self.filter_outbound = false;
                }
            }
            KeyCode::Char('o') => {
                self.filter_outbound = !self.filter_outbound;
                if self.filter_outbound {
                    self.filter_inbound = false;
                }
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                if self.min_threat < 10 {
                    self.min_threat += 1;
                }
            }
            KeyCode::Char('-') => {
                if self.min_threat > 0 {
                    self.min_threat -= 1;
                }
            }

            // Sorting
            KeyCode::Char('s') => {
                match self.view {
                    View::Targets => {
                        self.target_sort = match self.target_sort {
                            TargetSort::Value => TargetSort::Threat,
                            TargetSort::Threat => TargetSort::Ship,
                            TargetSort::Ship => TargetSort::Commodity,
                            TargetSort::Commodity => TargetSort::Value,
                        };
                        self.sort_targets();
                    }
                    View::Routes => {
                        self.route_sort = match self.route_sort {
                            RouteSort::Profit => RouteSort::Value,
                            RouteSort::Value => RouteSort::Commodity,
                            RouteSort::Commodity => RouteSort::Profit,
                        };
                        self.sort_routes();
                    }
                    _ => {}
                }
            }
            KeyCode::Char('S') => {
                self.sort_asc = !self.sort_asc;
                self.sort_targets();
                self.sort_routes();
            }

            _ => {}
        }

        false
    }

    fn prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn next(&mut self) {
        let max = self.filtered_len().saturating_sub(1);
        if self.selected < max {
            self.selected += 1;
        }
    }

    fn page_up(&mut self) {
        self.selected = self.selected.saturating_sub(20);
    }

    fn page_down(&mut self) {
        let max = self.filtered_len().saturating_sub(1);
        self.selected = (self.selected + 20).min(max);
    }

    fn home(&mut self) {
        self.selected = 0;
    }

    fn end(&mut self) {
        self.selected = self.filtered_len().saturating_sub(1);
    }

    fn next_view(&mut self) {
        self.view = match self.view {
            View::Targets => View::Routes,
            View::Routes => View::Map,
            View::Map => View::Targets,
            View::Help => View::Targets,
        };
        self.selected = 0;
    }

    fn filtered_len(&self) -> usize {
        match self.view {
            View::Targets => self.filtered_targets().count(),
            View::Routes => self.routes.len(),
            View::Map => self.visible_hotspot_count(),
            View::Help => 0,
        }
    }

    /// Get the number of visible hotspots (limited by hotspot_limit).
    pub fn visible_hotspot_count(&self) -> usize {
        self.hotspot_limit.min(self.hotspots.len())
    }

    /// Get visible hotspots iterator (limited by hotspot_limit).
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

    fn sort_targets(&mut self) {
        let asc = self.sort_asc;
        match self.target_sort {
            TargetSort::Value => {
                self.targets.sort_by(|a, b| {
                    let cmp = b.estimated_cargo_value.partial_cmp(&a.estimated_cargo_value);
                    if asc { cmp.map(|c| c.reverse()) } else { cmp }.unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            TargetSort::Threat => {
                self.targets.sort_by(|a, b| {
                    let cmp = b.likely_ship.threat_level.cmp(&a.likely_ship.threat_level);
                    if asc { cmp.reverse() } else { cmp }
                });
            }
            TargetSort::Ship => {
                self.targets.sort_by(|a, b| {
                    let cmp = a.likely_ship.name.cmp(&b.likely_ship.name);
                    if asc { cmp } else { cmp.reverse() }
                });
            }
            TargetSort::Commodity => {
                self.targets.sort_by(|a, b| {
                    let cmp = a.commodity.cmp(&b.commodity);
                    if asc { cmp } else { cmp.reverse() }
                });
            }
        }
    }

    fn sort_routes(&mut self) {
        let asc = self.sort_asc;
        match self.route_sort {
            RouteSort::Profit => {
                self.routes.sort_by(|a, b| {
                    let cmp = b.profit_per_scu.partial_cmp(&a.profit_per_scu);
                    if asc { cmp.map(|c| c.reverse()) } else { cmp }.unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            RouteSort::Value => {
                self.routes.sort_by(|a, b| {
                    let cmp = b.estimated_haul_value.partial_cmp(&a.estimated_haul_value);
                    if asc { cmp.map(|c| c.reverse()) } else { cmp }.unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            RouteSort::Commodity => {
                self.routes.sort_by(|a, b| {
                    let cmp = a.commodity.cmp(&b.commodity);
                    if asc { cmp } else { cmp.reverse() }
                });
            }
        }
    }
}

/// Infer system name from a location string.
fn infer_system(location: &str) -> String {
    let loc_lower = location.to_lowercase();

    // Stanton system locations
    let stanton_locations = [
        "hurston", "crusader", "arccorp", "microtech",
        "lorville", "orison", "area18", "new babbage",
        "port olisar", "everus", "baijini", "tressler",
        "grim hex", "levski",
        "hur-l", "cru-l", "arc-l", "mic-l",
        "arial", "aberdeen", "magda", "ita",
        "cellin", "daymar", "yela",
        "lyria", "wala",
        "calliope", "clio", "euterpe",
    ];

    for loc in stanton_locations {
        if loc_lower.contains(loc) {
            return "Stanton".to_string();
        }
    }

    // Pyro system locations
    let pyro_locations = ["pyro", "ruin station", "checkmate"];
    for loc in pyro_locations {
        if loc_lower.contains(loc) {
            return "Pyro".to_string();
        }
    }

    "Stanton".to_string() // Default
}

/// Build static map locations for a system.
/// Uses angular positions to spread planets around the star for visualization.
fn build_map_locations(system: &str) -> Vec<MapLocation> {
    use std::f64::consts::PI;

    let mut locations = Vec::new();

    match system {
        "Stanton" => {
            // Star at center
            locations.push(MapLocation {
                name: "Stanton".to_string(),
                angle: 0.0,
                orbital_radius: 0.0,
                loc_type: MapLocationType::Star,
                parent: None,
            });

            // Planets - spread around the star at different angles
            // Hurston: ~45 degrees (inner)
            locations.push(MapLocation {
                name: "Hurston".to_string(),
                angle: PI * 0.25, // 45 degrees
                orbital_radius: 12.85,
                loc_type: MapLocationType::Planet,
                parent: None,
            });

            // Crusader: ~135 degrees
            locations.push(MapLocation {
                name: "Crusader".to_string(),
                angle: PI * 0.75, // 135 degrees
                orbital_radius: 18.96,
                loc_type: MapLocationType::Planet,
                parent: None,
            });

            // ArcCorp: ~225 degrees (similar orbit to Crusader)
            locations.push(MapLocation {
                name: "ArcCorp".to_string(),
                angle: PI * 1.25, // 225 degrees
                orbital_radius: 18.59,
                loc_type: MapLocationType::Planet,
                parent: None,
            });

            // microTech: ~315 degrees (outer)
            locations.push(MapLocation {
                name: "microTech".to_string(),
                angle: PI * 1.75, // 315 degrees
                orbital_radius: 22.46,
                loc_type: MapLocationType::Planet,
                parent: None,
            });

            // Moons - small offsets from parent planet angle
            // Hurston moons
            for (i, name) in ["Arial", "Aberdeen", "Magda", "Ita"].iter().enumerate() {
                locations.push(MapLocation {
                    name: name.to_string(),
                    angle: PI * 0.25 + (i as f64 * PI * 0.5), // Spread around Hurston
                    orbital_radius: 1.5, // Moon orbital radius from planet
                    loc_type: MapLocationType::Moon,
                    parent: Some("Hurston".to_string()),
                });
            }

            // Crusader moons
            for (i, name) in ["Cellin", "Daymar", "Yela"].iter().enumerate() {
                locations.push(MapLocation {
                    name: name.to_string(),
                    angle: PI * 0.75 + (i as f64 * PI * 0.67), // Spread around Crusader
                    orbital_radius: 1.8,
                    loc_type: MapLocationType::Moon,
                    parent: Some("Crusader".to_string()),
                });
            }

            // ArcCorp moons
            for (i, name) in ["Lyria", "Wala"].iter().enumerate() {
                locations.push(MapLocation {
                    name: name.to_string(),
                    angle: PI * 1.25 + (i as f64 * PI), // Spread around ArcCorp
                    orbital_radius: 1.5,
                    loc_type: MapLocationType::Moon,
                    parent: Some("ArcCorp".to_string()),
                });
            }

            // microTech moons
            for (i, name) in ["Calliope", "Clio", "Euterpe"].iter().enumerate() {
                locations.push(MapLocation {
                    name: name.to_string(),
                    angle: PI * 1.75 + (i as f64 * PI * 0.67), // Spread around microTech
                    orbital_radius: 1.8,
                    loc_type: MapLocationType::Moon,
                    parent: Some("microTech".to_string()),
                });
            }

            // Stations - close to their parent bodies
            locations.push(MapLocation {
                name: "Port Olisar".to_string(),
                angle: PI * 0.75 + 0.1,
                orbital_radius: 0.5,
                loc_type: MapLocationType::Station,
                parent: Some("Crusader".to_string()),
            });
            locations.push(MapLocation {
                name: "Everus Harbor".to_string(),
                angle: PI * 0.25 + 0.1,
                orbital_radius: 0.5,
                loc_type: MapLocationType::Station,
                parent: Some("Hurston".to_string()),
            });
            locations.push(MapLocation {
                name: "Baijini Point".to_string(),
                angle: PI * 1.25 + 0.1,
                orbital_radius: 0.5,
                loc_type: MapLocationType::Station,
                parent: Some("ArcCorp".to_string()),
            });
            locations.push(MapLocation {
                name: "Port Tressler".to_string(),
                angle: PI * 1.75 + 0.1,
                orbital_radius: 0.5,
                loc_type: MapLocationType::Station,
                parent: Some("microTech".to_string()),
            });
            locations.push(MapLocation {
                name: "Grim HEX".to_string(),
                angle: PI * 0.75 + PI * 0.67 * 2.0 + 0.2, // Near Yela
                orbital_radius: 0.3,
                loc_type: MapLocationType::Station,
                parent: Some("Yela".to_string()),
            });
        }
        "Pyro" => {
            // Pyro star
            locations.push(MapLocation {
                name: "Pyro".to_string(),
                angle: 0.0,
                orbital_radius: 0.0,
                loc_type: MapLocationType::Star,
                parent: None,
            });

            // Planets spread around the star
            let pyro_planets = [
                ("Pyro I", 5.0, 0.0),
                ("Pyro II", 10.0, PI * 0.4),
                ("Pyro III", 18.0, PI * 0.8),
                ("Pyro IV", 28.0, PI * 1.2),
                ("Pyro V", 40.0, PI * 1.5),
                ("Pyro VI", 55.0, PI * 1.8),
            ];

            for (name, radius, angle) in pyro_planets {
                locations.push(MapLocation {
                    name: name.to_string(),
                    angle,
                    orbital_radius: radius,
                    loc_type: MapLocationType::Planet,
                    parent: None,
                });
            }

            // Stations
            locations.push(MapLocation {
                name: "Ruin Station".to_string(),
                angle: 0.1,
                orbital_radius: 0.5,
                loc_type: MapLocationType::Station,
                parent: Some("Pyro I".to_string()),
            });
            locations.push(MapLocation {
                name: "Checkmate".to_string(),
                angle: PI * 1.2 + 0.1,
                orbital_radius: 0.5,
                loc_type: MapLocationType::Station,
                parent: Some("Pyro IV".to_string()),
            });
        }
        _ => {}
    }

    locations
}

/// Load hotspots from routes by finding intersections.
fn load_hotspots(routes: &[HotRoute]) -> Vec<RouteIntersection> {
    use route_graph::{find_route_intersections, RouteSegment};

    // Convert hot routes to route segments for intersection analysis
    let segments: Vec<RouteSegment> = routes
        .iter()
        .filter_map(|route| {
            let origin_pos = route_graph::estimate_position(&route.origin)?;
            let dest_pos = route_graph::estimate_position(&route.destination)?;

            Some(RouteSegment {
                origin: origin_pos,
                destination: dest_pos,
                origin_name: route.origin.clone(),
                destination_name: route.destination.clone(),
                cargo_value: route.estimated_haul_value,
                commodity: route.commodity.clone(),
                ship_name: route.likely_ship.name.to_string(),
                threat_level: route.likely_ship.threat_level,
            })
        })
        .collect();

    // Find intersections with reasonable proximity (2 Mkm = 2 million km)
    let mut intersections = find_route_intersections(&segments, 2.0, 2);

    // Sort by route count (traffic) for display
    intersections.sort_by(|a, b| b.route_pair_count.cmp(&a.route_pair_count));

    // Take top 20 hotspots
    intersections.truncate(20);

    intersections
}

