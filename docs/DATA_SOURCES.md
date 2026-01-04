# Data Sources & Verification

This document tracks all external data sources used in the SC Interdiction tool and their verification status.

## ⚠️ CRITICAL: Unverified Data Summary

**The following features use placeholder/estimated data and may not reflect accurate in-game values:**

1. **Fuel Pricing** - Arbitrary placeholder values
2. **Quantum Drive Efficiency** - Estimated from observed patterns
3. **Salvage Yield Rates** - Community estimates, not official
4. **Component Values** - Heuristic-based estimation
5. **Mining Resource Prices** - Approximate market ranges
6. **Interdiction Proximity Range** - Gameplay assumption (2.0 Mkm)

See detailed breakdown below for impact and verification requirements.

---

## Verified Data Sources ✅

### UEX Corporation API
- **URL**: https://uexcorp.space/api/documentation/
- **Status**: Active, verified
- **Data Used**:
  - `commodities_prices_all` - Commodity prices at all terminals
  - `terminals` - Terminal locations with types (commodity, fuel, refinery, etc.)
- **Verification**: API responses tested in integration tests
- **Update Frequency**: Real-time from game data

### Fleet Yards API
- **URL**: https://fleetyards.net/
- **Status**: Active, verified
- **Data Used**:
  - Ship specifications (cargo capacity, fuel tanks, quantum drive sizes)
- **Verification**: API responses tested, matches in-game values
- **Update Frequency**: Community-maintained, updated with game patches

## Placeholder Data (Needs Verification) ⚠️

### Fuel Pricing Constants
**Location**: `crates/route-graph/src/fuel.rs`

#### Hydrogen Fuel Price
- **Current Value**: `1.0 aUEC per unit`
- **Status**: ⚠️ **PLACEHOLDER - NOT VERIFIED**
- **Source Needed**: In-game measurement or UEX commodity data
- **Impact**: Used in refueling cost calculations
- **Verification Required**:
  1. Visit refueling station in-game (e.g., Port Olisar, Everus Harbor)
  2. Check fuel service pricing for hydrogen
  3. Take screenshot showing price per unit
  4. Update constant with actual value

#### Quantum Fuel Price  
- **Current Value**: `1.5 aUEC per unit`
- **Status**: ⚠️ **PLACEHOLDER - NOT VERIFIED**
- **Source Needed**: In-game measurement or UEX commodity data
- **Impact**: Used in quantum travel refueling cost calculations
- **Verification Required**:
  1. Visit refueling station in-game
  2. Check fuel service pricing for quantum fuel
  3. Take screenshot showing price per unit
  4. Update constant with actual value
  5. Verify 1.5x multiplier assumption vs hydrogen

### Quantum Drive Efficiency Ratings
**Location**: `crates/route-graph/src/fuel.rs`

- **S1 (Small)**: `40.0 fuel units per Mkm`
- **S2 (Medium)**: `80.0 fuel units per Mkm`  
- **S3 (Large)**: `160.0 fuel units per Mkm`
- **Status**: ⚠️ **ESTIMATED - NEEDS VERIFICATION**
- **Source**: Based on relative consumption patterns observed in-game
- **Verification Required**:
  1. Test flights with different drive sizes
  2. Measure fuel consumption over known distances
  3. Compare with community measurements
  4. Update efficiency constants if needed

### Mining Resource Values
**Location**: `crates/route-graph/src/mining.rs`

Resource price ranges (aUEC per unit):
- Quantainium: 88-110
- Bexalite: 40-50
- Taranite: 25-35
- Gold: 20-30
- Others: See `ResourceType::typical_value_range()`

- **Status**: ⚠️ **ESTIMATED FROM COMMUNITY DATA**
- **Source**: General knowledge of mining market prices
- **Verification Required**:
  1. Query UEX API for raw commodity prices
  2. Cross-reference with current market data
  3. Update ranges based on actual trading data

## Static/Measured Data ✅

### Location Coordinates
**Location**: `crates/route-graph/src/locations.rs`

- **Source**: In-game `/showlocation` command + community measurements
- **Status**: ✅ Approximate positions verified
- **Notes**: 
  - Positions are relative to system star in Mkm
  - Orbital angles estimated for realistic route intersections
  - Accurate enough for distance/fuel calculations

### Ship Data
**Location**: `crates/intel/src/ships.rs`

- **Source**: Fleet Yards API + in-game verification
- **Status**: ✅ Verified for implemented ships
- **Notes**: Fuel capacities populated from Fleet Yards API data

### Mining Site Locations
**Location**: `crates/route-graph/src/locations.rs` + `mining.rs`

- **Source**: Community knowledge of mining locations
- **Status**: ⚠️ Approximate positions
- **Verification Required**:
  1. Verify positions with in-game `/showlocation` data
  2. Update coordinates for accuracy

---

## Estimated/Heuristic Data (Needs Review) ⚠️

### Salvage System Values
**Location**: `crates/intel/src/ships/types.rs`

#### RMC Yield Rate
- **Current Value**: `~1 SCU per 100kg of ship mass`
- **Status**: ⚠️ **COMMUNITY ESTIMATE**
- **Source**: General salvage gameplay observations
- **Impact**: Used in ship salvage value calculations
- **Verification Required**:
  1. Test in-game with Reclaimer/Vulture
  2. Measure actual RMC yield from known ship masses
  3. Document variance by ship type/material

#### Refinery Yield Rates
- **Current Range**: `50% (worst) to 80% (best)`
- **Status**: ⚠️ **COMMUNITY ESTIMATE**
- **Source**: Observed refinery mechanics
- **Impact**: Used in salvage profitability calculations
- **Verification Required**:
  1. Test refinement with various ore/material quality levels
  2. Document actual yield percentages
  3. Check if rates vary by refinery location

#### Component Value Estimation
**Location**: `crates/intel/src/ships/types.rs:91-122`

Base values by ship size:
- S1 (Small): 5,000 aUEC
- S2 (Medium): 18,000 aUEC
- S3 (Large): 50,000 aUEC

Manufacturer multipliers:
- Drake: 0.6x (budget)
- Greycat: 0.7x (budget industrial)
- MISC/CO: 0.85x (mid-tier)
- Argo: 0.9x (industrial)
- Anvil/Aegis: 1.2x (military)
- Origin: 1.5x (premium)

Role multipliers:
- Cargo: 0.7x
- Mining: 1.3x
- Salvage: 1.2x

- **Status**: ⚠️ **HEURISTIC MODEL - NOT VERIFIED**
- **Source**: Logical estimation based on game lore and observations
- **Impact**: Used to estimate salvage value from ship components
- **Verification Required**:
  1. Salvage multiple ships and record actual component values
  2. Build database of real component prices by size/manufacturer
  3. Validate multiplier assumptions
  4. Update model with regression from real data

### Interdiction Mechanics

#### QT Interdiction Range
**Location**: `crates/intel/src/targets.rs:503`
- **Current Value**: `2.0 Mkm proximity threshold`
- **Status**: ⚠️ **GAMEPLAY ASSUMPTION**
- **Source**: Estimated effective range for QT interdiction positioning
- **Impact**: Used to find route chokepoints/intersections
- **Verification Required**:
  1. Test in-game interdiction ranges
  2. Measure actual pull-out distances
  3. Document by interdictor type (mantis, QED, etc.)
  4. Update threshold based on actual mechanics

#### Mass-to-Cargo Estimation
**Location**: `crates/intel/src/ships/types.rs:69`
- **Current Value**: `~1000kg per SCU`
- **Status**: ⚠️ **FALLBACK ESTIMATE**
- **Source**: Rough approximation for missing mass data
- **Impact**: Used when ship mass is unknown for salvage calculations
- **Verification Required**:
  1. Get actual ship masses from Fleet Yards or in-game
  2. Update ship database with real mass values
  3. Remove fallback estimation

## How to Update Data Sources

### When Game Updates
1. Check if UEX API still returns expected data format
2. Verify ship data against Fleet Yards for new/changed ships
3. Re-verify fuel pricing if economy changes
4. Update location coordinates if planets/moons move

### Adding New Data
1. Document the source with URL/reference
2. Add verification method (how to check it's accurate)
3. Mark status (verified/placeholder/estimated)
4. Add to appropriate section in this document
5. Include comments in code referencing this doc

### Verification Process
1. Collect data from source (API, in-game, community)
2. Cross-reference with multiple sources when possible
3. Test in code with real examples
4. Document verification date and method
5. Update status from placeholder to verified

## Contributing Data

If you have verified in-game data:
1. Take screenshots showing the data source
2. Note the game version/patch
3. Submit via issue or pull request
4. Include verification method
5. Reference this document

---

**Last Updated**: 2026-01-04  
**Next Review**: When fuel/economy system changes in Star Citizen
