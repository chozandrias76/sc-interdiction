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

- [ ] **Display fuel warnings in output**
  - Update CLI route display to show fuel status
  - Add `⚠️ REFUEL REQUIRED` indicator for routes exceeding tank capacity
  - Show estimated fuel cost

#### Find Refueling Waypoints
- [ ] **Index fuel terminals**
  - Use existing `Terminal.is_refuel` field (already in UEX API!)
  - Create `FuelStationIndex` in `crates/route-graph/src/fuel.rs`
  - Build spatial index of refuel locations
  
- [ ] **Path finding with fuel constraints**
  - Extend `RouteGraph` to support multi-hop routing with refuel stops
  - Implement `find_route_with_refueling(origin, dest, ship_fuel_capacity) -> Vec<Waypoint>`
  - Each waypoint includes: location, refuel (yes/no), fuel cost
  
- [ ] **Refueling cost calculation**
  - Add hydrogen fuel pricing to commodity data
  - Calculate refuel cost per stop: `refuel_cost = (fuel_needed * fuel_price_per_unit)`
  - Subtract from route profitability
  
- [ ] **CLI display for multi-hop routes**
  - Show waypoints in route output
  - Format: `Origin -> [REFUEL: Station Name] -> Destination`
  - Display total route time including refuel stops

#### Mark Fuel Terminals in Route Graph
- [ ] **Add fuel station markers to graph**
  - Update `Node` struct in `crates/route-graph/src/graph.rs`
  - Add field: `is_fuel_station: bool`
  - Populate from `Terminal.is_refuel` when building graph
  
- [ ] **Filter terminals by type**
  - Add `--type fuel` flag to `terminals` command
  - Support types: fuel, refinery, commodity, commodity_raw, vehicle_rent, vehicle_buy
  - Use existing `Terminal.terminal_type` field
  
- [ ] **Visual indicators**
  - CLI: Add `[⛽]` icon next to fuel stations in output
  - API: Include `is_fuel_station` in node JSON responses

---

### Mining & Refinery Route Support

#### Asteroid/Mining Location Database
- [ ] **Add mining locations to position database**
  - Extend `crates/route-graph/src/locations.rs`
  - Add known asteroid belts (Aaron Halo, Yela belt, etc.)
  - Include approximate coordinates for mining hotspots
  
- [ ] **Create mining site data structure**
  - New struct: `MiningSite { name, location, resource_types, avg_yield }`
  - Support resources: Quantainium, Bexalite, Taranite, Gold, etc.

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
- [ ] Add CONTRIBUTING.md with development setup
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

- [ ] **Optional: Faster linker integration**
  - Install and configure `mold` or `lld` linker
  - Add linker configuration to `.cargo/config.toml`

- [ ] **CI/CD pipeline optimization**
  - Configure GitHub Actions to use tmpfs for builds
  - Add caching strategy for dependencies
  - Optimize build matrix for different targets
