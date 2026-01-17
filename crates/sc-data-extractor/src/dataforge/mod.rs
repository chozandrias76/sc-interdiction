//! `DataForge` data extraction from scunpacked-data repository
//!
//! This module provides access to Star Citizen game data extracted from the p4k archive.
//! Rather than extracting directly from the 150GB p4k file, we use the pre-extracted
//! JSON files from the scunpacked-data repository.
//!
//! # Data Sources
//!
//! The scunpacked-data repository provides:
//! - `items.json` - All game items (47MB, ~18K items)
//! - `ships.json` - Ship definitions (~281 ships)
//! - `labels.json` - Localization strings (~50K entries)
//! - `manufacturers.json` - Manufacturer definitions
//! - `fps-items.json` - FPS weapons and equipment
//! - `ship-items.json` - Ship components
//!
//! # Caching Strategy
//!
//! Since the data is already pre-extracted JSON, we cache parsed data in memory
//! and can optionally serialize parsed structures for faster subsequent loads.

pub mod types;

use crate::{Error, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub use types::*;

/// `DataForge` data accessor for scunpacked-data repository
///
/// Provides typed access to game data with optional caching.
#[derive(Debug)]
pub struct DataForgeExtractor {
    /// Path to scunpacked-data directory (e.g., `extracted/scunpacked-data/`)
    data_dir: PathBuf,
    /// Optional cache directory for parsed data
    cache_dir: Option<PathBuf>,
    /// Cached items (loaded on demand)
    items: Option<Vec<GameItem>>,
    /// Cached ships (loaded on demand)
    ships: Option<Vec<Ship>>,
    /// Cached labels (loaded on demand)
    labels: Option<HashMap<String, String>>,
}

impl DataForgeExtractor {
    /// Create a new extractor pointing to scunpacked-data directory
    #[must_use]
    pub fn new(data_dir: impl Into<PathBuf>) -> Self {
        Self {
            data_dir: data_dir.into(),
            cache_dir: None,
            items: None,
            ships: None,
            labels: None,
        }
    }

    /// Set cache directory for parsed data
    #[must_use]
    pub fn with_cache_dir(mut self, cache_dir: impl Into<PathBuf>) -> Self {
        self.cache_dir = Some(cache_dir.into());
        self
    }

    /// Get the data directory path
    #[must_use]
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Check if data directory exists and contains expected files
    ///
    /// # Errors
    ///
    /// Returns an error if file metadata cannot be read.
    pub fn validate(&self) -> Result<DataForgeStatus> {
        if !self.data_dir.exists() {
            return Ok(DataForgeStatus::NotFound {
                path: self.data_dir.clone(),
            });
        }

        let items_path = self.data_dir.join("items.json");
        let ships_path = self.data_dir.join("ships.json");
        let labels_path = self.data_dir.join("labels.json");

        let mut missing = Vec::new();
        if !items_path.exists() {
            missing.push("items.json".to_string());
        }
        if !ships_path.exists() {
            missing.push("ships.json".to_string());
        }
        if !labels_path.exists() {
            missing.push("labels.json".to_string());
        }

        if !missing.is_empty() {
            return Ok(DataForgeStatus::Incomplete {
                path: self.data_dir.clone(),
                missing,
            });
        }

        // Get metadata for cache validation
        let meta = fs::metadata(&items_path)?;
        Ok(DataForgeStatus::Ready {
            path: self.data_dir.clone(),
            items_size: meta.len(),
            modified: meta.modified().ok(),
        })
    }

    /// Load all game items from `items.json`
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_items(&mut self) -> Result<&[GameItem]> {
        if self.items.is_none() {
            let path = self.data_dir.join("items.json");
            let content = fs::read_to_string(&path).map_err(|e| Error::InvalidData {
                path: path.clone(),
                reason: format!("Failed to read items.json: {e}"),
            })?;
            let items: Vec<GameItem> = serde_json::from_str(&content)?;
            self.items = Some(items);
        }
        // SAFETY: We just set items to Some above, so this is guaranteed to succeed
        Ok(self.items.as_deref().unwrap_or(&[]))
    }

    /// Load all ships from `ships.json`
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_ships(&mut self) -> Result<&[Ship]> {
        if self.ships.is_none() {
            let path = self.data_dir.join("ships.json");
            let content = fs::read_to_string(&path).map_err(|e| Error::InvalidData {
                path: path.clone(),
                reason: format!("Failed to read ships.json: {e}"),
            })?;
            let ships: Vec<Ship> = serde_json::from_str(&content)?;
            self.ships = Some(ships);
        }
        // SAFETY: We just set ships to Some above, so this is guaranteed to succeed
        Ok(self.ships.as_deref().unwrap_or(&[]))
    }

    /// Load localization labels from `labels.json`
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    #[allow(clippy::missing_panics_doc)]
    pub fn load_labels(&mut self) -> Result<&HashMap<String, String>> {
        if self.labels.is_none() {
            let path = self.data_dir.join("labels.json");
            let content = fs::read_to_string(&path).map_err(|e| Error::InvalidData {
                path: path.clone(),
                reason: format!("Failed to read labels.json: {e}"),
            })?;
            let labels: HashMap<String, String> = serde_json::from_str(&content)?;
            self.labels = Some(labels);
        }
        // SAFETY: We just set labels to Some above, or return early above, so this is guaranteed
        // Using unwrap here because unwrap_or would require a static reference
        #[allow(clippy::unwrap_used)]
        Ok(self.labels.as_ref().unwrap())
    }

    /// Find items matching a predicate
    ///
    /// # Errors
    ///
    /// Returns an error if items cannot be loaded.
    pub fn find_items(&mut self, predicate: impl Fn(&GameItem) -> bool) -> Result<Vec<&GameItem>> {
        let items = self.load_items()?;
        Ok(items.iter().filter(|i| predicate(i)).collect())
    }

    /// Find ships matching a predicate
    ///
    /// # Errors
    ///
    /// Returns an error if ships cannot be loaded.
    pub fn find_ships(&mut self, predicate: impl Fn(&Ship) -> bool) -> Result<Vec<&Ship>> {
        let ships = self.load_ships()?;
        Ok(ships.iter().filter(|s| predicate(s)).collect())
    }

    /// Find all Wikelo-related items (favors, weapons, clothing, etc.)
    ///
    /// # Errors
    ///
    /// Returns an error if items cannot be loaded.
    pub fn wikelo_items(&mut self) -> Result<Vec<&GameItem>> {
        self.find_items(|item| {
            item.class_name.to_lowercase().contains("wikelo")
                || item
                    .name
                    .as_ref()
                    .is_some_and(|n| n.to_lowercase().contains("wikelo"))
                || item
                    .name
                    .as_ref()
                    .is_some_and(|n| n.contains("Quite Useful"))
        })
    }

    /// Find all Wikelo collector ships
    ///
    /// # Errors
    ///
    /// Returns an error if ships cannot be loaded.
    pub fn wikelo_ships(&mut self) -> Result<Vec<&Ship>> {
        self.find_ships(|ship| {
            ship.class_name.contains("Collector")
                || ship.name.as_ref().is_some_and(|n| n.contains("Wikelo"))
        })
    }

    /// Look up a localization key
    ///
    /// # Errors
    ///
    /// Returns an error if labels cannot be loaded.
    pub fn localize(&mut self, key: &str) -> Result<Option<String>> {
        let labels = self.load_labels()?;
        Ok(labels.get(key).cloned())
    }

    /// Clear all cached data to free memory
    pub fn clear_cache(&mut self) {
        self.items = None;
        self.ships = None;
        self.labels = None;
    }
}

/// Status of `DataForge` data availability
#[derive(Debug)]
pub enum DataForgeStatus {
    /// Data directory not found
    NotFound { path: PathBuf },
    /// Data directory exists but missing required files
    Incomplete { path: PathBuf, missing: Vec<String> },
    /// Data is ready to use
    Ready {
        path: PathBuf,
        items_size: u64,
        modified: Option<std::time::SystemTime>,
    },
}

impl DataForgeStatus {
    /// Returns true if data is ready to use
    #[must_use]
    pub fn is_ready(&self) -> bool {
        matches!(self, DataForgeStatus::Ready { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor_creation() {
        let extractor = DataForgeExtractor::new("test/path");
        assert_eq!(extractor.data_dir(), Path::new("test/path"));
    }

    #[test]
    fn test_status_not_found() {
        let extractor = DataForgeExtractor::new("/nonexistent/path");
        let status = extractor.validate().unwrap();
        assert!(matches!(status, DataForgeStatus::NotFound { .. }));
    }
}
