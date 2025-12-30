//! Navigation handlers for list views.

use super::super::types::View;
use super::super::App;

impl App {
    pub(crate) fn prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub(crate) fn next(&mut self) {
        let max = self.filtered_len().saturating_sub(1);
        if self.selected < max {
            self.selected += 1;
        }
    }

    pub(crate) fn page_up(&mut self) {
        self.selected = self.selected.saturating_sub(20);
    }

    pub(crate) fn page_down(&mut self) {
        let max = self.filtered_len().saturating_sub(1);
        self.selected = (self.selected + 20).min(max);
    }

    pub(crate) fn home(&mut self) {
        self.selected = 0;
    }

    pub(crate) fn end(&mut self) {
        self.selected = self.filtered_len().saturating_sub(1);
    }

    pub(crate) fn next_view(&mut self) {
        self.view = match self.view {
            View::Targets => View::Routes,
            View::Routes => View::Map,
            View::Map => View::Targets,
            View::Help => View::Targets,
        };
        self.selected = 0;
    }

    pub(crate) fn filtered_len(&self) -> usize {
        match self.view {
            View::Targets => self.filtered_targets().count(),
            View::Routes => self.routes.len(),
            View::Map => self.visible_hotspot_count(),
            View::Help => 0,
        }
    }
}
