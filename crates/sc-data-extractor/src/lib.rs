//! SC Data Extractor
//!
//! Parses Star Citizen game data from the `SCLogistics` repository to build
//! databases for locations, quantum travel routes, and trade terminal information.

pub mod database;
pub mod error;
pub mod models;
pub mod parsers;

pub use error::{Error, Result};

/// Path to the `SCLogistics` repository clone (relative to workspace root)
pub const SCLOGISTICS_PATH: &str = "../SCLogistics";
