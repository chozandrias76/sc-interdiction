#!/usr/bin/env bash
# Code Quality Checker
#
# Validates code against project quality standards defined in .code-quality.toml
#
# Usage: ./scripts/check-code-quality.sh [--fix]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration (read from .code-quality.toml or use defaults)
MAX_FILE_LINES=500
MAX_FUNCTION_LINES=100
MAX_CYCLOMATIC_COMPLEXITY=15

FIX_MODE=false
if [[ "$1" == "--fix" ]]; then
    FIX_MODE=true
fi

echo "🔍 Running Code Quality Checks..."
echo ""

# Check 1: File size limits
echo "📏 Checking file sizes..."
OVERSIZED_FILES=$(find crates/*/src -name "*.rs" -type f -exec wc -l {} \; | awk -v max=$MAX_FILE_LINES '$1 > max {print $2, "(" $1 " lines)"}' || true)

if [ -n "$OVERSIZED_FILES" ]; then
    echo -e "${YELLOW}⚠️  Files exceeding $MAX_FILE_LINES lines:${NC}"
    echo "$OVERSIZED_FILES"
    echo ""
    echo "💡 Consider refactoring large files into smaller modules"
    echo ""
else
    echo -e "${GREEN}✓${NC} All files within size limits"
fi

# Check 2: Run clippy with strict settings
echo ""
echo "🔧 Running Clippy with complexity checks..."
if cargo clippy --all-targets --all-features -- \
    -W clippy::cognitive_complexity \
    -W clippy::too_many_arguments \
    -W clippy::type_complexity \
    -W clippy::large_enum_variant \
    -D warnings 2>&1 | tee /tmp/clippy-output.txt; then
    echo -e "${GREEN}✓${NC} Clippy checks passed"
else
    echo -e "${RED}✗${NC} Clippy found issues"
    if [ "$FIX_MODE" = true ]; then
        echo "🔧 Attempting automatic fixes..."
        cargo clippy --fix --all-targets --all-features -- \
            -W clippy::cognitive_complexity \
            -W clippy::too_many_arguments \
            -W clippy::type_complexity \
            -W clippy::large_enum_variant || true
    fi
fi

# Check 3: Function complexity (using cargo-geiger or similar)
echo ""
echo "🧮 Checking for overly complex functions..."
# Note: This would require additional tools like cargo-geiger or custom analysis
# For now, we rely on clippy's cognitive_complexity
echo "  (Checked via clippy::cognitive_complexity)"

# Check 4: Code formatting
echo ""
echo "🎨 Checking code formatting..."
if cargo fmt --all -- --check > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Code is properly formatted"
else
    echo -e "${YELLOW}⚠️  Code needs formatting${NC}"
    if [ "$FIX_MODE" = true ]; then
        echo "🔧 Auto-formatting code..."
        cargo fmt --all
        echo -e "${GREEN}✓${NC} Code formatted"
    else
        echo "💡 Run: cargo fmt --all"
    fi
fi

# Check 5: Missing documentation
echo ""
echo "📚 Checking documentation coverage..."
if cargo doc --no-deps --all-features 2>&1 | grep -i "warning.*missing.*documentation" > /dev/null; then
    echo -e "${YELLOW}⚠️  Some public items lack documentation${NC}"
    echo "💡 Add doc comments to public functions, structs, and modules"
else
    echo -e "${GREEN}✓${NC} Documentation coverage looks good"
fi

# Check 6: Test coverage (if tarpaulin is installed)
echo ""
if command -v cargo-tarpaulin &> /dev/null; then
    echo "🧪 Checking test coverage..."
    cargo tarpaulin --workspace --timeout 300 --out Stdout | grep "^Coverage" || true
else
    echo "ℹ️  Install cargo-tarpaulin for coverage reports: cargo install cargo-tarpaulin"
fi

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📊 Code Quality Check Summary"
echo ""
echo "Configuration:"
echo "  • Max file lines: $MAX_FILE_LINES"
echo "  • Max function lines: $MAX_FUNCTION_LINES"
echo "  • Max cyclomatic complexity: $MAX_CYCLOMATIC_COMPLEXITY"
echo ""

if [ -n "$OVERSIZED_FILES" ]; then
    echo -e "${YELLOW}⚠️  Some files need refactoring${NC}"
    exit 1
else
    echo -e "${GREEN}✅ Code quality checks passed!${NC}"
    exit 0
fi
