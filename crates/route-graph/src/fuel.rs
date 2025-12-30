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
    pub const fn new(name: &'static str, fuel_per_mkm: f64) -> Self {
        Self { name, fuel_per_mkm }
    }
}

/// Default quantum drive efficiency ratings by size class.
///
/// These are estimated values based on typical ship quantum drives.
/// Actual values may vary based on specific drive models.
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
pub fn max_range_mkm(fuel_capacity: f64, efficiency: &QtDriveEfficiency) -> f64 {
    if efficiency.fuel_per_mkm <= 0.0 {
        return 0.0;
    }
    fuel_capacity / efficiency.fuel_per_mkm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_qt_fuel_consumption() {
        let s2_efficiency = match efficiency_for_size(2) {
            Some(eff) => eff,
            None => {
                assert!(false, "S2 efficiency should be available");
                return;
            }
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
        let s1 = match efficiency_for_size(1) {
            Some(eff) => eff,
            None => {
                assert!(false, "S1 efficiency should be available");
                return;
            }
        };
        let s2 = match efficiency_for_size(2) {
            Some(eff) => eff,
            None => {
                assert!(false, "S2 efficiency should be available");
                return;
            }
        };
        let s3 = match efficiency_for_size(3) {
            Some(eff) => eff,
            None => {
                assert!(false, "S3 efficiency should be available");
                return;
            }
        };

        assert!(s1.fuel_per_mkm < s2.fuel_per_mkm);
        assert!(s2.fuel_per_mkm < s3.fuel_per_mkm);

        // Invalid sizes return None
        assert!(efficiency_for_size(0).is_none());
        assert!(efficiency_for_size(4).is_none());
    }

    #[test]
    fn test_can_complete_route() {
        let s2_efficiency = match efficiency_for_size(2) {
            Some(eff) => eff,
            None => {
                assert!(false, "S2 efficiency should be available");
                return;
            }
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
        let s2_efficiency = match efficiency_for_size(2) {
            Some(eff) => eff,
            None => {
                assert!(false, "S2 efficiency should be available");
                return;
            }
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
    pub fn all_stations(&self) -> &[FuelStation] {
        &self.stations
    }

    /// Get fuel stations in a specific star system.
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
    /// Returns (station, distance_mkm) or None if no stations have positions.
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
    /// Returns (station, distance_from_route_mkm) if one is found.
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
