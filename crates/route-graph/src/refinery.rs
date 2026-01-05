//! Refinery location indexing and processing methods.
//!
//! Provides structures to index refineries and their processing capabilities.
//! Refineries process raw materials (ore, RMC, CMATs) into refined commodities.

use crate::Point3D;
use serde::Serialize;

/// A refinery processing method with yield and cost information.
#[derive(Debug, Clone, Serialize)]
pub struct RefineryMethod {
    pub name: &'static str,
    /// Yield percentage (0.0-1.0). E.g., 0.75 = 75% yield
    pub yield_percentage: f64,
    /// Processing time in hours
    pub processing_time_hours: f64,
    /// Cost per SCU of material processed
    pub cost_per_scu: f64,
}

/// Standard refinery processing methods available across Stanton.
///
/// Note: These are placeholder values. Actual refinement methods and yields
/// should be researched from Star Citizen databases and player data.
pub static REFINERY_METHODS: &[RefineryMethod] = &[
    RefineryMethod {
        name: "Standard",
        yield_percentage: 0.75,
        processing_time_hours: 6.0,
        cost_per_scu: 15.0,
    },
    RefineryMethod {
        name: "Fast Track",
        yield_percentage: 0.50,
        processing_time_hours: 1.0,
        cost_per_scu: 30.0,
    },
    RefineryMethod {
        name: "Maximum Yield",
        yield_percentage: 0.90,
        processing_time_hours: 24.0,
        cost_per_scu: 50.0,
    },
];

/// A refinery location with processing capabilities.
#[derive(Debug, Clone, Serialize)]
pub struct Refinery {
    pub name: String,
    pub code: Option<String>,
    pub system: Option<String>,
    pub position: Option<Point3D>,
    pub methods: Vec<RefineryMethod>,
}

/// Spatial index of refinery locations.
#[derive(Debug, Clone)]
pub struct RefineryIndex {
    refineries: Vec<Refinery>,
}

impl RefineryIndex {
    /// Create a refinery index from terminal data.
    /// Filters terminals by `is_refinery = true` flag.
    pub fn from_terminals(terminals: &[api_client::Terminal]) -> Self {
        let refineries = terminals
            .iter()
            .filter(|t| t.is_refinery)
            .map(|t| {
                let name = t
                    .name
                    .clone()
                    .or_else(|| t.nickname.clone())
                    .unwrap_or_else(|| format!("Refinery {}", t.id));

                let position = t
                    .code
                    .as_ref()
                    .and_then(|code| crate::estimate_position(code));

                Refinery {
                    name,
                    code: t.code.clone(),
                    system: t.star_system_name.clone(),
                    position,
                    methods: REFINERY_METHODS.to_vec(),
                }
            })
            .collect();

        Self { refineries }
    }

    /// Get all refineries.
    pub fn all_refineries(&self) -> &[Refinery] {
        &self.refineries
    }

    /// Get refineries in a specific system.
    pub fn refineries_in_system(&self, system: &str) -> Vec<&Refinery> {
        self.refineries
            .iter()
            .filter(|r| {
                r.system
                    .as_ref()
                    .map(|s| s.eq_ignore_ascii_case(system))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Find the nearest refinery to a given position.
    /// Returns the refinery and distance in millions of km.
    pub fn find_nearest(&self, position: &Point3D) -> Option<(&Refinery, f64)> {
        self.refineries
            .iter()
            .filter_map(|r| {
                r.position.as_ref().map(|pos| {
                    let distance = position.distance_to(pos);
                    (r, distance)
                })
            })
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Find the nearest refinery along a route path.
    /// Uses perpendicular distance to find refineries close to the route.
    ///
    /// # Arguments
    /// * `start` - Route start position
    /// * `end` - Route end position
    /// * `max_deviation_mkm` - Maximum perpendicular distance from route (in Mkm)
    pub fn find_nearest_on_route(
        &self,
        start: &Point3D,
        end: &Point3D,
        max_deviation_mkm: f64,
    ) -> Option<(&Refinery, f64)> {
        self.refineries
            .iter()
            .filter_map(|r| {
                r.position.as_ref().and_then(|pos| {
                    let perp_dist = perpendicular_distance_to_line(pos, start, end);
                    if perp_dist <= max_deviation_mkm {
                        let distance = pos.distance_to(start);
                        Some((r, distance))
                    } else {
                        None
                    }
                })
            })
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal))
    }
}

/// Calculate perpendicular distance from a point to a line segment.
fn perpendicular_distance_to_line(
    point: &Point3D,
    line_start: &Point3D,
    line_end: &Point3D,
) -> f64 {
    let line_vec_x = line_end.x - line_start.x;
    let line_vec_y = line_end.y - line_start.y;
    let line_vec_z = line_end.z - line_start.z;

    let point_vec_x = point.x - line_start.x;
    let point_vec_y = point.y - line_start.y;
    let point_vec_z = point.z - line_start.z;

    let line_length_sq =
        line_vec_x * line_vec_x + line_vec_y * line_vec_y + line_vec_z * line_vec_z;

    if line_length_sq < 1e-10 {
        // Line segment is actually a point
        return point.distance_to(line_start);
    }

    // Project point onto line segment (parameter t from 0 to 1)
    let t = ((point_vec_x * line_vec_x + point_vec_y * line_vec_y + point_vec_z * line_vec_z)
        / line_length_sq)
        .clamp(0.0, 1.0);

    // Find closest point on line segment
    let closest_x = line_start.x + t * line_vec_x;
    let closest_y = line_start.y + t * line_vec_y;
    let closest_z = line_start.z + t * line_vec_z;

    let closest = Point3D::new(closest_x, closest_y, closest_z);
    point.distance_to(&closest)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_terminal(
        id: i64,
        name: &str,
        code: &str,
        is_refinery: bool,
    ) -> api_client::Terminal {
        api_client::Terminal {
            id,
            name: Some(name.to_string()),
            code: Some(code.to_string()),
            nickname: None,
            terminal_type: Some("STATION".to_string()),
            star_system_name: Some("Stanton".to_string()),
            planet_name: Some("Hurston".to_string()),
            moon_name: None,
            space_station_name: None,
            outpost_name: None,
            city_name: None,
            has_freight_elevator: false,
            has_loading_dock: false,
            has_docking_port: false,
            is_refuel: false,
            is_refinery,
        }
    }

    #[test]
    fn test_refinery_index_from_terminals() {
        let terminals = vec![
            create_test_terminal(1, "Refinery Alpha", "REFA", true),
            create_test_terminal(2, "Port Olisar", "PO", false),
            create_test_terminal(3, "Refinery Beta", "REFB", true),
        ];

        let index = RefineryIndex::from_terminals(&terminals);

        assert_eq!(index.all_refineries().len(), 2);
    }

    #[test]
    fn test_refinery_filtering() {
        let terminals = vec![
            create_test_terminal(1, "Refinery Station", "REF1", true),
            create_test_terminal(2, "Cargo Station", "CARGO", false),
        ];

        let index = RefineryIndex::from_terminals(&terminals);

        // Only refinery terminals should be included
        assert_eq!(index.all_refineries().len(), 1);
        assert_eq!(index.all_refineries()[0].name, "Refinery Station");
    }

    #[test]
    fn test_refineries_in_system() {
        let terminals = vec![create_test_terminal(1, "Stanton Refinery", "SREF", true)];

        let index = RefineryIndex::from_terminals(&terminals);
        let stanton_refineries = index.refineries_in_system("Stanton");

        assert_eq!(stanton_refineries.len(), 1);
        assert_eq!(stanton_refineries[0].name, "Stanton Refinery");
    }

    #[test]
    fn test_refinery_methods() {
        assert_eq!(REFINERY_METHODS.len(), 3);

        let standard = &REFINERY_METHODS[0];
        assert_eq!(standard.name, "Standard");
        assert_eq!(standard.yield_percentage, 0.75);

        let fast = &REFINERY_METHODS[1];
        assert_eq!(fast.name, "Fast Track");
        assert_eq!(fast.yield_percentage, 0.50);

        let max = &REFINERY_METHODS[2];
        assert_eq!(max.name, "Maximum Yield");
        assert_eq!(max.yield_percentage, 0.90);
    }
}
