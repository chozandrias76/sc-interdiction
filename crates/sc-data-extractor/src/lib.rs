//! SC Data Extractor
//!
//! Parses Star Citizen game data from the `SCLogistics` repository to build
//! databases for locations, quantum travel routes, and trade terminal information.
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
pub mod error;
pub mod generated;
pub mod models;
pub mod parsers;

pub use error::{Error, Result};

/// Path to the `SCLogistics` repository clone (relative to workspace root)
pub const SCLOGISTICS_PATH: &str = "../SCLogistics";
