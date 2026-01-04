# Clippy Configuration

This directory contains shared configuration for clippy linting.

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
