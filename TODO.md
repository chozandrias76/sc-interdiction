# TODO

## Features

### UI/Visualization Improvements

- [ ] **TUI Dashboard**: Add interactive hotspot count selector
  - Hotspot visibility logic exists (`visible_hotspot_count()` in app.rs)
  - Need: UI controls to adjust visible hotspots in real-time

---

### Route Planning & Fuel Management

#### Validate Routes Against Ship Fuel Range
- [x] **Add quantum fuel consumption calculation**
  - Created `crates/route-graph/src/fuel.rs` module
  - Implemented `calculate_qt_fuel_consumption()`, `can_complete_route()`, `max_range_mkm()`
  - Added `QtDriveEfficiency` struct with S1/S2/S3 drive efficiency ratings

- [x] **Extend ship data with fuel capacity**
  - Updated `CargoShip` struct in `crates/intel/src/ships.rs`
  - Added fields: `quantum_fuel_capacity`, `hydrogen_fuel_capacity`, `qt_drive_size`
  - Added helper methods: `qt_drive_efficiency()`, `can_complete_route()`, `max_range_mkm()`
  - Populated all ships with realistic fuel capacity data based on size class

- [x] **Route validation logic**
  - Created `FuelValidation` struct with fields: `is_possible`, `fuel_required`, `fuel_remaining`, `refuel_needed`
  - Added `validate_route_fuel(ship, distance)` function
  - Added `validate_route(distance)` method to `CargoShip`
  - Integrated into `HotRoute` and `TradeRun` structs with distance calculation
  - Routes now automatically include fuel feasibility assessment
  - ✅ Added `distance_mkm`, `fuel_sufficient`, `fuel_required` fields to route structs

#### Find Refueling Waypoints
- [x] **Index fuel terminals**
  - ✅ Created `FuelStation` and `FuelStationIndex` structures
  - ✅ Uses existing `Terminal.is_refuel` field from UEX API
  - ✅ Built spatial index with nearest-neighbor queries
  - ✅ Added system filtering and route proximity searches
  
- [x] **Path finding with fuel constraints**
  - ✅ Implemented `find_route_with_refueling(origin, dest, ship_fuel_capacity) -> Vec<Waypoint>`
  - ✅ Each waypoint includes: location, refuel (yes/no), distances
  - ✅ Uses `fuel_sufficient` field to determine when refueling needed
  - ✅ Returns direct route if possible, multi-hop with refuel stops otherwise
  
- [x] **Refueling cost calculation**
  - ✅ Added hydrogen fuel pricing constants (1.0 aUEC/unit) - **PLACEHOLDER, NEEDS VERIFICATION**
  - ✅ Added quantum fuel pricing constants (1.5 aUEC/unit) - **PLACEHOLDER, NEEDS VERIFICATION**
  - ✅ Implemented `calculate_refuel_cost(fuel_needed, price_per_unit)`
  - ✅ Implemented `calculate_route_refuel_cost()` for multi-hop routes
  - ✅ Comprehensive tests for refueling cost calculations
  - ⚠️  **CRITICAL**: Fuel prices are unverified placeholders - see `docs/DATA_SOURCES.md`
  - [ ] **TODO**: Verify actual fuel prices in-game or via UEX API
  
- [ ] **CLI display for multi-hop routes**
  - Show waypoints in route output when refueling required
  - Format: `Origin -> [REFUEL: Station Name] -> Destination`
  - Display total route time including refuel stops

#### Mark Fuel Terminals in Route Graph
- [x] **Add fuel station markers to graph**
  - ✅ Updated `Node` struct in `crates/route-graph/src/graph.rs`
  - ✅ Added field: `is_fuel_station: bool`
  - ✅ Populated from `Terminal.is_refuel` when building graph via `add_terminal()`
  - ✅ Defaults to false for Station nodes (no refuel data available)
  
- [x] **Filter terminals by type**
  - ✅ Added `--type` flag to `terminals` command
  - ✅ Supports types: fuel (is_refuel), refinery (is_refinery), or any terminal_type value
  - ✅ Case-insensitive matching on terminal_type field
  - ✅ Works with existing `--system` filter
  
- [x] **Visual indicators**
  - ✅ CLI: Added `⛽` icon to routes/runs that need refueling
  - ✅ Display fuel requirements (distance, fuel needed) on routes
  - ✅ Show refueling warnings on trade runs
  - [ ] API: Include `is_fuel_station` in node JSON responses

---

### Mining & Refinery Route Support

#### Asteroid/Mining Location Database
- [x] **Add mining locations to position database**
  - ✅ Extended `crates/route-graph/src/locations.rs`
  - ✅ Added known asteroid belts (Aaron Halo, Yela belt, AMA045, AMA141)
  - ✅ Added surface mining sites (Aberdeen Caves, Daymar Caves, Lyria)
  - ✅ Included approximate coordinates for all mining hotspots
  
- [x] **Create mining site data structure**
  - ✅ New module: `crates/route-graph/src/mining.rs`
  - ✅ Struct: `MiningSite { name, location, resource_types, avg_yield, is_surface }`
  - ✅ Enum: `ResourceType` - Quantainium, Bexalite, Taranite, Gold, etc.
  - ✅ Helper functions: `sites_with_resource()`, `nearest_mining_site()`
  - ✅ Typical value ranges for each resource type

#### Refinery Terminal Integration
- [ ] **Index refinery locations**
  - Use existing `Terminal.is_refinery` field from UEX API
  - Create `RefineryIndex` similar to fuel station index
  - Track which refineries accept which raw materials
  
- [ ] **Raw commodity pricing**
  - Extend UEX commodity queries for raw materials
  - Track ore buy prices at refineries
  - Track refined material sell prices

#### Mining Route Analysis
- [ ] **Calculate mining route profitability**
  - New module: `crates/intel/src/mining.rs`
  - Function: `analyze_mining_routes() -> Vec<MiningRoute>`
  - MiningRoute: asteroid_field -> refinery -> commodity_sell_location
  
- [ ] **Add mining ships to ship database**
  - Update `CARGO_SHIPS` in `crates/intel/src/ships.rs`
  - Add MOLE (already exists), Prospector, Vulture, etc.
  - Include mining laser stats, cargo for refined ore
  
- [ ] **CLI command for mining routes**
  - Add `Commands::MiningRoutes` subcommand
  - Display: mining site -> refinery -> market
  - Show estimated profit per load including refining time/cost
  
- [ ] **Intel for mining interdiction**
  - Predict miners departing refineries with valuable cargo
  - Show high-value refined commodities (Quantainium, etc.)
  - Flag vulnerable mining ships (slow, unarmed)

---

## Technical Debt

- [ ] **Improve error handling in API client**
  - Add retry logic with exponential backoff in `api-client/src/`
  - Better error messages for rate limits, network failures
  
- [ ] **Add more comprehensive unit tests**
  - Test route calculation accuracy
  - Test fuel consumption formulas
  - Mock API responses for deterministic tests
  
- [ ] **Optimize graph traversal algorithms**
  - Profile `find_chokepoints()` for large graphs
  - Consider caching route calculations
  - Use parallel iterators where applicable
  
- [ ] **Better caching strategies for API responses**
  - Implement TTL-based cache expiration
  - Add cache warming on server startup
  - Cache invalidation based on data freshness

---

## Documentation

- [ ] Document ship data schema and sources
- [ ] Create architecture diagram (crates and data flow)
- [x] **Add CONTRIBUTING.md with development setup**
  - ✅ Created docs/CONTRIBUTING.md
  - ✅ Documented commit size policy
  - ✅ Conventional commit format examples
  - ✅ Pre-commit hook installation instructions
- [ ] Document fuel consumption calculation methodology
- [ ] Create `docs/BUILD_CONFIGURATION.md` with detailed build setup
- [ ] Create `docs/QUICK_BUILD_SETUP.md` quick reference

---

## Infrastructure & Build System

### Build Optimization
- [x] **Configure /tmp as build target directory**
  - `.envrc` with CARGO_TARGET_DIR
  - `.cargo/config.toml.template` with optimized build profiles
  - `scripts/setup-build-env.sh` for environment setup

- [x] **Prevent local release builds**
  - ✅ Created `.bin/cargo` wrapper script to block `--release` flag
  - ✅ Updated Makefile to prevent `make build-release`
  - ✅ Forces use of CI/CD for consistent release artifacts
  - ✅ Prevents confusion about which binary to use

- [ ] **Optional: Faster linker integration**
  - Install and configure `mold` or `lld` linker
  - Add linker configuration to `.cargo/config.toml`

- [ ] **CI/CD pipeline optimization**
  - Configure GitHub Actions to use tmpfs for builds
  - Add caching strategy for dependencies
  - Optimize build matrix for different targets

### Code Quality
- [x] **Commit size enforcement**
  - ✅ Added pre-commit hook to warn on commits >500 lines
  - ✅ Interactive prompt with guidance on valid exceptions
  - ✅ Updated scripts/pre-commit.sh source
