#!/usr/bin/env bash
# Setup script for sc-interdiction build environment
# This script prepares the /tmp build directory with proper permissions

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Setting up build environment for sc-interdiction...${NC}"

# Define the build directory
BUILD_DIR="${CARGO_TARGET_DIR:-/tmp/cargo-target-sc-interdiction}"

# Create the build directory if it doesn't exist
if [ ! -d "$BUILD_DIR" ]; then
    echo -e "${YELLOW}Creating build directory: $BUILD_DIR${NC}"
    mkdir -p "$BUILD_DIR"
    echo -e "${GREEN}✓ Build directory created${NC}"
else
    echo -e "${GREEN}✓ Build directory already exists: $BUILD_DIR${NC}"
fi

# Set proper permissions (user read/write/execute)
chmod -R u+rwX "$BUILD_DIR" 2>/dev/null || true

# Display build directory info
echo ""
echo "Build Configuration:"
echo "  Target directory: $BUILD_DIR"
echo "  Size: $(du -sh "$BUILD_DIR" 2>/dev/null | cut -f1 || echo 'unknown')"
echo "  Free space: $(df -h "$BUILD_DIR" 2>/dev/null | tail -1 | awk '{print $4}' || echo 'unknown')"

# Check if directory is on tmpfs
if mount | grep -q "^tmpfs on /tmp"; then
    echo -e "  ${GREEN}✓ Using tmpfs (RAM filesystem) for faster builds${NC}"
else
    echo -e "  ${YELLOW}⚠ Not using tmpfs - builds may be slower${NC}"
fi

echo ""
echo -e "${GREEN}Build environment ready!${NC}"
echo ""
echo "Usage:"
echo "  cargo build          - Build in debug mode"
echo "  cargo build --release - Build optimized release"
echo "  cargo clean          - Clean build artifacts"
echo ""
echo "To clean only the tmp build directory:"
echo "  rm -rf $BUILD_DIR"
