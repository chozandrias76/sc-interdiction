//! Wikelo item and source intelligence.
//!
//! Types for tracking items that Wikelo accepts and where those items come from.
//! This enables source location flagging - ships leaving certain locations
//! are likely carrying specific valuable items.

mod contract_data;
mod contract_registry;
mod contracts;
mod intel;
pub mod items;
mod registry;
mod types;

pub use contract_data::all_contracts;
pub use contract_registry::ContractRegistry;
pub use contracts::*;
pub use intel::{DemandFlag, SourceFlag, SystemFlag, WikieloIntel, WikieloItemSummary};
pub use items::all_items;
pub use registry::WikieloRegistry;
pub use types::*;
