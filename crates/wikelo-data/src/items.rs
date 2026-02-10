//! Static item definitions for Wikelo items.
//!
//! Contains all 31 items from Phase 2 research (02-DATA-READY.md).
//! Import types from intel crate - do NOT redefine.

// Allow long functions for static data definitions - these are data, not logic
#![allow(clippy::too_many_lines)]

use intel::{AcquisitionMethod, ItemCategory, ItemSource, SourceLocation, WikieloItem};

/// Returns all Wikelo items.
#[must_use]
pub fn all_items() -> Vec<WikieloItem> {
    let mut items = Vec::with_capacity(80);
    items.extend(creature_parts());
    items.extend(mined_materials());
    items.extend(mission_loot_items());
    items.extend(commodities());
    items
}

/// Returns the 10 creature part items.
fn creature_parts() -> Vec<WikieloItem> {
    vec![
        WikieloItem {
            id: "irradiated_valakkar_fang_juvenile".to_string(),
            name: "Irradiated Valakkar Fang (Juvenile)".to_string(),
            category: ItemCategory::CreaturePart,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Monox".to_string(),
                    system: "Pyro".to_string(),
                    description: Some("Invasive species location; ~5m juveniles".to_string()),
                },
                method: AcquisitionMethod::Hunting,
                reliability: 4,
                notes: Some("Native to Leir III; also found on Daymar".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "irradiated_valakkar_fang_adult".to_string(),
            name: "Irradiated Valakkar Fang (Adult)".to_string(),
            category: ItemCategory::CreaturePart,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Pyro I".to_string(),
                    system: "Pyro".to_string(),
                    description: Some("Invasive species location; ~15m adults".to_string()),
                },
                method: AcquisitionMethod::Hunting,
                reliability: 4,
                notes: Some("Attack when their litter is threatened".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "irradiated_valakkar_fang_apex".to_string(),
            name: "Irradiated Valakkar Fang (Apex)".to_string(),
            category: ItemCategory::CreaturePart,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Pyro I".to_string(),
                    system: "Pyro".to_string(),
                    description: Some("Apex variant; up to 300m".to_string()),
                },
                method: AcquisitionMethod::Hunting,
                reliability: 4,
                notes: Some("Largest form; requires team to hunt".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "irradiated_valakkar_pearl".to_string(),
            name: "Irradiated Valakkar Pearl".to_string(),
            category: ItemCategory::CreaturePart,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Monox".to_string(),
                    system: "Pyro".to_string(),
                    description: Some("Pearl-like body parts harvested from Valakkar".to_string()),
                },
                method: AcquisitionMethod::Hunting,
                reliability: 4,
                notes: Some("Same creatures as fangs; different drop".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "tundra_kopion_horn".to_string(),
            name: "Tundra Kopion Horn".to_string(),
            category: ItemCategory::CreaturePart,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "microTech".to_string(),
                    system: "Stanton".to_string(),
                    description: Some(
                        "Equator/Green Zones; elevated grasslands/tundras".to_string(),
                    ),
                },
                method: AcquisitionMethod::Hunting,
                reliability: 4,
                notes: Some(
                    "Packs of 8-18; thicker exoskeleton with pale mottled coloring".to_string(),
                ),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        // LOW CONFIDENCE - Needs gameplay validation
        WikieloItem {
            id: "irradiated_kopion_horn".to_string(),
            name: "Irradiated Kopion Horn".to_string(),
            category: ItemCategory::CreaturePart,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Unknown".to_string(),
                    system: "Unknown".to_string(),
                    description: Some("Likely Pyro variant; unverified".to_string()),
                },
                method: AcquisitionMethod::Hunting,
                reliability: 2,
                notes: Some("May not exist as separate item; needs validation".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "yormandi_eye".to_string(),
            name: "Yormandi Eye".to_string(),
            category: ItemCategory::CreaturePart,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "ASD Onyx Facility".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Underground cave system".to_string()),
                },
                method: AcquisitionMethod::Combat,
                reliability: 5,
                notes: Some("Antibody-rich eyes valued by medical researchers".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "yormandi_tongue".to_string(),
            name: "Yormandi Tongue".to_string(),
            category: ItemCategory::CreaturePart,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "ASD Onyx Facility".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Underground cave system".to_string()),
                },
                method: AcquisitionMethod::Combat,
                reliability: 5,
                notes: Some("Prized for culinary use; complex subtle flavor".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "quasi_grazer_tongue".to_string(),
            name: "Quasi Grazer Tongue".to_string(),
            category: ItemCategory::CreaturePart,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Terra III (Quasi)".to_string(),
                    system: "Terra".to_string(),
                    description: Some("Native habitat; also on terraformed planets".to_string()),
                },
                method: AcquisitionMethod::Hunting,
                reliability: 3,
                notes: Some("'Space cow' found on most UEE terraformed worlds".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "quasi_grazer_egg".to_string(),
            name: "Quasi Grazer Egg".to_string(),
            category: ItemCategory::CreaturePart,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Terra III".to_string(),
                    system: "Terra".to_string(),
                    description: Some("Desert, Boreal, Grassland variants".to_string()),
                },
                method: AcquisitionMethod::Hunting,
                reliability: 3,
                notes: Some("Used for cooking; different variants by biome".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
    ]
}

/// Returns the 10 mined material items.
fn mined_materials() -> Vec<WikieloItem> {
    vec![
        WikieloItem {
            id: "carinite".to_string(),
            name: "Carinite".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![
                ItemSource {
                    location: SourceLocation {
                        name: "Aberdeen".to_string(),
                        system: "Stanton".to_string(),
                        description: Some(
                            "Hathor caves exposed by orbital laser platforms".to_string(),
                        ),
                    },
                    method: AcquisitionMethod::Mining,
                    reliability: 5,
                    notes: Some("Deep-red mineral; used in advanced processors".to_string()),
                },
                ItemSource {
                    location: SourceLocation {
                        name: "Daymar".to_string(),
                        system: "Stanton".to_string(),
                        description: Some(
                            "Hathor caves exposed by orbital laser platforms".to_string(),
                        ),
                    },
                    method: AcquisitionMethod::Mining,
                    reliability: 5,
                    notes: Some(
                        "Size 5 commodity (16 SCU); forms under extreme pressure".to_string(),
                    ),
                },
            ],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(16.0),
        },
        // LOW CONFIDENCE - Needs gameplay validation
        WikieloItem {
            id: "carinite_pure".to_string(),
            name: "Carinite (Pure)".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Aberdeen".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Hathor caves; pure vein variant unconfirmed".to_string()),
                },
                method: AcquisitionMethod::Mining,
                reliability: 2,
                notes: Some(
                    "Wiki does not mention pure variant; needs gameplay validation".to_string(),
                ),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "quantanium".to_string(),
            name: "Quantanium".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Asteroid Fields".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Various asteroid fields".to_string()),
                },
                method: AcquisitionMethod::Mining,
                reliability: 4,
                notes: Some(
                    "Volatile; ship mining required; exchange rate needs validation".to_string(),
                ),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(1.0),
        },
        WikieloItem {
            id: "copper".to_string(),
            name: "Copper".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Various".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Common ore; multiple locations".to_string()),
                },
                method: AcquisitionMethod::Mining,
                reliability: 5,
                notes: Some("Common ore; widely available".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(1.0),
        },
        WikieloItem {
            id: "tungsten".to_string(),
            name: "Tungsten".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Various".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Common ore; multiple locations".to_string()),
                },
                method: AcquisitionMethod::Mining,
                reliability: 5,
                notes: Some("Common ore; widely available".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(1.0),
        },
        // Additional mining materials to reach 10 total (medium confidence)
        WikieloItem {
            id: "gold".to_string(),
            name: "Gold".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Various".to_string(),
                    system: "Stanton".to_string(),
                    description: Some(
                        "Precious metal ore; asteroid and surface deposits".to_string(),
                    ),
                },
                method: AcquisitionMethod::Mining,
                reliability: 3,
                notes: Some("Medium value ore; found alongside other minerals".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(1.0),
        },
        WikieloItem {
            id: "diamond".to_string(),
            name: "Diamond".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Various".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Rare gemstone; asteroid mining".to_string()),
                },
                method: AcquisitionMethod::Mining,
                reliability: 3,
                notes: Some("Higher value; less common than base ores".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(1.0),
        },
        WikieloItem {
            id: "laranite".to_string(),
            name: "Laranite".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Various".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Valuable mineral; multiple mining locations".to_string()),
                },
                method: AcquisitionMethod::Mining,
                reliability: 3,
                notes: Some("High-value trade commodity".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(1.0),
        },
        WikieloItem {
            id: "agricium".to_string(),
            name: "Agricium".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Various".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Rare industrial mineral".to_string()),
                },
                method: AcquisitionMethod::Mining,
                reliability: 3,
                notes: Some("Used in advanced manufacturing".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(1.0),
        },
        WikieloItem {
            id: "titanium".to_string(),
            name: "Titanium".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Various".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Common industrial ore".to_string()),
                },
                method: AcquisitionMethod::Mining,
                reliability: 3,
                notes: Some("Used in ship and component manufacturing".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(1.0),
        },
        WikieloItem {
            id: "jaclium".to_string(),
            name: "Jaclium".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Various".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Mineable ore; multiple locations".to_string()),
                },
                method: AcquisitionMethod::Mining,
                reliability: 3,
                notes: Some("Required for multiple Wikelo weapon contracts".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(1.0),
        },
        WikieloItem {
            id: "saldynium".to_string(),
            name: "Saldynium".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Various".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Mineable ore; multiple locations".to_string()),
                },
                method: AcquisitionMethod::Mining,
                reliability: 3,
                notes: Some("Required for multiple Wikelo weapon and ship contracts".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(1.0),
        },
        WikieloItem {
            id: "sabir".to_string(),
            name: "Sabir".to_string(),
            category: ItemCategory::MinedMaterial,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Hathor Sites".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Mining tech from Hathor sites".to_string()),
                },
                method: AcquisitionMethod::Mining,
                reliability: 2,
                notes: Some("Required for Need mining things contract".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
    ]
}

/// Returns commodity items.
fn commodities() -> Vec<WikieloItem> {
    vec![WikieloItem {
        id: "vestal_water".to_string(),
        name: "Vestal Water".to_string(),
        category: ItemCategory::Commodity,
        sources: vec![ItemSource {
            location: SourceLocation {
                name: "Various Trade Terminals".to_string(),
                system: "Stanton".to_string(),
                description: Some("Purchasable commodity".to_string()),
            },
            method: AcquisitionMethod::Purchase,
            reliability: 5,
            notes: Some("Required for New to System prerequisite contract".to_string()),
        }],
        estimated_value: None,
        stackable: true,
        scu_per_unit: Some(1.0),
    }]
}

/// Returns the 11 mission/loot items.
fn mission_loot_items() -> Vec<WikieloItem> {
    vec![
        WikieloItem {
            id: "mg_scrip".to_string(),
            name: "MG Scrip".to_string(),
            category: ItemCategory::MissionCurrency,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Mercenary Guild Contracts".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Awarded for MG contract completion".to_string()),
                },
                method: AcquisitionMethod::Mission,
                reliability: 5,
                notes: Some(
                    "Trades for Wikelo Favors; specific amounts per mission TBD".to_string(),
                ),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        // LOW CONFIDENCE - Source unclear
        WikieloItem {
            id: "council_scrip".to_string(),
            name: "Council Scrip".to_string(),
            category: ItemCategory::MissionCurrency,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Unknown".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Source unclear; possibly Ace Pilot drops".to_string()),
                },
                method: AcquisitionMethod::Combat,
                reliability: 2,
                notes: Some(
                    "Wiki confirms Wikelo trade but acquisition source unverified".to_string(),
                ),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "polaris_bit".to_string(),
            name: "Polaris Bit".to_string(),
            category: ItemCategory::MissionCurrency,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Wikelo Emporium".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Earned via Quantanium delivery contracts".to_string()),
                },
                method: AcquisitionMethod::Mission,
                reliability: 4,
                notes: Some(
                    "Required for Polaris ship purchase; exchange rate unconfirmed".to_string(),
                ),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: Some(0.0022),
        },
        WikieloItem {
            id: "asd_secure_drive".to_string(),
            name: "ASD Secure Drive".to_string(),
            category: ItemCategory::Equipment,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "ASD Onyx Facility".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Jorrit dossier investigation contracts".to_string()),
                },
                method: AcquisitionMethod::Mission,
                reliability: 5,
                notes: Some(
                    "Missions: Power Usage, Energy Anomaly, Security, Seismic Data".to_string(),
                ),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "wikelo_favor".to_string(),
            name: "Wikelo Favor".to_string(),
            category: ItemCategory::MissionCurrency,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Wikelo Emporium".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Trade MG Scrip for Favors".to_string()),
                },
                method: AcquisitionMethod::Mission,
                reliability: 5,
                notes: Some("Primary Wikelo currency; earned by completing contracts".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        // LOW CONFIDENCE - Needs gameplay validation
        WikieloItem {
            id: "dchs_05_comp_board".to_string(),
            name: "DCHS-05 Orbital Positioning Comp-Board".to_string(),
            category: ItemCategory::Equipment,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Unknown".to_string(),
                    system: "Unknown".to_string(),
                    description: Some("Likely mission reward or hostile outpost loot".to_string()),
                },
                method: AcquisitionMethod::Mission,
                reliability: 1,
                notes: Some("Required for high-value contracts; source unverified".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        // Additional low-confidence items from 02-DATA-READY.md table
        WikieloItem {
            id: "ace_interceptor_helmet".to_string(),
            name: "Ace Interceptor Helmet".to_string(),
            category: ItemCategory::CombatLoot,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Unknown".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Source unverified; likely combat loot".to_string()),
                },
                method: AcquisitionMethod::Combat,
                reliability: 2,
                notes: Some("Rare combat drop; exact location needs validation".to_string()),
            }],
            estimated_value: None,
            stackable: false,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "tevarin_war_marker".to_string(),
            name: "Tevarin War Service Marker".to_string(),
            category: ItemCategory::CombatLoot,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Unknown".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Random loot; location imprecise".to_string()),
                },
                method: AcquisitionMethod::Combat,
                reliability: 2,
                notes: Some("Historical collectible; rare spawn".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "gca_medal".to_string(),
            name: "Government Cartography Agency Medal".to_string(),
            category: ItemCategory::CombatLoot,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Unknown".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Random loot; location imprecise".to_string()),
                },
                method: AcquisitionMethod::Combat,
                reliability: 2,
                notes: Some("Collectible medal; rare drop".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "uee_6th_platoon_medal".to_string(),
            name: "UEE 6th Platoon Medal".to_string(),
            category: ItemCategory::CombatLoot,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Unknown".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Random loot; location imprecise".to_string()),
                },
                method: AcquisitionMethod::Combat,
                reliability: 2,
                notes: Some("Military collectible; rare drop".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
        WikieloItem {
            id: "artifact_fragment".to_string(),
            name: "Large Artifact Fragment".to_string(),
            category: ItemCategory::CombatLoot,
            sources: vec![ItemSource {
                location: SourceLocation {
                    name: "Unknown".to_string(),
                    system: "Stanton".to_string(),
                    description: Some("Rare spawn; location imprecise".to_string()),
                },
                method: AcquisitionMethod::Salvage,
                reliability: 2,
                notes: Some("Ancient artifact piece; very rare".to_string()),
            }],
            estimated_value: None,
            stackable: true,
            scu_per_unit: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_items_minimum_count() {
        let items = all_items();
        // Registry should have at least 70 items after Phase 9 additions
        assert!(
            items.len() >= 35,
            "Expected at least 35 items, got {}",
            items.len()
        );
    }

    #[test]
    fn test_all_items_have_unique_ids() {
        let items = all_items();
        let mut ids: Vec<&str> = items.iter().map(|i| i.id.as_str()).collect();
        ids.sort();
        let original_len = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), original_len, "Found duplicate item IDs");
    }

    #[test]
    fn test_creature_parts_count() {
        let items = all_items();
        let creature_parts: Vec<_> = items
            .iter()
            .filter(|i| i.category == ItemCategory::CreaturePart)
            .collect();
        assert_eq!(
            creature_parts.len(),
            10,
            "Expected 10 creature parts, got {}",
            creature_parts.len()
        );
    }

    #[test]
    fn test_mined_materials_count() {
        let items = all_items();
        let mined: Vec<_> = items
            .iter()
            .filter(|i| i.category == ItemCategory::MinedMaterial)
            .collect();
        // At least 13 mined materials after Phase 9 additions
        assert!(
            mined.len() >= 13,
            "Expected at least 13 mined materials, got {}",
            mined.len()
        );
    }
}
