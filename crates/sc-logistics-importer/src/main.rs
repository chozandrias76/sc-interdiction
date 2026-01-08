//! CLI tool for importing `SCLogistics` data into `PostgreSQL` databases

// CLI binaries intentionally use stdout for user output
#![allow(clippy::print_stdout)]

use clap::{Parser, Subcommand};
use diesel::prelude::*;
use diesel::sql_types::BigInt;
use eyre::Result;
use sc_data_extractor::{
    database::{Database, DatabaseBuilder},
    parsers::{ShopsParser, StarmapParser},
};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "sc-logistics-importer")]
#[command(about = "Import Star Citizen game data from SCLogistics repository")]
struct Cli {
    /// Path to `SCLogistics` repository
    #[arg(short, long, default_value = "../SCLogistics")]
    sclogistics_path: PathBuf,

    /// `PostgreSQL` database URL (uses `DATABASE_URL` env var if not provided)
    #[arg(short, long, env = "DATABASE_URL")]
    database_url: Option<String>,

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
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    let database_url = cli.database_url.ok_or_else(|| {
        eyre::eyre!(
            "DATABASE_URL not set. Use --database-url or set DATABASE_URL environment variable"
        )
    })?;

    match cli.command {
        Commands::All => {
            println!("Importing all data from {}", cli.sclogistics_path.display());
            import_all(&cli.sclogistics_path, &database_url)?;
        }
        Commands::Starmap => {
            println!(
                "Importing starmap data from {}",
                cli.sclogistics_path.display()
            );
            import_starmap(&cli.sclogistics_path, &database_url)?;
        }
        Commands::Shops => {
            println!(
                "Importing shop data from {}",
                cli.sclogistics_path.display()
            );
            import_shops(&cli.sclogistics_path, &database_url)?;
        }
        Commands::Stats => {
            show_stats(&database_url)?;
        }
    }

    Ok(())
}

fn import_all(sclogistics_path: &Path, database_url: &str) -> Result<()> {
    let db = Database::new(database_url)?;
    let mut builder = DatabaseBuilder::new(db);

    println!("Parsing starmap XML files...");
    let parser = StarmapParser::new(sclogistics_path);
    let locations = parser.parse_all()?;
    println!("Found {} locations", locations.len());

    println!("Inserting locations into database (raw schema)...");
    let count = builder.insert_locations(&locations)?;
    println!("Inserted {} locations", count);

    println!("\nParsing shop inventory JSON files...");
    let shops_parser = ShopsParser::new(sclogistics_path);
    let inventories = shops_parser.parse_all()?;
    println!("Found {} shop inventories", inventories.len());

    println!("Inserting shops into database (raw schema)...");
    let shop_count = builder.insert_shops(&inventories)?;
    println!("Inserted {} shops", shop_count);

    println!("\n✓ Data imported to PostgreSQL (raw schema)");
    println!("Run 'make dbt-all' to build silver/gold layers");

    Ok(())
}

fn import_starmap(sclogistics_path: &Path, database_url: &str) -> Result<()> {
    let db = Database::new(database_url)?;
    let mut builder = DatabaseBuilder::new(db);

    println!("Parsing starmap XML files...");
    let parser = StarmapParser::new(sclogistics_path);
    let locations = parser.parse_all()?;
    println!("Found {} locations", locations.len());

    let count = builder.insert_locations(&locations)?;
    println!("Inserted {} locations", count);
    println!("✓ Starmap data imported to PostgreSQL (raw schema)");

    Ok(())
}

fn import_shops(sclogistics_path: &Path, database_url: &str) -> Result<()> {
    let db = Database::new(database_url)?;
    let mut builder = DatabaseBuilder::new(db);

    println!("Parsing shop inventory JSON files...");
    let parser = ShopsParser::new(sclogistics_path);
    let inventories = parser.parse_all()?;
    println!("Found {} inventories", inventories.len());

    let count = builder.insert_shops(&inventories)?;
    println!("Inserted {} shops", count);
    println!("✓ Shop data imported to PostgreSQL (raw schema)");

    Ok(())
}

fn show_stats(database_url: &str) -> Result<()> {
    let db = Database::new(database_url)?;
    let mut conn = db.get_connection()?;

    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        cnt: i64,
    }

    let location_count: CountRow =
        diesel::sql_query("SELECT COUNT(*) as cnt FROM raw.locations").get_result(&mut *conn)?;

    let shop_count: CountRow =
        diesel::sql_query("SELECT COUNT(*) as cnt FROM raw.shops").get_result(&mut *conn)?;

    let item_count: CountRow =
        diesel::sql_query("SELECT COUNT(*) as cnt FROM raw.shop_items").get_result(&mut *conn)?;

    println!("Database Statistics (raw schema):");
    println!("  Locations: {}", location_count.cnt);
    println!("  Shops: {}", shop_count.cnt);
    println!("  Shop Items: {}", item_count.cnt);

    // Also show silver schema stats if available
    let silver_location_count: Result<CountRow, _> =
        diesel::sql_query("SELECT COUNT(*) as cnt FROM silver.locations").get_result(&mut *conn);

    if let Ok(count) = silver_location_count {
        let silver_shop_count: CountRow =
            diesel::sql_query("SELECT COUNT(*) as cnt FROM silver.shops").get_result(&mut *conn)?;
        let silver_item_count: CountRow =
            diesel::sql_query("SELECT COUNT(*) as cnt FROM silver.shop_items")
                .get_result(&mut *conn)?;

        println!("\nDatabase Statistics (silver schema):");
        println!("  Locations: {}", count.cnt);
        println!("  Shops: {}", silver_shop_count.cnt);
        println!("  Shop Items: {}", silver_item_count.cnt);
    }

    Ok(())
}
