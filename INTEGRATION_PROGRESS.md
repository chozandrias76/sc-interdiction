# Stash Integration Progress Report

## Completed: Phase 1 - Foundation ✅

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

**Status:** Compiles cleanly, ready for testing

---

### 1.2 API Client UEX Enhancements ⏭️ SKIPPED
**Reason:** Changes are primarily for Construction Materials (CM) pricing which is part of the salvage feature we're excluding. The Commodity struct changes (bool -> i32 for API compatibility) can be added later if needed.

**What Was Skipped:**
- `get_cm_sale_stations()` method
- `get_construction_materials_prices()` method with disk caching
- `CachedCMPrices` struct
- `CMSaleStation` struct
- Commodity struct field updates

**Decision:** Skip for now, revisit if API compatibility issues arise

---

### 1.3 Intel Targets Updates ✅ (Commit: 545c28b)
**Changes Applied:**
- ✅ Added `origin_system` and `destination_system` fields to `HotRoute`
- ✅ Updated `HotRoute` construction to extract system information
- ✅ Enables cross-system route detection in trade analysis

**Files Modified:**
- `crates/intel/src/targets.rs`

**Status:** Compiles cleanly

---

## Next: Phase 2 - TUI Core

### 2.1 App State Updates (Not Started)
**Required Changes:**
- Add `detail_expanded: bool` field to `App` struct in `crates/cli/src/tui/app.rs`
- Initialize to `false` in constructor
- Enable toggling between compact and expanded detail views

**Files to Modify:**
- `crates/cli/src/tui/app.rs`

**Estimated Complexity:** LOW (simple field addition)

---

### 2.2 Hotspot Data Enhancements (Not Started)
**Required Changes from Stash:**
- Enhanced hotspot loading logic
- Better filtering mechanisms  
- System-aware hotspot handling
- ~143 lines of improvements

**Files to Modify:**
- `crates/cli/src/tui/data/hotspots.rs`

**Estimated Complexity:** MEDIUM (logic enhancements)

---

## Remaining Phases

### Phase 3: TUI Views (6 tasks)
- Map view enhancements (+160 lines - armistice warnings, cross-system indicators, expanded details)
- Routes/targets/help view updates
- Key handlers (Enter for detail toggle, h/l navigation)
- Navigation improvements

### Phase 4: CLI & Module Organization (2 tasks)
- TUI module reorganization
- CLI main enhancements (⚠️ HIGH CONFLICT RISK with ships refactor)

### Phase 5: Infrastructure (3 tasks)
- Server updates (⚠️ MODERATE CONFLICT RISK)
- Cargo/dependency updates (⚠️ HIGH CONFLICT RISK - lock file)
- Ship tests updates

### Phase 6: Cleanup
- Drop stash@{2} (trivial .claude/settings change)
- Drop stash@{0} and stash@{1} after extraction complete

---

## Current Branch Status

**Branch:** `integrate/stashed-features`
**Base:** `refactor/ships-module-reorganization` (commit 13d16b9)
**Commits:**
1. 062d206 - feat(route-graph): add cross-system route tracking
2. 545c28b - feat(intel): add system tracking to HotRoute

**Compilation Status:** ✅ All changes compile cleanly
**Tests:** Not run yet (should run after completing TUI phases)

---

## Integration Strategy Going Forward

### Immediate Next Steps:
1. **Phase 2.1:** Add `detail_expanded` to App struct (5 min)
2. **Phase 2.2:** Apply hotspot data enhancements (15 min)
3. **Phase 3:** Apply TUI view enhancements in order (30 min)
4. **Test:** Run full test suite after Phase 3
5. **Phase 4-5:** Carefully handle conflicts with ships refactor

### Risk Mitigation:
- Commit after each major change
- Test compilation frequently
- Skip anything that conflicts heavily with ships refactor
- Document skipped features for potential future integration

---

## Summary

**Phase 1 Progress:** 2 of 3 tasks complete (66%)
- ✅ Spatial enhancements
- ⏭️ UEX API (skipped - salvage feature)
- ✅ Intel targets

**Overall Progress:** 2 of ~20 tasks (10%)

**Time Spent:** ~30 minutes
**Estimated Remaining:** ~2 hours for full integration

**Blockers:** None currently
**Issues:** None - all changes compile and integrate cleanly so far

---

## Files Changed Summary

### Modified (3 files):
1. `crates/route-graph/src/spatial.rs` - Cross-system tracking
2. `crates/intel/src/targets.rs` - System fields in HotRoute
3. `crates/cli/src/tui/data/hotspots.rs` - System extraction helper

### Created (1 file):
1. `STASH_INTEGRATION_PLAN.md` - Integration guide

### Next to Modify:
1. `crates/cli/src/tui/app.rs` - App state
2. `crates/cli/src/tui/data/hotspots.rs` - More enhancements
3. `crates/cli/src/tui/views/map.rs` - Map view enhancements

