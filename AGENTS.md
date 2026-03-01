# Agent Instructions

## Overview

Rust workspace -- Star Citizen interdiction planner. Analyzes UEX trade data,
finds quantum chokepoints, predicts hauler routes across Stanton system.

## Structure

```
crates/
  api-client/            HTTP clients (UEX, SC API)
  route-graph/           Graph structures + pathfinding
  intel/                 Target analysis + prediction
  server/                Axum REST API
  cli/                   Clap CLI + Ratatui TUI dashboard
  sc-data-extractor/     Diesel ORM, Postgres migrations
  sc-logistics-importer/ SCLogistics data import
  data-viewer/           TUI data browser
  wikelo-data/           Wiki data models
  dataforge-explorer/    DataForge file reader
  scunpacked-explorer/   SCUnpacked data browser
dbt/                     dbt transforms (silver/gold layers)
scripts/                 Dev setup, hooks, quality checks
docs/                    DATA_SOURCES, BUILD_CONFIG, RELEASE_PROCESS
```

## Where to Look

| Domain                 | Path                                      |
|------------------------|-------------------------------------------|
| Trade route logic      | `crates/route-graph/`                     |
| Interdiction analysis  | `crates/intel/`                           |
| REST API endpoints     | `crates/server/`                          |
| CLI commands + TUI     | `crates/cli/`                             |
| DB schema / migrations | `crates/sc-data-extractor/`               |
| Data import pipeline   | `crates/sc-logistics-importer/` + `dbt/`  |

## Commands

```bash
make build        # Debug build (target: /tmp/cargo-target-sc-interdiction)
make test         # All tests
make clippy       # Linter
make fmt          # Format
make dev          # fmt + clippy + test
make db-setup     # Docker Postgres + migrate + import + dbt
make data-viewer  # TUI data browser
cargo quality     # Pre-commit quality checks
```

## Issue Tracking (bd)

This project uses **bd** (beads) for issue tracking. Run `bd onboard` to get started.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --status in_progress  # Claim work
bd close <id>         # Complete work
bd dolt pull          # Sync beads from remote (bd sync deprecated)
bd dolt push          # Sync beads to remote (bd sync deprecated)
```

## Repo Safety (Hard Block)

- **Hard block**: Do not proceed with any task unless you are inside an initialized git repository and inside a git worktree.
- **Hard block check**: `git rev-parse --is-inside-work-tree` must return `true` before doing any work.
- If the check fails, stop immediately and set up/select a valid git repo/worktree before continuing.

### bd/dolt Artifacts

- Running `bd` and `bd dolt` commands can create local tooling artifacts such as `.dolt/` and `.beads/`.
- Treat `.dolt/` and `.beads/` as local state by default: keep them untracked and do not commit them unless the user explicitly asks.
- If these artifacts are created unintentionally during agent work, clean them before finishing (for example with targeted `git clean`), while preserving intentional project changes.

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd dolt pull  # bd sync deprecated
   bd dolt push  # bd sync deprecated
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds

## Notes

- Conventional Commits required (feat/fix/docs/refactor/test/chore)
- Pre-commit hooks: clippy, tests, fmt, commit size <500 lines
- Code limits: 500 lines/file, 100 lines/fn, complexity <=15
- Build dir: `/tmp/cargo-target-sc-interdiction` (via direnv)
- Branches: main (releases), develop (integration), feature/fix/chore/*
