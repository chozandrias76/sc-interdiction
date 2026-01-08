//! Database creation and population with Diesel ORM
//!
//! This module provides `PostgreSQL` database support using the Diesel ORM
//! with a medallion architecture (raw → silver → gold schemas).

pub mod builder;
pub mod connection;
pub mod models;
pub mod queries;
pub mod schema;

pub use builder::DatabaseBuilder;
pub use connection::Database;
pub use queries::MapLocation;
