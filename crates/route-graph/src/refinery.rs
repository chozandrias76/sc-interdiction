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
    /// Cost per SCU of material processed (in aUEC)
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
    #[must_use]
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
    #[must_use]
    pub fn all_refineries(&self) -> &[Refinery] {
        &self.refineries
    }

    /// Get refineries in a specific system.
    #[must_use]
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
    #[must_use]
    pub fn find_nearest(&self, position: &Point3D) -> Option<(&Refinery, f64)> {
        self.refineries
            .iter()
            .filter_map(|r| {
                r.position.as_ref().map(|pos| {
                    // Use distance_squared for comparison (avoids sqrt)
                    let dist_sq = position.distance_squared(pos);
                    (r, pos, dist_sq)
                })
            })
            .min_by(|(_, _, d1), (_, _, d2)| {
                d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(r, pos, _)| {
                // Only compute actual distance for the result
                (r, position.distance_to(pos))
            })
    }

    /// Find the nearest refinery along a route path.
    /// Uses perpendicular distance to find refineries close to the route.
    ///
    /// # Arguments
    /// * `start` - Route start position
    /// * `end` - Route end position
    /// * `max_deviation_mkm` - Maximum perpendicular distance from route (in Mkm)
    #[must_use]
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
    #![allow(clippy::indexing_slicing)]
    #![allow(clippy::unwrap_used)]

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

    // RefineryIndex tests
    #[test]
    fn test_from_terminals_filters_refineries() {
        let terminals = vec![
            create_test_terminal(1, "Refinery Alpha", "REFA", true),
            create_test_terminal(2, "Port Olisar", "PO", false),
            create_test_terminal(3, "Refinery Beta", "REFB", true),
        ];

        let index = RefineryIndex::from_terminals(&terminals);

        // Only is_refinery=true terminals should be included
        assert_eq!(index.all_refineries().len(), 2);
    }

    #[test]
    fn test_from_terminals_empty_input() {
        let terminals: Vec<api_client::Terminal> = vec![];
        let index = RefineryIndex::from_terminals(&terminals);

        // Empty input → empty index
        assert_eq!(index.all_refineries().len(), 0);
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
    fn test_all_refineries_returns_all() {
        let terminals = vec![
            create_test_terminal(1, "Refinery Alpha", "REFA", true),
            create_test_terminal(2, "Refinery Beta", "REFB", true),
            create_test_terminal(3, "Refinery Gamma", "REFG", true),
        ];

        let index = RefineryIndex::from_terminals(&terminals);
        let all = index.all_refineries();

        assert_eq!(all.len(), 3);
        // Verify all refineries are accessible
        let names: Vec<&str> = all.iter().map(|r| r.name.as_str()).collect();
        assert!(names.contains(&"Refinery Alpha"));
        assert!(names.contains(&"Refinery Beta"));
        assert!(names.contains(&"Refinery Gamma"));
    }

    #[test]
    fn test_refineries_in_system() {
        let terminals = vec![create_test_terminal(1, "Stanton Refinery", "SREF", true)];

        let index = RefineryIndex::from_terminals(&terminals);
        let stanton_refineries = index.refineries_in_system("Stanton");

        assert_eq!(stanton_refineries.len(), 1);
        assert_eq!(stanton_refineries[0].name, "Stanton Refinery");
    }

    // Static data tests for REFINERY_METHODS
    #[test]
    fn test_refinery_methods_count() {
        assert_eq!(REFINERY_METHODS.len(), 3);
    }

    #[test]
    fn test_refinery_methods_yield_range() {
        // All yields should be between 0.0 and 1.0
        for method in REFINERY_METHODS.iter() {
            assert!(
                method.yield_percentage >= 0.0 && method.yield_percentage <= 1.0,
                "Yield {} for method '{}' is out of range [0.0, 1.0]",
                method.yield_percentage,
                method.name
            );
        }
    }

    #[test]
    fn test_refinery_methods_ordered_by_time() {
        // Verify processing_time_hours ordering: Fast < Standard < Maximum
        let fast = &REFINERY_METHODS[1]; // Fast Track
        let standard = &REFINERY_METHODS[0]; // Standard
        let max = &REFINERY_METHODS[2]; // Maximum Yield

        assert!(
            fast.processing_time_hours < standard.processing_time_hours,
            "Fast Track ({}) should be faster than Standard ({})",
            fast.processing_time_hours,
            standard.processing_time_hours
        );
        assert!(
            standard.processing_time_hours < max.processing_time_hours,
            "Standard ({}) should be faster than Maximum Yield ({})",
            standard.processing_time_hours,
            max.processing_time_hours
        );
    }

    #[test]
    fn test_refinery_methods_values() {
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

    #[test]
    fn test_perpendicular_distance_point_on_line() {
        // Point exactly on the line should have zero distance
        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(10.0, 0.0, 0.0);
        let point = Point3D::new(5.0, 0.0, 0.0);

        let dist = perpendicular_distance_to_line(&point, &start, &end);
        assert!(dist < 1e-10, "Point on line should have ~0 distance");
    }

    #[test]
    fn test_perpendicular_distance_perpendicular_point() {
        // Point 5 units perpendicular to the line
        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(10.0, 0.0, 0.0);
        let point = Point3D::new(5.0, 5.0, 0.0);

        let dist = perpendicular_distance_to_line(&point, &start, &end);
        assert!(
            (dist - 5.0).abs() < 1e-10,
            "Expected distance of 5.0, got {}",
            dist
        );
    }

    #[test]
    fn test_perpendicular_distance_beyond_segment_start() {
        // Point beyond the start of the segment
        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(10.0, 0.0, 0.0);
        let point = Point3D::new(-3.0, 4.0, 0.0); // 5 units from start (3-4-5 triangle)

        let dist = perpendicular_distance_to_line(&point, &start, &end);
        assert!(
            (dist - 5.0).abs() < 1e-10,
            "Expected distance of 5.0, got {}",
            dist
        );
    }

    #[test]
    fn test_perpendicular_distance_beyond_segment_end() {
        // Point beyond the end of the segment
        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(10.0, 0.0, 0.0);
        let point = Point3D::new(13.0, 4.0, 0.0); // 5 units from end (3-4-5 triangle)

        let dist = perpendicular_distance_to_line(&point, &start, &end);
        assert!(
            (dist - 5.0).abs() < 1e-10,
            "Expected distance of 5.0, got {}",
            dist
        );
    }

    #[test]
    fn test_perpendicular_distance_degenerate_line() {
        // Line segment is a point (start == end)
        let start = Point3D::new(5.0, 5.0, 5.0);
        let end = Point3D::new(5.0, 5.0, 5.0);
        let point = Point3D::new(8.0, 9.0, 5.0); // 5 units away (3-4-5 triangle)

        let dist = perpendicular_distance_to_line(&point, &start, &end);
        assert!(
            (dist - 5.0).abs() < 1e-10,
            "Expected distance of 5.0, got {}",
            dist
        );
    }

    fn create_refinery_with_position(name: &str, x: f64, y: f64, z: f64) -> Refinery {
        Refinery {
            name: name.to_string(),
            code: Some(format!("{}_CODE", name)),
            system: Some("Stanton".to_string()),
            position: Some(Point3D::new(x, y, z)),
            methods: REFINERY_METHODS.to_vec(),
        }
    }

    #[test]
    fn test_find_nearest_single_refinery() {
        // Single refinery should always be returned when position is valid
        let index = RefineryIndex {
            refineries: vec![create_refinery_with_position(
                "Only Refinery",
                10.0,
                20.0,
                30.0,
            )],
        };

        let position = Point3D::new(0.0, 0.0, 0.0);
        let result = index.find_nearest(&position);

        assert!(result.is_some());
        let (refinery, distance) = result.unwrap();
        assert_eq!(refinery.name, "Only Refinery");
        // Distance should be sqrt(10^2 + 20^2 + 30^2) = sqrt(1400) ≈ 37.42
        assert!((distance - 37.416_573_867_739_41).abs() < 0.001);
    }

    #[test]
    fn test_find_nearest_on_route_within_threshold() {
        // Create an index with a refinery close to the route
        let index = RefineryIndex {
            refineries: vec![
                create_refinery_with_position("Close Refinery", 5.0, 1.0, 0.0), // 1 unit from route
                create_refinery_with_position("Far Refinery", 5.0, 10.0, 0.0), // 10 units from route
            ],
        };

        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(10.0, 0.0, 0.0);

        // Max deviation of 2.0 should find the close refinery
        let result = index.find_nearest_on_route(&start, &end, 2.0);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0.name, "Close Refinery");
    }

    #[test]
    fn test_find_nearest_on_route_outside_threshold() {
        let index = RefineryIndex {
            refineries: vec![create_refinery_with_position(
                "Far Refinery",
                5.0,
                10.0,
                0.0,
            )],
        };

        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(10.0, 0.0, 0.0);

        // Max deviation of 5.0 should not find the refinery at 10 units away
        let result = index.find_nearest_on_route(&start, &end, 5.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_nearest_on_route_selects_closest_to_start() {
        let index = RefineryIndex {
            refineries: vec![
                create_refinery_with_position("Near Start", 2.0, 0.5, 0.0),
                create_refinery_with_position("Near End", 8.0, 0.5, 0.0),
            ],
        };

        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(10.0, 0.0, 0.0);

        // Both are within threshold, should return the one closest to start
        let result = index.find_nearest_on_route(&start, &end, 1.0);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0.name, "Near Start");
    }

    #[test]
    fn test_from_terminals_uses_nickname_fallback() {
        let terminal = api_client::Terminal {
            id: 1,
            name: None,
            nickname: Some("Nickname Refinery".to_string()),
            code: Some("REF".to_string()),
            terminal_type: None,
            star_system_name: Some("Stanton".to_string()),
            planet_name: None,
            moon_name: None,
            space_station_name: None,
            outpost_name: None,
            city_name: None,
            has_freight_elevator: false,
            has_loading_dock: false,
            has_docking_port: false,
            is_refuel: false,
            is_refinery: true,
        };

        let index = RefineryIndex::from_terminals(&[terminal]);
        assert_eq!(index.all_refineries()[0].name, "Nickname Refinery");
    }

    #[test]
    fn test_from_terminals_uses_id_fallback() {
        let terminal = api_client::Terminal {
            id: 42,
            name: None,
            nickname: None,
            code: Some("REF".to_string()),
            terminal_type: None,
            star_system_name: Some("Stanton".to_string()),
            planet_name: None,
            moon_name: None,
            space_station_name: None,
            outpost_name: None,
            city_name: None,
            has_freight_elevator: false,
            has_loading_dock: false,
            has_docking_port: false,
            is_refuel: false,
            is_refinery: true,
        };

        let index = RefineryIndex::from_terminals(&[terminal]);
        assert_eq!(index.all_refineries()[0].name, "Refinery 42");
    }

    #[test]
    fn test_refineries_in_system_case_insensitive() {
        let terminals = vec![create_test_terminal(1, "Test Refinery", "REF", true)];
        let index = RefineryIndex::from_terminals(&terminals);

        assert_eq!(index.refineries_in_system("STANTON").len(), 1);
        assert_eq!(index.refineries_in_system("stanton").len(), 1);
        assert_eq!(index.refineries_in_system("StAnToN").len(), 1);
    }

    #[test]
    fn test_refineries_in_system_no_match() {
        let terminals = vec![create_test_terminal(1, "Test Refinery", "REF", true)];
        let index = RefineryIndex::from_terminals(&terminals);

        assert_eq!(index.refineries_in_system("Pyro").len(), 0);
        assert_eq!(index.refineries_in_system("Unknown").len(), 0);
    }

    #[test]
    fn test_find_nearest_multiple_refineries() {
        let index = RefineryIndex {
            refineries: vec![
                create_refinery_with_position("Far", 100.0, 0.0, 0.0),
                create_refinery_with_position("Close", 5.0, 0.0, 0.0),
                create_refinery_with_position("Medium", 50.0, 0.0, 0.0),
            ],
        };

        let position = Point3D::new(0.0, 0.0, 0.0);
        let result = index.find_nearest(&position);

        assert!(result.is_some());
        let (refinery, distance) = result.unwrap();
        assert_eq!(refinery.name, "Close");
        assert!((distance - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_find_nearest_empty_index() {
        let index = RefineryIndex { refineries: vec![] };
        let position = Point3D::new(0.0, 0.0, 0.0);

        assert!(index.find_nearest(&position).is_none());
    }

    #[test]
    fn test_find_nearest_no_positions() {
        let refinery = Refinery {
            name: "No Position".to_string(),
            code: None,
            system: Some("Stanton".to_string()),
            position: None,
            methods: REFINERY_METHODS.to_vec(),
        };

        let index = RefineryIndex {
            refineries: vec![refinery],
        };
        let position = Point3D::new(0.0, 0.0, 0.0);

        assert!(index.find_nearest(&position).is_none());
    }

    #[test]
    fn test_find_nearest_on_route_empty_index() {
        let index = RefineryIndex { refineries: vec![] };

        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(10.0, 0.0, 0.0);

        assert!(index.find_nearest_on_route(&start, &end, 100.0).is_none());
    }

    #[test]
    fn test_refinery_struct_debug() {
        let refinery = create_refinery_with_position("Debug Test", 1.0, 2.0, 3.0);
        let debug_str = format!("{:?}", refinery);

        assert!(debug_str.contains("Debug Test"));
        assert!(debug_str.contains("Refinery"));
    }

    #[test]
    fn test_refinery_index_debug() {
        let index = RefineryIndex {
            refineries: vec![create_refinery_with_position("Test", 0.0, 0.0, 0.0)],
        };
        let debug_str = format!("{:?}", index);

        assert!(debug_str.contains("RefineryIndex"));
    }

    #[test]
    fn test_refinery_method_struct_debug() {
        let method = &REFINERY_METHODS[0];
        let debug_str = format!("{:?}", method);

        assert!(debug_str.contains("Standard"));
        assert!(debug_str.contains("RefineryMethod"));
    }

    #[test]
    fn test_refinery_clone() {
        let refinery = create_refinery_with_position("Clone Test", 5.0, 10.0, 15.0);
        let cloned = refinery.clone();

        assert_eq!(cloned.name, "Clone Test");
        assert_eq!(cloned.system, Some("Stanton".to_string()));
        assert!(cloned.position.is_some());
    }

    #[test]
    fn test_refinery_index_clone() {
        let index = RefineryIndex {
            refineries: vec![create_refinery_with_position("Original", 0.0, 0.0, 0.0)],
        };
        let cloned = index.clone();

        assert_eq!(cloned.all_refineries().len(), 1);
        assert_eq!(cloned.all_refineries()[0].name, "Original");
    }
}
