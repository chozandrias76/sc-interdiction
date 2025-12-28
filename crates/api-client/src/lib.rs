//! API clients for Star Citizen data sources.
//!
//! This crate provides clients for:
//! - [starcitizen-api.com](https://starcitizen-api.com/) - Starmap, ships, orgs
//! - [uexcorp.space](https://uexcorp.space/) - Commodity prices, trade data

mod error;
mod sc_api;
mod uex;

pub use error::{ApiError, Result};
pub use sc_api::{ScApiClient, StarSystem, StarmapObject, Station};
pub use uex::{Commodity, CommodityPrice, Terminal, TradeRoute, UexClient};
