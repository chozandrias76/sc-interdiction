//! Target prediction based on trade route analysis.

use std::collections::HashMap;
use api_client::{TradeRoute, UexClient};
use ordered_float::OrderedFloat;
use route_graph::{
    estimate_position, find_chokepoints, find_route_intersections, Chokepoint,
    RouteGraph, RouteIntersection, RouteSegment,
};
use serde::{Deserialize, Serialize};
use crate::ships::{CargoShip, estimate_ship_for_route, estimate_ship_for_routes};

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
                    origin_system: r.origin_system.clone(),
                    destination: r.terminal_destination_name.clone(),
                    destination_system: r.destination_system.clone(),
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
        let location_lower = location.to_lowercase();

        let predictions: Vec<TargetPrediction> = routes
            .into_iter()
            .filter(|r| {
                r.terminal_origin_name.to_lowercase().contains(&location_lower)
                    || r.terminal_destination_name.to_lowercase().contains(&location_lower)
            })
            .map(|r| {
                let is_departing = r.terminal_origin_name.to_lowercase().contains(&location_lower);
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

        // Build a map of terminal ID to code for lookup
        let terminals = self.uex.get_terminals().await?;
        let id_to_code: std::collections::HashMap<i64, String> = terminals
            .iter()
            .filter_map(|t| t.code.as_ref().map(|code| (t.id, code.clone())))
            .collect();

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
            .filter_map(|r| {
                // Look up terminal codes, skip if not found
                let origin_code = id_to_code.get(&r.id_terminal_origin)?;
                let dest_code = id_to_code.get(&r.id_terminal_destination)?;
                Some((
                    origin_code.clone(),
                    dest_code.clone(),
                    r.profit_per_unit,
                ))
            })
            .collect();

        let mut chokepoints = find_chokepoints(graph, &trade_routes);
        chokepoints.truncate(top_n);

        Ok(chokepoints)
    }

    /// Find jump point chokepoints for cross-system routes.
    /// Groups routes by the systems they connect (e.g., Stanton-Pyro).
    pub async fn find_jump_point_chokepoints(
        &self,
        top_n: usize,
    ) -> api_client::Result<Vec<JumpPointChokepoint>> {
        let routes = self.uex.get_trade_routes().await?;

        // Group cross-system routes by system pair
        let mut system_pairs: std::collections::HashMap<(String, String), Vec<CrossSystemRoute>> = 
            std::collections::HashMap::new();

        for route in routes.iter() {
            if route.origin_system.is_empty() || route.destination_system.is_empty() {
                continue;
            }

            // Skip intra-system routes
            if route.origin_system.eq_ignore_ascii_case(&route.destination_system) {
                continue;
            }

            // Normalize system pair (alphabetically ordered for consistency)
            let (sys1, sys2) = if route.origin_system < route.destination_system {
                (route.origin_system.clone(), route.destination_system.clone())
            } else {
                (route.destination_system.clone(), route.origin_system.clone())
            };

            let cross_route = CrossSystemRoute {
                commodity: route.commodity_name.clone(),
                origin: route.terminal_origin_name.clone(),
                destination: route.terminal_destination_name.clone(),
                origin_system: route.origin_system.clone(),
                destination_system: route.destination_system.clone(),
                profit_per_scu: route.profit_per_unit,
            };

            system_pairs
                .entry((sys1, sys2))
                .or_default()
                .push(cross_route);
        }

        // Convert to jump point chokepoints
        let mut jump_points: Vec<JumpPointChokepoint> = system_pairs
            .into_iter()
            .map(|((sys1, sys2), routes)| {
                let route_count = routes.len();
                let traffic_score: f64 = routes.iter().map(|r| r.profit_per_scu).sum();

                JumpPointChokepoint {
                    system_a: sys1.clone(),
                    system_b: sys2.clone(),
                    jump_point_name: format!("{}-{} Jump Point", sys1, sys2),
                    route_count,
                    traffic_score,
                    routes,
                }
            })
            .collect();

        // Sort by traffic score
        jump_points.sort_by(|a, b| {
            b.traffic_score
                .partial_cmp(&a.traffic_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        jump_points.truncate(top_n);

        Ok(jump_points)
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
                        let a_returns_home = a.terminal_destination_name == outbound.terminal_origin_name;
                        let b_returns_home = b.terminal_destination_name == outbound.terminal_origin_name;

                        match (a_returns_home, b_returns_home) {
                            (true, false) => std::cmp::Ordering::Greater,
                            (false, true) => std::cmp::Ordering::Less,
                            _ => a.profit_per_unit.partial_cmp(&b.profit_per_unit)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        }
                    })
                    .copied()
            });

            // Estimate ship for both legs
            let likely_ship = if let Some(ret) = best_return {
                estimate_ship_for_routes(&[outbound, ret])
            } else {
                estimate_ship_for_route(outbound)
            };

            let cargo_scu = likely_ship.cargo_scu as f64;

            let outbound_profit = outbound.profit_for_scu(cargo_scu);
            let outbound_cargo_value = outbound.price_origin * cargo_scu;

            let (return_leg, return_profit) = if let Some(ret) = best_return {
                let ret_profit = ret.profit_for_scu(cargo_scu);
                let ret_cargo_value = ret.price_origin * cargo_scu;
                (
                    Some(RouteLeg {
                        commodity: ret.commodity_name.clone(),
                        origin: ret.terminal_origin_name.clone(),
                        destination: ret.terminal_destination_name.clone(),
                        profit_per_scu: ret.profit_per_unit,
                        cargo_value: ret_cargo_value,
                    }),
                    ret_profit,
                )
            } else {
                (None, 0.0)
            };

            let total_profit = outbound_profit + return_profit;

            trade_runs.push(TradeRun {
                outbound: RouteLeg {
                    commodity: outbound.commodity_name.clone(),
                    origin: outbound.terminal_origin_name.clone(),
                    destination: outbound.terminal_destination_name.clone(),
                    profit_per_scu: outbound.profit_per_unit,
                    cargo_value: outbound_cargo_value,
                },
                return_leg,
                likely_ship,
                total_profit,
                has_return_cargo: best_return.is_some(),
            });
        }

        // Sort by total profit descending
        trade_runs.sort_by_key(|r| std::cmp::Reverse(OrderedFloat(r.total_profit)));
        trade_runs.truncate(limit);

        Ok(trade_runs)
    }
}

/// A profitable trade route.
#[derive(Debug, Clone, Serialize)]
pub struct HotRoute {
    pub commodity: String,
    pub commodity_code: String,
    pub origin: String,
    pub origin_system: String,
    pub destination: String,
    pub destination_system: String,
    pub profit_per_scu: f64,
    pub available_scu: f64,
    pub likely_ship: CargoShip,
    pub estimated_haul_value: f64,
    /// Risk score 0-100 (higher = more likely to be used).
    pub risk_score: f64,
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
}

/// A single leg of a trade route.
#[derive(Debug, Clone, Serialize)]
pub struct RouteLeg {
    pub commodity: String,
    pub origin: String,
    pub destination: String,
    pub profit_per_scu: f64,
    pub cargo_value: f64,
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

/// Ship frequency at a location.
#[derive(Debug, Clone, Serialize)]
pub struct ShipFrequency {
    pub ship_name: String,
    pub frequency: usize,
    pub avg_threat_level: f64,
}

/// A jump point chokepoint for cross-system routes.
#[derive(Debug, Clone, Serialize)]
pub struct JumpPointChokepoint {
    /// First system in the connection.
    pub system_a: String,
    /// Second system in the connection.
    pub system_b: String,
    /// Name of the jump point.
    pub jump_point_name: String,
    /// Number of routes passing through this jump point.
    pub route_count: usize,
    /// Total traffic score based on route profitability.
    pub traffic_score: f64,
    /// Cross-system routes using this jump point.
    pub routes: Vec<CrossSystemRoute>,
}

/// A cross-system trade route.
#[derive(Debug, Clone, Serialize)]
pub struct CrossSystemRoute {
    pub commodity: String,
    pub origin: String,
    pub destination: String,
    pub origin_system: String,
    pub destination_system: String,
    pub profit_per_scu: f64,
}

impl TargetAnalyzer {
    /// Get top interdiction hotspots ranked by total cargo value.
    pub async fn get_interdiction_hotspots(&self, limit: usize) -> api_client::Result<Vec<InterdictionHotspot>> {
        let routes = self.uex.get_trade_routes().await?;

        // Aggregate data by location (both origin and destination)
        let mut location_data: HashMap<String, LocationAggregator> = HashMap::new();

        for route in &routes {
            if route.profit_per_unit <= 0.0 {
                continue;
            }

            let ship = estimate_ship_for_route(route);
            let cargo_value = route.profit_for_scu(ship.cargo_scu as f64)
                + (route.price_origin * ship.cargo_scu as f64);

            // Add to origin location
            let origin = &route.terminal_origin_name;
            let origin_agg = location_data.entry(origin.clone()).or_insert_with(|| {
                LocationAggregator::new(origin.clone(), extract_system(origin))
            });
            origin_agg.add_route(&route.commodity_name, cargo_value, &ship);

            // Add to destination location
            let dest = &route.terminal_destination_name;
            let dest_agg = location_data.entry(dest.clone()).or_insert_with(|| {
                LocationAggregator::new(dest.clone(), extract_system(dest))
            });
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

                let ship = estimate_ship_for_route(r);
                let cargo_value = r.profit_for_scu(ship.cargo_scu as f64)
                    + (r.price_origin * ship.cargo_scu as f64);

                Some(RouteSegment {
                    origin: origin_pos,
                    destination: dest_pos,
                    origin_name: r.terminal_origin_name.clone(),
                    destination_name: r.terminal_destination_name.clone(),
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

        *self.commodity_values.entry(commodity.to_string()).or_default() += cargo_value;

        let entry = self.ship_counts.entry(ship.name.to_string()).or_insert((0, ship.threat_level));
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
        ships.sort_by(|a, b| b.1.0.cmp(&a.1.0));
        let likely_ships: Vec<ShipFrequency> = ships
            .into_iter()
            .take(5)
            .map(|(name, (count, threat))| ShipFrequency {
                ship_name: name,
                frequency: count,
                avg_threat_level: threat as f64,
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
