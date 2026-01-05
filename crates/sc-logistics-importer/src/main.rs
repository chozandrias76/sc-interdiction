//! CLI tool for importing SCLogistics data into databases

use clap::{Parser, Subcommand};
use eyre::Result;
use sc_data_extractor::{
    database::{Database, DatabaseBuilder},
    parsers::{ShopsParser, StarmapParser},
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "sc-logistics-importer")]
#[command(about = "Import Star Citizen game data from SCLogistics repository")]
struct Cli {
    /// Path to SCLogistics repository
    #[arg(short, long, default_value = "../SCLogistics")]
    sclogistics_path: PathBuf,

    /// Output database path
    #[arg(short, long, default_value = "data/sc-game-data.db")]
    output: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract and import all data
    All,
    /// Extract only starmap locations
    Starmap,
    /// Extract only shop inventories
    Shops,
    /// Show statistics about the database
    Stats,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::All => {
            println!("Importing all data from {}", cli.sclogistics_path.display());
            import_all(&cli.sclogistics_path, &cli.output)?;
        }
        Commands::Starmap => {
            println!(
                "Importing starmap data from {}",
                cli.sclogistics_path.display()
            );
            import_starmap(&cli.sclogistics_path, &cli.output)?;
        }
        Commands::Shops => {
            println!(
                "Importing shop data from {}",
                cli.sclogistics_path.display()
            );
            import_shops(&cli.sclogistics_path, &cli.output)?;
        }
        Commands::Stats => {
            show_stats(&cli.output)?;
        }
    }

    Ok(())
}

fn import_all(sclogistics_path: &PathBuf, output: &PathBuf) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let output_str = output
        .to_str()
        .ok_or_else(|| eyre::eyre!("Invalid UTF-8 in output path"))?;
    let db = Database::new(output_str)?;
    let mut builder = DatabaseBuilder::new(db);
    builder.init_schema()?;

    println!("Parsing starmap XML files...");
    let parser = StarmapParser::new(sclogistics_path);
    let locations = parser.parse_all()?;
    println!("Found {} locations", locations.len());

    println!("Inserting locations into database...");
    let count = builder.insert_locations(&locations)?;
    println!("Inserted {} locations", count);

    println!("\nParsing shop inventory JSON files...");
    let shops_parser = ShopsParser::new(sclogistics_path);
    let inventories = shops_parser.parse_all()?;
    println!("Found {} shop inventories", inventories.len());

    println!("Inserting shops into database...");
    let shop_count = builder.insert_shops(&inventories)?;
    println!("Inserted {} shops", shop_count);

    println!("\n✓ Database created at {}", output.display());

    Ok(())
}

fn import_starmap(sclogistics_path: &PathBuf, output: &PathBuf) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let output_str = output
        .to_str()
        .ok_or_else(|| eyre::eyre!("Invalid UTF-8 in output path"))?;
    let db = Database::new(output_str)?;
    let mut builder = DatabaseBuilder::new(db);
    builder.init_schema()?;

    println!("Parsing starmap XML files...");
    let parser = StarmapParser::new(sclogistics_path);
    let locations = parser.parse_all()?;
    println!("Found {} locations", locations.len());

    let count = builder.insert_locations(&locations)?;
    println!("Inserted {} locations", count);
    println!("✓ Database updated at {}", output.display());

    Ok(())
}

fn import_shops(sclogistics_path: &PathBuf, output: &PathBuf) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let output_str = output
        .to_str()
        .ok_or_else(|| eyre::eyre!("Invalid UTF-8 in output path"))?;
    let db = Database::new(output_str)?;
    let mut builder = DatabaseBuilder::new(db);
    builder.init_schema()?;

    println!("Parsing shop inventory JSON files...");
    let parser = ShopsParser::new(sclogistics_path);
    let inventories = parser.parse_all()?;
    println!("Found {} inventories", inventories.len());

    let count = builder.insert_shops(&inventories)?;
    println!("Inserted {} shops", count);
    println!("✓ Database updated at {}", output.display());

    Ok(())
}

fn show_stats(db_path: &PathBuf) -> Result<()> {
    let db_path_str = db_path
        .to_str()
        .ok_or_else(|| eyre::eyre!("Invalid UTF-8 in database path"))?;
    let db = Database::new(db_path_str)?;
    let conn = db.connection();

    let location_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM locations", [], |row| row.get(0))?;

    let shop_count: i64 = conn.query_row("SELECT COUNT(*) FROM shops", [], |row| row.get(0))?;

    let item_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM shop_items", [], |row| row.get(0))?;

    println!("Database Statistics:");
    println!("  Locations: {}", location_count);
    println!("  Shops: {}", shop_count);
    println!("  Shop Items: {}", item_count);

    Ok(())
}
