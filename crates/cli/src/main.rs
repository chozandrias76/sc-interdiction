//! SC Interdiction Tool CLI
//!
//! Command-line interface for route analysis and target intel.

use clap::{Parser, Subcommand};
use eyre::Result;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api_client::{ScApiClient, UexClient};
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

    /// Find interdiction chokepoints
    Chokepoints {
        /// Number of chokepoints to show
        #[arg(short, long, default_value = "10")]
        limit: usize,

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

    /// List available cargo ships
    Ships {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// List all terminals in a system
    Terminals {
        /// System name (e.g., "Stanton")
        #[arg(short, long)]
        system: Option<String>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Find nearest interdiction hotspots to a location
    Nearby {
        /// Location to search from (e.g., "Port Olisar", "Hurston")
        location: String,

        /// Number of nearby hotspots to find
        #[arg(short, long, default_value = "5")]
        count: usize,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up tracing
    let filter = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| filter.into()))
        .init();

    let api_key = cli.api_key.unwrap_or_default();

    match cli.command {
        Commands::Serve { addr } => {
            let state = AppState::new(&api_key);
            state.init_graph().await?;
            server::run(addr, state).await?;
        }

        Commands::Routes { limit, json } => {
            let uex = UexClient::new();
            let analyzer = TargetAnalyzer::new(uex);
            let routes = analyzer.get_hot_routes(limit).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&routes)?);
            } else {
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
                    println!(
                        "   Est. haul value: {:.0} aUEC",
                        route.estimated_haul_value
                    );
                    println!();
                }
            }
        }

        Commands::Chokepoints { limit, json } => {
            let uex = UexClient::new();
            let analyzer = TargetAnalyzer::new(uex.clone());

            // Build graph from terminals
            let terminals = uex.get_terminals().await?;
            let mut graph = route_graph::RouteGraph::new();

            for terminal in &terminals {
                graph.add_terminal(terminal);
            }

            let systems: std::collections::HashSet<_> =
                terminals.iter().map(|t| t.star_system_name.clone()).collect();
            for system in systems {
                graph.connect_system(&system);
            }

            let chokepoints = analyzer.find_interdiction_points(&graph, limit).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&chokepoints)?);
            } else {
                println!("\n{:=<80}", "");
                println!(" INTERDICTION CHOKEPOINTS (Top {})", limit);
                println!("{:=<80}\n", "");

                for (i, cp) in chokepoints.iter().enumerate() {
                    println!(
                        "{}. {} ({})",
                        i + 1,
                        cp.node.name,
                        cp.node.system
                    );
                    println!("   Routes through: {}", cp.route_count);
                    println!("   Traffic score: {:.0}", cp.traffic_score);
                    println!("   Position: {}", cp.suggested_position.description);
                    println!();
                }
            }
        }

        Commands::Intel { location, json } => {
            let uex = UexClient::new();
            let analyzer = TargetAnalyzer::new(uex);
            let targets = analyzer.predict_targets_at(&location).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&targets)?);
            } else {
                println!("\n{:=<80}", "");
                println!(" TARGET INTEL: {}", location);
                println!("{:=<80}\n", "");

                if targets.is_empty() {
                    println!("No predicted targets at this location.");
                } else {
                    for target in &targets {
                        let dir = match target.direction {
                            intel::TrafficDirection::Arriving => "INBOUND",
                            intel::TrafficDirection::Departing => "OUTBOUND",
                        };
                        println!(
                            "[{}] {} hauling {}",
                            dir, target.likely_ship.name, target.commodity
                        );
                        println!(
                            "   Destination: {}",
                            target.destination
                        );
                        println!(
                            "   Est. cargo value: {:.0} aUEC",
                            target.estimated_cargo_value
                        );
                        println!(
                            "   Threat level: {}/10",
                            target.likely_ship.threat_level
                        );
                        println!();
                    }
                }
            }
        }

        Commands::Ships { json } => {
            if json {
                println!("{}", serde_json::to_string_pretty(&intel::CARGO_SHIPS)?);
            } else {
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
                        ship.name,
                        ship.cargo_scu,
                        ship.threat_level,
                        ship.ship_value_uec
                    );
                }
            }
        }

        Commands::Terminals { system, json } => {
            let uex = UexClient::new();
            let terminals = match &system {
                Some(s) => uex.get_terminals_in_system(s).await?,
                None => uex.get_terminals().await?,
            };

            if json {
                println!("{}", serde_json::to_string_pretty(&terminals)?);
            } else {
                let title = match &system {
                    Some(s) => format!("TERMINALS IN {}", s.to_uppercase()),
                    None => "ALL TERMINALS".to_string(),
                };

                println!("\n{:=<80}", "");
                println!(" {}", title);
                println!("{:=<80}\n", "");

                for terminal in &terminals {
                    println!("{}", terminal.name);
                    println!("   {}", terminal.location_string());
                    println!();
                }

                println!("Total: {} terminals", terminals.len());
            }
        }

        Commands::Nearby { location, count, json } => {
            let uex = UexClient::new();
            let analyzer = TargetAnalyzer::new(uex.clone());

            // Build graph and get chokepoints
            let terminals = uex.get_terminals().await?;
            let mut graph = route_graph::RouteGraph::new();

            for terminal in &terminals {
                graph.add_terminal(terminal);
            }

            let systems: std::collections::HashSet<_> =
                terminals.iter().map(|t| t.star_system_name.clone()).collect();
            for system in systems {
                graph.connect_system(&system);
            }

            let chokepoints = analyzer.find_interdiction_points(&graph, 50).await?;

            // Build spatial index
            let spatial_index = route_graph::SpatialIndex::from_chokepoints(chokepoints);

            // Estimate position of the search location
            let search_point = estimate_location_position(&location);

            // Find nearest hotspots
            let nearby = spatial_index.find_nearest(&search_point, count);

            if json {
                println!("{}", serde_json::to_string_pretty(&nearby)?);
            } else {
                println!("\n{:=<80}", "");
                println!(" NEAREST HOTSPOTS TO: {}", location);
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
        }
    }

    Ok(())
}

/// Estimate position for a location name.
fn estimate_location_position(location: &str) -> route_graph::Point3D {
    let loc_lower = location.to_lowercase();

    // Stanton system locations (approximate positions in millions of km)
    if loc_lower.contains("hurston") || loc_lower.contains("lorville") {
        route_graph::Point3D::new(12.0, 0.0, 0.0)
    } else if loc_lower.contains("crusader") || loc_lower.contains("orison") {
        route_graph::Point3D::new(-6.0, 8.0, 0.0)
    } else if loc_lower.contains("arccorp") || loc_lower.contains("area18") || loc_lower.contains("area 18") {
        route_graph::Point3D::new(-18.0, 0.0, 0.0)
    } else if loc_lower.contains("microtech") || loc_lower.contains("new babbage") {
        route_graph::Point3D::new(0.0, 22.0, 0.0)
    } else if loc_lower.contains("port olisar") {
        route_graph::Point3D::new(-6.0, 7.0, 0.5)
    } else if loc_lower.contains("everus") {
        route_graph::Point3D::new(11.5, 0.0, 0.5)
    } else if loc_lower.contains("grim hex") {
        route_graph::Point3D::new(15.0, 3.0, 0.0)
    } else {
        // Default to center
        route_graph::Point3D::new(0.0, 0.0, 0.0)
    }
}
