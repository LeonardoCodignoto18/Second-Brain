//! Fixed commitments, weekly recurrence, and derived daily availability.

use std::collections::{BTreeMap, BTreeSet};

use crate::MinuteOfDay;

/// Stable identifier of a commitment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommitmentId(u64);

impl CommitmentId {
    /// Creates a non-zero identifier.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }

    /// Returns the numeric value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Calendar date in the user's local temporal intention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalDate {
    year: u16,
    month: u8,
    day: u8,
}

impl LocalDate {
    /// Creates a valid Gregorian date.
    ///
    /// # Errors
    /// Returns [`ScheduleError::InvalidDate`] for an impossible date.
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, ScheduleError> {
        if !(1..=9999).contains(&year)
            || !(1..=12).contains(&month)
            || day == 0
            || day > days_in_month(year, month)
        {
            return Err(ScheduleError::InvalidDate);
        }
        Ok(Self { year, month, day })
    }

    /// Returns the year.
    #[must_use]
    pub const fn year(self) -> u16 {
        self.year
    }
    /// Returns the month.
    #[must_use]
    pub const fn month(self) -> u8 {
        self.month
    }
    /// Returns the day of month.
    #[must_use]
    pub const fn day(self) -> u8 {
        self.day
    }
    /// Returns the weekday.
    #[must_use]
    pub fn weekday(self) -> Weekday {
        let mut year = i32::from(self.year);
        let mut month = i32::from(self.month);
        if month < 3 {
            year -= 1;
            month += 12;
        }
        let value = (i32::from(self.day) + (13 * (month + 1)) / 5 + year + year / 4 - year / 100
            + year / 400)
            % 7;
        match value {
            0 => Weekday::Saturday,
            1 => Weekday::Sunday,
            2 => Weekday::Monday,
            3 => Weekday::Tuesday,
            4 => Weekday::Wednesday,
            5 => Weekday::Thursday,
            _ => Weekday::Friday,
        }
    }

    fn next(self) -> Self {
        let maximum = days_in_month(self.year, self.month);
        if self.day < maximum {
            Self {
                day: self.day + 1,
                ..self
            }
        } else if self.month < 12 {
            Self {
                year: self.year,
                month: self.month + 1,
                day: 1,
            }
        } else {
            Self {
                year: self.year + 1,
                month: 1,
                day: 1,
            }
        }
    }
}

/// Local weekday.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Weekday {
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday.
    Sunday,
}

/// A non-empty local wall-clock interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeWindow {
    start: MinuteOfDay,
    end: MinuteOfDay,
}

impl TimeWindow {
    /// Creates an interval whose end is strictly after its start.
    ///
    /// # Errors
    /// Returns [`ScheduleError::InvalidTimeWindow`] for an empty or reversed interval.
    pub fn new(start: MinuteOfDay, end: MinuteOfDay) -> Result<Self, ScheduleError> {
        if start.value() >= end.value() {
            return Err(ScheduleError::InvalidTimeWindow);
        }
        Ok(Self { start, end })
    }
    /// Returns the inclusive start.
    #[must_use]
    pub const fn start(self) -> MinuteOfDay {
        self.start
    }
    /// Returns the exclusive end.
    #[must_use]
    pub const fn end(self) -> MinuteOfDay {
        self.end
    }
    /// Returns the interval duration in minutes.
    #[must_use]
    pub fn duration_minutes(self) -> u16 {
        self.end.value() - self.start.value()
    }
    fn overlaps(self, other: Self) -> bool {
        self.start.value() < other.end.value() && other.start.value() < self.end.value()
    }
}

/// Explicit handling selected when a fixed commitment conflicts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictDecision {
    /// Reject an overlap and request explicit confirmation.
    Reject,
    /// Apply an overlap after explicit user confirmation.
    Confirm,
}

/// Weekly recurrence restricted to selected weekdays.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeeklyRecurrence {
    weekdays: BTreeSet<Weekday>,
}

impl WeeklyRecurrence {
    /// Creates a recurrence with at least one selected weekday.
    ///
    /// # Errors
    /// Returns [`ScheduleError::EmptyRecurrence`] when no weekday is selected.
    pub fn new(weekdays: impl IntoIterator<Item = Weekday>) -> Result<Self, ScheduleError> {
        let weekdays = weekdays.into_iter().collect::<BTreeSet<_>>();
        if weekdays.is_empty() {
            return Err(ScheduleError::EmptyRecurrence);
        }
        Ok(Self { weekdays })
    }
    /// Returns whether the weekday belongs to the series.
    #[must_use]
    pub fn contains(&self, weekday: Weekday) -> bool {
        self.weekdays.contains(&weekday)
    }
    /// Iterates selected weekdays in stable order.
    pub fn weekdays(&self) -> impl Iterator<Item = Weekday> + '_ {
        self.weekdays.iter().copied()
    }
}

/// Immutable commitment aggregate snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commitment {
    id: CommitmentId,
    revision: u64,
    title: String,
    anchor_date: LocalDate,
    window: TimeWindow,
    note: Option<String>,
    recurrence: Option<WeeklyRecurrence>,
    removed: bool,
    exceptions: BTreeMap<LocalDate, OccurrenceException>,
}

impl Commitment {
    /// Returns the identifier.
    #[must_use]
    pub const fn id(&self) -> CommitmentId {
        self.id
    }
    /// Returns the optimistic revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }
    /// Returns the title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }
    /// Returns the anchor date that preserves local temporal intention.
    #[must_use]
    pub const fn anchor_date(&self) -> LocalDate {
        self.anchor_date
    }
    /// Returns the fixed local time window.
    #[must_use]
    pub const fn window(&self) -> TimeWindow {
        self.window
    }
    /// Returns the optional note.
    #[must_use]
    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }
    /// Returns the weekly recurrence, when present.
    #[must_use]
    pub const fn recurrence(&self) -> Option<&WeeklyRecurrence> {
        self.recurrence.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OccurrenceException {
    Removed,
    Changed {
        title: String,
        window: TimeWindow,
        note: Option<String>,
    },
}

/// Materialized commitment occurrence returned by period queries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitmentOccurrence {
    commitment_id: CommitmentId,
    date: LocalDate,
    title: String,
    window: TimeWindow,
    note: Option<String>,
    recurring: bool,
}

impl CommitmentOccurrence {
    /// Returns the owning commitment identifier.
    #[must_use]
    pub const fn commitment_id(&self) -> CommitmentId {
        self.commitment_id
    }
    /// Returns the occurrence date.
    #[must_use]
    pub const fn date(&self) -> LocalDate {
        self.date
    }
    /// Returns the occurrence title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }
    /// Returns the fixed interval.
    #[must_use]
    pub const fn window(&self) -> TimeWindow {
        self.window
    }
    /// Returns the optional note.
    #[must_use]
    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }
    /// Returns whether this occurrence belongs to a weekly series.
    #[must_use]
    pub const fn is_recurring(&self) -> bool {
        self.recurring
    }
}

/// Scope explicitly selected for a recurring edit or removal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecurringScope {
    /// Only the occurrence on the supplied date.
    Occurrence(LocalDate),
    /// The complete recurring series or a one-off commitment.
    Series,
}

/// Full editable values for a commitment or occurrence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitmentChange {
    /// Replacement title.
    pub title: String,
    /// Replacement fixed interval.
    pub window: TimeWindow,
    /// Replacement optional note.
    pub note: Option<String>,
}

/// Date-specific availability override.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvailabilityOverride {
    /// Replacement availability window for the date.
    Window(TimeWindow),
    /// The complete date is unavailable.
    Unavailable,
}

/// Derived availability and its fixed restrictions for one date.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyAvailability {
    date: LocalDate,
    windows: Vec<TimeWindow>,
    restrictive_commitments: Vec<CommitmentOccurrence>,
    complete: bool,
}

impl DailyAvailability {
    /// Returns the date.
    #[must_use]
    pub const fn date(&self) -> LocalDate {
        self.date
    }
    /// Returns free windows in chronological order.
    #[must_use]
    pub fn windows(&self) -> &[TimeWindow] {
        &self.windows
    }
    /// Returns commitments subtracted from the configured window.
    #[must_use]
    pub fn restrictive_commitments(&self) -> &[CommitmentOccurrence] {
        &self.restrictive_commitments
    }
    /// Returns whether a base window or explicit date exception was configured.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }
}

/// Owner of fixed commitments and availability rules.
#[derive(Debug, Default)]
pub struct Schedule {
    commitments: BTreeMap<CommitmentId, Commitment>,
    weekly_availability: BTreeMap<Weekday, TimeWindow>,
    availability_exceptions: BTreeMap<LocalDate, AvailabilityOverride>,
    availability_revision: u64,
}

impl Schedule {
    /// Creates a fixed commitment; an exact identifier replay returns its current snapshot.
    ///
    /// # Errors
    /// Returns [`ScheduleError`] for invalid text, conflicting identity, or unconfirmed overlap.
    #[expect(
        clippy::too_many_arguments,
        reason = "fields mirror the frozen create-commitment command"
    )]
    pub fn create_commitment(
        &mut self,
        id: CommitmentId,
        title: impl Into<String>,
        anchor_date: LocalDate,
        window: TimeWindow,
        note: Option<String>,
        recurrence: Option<WeeklyRecurrence>,
        conflict: ConflictDecision,
    ) -> Result<Commitment, ScheduleError> {
        let title = normalize_required(title.into())?;
        let note = normalize_optional(note)?;
        if let Some(existing) = self.commitments.get(&id) {
            if existing.title == title
                && existing.anchor_date == anchor_date
                && existing.window == window
                && existing.note == note
                && existing.recurrence == recurrence
            {
                return Ok(existing.clone());
            }
            return Err(ScheduleError::ConflictingCreate);
        }
        let candidate = Commitment {
            id,
            revision: 0,
            title,
            anchor_date,
            window,
            note,
            recurrence,
            removed: false,
            exceptions: BTreeMap::new(),
        };
        if conflict == ConflictDecision::Reject && self.conflicts_with(&candidate, None) {
            return Err(ScheduleError::OverlapRequiresConfirmation);
        }
        self.commitments.insert(id, candidate.clone());
        Ok(candidate)
    }

    /// Changes one occurrence or the entire commitment series.
    ///
    /// # Errors
    /// Returns [`ScheduleError`] for missing data, ambiguity, stale revisions, invalid text, or unconfirmed overlap.
    pub fn change_commitment(
        &mut self,
        id: CommitmentId,
        expected_revision: u64,
        scope: RecurringScope,
        change: CommitmentChange,
        conflict: ConflictDecision,
    ) -> Result<Commitment, ScheduleError> {
        let change = CommitmentChange {
            title: normalize_required(change.title)?,
            window: change.window,
            note: normalize_optional(change.note)?,
        };
        let existing = self.commitments.get(&id).ok_or(ScheduleError::NotFound)?;
        validate_scope(existing, scope)?;
        if change_matches(existing, scope, &change) {
            return Ok(existing.clone());
        }
        ensure_revision(existing.revision, expected_revision)?;
        let mut candidate = existing.clone();
        match scope {
            RecurringScope::Series => {
                candidate.title = change.title;
                candidate.window = change.window;
                candidate.note = change.note;
            }
            RecurringScope::Occurrence(date) => {
                candidate.exceptions.insert(
                    date,
                    OccurrenceException::Changed {
                        title: change.title,
                        window: change.window,
                        note: change.note,
                    },
                );
            }
        }
        if conflict == ConflictDecision::Reject && self.conflicts_with(&candidate, Some(id)) {
            return Err(ScheduleError::OverlapRequiresConfirmation);
        }
        candidate.revision += 1;
        self.commitments.insert(id, candidate.clone());
        Ok(candidate)
    }

    /// Removes one recurring occurrence or the complete commitment.
    ///
    /// # Errors
    /// Returns [`ScheduleError`] for a missing commitment, ambiguous scope, or stale revision.
    pub fn remove_commitment(
        &mut self,
        id: CommitmentId,
        expected_revision: u64,
        scope: RecurringScope,
    ) -> Result<Commitment, ScheduleError> {
        let existing = self.commitments.get(&id).ok_or(ScheduleError::NotFound)?;
        validate_scope(existing, scope)?;
        let already_removed = match scope {
            RecurringScope::Series => existing.removed,
            RecurringScope::Occurrence(date) => {
                existing.exceptions.get(&date) == Some(&OccurrenceException::Removed)
            }
        };
        if already_removed {
            return Ok(existing.clone());
        }
        ensure_revision(existing.revision, expected_revision)?;
        let commitment = self
            .commitments
            .get_mut(&id)
            .ok_or(ScheduleError::NotFound)?;
        match scope {
            RecurringScope::Series => commitment.removed = true,
            RecurringScope::Occurrence(date) => {
                commitment
                    .exceptions
                    .insert(date, OccurrenceException::Removed);
            }
        }
        commitment.revision += 1;
        Ok(commitment.clone())
    }

    /// Replaces the selected weekdays of a recurring series.
    ///
    /// # Errors
    /// Returns [`ScheduleError`] for a missing/non-recurring commitment, stale revision, or unconfirmed overlap.
    pub fn change_recurrence(
        &mut self,
        id: CommitmentId,
        expected_revision: u64,
        recurrence: WeeklyRecurrence,
        conflict: ConflictDecision,
    ) -> Result<Commitment, ScheduleError> {
        let existing = self.commitments.get(&id).ok_or(ScheduleError::NotFound)?;
        if existing.recurrence.is_none() {
            return Err(ScheduleError::ScopeRequired);
        }
        if existing.recurrence.as_ref() == Some(&recurrence) {
            return Ok(existing.clone());
        }
        ensure_revision(existing.revision, expected_revision)?;
        let mut candidate = existing.clone();
        candidate.recurrence = Some(recurrence);
        if conflict == ConflictDecision::Reject && self.conflicts_with(&candidate, Some(id)) {
            return Err(ScheduleError::OverlapRequiresConfirmation);
        }
        candidate.revision += 1;
        self.commitments.insert(id, candidate.clone());
        Ok(candidate)
    }

    /// Configures or removes the default window for a weekday.
    ///
    /// # Errors
    /// Returns [`ScheduleError::RevisionConflict`] for an obsolete change.
    pub fn configure_weekday_availability(
        &mut self,
        weekday: Weekday,
        window: Option<TimeWindow>,
        expected_revision: u64,
    ) -> Result<u64, ScheduleError> {
        if self.weekly_availability.get(&weekday).copied() == window {
            return Ok(self.availability_revision);
        }
        ensure_revision(self.availability_revision, expected_revision)?;
        if let Some(window) = window {
            self.weekly_availability.insert(weekday, window);
        } else {
            self.weekly_availability.remove(&weekday);
        }
        self.availability_revision += 1;
        Ok(self.availability_revision)
    }

    /// Configures a date-specific window or explicit unavailability.
    ///
    /// # Errors
    /// Returns [`ScheduleError::RevisionConflict`] for an obsolete change.
    pub fn configure_availability_exception(
        &mut self,
        date: LocalDate,
        value: AvailabilityOverride,
        expected_revision: u64,
    ) -> Result<u64, ScheduleError> {
        if self.availability_exceptions.get(&date).copied() == Some(value) {
            return Ok(self.availability_revision);
        }
        ensure_revision(self.availability_revision, expected_revision)?;
        self.availability_exceptions.insert(date, value);
        self.availability_revision += 1;
        Ok(self.availability_revision)
    }

    /// Removes a date exception so the weekday default applies again.
    ///
    /// # Errors
    /// Returns [`ScheduleError::RevisionConflict`] for an obsolete change.
    pub fn clear_availability_exception(
        &mut self,
        date: LocalDate,
        expected_revision: u64,
    ) -> Result<u64, ScheduleError> {
        if !self.availability_exceptions.contains_key(&date) {
            return Ok(self.availability_revision);
        }
        ensure_revision(self.availability_revision, expected_revision)?;
        self.availability_exceptions.remove(&date);
        self.availability_revision += 1;
        Ok(self.availability_revision)
    }

    /// Returns a commitment aggregate snapshot.
    #[must_use]
    pub fn commitment(&self, id: CommitmentId) -> Option<Commitment> {
        self.commitments
            .get(&id)
            .filter(|item| !item.removed)
            .cloned()
    }

    /// Materializes occurrences in an inclusive date period.
    ///
    /// # Errors
    /// Returns [`ScheduleError::InvalidPeriod`] when the end precedes the start.
    pub fn occurrences(
        &self,
        start: LocalDate,
        end: LocalDate,
    ) -> Result<Vec<CommitmentOccurrence>, ScheduleError> {
        if end < start {
            return Err(ScheduleError::InvalidPeriod);
        }
        let mut result = Vec::new();
        let mut date = start;
        loop {
            for commitment in self.commitments.values() {
                if let Some(occurrence) = occurrence_on(commitment, date) {
                    result.push(occurrence);
                }
            }
            if date == end {
                break;
            }
            date = date.next();
        }
        result.sort_by_key(|item| (item.date, item.window.start().value(), item.commitment_id));
        Ok(result)
    }

    /// Derives free windows by subtracting fixed commitments from configured availability.
    ///
    /// # Errors
    /// Returns [`ScheduleError`] only if the one-day occurrence query cannot be evaluated.
    pub fn availability(&self, date: LocalDate) -> Result<DailyAvailability, ScheduleError> {
        let base = match self.availability_exceptions.get(&date).copied() {
            Some(AvailabilityOverride::Window(window)) => Some(window),
            Some(AvailabilityOverride::Unavailable) => {
                return Ok(DailyAvailability {
                    date,
                    windows: Vec::new(),
                    restrictive_commitments: Vec::new(),
                    complete: true,
                });
            }
            None => self.weekly_availability.get(&date.weekday()).copied(),
        };
        let Some(base) = base else {
            return Ok(DailyAvailability {
                date,
                windows: Vec::new(),
                restrictive_commitments: Vec::new(),
                complete: false,
            });
        };
        let restrictive_commitments = self
            .occurrences(date, date)?
            .into_iter()
            .filter(|item| item.window.overlaps(base))
            .collect::<Vec<_>>();
        let mut windows = vec![base];
        for commitment in &restrictive_commitments {
            windows = subtract(windows, commitment.window);
        }
        Ok(DailyAvailability {
            date,
            windows,
            restrictive_commitments,
            complete: true,
        })
    }

    fn conflicts_with(&self, candidate: &Commitment, ignored: Option<CommitmentId>) -> bool {
        self.commitments
            .values()
            .filter(|other| Some(other.id) != ignored && !other.removed)
            .any(|other| {
                commitments_can_overlap(candidate, other)
                    || candidate.exceptions.iter().any(|(date, exception)| {
                        matches!(exception, OccurrenceException::Changed { .. })
                            && occurrence_on(candidate, *date).is_some_and(|left| {
                                occurrence_on(other, *date)
                                    .is_some_and(|right| left.window.overlaps(right.window))
                            })
                    })
            })
    }
}

/// Deterministic schedule failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleError {
    /// A Gregorian date is impossible.
    InvalidDate,
    /// A time interval is empty or reversed.
    InvalidTimeWindow,
    /// Required text is empty or contains control characters.
    InvalidText,
    /// A weekly recurrence has no selected day.
    EmptyRecurrence,
    /// A queried period is reversed.
    InvalidPeriod,
    /// The requested commitment does not exist.
    NotFound,
    /// An identifier was replayed with conflicting creation data.
    ConflictingCreate,
    /// An overlapping fixed commitment requires explicit confirmation.
    OverlapRequiresConfirmation,
    /// A recurring operation did not identify occurrence or series correctly.
    ScopeRequired,
    /// A caller attempted to alter an obsolete snapshot.
    RevisionConflict {
        /// Revision supplied by the caller.
        expected: u64,
        /// Current aggregate revision.
        actual: u64,
    },
}

fn validate_scope(commitment: &Commitment, scope: RecurringScope) -> Result<(), ScheduleError> {
    match (commitment.recurrence.is_some(), scope) {
        (false, RecurringScope::Occurrence(_)) => Err(ScheduleError::ScopeRequired),
        (true, RecurringScope::Occurrence(date)) if occurrence_on(commitment, date).is_none() => {
            Err(ScheduleError::NotFound)
        }
        _ => Ok(()),
    }
}

fn change_matches(
    commitment: &Commitment,
    scope: RecurringScope,
    change: &CommitmentChange,
) -> bool {
    match scope {
        RecurringScope::Series => {
            commitment.title == change.title
                && commitment.window == change.window
                && commitment.note == change.note
        }
        RecurringScope::Occurrence(date) => match commitment.exceptions.get(&date) {
            Some(OccurrenceException::Changed {
                title,
                window,
                note,
            }) => title == &change.title && window == &change.window && note == &change.note,
            _ => false,
        },
    }
}

fn occurrence_on(commitment: &Commitment, date: LocalDate) -> Option<CommitmentOccurrence> {
    if commitment.removed || date < commitment.anchor_date {
        return None;
    }
    let scheduled = commitment
        .recurrence
        .as_ref()
        .map_or(date == commitment.anchor_date, |rule| {
            rule.contains(date.weekday())
        });
    if !scheduled {
        return None;
    }
    let (title, window, note) = match commitment.exceptions.get(&date) {
        Some(OccurrenceException::Removed) => return None,
        Some(OccurrenceException::Changed {
            title,
            window,
            note,
        }) => (title.clone(), *window, note.clone()),
        None => (
            commitment.title.clone(),
            commitment.window,
            commitment.note.clone(),
        ),
    };
    Some(CommitmentOccurrence {
        commitment_id: commitment.id,
        date,
        title,
        window,
        note,
        recurring: commitment.recurrence.is_some(),
    })
}

fn commitments_can_overlap(left: &Commitment, right: &Commitment) -> bool {
    if !left.window.overlaps(right.window) {
        return false;
    }
    match (&left.recurrence, &right.recurrence) {
        (None, None) => left.anchor_date == right.anchor_date,
        (Some(rule), None) => {
            right.anchor_date >= left.anchor_date
                && rule.contains(right.anchor_date.weekday())
                && occurrence_on(left, right.anchor_date).is_some()
        }
        (None, Some(rule)) => {
            left.anchor_date >= right.anchor_date
                && rule.contains(left.anchor_date.weekday())
                && occurrence_on(right, left.anchor_date).is_some()
        }
        (Some(left_rule), Some(right_rule)) => {
            left_rule.weekdays().any(|day| right_rule.contains(day))
        }
    }
}

fn subtract(windows: Vec<TimeWindow>, blocked: TimeWindow) -> Vec<TimeWindow> {
    let mut result = Vec::new();
    for window in windows {
        if !window.overlaps(blocked) {
            result.push(window);
            continue;
        }
        if window.start.value() < blocked.start.value() {
            result.push(TimeWindow {
                start: window.start,
                end: blocked.start,
            });
        }
        if blocked.end.value() < window.end.value() {
            result.push(TimeWindow {
                start: blocked.end,
                end: window.end,
            });
        }
    }
    result
}

fn ensure_revision(actual: u64, expected: u64) -> Result<(), ScheduleError> {
    if actual == expected {
        Ok(())
    } else {
        Err(ScheduleError::RevisionConflict { expected, actual })
    }
}

fn normalize_required(value: String) -> Result<String, ScheduleError> {
    let normalized = value.trim();
    if normalized.is_empty() || normalized.chars().any(char::is_control) {
        return Err(ScheduleError::InvalidText);
    }
    if normalized.len() == value.len() {
        Ok(value)
    } else {
        Ok(normalized.to_owned())
    }
}

fn normalize_optional(value: Option<String>) -> Result<Option<String>, ScheduleError> {
    value
        .map(|value| {
            let normalized = value.trim();
            if normalized.chars().any(char::is_control) {
                Err(ScheduleError::InvalidText)
            } else if normalized.is_empty() {
                Ok(None)
            } else {
                Ok(Some(normalized.to_owned()))
            }
        })
        .transpose()
        .map(Option::flatten)
}

const fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}
const fn is_leap_year(year: u16) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u8) -> LocalDate {
        LocalDate::new(2026, 8, day).expect("valid date")
    }
    fn minute(value: u16) -> MinuteOfDay {
        MinuteOfDay::new(value).expect("valid minute")
    }
    fn window(start: u16, end: u16) -> TimeWindow {
        TimeWindow::new(minute(start), minute(end)).expect("valid window")
    }
    fn id(value: u64) -> CommitmentId {
        CommitmentId::new(value).expect("valid id")
    }

    #[test]
    fn validates_dates_windows_and_weekdays() {
        assert_eq!(LocalDate::new(2025, 2, 29), Err(ScheduleError::InvalidDate));
        assert_eq!(
            LocalDate::new(2024, 2, 29).expect("leap").weekday(),
            Weekday::Thursday
        );
        assert_eq!(
            TimeWindow::new(minute(60), minute(60)),
            Err(ScheduleError::InvalidTimeWindow)
        );
    }

    #[test]
    fn create_change_and_remove_are_revision_safe_and_idempotent() {
        let mut schedule = Schedule::default();
        let created = schedule
            .create_commitment(
                id(1),
                "Class",
                date(6),
                window(600, 660),
                None,
                None,
                ConflictDecision::Reject,
            )
            .expect("create");
        assert_eq!(
            schedule
                .create_commitment(
                    id(1),
                    "Class",
                    date(6),
                    window(600, 660),
                    None,
                    None,
                    ConflictDecision::Reject
                )
                .expect("replay"),
            created
        );
        let changed = schedule
            .change_commitment(
                id(1),
                0,
                RecurringScope::Series,
                CommitmentChange {
                    title: "Lecture".to_owned(),
                    window: window(600, 660),
                    note: None,
                },
                ConflictDecision::Reject,
            )
            .expect("change");
        assert_eq!(
            schedule
                .change_commitment(
                    id(1),
                    0,
                    RecurringScope::Series,
                    CommitmentChange {
                        title: "Lecture".to_owned(),
                        window: window(600, 660),
                        note: None
                    },
                    ConflictDecision::Reject
                )
                .expect("replay"),
            changed
        );
        assert_eq!(
            schedule.remove_commitment(id(1), 0, RecurringScope::Series),
            Err(ScheduleError::RevisionConflict {
                expected: 0,
                actual: 1
            })
        );
        schedule
            .remove_commitment(id(1), 1, RecurringScope::Series)
            .expect("remove");
        assert!(schedule.commitment(id(1)).is_none());
    }

    #[test]
    fn overlap_requires_explicit_confirmation() {
        let mut schedule = Schedule::default();
        schedule
            .create_commitment(
                id(1),
                "Class",
                date(6),
                window(600, 660),
                None,
                None,
                ConflictDecision::Reject,
            )
            .expect("first");
        assert_eq!(
            schedule.create_commitment(
                id(2),
                "Call",
                date(6),
                window(630, 690),
                None,
                None,
                ConflictDecision::Reject
            ),
            Err(ScheduleError::OverlapRequiresConfirmation)
        );
        schedule
            .create_commitment(
                id(2),
                "Call",
                date(6),
                window(630, 690),
                None,
                None,
                ConflictDecision::Confirm,
            )
            .expect("confirmed");
    }

    #[test]
    fn weekly_recurrence_requires_explicit_occurrence_scope() {
        let mut schedule = Schedule::default();
        let rule = WeeklyRecurrence::new([Weekday::Thursday]).expect("rule");
        schedule
            .create_commitment(
                id(1),
                "Class",
                date(6),
                window(600, 660),
                None,
                Some(rule),
                ConflictDecision::Reject,
            )
            .expect("series");
        assert_eq!(
            schedule.change_commitment(
                id(1),
                0,
                RecurringScope::Occurrence(date(7)),
                CommitmentChange {
                    title: "Other".to_owned(),
                    window: window(600, 660),
                    note: None
                },
                ConflictDecision::Reject
            ),
            Err(ScheduleError::NotFound)
        );
        let changed = schedule
            .change_commitment(
                id(1),
                0,
                RecurringScope::Occurrence(date(13)),
                CommitmentChange {
                    title: "Special class".to_owned(),
                    window: window(720, 780),
                    note: None,
                },
                ConflictDecision::Reject,
            )
            .expect("occurrence");
        assert_eq!(
            schedule.occurrences(date(13), date(13)).expect("query")[0].title(),
            "Special class"
        );
        schedule
            .remove_commitment(
                id(1),
                changed.revision(),
                RecurringScope::Occurrence(date(20)),
            )
            .expect("remove occurrence");
        assert!(
            schedule
                .occurrences(date(20), date(20))
                .expect("query")
                .is_empty()
        );
    }

    #[test]
    fn changed_occurrence_cannot_create_a_silent_overlap() {
        let mut schedule = Schedule::default();
        let rule = WeeklyRecurrence::new([Weekday::Thursday]).expect("rule");
        schedule
            .create_commitment(
                id(1),
                "Class",
                date(6),
                window(600, 660),
                None,
                Some(rule),
                ConflictDecision::Reject,
            )
            .expect("series");
        schedule
            .create_commitment(
                id(2),
                "Call",
                date(13),
                window(720, 780),
                None,
                None,
                ConflictDecision::Reject,
            )
            .expect("call");

        assert_eq!(
            schedule.change_commitment(
                id(1),
                0,
                RecurringScope::Occurrence(date(13)),
                CommitmentChange {
                    title: "Special class".to_owned(),
                    window: window(750, 810),
                    note: None
                },
                ConflictDecision::Reject,
            ),
            Err(ScheduleError::OverlapRequiresConfirmation)
        );
    }
    #[test]
    fn availability_subtracts_commitments_and_marks_missing_configuration() {
        let mut schedule = Schedule::default();
        assert!(
            !schedule
                .availability(date(6))
                .expect("missing")
                .is_complete()
        );
        schedule
            .configure_weekday_availability(Weekday::Thursday, Some(window(540, 1020)), 0)
            .expect("configure");
        schedule
            .create_commitment(
                id(1),
                "Class",
                date(6),
                window(600, 660),
                None,
                None,
                ConflictDecision::Reject,
            )
            .expect("commitment");
        schedule
            .create_commitment(
                id(2),
                "Call",
                date(6),
                window(900, 960),
                None,
                None,
                ConflictDecision::Reject,
            )
            .expect("commitment");
        let result = schedule.availability(date(6)).expect("availability");
        assert_eq!(
            result.windows(),
            &[window(540, 600), window(660, 900), window(960, 1020)]
        );
        assert_eq!(result.restrictive_commitments().len(), 2);
    }

    #[test]
    fn date_exception_overrides_and_can_restore_weekday_default() {
        let mut schedule = Schedule::default();
        let revision = schedule
            .configure_weekday_availability(Weekday::Thursday, Some(window(540, 1020)), 0)
            .expect("weekday");
        let revision = schedule
            .configure_availability_exception(date(6), AvailabilityOverride::Unavailable, revision)
            .expect("exception");
        assert!(
            schedule
                .availability(date(6))
                .expect("unavailable")
                .windows()
                .is_empty()
        );
        schedule
            .clear_availability_exception(date(6), revision)
            .expect("clear");
        assert_eq!(
            schedule.availability(date(6)).expect("default").windows(),
            &[window(540, 1020)]
        );
    }
}
