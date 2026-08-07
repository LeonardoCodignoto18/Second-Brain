//! Public contract tests for the post-approval Agora and focus lifecycle.

use second_brain_application::{
    ActionsAndProjects, ApprovalKey, ApprovalSelection, ContextFingerprint, DailyPlanning,
    DecisionEngine, DraftId, Execution, ExecutionError, FocusState, LocalDate, MinuteOfDay, PlanId,
    ReplanReason, Schedule, TaskDuration, TaskId, TimeWindow, Weekday,
};

fn id(value: u64) -> TaskId {
    TaskId::new(value).expect("task")
}

fn approved_plan() -> second_brain_application::DailyPlan {
    let mut actions = ActionsAndProjects::default();
    for value in 1..=2 {
        actions
            .create_task(id(value), format!("Task {value}"))
            .expect("task");
        actions
            .set_task_duration(id(value), 0, TaskDuration::new(30))
            .expect("duration");
    }
    let day = LocalDate::new(2026, 8, 6).expect("day");
    let mut schedule = Schedule::default();
    schedule
        .configure_weekday_availability(
            Weekday::Thursday,
            Some(
                TimeWindow::new(
                    MinuteOfDay::new(540).expect("start"),
                    MinuteOfDay::new(660).expect("end"),
                )
                .expect("window"),
            ),
            0,
        )
        .expect("availability");
    let proposal = DecisionEngine::propose(
        &actions.eligible_tasks(),
        &schedule.availability(day).expect("availability"),
    );
    let mut planning = DailyPlanning::default();
    let draft = planning
        .create_draft(
            DraftId::new(1).expect("draft"),
            day,
            ContextFingerprint::new(1),
            &proposal,
        )
        .expect("draft");
    planning
        .approve(
            draft.id(),
            0,
            ContextFingerprint::new(1),
            PlanId::new(1).expect("plan"),
            ApprovalKey::new(1).expect("key"),
            ApprovalSelection::All,
        )
        .expect("plan")
}

#[test]
fn public_contract_advances_now_and_requests_only_relevant_replanning() {
    let plan = approved_plan();
    let mut execution = Execution::default();
    let activated = execution.activate(&plan).expect("activate");
    assert_eq!(activated.current(), Some(id(1)));
    let active = execution.start(id(1), 0).expect("start");
    assert_eq!(
        active.session().expect("session").state(),
        FocusState::Active
    );
    let paused = execution.pause(1).expect("pause");
    assert_eq!(
        paused.session().expect("session").state(),
        FocusState::Paused
    );
    let resumed = execution.resume(2).expect("resume");
    let (advanced, signal) = execution
        .complete(id(1), resumed.revision())
        .expect("complete");
    assert_eq!(advanced.current(), Some(id(2)));
    assert_eq!(
        execution.activate(&plan).expect("activation replay"),
        advanced
    );
    assert_eq!(signal, None);
    let (empty, signal) = execution
        .postpone(id(2), advanced.revision())
        .expect("postpone");
    assert_eq!(empty.current(), None);
    assert_eq!(
        signal.expect("signal").reason(),
        ReplanReason::PriorityPostponed
    );
}

#[test]
fn public_contract_rejects_obsolete_or_non_current_execution() {
    let plan = approved_plan();
    let mut execution = Execution::default();
    execution.activate(&plan).expect("activate");
    execution.start(id(1), 0).expect("start");
    assert_eq!(
        execution.pause(0),
        Err(ExecutionError::RevisionConflict {
            expected: 0,
            actual: 1
        })
    );
    assert_eq!(
        execution.start(id(2), 1),
        Err(ExecutionError::SessionAlreadyOpen)
    );
}

#[test]
fn public_contract_requires_and_applies_explicit_replanning() {
    let first = approved_plan();
    let mut execution = Execution::default();
    let active = execution.activate(&first).expect("activate");
    let (pending, signal) = execution
        .postpone(id(1), active.revision())
        .expect("postpone");
    assert_eq!(
        pending.replan_reason(),
        Some(ReplanReason::PriorityPostponed)
    );
    assert!(signal.is_some());

    let mut actions = ActionsAndProjects::default();
    actions.create_task(id(3), "Replacement").expect("task");
    actions
        .set_task_duration(id(3), 0, TaskDuration::new(30))
        .expect("duration");
    let day = LocalDate::new(2026, 8, 6).expect("day");
    let mut schedule = Schedule::default();
    schedule
        .configure_weekday_availability(
            Weekday::Thursday,
            Some(
                TimeWindow::new(
                    MinuteOfDay::new(540).expect("start"),
                    MinuteOfDay::new(660).expect("end"),
                )
                .expect("window"),
            ),
            0,
        )
        .expect("availability");
    let proposal = DecisionEngine::propose(
        &actions.eligible_tasks(),
        &schedule.availability(day).expect("availability"),
    );
    let mut planning = DailyPlanning::default();
    let draft = planning
        .create_draft(
            DraftId::new(2).expect("draft"),
            day,
            ContextFingerprint::new(2),
            &proposal,
        )
        .expect("draft");
    let replacement = planning
        .approve(
            draft.id(),
            0,
            ContextFingerprint::new(2),
            PlanId::new(2).expect("plan"),
            ApprovalKey::new(2).expect("key"),
            ApprovalSelection::All,
        )
        .expect("plan");
    let replanned = execution
        .replan(&replacement, pending.revision())
        .expect("replan");
    assert_eq!(replanned.current(), Some(id(3)));
    assert_eq!(replanned.replan_reason(), None);
}
