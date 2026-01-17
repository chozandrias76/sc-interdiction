//! `DataForge` Explorer - CLI for exploring Star Citizen game data from scunpacked-data.
//!
//! This tool works with JSON files from the scunpacked-data repository, which contains
//! pre-extracted game data including items, ships, factions, and labels.

use clap::{Parser, Subcommand};
use eyre::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use regex::RegexBuilder;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

/// CLI for exploring Star Citizen scunpacked-data JSON files.
#[derive(Parser, Debug)]
#[command(name = "dataforge-explorer")]
#[command(about = "Explore Star Citizen game data from scunpacked-data JSON files")]
struct Args {
    /// Path to scunpacked-data directory
    #[arg(short, long, default_value = "extracted/scunpacked-data")]
    data_dir: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Search for records matching a pattern (case-insensitive regex)
    Search {
        /// Pattern to search for in record names, types, and content
        pattern: String,

        /// Limit number of results
        #[arg(short, long, default_value = "50")]
        limit: usize,

        /// Search in specific file (items, ships, factions, labels)
        #[arg(short, long)]
        file: Option<String>,
    },

    /// List all unique types with counts
    Types {
        /// File to analyze (items, ships, or individual folder)
        #[arg(short, long, default_value = "items")]
        file: String,
    },

    /// Show a specific record by className or name
    Show {
        /// Record name or className to find
        name: String,

        /// File to search in
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Dump all records of a given type
    DumpTypes {
        /// Type to filter by (e.g., "`WeaponPersonal`", "Usable")
        type_filter: String,

        /// File to search in
        #[arg(short, long, default_value = "items")]
        file: String,

        /// Limit number of results
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// List available data files
    Files,

    /// Show statistics about the data
    Stats,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Verify data directory exists
    if !args.data_dir.exists() {
        eyre::bail!(
            "Data directory not found: {}\nRun from project root or specify --data-dir",
            args.data_dir.display()
        );
    }

    match args.command {
        Command::Search {
            pattern,
            limit,
            file,
        } => search(&args.data_dir, &pattern, limit, file.as_deref()),
        Command::Types { file } => list_types(&args.data_dir, &file),
        Command::Show { name, file } => show_record(&args.data_dir, &name, file.as_deref()),
        Command::DumpTypes {
            type_filter,
            file,
            limit,
        } => dump_types(&args.data_dir, &type_filter, &file, limit),
        Command::Files => list_files(&args.data_dir),
        Command::Stats => show_stats(&args.data_dir),
    }
}

/// Load a JSON file with progress indicator.
fn load_json_file(path: &Path) -> Result<Value> {
    let file = File::open(path).wrap_err_with(|| format!("Failed to open {}", path.display()))?;
    let size = file.metadata()?.len();

    let pb = ProgressBar::new(size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40}] {bytes}/{total_bytes}")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("=> "),
    );
    pb.set_message(format!(
        "Loading {}",
        path.file_name().unwrap_or_default().to_string_lossy()
    ));

    let reader = BufReader::new(file);
    let value: Value = serde_json::from_reader(reader)
        .wrap_err_with(|| format!("Failed to parse JSON from {}", path.display()))?;

    pb.finish_and_clear();
    Ok(value)
}

/// Search for records matching a pattern.
#[allow(clippy::print_stdout)]
fn search(data_dir: &Path, pattern: &str, limit: usize, file: Option<&str>) -> Result<()> {
    let regex = RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .wrap_err("Invalid regex pattern")?;

    let mut results: Vec<(String, String, String)> = Vec::new();

    let files_to_search: Vec<(&str, PathBuf)> = if let Some(f) = file {
        vec![(f, data_dir.join(format!("{f}.json")))]
    } else {
        vec![
            ("items", data_dir.join("items.json")),
            ("ships", data_dir.join("ships.json")),
            ("labels", data_dir.join("labels.json")),
            ("manufacturers", data_dir.join("manufacturers.json")),
        ]
    };

    for (name, path) in files_to_search {
        if !path.exists() {
            continue;
        }

        let data = load_json_file(&path)?;

        if let Some(arr) = data.as_array() {
            for item in arr {
                let json_str = item.to_string();
                if regex.is_match(&json_str) {
                    let class_name = item
                        .get("className")
                        .or_else(|| item.get("ClassName"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let item_type = item
                        .get("type")
                        .or_else(|| item.get("Type"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let item_name = item
                        .get("name")
                        .or_else(|| item.get("Name"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    results.push((
                        format!("{name}:{class_name}"),
                        item_type.to_string(),
                        item_name.to_string(),
                    ));

                    if results.len() >= limit {
                        break;
                    }
                }
            }
        }

        if results.len() >= limit {
            break;
        }
    }

    // Also search individual files in subdirectories
    if results.len() < limit && file.is_none() {
        for subdir in ["ships", "items", "factions"] {
            let dir_path = data_dir.join(subdir);
            if dir_path.is_dir() {
                search_directory(&dir_path, &regex, subdir, &mut results, limit)?;
            }
            if results.len() >= limit {
                break;
            }
        }
    }

    println!(
        "\nFound {} matches for pattern '{pattern}':\n",
        results.len()
    );
    println!("{:<60} {:<25} Name", "Record", "Type");
    println!("{}", "-".repeat(120));

    for (record, type_name, name) in &results {
        let display_name = if name.len() > 35 {
            format!("{}...", &name[..32])
        } else {
            name.clone()
        };
        println!("{record:<60} {type_name:<25} {display_name}");
    }

    Ok(())
}

/// Search a directory of individual JSON files.
fn search_directory(
    dir: &Path,
    regex: &regex::Regex,
    subdir_name: &str,
    results: &mut Vec<(String, String, String)>,
    limit: usize,
) -> Result<()> {
    let entries: Vec<_> = fs::read_dir(dir)?.filter_map(|e| e.ok()).collect();

    for entry in entries {
        if results.len() >= limit {
            break;
        }

        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            let filename = path.file_stem().unwrap_or_default().to_string_lossy();
            if regex.is_match(&filename) {
                results.push((
                    format!("{subdir_name}/{filename}"),
                    "file".to_string(),
                    filename.to_string(),
                ));
            }
        }
    }

    Ok(())
}

/// List all unique types with counts.
#[allow(clippy::print_stdout)]
fn list_types(data_dir: &Path, file: &str) -> Result<()> {
    let path = data_dir.join(format!("{file}.json"));
    let data = load_json_file(&path)?;

    let mut type_counts: HashMap<String, usize> = HashMap::new();

    if let Some(arr) = data.as_array() {
        for item in arr {
            let item_type = item
                .get("type")
                .or_else(|| item.get("Type"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            *type_counts.entry(item_type.to_string()).or_insert(0) += 1;
        }
    }

    let mut sorted: Vec<_> = type_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\nTypes in {file}.json ({} unique types):\n", sorted.len());
    println!("{:<40} Count", "Type");
    println!("{}", "-".repeat(50));

    for (type_name, count) in sorted {
        println!("{type_name:<40} {count}");
    }

    Ok(())
}

/// Show a specific record by name.
#[allow(clippy::print_stdout)]
fn show_record(data_dir: &Path, name: &str, file: Option<&str>) -> Result<()> {
    let regex = RegexBuilder::new(&format!("^{name}$"))
        .case_insensitive(true)
        .build()?;

    let files_to_search: Vec<PathBuf> = if let Some(f) = file {
        vec![data_dir.join(format!("{f}.json"))]
    } else {
        vec![data_dir.join("items.json"), data_dir.join("ships.json")]
    };

    for path in files_to_search {
        if !path.exists() {
            continue;
        }

        let data = load_json_file(&path)?;

        if let Some(arr) = data.as_array() {
            for item in arr {
                let class_name = item
                    .get("className")
                    .or_else(|| item.get("ClassName"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                if regex.is_match(class_name) {
                    println!("\nFound in {}:\n", path.display());
                    println!("{}", serde_json::to_string_pretty(item)?);
                    return Ok(());
                }
            }
        }
    }

    // Check individual files in subdirectories
    for subdir in ["ships", "items", "factions"] {
        let file_path = data_dir.join(subdir).join(format!("{name}.json"));
        if file_path.exists() {
            let data = load_json_file(&file_path)?;
            println!("\nFound {}:\n", file_path.display());
            println!("{}", serde_json::to_string_pretty(&data)?);
            return Ok(());
        }
    }

    eyre::bail!("Record '{name}' not found")
}

/// Dump all records of a given type.
#[allow(clippy::print_stdout)]
fn dump_types(data_dir: &Path, type_filter: &str, file: &str, limit: usize) -> Result<()> {
    let path = data_dir.join(format!("{file}.json"));
    let data = load_json_file(&path)?;

    let regex = RegexBuilder::new(type_filter)
        .case_insensitive(true)
        .build()?;

    let mut count = 0;

    if let Some(arr) = data.as_array() {
        for item in arr {
            let item_type = item
                .get("type")
                .or_else(|| item.get("Type"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            if regex.is_match(item_type) {
                let class_name = item
                    .get("className")
                    .or_else(|| item.get("ClassName"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let name = item
                    .get("name")
                    .or_else(|| item.get("Name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                println!("\n--- {class_name} ({name}) ---");
                println!("{}", serde_json::to_string_pretty(item)?);

                count += 1;
                if count >= limit {
                    println!("\n... (limited to {limit} results)");
                    break;
                }
            }
        }
    }

    println!("\nFound {count} records matching type '{type_filter}'");
    Ok(())
}

/// List available data files.
#[allow(clippy::print_stdout)]
fn list_files(data_dir: &Path) -> Result<()> {
    println!("\nAvailable data files in {}:\n", data_dir.display());

    // Main JSON files
    println!("Main files:");
    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|e| e == "json") {
            let size = entry.metadata()?.len();
            let size_str = if size > 1_000_000 {
                format!("{:.1} MB", size as f64 / 1_000_000.0)
            } else if size > 1_000 {
                format!("{:.1} KB", size as f64 / 1_000.0)
            } else {
                format!("{size} B")
            };
            println!(
                "  {:<25} {}",
                path.file_name().unwrap_or_default().to_string_lossy(),
                size_str
            );
        }
    }

    // Subdirectories
    println!("\nSubdirectories:");
    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
            && !path
                .file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with('.'))
        {
            let count = fs::read_dir(&path)?.count();
            println!(
                "  {:<25} {} files",
                path.file_name().unwrap_or_default().to_string_lossy(),
                count
            );
        }
    }

    Ok(())
}

/// Show statistics about the data.
#[allow(clippy::print_stdout)]
fn show_stats(data_dir: &Path) -> Result<()> {
    println!("\nScunpacked Data Statistics:\n");

    let files = [
        ("items.json", "Items"),
        ("ships.json", "Ships"),
        ("labels.json", "Labels"),
        ("manufacturers.json", "Manufacturers"),
        ("fps-items.json", "FPS Items"),
        ("ship-items.json", "Ship Items"),
    ];

    for (filename, label) in files {
        let path = data_dir.join(filename);
        if path.exists() {
            let data = load_json_file(&path)?;
            if let Some(arr) = data.as_array() {
                println!("  {label:<20} {:>6} records", arr.len());
            }
        }
    }

    // Count subdirectory files
    println!("\nDetailed files:");
    for subdir in ["ships", "items", "factions"] {
        let dir_path = data_dir.join(subdir);
        if dir_path.is_dir() {
            let count = fs::read_dir(&dir_path)?.count();
            println!("  {subdir:<20} {count:>6} individual files");
        }
    }

    Ok(())
}
