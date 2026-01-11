//! Build script for compile-time schema inference from `SCLogistics` data files
//!
//! This script scans the `SCLogistics` repository at compile time to:
//! 1. Discover all XML attributes from starmap files
//! 2. Discover all JSON fields from shop inventory files
//! 3. Infer types from actual values
//! 4. Generate Rust structs and SQL schema

// Build scripts are expected to panic on errors
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde_json::Value as JsonValue;
use walkdir::WalkDir;

/// Represents an inferred field type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum InferredType {
    String,
    Integer,
    Float,
    Boolean,
    /// Nested object with its own fields
    Object(String),
    /// Array of a type
    Array(Box<InferredType>),
}

impl InferredType {
    fn to_rust_type(&self, optional: bool) -> String {
        let base = match self {
            InferredType::String => "String".to_string(),
            InferredType::Integer => "i64".to_string(),
            InferredType::Float => "f64".to_string(),
            InferredType::Boolean => "bool".to_string(),
            InferredType::Object(name) => name.clone(),
            InferredType::Array(inner) => format!("Vec<{}>", inner.to_rust_type(false)),
        };
        if optional {
            format!("Option<{base}>")
        } else {
            base
        }
    }

    fn to_sql_type(&self) -> &'static str {
        match self {
            InferredType::String | InferredType::Object(_) | InferredType::Array(_) => "TEXT",
            InferredType::Integer | InferredType::Boolean => "INTEGER",
            InferredType::Float => "REAL",
        }
    }

    /// Merge two types, preferring the more general type
    fn merge(&self, other: &InferredType) -> InferredType {
        match (self, other) {
            // Same types
            (a, b) if a == b => a.clone(),
            // Integer can be promoted to Float
            (InferredType::Integer, InferredType::Float)
            | (InferredType::Float, InferredType::Integer) => InferredType::Float,
            // Everything else becomes String (most general)
            _ => InferredType::String,
        }
    }
}

/// Infer type from a string value
fn infer_type_from_value(value: &str) -> InferredType {
    let trimmed = value.trim();

    // Empty or whitespace -> String
    if trimmed.is_empty() {
        return InferredType::String;
    }

    // Boolean check (0/1 style common in XML)
    if trimmed == "0" || trimmed == "1" {
        // Could be boolean or integer, we'll use Integer and let context decide
        return InferredType::Integer;
    }

    // Boolean check (true/false style)
    if trimmed.eq_ignore_ascii_case("true") || trimmed.eq_ignore_ascii_case("false") {
        return InferredType::Boolean;
    }

    // Try parsing as integer
    if trimmed.parse::<i64>().is_ok() {
        return InferredType::Integer;
    }

    // Try parsing as float
    if trimmed.parse::<f64>().is_ok() {
        return InferredType::Float;
    }

    // Default to String
    InferredType::String
}

/// Represents a discovered field with its inferred type
#[derive(Debug, Clone)]
struct FieldInfo {
    /// Original name in source (e.g., "@navIcon", "`BuyPrice`")
    original_name: String,
    /// Rust-friendly name (e.g., "`nav_icon`", "`buy_price`")
    rust_name: String,
    /// Inferred type
    field_type: InferredType,
    /// Number of times this field was seen
    occurrence_count: usize,
    /// Total files scanned
    total_files: usize,
}

impl FieldInfo {
    fn is_optional(&self) -> bool {
        // Field is optional if it doesn't appear in all files
        self.occurrence_count < self.total_files
    }
}

/// Schema for a single entity type (e.g., `StarmapObject`, `ShopInventory`)
#[derive(Debug, Default)]
struct EntitySchema {
    name: String,
    fields: BTreeMap<String, FieldInfo>,
    nested_schemas: BTreeMap<String, EntitySchema>,
}

/// Rust reserved keywords that need special handling
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

/// Convert camelCase or `PascalCase` to `snake_case` and handle reserved keywords
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;
    let mut prev_was_underscore = true; // Start true to avoid leading underscore

    for (i, c) in s.chars().enumerate() {
        if c == '@' || c == '_' {
            if !result.is_empty() && !prev_was_underscore {
                result.push('_');
            }
            prev_was_underscore = true;
            continue;
        }

        if c.is_uppercase() {
            // Add underscore before uppercase if:
            // - Not at start
            // - Previous wasn't uppercase (camelCase)
            // - Or next char is lowercase (end of acronym like "XMLParser" -> "xml_parser")
            let next_is_lower = s
                .chars()
                .nth(i + 1)
                .map(|c| c.is_lowercase())
                .unwrap_or(false);
            if !prev_was_underscore && (!prev_was_upper || next_is_lower) {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_was_upper = true;
            prev_was_underscore = false;
        } else {
            result.push(c);
            prev_was_upper = false;
            prev_was_underscore = c == '_';
        }
    }

    // Clean up any double underscores
    while result.contains("__") {
        result = result.replace("__", "_");
    }

    // Remove leading/trailing underscores
    let result = result.trim_matches('_').to_string();

    // Handle reserved keywords by appending _field suffix
    if RUST_KEYWORDS.contains(&result.as_str()) {
        format!("{}_field", result)
    } else {
        result
    }
}

/// Scan all XML files and extract attribute schemas
fn scan_xml_files(starmap_dir: &Path) -> EntitySchema {
    let mut schema = EntitySchema {
        name: "StarmapLocation".to_string(),
        ..Default::default()
    };

    let mut quantum_travel_schema = EntitySchema {
        name: "QuantumTravelData".to_string(),
        ..Default::default()
    };

    let mut file_count = 0;
    let mut quantum_file_count = 0;

    for entry in WalkDir::new(starmap_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "xml")
                .unwrap_or(false)
        })
    {
        file_count += 1;

        if let Ok(content) = fs::read_to_string(entry.path()) {
            let mut reader = Reader::from_str(&content);
            reader.config_mut().trim_text(true);

            let mut buf = Vec::new();

            loop {
                match reader.read_event_into(&mut buf) {
                    Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                        let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                        if tag_name.starts_with("StarMapQuantumTravelDataParams") {
                            quantum_file_count += 1;
                            process_xml_attributes(
                                e,
                                &mut quantum_travel_schema,
                                quantum_file_count,
                            );
                        } else if tag_name.starts_with("StarMapObject")
                            || tag_name.starts_with("StarMap")
                        {
                            process_xml_attributes(e, &mut schema, file_count);
                        }
                    }
                    Ok(Event::Eof) | Err(_) => break,
                    _ => {}
                }
                buf.clear();
            }
        }
    }

    // Update total file counts for optional determination
    for field in schema.fields.values_mut() {
        field.total_files = file_count;
    }
    for field in quantum_travel_schema.fields.values_mut() {
        field.total_files = quantum_file_count;
    }

    // Add quantum travel as nested schema
    if !quantum_travel_schema.fields.is_empty() {
        schema
            .nested_schemas
            .insert("quantum_travel_data".to_string(), quantum_travel_schema);
    }

    println!(
        "cargo::warning=Scanned {} starmap XML files, found {} attributes",
        file_count,
        schema.fields.len()
    );

    schema
}

fn process_xml_attributes(element: &BytesStart, schema: &mut EntitySchema, file_count: usize) {
    for attr in element.attributes().flatten() {
        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
        let value = String::from_utf8_lossy(&attr.value).to_string();

        // Skip internal/metadata attributes we don't need
        if key == "__type" || key == "__path" {
            continue;
        }

        let rust_name = to_snake_case(&key);
        let inferred_type = infer_type_from_value(&value);

        schema
            .fields
            .entry(key.clone())
            .and_modify(|f| {
                f.occurrence_count += 1;
                f.field_type = f.field_type.merge(&inferred_type);
            })
            .or_insert(FieldInfo {
                original_name: key,
                rust_name,
                field_type: inferred_type,
                occurrence_count: 1,
                total_files: file_count,
            });
    }
}

/// Scan all JSON files and extract field schemas
fn scan_json_files(shops_dir: &Path) -> EntitySchema {
    let mut schema = EntitySchema {
        name: "ShopInventory".to_string(),
        ..Default::default()
    };

    let mut collection_schema = EntitySchema {
        name: "InventoryCollection".to_string(),
        ..Default::default()
    };

    let mut item_schema = EntitySchema {
        name: "InventoryItem".to_string(),
        ..Default::default()
    };

    let mut item_id_schema = EntitySchema {
        name: "ItemId".to_string(),
        ..Default::default()
    };

    let inventories_dir = shops_dir.join("shopinventories");
    let mut file_count = 0;
    let mut item_count = 0;

    for entry in WalkDir::new(&inventories_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "json")
                .unwrap_or(false)
        })
    {
        file_count += 1;

        if let Ok(content) = fs::read_to_string(entry.path()) {
            if let Ok(json) = serde_json::from_str::<JsonValue>(&content) {
                process_json_object(&json, &mut schema, file_count);

                // Process nested Collection
                if let Some(collection) = json.get("Collection") {
                    process_json_object(collection, &mut collection_schema, file_count);

                    // Process Inventory array items
                    if let Some(JsonValue::Array(items)) = collection.get("Inventory") {
                        for item in items {
                            item_count += 1;
                            process_json_object(item, &mut item_schema, item_count);

                            // Process ID nested object
                            if let Some(id_obj) = item.get("ID") {
                                process_json_object(id_obj, &mut item_id_schema, item_count);
                            }
                        }
                    }
                }
            }
        }
    }

    // Update total counts
    for field in schema.fields.values_mut() {
        field.total_files = file_count;
    }
    for field in collection_schema.fields.values_mut() {
        field.total_files = file_count;
    }
    for field in item_schema.fields.values_mut() {
        field.total_files = item_count;
    }
    for field in item_id_schema.fields.values_mut() {
        field.total_files = item_count;
    }

    // Build nested structure
    item_schema
        .nested_schemas
        .insert("id".to_string(), item_id_schema);
    collection_schema
        .nested_schemas
        .insert("inventory".to_string(), item_schema);
    schema
        .nested_schemas
        .insert("collection".to_string(), collection_schema);

    println!(
        "cargo::warning=Scanned {} shop JSON files, found {} top-level fields",
        file_count,
        schema.fields.len()
    );

    schema
}

fn process_json_object(obj: &JsonValue, schema: &mut EntitySchema, count: usize) {
    if let JsonValue::Object(map) = obj {
        for (key, value) in map {
            let rust_name = to_snake_case(key);
            let inferred_type = infer_json_type(value);

            schema
                .fields
                .entry(key.clone())
                .and_modify(|f| {
                    f.occurrence_count += 1;
                    f.field_type = f.field_type.merge(&inferred_type);
                })
                .or_insert(FieldInfo {
                    original_name: key.clone(),
                    rust_name,
                    field_type: inferred_type,
                    occurrence_count: 1,
                    total_files: count,
                });
        }
    }
}

fn infer_json_type(value: &JsonValue) -> InferredType {
    match value {
        // Nullable and String both map to String (fallback for null)
        JsonValue::Null | JsonValue::String(_) => InferredType::String,
        JsonValue::Bool(_) => InferredType::Boolean,
        JsonValue::Number(n) => {
            if n.is_i64() {
                InferredType::Integer
            } else {
                InferredType::Float
            }
        }
        JsonValue::Array(arr) => {
            if let Some(first) = arr.first() {
                InferredType::Array(Box::new(infer_json_type(first)))
            } else {
                InferredType::Array(Box::new(InferredType::String))
            }
        }
        // Use serde_json::Value for complex objects
        JsonValue::Object(_) => InferredType::Object("serde_json::Value".to_string()),
    }
}

/// Generate Rust struct code for a schema
fn generate_rust_struct(schema: &EntitySchema, writer: &mut impl Write) -> std::io::Result<()> {
    // Generate nested schemas first
    for nested in schema.nested_schemas.values() {
        generate_rust_struct(nested, writer)?;
        writeln!(writer)?;
    }

    writeln!(
        writer,
        "#[derive(Debug, Clone, Serialize, Deserialize, Default)]"
    )?;
    writeln!(writer, "pub struct {} {{", schema.name)?;

    // Collect nested schema field names (snake_case) to avoid duplicates
    let nested_field_names: std::collections::HashSet<_> = schema.nested_schemas.keys().collect();

    for field in schema.fields.values() {
        // Skip fields that have a corresponding nested schema
        if nested_field_names.contains(&field.rust_name) {
            continue;
        }

        let rust_type = field.field_type.to_rust_type(field.is_optional());

        // Add serde rename if original differs from rust name
        if field.original_name != field.rust_name {
            writeln!(writer, "    #[serde(rename = \"{}\")]", field.original_name)?;
        }

        // Add default for optional fields
        if field.is_optional() {
            writeln!(writer, "    #[serde(default)]")?;
        }

        writeln!(writer, "    pub {}: {},", field.rust_name, rust_type)?;
    }

    // Add fields for nested schemas
    for (name, nested) in &schema.nested_schemas {
        // Find the original name for serde rename
        let original_name = schema
            .fields
            .values()
            .find(|f| f.rust_name == *name)
            .map(|f| f.original_name.clone());

        if let Some(orig) = original_name {
            if orig != *name {
                writeln!(writer, "    #[serde(rename = \"{}\")]", orig)?;
            }
        }
        writeln!(writer, "    #[serde(default)]")?;
        writeln!(writer, "    pub {}: Option<{}>,", name, nested.name)?;
    }

    writeln!(writer, "}}")?;

    Ok(())
}

/// Generate SQL CREATE TABLE statement
fn generate_sql_schema(
    schema: &EntitySchema,
    table_name: &str,
    writer: &mut impl Write,
) -> std::io::Result<()> {
    writeln!(writer, "CREATE TABLE IF NOT EXISTS {} (", table_name)?;

    let mut columns = Vec::new();

    // Find the primary key field (usually 'id', '__ref', or 'ref_field' after keyword handling)
    let pk_field = schema.fields.values().find(|f| {
        f.rust_name == "ref_field"
            || f.rust_name == "id"
            || f.rust_name == "shop_id"
            || f.original_name == "__ref"
    });

    // Collect nested schema field names to avoid duplicates
    let nested_field_names: std::collections::HashSet<_> = schema.nested_schemas.keys().collect();

    for field in schema.fields.values() {
        // Skip fields that have a corresponding nested schema
        if nested_field_names.contains(&field.rust_name) {
            continue;
        }

        let sql_type = field.field_type.to_sql_type();
        let nullable = if field.is_optional() { "" } else { " NOT NULL" };

        let is_pk = pk_field
            .map(|pk| pk.rust_name == field.rust_name)
            .unwrap_or(false);
        let pk_suffix = if is_pk { " PRIMARY KEY" } else { "" };

        columns.push(format!(
            "    {} {}{}{}",
            field.rust_name, sql_type, nullable, pk_suffix
        ));
    }

    writeln!(writer, "{}", columns.join(",\n"))?;
    writeln!(writer, ");")?;

    Ok(())
}

fn main() {
    println!("cargo::rerun-if-env-changed=SCLOGISTICS_PATH");

    let sclogistics_path = match env::var("SCLOGISTICS_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            println!("cargo::warning=SCLOGISTICS_PATH not set, using default schema");
            // Generate minimal default schema for when SCLogistics isn't available
            generate_default_schema();
            return;
        }
    };

    if !sclogistics_path.exists() {
        println!(
            "cargo::warning=SCLOGISTICS_PATH {} does not exist, using default schema",
            sclogistics_path.display()
        );
        generate_default_schema();
        return;
    }

    println!("cargo::rerun-if-changed={}", sclogistics_path.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    // Scan XML files
    let starmap_dir = sclogistics_path.join("starmap");
    let starmap_schema = if starmap_dir.exists() {
        scan_xml_files(&starmap_dir)
    } else {
        println!("cargo::warning=Starmap directory not found");
        EntitySchema::default()
    };

    // Scan JSON files
    let shops_dir = sclogistics_path.join("Shops");
    let shop_schema = if shops_dir.exists() {
        scan_json_files(&shops_dir)
    } else {
        println!("cargo::warning=Shops directory not found");
        EntitySchema::default()
    };

    // Generate Rust code
    let rust_path = out_dir.join("generated_schema.rs");
    let mut rust_file =
        BufWriter::new(File::create(&rust_path).expect("Failed to create rust file"));

    writeln!(rust_file, "// Auto-generated schema from SCLogistics data").unwrap();
    writeln!(rust_file, "// Generated at compile time by build.rs").unwrap();
    writeln!(rust_file).unwrap();
    writeln!(rust_file, "use serde::{{Deserialize, Serialize}};").unwrap();
    writeln!(rust_file).unwrap();

    writeln!(rust_file, "// ===== Starmap Schema =====").unwrap();
    writeln!(rust_file).unwrap();
    generate_rust_struct(&starmap_schema, &mut rust_file).unwrap();

    writeln!(rust_file).unwrap();
    writeln!(rust_file, "// ===== Shop Schema =====").unwrap();
    writeln!(rust_file).unwrap();
    generate_rust_struct(&shop_schema, &mut rust_file).unwrap();

    // Generate SQL schema
    let sql_path = out_dir.join("generated_schema.sql");
    let mut sql_file = BufWriter::new(File::create(&sql_path).expect("Failed to create sql file"));

    writeln!(sql_file, "-- Auto-generated schema from SCLogistics data").unwrap();
    writeln!(sql_file, "-- Generated at compile time by build.rs").unwrap();
    writeln!(sql_file).unwrap();

    generate_sql_schema(&starmap_schema, "locations", &mut sql_file).unwrap();
    writeln!(sql_file).unwrap();

    // Generate schema info constant
    let info_path = out_dir.join("schema_info.rs");
    let mut info_file =
        BufWriter::new(File::create(&info_path).expect("Failed to create info file"));

    writeln!(info_file, "// Schema metadata").unwrap();
    writeln!(info_file).unwrap();
    writeln!(
        info_file,
        "pub const STARMAP_FIELD_COUNT: usize = {};",
        starmap_schema.fields.len()
    )
    .unwrap();
    writeln!(
        info_file,
        "pub const SHOP_FIELD_COUNT: usize = {};",
        shop_schema.fields.len()
    )
    .unwrap();
    writeln!(
        info_file,
        "pub const GENERATED_SQL: &str = include_str!(concat!(env!(\"OUT_DIR\"), \"/generated_schema.sql\"));"
    )
    .unwrap();

    // Flush all writers to ensure data is written to disk before copying
    rust_file.flush().expect("Failed to flush rust file");
    sql_file.flush().expect("Failed to flush sql file");
    info_file.flush().expect("Failed to flush info file");

    println!(
        "cargo::warning=Generated schema with {} starmap fields and {} shop fields",
        starmap_schema.fields.len(),
        shop_schema.fields.len()
    );
    println!("Written to {}", out_dir.display());

    // Copy generated_schema.rs generated_schema.sql and schema_info.rs to resources
    let resources_dir = PathBuf::from(
        "/home/choza/projects/sc-interdiction/crates/sc-logistics-importer/resources",
    );
    fs::create_dir_all(&resources_dir).expect("Failed to create resources directory");
    fs::copy(&rust_path, resources_dir.join("generated_schema.rs"))
        .expect("Failed to copy generated_schema.rs");
    fs::copy(&sql_path, resources_dir.join("generated_schema.sql"))
        .expect("Failed to copy generated_schema.sql");
    fs::copy(&info_path, resources_dir.join("schema_info.rs"))
        .expect("Failed to copy schema_info.rs");
}

fn generate_default_schema() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    // Generate minimal default Rust schema
    let rust_path = out_dir.join("generated_schema.rs");
    let mut rust_file =
        BufWriter::new(File::create(&rust_path).expect("Failed to create rust file"));

    writeln!(rust_file, "// Default schema (SCLOGISTICS_PATH not set)").unwrap();
    writeln!(rust_file, "use serde::{{Deserialize, Serialize}};").unwrap();
    writeln!(rust_file).unwrap();
    writeln!(
        rust_file,
        "#[derive(Debug, Clone, Serialize, Deserialize, Default)]"
    )
    .unwrap();
    writeln!(rust_file, "pub struct StarmapLocation {{").unwrap();
    writeln!(rust_file, "    #[serde(rename = \"@__ref\")]").unwrap();
    writeln!(rust_file, "    #[serde(default)]").unwrap();
    writeln!(rust_file, "    pub ref_attr: Option<String>,").unwrap();
    writeln!(rust_file, "    #[serde(rename = \"@name\")]").unwrap();
    writeln!(rust_file, "    #[serde(default)]").unwrap();
    writeln!(rust_file, "    pub name: Option<String>,").unwrap();
    writeln!(rust_file, "}}").unwrap();

    // Generate minimal default SQL schema
    let sql_path = out_dir.join("generated_schema.sql");
    let mut sql_file = BufWriter::new(File::create(&sql_path).expect("Failed to create sql file"));
    writeln!(sql_file, "-- Default schema (SCLOGISTICS_PATH not set)").unwrap();
    writeln!(sql_file, "CREATE TABLE IF NOT EXISTS locations (").unwrap();
    writeln!(sql_file, "    id TEXT PRIMARY KEY,").unwrap();
    writeln!(sql_file, "    name TEXT").unwrap();
    writeln!(sql_file, ");").unwrap();

    // Generate schema info
    let info_path = out_dir.join("schema_info.rs");
    let mut info_file =
        BufWriter::new(File::create(&info_path).expect("Failed to create info file"));
    writeln!(info_file, "pub const STARMAP_FIELD_COUNT: usize = 2;").unwrap();
    writeln!(info_file, "pub const SHOP_FIELD_COUNT: usize = 0;").unwrap();
    writeln!(
        info_file,
        "pub const GENERATED_SQL: &str = include_str!(concat!(env!(\"OUT_DIR\"), \"/generated_schema.sql\"));"
    )
    .unwrap();
}
