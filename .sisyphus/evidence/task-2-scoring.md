# Task 2: Directory Scoring & AGENTS Location Finalization

## Date: 2026-02-26 (corrected re-run)

## Methodology

Scored directories using three metrics (equally weighted, normalized to 0–100 composite):
- **Source file count** — breadth of module complexity (33 pts max)
- **Lines of code** — depth of implementation (33 pts max)
- **Public API symbols** — `pub (struct|enum|fn|trait)` count = integration surface area (34 pts max)

**Tool note:** `rg` (ripgrep) unavailable; `mcp_grep` tool used for pub symbol counts with pattern `\bpub\s+(struct|enum|fn|trait)\b`. Verified per-file.

## Scoring Table — All Crates

| Crate | Total Files | .rs Src Files | LOC | Pub Symbols | Composite Score | On Allowlist |
|-------|-------------|---------------|------|-------------|-----------------|--------------|
| `intel` | 17 | 14 | 6,320 | 88 | **90.1** | ✅ YES |
| `route-graph` | 9 | 8 | 5,954 | 81 | **75.6** | ✅ YES |
| `sc-data-extractor` | 27 | 18 | 2,166 | 85 | **73.8** | ✅ YES |
| `cli` | 34 | 20 | 4,465 | 38 | **71.0** | ✅ YES |
| `api-client` | 9 | 8 | 3,262 | 35 | **43.7** | ✅ YES |
| `data-viewer` | 11 | 10 | 1,595 | 41 | 40.6 | ❌ NO |
| `wikelo-data` | 4 | 3 | 1,302 | 14 | 17.2 | ❌ NO |
| `server` | 6 | 3 | 332 | 2 | 7.5 | ❌ NO |
| `dataforge-explorer` | 2 | 1 | 482 | 0 | 4.2 | ❌ NO |
| `scunpacked-explorer` | 2 | 1 | 392 | 0 | 3.7 | ❌ NO |
| `sc-logistics-importer` | 6 | 1 | 183 | 0 | 2.6 | ❌ NO |

### Score Formula

```
score = (src_files / 20) * 33 + (loc / 6320) * 33 + (pub_symbols / 88) * 34
```

Where 20, 6320, 88 are the max values across all crates (cli src files, intel LOC, intel pub symbols).

### Correction Note

Previous run had cli LOC as 2,007 (composite 58.2). Fresh `wc -l` confirms 4,465 lines across 20 .rs files in `crates/cli/src/`. Corrected composite: **71.0**. Ranking improved (4th → 3rd among allowlisted) but no allowlist changes.

## Scoring — dbt Directory

| Metric | Value |
|--------|-------|
| Total files | 23 |
| SQL models | 13 |
| YAML configs | 3 |
| Model layers | 3 (staging/silver/gold) |
| Total SQL lines | 305 |

**dbt is scored separately** — it uses a different tech stack (SQL/YAML, not Rust) and represents an independent analytics domain. Composite Rust scoring doesn't apply. Inclusion is justified by:
- Separate tech stack requiring distinct workflow guidance (dbt CLI, docker, medallion layers)
- 13 SQL models spanning 3 layers = non-trivial domain knowledge
- Explicit denylist patterns (`dbt/target`, `dbt_packages`) that agents must know to avoid

## Finalized AGENTS.md Locations

| # | Path | Score/Justification |
|---|------|---------------------|
| 1 | `.` (root) | Update existing — project-wide context, bd workflow, commands |
| 2 | `crates/intel` | 90.1 — Highest score: most LOC, most pub symbols, complex domain |
| 3 | `crates/route-graph` | 75.6 — Dense graph/spatial algorithms, high pub surface |
| 4 | `crates/sc-data-extractor` | 73.8 — build.rs schema gen, parser/database subsystems, high pub count |
| 5 | `crates/cli` | 71.0 — TUI entrypoint, snapshot tests, largest file count (34) |
| 6 | `crates/api-client` | 43.7 — External API clients (UEX, SC API, FleetYards), mockito tests |
| 7 | `dbt` | Separate stack — SQL analytics, medallion layers, docker workflow |

## Excluded Crates — Rationale

| Crate | Score | Exclusion Reason |
|-------|-------|------------------|
| `data-viewer` | 40.6 | Not in interview allowlist; standalone TUI viewer, low integration with core pipeline |
| `wikelo-data` | 17.2 | Data-only crate consumed by `intel`; low independent guidance value |
| `server` | 7.5 | Thin HTTP wrapper; 2 pub symbols, 332 LOC — minimal agent guidance needed |
| `dataforge-explorer` | 4.2 | Single-file utility; no pub API surface |
| `scunpacked-explorer` | 3.7 | Single-file utility; no pub API surface |
| `sc-logistics-importer` | 2.6 | Single-file CLI; no pub API surface |

## Validation

- All 5 allowlisted crates score above the highest excluded crate (data-viewer at 40.6)
- Clear scoring gap: lowest allowlisted (api-client: 43.7) > highest excluded (data-viewer: 40.6)
- dbt justified by separate tech stack, not Rust scoring
- Allowlist matches plan interview summary exactly (no additions, no removals)
- All 7 allowlist paths confirmed to exist on disk (see task-2-allowlist.txt)
