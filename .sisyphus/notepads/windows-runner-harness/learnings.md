
- Added `.github/workflows/windows-harness.yml` with workflow_dispatch mode input, self-hosted Windows runner labels, matrix commands including llvm-cov, LLVM install via Chocolatey, and artifact uploads.
- Documented Windows Harness prerequisites and dispatch command in docs/RELEASE_WORKFLOW.md.
- Windows runners need Git Bash available for dtolnay/rust-toolchain; prepend Git Bash to PATH before toolchain install.
- Ensure Cargo bin ($USERPROFILE\.cargo\bin) is added to PATH before running cargo commands in pwsh.
- Documented bd sync deprecation with bd dolt pull/push in AGENTS.md.

## F1. Plan Compliance Audit — oracle

### Evidence Checklist

| Acceptance Criteria | File | Line(s) | Evidence |
|---------------------|------|---------|----------|
| workflow_dispatch input | `.github/workflows/windows-harness.yml` | 4-14 | `workflow_dispatch:` with mode input (full/quick) |
| Windows runner labels | `.github/workflows/windows-harness.yml` | 21-24 | `runs-on: [self-hosted, Windows, X64]` |
| llvm-cov coverage | `.github/workflows/windows-harness.yml` | 52 | `cargo llvm-cov --all-features --workspace --lcov` |
| LLVM via Chocolatey | `.github/workflows/windows-harness.yml` | 114 | `choco install llvm -y --no-progress` |
| cargo-llvm-cov install | `.github/workflows/windows-harness.yml` | 96 | `cargo install cargo-llvm-cov --locked` |
| cargo-audit install | `.github/workflows/windows-harness.yml` | 102 | `cargo install cargo-audit --locked` |
| Git Bash PATH fix | `.github/workflows/windows-harness.yml` | 60-75 | Add Git bin/usr/bin to PATH before toolchain |
| Cargo bin PATH fix | `.github/workflows/windows-runner-harness` | 82-90 | Add $env:USERPROFILE/.cargo/bin to PATH |
| Log artifacts | `.github/workflows/windows-harness.yml` | 228-233 | `actions/upload-artifact` for each matrix job |
| Coverage artifact | `.github/workflows/windows-harness.yml` | 235-240 | Separate lcov artifact upload |
| Docs: Windows Harness | `docs/RELEASE_WORKFLOW.md` | 123-144 | Runner labels, tools, dispatch cmd |
| Evidence files exist | `.sisyphus/evidence/` | — | task-1-workflow.txt, task-1-coverage-config.txt, task-2-docs.txt, task-3-run.txt |

### Verification Commands

```bash
# Verify workflow_dispatch
grep -n "workflow_dispatch:" .github/workflows/windows-harness.yml

# Verify runner labels
grep -n "runs-on:" .github/workflows/windows-harness.yml

# Verify llvm-cov
grep -n "llvm-cov" .github/workflows/windows-harness.yml

# Verify Chocolatey LLVM
grep -n "choco install llvm" .github/workflows/windows-harness.yml

# Verify docs section
grep -n "Windows Harness" docs/RELEASE_WORKFLOW.md

# Verify successful run (from evidence)
# task-3-run.txt: run_id=22526394056, conclusion=success
```

### Known Issues (from issues.md)
- gh workflow run returned 404 until workflow pushed to remote
- bash temp path issue with dtolnay/rust-toolchain (fixed with Git Bash PATH)
- cargo not in PATH in pwsh (fixed with Cargo bin PATH step)
- cargo-audit install missing (added step)
- Log capture needed cmd.exe vs pwsh (implemented)

### Compliance Status
✅ All acceptance criteria from oracle plan have been implemented and verified.

## F3. Real Manual QA — GitHub CLI Workflow Commands

### gh workflow run
**Documentation**: https://cli.github.com/manual/gh_workflow_run

Triggers a workflow via `workflow_dispatch` event. Requires workflow to have `on.workflow_dispatch` trigger.

**Key Flags**:
- `-f, --raw-field <key=value>` — Add string parameter (simple values)
- `-F, --field <key=value>` — Add string parameter with @ syntax support
- `--json` — Read workflow inputs as JSON via STDIN
- `-r, --ref <branch>` — Branch/tag containing workflow file

**Common Examples**:
```bash
# Interactive mode — prompts for workflow and inputs
gh workflow run

# Run specific workflow by name
gh workflow run windows-harness.yml

# Run with inputs (flags)
gh workflow run windows-harness.yml -f mode=full

# Run with JSON input via stdin
echo '{"mode": "full"}' | gh workflow run windows-harness.yml --json

# Run on specific branch
gh workflow run windows-harness.yml -r feature/xyz
```

**Typical Errors**:
- `404 Not Found` — Workflow file not in default branch or not yet pushed to remote
- Missing inputs — Workflow requires inputs but none provided

### gh run watch
**Documentation**: https://cli.github.com/manual/gh_run_watch

Watch a workflow run until completion, showing progress.

**Key Flags**:
- `--compact` — Show only relevant/failed steps
- `--exit-status` — Exit non-zero if run fails
- `-i, --interval <seconds>` — Refresh interval (default: 3)

**Common Examples**:
```bash
# Watch most recent run (interactive selection)
gh run watch

# Watch specific run ID
gh run watch 22526394056

# Watch in compact mode
gh run watch 22526394056 --compact

# Exit with failure status if run fails
gh run watch 22526394056 --exit-status
```

### gh run view
**Documentation**: https://cli.github.com/manual/gh_run_view

View summary of a workflow run.

**Key Flags**:
- `-a, --attempt <n>` — View specific attempt number
- `--exit-status` — Exit non-zero if run failed
- `-j, --job <id>` — View specific job
- `--log` — View full log for run or job
- `--log-failed` — View logs for failed steps only
- `-w, --web` — Open run in browser

**Common Examples**:
```bash
# Interactive selection
gh run view

# View specific run
gh run view 22526394056

# View job details
gh run view 22526394056 --job 456789

# View failed step logs
gh run view 22526394056 --log-failed

# Exit non-zero if failed
gh run view 22526394056 --exit-status && echo "passed"
```

### Workflow Run Output (v2.87.0+)
As of February 2026, `gh workflow run` returns the workflow run URL when triggered:
- https://github.blog/changelog/2026-02-19-workflow-dispatch-api-now-returns-run-ids

This allows chaining:
```bash
RUN_URL=$(gh workflow run windows-harness.yml -f mode=full --json | jq -r '.url')
gh run watch $(echo $RUN_URL | grep -oP '\d+$')
```

### Manual QA Workflow
```bash
# 1. List workflows to find name
gh workflow list

# 2. Trigger workflow with inputs
gh workflow run windows-harness.yml -f mode=full

# 3. Get run ID from output (or use most recent)
gh run list --branch main --status success | head -5

# 4. Watch run progress
gh run watch <run-id>

# 5. View results
gh run view <run-id> --log-failed
```

### Important Notes
- Workflow must be in default branch for `gh workflow run` to find it
- Workflow must have `on.workflow_dispatch` trigger defined
- 404 errors occur if workflow hasn't been pushed to remote yet
- Fine-grained PATs don't support `checks:read` for `gh run watch`


#XZ|- Research: Windows runner labels and cargo-llvm-cov
## Self-hosted Windows Runner Labels
- **Official Docs**: https://docs.github.com/actions/hosting-your-own-runners/using-labels-with-self-hosted-runners
- **Using in Workflows**: https://docs.github.com/en/actions/how-tos/manage-runners/self-hosted-runners/use-in-a-workflow
- Default labels: `self-hosted`, `windows`, `x64` (or ARM/ARM64)
- Example: `runs-on: [self-hosted, windows, x64]`
- Labels are case-insensitive and operate cumulatively

## cargo-llvm-cov Documentation
- **Repository**: https://github.com/taiki-e/cargo-llvm-cov
- **Latest version**: 0.8.4 (2026-02-06)
- **Windows Support**:
  - Works: `{x86_64,i686}-pc-windows-msvc`, `{x86_64,aarch64}-pc-windows-gnullvm`
  - Does NOT work (as of 2025-12-30): `x86_64-pc-windows-gnu`, `aarch64-pc-windows-msvc`
- **Required component**: `llvm-tools-preview` via rustup
- **Installation**: Use `taiki-e/install-action@cargo-llvm-cov` in GitHub Actions
- **LCOV output**: `cargo llvm-cov --lcov --output-path coverage/lcov.info`

## Example Workflows (GitHub Actions)
1. **pola-rs/polars** - https://github.com/pola-rs/polars/blob/main/.github/workflows/test-coverage.yml
2. **delta-io/delta-rs** - https://github.com/delta-io/delta-rs/blob/main/.github/workflows/integration.yml
3. **qdrant/qdrant** - https://github.com/qdrant/qdrant/blob/master/.github/workflows/coverage.yml
4. **tremor-rs/tremor-runtime** - https://github.com/tremor-rs/tremor-runtime/blob/main/.github/workflows/tests.yaml

## Windows-Specific Gotchas
1. **Shell selection**: CMD more reliable than PowerShell for log capture in cargo-llvm-cov
2. **dtolnay/rust-toolchain**: Uses bash internally; requires Git Bash on PATH before toolchain install
3. **Cargo PATH**: Must explicitly add `$USERPROFILE\.cargo\bin` to PATH in pwsh
4. **Temp script path**: Windows temp paths (C:\Users\...) cause bash failures; prepend Git Bash PATH
5. **Log capture**: Use cmd.exe instead of pwsh for reliable output capture when cargo commands fail

## Recommended Workflow Snippet
```yaml
- name: Setup Git Bash PATH
  run: |
    echo "$ProgramFiles\Git\bin" | Out-File -FilePath $env:GITHUB_PATH -Encoding utf8 -Append
  shell: pwsh

- uses: dtolnay/rust-toolchain@stable
  with:
    components: llvm-tools-preview

- uses: taiki-e/install-action@cargo-llvm-cov

- name: Generate coverage
  run: cargo llvm-cov --lcov --output-path lcov.info
  shell: cmd
```
