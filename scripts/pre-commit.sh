#!/bin/bash
# Git pre-commit hook for sc-interdiction
# Runs code quality checks before allowing a commit
#
# To install this hook:
#   cp scripts/pre-commit.sh .git/hooks/pre-commit
#   chmod +x .git/hooks/pre-commit

set -e

echo "🔍 Running pre-commit quality checks..."

# Run cargo clippy with strict quality checks
echo "  → Running clippy with cognitive complexity and code quality checks..."
if ! cargo clippy --all-targets --all-features -- \
    -D clippy::cognitive_complexity \
    -D clippy::too_many_lines \
    -W clippy::unwrap_used \
    -W clippy::expect_used \
    -W clippy::panic; then
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
