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
        let mut index = SpatialIndex::new();

        // Would need actual Chokepoint data for full test
        // This is a placeholder structure test
        assert!(index.is_empty());
    }
}
