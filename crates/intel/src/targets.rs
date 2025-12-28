//! Target prediction based on trade route analysis.

use api_client::{TradeRoute, UexClient};
use ordered_float::OrderedFloat;
use route_graph::{Chokepoint, RouteGraph, find_chokepoints};
use serde::{Deserialize, Serialize};
use crate::ships::{CargoShip, estimate_ship_for_route};

/// Analyzes trade data to predict targets.
pub struct TargetAnalyzer {
    uex: UexClient,
}

impl TargetAnalyzer {
    /// Create a new target analyzer.
    pub fn new(uex: UexClient) -> Self {
        Self { uex }
    }

    /// Get hot trade routes sorted by profitability.
    pub async fn get_hot_routes(&self, limit: usize) -> api_client::Result<Vec<HotRoute>> {
        let routes = self.uex.get_trade_routes().await?;

        let mut hot_routes: Vec<HotRoute> = routes
            .into_iter()
            .filter(|r| r.profit_per_unit > 0.0 && r.scu_origin > 0.0)
            .map(|r| {
                let likely_ship = estimate_ship_for_route(&r);
                let estimated_value = r.profit_for_scu(likely_ship.cargo_scu as f64);

                HotRoute {
                    commodity: r.commodity_name.clone(),
                    commodity_code: r.commodity_code.clone(),
                    origin: r.terminal_origin_name.clone(),
                    destination: r.terminal_destination_name.clone(),
                    profit_per_scu: r.profit_per_unit,
                    available_scu: r.max_profitable_scu(),
                    likely_ship,
                    estimated_haul_value: estimated_value,
                    risk_score: calculate_risk_score(&r),
                }
            })
            .collect();

        // Sort by estimated haul value
        hot_routes.sort_by_key(|r| std::cmp::Reverse(OrderedFloat(r.estimated_haul_value)));
        hot_routes.truncate(limit);

        Ok(hot_routes)
    }

    /// Predict likely targets at a specific location.
    pub async fn predict_targets_at(
        &self,
        location: &str,
    ) -> api_client::Result<Vec<TargetPrediction>> {
        let routes = self.uex.get_trade_routes().await?;

        let predictions: Vec<TargetPrediction> = routes
            .into_iter()
            .filter(|r| {
                r.terminal_origin_name.contains(location)
                    || r.terminal_destination_name.contains(location)
            })
            .map(|r| {
                let is_departing = r.terminal_origin_name.contains(location);
                let likely_ship = estimate_ship_for_route(&r);
                let estimated_cargo_value = r.profit_for_scu(likely_ship.cargo_scu as f64)
                    + (r.price_origin * likely_ship.cargo_scu as f64);

                TargetPrediction {
                    direction: if is_departing {
                        TrafficDirection::Departing
                    } else {
                        TrafficDirection::Arriving
                    },
                    commodity: r.commodity_name,
                    likely_ship,
                    estimated_cargo_value,
                    destination: if is_departing {
                        r.terminal_destination_name
                    } else {
                        r.terminal_origin_name
                    },
                }
            })
            .collect();

        Ok(predictions)
    }

    /// Find best interdiction points based on current trade data.
    pub async fn find_interdiction_points(
        &self,
        graph: &RouteGraph,
        top_n: usize,
    ) -> api_client::Result<Vec<Chokepoint>> {
        let routes = self.uex.get_trade_routes().await?;

        let trade_routes: Vec<_> = routes
            .iter()
            .map(|r| {
                (
                    r.terminal_origin_name.clone(),
                    r.terminal_destination_name.clone(),
                    r.profit_per_unit,
                )
            })
            .collect();

        let mut chokepoints = find_chokepoints(graph, &trade_routes);
        chokepoints.truncate(top_n);

        Ok(chokepoints)
    }
}

/// A profitable trade route.
#[derive(Debug, Clone, Serialize)]
pub struct HotRoute {
    pub commodity: String,
    pub commodity_code: String,
    pub origin: String,
    pub destination: String,
    pub profit_per_scu: f64,
    pub available_scu: f64,
    pub likely_ship: CargoShip,
    pub estimated_haul_value: f64,
    /// Risk score 0-100 (higher = more likely to be used).
    pub risk_score: f64,
}

/// Prediction of a target at a location.
#[derive(Debug, Clone, Serialize)]
pub struct TargetPrediction {
    pub direction: TrafficDirection,
    pub commodity: String,
    pub likely_ship: CargoShip,
    pub estimated_cargo_value: f64,
    pub destination: String,
}

/// Direction of traffic flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrafficDirection {
    Arriving,
    Departing,
}

/// Trade activity summary for a location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeActivity {
    pub location: String,
    pub inbound_routes: usize,
    pub outbound_routes: usize,
    pub top_imports: Vec<String>,
    pub top_exports: Vec<String>,
    pub estimated_daily_traffic: f64,
}

/// Calculate risk score based on route characteristics.
fn calculate_risk_score(route: &TradeRoute) -> f64 {
    let mut score: f64 = 0.0;

    // Higher profit = more likely to be run
    score += (route.profit_per_unit / 10.0).min(30.0);

    // More available SCU = more likely
    score += (route.scu_origin / 100.0).min(20.0);

    // Large cargo capacity routes are juicy targets
    if route.scu_origin > 500.0 {
        score += 20.0;
    }

    // Cap at 100
    score.min(100.0)
}
