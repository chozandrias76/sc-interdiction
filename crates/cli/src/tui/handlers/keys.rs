// ! Key event handlers for the TUI application.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::super::types::View;
use super::super::App;

impl App {
    /// Handle a key event. Returns true if the application should quit.
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
            KeyCode::Left | KeyCode::Char('h') => self.handle_map_left(),
            KeyCode::Right | KeyCode::Char('l') => self.handle_map_right(),

            // Map zoom (z/Z or [/])
            KeyCode::Char('z') | KeyCode::Char('[') => self.zoom_out(),
            KeyCode::Char('Z') | KeyCode::Char(']') => self.zoom_in(),
            KeyCode::Char('0') => self.reset_zoom(),

            // Hotspot limit (n/N to decrease/increase, a for all)
            KeyCode::Char('n') => self.decrease_hotspot_limit(),
            KeyCode::Char('N') => self.increase_hotspot_limit(),
            KeyCode::Char('a') => self.toggle_all_hotspots(),

            // Wikelo filter (w to toggle)
            KeyCode::Char('w') => self.toggle_wikelo_filter(),

            // Filtering
            KeyCode::Char('i') => self.toggle_inbound_filter(),
            KeyCode::Char('o') => self.toggle_outbound_filter(),
            KeyCode::Char('+') | KeyCode::Char('=') => self.increase_threat_filter(),
            KeyCode::Char('-') => self.decrease_threat_filter(),

            // Sorting
            KeyCode::Char('s') => self.cycle_sort(),
            KeyCode::Char('S') => self.toggle_sort_direction(),

            // Detail expansion
            KeyCode::Enter => self.toggle_target_detail(),

            _ => {}
        }

        false
    }

    fn handle_map_left(&mut self) {
        if self.view == View::Map && self.map_selected > 0 {
            self.map_selected -= 1;
        }
    }

    fn handle_map_right(&mut self) {
        if self.view == View::Map {
            let max = self.visible_hotspot_count().saturating_sub(1);
            if self.map_selected < max {
                self.map_selected += 1;
            }
        }
    }

    fn zoom_out(&mut self) {
        if self.view == View::Map {
            self.map_zoom = (self.map_zoom * 0.8).max(0.2);
        }
    }

    fn zoom_in(&mut self) {
        if self.view == View::Map {
            self.map_zoom = (self.map_zoom * 1.25).min(3.0);
        }
    }

    fn reset_zoom(&mut self) {
        if self.view == View::Map {
            self.map_zoom = 1.0;
        }
    }

    fn decrease_hotspot_limit(&mut self) {
        if self.view == View::Map && self.hotspot_limit > 1 {
            self.hotspot_limit -= 1;
            // Adjust selection if it's now out of bounds
            if self.map_selected >= self.hotspot_limit {
                self.map_selected = self.hotspot_limit.saturating_sub(1);
            }
        }
    }

    fn increase_hotspot_limit(&mut self) {
        if self.view == View::Map {
            let max = self.hotspots.len();
            if self.hotspot_limit < max {
                self.hotspot_limit += 1;
            }
        }
    }

    fn toggle_all_hotspots(&mut self) {
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

    fn toggle_wikelo_filter(&mut self) {
        if self.view == View::Map {
            self.wikelo_filter = !self.wikelo_filter;
            self.map_selected = 0;
        }
    }

    fn toggle_inbound_filter(&mut self) {
        self.filter_inbound = !self.filter_inbound;
        if self.filter_inbound {
            self.filter_outbound = false;
        }
    }

    fn toggle_outbound_filter(&mut self) {
        self.filter_outbound = !self.filter_outbound;
        if self.filter_outbound {
            self.filter_inbound = false;
        }
    }

    fn increase_threat_filter(&mut self) {
        if self.min_threat < 10 {
            self.min_threat += 1;
        }
    }

    fn decrease_threat_filter(&mut self) {
        if self.min_threat > 0 {
            self.min_threat -= 1;
        }
    }

    fn cycle_sort(&mut self) {
        use super::super::types::{RouteSort, TargetSort};

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

    fn toggle_sort_direction(&mut self) {
        self.sort_asc = !self.sort_asc;
        self.sort_targets();
        self.sort_routes();
    }

    fn toggle_target_detail(&mut self) {
        if self.view != View::Targets {
            return;
        }

        // Only toggle if selected target has wikelo_flag
        let has_wikelo = self
            .filtered_targets()
            .nth(self.selected)
            .is_some_and(|t| t.wikelo_flag.is_some());

        if has_wikelo {
            self.target_detail_expanded = !self.target_detail_expanded;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::text::ScrollState;
    use super::super::super::types::{RouteSort, TargetSort, View};
    use super::*;
    use intel::WikieloIntel;

    fn test_app() -> App {
        App {
            view: View::Map,
            location: "Test".to_string(),
            targets: Vec::new(),
            routes: Vec::new(),
            hotspots: Vec::new(),
            map_locations: Vec::new(),
            map_system: "Stanton".to_string(),
            map_selected: 0,
            map_zoom: 1.0,
            hotspot_limit: 3,
            selected: 0,
            filter_inbound: false,
            filter_outbound: false,
            min_threat: 0,
            target_sort: TargetSort::Value,
            route_sort: RouteSort::Profit,
            sort_asc: false,
            loading: false,
            error: None,
            status: String::new(),
            scroll: ScrollState::new(),
            detail_expanded: false,
            detail_selected: 0,
            target_detail_expanded: false,
            wikelo_intel: WikieloIntel::from_static(),
            wikelo_filter: false,
        }
    }

    // ==================== Wikelo Filter Tests ====================

    #[test]
    fn test_toggle_wikelo_filter_enables() {
        let mut app = test_app();
        app.wikelo_filter = false;
        app.map_selected = 5;

        app.toggle_wikelo_filter();

        assert!(app.wikelo_filter);
        assert_eq!(app.map_selected, 0, "map_selected should reset to 0");
    }

    #[test]
    fn test_toggle_wikelo_filter_disables() {
        let mut app = test_app();
        app.wikelo_filter = true;

        app.toggle_wikelo_filter();

        assert!(!app.wikelo_filter);
    }

    #[test]
    fn test_toggle_wikelo_filter_only_on_map_view() {
        let mut app = test_app();
        app.wikelo_filter = false;
        app.view = View::Targets;

        app.toggle_wikelo_filter();

        assert!(
            !app.wikelo_filter,
            "wikelo_filter should remain unchanged on non-Map view"
        );
    }

    // ==================== Zoom Tests ====================

    #[test]
    fn test_zoom_in_increases_zoom() {
        let mut app = test_app();
        app.map_zoom = 1.0;

        app.zoom_in();

        assert!((app.map_zoom - 1.25).abs() < 0.001);
    }

    #[test]
    fn test_zoom_in_caps_at_max() {
        let mut app = test_app();
        app.map_zoom = 3.0;

        app.zoom_in();

        assert!((app.map_zoom - 3.0).abs() < 0.001, "zoom should cap at 3.0");
    }

    #[test]
    fn test_zoom_out_decreases_zoom() {
        let mut app = test_app();
        app.map_zoom = 1.0;

        app.zoom_out();

        assert!((app.map_zoom - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_zoom_out_caps_at_min() {
        let mut app = test_app();
        app.map_zoom = 0.2;

        app.zoom_out();

        assert!((app.map_zoom - 0.2).abs() < 0.001, "zoom should cap at 0.2");
    }

    #[test]
    fn test_reset_zoom() {
        let mut app = test_app();
        app.map_zoom = 2.5;

        app.reset_zoom();

        assert!((app.map_zoom - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_zoom_only_on_map_view() {
        let mut app = test_app();
        app.view = View::Targets;
        app.map_zoom = 1.0;

        app.zoom_in();
        assert!(
            (app.map_zoom - 1.0).abs() < 0.001,
            "zoom_in should not change zoom on non-Map view"
        );

        app.zoom_out();
        assert!(
            (app.map_zoom - 1.0).abs() < 0.001,
            "zoom_out should not change zoom on non-Map view"
        );

        app.map_zoom = 2.0;
        app.reset_zoom();
        assert!(
            (app.map_zoom - 2.0).abs() < 0.001,
            "reset_zoom should not change zoom on non-Map view"
        );
    }

    // ==================== Hotspot Limit Tests ====================

    /// Create an app with mock hotspots for hotspot limit testing.
    fn test_app_with_hotspots(count: usize) -> App {
        use route_graph::{JumpInstruction, Point3D, RouteIntersection};

        let mut app = test_app();
        app.hotspots = (0..count)
            .map(|i| RouteIntersection {
                position: Point3D::new(0.0, 0.0, 0.0),
                name: format!("Hotspot {}", i),
                system: "Stanton".to_string(),
                is_cross_system: false,
                intersecting_routes: Vec::new(),
                total_cargo_value: 1_000_000.0,
                route_pair_count: 2,
                avg_threat_level: 3.0,
                interdiction_value: 500_000.0,
                suggested_tactics: "Test".to_string(),
                jump_to: JumpInstruction {
                    destination: "Test".to_string(),
                    exit_at_mm: 1000,
                    distance_from_dest_mm: 500,
                    lateral_offset_km: 10.0,
                    alternatives: Vec::new(),
                },
            })
            .collect();
        app
    }

    #[test]
    fn test_decrease_hotspot_limit() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 3;

        app.decrease_hotspot_limit();

        assert_eq!(app.hotspot_limit, 2);
    }

    #[test]
    fn test_decrease_hotspot_limit_min() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 1;

        app.decrease_hotspot_limit();

        assert_eq!(app.hotspot_limit, 1, "hotspot_limit should not go below 1");
    }

    #[test]
    fn test_decrease_hotspot_limit_adjusts_selection() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 3;
        app.map_selected = 2; // At the edge

        app.decrease_hotspot_limit();

        assert_eq!(app.hotspot_limit, 2);
        assert_eq!(
            app.map_selected, 1,
            "map_selected should adjust when out of bounds"
        );
    }

    #[test]
    fn test_increase_hotspot_limit() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 3;

        app.increase_hotspot_limit();

        assert_eq!(app.hotspot_limit, 4);
    }

    #[test]
    fn test_increase_hotspot_limit_max() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 5;

        app.increase_hotspot_limit();

        assert_eq!(
            app.hotspot_limit, 5,
            "hotspot_limit should not exceed hotspots.len()"
        );
    }

    #[test]
    fn test_toggle_all_hotspots_to_all() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 2;

        app.toggle_all_hotspots();

        assert_eq!(
            app.hotspot_limit, 5,
            "toggle_all should set limit to hotspots.len()"
        );
    }

    #[test]
    fn test_toggle_all_hotspots_to_one() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 5; // Already showing all
        app.map_selected = 3;

        app.toggle_all_hotspots();

        assert_eq!(
            app.hotspot_limit, 1,
            "toggle_all should set limit to 1 when already showing all"
        );
        assert_eq!(app.map_selected, 0, "map_selected should reset to 0");
    }

    #[test]
    fn test_hotspot_limit_only_on_map_view() {
        let mut app = test_app_with_hotspots(5);
        app.view = View::Targets;
        app.hotspot_limit = 3;

        app.decrease_hotspot_limit();
        assert_eq!(
            app.hotspot_limit, 3,
            "decrease should not work on non-Map view"
        );

        app.increase_hotspot_limit();
        assert_eq!(
            app.hotspot_limit, 3,
            "increase should not work on non-Map view"
        );

        app.toggle_all_hotspots();
        assert_eq!(
            app.hotspot_limit, 3,
            "toggle_all should not work on non-Map view"
        );
    }

    // ==================== Filter Tests ====================

    #[test]
    fn test_toggle_inbound_filter() {
        let mut app = test_app();
        app.filter_inbound = false;

        app.toggle_inbound_filter();

        assert!(app.filter_inbound);
    }

    #[test]
    fn test_toggle_inbound_disables_outbound() {
        let mut app = test_app();
        app.filter_outbound = true;
        app.filter_inbound = false;

        app.toggle_inbound_filter();

        assert!(app.filter_inbound);
        assert!(
            !app.filter_outbound,
            "outbound filter should be disabled when inbound is enabled"
        );
    }

    #[test]
    fn test_toggle_outbound_filter() {
        let mut app = test_app();
        app.filter_outbound = false;

        app.toggle_outbound_filter();

        assert!(app.filter_outbound);
    }

    #[test]
    fn test_toggle_outbound_disables_inbound() {
        let mut app = test_app();
        app.filter_inbound = true;
        app.filter_outbound = false;

        app.toggle_outbound_filter();

        assert!(app.filter_outbound);
        assert!(
            !app.filter_inbound,
            "inbound filter should be disabled when outbound is enabled"
        );
    }

    #[test]
    fn test_threat_filter_increase() {
        let mut app = test_app();
        app.min_threat = 0;

        app.increase_threat_filter();

        assert_eq!(app.min_threat, 1);
    }

    #[test]
    fn test_threat_filter_decrease() {
        let mut app = test_app();
        app.min_threat = 5;

        app.decrease_threat_filter();

        assert_eq!(app.min_threat, 4);
    }

    #[test]
    fn test_threat_filter_max_bound() {
        let mut app = test_app();
        app.min_threat = 10;

        app.increase_threat_filter();

        assert_eq!(app.min_threat, 10, "threat filter should cap at 10");
    }

    #[test]
    fn test_threat_filter_min_bound() {
        let mut app = test_app();
        app.min_threat = 0;

        app.decrease_threat_filter();

        assert_eq!(app.min_threat, 0, "threat filter should not go below 0");
    }

    // ==================== Map Navigation Tests ====================

    #[test]
    fn test_handle_map_left() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 5;
        app.map_selected = 2;

        app.handle_map_left();

        assert_eq!(app.map_selected, 1);
    }

    #[test]
    fn test_handle_map_left_at_zero() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 5;
        app.map_selected = 0;

        app.handle_map_left();

        assert_eq!(app.map_selected, 0, "map_selected should stay at 0");
    }

    #[test]
    fn test_handle_map_right() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 5;
        app.map_selected = 1;

        app.handle_map_right();

        assert_eq!(app.map_selected, 2);
    }

    #[test]
    fn test_handle_map_right_at_max() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 5;
        app.map_selected = 4; // max is 4 (len - 1)

        app.handle_map_right();

        assert_eq!(app.map_selected, 4, "map_selected should stay at max");
    }

    #[test]
    fn test_map_navigation_only_on_map_view() {
        let mut app = test_app_with_hotspots(5);
        app.hotspot_limit = 5;
        app.view = View::Targets;
        app.map_selected = 2;

        app.handle_map_left();
        assert_eq!(
            app.map_selected, 2,
            "map_left should not work on non-Map view"
        );

        app.handle_map_right();
        assert_eq!(
            app.map_selected, 2,
            "map_right should not work on non-Map view"
        );
    }

    // ==================== Target Detail Toggle Tests ====================

    /// Create an app with targets for target detail testing.
    fn test_app_with_targets() -> App {
        use intel::{
            CargoShip, ItemCategory, ShipRole, SourceFlag, TargetPrediction, TrafficDirection,
            WikieloItemSummary,
        };

        let mut app = test_app();
        app.view = View::Targets;

        let mock_ship = CargoShip {
            name: "Hull C".to_string(),
            manufacturer: "MISC".to_string(),
            cargo_scu: 4608,
            crew_size: 1,
            threat_level: 2,
            ship_value_uec: 3_500_000,
            requires_freight_elevator: true,
            quantum_fuel_capacity: 1850.0,
            hydrogen_fuel_capacity: 40000.0,
            qt_drive_size: 3,
            role: ShipRole::Cargo,
            mining_capacity_scu: None,
            mass_kg: Some(1_200_000.0),
        };

        // Target with wikelo_flag
        let target_with_wikelo = TargetPrediction {
            direction: TrafficDirection::Departing,
            commodity: "Titanium".to_string(),
            likely_ship: mock_ship.clone(),
            estimated_cargo_value: 1_000_000.0,
            destination: "ArcCorp".to_string(),
            wikelo_flag: Some(SourceFlag {
                location: "Lyria".to_string(),
                item_count: 3,
                top_items: vec![WikieloItemSummary {
                    name: "Test Item".to_string(),
                    category: ItemCategory::CreaturePart,
                    estimated_value: Some(5000),
                }],
                has_high_value: true,
            }),
        };

        // Target without wikelo_flag
        let target_without_wikelo = TargetPrediction {
            direction: TrafficDirection::Arriving,
            commodity: "Agricium".to_string(),
            likely_ship: mock_ship,
            estimated_cargo_value: 500_000.0,
            destination: "Crusader".to_string(),
            wikelo_flag: None,
        };

        app.targets = vec![target_with_wikelo, target_without_wikelo];
        app
    }

    #[test]
    fn test_toggle_target_detail_with_wikelo() {
        let mut app = test_app_with_targets();
        app.selected = 0; // First target has wikelo_flag
        app.target_detail_expanded = false;

        app.toggle_target_detail();

        assert!(
            app.target_detail_expanded,
            "detail should expand for target with wikelo_flag"
        );
    }

    #[test]
    fn test_toggle_target_detail_without_wikelo() {
        let mut app = test_app_with_targets();
        app.selected = 1; // Second target has no wikelo_flag
        app.target_detail_expanded = false;

        app.toggle_target_detail();

        assert!(
            !app.target_detail_expanded,
            "detail should not expand for target without wikelo_flag"
        );
    }

    #[test]
    fn test_toggle_target_detail_only_on_targets_view() {
        let mut app = test_app_with_targets();
        app.selected = 0;
        app.target_detail_expanded = false;
        app.view = View::Map;

        app.toggle_target_detail();

        assert!(
            !app.target_detail_expanded,
            "detail should not expand on non-Targets view"
        );
    }
}
