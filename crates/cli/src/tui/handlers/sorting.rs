//! Sorting logic for targets and routes.

use super::super::types::{RouteSort, TargetSort};
use super::super::App;

impl App {
    pub(crate) fn sort_targets(&mut self) {
        let asc = self.sort_asc;
        match self.target_sort {
            TargetSort::Value => {
                self.targets.sort_by(|a, b| {
                    let cmp = b
                        .estimated_cargo_value
                        .partial_cmp(&a.estimated_cargo_value);
                    if asc { cmp.map(|c| c.reverse()) } else { cmp }
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            TargetSort::Threat => {
                self.targets.sort_by(|a, b| {
                    let cmp = b.likely_ship.threat_level.cmp(&a.likely_ship.threat_level);
                    if asc {
                        cmp.reverse()
                    } else {
                        cmp
                    }
                });
            }
            TargetSort::Ship => {
                self.targets.sort_by(|a, b| {
                    let cmp = a.likely_ship.name.cmp(b.likely_ship.name);
                    if asc {
                        cmp
                    } else {
                        cmp.reverse()
                    }
                });
            }
            TargetSort::Commodity => {
                self.targets.sort_by(|a, b| {
                    let cmp = a.commodity.cmp(&b.commodity);
                    if asc {
                        cmp
                    } else {
                        cmp.reverse()
                    }
                });
            }
        }
    }

    pub(crate) fn sort_routes(&mut self) {
        let asc = self.sort_asc;
        match self.route_sort {
            RouteSort::Profit => {
                self.routes.sort_by(|a, b| {
                    let cmp = b.profit_per_scu.partial_cmp(&a.profit_per_scu);
                    if asc { cmp.map(|c| c.reverse()) } else { cmp }
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            RouteSort::Value => {
                self.routes.sort_by(|a, b| {
                    let cmp = b.estimated_haul_value.partial_cmp(&a.estimated_haul_value);
                    if asc { cmp.map(|c| c.reverse()) } else { cmp }
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            RouteSort::Commodity => {
                self.routes.sort_by(|a, b| {
                    let cmp = a.commodity.cmp(&b.commodity);
                    if asc {
                        cmp
                    } else {
                        cmp.reverse()
                    }
                });
            }
        }
    }
}
