//! Localization string lookup for Star Citizen game data
//!
//! This module provides access to localization strings used in the game.
//! It supports two sources:
//!
//! - `labels.json` from scunpacked-data repository (pre-extracted, recommended)
//! - `global.ini` from game data (direct extraction from p4k)
//!
//! # Usage
//!
//! ```rust,ignore
//! use sc_data_extractor::localization::LocalizationStore;
//!
//! // Load from labels.json (recommended)
//! let store = LocalizationStore::from_labels_json("extracted/scunpacked-data/labels.json")?;
//!
//! // Or load from global.ini
//! let store = LocalizationStore::from_global_ini("Data/Localization/english/global.ini")?;
//!
//! // Look up a key
//! if let Some(value) = store.get("TheCollector_Coin_Name") {
//!     println!("Wikelo Favor: {}", value);
//! }
//! ```

use crate::{Error, Result};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// A store of localization strings (key → translated value)
#[derive(Debug, Clone)]
pub struct LocalizationStore {
    entries: HashMap<String, String>,
}

impl LocalizationStore {
    /// Create an empty localization store
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Create from a pre-loaded `HashMap`
    #[must_use]
    pub fn from_map(entries: HashMap<String, String>) -> Self {
        Self { entries }
    }

    /// Load from `labels.json` (scunpacked-data format)
    ///
    /// This is the recommended method when using scunpacked-data.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn from_labels_json(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|e| Error::InvalidData {
            path: path.to_path_buf(),
            reason: format!("Failed to read labels.json: {e}"),
        })?;
        let entries: HashMap<String, String> = serde_json::from_str(&content)?;
        Ok(Self { entries })
    }

    /// Load from `global.ini` (game data format)
    ///
    /// Parses the INI-style format used in Star Citizen's localization files.
    /// Handles UTF-8 BOM and comment lines.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn from_global_ini(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file = fs::File::open(path).map_err(|e| Error::InvalidData {
            path: path.to_path_buf(),
            reason: format!("Failed to open global.ini: {e}"),
        })?;
        let reader = BufReader::new(file);
        let mut entries = HashMap::new();

        for (line_num, line_result) in reader.lines().enumerate() {
            let line = line_result.map_err(|e| Error::InvalidData {
                path: path.to_path_buf(),
                reason: format!("Failed to read line {}: {e}", line_num + 1),
            })?;

            // Handle UTF-8 BOM on first line
            let line = if line_num == 0 {
                line.strip_prefix('\u{feff}').unwrap_or(&line).to_string()
            } else {
                line
            };

            // Skip empty lines and comments
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('#') {
                continue;
            }

            // Parse key=value format
            if let Some((key, value)) = line.split_once('=') {
                entries.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        Ok(Self { entries })
    }

    /// Look up a localization key
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Look up a key, returning the key itself if not found
    #[must_use]
    pub fn get_or_key<'a>(&'a self, key: &'a str) -> &'a str {
        self.get(key).unwrap_or(key)
    }

    /// Check if a key exists
    #[must_use]
    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Get the number of entries
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterate over all entries
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    /// Find entries matching a key pattern (case-insensitive contains)
    #[must_use]
    pub fn find_keys(&self, pattern: &str) -> Vec<(&str, &str)> {
        let pattern_lower = pattern.to_lowercase();
        self.entries
            .iter()
            .filter(|(k, _)| k.to_lowercase().contains(&pattern_lower))
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }

    /// Find entries where value matches a pattern (case-insensitive contains)
    #[must_use]
    pub fn find_values(&self, pattern: &str) -> Vec<(&str, &str)> {
        let pattern_lower = pattern.to_lowercase();
        self.entries
            .iter()
            .filter(|(_, v)| v.to_lowercase().contains(&pattern_lower))
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }
}

impl Default for LocalizationStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_map() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());

        let store = LocalizationStore::from_map(map);
        assert_eq!(store.get("key1"), Some("value1"));
        assert_eq!(store.get("key2"), Some("value2"));
        assert_eq!(store.get("key3"), None);
    }

    #[test]
    fn test_get_or_key() {
        let mut map = HashMap::new();
        map.insert("found".to_string(), "result".to_string());

        let store = LocalizationStore::from_map(map);
        assert_eq!(store.get_or_key("found"), "result");
        assert_eq!(store.get_or_key("not_found"), "not_found");
    }

    #[test]
    fn test_find_keys() {
        let mut map = HashMap::new();
        map.insert("Wikelo_Station_01".to_string(), "Dasi".to_string());
        map.insert("Wikelo_Station_02".to_string(), "Selo".to_string());
        map.insert("Other_Key".to_string(), "Value".to_string());

        let store = LocalizationStore::from_map(map);
        let results = store.find_keys("wikelo");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_empty_store() {
        let store = LocalizationStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }
}
