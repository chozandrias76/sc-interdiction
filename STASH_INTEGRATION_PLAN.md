# Stash Integration Plan

## Overview
Extract valuable features from stash@{0} and stash@{1}, excluding mining/salvage/sc-data-parser changes.
Current branch: `refactor/ships-module-reorganization` (commit 13d16b9)

## Files to Integrate (27 files, excluding mining/salvage)

### 1. API Client Enhancements (Priority: HIGH)
**File:** `crates/api-client/src/uex.rs` (+155 lines)

**Changes:**
- Add `get_cm_sale_stations()` - fetch Construction Materials sale stations
- Add `get_construction_materials_prices()` - cached CM pricing (24hr cache)
- Add `CachedCMPrices` struct for disk caching
- Add `CMSaleStation` struct
- Update `Commodity` struct fields (price_buy, price_sell, type conversions)
- Add helper methods: `is_illegal()`, `is_raw()`, `is_harvestable()`

**Integration Steps:**
1. Copy new methods to uex.rs
2. Add new structs at bottom of file
3. Update Commodity struct fields
4. Test API calls don't break existing functionality

**Conflicts:** None expected (pure additions)

---

### 2. Route Graph Spatial Enhancements (Priority: HIGH)
**File:** `crates/route-graph/src/spatial.rs` (+133 lines)

**Changes:**
- Add `origin_system` and `destination_system` fields to `RouteSegment`
- Add `is_cross_system()` method to RouteSegment
- Add `is_cross_system` field to `RouteIntersection`
- Enhanced intersection naming for cross-system routes
- New helper: `extract_system_name()` - parse system from terminal name
- New helper: `check_if_cross_system()` - detect cross-system hotspots
- Improved `infer_system_from_routes()` - better system detection

**Integration Steps:**
1. Add new fields to structs
2. Update all RouteSegment constructors to include system info
3. Add new helper functions
4. Update intersection generation logic
5. Verify existing spatial tests still pass

**Conflicts:** Moderate (struct changes require updating callers)

---

### 3. TUI Map View Enhancements (Priority: HIGH)
**File:** `crates/cli/src/tui/views/map.rs` (+160 lines)

**Changes:**
- Enhanced title bar with bottom legend (keyboard shortcuts)
- Cross-system route indicator in zone display
- Armistice zone warnings (< 100 Mm = danger, < 500 Mm = monitored)
- New expanded detail view mode (`detail_expanded` flag)
- Compact vs expanded hotspot details
- Full scrollable route list in expanded mode
- Better UX with visual indicators

**Integration Steps:**
1. Add `detail_expanded: bool` to App struct
2. Replace `render_hotspot_details()` with new version
3. Add `render_hotspot_details_compact()` helper
4. Add `render_hotspot_details_expanded()` helper
5. Update title rendering with bottom legend
6. Test map view rendering

**Conflicts:** Low (mostly additions)
**Dependencies:** Requires App struct update in tui/app.rs

---

### 4. TUI App State Updates (Priority: HIGH)
**File:** `crates/cli/src/tui/app.rs` (+62 lines)

**Changes:**
- Add `detail_expanded: bool` field to App
- Initialize to `false` in constructor
- Support toggling detail expansion

**Integration Steps:**
1. Add field to App struct
2. Initialize in `new()` method
3. Add toggle method (if needed)

**Conflicts:** None (pure addition)

---

### 5. TUI Key Handlers (Priority: HIGH)
**File:** `crates/cli/src/tui/handlers/keys.rs` (+68 lines)

**Changes:**
- Add Enter key handler to toggle detail expansion
- Add 'h'/'l' navigation keys for map view
- Enhanced key routing logic

**Integration Steps:**
1. Add new key handlers
2. Connect to App state changes
3. Test keyboard shortcuts work

**Conflicts:** Low

---

### 6. TUI Navigation Improvements (Priority: MEDIUM)
**File:** `crates/cli/src/tui/handlers/navigation.rs` (+35 lines)

**Changes:**
- Enhanced navigation helpers
- Better selection wrapping

**Integration Steps:**
1. Review and merge navigation improvements
2. Test navigation flows

**Conflicts:** Low

---

### 7. TUI Hotspot Data Enhancements (Priority: MEDIUM)
**File:** `crates/cli/src/tui/data/hotspots.rs` (+143 lines)

**Changes:**
- Better hotspot loading
- Enhanced filtering
- System-aware hotspot handling

**Integration Steps:**
1. Review data loading logic
2. Merge enhancements
3. Test hotspot data accuracy

**Conflicts:** Low

---

### 8. TUI View Updates (Priority: MEDIUM)
**Files:**
- `crates/cli/src/tui/views/routes.rs` (+22 lines)
- `crates/cli/src/tui/views/targets.rs` (+27 lines)
- `crates/cli/src/tui/views/help.rs` (+3 lines)

**Changes:**
- Better formatting
- Enhanced display logic
- Help text updates

**Integration Steps:**
1. Review each view file
2. Merge display improvements
3. Test rendering

**Conflicts:** Low

---

### 9. TUI Module Updates (Priority: MEDIUM)
**Files:**
- `crates/cli/src/tui/mod.rs` (+70 lines)
- `crates/cli/src/tui/data/mod.rs` (+2 lines)
- `crates/cli/src/tui/data/map_locations.rs` (+12 lines)
- `crates/cli/src/tui/handlers/sorting.rs` (+2 lines)

**Changes:**
- Module organization improvements
- New exports
- Better structure

**Integration Steps:**
1. Review module changes
2. Merge organizational improvements
3. Verify exports work

**Conflicts:** Low

---

### 10. CLI Main Enhancements (Priority: MEDIUM)
**File:** `crates/cli/src/main.rs` (+228 lines)

**Changes:**
- New command implementations
- Enhanced error handling
- Better output formatting

**Integration Steps:**
1. Review new commands
2. Check for conflicts with ships module changes
3. Merge CLI improvements
4. Test commands work

**Conflicts:** MODERATE (may conflict with ships refactor changes)

---

### 11. Intel Targets Updates (Priority: MEDIUM)
**File:** `crates/intel/src/targets.rs` (+57 lines)

**Changes:**
- Enhanced target prediction
- Better route analysis

**Integration Steps:**
1. Review analyzer improvements
2. Merge with current code
3. Test target predictions

**Conflicts:** Low to moderate

---

### 12. Server Updates (Priority: LOW)
**Files:**
- `crates/server/src/routes.rs` (+8 lines)
- `crates/server/src/state.rs` (+13 lines)

**Changes:**
- Minor API improvements
- Better state handling

**Integration Steps:**
1. Review server changes
2. Check compatibility with ships refactor
3. Merge if compatible

**Conflicts:** MODERATE (may conflict with ships refactor)

---

### 13. Dependency/Config Updates (Priority: LOW)
**Files:**
- `crates/intel/Cargo.toml` (+1 line)
- `crates/intel/src/ships_tests.rs` (+13 lines)
- `Cargo.toml` (+35 lines workspace changes)
- `Cargo.lock` (dependency changes)
- `Makefile` (+20 lines)

**Changes:**
- New dependencies
- Test improvements
- Build configuration

**Integration Steps:**
1. Review dependency additions
2. Check for conflicts with current deps
3. Merge carefully
4. Run cargo update if needed

**Conflicts:** High (lock file conflicts expected)

---

### 14. Ignore Files (Priority: IGNORE)
**Files:**
- `.envrc` (just newline)
- `data/cache/.gitkeep` (deleted)

**Integration Steps:**
- Skip these changes

---

## Integration Order (Recommended)

### Phase 1: Foundation (Core functionality)
1. ✅ Route-graph spatial enhancements (system tracking)
2. ✅ API client UEX enhancements (CM prices)
3. ✅ Intel targets updates

### Phase 2: TUI Core
4. ✅ App state updates (detail_expanded flag)
5. ✅ Hotspot data enhancements

### Phase 3: TUI Views
6. ✅ Map view enhancements
7. ✅ Routes/targets/help view updates
8. ✅ Key handlers
9. ✅ Navigation improvements

### Phase 4: CLI & Module Organization
10. ✅ TUI module reorganization
11. ✅ CLI main enhancements (careful with conflicts)

### Phase 5: Infrastructure
12. ⚠️  Server updates (check compatibility)
13. ⚠️  Cargo/dependency updates (resolve conflicts)
14. ⚠️  Ship tests updates (verify with new ships module)

### Phase 6: Cleanup
15. ✅ Drop stash@{2} (trivial change)
16. ✅ Drop stash@{0} and stash@{1} after extraction

---

## Risk Assessment

### High Risk (Conflicts Expected)
- `cli/src/main.rs` - lots of changes, may conflict with ships refactor
- `server/` files - may reference old ships module
- `Cargo.lock` - guaranteed conflicts
- `intel/src/ships_tests.rs` - references old ships.rs

### Medium Risk
- `route-graph/src/spatial.rs` - struct changes affect callers
- `intel/src/targets.rs` - may have conflicts

### Low Risk
- TUI files - mostly isolated
- API client - pure additions
- View files - display logic only

---

## Testing Strategy

After each phase:
1. `cargo check --all-targets` - verify compilation
2. `cargo test` - run tests
3. `cargo clippy` - check for issues
4. Manual testing of affected features

---

## Conflict Resolution Notes

### Ships Module References
Stash was created BEFORE ships module refactor. Need to update:
- Any `intel::CARGO_SHIPS` references
- Any `use intel::CargoShip` imports
- Ship field accesses (name is now String, not &str)

### Server State
Server state now loads ShipRegistry async. Check if stashed server changes conflict.

### CLI Commands
CLI commands now use `load_registry()` helper. May need to reconcile with stashed CLI changes.

---

## Post-Integration Verification

1. All tests pass
2. TUI renders correctly
3. Map view shows cross-system routes
4. Detail expansion works
5. CM prices fetch correctly
6. Spatial hotspots detect cross-system correctly
7. No clippy warnings
8. Code is formatted

---

## Files to EXCLUDE from Integration

- `crates/intel/src/mining.rs` (mining enhancements)
- `crates/intel/src/salvage.rs` (salvage enhancements)  
- `crates/sc-data-parser/**` (entire deleted crate)
- `.envrc` (personal config)
- `data/cache/.gitkeep` (trivial)

---

## Execution Plan

### Step 1: Create Integration Branch
```bash
git checkout -b integrate/stashed-features
git checkout refactor/ships-module-reorganization -- .
```

### Step 2: Extract Stash Selectively
```bash
# Apply only non-mining/salvage files
git checkout stash@{0} -- crates/api-client/src/uex.rs
git checkout stash@{0} -- crates/route-graph/src/spatial.rs
# ... continue for each file
```

### Step 3: Resolve Conflicts Manually
- Update ships module references
- Fix struct field types
- Reconcile server state changes

### Step 4: Test Incrementally
After each file integration, run tests.

### Step 5: Commit in Phases
Commit after each phase for easier debugging.

---

## Success Criteria

✅ All stashed features integrated (except mining/salvage)
✅ No compilation errors
✅ All tests passing
✅ TUI enhanced with cross-system support
✅ CM pricing API working
✅ Spatial hotspots improved
✅ Code quality maintained (clippy, fmt)
