//! Canonical, versioned types that may cross an application boundary.
//!
//! This crate owns representation only. It contains no domain policy, transport,
//! persistence, provider, operating-system, or framework dependency.

use serde::{Deserialize, Serialize};

/// Current major version of the foundation IPC contract.
pub const FOUNDATION_CONTRACT_VERSION: u16 = 1;

/// Read-only metadata used to prove the desktop-to-core boundary is operational.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FoundationStatus {
    /// Human-readable product name.
    pub product_name: String,
    /// Version of the native application.
    pub application_version: String,
    /// Version of this response contract.
    pub contract_version: u16,
}
