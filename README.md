# SC Interdiction

A tactical planning tool for Star Citizen interdiction operations. Analyzes real-time trade data to identify high-value targets, optimal chokepoints, and predict hauler routes across the Stanton system.

## Purpose

This tool helps interdictors make data-driven decisions by:

- Identifying the most profitable trade routes currently in use
- Finding spatial chokepoints where multiple high-value routes converge
- Predicting likely cargo ship types and cargo values at specific locations
- Calculating optimal interdiction positions based on traffic patterns

The analysis is based on live commodity pricing from the UEX (Universal Exchange) database and the Star Citizen API, combined with spatial graph analysis of quantum travel routes.

## Features

### Route Analysis
- Discovers active high-profit trade routes
- Estimates likely ship types based on cargo capacity requirements
- Calculates expected haul values

### Chokepoint Detection
- Builds a graph of quantum travel routes between terminals
- Identifies locations where multiple profitable routes intersect
- Provides suggested interdiction positions with traffic scores

### Target Intelligence
- Predicts inbound and outbound hauler traffic at any location
- Shows expected cargo types and values
- Includes threat assessment based on ship defensive capabilities

### Spatial Search
- Finds nearby interdiction hotspots relative to your current position
- Distance-based ranking of opportunities

### REST API Server
- Exposes all analysis capabilities via HTTP endpoints
- Suitable for integration with other tools or overlays

## Installation

### Prerequisites

- Rust 1.70 or later
- Optional: Star Citizen API key from [starcitizen-api.com](https://starcitizen-api.com)

### Build from Source

```bash
git clone https://github.com/chozandrias76/sc-interdiction.git
cd sc-interdiction

# Set up development environment (installs git hooks, creates config)
./scripts/setup-dev.sh

# Build the project
cargo build --release
```

The compiled binary will be available at `target/release/sc-interdiction`.

## Usage

### Command Line Interface

#### Show Hot Trade Routes
```bash
sc-interdiction routes --limit 10
```

#### Find Chokepoints
```bash
# Show top 5 interdiction chokepoints (default: 10, max: 100)
sc-interdiction chokepoints --top 5

# Show top 20 chokepoints
sc-interdiction chokepoints --top 20
```

#### Get Target Intel for a Location
```bash
sc-interdiction intel "Port Olisar"
```

#### List Cargo Ships Database
```bash
sc-interdiction ships
```

#### Find Nearby Hotspots
```bash
# Find top 5 nearest hotspots to a location (default: 5, max: 50)
sc-interdiction nearby "Crusader" --top 5

# Find top 10 nearest hotspots
sc-interdiction nearby "Port Olisar" --top 10
```

#### List All Terminals
```bash
sc-interdiction terminals

# Filter by system
sc-interdiction terminals --system Stanton
```

### REST API Server

Start the server:
```bash
sc-interdiction serve --addr 127.0.0.1:3000
```

#### API Endpoints

**Get Chokepoints**
```bash
# Get top 10 chokepoints (default)
curl http://localhost:3000/api/routes/chokepoints

# Get top 20 chokepoints (max: 100)
curl http://localhost:3000/api/routes/chokepoints?top=20
```

**Get Hotspots**
```bash
# Get top 10 hotspots (default)
curl http://localhost:3000/api/intel/hotspots

# Get top 25 hotspots (max: 100)
curl http://localhost:3000/api/intel/hotspots?top=25
```

The API will be available at `http://127.0.0.1:3000`.

### Configuration

All commands support the following options:

- `--api-key <KEY>` or `SC_API_KEY` environment variable for Star Citizen API access
- `--verbose` or `-v` for detailed logging
- `--json` for machine-readable JSON output

## Project Structure

This is a Rust workspace with the following crates:

- `api-client` - HTTP clients for Star Citizen API and UEX
- `route-graph` - Graph data structures and pathfinding algorithms
- `intel` - Target analysis and prediction logic
- `server` - REST API server implementation
- `cli` - Command-line interface

## Data Sources

- **UEX (Universal Exchange)**: Live commodity pricing and terminal data
- **Star Citizen API**: Ship and location information
- **Local Cache**: Results are cached to minimize API calls

## Development

### Build Configuration

This project is optimized to use `/tmp` as the build directory for faster compilation. See [docs/BUILD_CONFIGURATION.md](docs/BUILD_CONFIGURATION.md) for detailed setup instructions.

**Quick Setup (using direnv):**
```bash
# Install direnv
sudo apt install direnv  # or: brew install direnv

# Enable direnv in your shell (~/.bashrc or ~/.zshrc)
eval "$(direnv hook bash)"  # or zsh, fish, etc.

# Allow direnv in this project
direnv allow
```

The build artifacts will be placed in `/tmp/cargo-target-sc-interdiction` for faster builds.

### Quick Commands (Makefile)

Common development tasks are available via `make`:

```bash
make setup          # Initialize build environment
make build          # Build in debug mode
make build-release  # Build optimized release
make test           # Run all tests
make clippy         # Run linter
make fmt            # Format code
make run            # Run CLI
make serve          # Start API server
make clean          # Remove build artifacts
make help           # Show all available commands
```

## Development

### Setup

Run the setup script to install git hooks and configure your development environment:

```bash
./scripts/setup-dev.sh
```

This will:
- Install pre-commit hooks that run quality checks automatically
- Create `.cargo/config.toml` from the template
- Run initial quality checks

### Code Quality Standards

This project enforces strict code quality rules:

- **Maximum file size**: 500 lines
- **Maximum function length**: 100 lines
- **Cognitive complexity**: ≤ 15
- **Cyclomatic complexity**: ≤ 10
- **Type complexity**: ≤ 250

Quality checks run automatically on commit. To run them manually:

```bash
# Quick quality check
cargo quality

# Full pre-commit checks
cargo pre-commit

# Or use the script directly
./scripts/check-code-quality.sh
```

### Running Tests
```bash
cargo test
# or
make test
```

### Linting
```bash
# Run clippy with warnings
cargo clippy

# Run clippy with strict quality checks (as in pre-commit)
cargo lint
```

### Format Code
```bash
cargo fmt
```

### Build Documentation
```bash
cargo doc --no-deps --open
```

## License

MIT

## Disclaimer

This tool is for educational and entertainment purposes. Star Citizen is a trademark of Cloud Imperium Rights LLC and Cloud Imperium Rights Ltd. This project is not affiliated with or endorsed by Cloud Imperium Games.

## Contributing

Contributions are welcome! Please follow our development workflow:

### Development Workflow

1. Fork the repository
2. Create a feature branch from `develop`:
   ```bash
   git checkout develop
   git checkout -b feature/your-feature-name
   ```
3. Make your changes following our [code quality standards](#code-quality-standards)
4. Ensure all tests pass and code is formatted
5. Commit using [Conventional Commits](https://www.conventionalcommits.org/)
6. Submit a pull request to `develop`

### Pull Request Requirements

All PRs must:
- ✅ Follow conventional commit format in title
- ✅ Include detailed description (min 100 characters)
- ✅ Have required sections: Summary, Changes, Test Results
- ✅ Pass all CI checks (format, lint, tests, security)
- ✅ Meet code coverage requirements

See [PR Validation](.github/workflows/pr-validation.yml) for complete requirements.

### Release Process

Releases follow semantic versioning and are automated through GitHub Actions:

1. **Trigger Release**: Go to Actions → Create Release → Select version bump (patch/minor/major)
2. **Automation**: Creates `release/X.Y.Z` branch and PRs automatically
3. **Review**: Wait for CI checks, then merge PRs
4. **Publish**: Release is automatically published to GitHub with changelog

See [docs/RELEASE_PROCESS.md](docs/RELEASE_PROCESS.md) for detailed documentation.

### Branch Strategy

- **`main`**: Production releases only (PRs from `release/*` branches only)
- **`develop`**: Integration branch (PRs from feature/fix/chore branches)
- **`release/*`**: Release preparation (auto-created by release workflow)
- **`feature/*`**, **`fix/*`**, **`chore/*`**: Development branches

