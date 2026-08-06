//! Explicit user preferences that affect the daily-cycle behavior.

/// Number of minutes elapsed since local midnight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinuteOfDay(u16);

impl MinuteOfDay {
    /// Midnight, the default operational-day transition.
    pub const MIDNIGHT: Self = Self(0);

    /// Creates a valid local wall-clock minute.
    ///
    /// # Errors
    ///
    /// Returns [`PreferenceError::InvalidMinuteOfDay`] when the value is outside a day.
    pub fn new(value: u16) -> Result<Self, PreferenceError> {
        if value < 24 * 60 {
            Ok(Self(value))
        } else {
            Err(PreferenceError::InvalidMinuteOfDay(value))
        }
    }

    /// Returns the minute offset from local midnight.
    #[must_use]
    pub const fn value(self) -> u16 {
        self.0
    }
}

/// Explicit preferences currently required by the daily-cycle foundation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preferences {
    revision: u64,
    recognized_time_zone: Option<String>,
    day_transition: MinuteOfDay,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            revision: 0,
            recognized_time_zone: None,
            day_transition: MinuteOfDay::MIDNIGHT,
        }
    }
}

impl Preferences {
    /// Current optimistic-concurrency revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Time-zone identifier last recognized from an authorized OS context.
    #[must_use]
    pub fn recognized_time_zone(&self) -> Option<&str> {
        self.recognized_time_zone.as_deref()
    }

    /// Configured operational-day transition.
    #[must_use]
    pub const fn day_transition(&self) -> MinuteOfDay {
        self.day_transition
    }

    pub(crate) fn apply(&mut self, change: ChangePreferences) -> Result<(), PreferenceError> {
        if change.expected_revision != self.revision {
            return Err(PreferenceError::RevisionConflict {
                expected: change.expected_revision,
                actual: self.revision,
            });
        }
        if change.recognized_time_zone.is_none() && change.day_transition.is_none() {
            return Err(PreferenceError::EmptyChange);
        }

        let time_zone = change
            .recognized_time_zone
            .map(|value| validate_time_zone(&value))
            .transpose()?;
        let changed = time_zone
            .as_ref()
            .is_some_and(|value| self.recognized_time_zone.as_ref() != Some(value))
            || change
                .day_transition
                .is_some_and(|value| value != self.day_transition);

        if changed {
            if let Some(time_zone) = time_zone {
                self.recognized_time_zone = Some(time_zone);
            }
            if let Some(day_transition) = change.day_transition {
                self.day_transition = day_transition;
            }
            self.revision += 1;
        }
        Ok(())
    }
}

/// User-authorized preference changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangePreferences {
    /// Revision observed by the caller.
    pub expected_revision: u64,
    /// Newly recognized local time-zone identifier.
    pub recognized_time_zone: Option<String>,
    /// New operational-day transition.
    pub day_transition: Option<MinuteOfDay>,
}

/// Deterministic failures produced while changing preferences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreferenceError {
    /// The supplied local wall-clock minute is outside a day.
    InvalidMinuteOfDay(u16),
    /// A time-zone identifier was empty or contained control characters.
    InvalidTimeZone,
    /// No preference was supplied.
    EmptyChange,
    /// The caller changed a stale snapshot.
    RevisionConflict {
        /// Revision supplied by the caller.
        expected: u64,
        /// Current domain revision.
        actual: u64,
    },
    /// The in-process preference lock was poisoned.
    Unavailable,
}

fn validate_time_zone(value: &str) -> Result<String, PreferenceError> {
    let normalized = value.trim();
    if normalized.is_empty() || normalized.chars().any(char::is_control) {
        return Err(PreferenceError::InvalidTimeZone);
    }
    Ok(normalized.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_midnight_without_inventing_a_time_zone() {
        let preferences = Preferences::default();
        assert_eq!(preferences.revision(), 0);
        assert_eq!(preferences.day_transition(), MinuteOfDay::MIDNIGHT);
        assert_eq!(preferences.recognized_time_zone(), None);
    }

    #[test]
    fn rejects_minutes_outside_the_local_day() {
        assert_eq!(
            MinuteOfDay::new(1440),
            Err(PreferenceError::InvalidMinuteOfDay(1440))
        );
    }

    #[test]
    fn applies_explicit_changes_and_rejects_stale_writes() {
        let mut preferences = Preferences::default();
        preferences
            .apply(ChangePreferences {
                expected_revision: 0,
                recognized_time_zone: Some(" E. South America Standard Time ".to_owned()),
                day_transition: Some(MinuteOfDay::new(240).expect("valid minute")),
            })
            .expect("valid change");

        assert_eq!(preferences.revision(), 1);
        assert_eq!(
            preferences.recognized_time_zone(),
            Some("E. South America Standard Time")
        );
        assert_eq!(preferences.day_transition().value(), 240);
        assert_eq!(
            preferences.apply(ChangePreferences {
                expected_revision: 0,
                recognized_time_zone: None,
                day_transition: Some(MinuteOfDay::MIDNIGHT),
            }),
            Err(PreferenceError::RevisionConflict {
                expected: 0,
                actual: 1
            })
        );
    }

    #[test]
    fn rejects_empty_changes_and_keeps_repeated_values_idempotent() {
        let mut preferences = Preferences::default();
        assert_eq!(
            preferences.apply(ChangePreferences {
                expected_revision: 0,
                recognized_time_zone: None,
                day_transition: None,
            }),
            Err(PreferenceError::EmptyChange)
        );
        preferences
            .apply(ChangePreferences {
                expected_revision: 0,
                recognized_time_zone: None,
                day_transition: Some(MinuteOfDay::MIDNIGHT),
            })
            .expect("valid idempotent change");
        assert_eq!(preferences.revision(), 0);
    }
}
