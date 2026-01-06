//! Quantum fuel consumption calculations.
//!
//! Provides functions to calculate quantum travel fuel consumption
//! based on distance and ship quantum drive efficiency.
//!
//! Distance units are in millions of km (Mkm) to match the location database.

use serde::{Deserialize, Serialize};

/// Quantum drive efficiency rating.
///
/// Defines how much quantum fuel a drive consumes per million kilometers traveled.
/// Lower values = more efficient (less fuel per distance).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QtDriveEfficiency {
    /// Display name for this efficiency class.
    pub name: &'static str,
    /// Quantum fuel units consumed per million km traveled.
    pub fuel_per_mkm: f64,
}

impl QtDriveEfficiency {
    /// Create a new efficiency rating.
    #[must_use]
    pub const fn new(name: &'static str, fuel_per_mkm: f64) -> Self {
        Self { name, fuel_per_mkm }
    }
}

/// Default quantum drive efficiency ratings by size class.
///
/// **STATUS: ESTIMATED VALUES - NEEDS VERIFICATION**
/// See: `docs/DATA_SOURCES.md` - Quantum Drive Efficiency section
///
/// These are estimated values based on observed quantum travel fuel consumption patterns.
/// Actual values may vary based on specific drive models and game balance changes.
///
/// Source: Estimated from in-game observations
/// Verification needed: Controlled test flights measuring fuel over known distances
///
/// Reference: A cross-system trip (e.g., Hurston to microTech) is ~25 Mkm.
pub static QT_DRIVE_EFFICIENCY: &[QtDriveEfficiency] = &[
    // Size 1 - Small fighters and light haulers (Aurora, Avenger, etc.)
    // Typical tank: 583-1250 units, efficient but small capacity
    QtDriveEfficiency::new("S1 (Small)", 40.0),
    // Size 2 - Medium ships (Cutlass, Freelancer, Constellation)
    // Typical tank: 2500-5000 units, balanced efficiency
    QtDriveEfficiency::new("S2 (Medium)", 80.0),
    // Size 3 - Large ships (Caterpillar, C2 Hercules, Hull C)
    // Typical tank: 10000-20000 units, high consumption but massive capacity
    QtDriveEfficiency::new("S3 (Large)", 160.0),
];

/// Get the default efficiency for a quantum drive size (1-3).
///
/// Returns None if size is out of range.
#[must_use]
#[allow(clippy::indexing_slicing)] // Static array with known valid indices
pub fn efficiency_for_size(size: u8) -> Option<&'static QtDriveEfficiency> {
    match size {
        1 => Some(&QT_DRIVE_EFFICIENCY[0]),
        2 => Some(&QT_DRIVE_EFFICIENCY[1]),
        3 => Some(&QT_DRIVE_EFFICIENCY[2]),
        _ => None,
    }
}

/// Calculate quantum fuel consumption for a given distance.
///
/// # Arguments
/// * `distance_mkm` - Distance in millions of kilometers
/// * `efficiency` - Quantum drive efficiency rating
///
/// # Returns
/// Quantum fuel units consumed for the trip.
///
/// # Example
/// ```
/// use route_graph::fuel::{calculate_qt_fuel_consumption, efficiency_for_size};
///
/// if let Some(efficiency) = efficiency_for_size(2) {
///     let fuel = calculate_qt_fuel_consumption(25.0, efficiency);
///     // A 25 Mkm trip with S2 drive uses 2000 units (25 * 80)
///     assert_eq!(fuel, 2000.0);
/// }
/// ```
#[must_use]
pub fn calculate_qt_fuel_consumption(distance_mkm: f64, efficiency: &QtDriveEfficiency) -> f64 {
    if distance_mkm <= 0.0 {
        return 0.0;
    }
    distance_mkm * efficiency.fuel_per_mkm
}

/// Check if a ship can complete a route with given fuel capacity.
///
/// # Arguments
/// * `distance_mkm` - Total route distance in millions of km
/// * `fuel_capacity` - Ship's quantum fuel tank capacity
/// * `efficiency` - Quantum drive efficiency rating
///
/// # Returns
/// A tuple of (`can_complete`, `fuel_required`, `fuel_remaining`)
#[must_use]
pub fn can_complete_route(
    distance_mkm: f64,
    fuel_capacity: f64,
    efficiency: &QtDriveEfficiency,
) -> (bool, f64, f64) {
    let fuel_required = calculate_qt_fuel_consumption(distance_mkm, efficiency);
    let fuel_remaining = fuel_capacity - fuel_required;
    (
        fuel_remaining >= 0.0,
        fuel_required,
        fuel_remaining.max(0.0),
    )
}

/// Calculate maximum range for a given fuel capacity and efficiency.
///
/// # Arguments
/// * `fuel_capacity` - Available quantum fuel units
/// * `efficiency` - Quantum drive efficiency rating
///
/// # Returns
/// Maximum travel distance in millions of km.
#[must_use]
pub fn max_range_mkm(fuel_capacity: f64, efficiency: &QtDriveEfficiency) -> f64 {
    if efficiency.fuel_per_mkm <= 0.0 {
        return 0.0;
    }
    fuel_capacity / efficiency.fuel_per_mkm
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::assertions_on_constants)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_qt_fuel_consumption() {
        let Some(s2_efficiency) = efficiency_for_size(2) else {
            assert!(false, "S2 efficiency should be available");
            return;
        };

        // 25 Mkm trip (cross-system) with S2 drive
        let fuel = calculate_qt_fuel_consumption(25.0, s2_efficiency);
        assert_eq!(fuel, 2000.0);

        // Zero distance = zero fuel
        assert_eq!(calculate_qt_fuel_consumption(0.0, s2_efficiency), 0.0);

        // Negative distance = zero fuel
        assert_eq!(calculate_qt_fuel_consumption(-5.0, s2_efficiency), 0.0);
    }

    #[test]
    fn test_efficiency_by_size() {
        // S1 is most efficient (lowest fuel/Mkm)
        let Some(s1) = efficiency_for_size(1) else {
            assert!(false, "S1 efficiency should be available");
            return;
        };
        let Some(s2) = efficiency_for_size(2) else {
            assert!(false, "S2 efficiency should be available");
            return;
        };
        let Some(s3) = efficiency_for_size(3) else {
            assert!(false, "S3 efficiency should be available");
            return;
        };

        assert!(s1.fuel_per_mkm < s2.fuel_per_mkm);
        assert!(s2.fuel_per_mkm < s3.fuel_per_mkm);

        // Invalid sizes return None
        assert!(efficiency_for_size(0).is_none());
        assert!(efficiency_for_size(4).is_none());
    }

    #[test]
    fn test_can_complete_route() {
        let Some(s2_efficiency) = efficiency_for_size(2) else {
            assert!(false, "S2 efficiency should be available");
            return;
        };
        let fuel_capacity = 3000.0;

        // Short route - can complete with fuel to spare
        let (can_complete, fuel_required, remaining) =
            can_complete_route(25.0, fuel_capacity, s2_efficiency);
        assert!(can_complete);
        assert_eq!(fuel_required, 2000.0);
        assert_eq!(remaining, 1000.0);

        // Long route - cannot complete
        let (can_complete, fuel_required, remaining) =
            can_complete_route(50.0, fuel_capacity, s2_efficiency);
        assert!(!can_complete);
        assert_eq!(fuel_required, 4000.0);
        assert_eq!(remaining, 0.0); // Clamped to 0
    }

    #[test]
    fn test_max_range() {
        let Some(s2_efficiency) = efficiency_for_size(2) else {
            assert!(false, "S2 efficiency should be available");
            return;
        };

        // 3000 fuel / 80 per Mkm = 37.5 Mkm range
        let range = max_range_mkm(3000.0, s2_efficiency);
        assert_eq!(range, 37.5);

        // Zero fuel = zero range
        assert_eq!(max_range_mkm(0.0, s2_efficiency), 0.0);
    }
}

/// A fuel station/refueling location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuelStation {
    /// Terminal name
    pub name: String,
    /// Terminal code (if available)
    pub code: Option<String>,
    /// Star system name
    pub system: Option<String>,
    /// Estimated 3D position in space (Mkm from system center)
    pub position: Option<crate::Point3D>,
}

/// Spatial index of fuel stations for efficient nearest-neighbor queries.
#[derive(Debug, Clone)]
pub struct FuelStationIndex {
    stations: Vec<FuelStation>,
}

impl FuelStationIndex {
    /// Create a new fuel station index from terminal data.
    ///
    /// Only includes terminals where `is_refuel` is true.
    #[must_use]
    pub fn from_terminals(terminals: &[api_client::Terminal]) -> Self {
        let stations = terminals
            .iter()
            .filter(|t| t.is_refuel)
            .map(|t| {
                let name = t
                    .name
                    .clone()
                    .or_else(|| t.nickname.clone())
                    .unwrap_or_else(|| format!("Terminal {}", t.id));

                // Try to estimate position from location name
                let position = t
                    .code
                    .as_ref()
                    .and_then(|code| crate::estimate_position(code));

                FuelStation {
                    name,
                    code: t.code.clone(),
                    system: t.star_system_name.clone(),
                    position,
                }
            })
            .collect();

        Self { stations }
    }

    /// Get all fuel stations.
    #[must_use]
    pub fn all_stations(&self) -> &[FuelStation] {
        &self.stations
    }

    /// Get fuel stations in a specific star system.
    #[must_use]
    pub fn stations_in_system(&self, system: &str) -> Vec<&FuelStation> {
        self.stations
            .iter()
            .filter(|s| {
                s.system
                    .as_ref()
                    .map(|sys| sys.eq_ignore_ascii_case(system))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Find the nearest fuel station to a given position.
    ///
    /// Returns (station, `distance_mkm`) or None if no stations have positions.
    #[must_use]
    pub fn find_nearest(&self, position: &crate::Point3D) -> Option<(&FuelStation, f64)> {
        self.stations
            .iter()
            .filter_map(|station| {
                station.position.as_ref().map(|pos| {
                    let dx = pos.x - position.x;
                    let dy = pos.y - position.y;
                    let dz = pos.z - position.z;
                    let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                    (station, distance)
                })
            })
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Find the nearest fuel station along a route (within a given deviation threshold).
    ///
    /// Returns (station, `distance_from_route_mkm`) if one is found.
    #[must_use]
    pub fn find_nearest_on_route(
        &self,
        start: &crate::Point3D,
        end: &crate::Point3D,
        max_deviation_mkm: f64,
    ) -> Option<(&FuelStation, f64)> {
        self.stations
            .iter()
            .filter_map(|station| {
                station.position.as_ref().map(|pos| {
                    let deviation = perpendicular_distance_to_line(pos, start, end);
                    (station, deviation)
                })
            })
            .filter(|(_, deviation)| *deviation <= max_deviation_mkm)
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal))
    }
}

/// Calculate perpendicular distance from a point to a line segment.
fn perpendicular_distance_to_line(
    point: &crate::Point3D,
    line_start: &crate::Point3D,
    line_end: &crate::Point3D,
) -> f64 {
    let dx = line_end.x - line_start.x;
    let dy = line_end.y - line_start.y;
    let dz = line_end.z - line_start.z;

    let length_sq = dx * dx + dy * dy + dz * dz;

    if length_sq == 0.0 {
        // Line segment is actually a point
        let pdx = point.x - line_start.x;
        let pdy = point.y - line_start.y;
        let pdz = point.z - line_start.z;
        return (pdx * pdx + pdy * pdy + pdz * pdz).sqrt();
    }

    // Find projection of point onto line
    let t = ((point.x - line_start.x) * dx
        + (point.y - line_start.y) * dy
        + (point.z - line_start.z) * dz)
        / length_sq;

    let t = t.clamp(0.0, 1.0);

    let proj_x = line_start.x + t * dx;
    let proj_y = line_start.y + t * dy;
    let proj_z = line_start.z + t * dz;

    let dist_x = point.x - proj_x;
    let dist_y = point.y - proj_y;
    let dist_z = point.z - proj_z;

    (dist_x * dist_x + dist_y * dist_y + dist_z * dist_z).sqrt()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod fuel_station_tests {
    use super::*;

    #[test]
    fn test_fuel_station_index_creation() {
        let terminals = vec![
            api_client::Terminal {
                id: 1,
                code: Some("HUR-L1".to_string()),
                name: Some("HUR-L1".to_string()),
                nickname: None,
                star_system_name: Some("Stanton".to_string()),
                planet_name: None,
                moon_name: None,
                space_station_name: None,
                outpost_name: None,
                city_name: None,
                terminal_type: None,
                has_freight_elevator: false,
                has_loading_dock: false,
                has_docking_port: true,
                is_refuel: true,
                is_refinery: false,
            },
            api_client::Terminal {
                id: 2,
                code: Some("CRU-L1".to_string()),
                name: Some("CRU-L1".to_string()),
                nickname: None,
                star_system_name: Some("Stanton".to_string()),
                planet_name: None,
                moon_name: None,
                space_station_name: None,
                outpost_name: None,
                city_name: None,
                terminal_type: None,
                has_freight_elevator: false,
                has_loading_dock: false,
                has_docking_port: true,
                is_refuel: false, // Not a fuel station
                is_refinery: false,
            },
            api_client::Terminal {
                id: 3,
                code: Some("MIC-L1".to_string()),
                name: Some("MIC-L1".to_string()),
                nickname: None,
                star_system_name: Some("Stanton".to_string()),
                planet_name: None,
                moon_name: None,
                space_station_name: None,
                outpost_name: None,
                city_name: None,
                terminal_type: None,
                has_freight_elevator: false,
                has_loading_dock: false,
                has_docking_port: true,
                is_refuel: true,
                is_refinery: false,
            },
        ];

        let index = FuelStationIndex::from_terminals(&terminals);

        // Should only include terminals where is_refuel is true
        assert_eq!(index.all_stations().len(), 2);

        let stanton_stations = index.stations_in_system("Stanton");
        assert_eq!(stanton_stations.len(), 2);

        let pyro_stations = index.stations_in_system("Pyro");
        assert_eq!(pyro_stations.len(), 0);
    }

    #[test]
    fn test_find_nearest_fuel_station() {
        let terminals = vec![
            api_client::Terminal {
                id: 1,
                code: Some("Hurston".to_string()),
                name: Some("Hurston".to_string()),
                nickname: None,
                star_system_name: Some("Stanton".to_string()),
                planet_name: None,
                moon_name: None,
                space_station_name: None,
                outpost_name: None,
                city_name: None,
                terminal_type: None,
                has_freight_elevator: false,
                has_loading_dock: false,
                has_docking_port: true,
                is_refuel: true,
                is_refinery: false,
            },
            api_client::Terminal {
                id: 2,
                code: Some("Crusader".to_string()),
                name: Some("Crusader".to_string()),
                nickname: None,
                star_system_name: Some("Stanton".to_string()),
                planet_name: None,
                moon_name: None,
                space_station_name: None,
                outpost_name: None,
                city_name: None,
                terminal_type: None,
                has_freight_elevator: false,
                has_loading_dock: false,
                has_docking_port: true,
                is_refuel: true,
                is_refinery: false,
            },
        ];

        let index = FuelStationIndex::from_terminals(&terminals);

        // Position close to Hurston
        let test_pos = crate::Point3D::new(12.0, 0.0, 0.0);

        if let Some((station, distance)) = index.find_nearest(&test_pos) {
            assert!(station.name.contains("Hurston") || distance < 5.0);
        }
    }
}

/// A waypoint in a multi-hop route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    /// Location name/code.
    pub location: String,
    /// Whether this is a refueling stop.
    pub needs_refuel: bool,
    /// Distance to this waypoint from previous location (Mkm).
    pub distance_from_prev: f64,
    /// Cumulative distance from origin (Mkm).
    pub cumulative_distance: f64,
}

/// Find a route with refueling waypoints if needed.
///
/// Returns a list of waypoints from origin to destination.
/// If the ship can complete the route without refueling, returns just origin and destination.
/// Otherwise, inserts refueling stops at appropriate fuel stations along the way.
///
/// # Arguments
/// * `origin` - Starting location name
/// * `dest` - Destination location name  
/// * `fuel_capacity` - Ship's quantum fuel capacity (units)
/// * `efficiency` - Ship's quantum drive efficiency
/// * `fuel_index` - Index of available fuel stations
///
/// # Returns
/// * `Ok(Vec<Waypoint>)` - List of waypoints including refuel stops if needed
/// * `Err(String)` - If route cannot be completed (no positions found, no fuel stations available, etc.)
///
/// # Errors
///
/// Returns an error if positions cannot be resolved or no refueling route exists.
pub fn find_route_with_refueling(
    origin: &str,
    dest: &str,
    fuel_capacity: f64,
    efficiency: &QtDriveEfficiency,
    fuel_index: &FuelStationIndex,
) -> Result<Vec<Waypoint>, String> {
    use crate::locations::{distance_between, estimate_position};

    // Get positions
    let origin_pos =
        estimate_position(origin).ok_or_else(|| format!("Unknown origin location: {}", origin))?;
    let dest_pos =
        estimate_position(dest).ok_or_else(|| format!("Unknown destination location: {}", dest))?;

    let total_distance = distance_between(origin, dest)
        .ok_or_else(|| "Could not calculate distance between locations".to_string())?;

    // Check if we can make it without refueling
    let max_range = max_range_mkm(fuel_capacity, efficiency);

    if total_distance <= max_range {
        // Direct route - no refueling needed
        return Ok(vec![
            Waypoint {
                location: origin.to_string(),
                needs_refuel: false,
                distance_from_prev: 0.0,
                cumulative_distance: 0.0,
            },
            Waypoint {
                location: dest.to_string(),
                needs_refuel: false,
                distance_from_prev: total_distance,
                cumulative_distance: total_distance,
            },
        ]);
    }

    // Need refueling - find stations along the route
    let mut waypoints = vec![Waypoint {
        location: origin.to_string(),
        needs_refuel: false,
        distance_from_prev: 0.0,
        cumulative_distance: 0.0,
    }];

    let mut current_pos = origin_pos;
    let mut current_name = origin;
    let mut remaining_fuel = fuel_capacity;
    let mut cumulative_dist = 0.0;

    // Keep finding refuel stops until we reach destination
    let max_iterations = 10; // Prevent infinite loops
    for _ in 0..max_iterations {
        // Find nearest fuel station on route to destination
        let station_opt = fuel_index.find_nearest_on_route(&current_pos, &dest_pos, max_range);

        let (best_station, _deviation) = station_opt.ok_or_else(|| {
            format!(
                "No fuel stations found within range ({:.1} Mkm) from {}",
                max_range, current_name
            )
        })?;

        let station_pos = best_station
            .position
            .as_ref()
            .ok_or_else(|| format!("Fuel station {} has no position", best_station.name))?;

        let dist_to_station = current_pos.distance_to(station_pos);

        // Check if we can reach this station with remaining fuel
        let fuel_needed = calculate_qt_fuel_consumption(dist_to_station, efficiency);
        if fuel_needed > remaining_fuel {
            return Err(format!(
                "Cannot reach next fuel station {} - need {:.0} units but only {:.0} remaining",
                best_station.name, fuel_needed, remaining_fuel
            ));
        }

        cumulative_dist += dist_to_station;

        waypoints.push(Waypoint {
            location: best_station.name.clone(),
            needs_refuel: true,
            distance_from_prev: dist_to_station,
            cumulative_distance: cumulative_dist,
        });

        // Check if we can reach destination from this fuel station
        let dist_to_dest = station_pos.distance_to(&dest_pos);
        if dist_to_dest <= max_range {
            // Final hop to destination
            cumulative_dist += dist_to_dest;
            waypoints.push(Waypoint {
                location: dest.to_string(),
                needs_refuel: false,
                distance_from_prev: dist_to_dest,
                cumulative_distance: cumulative_dist,
            });
            return Ok(waypoints);
        }

        // Update current position for next iteration
        current_pos = *station_pos;
        current_name = &best_station.name;
        remaining_fuel = fuel_capacity; // Assume full refuel
    }

    Err(format!(
        "Could not find route to {} - exceeded maximum refueling stops",
        dest
    ))
}

/// Typical hydrogen fuel price per unit (aUEC).
///
/// **WARNING: PLACEHOLDER VALUE - NEEDS REAL DATA SOURCE**
///
/// This is an estimated value pending:
/// - In-game measurement at refueling stations
/// - UEX API commodity data for hydrogen fuel (if available)
/// - Community data sources for fuel pricing
///
/// TODO: Document actual source once verified
pub const HYDROGEN_FUEL_PRICE_PER_UNIT: f64 = 1.0;

/// Typical quantum fuel price per unit (aUEC).
///
/// **WARNING: PLACEHOLDER VALUE - NEEDS REAL DATA SOURCE**
///
/// This is an estimated value pending:
/// - In-game measurement at refueling stations  
/// - UEX API commodity data for quantum fuel (if available)
/// - Community data sources for fuel pricing
///
/// Quantum fuel is assumed more expensive than hydrogen based on in-game
/// scarcity and usage patterns, but the multiplier is not verified.
///
/// TODO: Document actual source once verified
pub const QUANTUM_FUEL_PRICE_PER_UNIT: f64 = 1.5;

/// Calculate the cost to refuel quantum fuel at a station.
///
/// # Arguments
/// * `fuel_needed` - Quantum fuel units needed
/// * `price_per_unit` - Optional custom price per unit (defaults to standard rate)
///
/// # Returns
/// Cost in aUEC to refuel
///
/// # Example
/// ```
/// use route_graph::fuel::calculate_refuel_cost;
///
/// let cost = calculate_refuel_cost(2000.0, None);
/// assert_eq!(cost, 3000.0); // 2000 units * 1.5 aUEC/unit
/// ```
#[must_use]
pub fn calculate_refuel_cost(fuel_needed: f64, price_per_unit: Option<f64>) -> f64 {
    let price = price_per_unit.unwrap_or(QUANTUM_FUEL_PRICE_PER_UNIT);
    fuel_needed * price
}

/// Calculate total refueling cost for a multi-hop route.
///
/// # Arguments
/// * `waypoints` - Route waypoints from `find_route_with_refueling`
/// * `fuel_capacity` - Ship's quantum fuel tank capacity
/// * `efficiency` - Ship's quantum drive efficiency
/// * `price_per_unit` - Optional custom fuel price (defaults to standard rate)
///
/// # Returns
/// Total refueling cost in aUEC
///
/// # Example
/// ```
/// use route_graph::fuel::{find_route_with_refueling, calculate_route_refuel_cost, efficiency_for_size, FuelStationIndex};
/// use route_graph::Point3D;
///
/// let efficiency = efficiency_for_size(2).unwrap();
/// let fuel_capacity = 2500.0;
///
/// // Mock fuel index for example
/// let terminals = vec![];
/// let index = FuelStationIndex::from_terminals(&terminals);
///
/// // In real usage, you'd get waypoints from find_route_with_refueling
/// // let waypoints = find_route_with_refueling("Hurston", "microTech", fuel_capacity, efficiency, &index)?;
/// // let cost = calculate_route_refuel_cost(&waypoints, fuel_capacity, efficiency, None);
/// ```
#[must_use]
pub fn calculate_route_refuel_cost(
    waypoints: &[Waypoint],
    fuel_capacity: f64,
    efficiency: &QtDriveEfficiency,
    price_per_unit: Option<f64>,
) -> f64 {
    let mut total_cost = 0.0;
    let mut current_fuel = fuel_capacity;

    for waypoint in waypoints {
        let fuel_consumed = calculate_qt_fuel_consumption(waypoint.distance_from_prev, efficiency);
        current_fuel -= fuel_consumed;

        if waypoint.needs_refuel {
            // Refuel to full capacity
            let fuel_needed = fuel_capacity - current_fuel;
            total_cost += calculate_refuel_cost(fuel_needed, price_per_unit);
            current_fuel = fuel_capacity;
        }
    }

    total_cost
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod refuel_cost_tests {
    use super::*;

    #[test]
    fn test_calculate_refuel_cost_default() {
        let cost = calculate_refuel_cost(2000.0, None);
        assert_eq!(cost, 3000.0); // 2000 * 1.5
    }

    #[test]
    fn test_calculate_refuel_cost_custom() {
        let cost = calculate_refuel_cost(1000.0, Some(2.0));
        assert_eq!(cost, 2000.0);
    }

    #[test]
    fn test_route_refuel_cost_no_stops() {
        let efficiency = efficiency_for_size(2).unwrap();
        let waypoints = vec![Waypoint {
            location: "Destination".to_string(),
            needs_refuel: false,
            distance_from_prev: 10.0,
            cumulative_distance: 10.0,
        }];

        let cost = calculate_route_refuel_cost(&waypoints, 5000.0, efficiency, None);
        assert_eq!(cost, 0.0); // No refueling needed
    }

    #[test]
    fn test_route_refuel_cost_one_stop() {
        let efficiency = efficiency_for_size(2).unwrap();
        let waypoints = vec![
            Waypoint {
                location: "Fuel Station".to_string(),
                needs_refuel: true,
                distance_from_prev: 20.0,
                cumulative_distance: 20.0,
            },
            Waypoint {
                location: "Destination".to_string(),
                needs_refuel: false,
                distance_from_prev: 20.0,
                cumulative_distance: 40.0,
            },
        ];

        // Start with 2500, use 1600 (20 * 80), refuel 1600
        let cost = calculate_route_refuel_cost(&waypoints, 2500.0, efficiency, None);
        assert_eq!(cost, 2400.0); // 1600 units * 1.5
    }
}
