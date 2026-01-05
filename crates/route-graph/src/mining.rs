//! Mining site data and utilities for route planning.
//!
//! Defines mining locations, resource types, and helper functions
//! for calculating mining route profitability.

use crate::Point3D;
use serde::{Deserialize, Serialize};

/// Types of mineable resources in Star Citizen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    /// Quantainium - highly valuable, time-sensitive (unstable)
    Quantainium,
    /// Bexalite - valuable, stable
    Bexalite,
    /// Taranite - common, moderate value
    Taranite,
    /// Gold - valuable, stable
    Gold,
    /// Copper - common, low value
    Copper,
    /// Diamond - valuable gemstone
    Diamond,
    /// Agricium - medical material
    Agricium,
    /// Laranite - valuable industrial material
    Laranite,
    /// Borase - common industrial material
    Borase,
    /// Hephaestanite - rare, valuable
    Hephaestanite,
}

impl ResourceType {
    /// Get the typical market value range for this resource (aUEC per unit)
    pub fn typical_value_range(&self) -> (f64, f64) {
        match self {
            Self::Quantainium => (88.0, 110.0),
            Self::Bexalite => (40.0, 50.0),
            Self::Taranite | Self::Agricium => (25.0, 35.0),
            Self::Gold => (20.0, 30.0),
            Self::Copper => (5.0, 10.0),
            Self::Diamond => (6.0, 9.0),
            Self::Laranite => (28.0, 40.0),
            Self::Borase => (18.0, 26.0),
            Self::Hephaestanite => (45.0, 60.0),
        }
    }

    /// Returns true if this resource is unstable/time-sensitive
    pub fn is_unstable(&self) -> bool {
        matches!(self, Self::Quantainium)
    }
}

/// A mining site with known resources.
#[derive(Debug, Clone, Serialize)]
pub struct MiningSite {
    /// Name of the mining location
    pub name: &'static str,
    /// Star system
    pub system: &'static str,
    /// Approximate position in 3D space (Mkm)
    pub position: Point3D,
    /// Primary resource types found here
    pub resource_types: &'static [ResourceType],
    /// Average yield quality (0.0-1.0, where 1.0 is highest)
    pub avg_yield_quality: f64,
    /// Whether this is a surface mining site (vs asteroid belt)
    pub is_surface: bool,
}

/// Static database of known mining sites in Stanton system.
pub static MINING_SITES: &[MiningSite] = &[
    MiningSite {
        name: "Aaron Halo",
        system: "Stanton",
        position: Point3D::new(0.0, 30.0, 0.0), // Outer system belt
        resource_types: &[
            ResourceType::Quantainium,
            ResourceType::Bexalite,
            ResourceType::Taranite,
            ResourceType::Gold,
        ],
        avg_yield_quality: 0.85,
        is_surface: false,
    },
    MiningSite {
        name: "Yela Asteroid Belt",
        system: "Stanton",
        position: Point3D::new(18.5, 19.4, 0.0), // Near Crusader moon Yela
        resource_types: &[
            ResourceType::Quantainium,
            ResourceType::Bexalite,
            ResourceType::Agricium,
        ],
        avg_yield_quality: 0.75,
        is_surface: false,
    },
    MiningSite {
        name: "ArcCorp Mining Area 045",
        system: "Stanton",
        position: Point3D::new(-15.0, -12.0, 0.0), // Near Lyria
        resource_types: &[
            ResourceType::Quantainium,
            ResourceType::Agricium,
            ResourceType::Hephaestanite,
        ],
        avg_yield_quality: 0.80,
        is_surface: false,
    },
    MiningSite {
        name: "ArcCorp Mining Area 141",
        system: "Stanton",
        position: Point3D::new(-18.5, -1.0, 0.0), // Near Wala
        resource_types: &[ResourceType::Bexalite, ResourceType::Taranite],
        avg_yield_quality: 0.65,
        is_surface: false,
    },
    MiningSite {
        name: "Aberdeen Surface",
        system: "Stanton",
        position: Point3D::new(-9.5, -8.5, 0.0), // Hurston moon Aberdeen
        resource_types: &[
            ResourceType::Quantainium,
            ResourceType::Bexalite,
            ResourceType::Gold,
        ],
        avg_yield_quality: 0.70,
        is_surface: true,
    },
    MiningSite {
        name: "Daymar Surface",
        system: "Stanton",
        position: Point3D::new(-13.0, 13.5, 0.0), // Crusader moon Daymar
        resource_types: &[
            ResourceType::Agricium,
            ResourceType::Hephaestanite,
            ResourceType::Bexalite,
        ],
        avg_yield_quality: 0.65,
        is_surface: true,
    },
    MiningSite {
        name: "Lyria Surface",
        system: "Stanton",
        position: Point3D::new(-16.5, -13.0, 0.0), // ArcCorp moon Lyria
        resource_types: &[
            ResourceType::Agricium,
            ResourceType::Laranite,
            ResourceType::Borase,
        ],
        avg_yield_quality: 0.60,
        is_surface: true,
    },
];

/// Find mining sites that contain a specific resource type.
pub fn sites_with_resource(resource: ResourceType) -> Vec<&'static MiningSite> {
    MINING_SITES
        .iter()
        .filter(|site| site.resource_types.contains(&resource))
        .collect()
}

/// Find the nearest mining site to a given position.
pub fn nearest_mining_site(position: &Point3D) -> Option<&'static MiningSite> {
    MINING_SITES.iter().min_by(|a, b| {
        let dist_a = a.position.distance_to(position);
        let dist_b = b.position.distance_to(position);
        dist_a
            .partial_cmp(&dist_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mining_sites_loaded() {
        assert!(!MINING_SITES.is_empty());
        assert!(MINING_SITES.len() >= 5);
    }

    #[test]
    fn test_quantainium_sites() {
        let sites = sites_with_resource(ResourceType::Quantainium);
        assert!(!sites.is_empty());
        assert!(sites.iter().any(|s| s.name == "Aaron Halo"));
    }

    #[test]
    fn test_resource_values() {
        let (min, max) = ResourceType::Quantainium.typical_value_range();
        assert!(max > min);
        assert!(min > 0.0);
    }

    #[test]
    fn test_nearest_site() {
        let pos = Point3D::new(0.0, 30.0, 0.0);
        let nearest = nearest_mining_site(&pos);
        assert!(nearest.is_some());
    }
}
