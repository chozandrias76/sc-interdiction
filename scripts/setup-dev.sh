#!/bin/bash
# Setup development environment for sc-interdiction
# This script installs git hooks and creates necessary config files

set -e

echo "🔧 Setting up development environment..."

# Install pre-commit hook
echo "  → Installing pre-commit hook..."
if [ -f ".git/hooks/pre-commit" ]; then
    echo "    Pre-commit hook already exists. Backing up..."
    mv .git/hooks/pre-commit .git/hooks/pre-commit.backup
fi
cp scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
echo "    ✓ Pre-commit hook installed"

# Create .cargo/config.toml from template if it doesn't exist
echo "  → Setting up cargo config..."
if [ ! -f ".cargo/config.toml" ]; then
    cp .cargo/config.toml.template .cargo/config.toml
    echo "    ✓ Created .cargo/config.toml from template"
else
    echo "    .cargo/config.toml already exists"
fi

# Run initial checks
echo "  → Running initial quality checks..."
if cargo clippy --all-targets --all-features -- -W clippy::cognitive_complexity -W clippy::too_many_lines; then
    echo "    ✓ Initial clippy check passed"
else
    echo "    ⚠️  Some clippy warnings found. Review them before committing."
fi

echo ""
echo "✅ Development environment setup complete!"
echo ""
echo "Available commands:"
echo "  cargo lint          - Run clippy with all warnings as errors"
echo "  cargo build-check   - Run clippy checks (warnings only)"
echo "  cargo pre-commit    - Run strict pre-commit checks manually"
echo "  cargo quality       - Quick quality check"
echo ""
echo "The pre-commit hook will automatically run quality checks before each commit."
echo "To skip the hook, use: git commit --no-verify"
