# Stash Integration Plan

⚠️ **This file is OUTDATED** ⚠️

This was the original planning document for integrating stashed features.

**Current status:** See [INTEGRATION_PROGRESS.md](INTEGRATION_PROGRESS.md) for up-to-date progress.

---

## What Was Planned

This document outlined the integration of 27 files from stash@{0} and stash@{1}, excluding mining/salvage features.

**Plan organized into 6 phases:**
1. Foundation (route-graph, intel, API client)
2. TUI Core (app state, hotspot data)
3. TUI Views (map, routes, targets, key handlers)
4. CLI & Module Organization
5. Infrastructure (server, dependencies)
6. Cleanup

---

## Current Status (as of 2026-01-04)

**✅ COMPLETED:**
- Phase 1: Foundation (mostly complete, API client skipped)
- Phase 2: TUI Core (complete)
- Phase 3.1: Map View Enhancements (complete)

**📊 Progress:** 45% complete (6 of ~16 tasks)

**📝 See:** [INTEGRATION_PROGRESS.md](INTEGRATION_PROGRESS.md) for:
- Detailed completion status
- Remaining work breakdown
- Time estimates
- Risk assessments
- Testing status

---

## Commits Created

1. 062d206 - feat(route-graph): add cross-system route tracking
2. 545c28b - feat(intel): add system tracking to HotRoute
3. 3116fa0 - docs: add integration progress report
4. f1cffdc - feat(tui): add detail_expanded flag to App state
5. 9b1eaa1 - feat(tui): enhance hotspot data loading and filtering
6. e85da29 - feat(tui): enhance map view with expanded details and warnings
7. 7803ffd - docs: update integration progress with Phase 3.1 completion

**Branch:** `integrate/stashed-features`

---

## What's Left

See INTEGRATION_PROGRESS.md for detailed breakdown. Summary:

**High Priority (~45 min):**
- Key handlers
- View updates
- Navigation improvements

**Medium Priority (~60 min):**
- Module reorganization
- CLI enhancements

**Low Priority (~30 min):**
- Server updates
- Dependencies
- Cleanup

---

*This file preserved for historical reference. Last updated: 2026-01-04*
