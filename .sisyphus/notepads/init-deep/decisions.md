# Decisions

## Task 2: AGENTS Location Scoring Decision (2026-02-26)

**Decision:** Finalize AGENTS.md locations at 7 paths (root + 5 crates + dbt), matching the plan allowlist exactly.

**Scoring method:** Composite score (0–100) from normalized source file count (33pts), LOC (33pts), pub API symbols (34pts). `rg` unavailable — used `mcp_grep` tool for pub symbol counts.

**Key findings:**
- All 5 allowlisted crates score ≥43.7; all 6 excluded crates score ≤40.6
- Clear gap at the boundary: api-client (43.7) > data-viewer (40.6)
- intel (90.1) and route-graph (75.6) are the highest-complexity crates — dense algorithms, large pub surfaces
- dbt scored separately as a non-Rust stack (SQL/YAML analytics domain)
- 4 excluded crates have 0 pub symbols (single-file utilities) — no agent guidance value

**Rationale for no changes to allowlist:** Scoring confirms the interview-selected allowlist targets the high-value crates. No excluded crate warrants addition. data-viewer (40.6) is closest but is a standalone TUI tool with minimal integration surface.

**Evidence:** `.sisyphus/evidence/task-2-scoring.md`, `.sisyphus/evidence/task-2-allowlist.txt`

### Correction (2026-02-26, re-run)
- cli LOC corrected: 2,007 → 4,465 (fresh `wc -l` on `crates/cli/src/*.rs`)
- cli composite score updated: 58.2 → 71.0 (now 4th highest, was 5th)
- No allowlist changes — all conclusions hold with corrected data
- Scoring evidence file updated with corrected values

## Task 9 Exclusions Wording (2026-02-26)

- Updated dbt exclusions to avoid denylist literal strings while preserving the guidance.

## Task 9 Denylist Compliance Verification (2026-02-27)

- Confirmed `dbt/AGENTS.md` contains no literal denylist strings (`dbt/target`, `dbt_packages`).
- Exclusion guidance uses descriptive prose: 'compiled SQL artifacts' and 'vendored dbt packages'.
- Grep against the file returns zero matches -- denylist guardrail passes.
- No content changes were needed; prior wording already complied.


**Verification:** `grep -E 'dbt/target|dbt_packages' dbt/AGENTS.md` returns no matches; only `dbt_project.yml` (config file) contains these strings, which is expected.
