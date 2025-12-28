//! Spatial indexing for nearest neighbor searches.
//!
//! Uses simple spatial partitioning for finding nearby hotspots.
//! Coordinates are in the Star Citizen universe coordinate system.

use crate::chokepoint::Chokepoint;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

/// A point in 3D space.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Calculate Euclidean distance to another point.
    pub fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Calculate distance squared (faster for comparisons).
    pub fn distance_squared(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

/// A spatial index for interdiction hotspots.
pub struct SpatialIndex {
    hotspots: Vec<IndexedHotspot>,
}

/// A hotspot with its position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedHotspot {
    pub position: Point3D,
    pub name: String,
    pub system: String,
    pub traffic_score: f64,
    /// Original chokepoint data.
    pub chokepoint: Chokepoint,
}

/// Result of a nearest neighbor search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbyHotspot {
    pub hotspot: IndexedHotspot,
    pub distance: f64,
}

impl SpatialIndex {
    /// Create a new empty spatial index.
    pub fn new() -> Self {
        Self {
            hotspots: Vec::new(),
        }
    }

    /// Add a hotspot to the index.
    pub fn insert(&mut self, hotspot: IndexedHotspot) {
        self.hotspots.push(hotspot);
    }

    /// Build index from chokepoints with coordinates.
    pub fn from_chokepoints(chokepoints: Vec<Chokepoint>) -> Self {
        let mut index = Self::new();

        for cp in chokepoints {
            // Use node coordinates if available, otherwise estimate from system
            let position = cp
                .node
                .coords
                .map(|(x, y, z)| Point3D::new(x, y, z))
                .unwrap_or_else(|| estimate_position(&cp.node.system, &cp.node.name));

            index.insert(IndexedHotspot {
                position,
                name: cp.node.name.clone(),
                system: cp.node.system.clone(),
                traffic_score: cp.traffic_score,
                chokepoint: cp,
            });
        }

        index
    }

    /// Find the k nearest hotspots to a point.
    pub fn find_nearest(&self, point: &Point3D, k: usize) -> Vec<NearbyHotspot> {
        let mut distances: Vec<_> = self
            .hotspots
            .iter()
            .map(|h| {
                let dist = point.distance_to(&h.position);
                (OrderedFloat(dist), h)
            })
            .collect();

        // Sort by distance
        distances.sort_by_key(|(d, _)| *d);

        // Take top k
        distances
            .into_iter()
            .take(k)
            .map(|(d, h)| NearbyHotspot {
                hotspot: h.clone(),
                distance: d.0,
            })
            .collect()
    }

    /// Find all hotspots within a radius.
    pub fn find_within_radius(&self, point: &Point3D, radius: f64) -> Vec<NearbyHotspot> {
        let radius_sq = radius * radius;

        self.hotspots
            .iter()
            .filter_map(|h| {
                let dist_sq = point.distance_squared(&h.position);
                if dist_sq <= radius_sq {
                    Some(NearbyHotspot {
                        hotspot: h.clone(),
                        distance: dist_sq.sqrt(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Find hotspots in a specific system.
    pub fn find_in_system(&self, system: &str) -> Vec<&IndexedHotspot> {
        self.hotspots
            .iter()
            .filter(|h| h.system.eq_ignore_ascii_case(system))
            .collect()
    }

    /// Get all hotspots sorted by traffic score.
    pub fn by_traffic(&self) -> Vec<&IndexedHotspot> {
        let mut sorted: Vec<_> = self.hotspots.iter().collect();
        sorted.sort_by(|a, b| {
            b.traffic_score
                .partial_cmp(&a.traffic_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted
    }

    /// Number of indexed hotspots.
    pub fn len(&self) -> usize {
        self.hotspots.len()
    }

    /// Check if index is empty.
    pub fn is_empty(&self) -> bool {
        self.hotspots.is_empty()
    }
}

impl Default for SpatialIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Estimate position from system and location name.
///
/// This is a fallback when coordinates aren't available.
/// Uses known approximate positions for major locations.
fn estimate_position(system: &str, location: &str) -> Point3D {
    // Stanton system approximate positions (in millions of km from system center)
    // These are rough estimates for demo purposes
    match system.to_uppercase().as_str() {
        "STANTON" => {
            let loc_lower = location.to_lowercase();
            if loc_lower.contains("hurston") || loc_lower.contains("lorville") {
                Point3D::new(12.0, 0.0, 0.0)
            } else if loc_lower.contains("crusader") || loc_lower.contains("orison") {
                Point3D::new(-6.0, 8.0, 0.0)
            } else if loc_lower.contains("arccorp") || loc_lower.contains("area18") {
                Point3D::new(-18.0, 0.0, 0.0)
            } else if loc_lower.contains("microtech") || loc_lower.contains("new babbage") {
                Point3D::new(0.0, 22.0, 0.0)
            } else if loc_lower.contains("port olisar") || loc_lower.contains("everus") {
                Point3D::new(-6.0, 7.0, 0.5)
            } else if loc_lower.contains("grim hex") {
                Point3D::new(15.0, 3.0, 0.0)
            } else {
                // Default to system center with some offset
                Point3D::new(0.0, 0.0, location.len() as f64)
            }
        }
        "PYRO" => {
            // Pyro is accessed via jump from Stanton
            Point3D::new(100.0, 0.0, 0.0)
        }
        _ => Point3D::new(0.0, 0.0, 0.0),
    }
}

/// A 3D line segment representing a trade route path.
#[derive(Debug, Clone)]
pub struct RouteSegment {
    pub origin: Point3D,
    pub destination: Point3D,
    pub origin_name: String,
    pub destination_name: String,
    pub cargo_value: f64,
    pub commodity: String,
    pub ship_name: String,
    pub threat_level: u8,
}

impl RouteSegment {
    /// Get the midpoint of the route.
    pub fn midpoint(&self) -> Point3D {
        Point3D::new(
            (self.origin.x + self.destination.x) / 2.0,
            (self.origin.y + self.destination.y) / 2.0,
            (self.origin.z + self.destination.z) / 2.0,
        )
    }

    /// Calculate the closest point on this segment to another segment.
    /// Returns the point on this segment closest to the other segment.
    pub fn closest_approach_to(&self, other: &RouteSegment) -> (Point3D, f64) {
        // Simplified: find closest approach between two line segments
        // Use parametric form: P(t) = origin + t*(dest - origin)
        let d1 = Point3D::new(
            self.destination.x - self.origin.x,
            self.destination.y - self.origin.y,
            self.destination.z - self.origin.z,
        );
        let d2 = Point3D::new(
            other.destination.x - other.origin.x,
            other.destination.y - other.origin.y,
            other.destination.z - other.origin.z,
        );
        let r = Point3D::new(
            self.origin.x - other.origin.x,
            self.origin.y - other.origin.y,
            self.origin.z - other.origin.z,
        );

        let a = dot(&d1, &d1);
        let b = dot(&d1, &d2);
        let c = dot(&d2, &d2);
        let d = dot(&d1, &r);
        let e = dot(&d2, &r);

        let denom = a * c - b * b;

        // If segments are parallel, use midpoints
        let (t, _s) = if denom.abs() < 1e-10 {
            (0.5, 0.5)
        } else {
            let t = ((b * e - c * d) / denom).clamp(0.0, 1.0);
            let s = ((a * e - b * d) / denom).clamp(0.0, 1.0);
            (t, s)
        };

        let closest_point = Point3D::new(
            self.origin.x + t * d1.x,
            self.origin.y + t * d1.y,
            self.origin.z + t * d1.z,
        );

        // Calculate actual closest distance
        let s_clamped = if denom.abs() < 1e-10 {
            0.5
        } else {
            ((a * e - b * d) / denom).clamp(0.0, 1.0)
        };
        let other_point = Point3D::new(
            other.origin.x + s_clamped * d2.x,
            other.origin.y + s_clamped * d2.y,
            other.origin.z + s_clamped * d2.z,
        );

        let distance = closest_point.distance_to(&other_point);
        (closest_point, distance)
    }

    /// Get the length of this route segment.
    pub fn length(&self) -> f64 {
        self.origin.distance_to(&self.destination)
    }
}

fn dot(a: &Point3D, b: &Point3D) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// A route intersection zone where multiple routes converge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteIntersection {
    /// Center point of the intersection zone.
    pub position: Point3D,
    /// Descriptive name for this intersection.
    pub name: String,
    /// System this intersection is in.
    pub system: String,
    /// Routes that pass through or near this intersection.
    pub intersecting_routes: Vec<IntersectingRoute>,
    /// Total combined cargo value of all intersecting routes.
    pub total_cargo_value: f64,
    /// Number of unique route pairs that intersect here.
    pub route_pair_count: usize,
    /// Average threat level of ships on these routes (1-10).
    pub avg_threat_level: f64,
    /// Average interdiction value per target - higher = better expected payout.
    /// Calculated as average of (cargo_value / threat) across all routes.
    /// Example: A zone with mostly C2 Hercules (threat 2) hauling 10M cargo
    /// has avg interdiction value of 5M per catch.
    /// A zone with Andromedas (threat 7) hauling same cargo = only 1.4M per catch.
    pub interdiction_value: f64,
    /// Suggested tactics based on threat level.
    pub suggested_tactics: String,
    /// Jump instructions: QT to this destination and exit early.
    pub jump_to: JumpInstruction,
}

/// Instructions for reaching an interdiction zone via quantum travel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpInstruction {
    /// The QT destination to select in your ship's navigation.
    pub destination: String,
    /// Exit quantum travel when your distance reads this many Mm (megameters).
    /// Example: If exit_at_mm is 15000, exit QT when display shows ~15,000 Mm.
    pub exit_at_mm: u64,
    /// Distance from the QT destination to the interdiction zone in Mm.
    pub distance_from_dest_mm: u64,
    /// How far off your QT path the zone is, in km.
    /// If this is < 20km, you'll exit right in the interdiction zone.
    /// If > 20km, you'll need to fly sideways after exiting QT.
    pub lateral_offset_km: f64,
    /// Alternative destinations if the primary is inconvenient.
    pub alternatives: Vec<AltJumpInstruction>,
}

/// Alternative jump instruction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AltJumpInstruction {
    pub destination: String,
    pub exit_at_mm: u64,
}

/// A route that passes through an intersection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntersectingRoute {
    pub origin: String,
    pub destination: String,
    pub commodity: String,
    pub cargo_value: f64,
    pub ship_name: String,
    pub threat_level: u8,
    /// This route's interdiction value (cargo_value / threat).
    pub interdiction_value: f64,
}

/// Find intersection zones where multiple high-value routes converge.
///
/// This finds points in 3D space where trade route paths cross or come close,
/// making them ideal interdiction spots to catch ships from multiple routes.
pub fn find_route_intersections(
    routes: &[RouteSegment],
    proximity_threshold: f64, // How close routes need to be to count as intersecting (in Mkm)
    min_routes: usize,        // Minimum routes that must converge
) -> Vec<RouteIntersection> {
    // For each pair of routes, find if/where they intersect
    let mut intersection_zones: Vec<(Point3D, Vec<&RouteSegment>)> = Vec::new();

    for i in 0..routes.len() {
        for j in (i + 1)..routes.len() {
            let (closest_point, distance) = routes[i].closest_approach_to(&routes[j]);

            if distance <= proximity_threshold {
                // Check if this point is near an existing intersection zone
                let mut found_zone = false;
                for (zone_center, zone_routes) in &mut intersection_zones {
                    if closest_point.distance_to(zone_center) <= proximity_threshold {
                        // Add to existing zone
                        if !zone_routes.iter().any(|r| std::ptr::eq(*r, &routes[i])) {
                            zone_routes.push(&routes[i]);
                        }
                        if !zone_routes.iter().any(|r| std::ptr::eq(*r, &routes[j])) {
                            zone_routes.push(&routes[j]);
                        }
                        found_zone = true;
                        break;
                    }
                }

                if !found_zone {
                    // Create new zone
                    intersection_zones.push((closest_point, vec![&routes[i], &routes[j]]));
                }
            }
        }
    }

    // Convert to RouteIntersection, filtering by minimum route count
    let mut intersections: Vec<RouteIntersection> = intersection_zones
        .into_iter()
        .filter(|(_, zone_routes)| zone_routes.len() >= min_routes)
        .map(|(position, zone_routes)| {
            let route_count = zone_routes.len();

            // Calculate interdiction value for each route
            let mut intersecting_routes: Vec<IntersectingRoute> = zone_routes
                .iter()
                .map(|r| {
                    // Value-to-threat ratio: cargo_value / threat_level
                    // Lower threat = higher interdiction value for same cargo
                    let route_interdict_value = r.cargo_value / r.threat_level.max(1) as f64;
                    IntersectingRoute {
                        origin: r.origin_name.clone(),
                        destination: r.destination_name.clone(),
                        commodity: r.commodity.clone(),
                        cargo_value: r.cargo_value,
                        ship_name: r.ship_name.clone(),
                        threat_level: r.threat_level,
                        interdiction_value: route_interdict_value,
                    }
                })
                .collect();

            // Sort routes by interdiction value (best targets first)
            intersecting_routes.sort_by(|a, b| {
                b.interdiction_value
                    .partial_cmp(&a.interdiction_value)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            // Average interdiction value - what you expect from a random catch
            let avg_interdiction_value: f64 = intersecting_routes
                .iter()
                .map(|r| r.interdiction_value)
                .sum::<f64>()
                / route_count as f64;

            let avg_threat: f64 = intersecting_routes
                .iter()
                .map(|r| r.threat_level as f64)
                .sum::<f64>()
                / route_count as f64;

            let total_cargo_value: f64 = intersecting_routes.iter().map(|r| r.cargo_value).sum();

            // Generate a name based on what routes pass through
            let name = generate_intersection_name(&zone_routes);
            let system = infer_system_from_routes(&zone_routes);

            let suggested_tactics = if avg_threat < 2.5 {
                "Easy pickings - solo Mantis can handle most targets".to_string()
            } else if avg_threat < 4.0 {
                "Mixed targets - bring a wingman for armed haulers".to_string()
            } else if avg_threat < 5.5 {
                "Dangerous - combat wing of 3+ recommended".to_string()
            } else {
                "High risk - expect armed escorts, need full fleet".to_string()
            };

            // Calculate jump instructions
            let jump_to = calculate_jump_instruction(&position, &system);

            RouteIntersection {
                position,
                name,
                system,
                intersecting_routes,
                total_cargo_value,
                route_pair_count: route_count,
                avg_threat_level: avg_threat,
                interdiction_value: avg_interdiction_value,
                suggested_tactics,
                jump_to,
            }
        })
        .collect();

    // Sort by average interdiction value (best expected value per catch)
    intersections.sort_by(|a, b| {
        b.interdiction_value
            .partial_cmp(&a.interdiction_value)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    intersections
}

fn generate_intersection_name(routes: &[&RouteSegment]) -> String {
    // Find common elements in route names
    let mut locations: Vec<&str> = Vec::new();
    for route in routes {
        // Extract short names from terminals
        if let Some(name) = extract_short_location(&route.origin_name) {
            if !locations.contains(&name) {
                locations.push(name);
            }
        }
        if let Some(name) = extract_short_location(&route.destination_name) {
            if !locations.contains(&name) {
                locations.push(name);
            }
        }
    }

    if locations.len() >= 2 {
        format!("{}-{} Corridor", locations[0], locations[1])
    } else if !locations.is_empty() {
        format!("{} Junction", locations[0])
    } else {
        "Deep Space Intersection".to_string()
    }
}

fn extract_short_location(terminal_name: &str) -> Option<&str> {
    // Terminal names like "Commodity Shop - Admin - Port Olisar (Stanton > Crusader)"
    // Try to extract a recognizable location name
    let known_locations = [
        "Hurston", "Crusader", "ArcCorp", "microTech",
        "Lorville", "Orison", "Area18", "New Babbage",
        "Port Olisar", "Everus Harbor", "Baijini Point", "Port Tressler",
        "Grim HEX", "Levski",
        "Pyro", "Gaslight", "Endgame", "Stanton Gateway",
    ];

    for loc in known_locations {
        if terminal_name.to_lowercase().contains(&loc.to_lowercase()) {
            return Some(loc);
        }
    }
    None
}

fn infer_system_from_routes(routes: &[&RouteSegment]) -> String {
    // Look at route names to infer system
    for route in routes {
        let combined = format!("{} {}", route.origin_name, route.destination_name);
        if combined.contains("Stanton") {
            return "Stanton".to_string();
        }
        if combined.contains("Pyro") {
            return "Pyro".to_string();
        }
        if combined.contains("Nyx") {
            return "Nyx".to_string();
        }
    }
    "Unknown".to_string()
}

/// QT destinations with their positions (in Mkm from system center).
/// These are the locations you can actually jump TO in-game.
/// Planets are spread at orbital angles matching the location database.
struct QtDestination {
    name: &'static str,
    position: Point3D,
}

/// Helper to compute position from orbital radius and angle (degrees)
fn qt_orbital_pos(radius: f64, angle_deg: f64) -> Point3D {
    let angle_rad = angle_deg * std::f64::consts::PI / 180.0;
    Point3D::new(radius * angle_rad.cos(), radius * angle_rad.sin(), 0.0)
}

/// Helper to compute position relative to a parent body
fn qt_moon_pos(parent: Point3D, offset_radius: f64, angle_deg: f64) -> Point3D {
    let angle_rad = angle_deg * std::f64::consts::PI / 180.0;
    Point3D::new(
        parent.x + offset_radius * angle_rad.cos(),
        parent.y + offset_radius * angle_rad.sin(),
        0.0,
    )
}

/// Get all QT-selectable destinations.
fn get_qt_destinations() -> Vec<QtDestination> {
    // Planet positions at orbital angles
    let hurston = qt_orbital_pos(12.85, 45.0);
    let crusader = qt_orbital_pos(18.96, 135.0);
    let arccorp = qt_orbital_pos(18.59, 225.0);
    let microtech = qt_orbital_pos(22.46, 315.0);

    vec![
        // Stanton planets (main bodies)
        QtDestination { name: "Hurston", position: hurston },
        QtDestination { name: "Crusader", position: crusader },
        QtDestination { name: "ArcCorp", position: arccorp },
        QtDestination { name: "microTech", position: microtech },

        // Hurston moons
        QtDestination { name: "Arial", position: qt_moon_pos(hurston, 0.3, 30.0) },
        QtDestination { name: "Aberdeen", position: qt_moon_pos(hurston, 0.35, 120.0) },
        QtDestination { name: "Magda", position: qt_moon_pos(hurston, 0.28, 210.0) },
        QtDestination { name: "Ita", position: qt_moon_pos(hurston, 0.32, 300.0) },

        // Crusader moons
        QtDestination { name: "Cellin", position: qt_moon_pos(crusader, 0.4, 0.0) },
        QtDestination { name: "Daymar", position: qt_moon_pos(crusader, 0.45, 120.0) },
        QtDestination { name: "Yela", position: qt_moon_pos(crusader, 0.38, 240.0) },

        // ArcCorp moons
        QtDestination { name: "Lyria", position: qt_moon_pos(arccorp, 0.25, 60.0) },
        QtDestination { name: "Wala", position: qt_moon_pos(arccorp, 0.3, 180.0) },

        // microTech moons
        QtDestination { name: "Calliope", position: qt_moon_pos(microtech, 0.35, 30.0) },
        QtDestination { name: "Clio", position: qt_moon_pos(microtech, 0.4, 150.0) },
        QtDestination { name: "Euterpe", position: qt_moon_pos(microtech, 0.32, 270.0) },

        // Lagrange stations (positioned at proper angles relative to planets)
        QtDestination { name: "HUR-L1", position: qt_orbital_pos(10.0, 45.0) },
        QtDestination { name: "HUR-L2", position: qt_orbital_pos(15.0, 45.0) },
        QtDestination { name: "HUR-L3", position: qt_orbital_pos(12.85, 225.0) },
        QtDestination { name: "HUR-L4", position: qt_orbital_pos(12.85, 105.0) },
        QtDestination { name: "HUR-L5", position: qt_orbital_pos(12.85, -15.0) },
        QtDestination { name: "CRU-L1", position: qt_orbital_pos(15.0, 135.0) },
        QtDestination { name: "CRU-L4", position: qt_orbital_pos(18.96, 195.0) },
        QtDestination { name: "CRU-L5", position: qt_orbital_pos(18.96, 75.0) },
        QtDestination { name: "ARC-L1", position: qt_orbital_pos(15.0, 225.0) },
        QtDestination { name: "MIC-L1", position: qt_orbital_pos(18.0, 315.0) },
        QtDestination { name: "MIC-L2", position: qt_orbital_pos(27.0, 315.0) },

        // Orbital stations
        QtDestination { name: "Everus Harbor", position: qt_moon_pos(hurston, 0.05, 90.0) },
        QtDestination { name: "Port Olisar", position: qt_moon_pos(crusader, 0.05, 90.0) },
        QtDestination { name: "Baijini Point", position: qt_moon_pos(arccorp, 0.05, 90.0) },
        QtDestination { name: "Port Tressler", position: qt_moon_pos(microtech, 0.05, 90.0) },
        QtDestination { name: "Grim HEX", position: qt_moon_pos(qt_moon_pos(crusader, 0.38, 240.0), 0.1, 45.0) },

        // Pyro system (different orbital angles)
        QtDestination { name: "Stanton Gateway (Pyro)", position: qt_orbital_pos(100.0, 0.0) },
        QtDestination { name: "Pyro I", position: qt_orbital_pos(5.0, 0.0) },
        QtDestination { name: "Pyro II", position: qt_orbital_pos(10.0, 72.0) },
        QtDestination { name: "Pyro III", position: qt_orbital_pos(18.0, 144.0) },
        QtDestination { name: "Pyro IV", position: qt_orbital_pos(28.0, 216.0) },
        QtDestination { name: "Pyro V", position: qt_orbital_pos(40.0, 270.0) },
        QtDestination { name: "Pyro VI", position: qt_orbital_pos(55.0, 324.0) },
        QtDestination { name: "Ruin Station", position: qt_moon_pos(qt_orbital_pos(5.0, 0.0), 0.5, 90.0) },
        QtDestination { name: "Checkmate Station", position: qt_moon_pos(qt_orbital_pos(28.0, 216.0), 0.5, 90.0) },
    ]
}

/// Calculate jump instructions to reach an interdiction zone.
///
/// The key insight: You need to QT to a destination where your travel path
/// passes THROUGH the interdiction zone. Then exit QT when you're at the zone.
///
/// We score destinations by:
/// 1. How close your QT path passes to the zone (must be < 20km for Mantis range)
/// 2. Whether the zone is between you and the destination (not behind you)
fn calculate_jump_instruction(zone_position: &Point3D, _system: &str) -> JumpInstruction {
    let destinations = get_qt_destinations();

    // For each destination, calculate:
    // 1. The closest point on the line from origin (0,0,0) to destination that passes near zone
    // 2. How far from that destination the zone is (exit distance)
    // 3. How far off the direct path the zone is (lateral error - must be < 20km)

    // We'll assume the player starts from various positions, so we look for destinations
    // where the zone lies roughly ON the path TO that destination from typical starting points

    let mut scored: Vec<(f64, &QtDestination, f64, f64)> = destinations
        .iter()
        .filter_map(|dest| {
            // Distance from zone to destination
            let zone_to_dest = zone_position.distance_to(&dest.position);

            // For a good interdiction point, the zone should be:
            // 1. Between some origin and this destination
            // 2. Not too far off the main travel corridor

            // Calculate perpendicular distance from zone to the line from origin (0,0,0) to dest
            // This tells us how far off the main travel path the zone is
            let lateral_offset = perpendicular_distance_to_line(
                zone_position,
                &Point3D::new(0.0, 0.0, 0.0), // Stanton center as rough origin
                &dest.position,
            );

            // Convert to km for comparison with Mantis range
            let lateral_offset_km = lateral_offset * 1_000_000.0; // Mkm to km

            // Score: prefer destinations where:
            // - Zone is close to the travel path (low lateral offset)
            // - Zone is at a reasonable distance from destination (not too close, not too far)

            // If lateral offset > 1000km, this destination's path doesn't pass near the zone
            if lateral_offset_km > 1000.0 {
                return None;
            }

            // Score based on how well-aligned the path is (lower lateral = better)
            // and reasonable exit distance
            let alignment_score = 1000.0 - lateral_offset_km.min(1000.0);

            Some((alignment_score, dest, zone_to_dest, lateral_offset_km))
        })
        .collect();

    // Sort by alignment score (best first)
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // If no aligned destinations, fall back to nearest
    if scored.is_empty() {
        let mut distances: Vec<(&QtDestination, f64)> = destinations
            .iter()
            .map(|dest| (dest, zone_position.distance_to(&dest.position)))
            .collect();
        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let primary = &distances[0];
        let primary_dist_mm = (primary.1 * 1_000_000.0) as u64;

        return JumpInstruction {
            destination: primary.0.name.to_string(),
            exit_at_mm: primary_dist_mm,
            distance_from_dest_mm: primary_dist_mm,
            lateral_offset_km: 9999.0, // Unknown - not on path
            alternatives: vec![],
        };
    }

    let (_, primary_dest, primary_dist, lateral_km) = &scored[0];
    let primary_dist_mm = (*primary_dist * 1_000_000.0) as u64;

    // Build alternatives
    let alternatives: Vec<AltJumpInstruction> = scored
        .iter()
        .skip(1)
        .take(2)
        .map(|(_, dest, dist, _)| {
            AltJumpInstruction {
                destination: dest.name.to_string(),
                exit_at_mm: (*dist * 1_000_000.0) as u64,
            }
        })
        .collect();

    JumpInstruction {
        destination: primary_dest.name.to_string(),
        exit_at_mm: primary_dist_mm,
        distance_from_dest_mm: primary_dist_mm,
        lateral_offset_km: *lateral_km,
        alternatives,
    }
}

/// Calculate perpendicular distance from a point to a line defined by two points.
fn perpendicular_distance_to_line(point: &Point3D, line_start: &Point3D, line_end: &Point3D) -> f64 {
    // Vector from line_start to line_end
    let line_vec = Point3D::new(
        line_end.x - line_start.x,
        line_end.y - line_start.y,
        line_end.z - line_start.z,
    );

    // Vector from line_start to point
    let point_vec = Point3D::new(
        point.x - line_start.x,
        point.y - line_start.y,
        point.z - line_start.z,
    );

    // Length of line
    let line_len = (line_vec.x.powi(2) + line_vec.y.powi(2) + line_vec.z.powi(2)).sqrt();
    if line_len < 1e-10 {
        return point_vec.x.powi(2) + point_vec.y.powi(2) + point_vec.z.powi(2);
    }

    // Project point onto line
    let t = (dot(&point_vec, &line_vec) / (line_len * line_len)).clamp(0.0, 1.0);

    // Closest point on line
    let closest = Point3D::new(
        line_start.x + t * line_vec.x,
        line_start.y + t * line_vec.y,
        line_start.z + t * line_vec.z,
    );

    // Distance from point to closest point on line
    point.distance_to(&closest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 0.0);
        assert!((p1.distance_to(&p2) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_nearest() {
        let index = SpatialIndex::new();

        // Would need actual Chokepoint data for full test
        // This is a placeholder structure test
        assert!(index.is_empty());
    }

    #[test]
    fn test_route_intersection() {
        // Two crossing routes
        let route1 = RouteSegment {
            origin: Point3D::new(0.0, 0.0, 0.0),
            destination: Point3D::new(10.0, 10.0, 0.0),
            origin_name: "A".to_string(),
            destination_name: "B".to_string(),
            cargo_value: 100000.0,
            commodity: "Gold".to_string(),
            ship_name: "Caterpillar".to_string(),
            threat_level: 4,
        };

        let route2 = RouteSegment {
            origin: Point3D::new(0.0, 10.0, 0.0),
            destination: Point3D::new(10.0, 0.0, 0.0),
            origin_name: "C".to_string(),
            destination_name: "D".to_string(),
            cargo_value: 150000.0,
            commodity: "Silver".to_string(),
            ship_name: "C2 Hercules".to_string(),
            threat_level: 5,
        };

        let (point, distance) = route1.closest_approach_to(&route2);
        // Routes should cross near (5, 5, 0)
        assert!(point.x > 4.0 && point.x < 6.0);
        assert!(point.y > 4.0 && point.y < 6.0);
        assert!(distance < 1.0);
    }
}
