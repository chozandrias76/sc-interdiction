# Makefile for sc-interdiction project
# Provides convenient shortcuts for common development tasks

# Export CARGO_TARGET_DIR - use /tmp for faster builds
# This can be overridden by direnv or user's environment
export CARGO_TARGET_DIR ?= /tmp/cargo-target-sc-interdiction
export CARGO_INCREMENTAL ?= 1

.PHONY: help setup build build-release test clean check fmt clippy doc run serve

# Default target
help:
	@echo "SC Interdiction - Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make setup          - Initialize build environment"
	@echo "  direnv allow        - Enable automatic environment loading"
	@echo ""
	@echo "Build:"
	@echo "  make build          - Build in debug mode"
	@echo "  make build-release  - Build optimized release"
	@echo "  make check          - Fast syntax check (no codegen)"
	@echo ""
	@echo "Quality:"
	@echo "  make test           - Run all tests"
	@echo "  make clippy         - Run linter"
	@echo "  make fmt            - Format code"
	@echo "  make doc            - Build and open documentation"
	@echo ""
	@echo "Run:"
	@echo "  make run            - Run CLI (debug build)"
	@echo "  make serve          - Start API server (debug build)"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean          - Remove build artifacts"
	@echo "  make clean-tmp      - Remove /tmp build directory"
	@echo "  make clean-all      - Remove all build artifacts"
	@echo ""
	@echo "Info:"
	@echo "  make info           - Show build configuration"
	@echo ""
	@echo "Current build target: $(CARGO_TARGET_DIR)"

# Initialize build environment
setup:
	@echo "Setting up build environment..."
	@mkdir -p /tmp/cargo-target-sc-interdiction
	@chmod -R u+rwX /tmp/cargo-target-sc-interdiction 2>/dev/null || true
	@echo "✓ Build directory ready: /tmp/cargo-target-sc-interdiction"
	@if [ -x ./scripts/setup-build-env.sh ]; then \
		bash ./scripts/setup-build-env.sh; \
	fi

# Build commands
build:
	cargo build

build-release:
	cargo build --release

check:
	cargo check --all

# Quality assurance
test:
	cargo test

clippy:
	cargo clippy --all-targets --all-features

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

doc:
	cargo doc --no-deps --open

# Run commands
run:
	cargo run --

serve:
	cargo run -- serve

# Convenience targets with arguments
routes:
	cargo run -- routes --top 10

chokepoints:
	cargo run -- chokepoints --top 10

ships:
	cargo run -- ships

# Cleanup
clean:
	cargo clean

clean-tmp:
	rm -rf /tmp/cargo-target-sc-interdiction
	@echo "✓ Removed /tmp/cargo-target-sc-interdiction"

# Full cleanup
clean-all: clean clean-tmp
	@echo "✓ All build artifacts removed"

# Development workflow
dev: fmt clippy test
	@echo "✓ Development checks passed"

# CI simulation
ci: fmt-check clippy test
	@echo "✓ CI checks passed"

# Show build configuration
info:
	@echo "Build Configuration:"
	@echo "  CARGO_TARGET_DIR: $(CARGO_TARGET_DIR)"
	@echo "  CARGO_INCREMENTAL: $(CARGO_INCREMENTAL)"
	@if [ -d "$(CARGO_TARGET_DIR)" ]; then \
		echo "  Target directory size: $$(du -sh $(CARGO_TARGET_DIR) 2>/dev/null | cut -f1 || echo 'unknown')"; \
		echo "  Free space: $$(df -h $(CARGO_TARGET_DIR) 2>/dev/null | tail -1 | awk '{print $$4}' || echo 'unknown')"; \
	else \
		echo "  Target directory: Not created yet"; \
	fi
	@echo ""
	@if mount | grep -q "^tmpfs on /tmp"; then \
		echo "  ✓ /tmp is on tmpfs (RAM) - builds will be fast!"; \
	else \
		echo "  ⚠ /tmp is NOT on tmpfs - builds may be slower"; \
	fi
	@echo ""
	@echo "Rust version:"
	@rustc --version
	@echo ""
	@echo "Cargo version:"
	@cargo --version
