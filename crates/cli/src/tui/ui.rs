//! UI rendering for the TUI.

use super::app::App;
use super::types::View;
use super::views::{render_help, render_map, render_routes, render_targets};
use super::widgets::{render_header, render_status_bar};
use ratatui::prelude::*;

/// Render the application.
#[allow(clippy::indexing_slicing)] // Layout guarantees chunks has required indices
pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header/tabs
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status bar
        ])
        .split(frame.area());

    render_header(frame, app, chunks[0]);

    match app.view {
        View::Targets => render_targets(frame, app, chunks[1]),
        View::Routes => render_routes(frame, app, chunks[1]),
        View::Map => render_map(frame, app, chunks[1]),
        View::Help => render_help(frame, chunks[1]),
    }

    render_status_bar(frame, app, chunks[2]);
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]
    #![allow(clippy::indexing_slicing)]

    use super::super::text::ScrollState;
    use super::super::types::{MapLocation, MapLocationType, RouteSort, TargetSort, View};
    use super::*;
    use insta::assert_snapshot;
    use intel::{ItemCategory, SourceFlag, WikieloItemSummary};
    use ratatui::{backend::TestBackend, Terminal};
    use route_graph::{
        AltJumpInstruction, IntersectingRoute, JumpInstruction, Point3D, RouteIntersection,
    };

    /// Create a test app with sample data for snapshot testing.
    fn test_app() -> App {
        use intel::{CargoShip, HotRoute, TargetPrediction, TrafficDirection};

        let ship = CargoShip {
            name: "Caterpillar".to_string(),
            manufacturer: "Drake".to_string(),
            cargo_scu: 576,
            crew_size: 4,
            threat_level: 3,
            ship_value_uec: 600_000,
            requires_freight_elevator: false,
            quantum_fuel_capacity: 10000.0,
            hydrogen_fuel_capacity: 1800.0,
            qt_drive_size: 3,
            mass_kg: Some(200_000.0),
            mining_capacity_scu: None,
            role: intel::ShipRole::Cargo,
        };

        let targets = vec![
            TargetPrediction {
                commodity: "Quantanium".to_string(),
                destination: "Area18 TDD".to_string(),
                likely_ship: ship.clone(),
                direction: TrafficDirection::Departing,
                estimated_cargo_value: 1_250_000.0,
                wikelo_flag: None,
            },
            TargetPrediction {
                commodity: "Laranite".to_string(),
                destination: "Port Olisar".to_string(),
                likely_ship: CargoShip {
                    name: "C2 Hercules".to_string(),
                    manufacturer: "Crusader".to_string(),
                    cargo_scu: 696,
                    crew_size: 3,
                    threat_level: 8,
                    ship_value_uec: 4_800_000,
                    requires_freight_elevator: true,
                    quantum_fuel_capacity: 10000.0,
                    hydrogen_fuel_capacity: 2500.0,
                    qt_drive_size: 3,
                    mass_kg: Some(300_000.0),
                    mining_capacity_scu: None,
                    role: intel::ShipRole::Cargo,
                },
                direction: TrafficDirection::Arriving,
                estimated_cargo_value: 980_000.0,
                wikelo_flag: None,
            },
        ];

        let routes = vec![HotRoute {
            commodity: "Quantanium".to_string(),
            commodity_code: "QUAN".to_string(),
            origin: "HDMS-Bezdek".to_string(),
            destination: "Area18 TDD".to_string(),
            origin_system: Some("Stanton".to_string()),
            destination_system: Some("Stanton".to_string()),
            profit_per_scu: 88.5,
            available_scu: 576.0,
            likely_ship: ship,
            estimated_haul_value: 51_000.0,
            risk_score: 75.0,
            distance_mkm: 12.5,
            fuel_sufficient: true,
            fuel_required: 1000.0,
            wikelo_score: None,
            wikelo_items: Vec::new(),
        }];

        App {
            view: View::Targets,
            location: "Crusader".to_string(),
            targets,
            routes,
            hotspots: Vec::new(),
            map_locations: Vec::new(),
            map_system: "Stanton".to_string(),
            map_selected: 0,
            map_zoom: 1.0,
            hotspot_limit: 5,
            selected: 0,
            filter_inbound: false,
            filter_outbound: false,
            min_threat: 0,
            target_sort: TargetSort::Value,
            route_sort: RouteSort::Profit,
            sort_asc: false,
            loading: false,
            error: None,
            status: "Ready".to_string(),
            scroll: ScrollState::new(),
            detail_expanded: false,
            detail_selected: 0,
            target_detail_expanded: false,
            wikelo_intel: intel::WikieloIntel::from_static(),
            wikelo_filter: false,
        }
    }

    /// Create a test app with Wikelo data for snapshot testing Wikelo-enabled views.
    fn test_app_with_wikelo() -> App {
        let mut app = test_app();

        // Target 0: HDMS-Bezdek with high-value Wikelo items
        app.targets[0].wikelo_flag = Some(SourceFlag {
            location: "HDMS-Bezdek".to_string(),
            item_count: 3,
            top_items: vec![
                WikieloItemSummary {
                    name: "Irradiated Valakkar Fang".to_string(),
                    category: ItemCategory::CreaturePart,
                    estimated_value: Some(75_000),
                },
                WikieloItemSummary {
                    name: "Quantanium".to_string(),
                    category: ItemCategory::MinedMaterial,
                    estimated_value: Some(50_000),
                },
                WikieloItemSummary {
                    name: "Carinite".to_string(),
                    category: ItemCategory::MinedMaterial,
                    estimated_value: Some(25_000),
                },
            ],
            has_high_value: true,
        });

        // Target 1: Port Olisar with low-value Wikelo items
        app.targets[1].wikelo_flag = Some(SourceFlag {
            location: "Port Olisar".to_string(),
            item_count: 1,
            top_items: vec![WikieloItemSummary {
                name: "Council Scrip".to_string(),
                category: ItemCategory::MissionCurrency,
                estimated_value: None,
            }],
            has_high_value: false,
        });

        // Route 0: Add Wikelo score and items
        app.routes[0].wikelo_score = Some(85.0);
        app.routes[0].wikelo_items = vec!["Quantanium".to_string(), "Carinite".to_string()];

        app
    }

    /// Create a test app with many Wikelo items to test overflow indicator.
    fn test_app_with_many_wikelo_items() -> App {
        let mut app = test_app();

        // Target 0: Location with 8 items (only 5 shown, overflow indicator for 3 more)
        app.targets[0].wikelo_flag = Some(SourceFlag {
            location: "Grim HEX".to_string(),
            item_count: 8,
            top_items: vec![
                WikieloItemSummary {
                    name: "Irradiated Valakkar Fang".to_string(),
                    category: ItemCategory::CreaturePart,
                    estimated_value: Some(75_000),
                },
                WikieloItemSummary {
                    name: "Quantanium".to_string(),
                    category: ItemCategory::MinedMaterial,
                    estimated_value: Some(50_000),
                },
                WikieloItemSummary {
                    name: "Carinite".to_string(),
                    category: ItemCategory::MinedMaterial,
                    estimated_value: Some(25_000),
                },
                WikieloItemSummary {
                    name: "Hadanite".to_string(),
                    category: ItemCategory::MinedMaterial,
                    estimated_value: Some(20_000),
                },
                WikieloItemSummary {
                    name: "Council Scrip".to_string(),
                    category: ItemCategory::MissionCurrency,
                    estimated_value: None,
                },
            ],
            has_high_value: true,
        });

        app
    }

    /// Create a test app with map data for snapshot testing map view.
    fn test_app_with_map_data() -> App {
        let mut app = test_app_with_wikelo();
        app.view = View::Map;
        app.map_system = "Stanton".to_string();

        // Populate map_locations with Wikelo data
        app.map_locations = vec![
            MapLocation {
                name: "Stanton".to_string(),
                x: 0.0,
                y: 0.0,
                loc_type: MapLocationType::Star,
                parent: None,
                wikelo_items: Vec::new(),
                wikelo_high_value: false,
            },
            MapLocation {
                name: "Crusader".to_string(),
                x: 5.0,
                y: 3.0,
                loc_type: MapLocationType::Planet,
                parent: None,
                wikelo_items: vec!["Quantanium".to_string()],
                wikelo_high_value: false,
            },
            MapLocation {
                name: "Port Olisar".to_string(),
                x: 5.5,
                y: 3.2,
                loc_type: MapLocationType::Station,
                parent: Some("Crusader".to_string()),
                wikelo_items: Vec::new(),
                wikelo_high_value: false,
            },
            MapLocation {
                name: "Hurston".to_string(),
                x: -4.0,
                y: -2.0,
                loc_type: MapLocationType::Planet,
                parent: None,
                wikelo_items: vec![
                    "Irradiated Valakkar Fang".to_string(),
                    "Council Scrip".to_string(),
                ],
                wikelo_high_value: true,
            },
        ];

        // Populate hotspots with RouteIntersection entries
        app.hotspots = vec![
            RouteIntersection {
                position: Point3D::new(2.5, 1.5, 0.0),
                name: "Port Olisar".to_string(),
                system: "Stanton".to_string(),
                is_cross_system: false,
                intersecting_routes: vec![
                    IntersectingRoute {
                        origin: "HDMS-Bezdek".to_string(),
                        destination: "Port Olisar".to_string(),
                        commodity: "Quantanium".to_string(),
                        cargo_value: 500_000.0,
                        ship_name: "Caterpillar".to_string(),
                        threat_level: 3,
                        interdiction_value: 166_666.67,
                    },
                    IntersectingRoute {
                        origin: "Lorville".to_string(),
                        destination: "Area18".to_string(),
                        commodity: "Laranite".to_string(),
                        cargo_value: 300_000.0,
                        ship_name: "C2 Hercules".to_string(),
                        threat_level: 2,
                        interdiction_value: 150_000.0,
                    },
                    IntersectingRoute {
                        origin: "New Babbage".to_string(),
                        destination: "Orison".to_string(),
                        commodity: "WiDoW".to_string(),
                        cargo_value: 800_000.0,
                        ship_name: "Hull C".to_string(),
                        threat_level: 1,
                        interdiction_value: 800_000.0,
                    },
                ],
                total_cargo_value: 1_600_000.0,
                route_pair_count: 3,
                avg_threat_level: 2.0,
                interdiction_value: 372_222.22,
                suggested_tactics: "Easy pickings - solo Mantis can handle most targets"
                    .to_string(),
                jump_to: JumpInstruction {
                    destination: "Crusader".to_string(),
                    exit_at_mm: 15000,
                    distance_from_dest_mm: 15000,
                    lateral_offset_km: 12.5,
                    alternatives: vec![AltJumpInstruction {
                        destination: "Port Olisar".to_string(),
                        exit_at_mm: 5000,
                    }],
                },
            },
            RouteIntersection {
                position: Point3D::new(-2.0, -1.0, 0.0),
                name: "Crusader Gateway".to_string(),
                system: "Stanton".to_string(),
                is_cross_system: false,
                intersecting_routes: vec![
                    IntersectingRoute {
                        origin: "Grim HEX".to_string(),
                        destination: "Hurston".to_string(),
                        commodity: "SLAM".to_string(),
                        cargo_value: 200_000.0,
                        ship_name: "Freelancer MAX".to_string(),
                        threat_level: 5,
                        interdiction_value: 40_000.0,
                    },
                    IntersectingRoute {
                        origin: "ArcCorp Mining Area 045".to_string(),
                        destination: "Lorville".to_string(),
                        commodity: "Hadanite".to_string(),
                        cargo_value: 400_000.0,
                        ship_name: "RAFT".to_string(),
                        threat_level: 2,
                        interdiction_value: 200_000.0,
                    },
                ],
                total_cargo_value: 600_000.0,
                route_pair_count: 2,
                avg_threat_level: 3.5,
                interdiction_value: 120_000.0,
                suggested_tactics: "Mixed targets - bring a wingman for armed haulers".to_string(),
                jump_to: JumpInstruction {
                    destination: "Hurston".to_string(),
                    exit_at_mm: 8000,
                    distance_from_dest_mm: 8000,
                    lateral_offset_km: 25.0,
                    alternatives: Vec::new(),
                },
            },
        ];

        app.hotspot_limit = 2;
        app
    }

    #[test]
    fn test_render_targets_view() {
        let mut app = test_app();
        app.view = View::Targets;

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_routes_view() {
        let mut app = test_app();
        app.view = View::Routes;

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_help_view() {
        let mut app = test_app();
        app.view = View::Help;

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_with_selection() {
        let mut app = test_app();
        app.view = View::Targets;
        app.selected = 1; // Select second row

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_with_error() {
        let mut app = test_app();
        app.error = Some("Connection failed: timeout".to_string());

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_loading_state() {
        let mut app = test_app();
        app.loading = true;
        app.status = "Loading data...".to_string();

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_targets_with_wikelo() {
        let mut app = test_app_with_wikelo();
        app.view = View::Targets;

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_routes_with_wikelo() {
        let mut app = test_app_with_wikelo();
        app.view = View::Routes;

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_targets_detail_expanded_high_value() {
        let mut app = test_app_with_wikelo();
        app.view = View::Targets;
        app.selected = 0; // First target has high-value Wikelo
        app.target_detail_expanded = true;

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_targets_detail_expanded_regular() {
        let mut app = test_app_with_wikelo();
        app.view = View::Targets;
        app.selected = 1; // Second target has regular Wikelo (not high-value)
        app.target_detail_expanded = true;

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_targets_detail_expanded_many_items() {
        let mut app = test_app_with_many_wikelo_items();
        app.view = View::Targets;
        app.selected = 0; // First target has 8 items
        app.target_detail_expanded = true;

        // Use taller terminal (35 lines) to show all 5 items + overflow indicator
        let mut terminal = Terminal::new(TestBackend::new(100, 35)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_map_view() {
        let mut app = test_app_with_map_data();

        // Map view needs more height (100x30) to show canvas and hotspot panels
        let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_map_view_wikelo_filter() {
        let mut app = test_app_with_map_data();
        app.wikelo_filter = true;

        // Map view needs more height (100x30)
        let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }
}
