//! Public contract tests for deterministic daily planning and explicit approval.

use second_brain_application::{
    ActionsAndProjects, ApprovalKey, ApprovalSelection, ContextFingerprint, DailyPlanning,
    DecisionEngine, DraftId, LocalDate, MinuteOfDay, PlanId, PlanningError, Schedule, TaskDuration,
    TaskId, TimeWindow, Weekday,
};

fn task_id(value: u64) -> TaskId {
    TaskId::new(value).expect("task id")
}

#[test]
fn public_contract_supports_proposal_adjustment_and_partial_approval() {
    let mut actions = ActionsAndProjects::default();
    for value in 1..=4 {
        actions
            .create_task(task_id(value), format!("Task {value}"))
            .expect("task");
        actions
            .set_task_duration(task_id(value), 0, TaskDuration::new(30))
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
        &schedule.availability(day).expect("derived availability"),
    );
    let mut planning = DailyPlanning::default();
    let draft = planning
        .create_draft(
            DraftId::new(1).expect("draft id"),
            day,
            ContextFingerprint::new(11),
            &proposal,
        )
        .expect("draft");
    let adjusted = planning
        .adjust_draft(draft.id(), 0, vec![task_id(4), task_id(2)])
        .expect("adjust");
    let plan = planning
        .approve(
            draft.id(),
            adjusted.revision(),
            ContextFingerprint::new(11),
            PlanId::new(1).expect("plan id"),
            ApprovalKey::new(1).expect("approval key"),
            ApprovalSelection::All,
        )
        .expect("approve");
    assert_eq!(plan.priorities(), &[task_id(4), task_id(2)]);
    assert_eq!(planning.active_plan(day), Some(plan));
}

#[test]
fn public_contract_preserves_incomplete_and_rejected_drafts() {
    let mut actions = ActionsAndProjects::default();
    actions
        .create_task(task_id(1), "Missing duration")
        .expect("task");
    let day = LocalDate::new(2026, 8, 6).expect("day");
    let availability = Schedule::default().availability(day).expect("availability");
    let proposal = DecisionEngine::propose(&actions.eligible_tasks(), &availability);
    assert!(!proposal.context_complete());
    assert_eq!(proposal.missing_duration(), &[task_id(1)]);
    let mut planning = DailyPlanning::default();
    let draft = planning
        .create_draft(
            DraftId::new(1).expect("draft id"),
            day,
            ContextFingerprint::new(3),
            &proposal,
        )
        .expect("draft");
    let rejected = planning.reject(draft.id(), 0).expect("reject");
    assert_eq!(planning.draft(draft.id()), Some(rejected));
    assert_eq!(
        planning.approve(
            draft.id(),
            1,
            ContextFingerprint::new(3),
            PlanId::new(1).expect("plan id"),
            ApprovalKey::new(1).expect("approval key"),
            ApprovalSelection::All
        ),
        Err(PlanningError::DraftNotPending)
    );
}
