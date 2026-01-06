//! REST API server for the interdiction tool.
//!
//! Provides endpoints for route analysis and target intel.

mod routes;
mod state;

pub use routes::create_router;
pub use state::AppState;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

/// Run the server on the specified address.
///
/// # Errors
///
/// Returns an error if the server fails to bind or serve.
pub async fn run(addr: SocketAddr, state: AppState) -> eyre::Result<()> {
    let app = create_router(state);

    info!("Starting server on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
