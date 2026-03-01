# Windows Self-Hosted Test Harness

## TL;DR
> **Summary**: Add a manual Windows self-hosted workflow that runs full CI parity (fmt/clippy/test/build/audit) plus Windows-compatible coverage using cargo-llvm-cov, and document runner prerequisites/labels.
> **Deliverables**: New `.github/workflows/windows-harness.yml`, docs update with runner setup, successful manual run evidence.
> **Effort**: Short
> **Parallel**: YES - 2 waves
> **Critical Path**: Workflow file → runner docs → manual run verification

## Context
### Original Request
"set up a testing harness that uses that on windows"

### Interview Summary
- Use patterns from `~/projects/mc-config` and `~/projects/krek`.
- Manual-only trigger.
- Runner labels: `[self-hosted, Windows, X64]`.
- Coverage required on Windows using an alternative to tarpaulin.

### Metis Review (gaps addressed)
- Metis unavailable (timeout). Proceeded with conservative guardrails and explicit acceptance criteria.

## Work Objectives
### Core Objective
Provide a Windows self-hosted test harness with full CI parity and Windows-compatible coverage, without impacting existing Linux CI.

### Deliverables
- `.github/workflows/windows-harness.yml` (manual dispatch, matrix, logs/artifacts)
- Runner documentation update in `docs/RELEASE_WORKFLOW.md`
- Evidence of a successful workflow run

### Definition of Done (verifiable conditions with commands)
- `gh workflow run windows-harness.yml -f mode=full` succeeds on the Windows runner
- Logs/artifacts uploaded for each matrix command
- Coverage artifact produced via cargo-llvm-cov

### Must Have
- Manual-only trigger (`workflow_dispatch`)
- `runs-on: [self-hosted, Windows, X64]`
- Coverage using `cargo llvm-cov` (not tarpaulin)

### Must NOT Have
- No changes to existing Linux CI jobs
- No use of tarpaulin on Windows
- No runner labels beyond those specified unless documented

## Verification Strategy
> ZERO HUMAN INTERVENTION — all verification is agent-executed.
- Test decision: tests-after (manual harness run)
- QA policy: Every task has agent-executed scenarios
- Evidence: .sisyphus/evidence/task-{N}-{slug}.{ext}

## Execution Strategy
### Parallel Execution Waves
Wave 1: Add workflow + docs
Wave 2: Manual dispatch + evidence capture

### Dependency Matrix (full, all tasks)
- Task 1 → Task 3
- Task 2 → Task 3

### Agent Dispatch Summary (wave → task count → categories)
- Wave 1: 2 tasks → quick, writing
- Wave 2: 1 task → quick

## TODOs
> Implementation + Test = ONE task. Never separate.
> EVERY task MUST have: Agent Profile + Parallelization + QA Scenarios.

- [x] 1. Add Windows harness workflow

  **What to do**: Create `.github/workflows/windows-harness.yml` patterned after `mc-config/.github/workflows/windows-harness.yml`. Use `workflow_dispatch` with optional `mode` input (full/quick). Use `runs-on: [self-hosted, Windows, X64]`. Add a matrix with commands: `cargo fmt --all --check`, `cargo clippy --all-targets --all-features`, `cargo test --all-targets --all-features`, `cargo build --release --all-features`, `cargo audit`, and Windows coverage using `cargo llvm-cov --all-features --workspace --lcov --output-path coverage.lcov`. Use `shell: pwsh`. Install Rust toolchain (`dtolnay/rust-toolchain@stable`) with components `rustfmt`, `clippy`, and add `llvm-tools-preview`. Install `cargo-llvm-cov` and ensure LLVM is present (via `choco install llvm -y` if `llvm-config` missing). Upload logs and coverage artifact per matrix entry.
  **Must NOT do**: Do not modify existing CI workflows.

  **Recommended Agent Profile**:
  - Category: `quick` — Reason: single workflow file addition
  - Skills: [] — No special skills needed
  - Omitted: [`git-master`] — commit handled later

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: [3] | Blocked By: []

  **References**:
  - Pattern: `/home/choza/projects/mc-config/.github/workflows/windows-harness.yml`
  - Pattern: `/home/choza/projects/krek/.github/workflows/windows-ci.yml`
  - Current CI: `.github/workflows/ci.yml`

  **Acceptance Criteria**:
  - [ ] Workflow file exists with `workflow_dispatch` and matrix of commands
  - [ ] Uses `runs-on: [self-hosted, Windows, X64]`
  - [ ] Coverage command uses `cargo llvm-cov` producing `coverage.lcov`
  - [ ] Evidence file saved: `.sisyphus/evidence/task-1-workflow.txt`

  **QA Scenarios**:
  ```
  Scenario: Workflow file present and valid
    Tool: Bash
    Steps: rg "windows-harness" .github/workflows/windows-harness.yml
    Expected: file exists and contains workflow_dispatch + matrix commands
    Evidence: .sisyphus/evidence/task-1-workflow.txt

  Scenario: Coverage command configured
    Tool: Bash
    Steps: rg "llvm-cov|coverage.lcov" .github/workflows/windows-harness.yml
    Expected: cargo llvm-cov command and artifact path present
    Evidence: .sisyphus/evidence/task-1-coverage-config.txt
  ```

- [x] 2. Document Windows runner prerequisites

  **What to do**: Update `docs/RELEASE_WORKFLOW.md` with a new section "Windows Harness" describing required runner labels (`self-hosted, Windows, X64`), required tools (Rust, git, PowerShell, LLVM, cargo-llvm-cov), and how to run `gh workflow run windows-harness.yml -f mode=full`. Reference `scripts/watch-runner.sh` for log monitoring on Linux hosts.
  **Must NOT do**: Do not change release process semantics beyond adding documentation.

  **Recommended Agent Profile**:
  - Category: `writing` — Reason: documentation change
  - Skills: [] — No special skills needed
  - Omitted: [`git-master`] — commit handled later

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: [3] | Blocked By: []

  **References**:
  - Doc: `docs/RELEASE_WORKFLOW.md`
  - Runner monitor: `scripts/watch-runner.sh`

  **Acceptance Criteria**:
  - [ ] Docs mention labels, prerequisites, and dispatch command
  - [ ] Evidence file saved: `.sisyphus/evidence/task-2-docs.txt`

  **QA Scenarios**:
  ```
  Scenario: Windows harness docs
    Tool: Bash
    Steps: rg "Windows Harness|windows-harness" docs/RELEASE_WORKFLOW.md
    Expected: section describes labels, prerequisites, and dispatch command
    Evidence: .sisyphus/evidence/task-2-docs.txt
  ```

- [ ] 3. Manual dispatch + verify run

  **What to do**: Trigger the workflow with `gh workflow run windows-harness.yml -f mode=full`. Watch run to completion and save logs/URLs as evidence.
  **Must NOT do**: Do not merge or change workflow logic in this step.

  **Recommended Agent Profile**:
  - Category: `quick` — Reason: command-based verification
  - Skills: [] — No special skills needed
  - Omitted: [`git-master`] — commit handled later

  **Parallelization**: Can Parallel: NO | Wave 2 | Blocks: [] | Blocked By: [1,2]

  **References**:
  - Workflow: `.github/workflows/windows-harness.yml`
  - GH CLI: `gh workflow run`

  **Acceptance Criteria**:
  - [ ] Workflow run completes successfully
  - [ ] Evidence file saved: `.sisyphus/evidence/task-3-run.txt`

  **QA Scenarios**:
  ```
  Scenario: Windows harness run
    Tool: Bash
    Steps: gh workflow run windows-harness.yml -f mode=full && gh run watch --exit-status
    Expected: Run completes with success
    Evidence: .sisyphus/evidence/task-3-run.txt
  ```

## Final Verification Wave (4 parallel agents, ALL must APPROVE)
- [x] F1. Plan Compliance Audit — oracle
- [x] F2. Code Quality Review — unspecified-high
- [x] F3. Real Manual QA — unspecified-high (+ playwright if UI)
- [x] F4. Scope Fidelity Check — deep

## Commit Strategy
1) `ci(windows): add self-hosted harness` — workflow file
2) `docs(ci): document windows harness` — docs update
3) `test(ci): record windows harness run` — evidence files only

## Success Criteria
- Windows harness available via manual dispatch and runs on self-hosted Windows runner
- Coverage produced via cargo-llvm-cov on Windows
- Documentation updated with prerequisites and invocation
