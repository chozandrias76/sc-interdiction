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

            // Filtering
            KeyCode::Char('i') => self.toggle_inbound_filter(),
            KeyCode::Char('o') => self.toggle_outbound_filter(),
            KeyCode::Char('+') | KeyCode::Char('=') => self.increase_threat_filter(),
            KeyCode::Char('-') => self.decrease_threat_filter(),

            // Sorting
            KeyCode::Char('s') => self.cycle_sort(),
            KeyCode::Char('S') => self.toggle_sort_direction(),

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
}
