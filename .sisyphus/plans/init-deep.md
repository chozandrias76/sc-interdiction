# Init Deep AGENTS Hierarchy

## TL;DR
> **Summary**: Generate a scoped, non-overwriting AGENTS.md hierarchy (root + selected subdirs) using discovery-based scoring and strict allow/deny lists.
> **Deliverables**: Updated root `AGENTS.md`, new per-dir `AGENTS.md` files for agreed locations, verification evidence.
> **Effort**: Medium
> **Parallel**: YES - 3 waves
> **Critical Path**: Discovery/Scoring → Generate AGENTS files → Review/Dedupe → QA validation

## Context
### Original Request
Run `/init-deep` to generate hierarchical AGENTS.md files (root + complexity-scored subdirectories).

### Interview Summary
- Update mode (no `--create-new`) with strict non-clobbering; preserve existing workflow rules.
- LSP unavailable; use grep-based metrics for scoring.
- Allowlist target dirs: root, `crates/cli`, `crates/intel`, `crates/route-graph`, `crates/api-client`, `crates/sc-data-extractor`, `dbt`.

### Metis Review (gaps addressed)
- Enforce allowlist/denylist and source-only discovery to avoid noisy scoring.
- Treat existing `AGENTS.md` as human-owned; merge rather than overwrite.
- Set size/structure limits and verify via deterministic checks.

## Work Objectives
### Core Objective
Produce a concise, practical AGENTS hierarchy that reflects actual workflows and module boundaries without adding boilerplate.

### Deliverables
- Updated `AGENTS.md` (root) with preserved workflow rules and refreshed structure/commands.
- New/updated AGENTS files in allowlisted subdirs.
- Evidence of deterministic QA checks.

### Definition of Done (verifiable conditions with commands)
- `git diff --name-only` shows only expected AGENTS.md paths.
- All target AGENTS.md files exist.
- No placeholders/TODOs remain.
- No references to excluded dirs.

### Must Have
- Root AGENTS.md retains bd workflow and “Landing the Plane” rules.
- Subdir AGENTS.md focus on local commands, conventions, and gotchas.
- 70/30 mix: “how to work here” vs “what it is”.

### Must NOT Have (guardrails, AI slop patterns, scope boundaries)
- No new files outside allowlisted AGENTS.md paths.
- No generic advice or restatement of parent content in child files.
- No references to `node_modules`, `target`, `dbt/target`, `dbt_packages`.

## Verification Strategy
> ZERO HUMAN INTERVENTION — all verification is agent-executed.
- Test decision: none (docs-only changes)
- QA policy: every task has agent-executed scenarios
- Evidence: `.sisyphus/evidence/task-{N}-{slug}.{ext}`

## Execution Strategy
### Parallel Execution Waves
Wave 1: Discovery + scoring + allowlist confirmation
Wave 2: Generate AGENTS files in parallel
Wave 3: Dedupe/review + QA validation

### Dependency Matrix (full, all tasks)
- T1 → T2
- T2 → T3–T9
- T3–T9 → T10 → T11

### Agent Dispatch Summary (wave → task count → categories)
- Wave 1: 2 tasks → unspecified-high (analysis-heavy)
- Wave 2: 7 tasks → writing (documentation generation)
- Wave 3: 3 tasks → unspecified-high (review/QA) + quick (commit)

## TODOs
> Implementation + Test = ONE task. Never separate.
> EVERY task MUST have: Agent Profile + Parallelization + QA Scenarios.

- [x] 1. Reconfirm allowlist/denylist inputs for discovery

  **What to do**: Re-run discovery listings (existing AGENTS, dir inventory, source-only counts). Declare explicit allowlist and denylist in a short evidence note.
  **Must NOT do**: Expand scope beyond allowlist without updating plan.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: repo-wide analysis and guardrails
  - Skills: `[]` — Reason: no specialized tool needed
  - Omitted: `git-master` — Reason: no git operations required

  **Parallelization**: Can Parallel: NO | Wave 1 | Blocks: T2–T11 | Blocked By: none

  **References** (executor has NO interview context — be exhaustive):
  - Existing AGENTS: `AGENTS.md`
  - Repo structure: `README.md`
  - Scripts conventions: `scripts/README.md`
  - dbt config: `dbt/dbt_project.yml`

  **Acceptance Criteria** (agent-executable only):
  - [ ] Evidence note lists allowlist and denylist inputs.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Capture allowlist/denylist
    Tool: Bash
    Steps: run `find . -type f -name "AGENTS.md"` and `find crates dbt scripts docs -type f | wc -l`
    Expected: evidence note created with explicit allowlist/denylist
    Evidence: .sisyphus/evidence/task-1-allowlist.md

  Scenario: Excludes enforced
    Tool: Bash
    Steps: run `rg -n "node_modules|/target/|dbt_packages|dbt/target" -g"AGENTS.md"` (expect no matches)
    Expected: no matches in existing AGENTS
    Evidence: .sisyphus/evidence/task-1-denylist.txt
  ```

  **Commit**: NO | Message: `docs(agents): set allowlist/denylist` | Files: none

- [x] 2. Score directories and finalize AGENTS locations

  **What to do**: Use file counts and grep-based public symbol/reference counts to score directories; finalize AGENTS locations and record rationale.
  **Must NOT do**: Generate AGENTS.md outside allowlist.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: scoring and decision-making
  - Skills: `[]` — Reason: standard tooling only
  - Omitted: `git-master` — Reason: no git operations required

  **Parallelization**: Can Parallel: NO | Wave 1 | Blocks: T3–T11 | Blocked By: T1

  **References**:
  - Crate boundaries: `Cargo.toml`
  - Crates list: `crates/`
  - dbt structure: `dbt/models/`

  **Acceptance Criteria**:
  - [ ] Evidence note lists final AGENTS locations with scores and reasons.

  **QA Scenarios**:
  ```
  Scenario: Produce scoring table
    Tool: Bash
    Steps: run `find crates -type f | wc -l` and `rg -n "\bpub\s+(struct|enum|fn|trait)\b" crates/*/src -c`
    Expected: scoring evidence recorded
    Evidence: .sisyphus/evidence/task-2-scoring.md

  Scenario: Confirm allowlist matches plan
    Tool: Bash
    Steps: verify allowlist paths exist with `test -d crates/cli` etc.
    Expected: all allowlist directories exist
    Evidence: .sisyphus/evidence/task-2-allowlist.txt
  ```

  **Commit**: NO | Message: `docs(agents): confirm locations` | Files: none

- [x] 3. Update root AGENTS.md (merge, do not clobber)

  **What to do**: Update `AGENTS.md` to include project overview, structure, where-to-look, commands, and notes while preserving existing bd workflow + landing-the-plane rules verbatim.
  **Must NOT do**: Remove or weaken existing workflow rules; add boilerplate advice.

  **Recommended Agent Profile**:
  - Category: `writing` — Reason: documentation synthesis
  - Skills: `[]` — Reason: no specialized tool needed
  - Omitted: `git-master` — Reason: no git operations required

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T10 | Blocked By: T2

  **References**:
  - Existing rules: `AGENTS.md`
  - Project overview: `README.md`
  - Commands: `Makefile`
  - Workflow: `docs/CONTRIBUTING.md`

  **Acceptance Criteria**:
  - [ ] Root AGENTS.md retains bd workflow and push rules.
  - [ ] Contains commands section with `make` and `cargo` usage.

  **QA Scenarios**:
  ```
  Scenario: Preserve workflow rules
    Tool: Bash
    Steps: run `rg -n "bd ready|Landing the Plane|git push" AGENTS.md`
    Expected: required workflow rules present
    Evidence: .sisyphus/evidence/task-3-root-verify.txt

  Scenario: Structure present
    Tool: Bash
    Steps: run `rg -n "## OVERVIEW|## STRUCTURE|## COMMANDS" AGENTS.md`
    Expected: core sections present
    Evidence: .sisyphus/evidence/task-3-root-sections.txt
  ```

  **Commit**: NO | Message: `docs(agents): refresh root guidance` | Files: `AGENTS.md`

- [x] 4. Create `crates/cli/AGENTS.md`

  **What to do**: Add CLI/TUI-specific guidance: entrypoint, TUI structure, snapshot tests, and local commands.
  **Must NOT do**: Repeat root rules or non-CLI crate guidance.

  **Recommended Agent Profile**:
  - Category: `writing` — Reason: concise module-specific guidance
  - Skills: `[]` — Reason: no specialized tool needed
  - Omitted: `git-master` — Reason: no git operations required

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T10 | Blocked By: T2

  **References**:
  - CLI entrypoint: `crates/cli/src/main.rs`
  - TUI structure: `crates/cli/src/tui/`
  - Snapshot tests: `crates/cli/src/tui/snapshots/`
  - Package name: `crates/cli/Cargo.toml`

  **Acceptance Criteria**:
  - [ ] Includes `cargo test -p sc-interdiction` and `cargo run -p sc-interdiction -- --help`.
  - [ ] Mentions TUI snapshots and insta usage.

  **QA Scenarios**:
  ```
  Scenario: File created with commands
    Tool: Bash
    Steps: run `test -f crates/cli/AGENTS.md && rg -n "sc-interdiction" crates/cli/AGENTS.md`
    Expected: file exists and includes package command
    Evidence: .sisyphus/evidence/task-4-cli.txt

  Scenario: Snapshot note present
    Tool: Bash
    Steps: run `rg -n "insta|snapshot" crates/cli/AGENTS.md`
    Expected: snapshot testing guidance present
    Evidence: .sisyphus/evidence/task-4-cli-snapshots.txt
  ```

  **Commit**: NO | Message: `docs(agents): add cli guidance` | Files: `crates/cli/AGENTS.md`

- [x] 5. Create `crates/intel/AGENTS.md`

  **What to do**: Add intel crate guidance: targets, ships, wikelo modules, tests, mockito usage.
  **Must NOT do**: Duplicate root instructions or dbt guidance.

  **Recommended Agent Profile**:
  - Category: `writing` — Reason: module-specific documentation
  - Skills: `[]` — Reason: no specialized tool needed
  - Omitted: `git-master` — Reason: no git operations required

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T10 | Blocked By: T2

  **References**:
  - Intel modules: `crates/intel/src/targets.rs`, `crates/intel/src/ships/`, `crates/intel/src/wikelo/`
  - Tests: `crates/intel/tests/`, `crates/intel/src/targets_tests.rs`
  - Package name: `crates/intel/Cargo.toml`

  **Acceptance Criteria**:
  - [ ] Includes `cargo test -p intel` and notes mockito usage.

  **QA Scenarios**:
  ```
  Scenario: File created with test commands
    Tool: Bash
    Steps: run `test -f crates/intel/AGENTS.md && rg -n "cargo test -p intel" crates/intel/AGENTS.md`
    Expected: file exists and test command present
    Evidence: .sisyphus/evidence/task-5-intel.txt

  Scenario: Mockito guidance present
    Tool: Bash
    Steps: run `rg -n "mockito" crates/intel/AGENTS.md`
    Expected: mockito guidance included
    Evidence: .sisyphus/evidence/task-5-intel-mockito.txt
  ```

  **Commit**: NO | Message: `docs(agents): add intel guidance` | Files: `crates/intel/AGENTS.md`

- [x] 6. Create `crates/route-graph/AGENTS.md`

  **What to do**: Add route-graph guidance: graph algorithms, spatial/fuel modules, and local test commands.
  **Must NOT do**: Describe dbt or CLI behavior.

  **Recommended Agent Profile**:
  - Category: `writing` — Reason: module-specific documentation
  - Skills: `[]` — Reason: no specialized tool needed
  - Omitted: `git-master` — Reason: no git operations required

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T10 | Blocked By: T2

  **References**:
  - Core modules: `crates/route-graph/src/graph.rs`, `crates/route-graph/src/spatial.rs`, `crates/route-graph/src/fuel.rs`
  - Package name: `crates/route-graph/Cargo.toml`

  **Acceptance Criteria**:
  - [ ] Includes `cargo test -p route-graph`.

  **QA Scenarios**:
  ```
  Scenario: File created with test command
    Tool: Bash
    Steps: run `test -f crates/route-graph/AGENTS.md && rg -n "cargo test -p route-graph" crates/route-graph/AGENTS.md`
    Expected: file exists and test command present
    Evidence: .sisyphus/evidence/task-6-route-graph.txt

  Scenario: Module notes present
    Tool: Bash
    Steps: run `rg -n "spatial|fuel|graph" crates/route-graph/AGENTS.md`
    Expected: module guidance present
    Evidence: .sisyphus/evidence/task-6-route-graph-modules.txt
  ```

  **Commit**: NO | Message: `docs(agents): add route-graph guidance` | Files: `crates/route-graph/AGENTS.md`

- [x] 7. Create `crates/api-client/AGENTS.md`

  **What to do**: Add api-client guidance: external APIs (UEX, SC API, FleetYards), mockito tests, cache behavior.
  **Must NOT do**: Repeat route-graph or intel guidance.

  **Recommended Agent Profile**:
  - Category: `writing` — Reason: module-specific documentation
  - Skills: `[]` — Reason: no specialized tool needed
  - Omitted: `git-master` — Reason: no git operations required

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T10 | Blocked By: T2

  **References**:
  - API modules: `crates/api-client/src/uex.rs`, `crates/api-client/src/sc_api.rs`, `crates/api-client/src/fleetyards.rs`
  - Tests: `crates/api-client/src/*_tests.rs`
  - Package name: `crates/api-client/Cargo.toml`

  **Acceptance Criteria**:
  - [ ] Includes `cargo test -p api-client` and mockito mention.

  **QA Scenarios**:
  ```
  Scenario: File created with test command
    Tool: Bash
    Steps: run `test -f crates/api-client/AGENTS.md && rg -n "cargo test -p api-client" crates/api-client/AGENTS.md`
    Expected: file exists and test command present
    Evidence: .sisyphus/evidence/task-7-api-client.txt

  Scenario: API guidance present
    Tool: Bash
    Steps: run `rg -n "UEX|FleetYards|Star Citizen" crates/api-client/AGENTS.md`
    Expected: API guidance present
    Evidence: .sisyphus/evidence/task-7-api-client-apis.txt
  ```

  **Commit**: NO | Message: `docs(agents): add api-client guidance` | Files: `crates/api-client/AGENTS.md`

- [x] 8. Create `crates/sc-data-extractor/AGENTS.md`

  **What to do**: Add sc-data-extractor guidance: parsers/database modules, build.rs schema generation, SCLogistics data expectations.
  **Must NOT do**: Document sc-logistics-importer CLI (separate crate).

  **Recommended Agent Profile**:
  - Category: `writing` — Reason: module-specific documentation
  - Skills: `[]` — Reason: no specialized tool needed
  - Omitted: `git-master` — Reason: no git operations required

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T10 | Blocked By: T2

  **References**:
  - Build script: `crates/sc-data-extractor/build.rs`
  - Parsers: `crates/sc-data-extractor/src/parsers/`
  - Database: `crates/sc-data-extractor/src/database/`
  - Package name: `crates/sc-data-extractor/Cargo.toml`
  - README: `crates/sc-data-extractor/README.md`

  **Acceptance Criteria**:
  - [ ] Includes `cargo test -p sc-data-extractor` and notes build.rs schema generation.

  **QA Scenarios**:
  ```
  Scenario: File created with test command
    Tool: Bash
    Steps: run `test -f crates/sc-data-extractor/AGENTS.md && rg -n "cargo test -p sc-data-extractor" crates/sc-data-extractor/AGENTS.md`
    Expected: file exists and test command present
    Evidence: .sisyphus/evidence/task-8-sc-data-extractor.txt

  Scenario: build.rs note present
    Tool: Bash
    Steps: run `rg -n "build.rs|schema" crates/sc-data-extractor/AGENTS.md`
    Expected: schema generation guidance present
    Evidence: .sisyphus/evidence/task-8-sc-data-extractor-build.txt
  ```

  **Commit**: NO | Message: `docs(agents): add sc-data-extractor guidance` | Files: `crates/sc-data-extractor/AGENTS.md`

- [x] 9. Create `dbt/AGENTS.md`

  **What to do**: Add dbt guidance: medallion layers, commands, docker/dbt usage, target path exclusions.
  **Must NOT do**: Duplicate Rust crate guidance.

  **Recommended Agent Profile**:
  - Category: `writing` — Reason: dbt documentation
  - Skills: `[]` — Reason: no specialized tool needed
  - Omitted: `git-master` — Reason: no git operations required

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T10 | Blocked By: T2

  **References**:
  - dbt config: `dbt/dbt_project.yml`
  - dbt models: `dbt/models/`
  - dbt seeds: `dbt/seeds/`
  - Docker: `docker-compose.yml`

  **Acceptance Criteria**:
  - [ ] Includes `dbt run`, `dbt test`, and notes `dbt/target` + `dbt_packages` are excluded.

  **QA Scenarios**:
  ```
  Scenario: File created with dbt commands
    Tool: Bash
    Steps: run `test -f dbt/AGENTS.md && rg -n "dbt run|dbt test" dbt/AGENTS.md`
    Expected: file exists and dbt commands present
    Evidence: .sisyphus/evidence/task-9-dbt.txt

  Scenario: Target exclusions present
    Tool: Bash
    Steps: run `rg -n "dbt/target|dbt_packages" dbt/AGENTS.md`
    Expected: exclusion guidance present
    Evidence: .sisyphus/evidence/task-9-dbt-excludes.txt
  ```

  **Commit**: NO | Message: `docs(agents): add dbt guidance` | Files: `dbt/AGENTS.md`

- [x] 10. Deduplicate and trim AGENTS hierarchy

  **What to do**: Review all generated AGENTS files; remove parent-child duplication, enforce ≤120 lines, and ensure local-only content.
  **Must NOT do**: Remove required commands or workflow rules.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: cross-file review
  - Skills: `[]` — Reason: standard tooling only
  - Omitted: `git-master` — Reason: no git operations required

  **Parallelization**: Can Parallel: NO | Wave 3 | Blocks: T11 | Blocked By: T3–T9

  **References**:
  - Root: `AGENTS.md`
  - Subdirs: `crates/cli/AGENTS.md`, `crates/intel/AGENTS.md`, `crates/route-graph/AGENTS.md`, `crates/api-client/AGENTS.md`, `crates/sc-data-extractor/AGENTS.md`, `dbt/AGENTS.md`

  **Acceptance Criteria**:
  - [ ] No duplicate sections between parent and child.
  - [ ] Each file ≤120 lines.

  **QA Scenarios**:
  ```
  Scenario: Check line counts
    Tool: Bash
    Steps: run `wc -l AGENTS.md crates/**/AGENTS.md dbt/AGENTS.md`
    Expected: all files ≤120 lines
    Evidence: .sisyphus/evidence/task-10-linecounts.txt

  Scenario: Spot duplication
    Tool: Bash
    Steps: run `rg -n "Landing the Plane" crates/**/AGENTS.md dbt/AGENTS.md`
    Expected: no child file repeats root workflow section
    Evidence: .sisyphus/evidence/task-10-dupcheck.txt
  ```

  **Commit**: NO | Message: `docs(agents): dedupe hierarchy` | Files: allowlist AGENTS.md

- [x] 11. QA validation checks

  **What to do**: Run deterministic checks to ensure only expected files changed, no placeholders, and no excluded-dir references.
  **Must NOT do**: Skip checks or accept warnings.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: QA validation
  - Skills: `[]` — Reason: standard tooling only
  - Omitted: `git-master` — Reason: no git operations required

  **Parallelization**: Can Parallel: NO | Wave 3 | Blocks: T12 | Blocked By: T10

  **References**:
  - Allowlist paths: `AGENTS.md`, `crates/cli/AGENTS.md`, `crates/intel/AGENTS.md`, `crates/route-graph/AGENTS.md`, `crates/api-client/AGENTS.md`, `crates/sc-data-extractor/AGENTS.md`, `dbt/AGENTS.md`

  **Acceptance Criteria**:
  - [ ] `git diff --name-only` shows only allowlisted files.
  - [ ] `rg -n "TODO|TBD|FIXME"` finds no matches in AGENTS files.

  **QA Scenarios**:
  ```
  Scenario: Only expected files changed
    Tool: Bash
    Steps: run `git diff --name-only`
    Expected: only allowlisted AGENTS.md paths
    Evidence: .sisyphus/evidence/task-11-gitdiff.txt

  Scenario: No placeholders
    Tool: Bash
    Steps: run `rg -n "TODO|TBD|FIXME|PLACEHOLDER" AGENTS.md crates/**/AGENTS.md dbt/AGENTS.md`
    Expected: no matches
    Evidence: .sisyphus/evidence/task-11-placeholders.txt
  ```

  **Commit**: NO | Message: `docs(agents): qa validation` | Files: none

- [x] 12. Commit AGENTS hierarchy

  **What to do**: Commit allowlisted AGENTS.md changes with conventional commit message.
  **Must NOT do**: Include any other files.

  **Recommended Agent Profile**:
  - Category: `quick` — Reason: simple git operation
  - Skills: [`git-master`] — Reason: safe atomic commit
  - Omitted: []

  **Parallelization**: Can Parallel: NO | Wave 3 | Blocks: none | Blocked By: T11

  **References**:
  - Commit conventions: `docs/CONTRIBUTING.md`

  **Acceptance Criteria**:
  - [ ] One commit created with message `docs(agents): generate hierarchy`.

  **QA Scenarios**:
  ```
  Scenario: Commit created
    Tool: Bash
    Steps: run `git log -1 --format=%s`
    Expected: subject is `docs(agents): generate hierarchy`
    Evidence: .sisyphus/evidence/task-12-commit.txt

  Scenario: Clean status
    Tool: Bash
    Steps: run `git status -sb`
    Expected: working tree clean (or only expected untracked evidence files)
    Evidence: .sisyphus/evidence/task-12-status.txt
  ```

  **Commit**: YES | Message: `docs(agents): generate hierarchy` | Files: allowlist AGENTS.md

## Final Verification Wave (4 parallel agents, ALL must APPROVE)
- [x] F1. Plan Compliance Audit — oracle
- [x] F2. Code Quality Review — unspecified-high
- [x] F3. Real Manual QA — unspecified-high (+ playwright if UI)
- [x] F4. Scope Fidelity Check — deep

## Commit Strategy
- Commit: `docs(agents): generate hierarchy` after QA validation.
- Files: only AGENTS.md paths in allowlist.

## Success Criteria
- All allowlisted AGENTS.md files exist and are ≤120 lines each.
- Root AGENTS.md retains bd workflow and mandatory push rules.
- No overlap/duplication between parent and child AGENTS.
