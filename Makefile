# Makefile for sc-interdiction project
# Provides convenient shortcuts for common development tasks

# Export CARGO_TARGET_DIR - use /tmp for faster builds
# This can be overridden by direnv or user's environment
export CARGO_TARGET_DIR ?= /tmp/cargo-target-sc-interdiction
export CARGO_INCREMENTAL ?= 1

.PHONY: help setup build build-release test test-pkg test-cli coverage coverage-html coverage-check install-coverage-tools clean check fmt clippy doc run serve db-up db-down db-migrate db-import data-viewer

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
	@echo "Database (PostgreSQL via Docker):"
	@echo "  make db-setup                 - All-in-one: start + migrate + import + dbt"
	@echo "  make data-viewer              - Launch data browser TUI"
	@echo "  make db-up                    - Start PostgreSQL (auto-finds free port)"
	@echo "  make db-down                  - Stop PostgreSQL"
	@echo "  make db-port                  - Show current port and DATABASE_URL"
	@echo "  make db-reset                 - Wipe and rebuild database"
	@echo "  make db-shell                 - Interactive psql shell"
	@echo "  make db-query SQL=\"...\"       - Run a SQL query"
	@echo ""
	@echo "dbt (transformations):"
	@echo "  make dbt-all                  - Full pipeline: seed + run + test"
	@echo "  make dbt-seed                 - Load seed data (display_names)"
	@echo "  make dbt-run                  - Run all models (silver/gold)"
	@echo "  make dbt-test                 - Run dbt tests"
	@echo "  make dbt-debug                - Test dbt connection"
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

# Database port file (persists allocated port)
DB_PORT_FILE := .db-port

# Find available port starting from base, incrementing until free
define find_port
$(shell \
	port=$(1); \
	while ss -tln 2>/dev/null | grep -q ":$$port " || \
	      netstat -tln 2>/dev/null | grep -q ":$$port "; do \
		port=$$((port + 1)); \
		if [ $$port -gt $$(($(1) + 100)) ]; then \
			echo "ERROR: No free port found" >&2; \
			exit 1; \
		fi; \
	done; \
	echo $$port)
endef

# Load or find DB port
DB_PORT := $(shell cat $(DB_PORT_FILE) 2>/dev/null || echo "")
ifeq ($(DB_PORT),)
  DB_PORT := $(call find_port,5432)
endif
export DB_PORT
export DATABASE_URL = postgres://sc:sc@localhost:$(DB_PORT)/sc_interdiction

# Database commands
db-up:
	@echo "Finding available port starting from 5432..."
	@port=$$(port=5432; \
		while ss -tln 2>/dev/null | grep -q ":$$port " || \
		      netstat -tln 2>/dev/null | grep -q ":$$port "; do \
			port=$$((port + 1)); \
		done; \
		echo $$port); \
	echo $$port > $(DB_PORT_FILE); \
	echo "Using port: $$port"; \
	DB_PORT=$$port docker compose up -d db
	@echo "Waiting for database to be ready..."
	@until docker compose exec -T db pg_isready -U sc -d sc_interdiction >/dev/null 2>&1; do \
		sleep 1; \
	done
	@echo "✓ PostgreSQL is ready on port $$(cat $(DB_PORT_FILE))"
	@echo "  DATABASE_URL=postgres://sc:sc@localhost:$$(cat $(DB_PORT_FILE))/sc_interdiction"

db-down:
	docker compose down
	@rm -f $(DB_PORT_FILE)

db-migrate:
	@if [ ! -f $(DB_PORT_FILE) ]; then echo "Database not running. Run 'make db-up' first."; exit 1; fi
	@echo "Running migrations..."
	@DATABASE_URL=postgres://sc:sc@localhost:$$(cat $(DB_PORT_FILE))/sc_interdiction \
		sh -c 'cd crates/sc-data-extractor && diesel migration run'
	@echo "✓ Migrations complete"

db-import:
	@if [ ! -f $(DB_PORT_FILE) ]; then echo "Database not running. Run 'make db-up' first."; exit 1; fi
	@echo "Importing SCLogistics data..."
	@DATABASE_URL=postgres://sc:sc@localhost:$$(cat $(DB_PORT_FILE))/sc_interdiction \
		cargo run -p sc-logistics-importer -- all
	@echo "✓ Import complete"

db-setup: db-up
	@sleep 2
	$(MAKE) db-migrate
	$(MAKE) db-import
	$(MAKE) dbt-all
	@echo ""
	@echo "✓ Database ready! Run 'make data-viewer' to browse."

data-viewer:
	@if [ ! -f $(DB_PORT_FILE) ]; then echo "Database not running. Run 'make db-up' first."; exit 1; fi
	@DATABASE_URL=postgres://sc:sc@localhost:$$(cat $(DB_PORT_FILE))/sc_interdiction \
		cargo run -p data-viewer

db-reset: db-down
	docker volume rm sc-interdiction_pgdata 2>/dev/null || true
	$(MAKE) db-setup

db-port:
	@if [ -f $(DB_PORT_FILE) ]; then \
		echo "Database port: $$(cat $(DB_PORT_FILE))"; \
		echo "DATABASE_URL=postgres://sc:sc@localhost:$$(cat $(DB_PORT_FILE))/sc_interdiction"; \
	else \
		echo "Database not running"; \
	fi

# Generic database query: make db-query SQL="SELECT * FROM gold.locations LIMIT 5"
db-query:
	@if [ -z "$(SQL)" ]; then \
		echo "Usage: make db-query SQL=\"your SQL query here\""; \
		echo "Example: make db-query SQL=\"SELECT * FROM gold.locations LIMIT 5\""; \
		exit 1; \
	fi
	@docker exec -i sc-interdiction-db-1 psql -U sc -d sc_interdiction -c "$(SQL)"

# Interactive psql shell
db-shell:
	@docker exec -it sc-interdiction-db-1 psql -U sc -d sc_interdiction

# ============================================================================
# dbt commands
# ============================================================================

# Run dbt in docker
define run_dbt
	@if [ ! -f $(DB_PORT_FILE) ]; then echo "Database not running. Run 'make db-up' first."; exit 1; fi
	@docker compose run --rm \
		-e DBT_HOST=db \
		-e DBT_PORT=5432 \
		-e DBT_USER=sc \
		-e DBT_PASSWORD=sc \
		-e DBT_DBNAME=sc_interdiction \
		dbt $(1)
endef

dbt-debug:
	$(call run_dbt,debug)

dbt-deps:
	$(call run_dbt,deps)

dbt-seed:
	$(call run_dbt,seed)

dbt-run:
	$(call run_dbt,run)

dbt-test:
	$(call run_dbt,test)

dbt-build:
	$(call run_dbt,build)

# Full dbt pipeline: seed + run + test
dbt-all: dbt-seed dbt-run dbt-test
	@echo "✓ dbt pipeline complete"

# Show dbt docs (generates and serves)
dbt-docs:
	$(call run_dbt,docs generate)
	@echo "Run 'docker compose run --rm -p 8080:8080 dbt docs serve' to view docs"
