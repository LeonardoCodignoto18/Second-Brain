//! Public semantic contract tests for Agenda e Disponibilidade.

use second_brain_application::{
    AvailabilityOverride, CommitmentId, ConflictDecision, LocalDate, MinuteOfDay, RecurringScope,
    Schedule, ScheduleError, TimeWindow, Weekday, WeeklyRecurrence,
};

fn date(day: u8) -> LocalDate {
    LocalDate::new(2026, 8, day).expect("valid date")
}
fn window(start: u16, end: u16) -> TimeWindow {
    TimeWindow::new(
        MinuteOfDay::new(start).expect("start"),
        MinuteOfDay::new(end).expect("end"),
    )
    .expect("window")
}
fn id(value: u64) -> CommitmentId {
    CommitmentId::new(value).expect("identifier")
}

#[test]
fn public_contract_derives_availability_from_window_commitments_and_exception() {
    let mut schedule = Schedule::default();
    let revision = schedule
        .configure_weekday_availability(Weekday::Thursday, Some(window(540, 1020)), 0)
        .expect("weekday");
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
    assert_eq!(
        schedule
            .availability(date(6))
            .expect("availability")
            .windows(),
        &[window(540, 600), window(660, 1020)]
    );
    schedule
        .configure_availability_exception(date(13), AvailabilityOverride::Unavailable, revision)
        .expect("exception");
    let unavailable = schedule.availability(date(13)).expect("unavailable");
    assert!(unavailable.is_complete());
    assert!(unavailable.windows().is_empty());
}

#[test]
fn public_contract_requires_scope_and_rejects_obsolete_recurring_changes() {
    let mut schedule = Schedule::default();
    let recurrence = WeeklyRecurrence::new([Weekday::Thursday]).expect("recurrence");
    let created = schedule
        .create_commitment(
            id(1),
            "Class",
            date(6),
            window(600, 660),
            None,
            Some(recurrence.clone()),
            ConflictDecision::Reject,
        )
        .expect("series");
    let unchanged = schedule
        .change_recurrence(
            id(1),
            created.revision(),
            recurrence,
            ConflictDecision::Reject,
        )
        .expect("replay");
    schedule
        .remove_commitment(
            id(1),
            unchanged.revision(),
            RecurringScope::Occurrence(date(13)),
        )
        .expect("remove");
    assert_eq!(
        schedule.remove_commitment(id(1), 0, RecurringScope::Occurrence(date(20))),
        Err(ScheduleError::RevisionConflict {
            expected: 0,
            actual: 1
        })
    );
}
