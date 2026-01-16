# Scripts

This directory contains utility scripts for development and data extraction.

## Game Data Extraction

### extract_gamedata.py

Extracts Star Citizen game data for the sc-interdiction project.

**Prerequisites:**
- Python 3.8+
- git (for scunpacked-data method)
- OR scdatatools (`pip install scdatatools`) for direct p4k extraction

**Usage:**

```bash
# Recommended: Clone pre-extracted data from Star Citizen Wiki
python scripts/extract_gamedata.py

# Update existing data
python scripts/extract_gamedata.py --update

# Alternative: Direct extraction from Data.p4k (may fail with newer game versions)
python scripts/extract_gamedata.py --from-p4k --p4k "/mnt/c/Star Citizen/StarCitizen/LIVE/Data.p4k"
```

**Output Files:**

| File | Size | Contents |
|------|------|----------|
| `extracted/scunpacked-data/items.json` | ~46MB | All game items with properties |
| `extracted/scunpacked-data/labels.json` | ~10MB | English localization strings |
| `extracted/scunpacked-data/ships.json` | ~1.4MB | Ship definitions |
| `extracted/scunpacked-data/fps-items.json` | ~19MB | FPS/player equipment |
| `extracted/scunpacked-data/ship-items.json` | ~11MB | Ship components |

**Troubleshooting:**

- **"git not found"**: Install git - this is required for the default extraction method
- **scdatatools fails**: The p4k format changes with game updates. Use the scunpacked-data method instead
- **Files missing after extraction**: Run with `--update` to refresh the data

**Data Source:**

The default method clones [StarCitizenWiki/scunpacked-data](https://github.com/StarCitizenWiki/scunpacked-data),
which contains pre-extracted JSON maintained by the Star Citizen Wiki community.
This data is updated when the game patches.

---

# Clippy Configuration

This directory also contains shared configuration for clippy linting.

## clippy-args.txt

This file contains the clippy arguments used by both:
- CI workflow (`.github/workflows/ci.yml`)
- Pre-commit hook (`scripts/pre-commit.sh`)

This ensures consistency between local development and CI checks.

### Modifying Clippy Rules

To add, remove, or modify clippy rules:

1. Edit `scripts/clippy-args.txt`
2. Each line should contain one argument (e.g., `-D warnings`)
3. Test locally: `bash scripts/pre-commit.sh`
4. Commit the change - both CI and local hooks will use the new rules

### Rule Categories

- `-D` = Deny (error)
- `-W` = Warn (warning)
- `-A` = Allow (suppress)

Common patterns:
- `-D warnings` - Treat all warnings as errors
- `-D clippy::rule_name` - Specific rule as error
- `-W clippy::rule_name` - Specific rule as warning
- `-A clippy::rule_name` - Allow/suppress specific rule
