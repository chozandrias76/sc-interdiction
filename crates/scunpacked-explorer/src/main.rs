//! scunpacked-explorer - CLI for exploring scunpacked-data JSON files.
//!
//! This is a development/exploration tool for finding game data like
//! Wikelo contracts, items, and other content in the extracted JSON files.

#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use eyre::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use regex::RegexBuilder;
use serde_json::Value;

/// CLI for exploring scunpacked-data JSON files.
///
/// Use this tool to search for items, labels, and other game data
/// extracted from Star Citizen via scunpacked-data repository.
#[derive(Parser, Debug)]
#[command(name = "scunpacked-explorer")]
#[command(about = "Explore scunpacked-data JSON files for game content")]
struct Args {
    /// Path to scunpacked-data directory
    #[arg(short, long, default_value = "extracted/scunpacked-data")]
    data_dir: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search for items/labels matching a pattern (case-insensitive regex)
    Search {
        /// Pattern to search for
        pattern: String,

        /// Search in items.json
        #[arg(long, default_value = "true")]
        items: bool,

        /// Search in labels.json
        #[arg(long)]
        labels: bool,

        /// Maximum results to show
        #[arg(short = 'n', long, default_value = "50")]
        limit: usize,
    },

    /// List all unique item types with counts
    Types {
        /// Show only types matching this pattern
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Show full JSON for a specific item by className
    Show {
        /// Item className to display
        class_name: String,
    },

    /// Dump all items of a given type
    DumpType {
        /// Item type to dump (e.g., "Cargo", "`WeaponPersonal`")
        item_type: String,

        /// Maximum items to show
        #[arg(short = 'n', long, default_value = "20")]
        limit: usize,
    },

    /// Search labels by key or value
    Labels {
        /// Pattern to search for in keys or values
        pattern: String,

        /// Maximum results to show
        #[arg(short = 'n', long, default_value = "50")]
        limit: usize,
    },

    /// List available JSON files in the data directory
    Files,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Search {
            pattern,
            items,
            labels,
            limit,
        } => search(&args.data_dir, &pattern, items, labels, limit),
        Commands::Types { filter } => list_types(&args.data_dir, filter.as_deref()),
        Commands::Show { class_name } => show_item(&args.data_dir, &class_name),
        Commands::DumpType { item_type, limit } => dump_type(&args.data_dir, &item_type, limit),
        Commands::Labels { pattern, limit } => search_labels(&args.data_dir, &pattern, limit),
        Commands::Files => list_files(&args.data_dir),
    }
}

/// Load items.json with progress indicator.
fn load_items(data_dir: &Path) -> Result<Vec<Value>> {
    let path = data_dir.join("items.json");
    let file = File::open(&path).wrap_err_with(|| format!("Failed to open {}", path.display()))?;
    let size = file.metadata()?.len();

    let pb = ProgressBar::new(size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("#>-"),
    );
    pb.set_message("Loading items.json...");

    let reader = BufReader::new(file);
    let items: Vec<Value> = serde_json::from_reader(reader)
        .wrap_err_with(|| format!("Failed to parse {}", path.display()))?;

    pb.finish_with_message("Loaded items.json");
    Ok(items)
}

/// Load labels.json with progress indicator.
fn load_labels(data_dir: &Path) -> Result<HashMap<String, String>> {
    let path = data_dir.join("labels.json");
    let file = File::open(&path).wrap_err_with(|| format!("Failed to open {}", path.display()))?;

    let pb = ProgressBar::new_spinner();
    pb.set_message("Loading labels.json...");

    let reader = BufReader::new(file);
    let labels: HashMap<String, String> = serde_json::from_reader(reader)
        .wrap_err_with(|| format!("Failed to parse {}", path.display()))?;

    pb.finish_with_message(format!("Loaded {} labels", labels.len()));
    Ok(labels)
}

/// Search items and/or labels for a pattern.
fn search(
    data_dir: &Path,
    pattern: &str,
    search_items: bool,
    search_labels: bool,
    limit: usize,
) -> Result<()> {
    let regex = RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .wrap_err("Invalid regex pattern")?;

    let mut count = 0;

    if search_items {
        let items = load_items(data_dir)?;
        println!("\n=== Items matching '{}' ===\n", pattern);

        for item in &items {
            if count >= limit {
                println!("\n... (truncated at {} results)", limit);
                break;
            }

            let item_str = serde_json::to_string(item)?;
            if regex.is_match(&item_str) {
                let class_name = item
                    .get("className")
                    .and_then(|v| v.as_str())
                    .unwrap_or("?");
                let item_type = item.get("type").and_then(|v| v.as_str()).unwrap_or("?");
                let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("?");

                println!("{:50} | {:20} | {}", class_name, item_type, name);
                count += 1;
            }
        }

        if count == 0 {
            println!("No items found matching '{}'", pattern);
        }
    }

    if search_labels {
        let labels = load_labels(data_dir)?;
        println!("\n=== Labels matching '{}' ===\n", pattern);

        let mut label_count = 0;
        for (key, value) in &labels {
            if label_count >= limit {
                println!("\n... (truncated at {} results)", limit);
                break;
            }

            if regex.is_match(key) || regex.is_match(value) {
                println!("{}: {}", key, truncate(value, 100));
                label_count += 1;
            }
        }

        if label_count == 0 {
            println!("No labels found matching '{}'", pattern);
        }
    }

    Ok(())
}

/// List all unique item types with counts.
fn list_types(data_dir: &Path, filter: Option<&str>) -> Result<()> {
    let items = load_items(data_dir)?;

    let mut type_counts: HashMap<String, usize> = HashMap::new();
    for item in &items {
        let item_type = item
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();
        *type_counts.entry(item_type).or_insert(0) += 1;
    }

    let mut sorted: Vec<_> = type_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let filter_regex = filter
        .map(|f| RegexBuilder::new(f).case_insensitive(true).build())
        .transpose()
        .wrap_err("Invalid filter regex")?;

    println!("\n=== Item Types ===\n");
    println!("{:>6}  Type", "Count");
    println!("{:-<50}", "");

    for (item_type, count) in sorted {
        if let Some(ref re) = filter_regex {
            if !re.is_match(&item_type) {
                continue;
            }
        }
        println!("{count:>6}  {item_type}");
    }

    Ok(())
}

/// Show full JSON for a specific item.
fn show_item(data_dir: &Path, class_name: &str) -> Result<()> {
    let items = load_items(data_dir)?;

    let regex = RegexBuilder::new(class_name)
        .case_insensitive(true)
        .build()
        .wrap_err("Invalid pattern")?;

    for item in &items {
        let cn = item.get("className").and_then(|v| v.as_str()).unwrap_or("");
        if regex.is_match(cn) {
            let pretty = serde_json::to_string_pretty(item)?;
            println!("{pretty}");
            return Ok(());
        }
    }

    println!("No item found with className matching '{}'", class_name);
    Ok(())
}

/// Dump all items of a given type.
fn dump_type(data_dir: &Path, item_type: &str, limit: usize) -> Result<()> {
    let items = load_items(data_dir)?;

    let regex = RegexBuilder::new(item_type)
        .case_insensitive(true)
        .build()
        .wrap_err("Invalid type pattern")?;

    println!("\n=== Items of type '{}' ===\n", item_type);

    let mut count = 0;
    for item in &items {
        if count >= limit {
            println!("\n... (truncated at {} results)", limit);
            break;
        }

        let t = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
        if regex.is_match(t) {
            let class_name = item
                .get("className")
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("?");
            let sub_type = item.get("subType").and_then(|v| v.as_str()).unwrap_or("?");

            println!("{:50} | {:15} | {}", class_name, sub_type, name);
            count += 1;
        }
    }

    if count == 0 {
        println!("No items found with type matching '{}'", item_type);
    }

    Ok(())
}

/// Search labels by key or value pattern.
fn search_labels(data_dir: &Path, pattern: &str, limit: usize) -> Result<()> {
    let labels = load_labels(data_dir)?;

    let regex = RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .wrap_err("Invalid regex pattern")?;

    println!("\n=== Labels matching '{}' ===\n", pattern);

    let mut count = 0;
    for (key, value) in &labels {
        if count >= limit {
            println!("\n... (truncated at {} results)", limit);
            break;
        }

        if regex.is_match(key) || regex.is_match(value) {
            println!("{}:", key);
            println!("  {}\n", truncate(value, 200));
            count += 1;
        }
    }

    if count == 0 {
        println!("No labels found matching '{}'", pattern);
    }

    Ok(())
}

/// List available JSON files in the data directory.
fn list_files(data_dir: &Path) -> Result<()> {
    println!("\n=== Available data files ===\n");

    let entries = std::fs::read_dir(data_dir)
        .wrap_err_with(|| format!("Cannot read {}", data_dir.display()))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            let size_mb = size as f64 / 1_000_000.0;
            println!("{:30} {:>8.1} MB", name, size_mb);
        }
    }

    let subdirs: Vec<_> = std::fs::read_dir(data_dir)?
        .flatten()
        .filter(|e| e.path().is_dir())
        .collect();

    if !subdirs.is_empty() {
        println!("\nSubdirectories:");
        for entry in subdirs {
            let name = entry.file_name();
            let count = std::fs::read_dir(entry.path())
                .map(|d| d.count())
                .unwrap_or(0);
            println!("  {:30} ({} files)", name.to_string_lossy(), count);
        }
    }

    Ok(())
}

/// Truncate a string to a maximum length.
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}
