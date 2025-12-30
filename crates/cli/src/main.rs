//! SC Interdiction Tool CLI
//!
//! Command-line interface for route analysis and target intel.

mod tui;

use clap::{Parser, Subcommand};
use eyre::Result;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api_client::{FleetYardsClient, UexClient};
use intel::TargetAnalyzer;
use server::AppState;

#[derive(Parser)]
#[command(name = "sc-interdiction")]
#[command(about = "Star Citizen interdiction planning tool")]
#[command(version)]
struct Cli {
    /// Star Citizen API key (from starcitizen-api.com)
    #[arg(long, env = "SC_API_KEY")]
    api_key: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the REST API server
    Serve {
        /// Address to bind to
        #[arg(short, long, default_value = "127.0.0.1:3000")]
        addr: SocketAddr,
    },

    /// Show hot trade routes (best targets)
    Routes {
        /// Number of routes to show
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Show complete trade runs (round-trips with return cargo)
    Runs {
        /// Number of trade runs to show
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Find interdiction chokepoints
    Chokepoints {
        /// Number of top chokepoints to show (max 100)
        #[arg(short, long, default_value = "10")]
        top: usize,

        /// Include cross-system routes (between different star systems)
        #[arg(long)]
        cross_system: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Get target intel for a location
    Intel {
        /// Location name (e.g., "Port Olisar", "Hurston")
        location: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// List available cargo ships (static database)
    Ships {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Fetch ship specs from FleetYards API
    FleetShips {
        /// Filter by ship name (partial match)
        #[arg(short, long)]
        name: Option<String>,

        /// Filter by manufacturer
        #[arg(short, long)]
        manufacturer: Option<String>,

        /// Show only cargo ships (cargo > 0)
        #[arg(short, long)]
        cargo_only: bool,

        /// Force refresh cache
        #[arg(long)]
        refresh: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// List all terminals in a system
    Terminals {
        /// System name (e.g., "Stanton")
        #[arg(short, long)]
        system: Option<String>,

        /// Filter by terminal type (fuel, refinery, commodity, etc.)
        #[arg(short = 't', long)]
        filter_type: Option<String>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Find nearest interdiction hotspots to a location
    Nearby {
        /// Location to search from (e.g., "Port Olisar", "Hurston")
        location: String,

        /// Number of top nearby hotspots to show (max 50)
        #[arg(short, long, default_value = "5")]
        top: usize,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Calculate distance between two locations
    Distance {
        /// Starting location (e.g., "Hurston", "Port Olisar")
        from: String,

        /// Destination location (e.g., "Crusader", "microTech")
        to: String,
    },

    /// List known locations in a system
    Locations {
        /// System name (e.g., "Stanton", "Pyro")
        #[arg(short, long, default_value = "Stanton")]
        system: String,
    },

    /// Launch interactive TUI dashboard
    Dashboard {
        /// Initial location to analyze (e.g., "Crusader", "Hurston")
        #[arg(short, long)]
        location: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up tracing
    let filter = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| filter.into()),
        )
        .init();

    let api_key = cli.api_key.unwrap_or_default();

    match cli.command {
        Commands::Serve { addr } => handle_serve(addr, &api_key).await?,
        Commands::Routes { limit, json } => handle_routes(limit, json).await?,
        Commands::Runs { limit, json } => handle_runs(limit, json).await?,
        Commands::Chokepoints {
            top,
            cross_system,
            json,
        } => handle_chokepoints(top, cross_system, json).await?,
        Commands::Intel { location, json } => handle_intel(&location, json).await?,
        Commands::Ships { json } => handle_ships(json)?,
        Commands::FleetShips {
            name,
            manufacturer,
            cargo_only,
            refresh,
            json,
        } => handle_fleet_ships(name, manufacturer, cargo_only, refresh, json).await?,
        Commands::Terminals {
            system,
            filter_type,
            json,
        } => handle_terminals(system, filter_type, json).await?,
        Commands::Nearby {
            location,
            top,
            json,
        } => handle_nearby(&location, top, json).await?,
        Commands::Distance { from, to } => handle_distance(&from, &to),
        Commands::Locations { system } => handle_locations(&system),
        Commands::Dashboard { location } => tui::run(location).await?,
    }

    Ok(())
}

async fn handle_serve(addr: SocketAddr, api_key: &str) -> Result<()> {
    let state = AppState::new(api_key);
    state.init_graph().await?;
    server::run(addr, state).await
}

async fn handle_routes(limit: usize, json: bool) -> Result<()> {
    let uex = UexClient::new();
    let analyzer = TargetAnalyzer::new(uex);
    let routes = analyzer.get_hot_routes(limit).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&routes)?);
    } else {
        print_routes_table(&routes, limit);
    }
    Ok(())
}

async fn handle_runs(limit: usize, json: bool) -> Result<()> {
    let uex = UexClient::new();
    let analyzer = TargetAnalyzer::new(uex);
    let runs = analyzer.get_trade_runs(limit).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&runs)?);
    } else {
        print_runs_table(&runs, limit);
    }
    Ok(())
}

async fn handle_chokepoints(top: usize, cross_system: bool, json: bool) -> Result<()> {
    const MAX_CHOKEPOINTS: usize = 100;
    let top = top.min(MAX_CHOKEPOINTS);

    let uex = UexClient::new();
    let analyzer = TargetAnalyzer::new(uex.clone());
    let graph = build_route_graph(&uex).await?;
    let chokepoints = analyzer
        .find_interdiction_points(&graph, top, cross_system)
        .await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&chokepoints)?);
    } else {
        print_chokepoints_table(&chokepoints);
    }
    Ok(())
}

async fn handle_intel(location: &str, json: bool) -> Result<()> {
    let uex = UexClient::new();
    let analyzer = TargetAnalyzer::new(uex);
    let targets = analyzer.predict_targets_at(location).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&targets)?);
    } else {
        print_intel_table(location, &targets);
    }
    Ok(())
}

fn handle_ships(json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(&intel::CARGO_SHIPS)?);
    } else {
        print_ships_table();
    }
    Ok(())
}

async fn handle_fleet_ships(
    name: Option<String>,
    manufacturer: Option<String>,
    cargo_only: bool,
    refresh: bool,
    json: bool,
) -> Result<()> {
    let cache_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("sc-interdiction")
        .join("cache");

    let client = FleetYardsClient::with_cache(cache_dir);
    let mut ships = if refresh {
        client.refresh_cache().await?
    } else {
        client.get_ships().await?
    };

    apply_ship_filters(&mut ships, name, manufacturer, cargo_only);

    if json {
        println!("{}", serde_json::to_string_pretty(&ships)?);
    } else {
        print_fleet_ships_table(&ships);
    }
    Ok(())
}

async fn handle_terminals(
    system: Option<String>,
    filter_type: Option<String>,
    json: bool,
) -> Result<()> {
    let uex = UexClient::new();
    let mut terminals = match &system {
        Some(s) => uex.get_terminals_in_system(s).await?,
        None => uex.get_terminals().await?,
    };

    // Apply type filter if specified
    if let Some(filter) = &filter_type {
        let filter_lower = filter.to_lowercase();
        terminals.retain(|t| match filter_lower.as_str() {
            "fuel" => t.is_refuel,
            "refinery" => t.is_refinery,
            _ => {
                // Match against terminal_type field
                t.terminal_type
                    .as_ref()
                    .map(|tt| tt.to_lowercase().contains(&filter_lower))
                    .unwrap_or(false)
            }
        });
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&terminals)?);
    } else {
        print_terminals_table(&terminals, system);
    }
    Ok(())
}

async fn handle_nearby(location: &str, top: usize, json: bool) -> Result<()> {
    const MAX_NEARBY_HOTSPOTS: usize = 50;
    let top = top.min(MAX_NEARBY_HOTSPOTS);

    let uex = UexClient::new();
    let analyzer = TargetAnalyzer::new(uex.clone());
    let graph = build_route_graph(&uex).await?;
    let chokepoints = analyzer.find_interdiction_points(&graph, 50, false).await?;

    let spatial_index = route_graph::SpatialIndex::from_chokepoints(chokepoints);
    let search_point = estimate_location_position(location);
    let nearby = spatial_index.find_nearest(&search_point, top);

    if json {
        println!("{}", serde_json::to_string_pretty(&nearby)?);
    } else {
        print_nearby_table(location, &nearby);
    }
    Ok(())
}

fn handle_distance(from: &str, to: &str) {
    match route_graph::distance_between(from, to) {
        Some(dist) => print_distance_info(from, to, dist),
        None => print_distance_error(),
    }
}

fn handle_locations(system: &str) {
    let locs = route_graph::locations_in_system(system);
    print_locations_table(system, &locs);
}

async fn build_route_graph(uex: &UexClient) -> Result<route_graph::RouteGraph> {
    let terminals = uex.get_terminals().await?;
    let mut graph = route_graph::RouteGraph::new();

    for terminal in &terminals {
        graph.add_terminal(terminal);
    }

    let systems: std::collections::HashSet<_> = terminals
        .iter()
        .filter_map(|t| t.star_system_name.clone())
        .collect();
    for system in systems {
        graph.connect_system(&system);
    }

    Ok(graph)
}

fn apply_ship_filters(
    ships: &mut Vec<api_client::ShipModel>,
    name: Option<String>,
    manufacturer: Option<String>,
    cargo_only: bool,
) {
    if let Some(ref name_filter) = name {
        let filter_lower = name_filter.to_lowercase();
        ships.retain(|s| s.name.to_lowercase().contains(&filter_lower));
    }

    if let Some(ref mfr_filter) = manufacturer {
        let filter_lower = mfr_filter.to_lowercase();
        ships.retain(|s| {
            s.manufacturer
                .as_ref()
                .is_some_and(|m| m.name.to_lowercase().contains(&filter_lower))
        });
    }

    if cargo_only {
        ships.retain(|s| s.cargo_capacity().unwrap_or(0) > 0);
    }

    ships.sort_by(|a, b| {
        b.cargo_capacity()
            .unwrap_or(0)
            .cmp(&a.cargo_capacity().unwrap_or(0))
    });
}

fn print_routes_table(routes: &[intel::HotRoute], limit: usize) {
    println!("\n{:=<80}", "");
    println!(" HOT TRADE ROUTES (Top {})", limit);
    println!("{:=<80}\n", "");

    for (i, route) in routes.iter().enumerate() {
        println!(
            "{}. {} ({:.0} aUEC/SCU)",
            i + 1,
            route.commodity,
            route.profit_per_scu
        );
        println!("   {} -> {}", route.origin, route.destination);
        println!(
            "   Likely ship: {} ({} SCU)",
            route.likely_ship.name, route.likely_ship.cargo_scu
        );
        println!("   Est. haul value: {:.0} aUEC", route.estimated_haul_value);
        println!();
    }
}

fn print_runs_table(runs: &[intel::TradeRun], limit: usize) {
    println!("\n{:=<80}", "");
    println!(" TRADE RUNS - Round Trips (Top {})", limit);
    println!("{:=<80}\n", "");

    for (i, run) in runs.iter().enumerate() {
        let return_info = if run.has_return_cargo {
            "✓ with return cargo"
        } else {
            "✗ deadhead return"
        };

        println!(
            "{}. {} ({}) - Total profit: {:.0} aUEC",
            i + 1,
            run.likely_ship.name,
            return_info,
            run.total_profit
        );
        println!(
            "   OUTBOUND: {} ({:.0}/SCU)",
            run.outbound.commodity, run.outbound.profit_per_scu
        );
        println!(
            "      {} -> {}",
            run.outbound.origin, run.outbound.destination
        );

        if let Some(ref ret) = run.return_leg {
            println!(
                "   RETURN: {} ({:.0}/SCU)",
                ret.commodity, ret.profit_per_scu
            );
            println!("      {} -> {}", ret.origin, ret.destination);
        }

        println!();
    }
}

fn print_chokepoints_table(chokepoints: &[route_graph::Chokepoint]) {
    println!("\n{:=<80}", "");
    println!(" INTERDICTION CHOKEPOINTS (Top {})", chokepoints.len());
    println!("{:=<80}\n", "");

    for (i, cp) in chokepoints.iter().enumerate() {
        println!("{}. {} ({})", i + 1, cp.node.name, cp.node.system);
        println!("   Routes through: {}", cp.route_count);
        println!("   Traffic score: {:.0}", cp.traffic_score);
        println!("   Position: {}", cp.suggested_position.description);
        println!();
    }
}

fn print_intel_table(location: &str, targets: &[intel::TargetPrediction]) {
    println!("\n{:=<80}", "");
    println!(" TARGET INTEL: {}", location);
    println!("{:=<80}\n", "");

    if targets.is_empty() {
        println!("No predicted targets at this location.");
    } else {
        for target in targets {
            let dir = match target.direction {
                intel::TrafficDirection::Arriving => "INBOUND",
                intel::TrafficDirection::Departing => "OUTBOUND",
            };
            println!(
                "[{}] {} hauling {}",
                dir, target.likely_ship.name, target.commodity
            );
            println!("   Destination: {}", target.destination);
            println!(
                "   Est. cargo value: {:.0} aUEC",
                target.estimated_cargo_value
            );
            println!("   Threat level: {}/10", target.likely_ship.threat_level);
            println!();
        }
    }
}

fn print_ships_table() {
    println!("\n{:=<80}", "");
    println!(" CARGO SHIPS DATABASE");
    println!("{:=<80}\n", "");
    println!(
        "{:<25} {:>8} {:>6} {:>12}",
        "Ship", "Cargo", "Threat", "Value"
    );
    println!("{:-<55}", "");

    for ship in intel::CARGO_SHIPS {
        println!(
            "{:<25} {:>6} SCU {:>6}/10 {:>10} aUEC",
            ship.name, ship.cargo_scu, ship.threat_level, ship.ship_value_uec
        );
    }
}

fn print_fleet_ships_table(ships: &[api_client::ShipModel]) {
    println!("\n{:=<90}", "");
    println!(" FLEETYARDS SHIP DATABASE ({} ships)", ships.len());
    println!("{:=<90}\n", "");
    println!(
        "{:<30} {:>8} {:>12} {:>12} {:>12}",
        "Ship", "Cargo", "H2 Fuel", "QT Fuel", "Price"
    );
    println!("{:-<90}", "");

    for ship in ships.iter().take(50) {
        let cargo = ship
            .cargo_capacity()
            .map(|c| format!("{} SCU", c))
            .unwrap_or_else(|| "-".to_string());
        let h2 = ship
            .hydrogen_fuel_capacity()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "-".to_string());
        let qt = ship
            .quantum_fuel_capacity()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "-".to_string());
        let price = ship
            .price
            .map(|p| format!("{:.0}", p))
            .unwrap_or_else(|| "-".to_string());

        println!(
            "{:<30} {:>8} {:>12} {:>12} {:>12}",
            ship.name, cargo, h2, qt, price
        );
    }

    if ships.len() > 50 {
        println!("\n... and {} more ships", ships.len() - 50);
    }
}

fn print_terminals_table(terminals: &[api_client::Terminal], system: Option<String>) {
    let title = match &system {
        Some(s) => format!("TERMINALS IN {}", s.to_uppercase()),
        None => "ALL TERMINALS".to_string(),
    };

    println!("\n{:=<80}", "");
    println!(" {}", title);
    println!("{:=<80}\n", "");

    for terminal in terminals {
        let mut icons = Vec::new();
        if terminal.is_refuel {
            icons.push("⛽");
        }
        if terminal.is_refinery {
            icons.push("🏭");
        }

        let icon_str = if icons.is_empty() {
            String::new()
        } else {
            format!("[{}] ", icons.join(" "))
        };

        println!(
            "{}{}",
            icon_str,
            terminal.name.as_deref().unwrap_or("Unknown")
        );
        println!("   {}", terminal.location_string());
        println!();
    }

    println!("Total: {} terminals", terminals.len());
}

fn print_nearby_table(location: &str, nearby: &[route_graph::NearbyHotspot]) {
    println!("\n{:=<80}", "");
    println!(" NEAREST HOTSPOTS TO: {} (Top {})", location, nearby.len());
    println!("{:=<80}\n", "");

    for (i, spot) in nearby.iter().enumerate() {
        println!(
            "{}. {} ({}) - {:.1} units away",
            i + 1,
            spot.hotspot.name,
            spot.hotspot.system,
            spot.distance
        );
        println!("   Traffic score: {:.0}", spot.hotspot.traffic_score);
        println!(
            "   Position: {}",
            spot.hotspot.chokepoint.suggested_position.description
        );
        println!();
    }
}

fn print_distance_info(from: &str, to: &str, dist: f64) {
    println!("\n{:=<60}", "");
    println!(" DISTANCE CALCULATION");
    println!("{:=<60}\n", "");
    println!("From: {}", from);
    println!("To:   {}", to);
    println!();
    println!("Distance: {:.2} million km", dist);
    println!("         ({:.4} AU)", dist / 149.6); // 1 AU ≈ 149.6 million km
}

fn print_distance_error() {
    println!("Could not find one or both locations in the database.");
    println!("Known locations can be viewed with: sc-interdiction locations");
}

fn print_locations_table(system: &str, locs: &[&route_graph::LocationPosition]) {
    println!("\n{:=<60}", "");
    println!(" KNOWN LOCATIONS IN {}", system.to_uppercase());
    println!("{:=<60}\n", "");

    if locs.is_empty() {
        println!("No locations found for system: {}", system);
    } else {
        println!("{:<25} {:>15} {:>15}", "Name", "Parent", "Position");
        println!("{:-<60}", "");

        for loc in locs {
            let parent = loc.parent.unwrap_or("-");
            let pos = format!(
                "({:.1}, {:.1}, {:.1})",
                loc.position.x, loc.position.y, loc.position.z
            );
            println!("{:<25} {:>15} {:>15}", loc.name, parent, pos);
        }
    }
}

fn estimate_location_position(location: &str) -> route_graph::Point3D {
    route_graph::estimate_position(location)
        .unwrap_or_else(|| route_graph::Point3D::new(0.0, 0.0, 0.0))
}
