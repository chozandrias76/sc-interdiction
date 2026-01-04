//! API clients for Star Citizen data sources.
//!
//! This crate provides clients for:
//! - [starcitizen-api.com](https://starcitizen-api.com/) - Starmap, ships, orgs
//! - [uexcorp.space](https://uexcorp.space/) - Commodity prices, trade data
//! - [fleetyards.net](https://fleetyards.net/) - Ship specifications, fuel capacity

mod error;
mod fleetyards;
mod sc_api;
mod uex;

#[cfg(test)]
mod uex_tests;

pub use error::{ApiError, Result};
pub use fleetyards::{CrewInfo, FleetYardsClient, Manufacturer, ShipCache, ShipMetrics, ShipModel};
pub use sc_api::{ScApiClient, StarSystem, StarmapObject, Station};
pub use uex::{Commodity, CommodityPrice, Terminal, TradeRoute, UexClient};
