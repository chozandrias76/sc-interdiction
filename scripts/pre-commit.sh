#!/bin/bash
# Git pre-commit hook for sc-interdiction
# Runs code quality checks before allowing a commit
#
# To install this hook:
#   cp scripts/pre-commit.sh .git/hooks/pre-commit
#   chmod +x .git/hooks/pre-commit

set -e

echo "🔍 Running pre-commit quality checks..."

# Check commit size (lines changed)
echo "  → Checking commit size..."
LINES_CHANGED=$(git diff --cached --numstat | awk '{sum += $1 + $2} END {print sum}')
MAX_LINES=500

if [ -n "$LINES_CHANGED" ] && [ "$LINES_CHANGED" -gt "$MAX_LINES" ]; then
    echo "⚠️  Large commit detected: $LINES_CHANGED lines changed (limit: $MAX_LINES)"
    echo "   Consider breaking this into smaller, focused commits."
    echo "   Exceptions:"
    echo "   - Merge commits (use standard merge process)"
    echo "   - Auto-generated code, lock files, or dependencies"
    echo "   - Initial implementation of large features (document why in commit message)"
    echo ""
    read -p "   Continue anyway? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Commit cancelled. Please split into smaller commits."
        exit 1
    fi
fi

# Run cargo clippy with strict quality checks
echo "  → Running clippy with cognitive complexity and code quality checks..."
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIPPY_ARGS=$(cat "$SCRIPT_DIR/clippy-args.txt" | tr '\n' ' ')
if ! cargo clippy --all-targets --all-features -- $CLIPPY_ARGS; then
    echo "❌ Clippy quality checks failed!"
    echo "   Fix the issues above or use 'git commit --no-verify' to skip checks."
    exit 1
fi

# Run cargo check to ensure everything compiles
echo "  → Running cargo check..."
if ! cargo check --all-targets --all-features; then
    echo "❌ Cargo check failed!"
    exit 1
fi

# Run tests
echo "  → Running tests..."
if ! cargo test --all-features; then
    echo "❌ Tests failed!"
    exit 1
fi

# Run formatting check
echo "  → Checking code formatting..."
if ! cargo fmt --all -- --check; then
    echo "⚠️  Code is not formatted. Running 'cargo fmt'..."
    cargo fmt --all
    echo "   Code has been formatted. Please review and stage the changes."
    exit 1
fi

echo "✅ All pre-commit checks passed!"
exit 0
