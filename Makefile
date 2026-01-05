# Makefile for sc-interdiction project
# Provides convenient shortcuts for common development tasks

# Export CARGO_TARGET_DIR - use /tmp for faster builds
# This can be overridden by direnv or user's environment
export CARGO_TARGET_DIR ?= /tmp/cargo-target-sc-interdiction
export CARGO_INCREMENTAL ?= 1

.PHONY: help setup build build-release test test-pkg test-cli coverage coverage-html coverage-check install-coverage-tools clean check fmt clippy doc run serve

# Coverage threshold
COVERAGE_THRESHOLD := 80

# Default target
help:
	@echo "SC Interdiction - Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make setup                    - Initialize build environment"
	@echo "  direnv allow                  - Enable automatic environment loading"
	@echo ""
	@echo "Build:"
	@echo "  make build                    - Build in debug mode"
	@echo "  make build-release            - Build optimized release"
	@echo "  make check                    - Fast syntax check (no codegen)"
	@echo ""
	@echo "Quality:"
	@echo "  make test                     - Run all tests"
	@echo "  make test-cli                 - Run tests in CLI crate"
	@echo "  make test-pkg PKG=<name>      - Run tests in specific package"
	@echo "  make test-pkg PKG=<name> TEST=<test> - Run specific test in package"
	@echo "  make coverage                 - Generate coverage report"
	@echo "  make coverage-html            - Generate HTML coverage report"
	@echo "  make coverage-check           - Check if coverage >= $(COVERAGE_THRESHOLD)%"
	@echo "  make install-coverage-tools   - Install cargo-tarpaulin"
	@echo "  make clippy                   - Run linter"
	@echo "  make fmt                      - Format code"
	@echo "  make doc                      - Build and open documentation"
	@echo ""
	@echo "Run:"
	@echo "  make run                      - Run CLI (debug build)"
	@echo "  make serve                    - Start API server (debug build)"
	@echo "  make routes                   - Show top 10 trade routes"
	@echo "  make routes-debug             - Show routes with system info"
	@echo "  make chokepoints              - Show intra-system chokepoints"
	@echo "  make chokepoints-cross-system - Show cross-system (jump point) chokepoints"
	@echo "  make ships                    - List cargo ships"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean                    - Remove build artifacts"
	@echo "  make clean-tmp                - Remove /tmp build directory"
	@echo "  make clean-all                - Remove all build artifacts"
	@echo ""
	@echo "Info:"
	@echo "  make info                     - Show build configuration"
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
	@echo "❌ ERROR: Local release builds are disabled!"
	@echo ""
	@echo "Release builds should only be created by CI/CD."
	@echo "Use 'make build' for local development (debug mode)."
	@echo ""
	@echo "If you really need a release build for testing:"
	@echo "  cargo build --release"
	@echo ""
	@exit 1

check:
	cargo check --all

# Quality assurance
test:
	cargo test

# Test specific crate: make test-pkg PKG=sc-interdiction
# Test specific test: make test-pkg PKG=sc-interdiction TEST=scroll_text
test-pkg:
	@if [ -z "$(PKG)" ]; then \
		echo "Error: PKG variable is required. Usage: make test-pkg PKG=sc-interdiction"; \
		exit 1; \
	fi; \
	if [ -n "$(TEST)" ]; then \
		cargo test -p $(PKG) $(TEST); \
	else \
		cargo test -p $(PKG); \
	fi

test-cli:
	cargo test -p sc-interdiction

# Coverage commands
coverage:
	cargo tarpaulin --out Stdout --skip-clean

coverage-html:
	cargo tarpaulin --out Html --skip-clean
	@echo "✓ Coverage report generated at: target/tarpaulin/tarpaulin-report.html"

coverage-check:
	@echo "Checking test coverage..."
	@cargo tarpaulin --out Stdout --skip-clean | tee /tmp/coverage.txt
	@COVERAGE=$$(grep -oP '(\d+\.\d+)%' /tmp/coverage.txt | tail -1 | sed 's/%//'); \
	echo "Current coverage: $$COVERAGE%"; \
	echo "Required coverage: $(COVERAGE_THRESHOLD)%"; \
	if [ $$(echo "$$COVERAGE < $(COVERAGE_THRESHOLD)" | bc -l) -eq 1 ]; then \
		echo "❌ Coverage $$COVERAGE% is below threshold $(COVERAGE_THRESHOLD)%"; \
		exit 1; \
	else \
		echo "✅ Coverage $$COVERAGE% meets threshold $(COVERAGE_THRESHOLD)%"; \
	fi

install-coverage-tools:
	@echo "Installing cargo-tarpaulin..."
	@echo "NOTE: This requires pkg-config and libssl-dev to be installed."
	@echo "On Ubuntu/Debian: sudo apt install pkg-config libssl-dev"
	@echo "On macOS: brew install pkg-config openssl"
	cargo install cargo-tarpaulin

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
	cargo run -- routes --limit 10

routes-debug:
	cargo run -- routes --limit 20 --show-systems

chokepoints:
	cargo run -- chokepoints --top 10

chokepoints-cross-system:
	cargo run -- chokepoints --top 10 --cross-system

dashboard:
	touch crates/cli/src/main.rs && cargo build && cargo run -- dashboard

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
