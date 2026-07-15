//! Application composition and dispatch boundary.
//!
//! This foundation exposes one framework-neutral query proving the dependency
//! direction. Product command/query handlers are added only with their use cases.

use second_brain_contracts::{FOUNDATION_CONTRACT_VERSION, FoundationStatus};

/// Composition root state shared by native entry points.
#[derive(Debug)]
pub struct Application {
    product_name: &'static str,
    application_version: &'static str,
}

impl Application {
    /// Creates the application root from build metadata.
    #[must_use]
    pub const fn new(product_name: &'static str, application_version: &'static str) -> Self {
        Self {
            product_name,
            application_version,
        }
    }

    /// Executes the foundation status query.
    #[must_use]
    pub fn foundation_status(&self) -> FoundationStatus {
        FoundationStatus {
            product_name: self.product_name.to_owned(),
            application_version: self.application_version.to_owned(),
            contract_version: FOUNDATION_CONTRACT_VERSION,
        }
    }
}
