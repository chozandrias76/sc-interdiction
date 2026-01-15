//! Wikelo item and source intelligence.
//!
//! Types for tracking items that Wikelo accepts and where those items come from.
//! This enables source location flagging - ships leaving certain locations
//! are likely carrying specific valuable items.

mod contracts;
mod types;

pub use contracts::*;
pub use types::*;
