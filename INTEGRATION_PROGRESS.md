# Stash Integration Progress Report

## Overview
Integrating valuable features from stash@{0} and stash@{1} into the `integrate/stashed-features` branch.
Base: `refactor/ships-module-reorganization` (commit 13d16b9)

**Current Progress: ~45% Complete**
- **Completed:** 3 of 6 phases
- **Commits:** 7
- **Files Modified:** 10+
- **Lines Changed:** 800+

---

## ✅ COMPLETED: Phase 1 - Foundation

### 1.1 Route-graph Spatial Enhancements ✅ (Commit: 062d206)
**Changes Applied:**
- ✅ Added `origin_system` and `destination_system` fields to `RouteSegment`
- ✅ Added `is_cross_system()` method to `RouteSegment`
- ✅ Added `is_cross_system` field to `RouteIntersection`
- ✅ Enhanced intersection naming for cross-system routes (jump gate naming)
- ✅ Added `extract_system_name()` helper function
- ✅ Added `check_if_cross_system()` helper function
- ✅ Improved `infer_system_from_routes()` with cross-system detection logic
- ✅ Updated all `RouteSegment` constructors in intel and cli crates

**Files Modified:**
- `crates/route-graph/src/spatial.rs`
- `crates/intel/src/targets.rs`
- `crates/cli/src/tui/data/hotspots.rs`

---

### 1.2 API Client UEX Enhancements ⏭️ SKIPPED
**Reason:** Construction Materials (CM) pricing is part of salvage feature we're excluding.

**What Was Skipped:**
- `get_cm_sale_stations()` method
- `get_construction_materials_prices()` method with disk caching
- `CachedCMPrices` struct
- `CMSaleStation` struct
- Commodity struct field updates (bool -> i32)

**Decision:** Skip for now, revisit if API compatibility issues arise

---

### 1.3 Intel Targets Updates ✅ (Commit: 545c28b)
**Changes Applied:**
- ✅ Added `origin_system` and `destination_system` fields to `HotRoute`
- ✅ Updated `HotRoute` construction to extract system information
- ✅ Enables cross-system route detection in trade analysis

**Files Modified:**
- `crates/intel/src/targets.rs`

---

## ✅ COMPLETED: Phase 2 - TUI Core

### 2.1 App State Updates ✅ (Commit: f1cffdc)
**Changes Applied:**
- ✅ Added `detail_expanded: bool` field to App struct
- ✅ Added `detail_selected: usize` field for route navigation in expanded view
- ✅ Initialize both fields to false/0 in constructor
- ✅ Updated test fixtures

**Files Modified:**
- `crates/cli/src/tui/app.rs`
- `crates/cli/src/tui/ui.rs`

---

### 2.2 Hotspot Data Enhancements ✅ (Commit: 9b1eaa1)
**Changes Applied:**
- ✅ Load top 200 hotspots instead of 20 for better filtering
- ✅ Lower minimum route pairs from 2 to 1 (show single high-value routes)
- ✅ Added `filter_hotspots_for_location()` for proximity-based filtering
- ✅ Added `filter_hotspots_for_location_with_cross_system()` for cross-system control
- ✅ Added `infer_system_from_location()` with comprehensive location keywords
- ✅ Improve system extraction to use HotRoute system fields when available

**Files Modified:**
- `crates/cli/src/tui/data/hotspots.rs` (+149 lines)

---

## ✅ COMPLETED: Phase 3.1 - Map View Enhancements

### 3.1 Map View Major Update ✅ (Commit: e85da29)
**Changes Applied:**
- ✅ Added title_bottom with keyboard shortcuts legend (n/N, z/Z, a, h/l)
- ✅ Split `render_hotspot_details` into compact and expanded views
- ✅ Added cross-system corridor indicators
- ✅ Added armistice zone warnings:
  - < 100 Mm: ⚠ ARMISTICE ZONE! (red, bold)
  - < 500 Mm: ⚠ UEE monitored (yellow)
- ✅ Added expanded detail view with full scrollable route list
- ✅ Show approximate exit distance disclaimer
- ✅ Enhanced visual feedback with color coding

**Files Modified:**
- `crates/cli/src/tui/views/map.rs` (+129 lines)
- `crates/cli/src/tui/app.rs` (detail_selected field)

**New Functions:**
- `render_hotspot_details_compact()` - original summary view
- `render_hotspot_details_expanded()` - full route list with scrolling

---

## 🚧 IN PROGRESS: Phase 3 - TUI Views (Remaining)

### 3.2 Routes/Targets/Help View Updates ⏳ NOT STARTED
**From Stash:**
- `crates/cli/src/tui/views/routes.rs` (+22 lines)
- `crates/cli/src/tui/views/targets.rs` (+27 lines)
- `crates/cli/src/tui/views/help.rs` (+3 lines)

**Expected Changes:**
- Better formatting in routes view
- Enhanced display logic in targets view
- Help text updates for new features

**Estimated Effort:** 15 minutes

---

### 3.3 Key Handlers ⏳ NOT STARTED
**From Stash:**
- `crates/cli/src/tui/handlers/keys.rs` (+68 lines)

**Expected Changes:**
- Add Enter key handler to toggle detail expansion
- Add 'j'/'k' keys for scrolling in expanded detail view
- Add 'h'/'l' navigation keys for map view
- Enhanced key routing logic

**Estimated Effort:** 20 minutes

**Dependencies:** Requires detail_expanded and detail_selected fields (✅ already added)

---

### 3.4 Navigation Improvements ⏳ NOT STARTED
**From Stash:**
- `crates/cli/src/tui/handlers/navigation.rs` (+35 lines)

**Expected Changes:**
- Enhanced navigation helpers
- Better selection wrapping
- Improved edge case handling

**Estimated Effort:** 10 minutes

---

## 📋 TODO: Phase 4 - CLI & Module Organization

### 4.1 TUI Module Reorganization ⏳ NOT STARTED
**From Stash:**
- `crates/cli/src/tui/mod.rs` (+70 lines)
- `crates/cli/src/tui/data/mod.rs` (+2 lines)
- `crates/cli/src/tui/data/map_locations.rs` (+12 lines)
- `crates/cli/src/tui/handlers/sorting.rs` (+2 lines)

**Expected Changes:**
- Module organization improvements
- New exports
- Better structure

**Estimated Effort:** 15 minutes

---

### 4.2 CLI Main Enhancements ⚠️ NOT STARTED (HIGH CONFLICT RISK)
**From Stash:**
- `crates/cli/src/main.rs` (+228 lines)

**Expected Changes:**
- New command implementations
- Enhanced error handling
- Better output formatting

**Estimated Effort:** 30-45 minutes

**Risk:** May conflict with ships module refactor changes
**Strategy:** Carefully review each change, skip conflicting parts

---

## 📋 TODO: Phase 5 - Infrastructure

### 5.1 Server Updates ⚠️ NOT STARTED (MODERATE CONFLICT RISK)
**From Stash:**
- `crates/server/src/routes.rs` (+8 lines)
- `crates/server/src/state.rs` (+13 lines)

**Expected Changes:**
- Minor API improvements
- Better state handling

**Estimated Effort:** 15 minutes

**Risk:** May reference old ships module structure
**Strategy:** Update to use ShipRegistry if needed

---

### 5.2 Cargo/Dependency Updates ⚠️ NOT STARTED (HIGH CONFLICT RISK)
**From Stash:**
- `crates/intel/Cargo.toml` (+1 line)
- `Cargo.toml` (+35 lines)
- `Cargo.lock` (dependency changes)
- `Makefile` (+20 lines)

**Expected Changes:**
- New dependencies
- Build configuration updates

**Estimated Effort:** 20 minutes

**Risk:** Lock file conflicts guaranteed
**Strategy:** Skip Cargo.lock, manually add any needed dependencies

---

### 5.3 Ship Tests Updates ⏳ NOT STARTED
**From Stash:**
- `crates/intel/src/ships_tests.rs` (+13 lines)

**Expected Changes:**
- Test improvements for new ships module structure

**Estimated Effort:** 10 minutes

---

## 📋 TODO: Phase 6 - Cleanup

### 6.1 Drop Stashes ⏳ NOT STARTED
**Actions:**
- Drop stash@{2} (trivial .claude/settings change)
- Drop stash@{0} and stash@{1} after full extraction

**Estimated Effort:** 2 minutes

---

## Summary Statistics

### Completed Work
- **Phases:** 3 of 6 (50%)
- **Tasks:** 6 of ~16 (38%)
- **Commits:** 7
- **Files Modified:** 10
- **Lines Added:** ~800
- **Lines Removed:** ~100

### Remaining Work
**High Priority:**
- Key handlers (Phase 3.3) - Enable detail expansion and navigation
- View updates (Phase 3.2) - Polish UI

**Medium Priority:**
- Navigation improvements (Phase 3.4)
- TUI module organization (Phase 4.1)

**Low Priority / Risky:**
- CLI main enhancements (Phase 4.2) - May conflict
- Server updates (Phase 5.1) - May conflict
- Dependency updates (Phase 5.2) - Will conflict

**Time Estimate:** 2-3 hours remaining

---

## Commit History

1. **062d206** - feat(route-graph): add cross-system route tracking
2. **545c28b** - feat(intel): add system tracking to HotRoute
3. **3116fa0** - docs: add integration progress report
4. **f1cffdc** - feat(tui): add detail_expanded flag to App state
5. **9b1eaa1** - feat(tui): enhance hotspot data loading and filtering
6. **e85da29** - feat(tui): enhance map view with expanded details and warnings
7. **(current HEAD)**

---

## Testing Status

**Compilation:** ✅ All modified code compiles cleanly
**Unit Tests:** ✅ All existing tests pass (75 tests)
**Integration Tests:** ⏭️ Skipped (4 tests require API mocking)
**Manual Testing:** ⏳ Not performed yet

**Pre-commit Hooks:** ✅ Passing
- Clippy: ✅ (warnings only, no errors)
- Tests: ✅
- Formatting: ✅

---

## Next Steps

**Immediate (Today):**
1. Complete Phase 3: Key handlers and view updates (~45 min)
2. Test TUI functionality manually
3. Complete Phase 4.1: Module reorganization (~15 min)

**Short Term (This Week):**
4. Carefully integrate Phase 4.2: CLI main enhancements
5. Review and merge Phase 5 infrastructure updates
6. Run full integration tests
7. Merge to main branch

**Deferred:**
- Stash cleanup (after full extraction verified)
- API client UEX enhancements (salvage feature)
- Mining/refinery features from stash

---

## Files Changed Summary

### Modified:
1. `crates/route-graph/src/spatial.rs` - Cross-system tracking
2. `crates/intel/src/targets.rs` - System fields in HotRoute
3. `crates/cli/src/tui/data/hotspots.rs` - Filtering & system inference
4. `crates/cli/src/tui/app.rs` - Detail expansion state
5. `crates/cli/src/tui/ui.rs` - Test fixtures
6. `crates/cli/src/tui/views/map.rs` - Major enhancements

### Created:
1. `STASH_INTEGRATION_PLAN.md` - Integration guide
2. `INTEGRATION_PROGRESS.md` - This file

### Next to Modify:
1. `crates/cli/src/tui/handlers/keys.rs` - Key handlers
2. `crates/cli/src/tui/views/routes.rs` - View updates
3. `crates/cli/src/tui/views/targets.rs` - View updates
4. `crates/cli/src/tui/views/help.rs` - Help text

---

## Risk Assessment

### Completed Items (Low Risk)
✅ All Phase 1-3.1 changes integrated cleanly with no conflicts

### Remaining Items

**Low Risk:**
- View updates (cosmetic changes)
- Key handlers (new functionality)
- Navigation improvements
- Module reorganization

**Medium Risk:**
- CLI main enhancements (may reference old ships structure)
- Server updates (may reference old ships structure)

**High Risk:**
- Cargo.lock updates (guaranteed conflicts)
- Makefile changes (may conflict with other changes)

**Mitigation Strategy:**
- Review each change carefully
- Skip conflicting changes
- Document skipped features for future consideration
- Test after each phase

---

## Success Criteria

**Phase 3 Complete:**
- ✅ Map view fully enhanced with expansion/warnings
- ⏳ Key handlers enable detail expansion (Enter)
- ⏳ Route scrolling works (j/k)
- ⏳ All view updates applied

**Phase 4 Complete:**
- ⏳ TUI modules well-organized
- ⏳ CLI commands work with new ships structure

**Phase 5 Complete:**
- ⏳ Server compatible with ships refactor
- ⏳ Dependencies up to date

**Final Success:**
- ✅ All code compiles
- ✅ All tests pass
- ⏳ Manual TUI testing successful
- ⏳ No regressions in existing functionality
- ⏳ Stashes can be safely dropped
