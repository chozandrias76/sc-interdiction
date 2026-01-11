//! `PostgreSQL` connection management with Diesel

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;

use crate::error::{Error, Result};

/// Type alias for the connection pool
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Type alias for a pooled connection
pub type PooledPgConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Database connection manager for `PostgreSQL`
pub struct Database {
    pool: DbPool,
}

impl Database {
    /// Creates a new database connection pool from the `DATABASE_URL` environment variable.
    ///
    /// # Errors
    ///
    /// Returns an error if `DATABASE_URL` is not set or if connection fails.
    pub fn from_env() -> Result<Self> {
        let database_url = env::var("DATABASE_URL").map_err(|_| {
            Error::Database("DATABASE_URL environment variable not set".to_string())
        })?;
        Self::new(&database_url)
    }

    /// Creates a new database connection pool with the given URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection pool cannot be created.
    pub fn new(database_url: &str) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .map_err(|e| Error::Database(format!("Failed to create connection pool: {e}")))?;

        Ok(Self { pool })
    }

    /// Gets a connection from the pool.
    ///
    /// # Errors
    ///
    /// Returns an error if no connection is available.
    pub fn get_connection(&self) -> Result<PooledPgConnection> {
        self.pool
            .get()
            .map_err(|e| Error::Database(format!("Failed to get connection: {e}")))
    }

    /// Returns a reference to the underlying pool.
    #[must_use]
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[test]
    #[ignore = "requires PostgreSQL database"]
    fn test_connection() {
        dotenv().ok();
        let db = Database::from_env().expect("Failed to create database");
        let _conn = db.get_connection().expect("Failed to get connection");
    }
}
