//! Target prediction based on trade route analysis.

use crate::ships::{CargoShip, ShipRegistry};
use api_client::{TradeRoute, UexClient};
use ordered_float::OrderedFloat;
use route_graph::{
    estimate_position, find_chokepoints, find_route_intersections, Chokepoint, RouteGraph,
    RouteIntersection, RouteSegment,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Analyzes trade data to predict targets.
pub struct TargetAnalyzer {
    uex: UexClient,
    registry: ShipRegistry,
}

impl TargetAnalyzer {
    /// Create a new target analyzer with a ship registry.
    pub fn new(uex: UexClient, registry: ShipRegistry) -> Self {
        Self { uex, registry }
    }

    /// Get hot trade routes sorted by profitability.
    pub async fn get_hot_routes(&self, limit: usize) -> api_client::Result<Vec<HotRoute>> {
        let routes = self.uex.get_trade_routes().await?;

        let mut hot_routes: Vec<HotRoute> = routes
            .into_iter()
            .filter(|r| r.profit_per_unit > 0.0 && r.scu_origin > 0.0)
            .map(|r| {
                let likely_ship = self.registry.estimate_for_route(&r);
                let estimated_value = r.profit_for_scu(likely_ship.cargo_scu as f64);

                // Calculate route distance
                let distance_mkm = route_graph::distance_between(
                    &r.terminal_origin_name,
                    &r.terminal_destination_name,
                )
                .unwrap_or(0.0);

                // Check fuel sufficiency
                let (fuel_sufficient, fuel_required, _) =
                    likely_ship.can_complete_route(distance_mkm);

                HotRoute {
                    commodity: r.commodity_name.clone(),
                    commodity_code: r.commodity_code.clone(),
                    origin: r.terminal_origin_name.clone(),
                    destination: r.terminal_destination_name.clone(),
                    origin_system: Some(extract_system(&r.terminal_origin_name)),
                    destination_system: Some(extract_system(&r.terminal_destination_name)),
                    profit_per_scu: r.profit_per_unit,
                    available_scu: r.max_profitable_scu(),
                    likely_ship,
                    estimated_haul_value: estimated_value,
                    risk_score: calculate_risk_score(&r),
                    distance_mkm,
                    fuel_sufficient,
                    fuel_required,
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
        let location_lower = location.to_lowercase();

        let predictions: Vec<TargetPrediction> = routes
            .into_iter()
            .filter(|r| {
                r.terminal_origin_name
                    .to_lowercase()
                    .contains(&location_lower)
                    || r.terminal_destination_name
                        .to_lowercase()
                        .contains(&location_lower)
            })
            .map(|r| {
                let is_departing = r
                    .terminal_origin_name
                    .to_lowercase()
                    .contains(&location_lower);
                let likely_ship = self.registry.estimate_for_route(&r);
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
    /// Find best interdiction points based on current trade data.
    ///
    /// If `cross_system` is true, includes routes between different systems.
    /// If false, only includes routes within the same system.
    pub async fn find_interdiction_points(
        &self,
        graph: &RouteGraph,
        top_n: usize,
        cross_system: bool,
    ) -> api_client::Result<Vec<Chokepoint>> {
        let routes = self.uex.get_trade_routes().await?;

        let trade_routes: Vec<_> = routes
            .iter()
            .filter(|r| {
                // Filter based on cross_system flag
                if cross_system {
                    // Only include routes where origin and destination are in DIFFERENT systems
                    !r.origin_system.is_empty()
                        && !r.destination_system.is_empty()
                        && !r.origin_system.eq_ignore_ascii_case(&r.destination_system)
                } else {
                    // Only include routes where both origin and destination are in the SAME system
                    !r.origin_system.is_empty()
                        && !r.destination_system.is_empty()
                        && r.origin_system.eq_ignore_ascii_case(&r.destination_system)
                }
            })
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

    /// Get complete trade runs (round-trips) sorted by total profitability.
    ///
    /// This finds routes where a ship can haul cargo outbound, then find return
    /// cargo to bring back, maximizing profit for the entire trip.
    pub async fn get_trade_runs(&self, limit: usize) -> api_client::Result<Vec<TradeRun>> {
        let routes = self.uex.get_trade_routes().await?;

        // Build a map of destination -> routes that START from that destination
        // This helps us find return cargo: after arriving at destination, what can we haul back?
        let mut routes_from: HashMap<String, Vec<&TradeRoute>> = HashMap::new();
        for route in &routes {
            routes_from
                .entry(route.terminal_origin_name.clone())
                .or_default()
                .push(route);
        }

        let mut trade_runs: Vec<TradeRun> = Vec::new();

        for outbound in &routes {
            if outbound.profit_per_unit <= 0.0 || outbound.scu_origin <= 0.0 {
                continue;
            }

            // Look for return routes: routes that START at our destination
            let return_routes = routes_from.get(&outbound.terminal_destination_name);

            // Find the best return route (if any) that goes back toward our origin
            // or at least is profitable
            let best_return = return_routes.and_then(|returns| {
                returns
                    .iter()
                    .filter(|r| r.profit_per_unit > 0.0 && r.scu_origin > 0.0)
                    // Prefer routes going back to original origin, but accept any profitable return
                    .max_by(|a, b| {
                        let a_returns_home =
                            a.terminal_destination_name == outbound.terminal_origin_name;
                        let b_returns_home =
                            b.terminal_destination_name == outbound.terminal_origin_name;

                        match (a_returns_home, b_returns_home) {
                            (true, false) => std::cmp::Ordering::Greater,
                            (false, true) => std::cmp::Ordering::Less,
                            _ => a
                                .profit_per_unit
                                .partial_cmp(&b.profit_per_unit)
                                .unwrap_or(std::cmp::Ordering::Equal),
                        }
                    })
                    .copied()
            });

            // Estimate ship for both legs
            let likely_ship = if let Some(ret) = best_return {
                self.registry.estimate_for_routes(&[outbound, ret])
            } else {
                self.registry.estimate_for_route(outbound)
            };

            let cargo_scu = likely_ship.cargo_scu as f64;

            let outbound_profit = outbound.profit_for_scu(cargo_scu);
            let outbound_cargo_value = outbound.price_origin * cargo_scu;

            // Calculate outbound distance
            let outbound_dist = route_graph::distance_between(
                &outbound.terminal_origin_name,
                &outbound.terminal_destination_name,
            )
            .unwrap_or(0.0);

            let (return_leg, return_profit, return_dist) = if let Some(ret) = best_return {
                let ret_profit = ret.profit_for_scu(cargo_scu);
                let ret_cargo_value = ret.price_origin * cargo_scu;

                let dist = route_graph::distance_between(
                    &ret.terminal_origin_name,
                    &ret.terminal_destination_name,
                )
                .unwrap_or(0.0);

                (
                    Some(RouteLeg {
                        commodity: ret.commodity_name.clone(),
                        origin: ret.terminal_origin_name.clone(),
                        destination: ret.terminal_destination_name.clone(),
                        profit_per_scu: ret.profit_per_unit,
                        cargo_value: ret_cargo_value,
                        distance_mkm: dist,
                    }),
                    ret_profit,
                    dist,
                )
            } else {
                (None, 0.0, 0.0)
            };

            let total_profit = outbound_profit + return_profit;
            let total_distance = outbound_dist + return_dist;
            let (fuel_sufficient, _, _) = likely_ship.can_complete_route(total_distance);

            trade_runs.push(TradeRun {
                outbound: RouteLeg {
                    commodity: outbound.commodity_name.clone(),
                    origin: outbound.terminal_origin_name.clone(),
                    destination: outbound.terminal_destination_name.clone(),
                    profit_per_scu: outbound.profit_per_unit,
                    cargo_value: outbound_cargo_value,
                    distance_mkm: outbound_dist,
                },
                return_leg,
                likely_ship,
                total_profit,
                has_return_cargo: best_return.is_some(),
                total_distance_mkm: total_distance,
                fuel_sufficient,
            });
        }

        // Sort by total profit descending
        trade_runs.sort_by_key(|r| std::cmp::Reverse(OrderedFloat(r.total_profit)));
        trade_runs.truncate(limit);

        Ok(trade_runs)
    }
}

/// A hot trade route (single commodity, origin -> destination).
#[derive(Debug, Clone, Serialize)]
pub struct HotRoute {
    pub commodity: String,
    pub commodity_code: String,
    pub origin: String,
    pub destination: String,
    pub origin_system: Option<String>,
    pub destination_system: Option<String>,
    pub profit_per_scu: f64,
    pub available_scu: f64,
    pub likely_ship: CargoShip,
    pub estimated_haul_value: f64,
    /// Risk score 0-100 (higher = more likely to be used).
    pub risk_score: f64,
    /// Route distance in millions of km (Mkm).
    pub distance_mkm: f64,
    /// Whether the likely ship can complete this route without refueling.
    pub fuel_sufficient: bool,
    /// Quantum fuel required for this route (units).
    pub fuel_required: f64,
}

/// A complete round-trip trade run (outbound + return with cargo).
#[derive(Debug, Clone, Serialize)]
pub struct TradeRun {
    /// The outbound leg of the trip.
    pub outbound: RouteLeg,
    /// The return leg (may be empty/deadhead if no profitable return cargo).
    pub return_leg: Option<RouteLeg>,
    /// Ship that services both legs.
    pub likely_ship: CargoShip,
    /// Total profit for the round trip.
    pub total_profit: f64,
    /// Whether this is a full round-trip with return cargo.
    pub has_return_cargo: bool,
    /// Total distance for round trip (Mkm).
    pub total_distance_mkm: f64,
    /// Whether the ship can complete the full trip without refueling.
    pub fuel_sufficient: bool,
}

/// A single leg of a trade route.
#[derive(Debug, Clone, Serialize)]
pub struct RouteLeg {
    pub commodity: String,
    pub origin: String,
    pub destination: String,
    pub profit_per_scu: f64,
    pub cargo_value: f64,
    /// Route distance in millions of km (Mkm).
    pub distance_mkm: f64,
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

/// An interdiction hotspot - a location with high-value traffic.
#[derive(Debug, Clone, Serialize)]
pub struct InterdictionHotspot {
    /// Location name.
    pub location: String,
    /// System the location is in.
    pub system: String,
    /// Total estimated value of cargo passing through (daily).
    pub total_cargo_value: f64,
    /// Number of profitable routes through this location.
    pub route_count: usize,
    /// Average threat level of ships (0-10 scale).
    pub avg_threat_level: f64,
    /// Top commodities by value.
    pub top_commodities: Vec<CommodityValue>,
    /// Top ships likely to be encountered.
    pub likely_ships: Vec<ShipFrequency>,
    /// Suggested interdiction approach.
    pub suggested_position: String,
}

/// Commodity with estimated value.
#[derive(Debug, Clone, Serialize)]
pub struct CommodityValue {
    pub name: String,
    pub estimated_value: f64,
}

/// Ship type with frequency.
#[derive(Debug, Clone, Serialize)]
pub struct ShipFrequency {
    pub ship_name: String,
    pub count: usize,
    pub threat_level: u8,
}

impl TargetAnalyzer {
    /// Get top interdiction hotspots ranked by total cargo value.
    pub async fn get_interdiction_hotspots(
        &self,
        limit: usize,
    ) -> api_client::Result<Vec<InterdictionHotspot>> {
        let routes = self.uex.get_trade_routes().await?;

        // Aggregate data by location (both origin and destination)
        let mut location_data: HashMap<String, LocationAggregator> = HashMap::new();

        for route in &routes {
            if route.profit_per_unit <= 0.0 {
                continue;
            }

            let ship = self.registry.estimate_for_route(route);
            let cargo_value = route.profit_for_scu(ship.cargo_scu as f64)
                + (route.price_origin * ship.cargo_scu as f64);

            // Add to origin location
            let origin = &route.terminal_origin_name;
            let origin_agg = location_data
                .entry(origin.clone())
                .or_insert_with(|| LocationAggregator::new(origin.clone(), extract_system(origin)));
            origin_agg.add_route(&route.commodity_name, cargo_value, &ship);

            // Add to destination location
            let dest = &route.terminal_destination_name;
            let dest_agg = location_data
                .entry(dest.clone())
                .or_insert_with(|| LocationAggregator::new(dest.clone(), extract_system(dest)));
            dest_agg.add_route(&route.commodity_name, cargo_value, &ship);
        }

        // Convert to hotspots and sort by value
        let mut hotspots: Vec<InterdictionHotspot> = location_data
            .into_values()
            .map(|agg| agg.into_hotspot())
            .collect();

        hotspots.sort_by(|a, b| {
            b.total_cargo_value
                .partial_cmp(&a.total_cargo_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        hotspots.truncate(limit);
        Ok(hotspots)
    }

    /// Find high-value route intersection points.
    ///
    /// This finds points in space where multiple profitable trade routes cross,
    /// making them ideal interdiction spots where you can catch ships traveling
    /// between different origin/destination pairs.
    pub async fn get_route_intersections(
        &self,
        limit: usize,
        min_routes: usize,
    ) -> api_client::Result<Vec<RouteIntersection>> {
        let routes = self.uex.get_trade_routes().await?;

        // Convert trade routes to spatial segments
        let segments: Vec<RouteSegment> = routes
            .iter()
            .filter(|r| r.profit_per_unit > 0.0 && r.scu_origin > 0.0)
            .filter_map(|r| {
                // Get positions for origin and destination
                let origin_pos = estimate_position(&r.terminal_origin_name)?;
                let dest_pos = estimate_position(&r.terminal_destination_name)?;

                let ship = self.registry.estimate_for_route(r);
                let cargo_value = r.profit_for_scu(ship.cargo_scu as f64)
                    + (r.price_origin * ship.cargo_scu as f64);

                Some(RouteSegment {
                    origin: origin_pos,
                    destination: dest_pos,
                    origin_name: r.terminal_origin_name.clone(),
                    destination_name: r.terminal_destination_name.clone(),
                    origin_system: Some(extract_system(&r.terminal_origin_name)),
                    destination_system: Some(extract_system(&r.terminal_destination_name)),
                    cargo_value,
                    commodity: r.commodity_name.clone(),
                    ship_name: ship.name.to_string(),
                    threat_level: ship.threat_level,
                })
            })
            .collect();

        // Find intersections with 2.0 Mkm proximity threshold
        // This is roughly the effective range for QT interdiction positioning
        let mut intersections = find_route_intersections(&segments, 2.0, min_routes);
        intersections.truncate(limit);

        Ok(intersections)
    }
}

/// Helper for aggregating location data.
struct LocationAggregator {
    location: String,
    system: String,
    total_value: f64,
    route_count: usize,
    threat_levels: Vec<u8>,
    commodity_values: HashMap<String, f64>,
    ship_counts: HashMap<String, (usize, u8)>, // (count, threat_level)
}

impl LocationAggregator {
    fn new(location: String, system: String) -> Self {
        Self {
            location,
            system,
            total_value: 0.0,
            route_count: 0,
            threat_levels: Vec::new(),
            commodity_values: HashMap::new(),
            ship_counts: HashMap::new(),
        }
    }

    fn add_route(&mut self, commodity: &str, cargo_value: f64, ship: &CargoShip) {
        self.total_value += cargo_value;
        self.route_count += 1;
        self.threat_levels.push(ship.threat_level);

        *self
            .commodity_values
            .entry(commodity.to_string())
            .or_default() += cargo_value;

        let entry = self
            .ship_counts
            .entry(ship.name.to_string())
            .or_insert((0, ship.threat_level));
        entry.0 += 1;
    }

    fn into_hotspot(self) -> InterdictionHotspot {
        // Calculate average threat
        let avg_threat = if self.threat_levels.is_empty() {
            5.0
        } else {
            self.threat_levels.iter().map(|&t| t as f64).sum::<f64>()
                / self.threat_levels.len() as f64
        };

        // Get top commodities
        let mut commodities: Vec<_> = self.commodity_values.into_iter().collect();
        commodities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let top_commodities: Vec<CommodityValue> = commodities
            .into_iter()
            .take(5)
            .map(|(name, value)| CommodityValue {
                name,
                estimated_value: value,
            })
            .collect();

        // Get likely ships
        let mut ships: Vec<_> = self.ship_counts.into_iter().collect();
        ships.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
        let likely_ships: Vec<ShipFrequency> = ships
            .into_iter()
            .take(5)
            .map(|(name, (count, threat))| ShipFrequency {
                ship_name: name,
                count,
                threat_level: threat,
            })
            .collect();

        // Generate position suggestion
        let suggested_position = if avg_threat < 3.0 {
            "Low risk - solo interdiction viable".to_string()
        } else if avg_threat < 6.0 {
            "Medium risk - wing recommended".to_string()
        } else {
            "High risk - multi-ship crew required".to_string()
        };

        InterdictionHotspot {
            location: self.location,
            system: self.system,
            total_cargo_value: self.total_value,
            route_count: self.route_count,
            avg_threat_level: avg_threat,
            top_commodities,
            likely_ships,
            suggested_position,
        }
    }
}

/// Extract system name from terminal name.
fn extract_system(terminal_name: &str) -> String {
    // Terminal names look like "Commodity Shop - Admin - ARC-L1 (Stanton > ArcCorp)"
    // Some have nested parens like "Stanton Gateway (Pyro) (Pyro > Stanton Gateway)"
    // We want the LAST paren group that contains ">"
    if let Some(paren_start) = terminal_name.rfind('(') {
        let after_paren = &terminal_name[paren_start..];
        if let Some(arrow) = after_paren.find('>') {
            // Extract system name (before the >)
            let system = &after_paren[1..arrow];
            return system.trim().to_string();
        }
        // No arrow, check if there's a closing paren
        if let Some(paren_end) = after_paren.find(')') {
            return after_paren[1..paren_end].trim().to_string();
        }
    }
    "Unknown".to_string()
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

#[cfg(test)]
#[path = "targets_tests.rs"]
mod tests;
