//! SC Data Extractor
//!
//! Parses Star Citizen game data from multiple sources:
//!
//! - **`SCLogistics`** - XML/JSON files for locations, quantum routes, trade terminals
//! - **scunpacked-data** - Pre-extracted game data (items, ships, labels)
//!
//! # Modules
//!
//! - [`dataforge`] - Access to scunpacked-data (items, ships, localization)
//! - [`database`] - `SQLite` database for extracted data
//! - [`parsers`] - `SCLogistics` XML/JSON parsers
//! - [`models`] - Domain types for locations, routes, shops
//!
//! # Schema Generation
//!
//! This crate uses compile-time schema inference. Set the `SCLOGISTICS_PATH`
//! environment variable to point to your `SCLogistics` repository clone, and
//! the build script will automatically discover all fields from the XML and
//! JSON source files.
//!
//! See the [`generated`] module for the auto-generated types.

pub mod database;
pub mod dataforge;
pub mod error;
pub mod generated;
pub mod models;
pub mod parsers;

pub use dataforge::{DataForgeExtractor, DataForgeStatus, GameItem, Ship};
pub use error::{Error, Result};

/// Path to the `SCLogistics` repository clone (relative to workspace root)
pub const SCLOGISTICS_PATH: &str = "../SCLogistics";
