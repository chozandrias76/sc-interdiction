# Phase 2 Complete: Data Ready for Phase 3

**Generated:** 2026-01-15
**Purpose:** Copy-paste-ready Rust struct examples for Phase 3 wikelo-data crate population

---

## Summary Statistics

| Category | Total Items | High (4-5) | Medium (3) | Low (1-2) |
|----------|-------------|------------|------------|-----------|
| Creature Parts | 8 | 4 | 2 | 2 |
| Mining Materials | 10 | 4 | 5 | 1 |
| Mission/Loot Items | 13 | 4 | 3 | 6 |
| **TOTAL** | **31** | **12** | **10** | **9** |

**Phase 3 Ready:** 22 items (reliability 3+)
**Needs Gameplay Validation:** 9 items (reliability 1-2)

---

## Creature Items (Plan 02-01)

### irradiated_valakkar_fang_juvenile
```rust
WikieloItem {
    id: "irradiated_valakkar_fang_juvenile".to_string(),
    name: "Irradiated Valakkar Fang (Juvenile)".to_string(),
    category: ItemCategory::CreaturePart,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Monox".to_string(),
                system: "Pyro".to_string(),
                description: Some("Invasive species location; ~5m juveniles".to_string()),
            },
            method: AcquisitionMethod::Hunting,
            reliability: 4,
            notes: Some("Native to Leir III; also found on Daymar".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### irradiated_valakkar_fang_adult
```rust
WikieloItem {
    id: "irradiated_valakkar_fang_adult".to_string(),
    name: "Irradiated Valakkar Fang (Adult)".to_string(),
    category: ItemCategory::CreaturePart,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Pyro I".to_string(),
                system: "Pyro".to_string(),
                description: Some("Invasive species location; ~15m adults".to_string()),
            },
            method: AcquisitionMethod::Hunting,
            reliability: 4,
            notes: Some("Attack when their litter is threatened".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### irradiated_valakkar_fang_apex
```rust
WikieloItem {
    id: "irradiated_valakkar_fang_apex".to_string(),
    name: "Irradiated Valakkar Fang (Apex)".to_string(),
    category: ItemCategory::CreaturePart,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Pyro I".to_string(),
                system: "Pyro".to_string(),
                description: Some("Apex variant; up to 300m".to_string()),
            },
            method: AcquisitionMethod::Hunting,
            reliability: 4,
            notes: Some("Largest form; requires team to hunt".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### irradiated_valakkar_pearl
```rust
WikieloItem {
    id: "irradiated_valakkar_pearl".to_string(),
    name: "Irradiated Valakkar Pearl".to_string(),
    category: ItemCategory::CreaturePart,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Monox".to_string(),
                system: "Pyro".to_string(),
                description: Some("Pearl-like body parts harvested from Valakkar".to_string()),
            },
            method: AcquisitionMethod::Hunting,
            reliability: 4,
            notes: Some("Same creatures as fangs; different drop".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### tundra_kopion_horn
```rust
WikieloItem {
    id: "tundra_kopion_horn".to_string(),
    name: "Tundra Kopion Horn".to_string(),
    category: ItemCategory::CreaturePart,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "microTech".to_string(),
                system: "Stanton".to_string(),
                description: Some("Equator/Green Zones; elevated grasslands/tundras".to_string()),
            },
            method: AcquisitionMethod::Hunting,
            reliability: 4,
            notes: Some("Packs of 8-18; thicker exoskeleton with pale mottled coloring".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### irradiated_kopion_horn
```rust
// LOW CONFIDENCE - Needs gameplay validation
WikieloItem {
    id: "irradiated_kopion_horn".to_string(),
    name: "Irradiated Kopion Horn".to_string(),
    category: ItemCategory::CreaturePart,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Unknown".to_string(),
                system: "Unknown".to_string(),
                description: Some("Likely Pyro variant; unverified".to_string()),
            },
            method: AcquisitionMethod::Hunting,
            reliability: 2,
            notes: Some("May not exist as separate item; needs validation".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### yormandi_eye
```rust
WikieloItem {
    id: "yormandi_eye".to_string(),
    name: "Yormandi Eye".to_string(),
    category: ItemCategory::CreaturePart,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "ASD Onyx Facility".to_string(),
                system: "Stanton".to_string(),
                description: Some("Underground cave system".to_string()),
            },
            method: AcquisitionMethod::Combat,
            reliability: 5,
            notes: Some("Antibody-rich eyes valued by medical researchers".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### yormandi_tongue
```rust
WikieloItem {
    id: "yormandi_tongue".to_string(),
    name: "Yormandi Tongue".to_string(),
    category: ItemCategory::CreaturePart,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "ASD Onyx Facility".to_string(),
                system: "Stanton".to_string(),
                description: Some("Underground cave system".to_string()),
            },
            method: AcquisitionMethod::Combat,
            reliability: 5,
            notes: Some("Prized for culinary use; complex subtle flavor".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### quasi_grazer_tongue
```rust
WikieloItem {
    id: "quasi_grazer_tongue".to_string(),
    name: "Quasi Grazer Tongue".to_string(),
    category: ItemCategory::CreaturePart,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Terra III (Quasi)".to_string(),
                system: "Terra".to_string(),
                description: Some("Native habitat; also on terraformed planets".to_string()),
            },
            method: AcquisitionMethod::Hunting,
            reliability: 3,
            notes: Some("'Space cow' found on most UEE terraformed worlds".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### quasi_grazer_egg
```rust
WikieloItem {
    id: "quasi_grazer_egg".to_string(),
    name: "Quasi Grazer Egg".to_string(),
    category: ItemCategory::CreaturePart,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Terra III".to_string(),
                system: "Terra".to_string(),
                description: Some("Desert, Boreal, Grassland variants".to_string()),
            },
            method: AcquisitionMethod::Hunting,
            reliability: 3,
            notes: Some("Used for cooking; different variants by biome".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

---

## Mining Materials (Plan 02-02)

### carinite
```rust
WikieloItem {
    id: "carinite".to_string(),
    name: "Carinite".to_string(),
    category: ItemCategory::MinedMaterial,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Aberdeen".to_string(),
                system: "Stanton".to_string(),
                description: Some("Hathor caves exposed by orbital laser platforms".to_string()),
            },
            method: AcquisitionMethod::Mining,
            reliability: 5,
            notes: Some("Deep-red mineral; used in advanced processors".to_string()),
        },
        ItemSource {
            location: SourceLocation {
                name: "Daymar".to_string(),
                system: "Stanton".to_string(),
                description: Some("Hathor caves exposed by orbital laser platforms".to_string()),
            },
            method: AcquisitionMethod::Mining,
            reliability: 5,
            notes: Some("Size 5 commodity (16 SCU); forms under extreme pressure".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: Some(16.0),
}
```

### carinite_pure
```rust
// LOW CONFIDENCE - Needs gameplay validation
WikieloItem {
    id: "carinite_pure".to_string(),
    name: "Carinite (Pure)".to_string(),
    category: ItemCategory::MinedMaterial,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Aberdeen".to_string(),
                system: "Stanton".to_string(),
                description: Some("Hathor caves; pure vein variant unconfirmed".to_string()),
            },
            method: AcquisitionMethod::Mining,
            reliability: 2,
            notes: Some("Wiki does not mention pure variant; needs gameplay validation".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### quantanium
```rust
WikieloItem {
    id: "quantanium".to_string(),
    name: "Quantanium".to_string(),
    category: ItemCategory::MinedMaterial,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Asteroid Fields".to_string(),
                system: "Stanton".to_string(),
                description: Some("Various asteroid fields".to_string()),
            },
            method: AcquisitionMethod::Mining,
            reliability: 4,
            notes: Some("Volatile; ship mining required; exchange rate needs validation".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: Some(1.0),
}
```

### copper
```rust
WikieloItem {
    id: "copper".to_string(),
    name: "Copper".to_string(),
    category: ItemCategory::MinedMaterial,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Various".to_string(),
                system: "Stanton".to_string(),
                description: Some("Common ore; multiple locations".to_string()),
            },
            method: AcquisitionMethod::Mining,
            reliability: 5,
            notes: Some("Common ore; widely available".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: Some(1.0),
}
```

### tungsten
```rust
WikieloItem {
    id: "tungsten".to_string(),
    name: "Tungsten".to_string(),
    category: ItemCategory::MinedMaterial,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Various".to_string(),
                system: "Stanton".to_string(),
                description: Some("Common ore; multiple locations".to_string()),
            },
            method: AcquisitionMethod::Mining,
            reliability: 5,
            notes: Some("Common ore; widely available".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: Some(1.0),
}
```

---

## Mission/Loot Items (Plan 02-03)

### mg_scrip
```rust
WikieloItem {
    id: "mg_scrip".to_string(),
    name: "MG Scrip".to_string(),
    category: ItemCategory::MissionCurrency,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Mercenary Guild Contracts".to_string(),
                system: "Stanton".to_string(),
                description: Some("Awarded for MG contract completion".to_string()),
            },
            method: AcquisitionMethod::Mission,
            reliability: 5,
            notes: Some("Trades for Wikelo Favors; specific amounts per mission TBD".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### council_scrip
```rust
// LOW CONFIDENCE - Source unclear
WikieloItem {
    id: "council_scrip".to_string(),
    name: "Council Scrip".to_string(),
    category: ItemCategory::MissionCurrency,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Unknown".to_string(),
                system: "Stanton".to_string(),
                description: Some("Source unclear; possibly Ace Pilot drops".to_string()),
            },
            method: AcquisitionMethod::Combat,
            reliability: 2,
            notes: Some("Wiki confirms Wikelo trade but acquisition source unverified".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### polaris_bit
```rust
WikieloItem {
    id: "polaris_bit".to_string(),
    name: "Polaris Bit".to_string(),
    category: ItemCategory::MissionCurrency,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Wikelo Emporium".to_string(),
                system: "Stanton".to_string(),
                description: Some("Earned via Quantanium delivery contracts".to_string()),
            },
            method: AcquisitionMethod::Mission,
            reliability: 4,
            notes: Some("Required for Polaris ship purchase; exchange rate unconfirmed".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: Some(0.0022),
}
```

### asd_secure_drive
```rust
WikieloItem {
    id: "asd_secure_drive".to_string(),
    name: "ASD Secure Drive".to_string(),
    category: ItemCategory::Equipment,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "ASD Onyx Facility".to_string(),
                system: "Stanton".to_string(),
                description: Some("Jorrit dossier investigation contracts".to_string()),
            },
            method: AcquisitionMethod::Mission,
            reliability: 5,
            notes: Some("Missions: Power Usage, Energy Anomaly, Security, Seismic Data".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### wikelo_favor
```rust
WikieloItem {
    id: "wikelo_favor".to_string(),
    name: "Wikelo Favor".to_string(),
    category: ItemCategory::MissionCurrency,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Wikelo Emporium".to_string(),
                system: "Stanton".to_string(),
                description: Some("Trade MG Scrip for Favors".to_string()),
            },
            method: AcquisitionMethod::Mission,
            reliability: 5,
            notes: Some("Primary Wikelo currency; earned by completing contracts".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

### dchs_05_comp_board
```rust
// LOW CONFIDENCE - Needs gameplay validation
WikieloItem {
    id: "dchs_05_comp_board".to_string(),
    name: "DCHS-05 Orbital Positioning Comp-Board".to_string(),
    category: ItemCategory::Equipment,
    sources: vec![
        ItemSource {
            location: SourceLocation {
                name: "Unknown".to_string(),
                system: "Unknown".to_string(),
                description: Some("Likely mission reward or hostile outpost loot".to_string()),
            },
            method: AcquisitionMethod::Mission,
            reliability: 1,
            notes: Some("Required for high-value contracts; source unverified".to_string()),
        },
    ],
    estimated_value: None,
    stackable: true,
    scu_per_unit: None,
}
```

---

## Low Confidence Items (Reliability 1-2)

These items need gameplay validation before Phase 3 implementation:

| Item ID | Name | Reliability | Issue |
|---------|------|-------------|-------|
| `irradiated_kopion_horn` | Irradiated Kopion Horn | 2 | Pyro variant unverified; may not exist |
| `carinite_pure` | Carinite (Pure) | 2 | Wiki doesn't mention pure variant |
| `council_scrip` | Council Scrip | 2 | Acquisition source unclear |
| `ace_interceptor_helmet` | Ace Interceptor Helmet | 2 | Source unverified |
| `tevarin_war_marker` | Tevarin War Service Marker | 2 | Random loot; location imprecise |
| `gca_medal` | Government Cartography Agency Medal | 2 | Random loot; location imprecise |
| `uee_6th_platoon_medal` | UEE 6th Platoon Medal | 2 | Random loot; location imprecise |
| `artifact_fragment` | Large Artifact Fragment | 2 | Rare spawn; location imprecise |
| `dchs_05_comp_board` | DCHS-05 Orbital Positioning Comp-Board | 1 | Source completely unknown |

---

## Phase 3 Implementation Notes

1. **High-confidence items (22)** can be implemented immediately with verified data
2. **Low-confidence items (9)** should be implemented with placeholder sources marked `reliability: 1-2`
3. **Location naming** should follow terminal naming convention from types.rs
4. **System field** should match routing system enum (Stanton, Pyro, Terra)
5. **SCU values** should use Option for non-commodity items

---

*Generated by Phase 2 Plan 01: Research Verification*
*Ready for Phase 3: Wikelo Data Module*
