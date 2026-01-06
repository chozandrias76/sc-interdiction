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
    use super::super::types::{RouteSort, TargetSort, View};
    use super::*;
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};

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
        }
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
}
