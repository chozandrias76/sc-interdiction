//! Error types for API clients.

use thiserror::Error;

/// Result type for API operations.
pub type Result<T> = std::result::Result<T, ApiError>;

/// Errors that can occur when interacting with APIs.
#[derive(Debug, Error)]
pub enum ApiError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// Failed to parse JSON response.
    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),

    /// API returned an error response.
    #[error("API error {status}: {message}")]
    Api { status: u16, message: String },

    /// Rate limit exceeded.
    #[error("Rate limit exceeded, retry after {retry_after_secs}s")]
    RateLimited { retry_after_secs: u64 },

    /// Resource not found.
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Missing API key.
    #[error("API key required but not configured")]
    MissingApiKey,
}
