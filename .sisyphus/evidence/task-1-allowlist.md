# Task 1: Allowlist/Denylist Discovery

## Date: 2026-02-26

## Existing AGENTS.md Files

```
./AGENTS.md   (root, 40 lines — bd workflow + landing-the-plane rules)
```

No subdirectory AGENTS.md files exist yet.

## Allowlist (AGENTS.md target locations)

Per plan interview summary, these directories are approved for AGENTS.md generation:

| Path | Exists | Source Files | Notes |
|------|--------|-------------|-------|
| `.` (root) | YES | N/A | Update existing, preserve workflow rules |
| `crates/cli` | YES | 34 | CLI/TUI entrypoint, snapshot tests |
| `crates/intel` | YES | 17 | Target analysis, ships, wikelo |
| `crates/route-graph` | YES | 9 | Graph algos, spatial, fuel |
| `crates/api-client` | YES | 9 | UEX, SC API, FleetYards clients |
| `crates/sc-data-extractor` | YES | 27 | Parsers, database, build.rs schema gen |
| `dbt` | YES | 23 | Medallion layers, models, seeds |

**Total source files in scope:** 164 (across crates + dbt + scripts + docs)

## Crates NOT on Allowlist (excluded from AGENTS generation)

| Crate | Reason |
|-------|--------|
| `crates/data-viewer` | Not in interview allowlist |
| `crates/dataforge-explorer` | Not in interview allowlist |
| `crates/sc-logistics-importer` | Not in interview allowlist |
| `crates/scunpacked-explorer` | Not in interview allowlist |
| `crates/server` | Not in interview allowlist |
| `crates/wikelo-data` | Not in interview allowlist |

## Denylist (patterns that MUST NOT appear in AGENTS.md files)

These patterns must never be referenced in any generated AGENTS.md:

| Pattern | Source |
|---------|--------|
| `node_modules` | Plan guardrail |
| `/target/` | Plan guardrail + .gitignore |
| `dbt_packages` | Plan guardrail |
| `dbt/target` | Plan guardrail + .gitignore |

## Verification

- `find . -type f -name "AGENTS.md"` → only `./AGENTS.md` (confirmed)
- `find crates dbt scripts docs -type f | wc -l` → 164 files (confirmed)
- All 6 allowlisted subdirs verified to exist
- No denylist directories found on disk (all gitignored properly)
- Existing `AGENTS.md` contains zero denylist pattern matches
