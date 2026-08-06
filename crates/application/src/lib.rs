//! Application composition and dispatch boundary.
//!
//! This foundation exposes one framework-neutral query proving the dependency
//! direction. Product command/query handlers are added only with their use cases.

use std::sync::RwLock;

use second_brain_contracts::{FOUNDATION_CONTRACT_VERSION, FoundationStatus};

mod actions_and_projects;
mod preferences;

pub use actions_and_projects::{
    ActionsAndProjects, ActionsError, Entity, Field, Project, ProjectId, ProjectState, Task,
    TaskId, TaskState,
};
pub use preferences::{ChangePreferences, MinuteOfDay, PreferenceError, Preferences};

/// Composition root state shared by native entry points.
#[derive(Debug)]
pub struct Application {
    product_name: &'static str,
    version: &'static str,
    preferences: RwLock<Preferences>,
}

impl Application {
    /// Creates the application root from build metadata.
    #[must_use]
    pub fn new(product_name: &'static str, version: &'static str) -> Self {
        Self {
            product_name,
            version,
            preferences: RwLock::new(Preferences::default()),
        }
    }

    /// Executes the foundation status query.
    #[must_use]
    pub fn foundation_status(&self) -> FoundationStatus {
        FoundationStatus {
            product_name: self.product_name.to_owned(),
            application_version: self.version.to_owned(),
            contract_version: FOUNDATION_CONTRACT_VERSION,
        }
    }

    /// Returns a consistent snapshot of explicit user preferences.
    ///
    /// # Errors
    ///
    /// Returns [`PreferenceError::Unavailable`] if the in-process lock was poisoned.
    pub fn preferences(&self) -> Result<Preferences, PreferenceError> {
        self.preferences
            .read()
            .map(|preferences| preferences.clone())
            .map_err(|_| PreferenceError::Unavailable)
    }

    /// Applies an explicit preference change after deterministic validation.
    ///
    /// # Errors
    ///
    /// Returns a deterministic validation, revision-conflict, or availability error.
    pub fn change_preferences(
        &self,
        change: ChangePreferences,
    ) -> Result<Preferences, PreferenceError> {
        let mut preferences = self
            .preferences
            .write()
            .map_err(|_| PreferenceError::Unavailable)?;
        preferences.apply(change)?;
        Ok(preferences.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_preferences_without_an_infrastructure_dependency() {
        let application = Application::new("Second Brain OS", "0.1.0");
        let updated = application
            .change_preferences(ChangePreferences {
                expected_revision: 0,
                recognized_time_zone: Some("E. South America Standard Time".to_owned()),
                day_transition: Some(MinuteOfDay::new(180).expect("valid minute")),
            })
            .expect("preference change succeeds");

        assert_eq!(updated.revision(), 1);
        assert_eq!(application.preferences().expect("snapshot"), updated);
    }
}
