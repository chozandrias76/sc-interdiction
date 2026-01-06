//! Starmap XML parser

use crate::error::{Error, Result};
use crate::models::starmap::{QuantumTravelData, StarmapLocation};
use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Parser for starmap XML files
pub struct StarmapParser {
    starmap_dir: PathBuf,
}

impl StarmapParser {
    /// Creates a new starmap parser
    ///
    /// # Arguments
    /// * `sclogistics_path` - Path to the `SCLogistics` repository root
    pub fn new(sclogistics_path: impl AsRef<Path>) -> Self {
        let starmap_dir = sclogistics_path.as_ref().join("starmap");
        Self { starmap_dir }
    }

    /// Parses all starmap XML files.
    ///
    /// # Errors
    ///
    /// Returns an error if reading the starmap directory fails.
    /// Individual file parse errors are logged and skipped.
    pub fn parse_all(&self) -> Result<Vec<StarmapLocation>> {
        let mut locations = Vec::new();

        for entry in WalkDir::new(&self.starmap_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "xml")
                    .unwrap_or(false)
            })
        {
            match self.parse_file(entry.path()) {
                Ok(mut locs) => locations.append(&mut locs),
                Err(_e) => {
                    // Skip files that fail to parse - caller can check location count
                }
            }
        }

        Ok(locations)
    }

    /// Parses a single starmap XML file
    fn parse_file(&self, path: &Path) -> Result<Vec<StarmapLocation>> {
        let content = fs::read_to_string(path)?;
        let raw: RawStarmapObject = from_str(&content).map_err(|e| Error::InvalidData {
            path: path.to_path_buf(),
            reason: format!("XML parse error: {e}"),
        })?;

        Ok(vec![StarmapLocation {
            id: raw.ref_attr.unwrap_or_default(),
            name: raw.name.unwrap_or_default(),
            parent: raw.parent,
            location_type: raw.location_type.unwrap_or_default(),
            nav_icon: raw.nav_icon,
            affiliation: raw.affiliation,
            description: raw.description,
            is_scannable: raw.is_scannable.unwrap_or(0) != 0,
            hide_in_starmap: raw.hide_in_starmap.unwrap_or(0) != 0,
            quantum_travel: raw.quantum_travel_data.and_then(|qtd| {
                qtd.params.map(|p| QuantumTravelData {
                    obstruction_radius: p.obstruction_radius,
                    arrival_radius: p.arrival_radius,
                    arrival_point_detection_offset: p.arrival_point_detection_offset,
                    adoption_radius: p.adoption_radius,
                })
            }),
            amenities: Vec::new(),
        }])
    }
}

/// Raw deserialization structures matching XML schema
#[derive(Debug, Deserialize)]
struct RawStarmapObject {
    #[serde(rename = "@__ref")]
    ref_attr: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@parent")]
    parent: Option<String>,
    #[serde(rename = "@type")]
    location_type: Option<String>,
    #[serde(rename = "@navIcon")]
    nav_icon: Option<String>,
    #[serde(rename = "@affiliation")]
    affiliation: Option<String>,
    #[serde(rename = "@description")]
    description: Option<String>,
    #[serde(rename = "@isScannable")]
    is_scannable: Option<i32>,
    #[serde(rename = "@hideInStarmap")]
    hide_in_starmap: Option<i32>,
    #[serde(rename = "quantumTravelData")]
    quantum_travel_data: Option<RawQuantumTravelData>,
}

#[derive(Debug, Deserialize)]
struct RawQuantumTravelData {
    #[serde(rename = "StarMapQuantumTravelDataParams")]
    params: Option<RawQuantumTravelParams>,
}

#[derive(Debug, Deserialize)]
struct RawQuantumTravelParams {
    #[serde(rename = "@obstructionRadius")]
    obstruction_radius: f64,
    #[serde(rename = "@arrivalRadius")]
    arrival_radius: f64,
    #[serde(rename = "@arrivalPointDetectionOffset")]
    arrival_point_detection_offset: f64,
    #[serde(rename = "@adoptionRadius")]
    adoption_radius: f64,
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sample_xml() {
        let xml = r#"<StarMapObject.Stanton1_Zephyr 
            name="@Stanton1_DerelictSettlement_Zephyr" 
            affiliation="6306a9a1-3b71-45c2-b6e8-f6d589e1868d" 
            description="@Stanton1_DerelictSettlement_Zephyr_desc" 
            type="e207a1ec-1395-4c1c-8e51-b38c4420784c" 
            navIcon="Outpost" 
            parent="551af60b-7727-4936-acc7-763d25d7a1de" 
            isScannable="0" 
            hideInStarmap="0" 
            __ref="2f26aee5-8a5f-48cd-8226-3fbdea6df3b6">
          <quantumTravelData>
            <StarMapQuantumTravelDataParams 
                obstructionRadius="0" 
                arrivalRadius="20000" 
                arrivalPointDetectionOffset="10000" 
                adoptionRadius="0" />
          </quantumTravelData>
        </StarMapObject.Stanton1_Zephyr>"#;

        let raw: RawStarmapObject = from_str(xml).expect("Failed to parse XML");
        assert_eq!(
            raw.ref_attr.as_deref(),
            Some("2f26aee5-8a5f-48cd-8226-3fbdea6df3b6")
        );
        assert!(raw.quantum_travel_data.is_some());
    }
}
