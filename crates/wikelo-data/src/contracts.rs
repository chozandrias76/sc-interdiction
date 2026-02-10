//! Static contract definitions for Wikelo contracts.
//!
//! Loads contracts from TOML at first access using lazy initialization.
//! Contains all 43 contracts from Phase 9 research (09-RESEARCH.md).

use intel::WikieloContract;
use once_cell::sync::Lazy;
use serde::Deserialize;

const CONTRACTS_TOML: &str = include_str!("../data/contracts.toml");

/// Wrapper struct for TOML root deserialization.
#[derive(Deserialize)]
struct ContractsFile {
    #[serde(default)]
    contracts: Vec<WikieloContract>,
}

static CONTRACTS: Lazy<Vec<WikieloContract>> = Lazy::new(|| {
    let file: ContractsFile =
        toml::from_str(CONTRACTS_TOML).expect("Failed to parse contracts.toml - check TOML syntax");
    file.contracts
});

/// Returns all Wikelo contracts.
#[must_use]
pub fn all_contracts() -> &'static [WikieloContract] {
    &CONTRACTS
}

/// Find a contract by ID.
#[must_use]
pub fn get_contract(id: &str) -> Option<&'static WikieloContract> {
    CONTRACTS.iter().find(|c| c.id == id)
}
