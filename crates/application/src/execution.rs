//! Focused execution of one approved daily plan without implicit replanning.

use std::collections::BTreeSet;

use crate::{DailyPlan, LocalDate, PlanId, TaskId};

/// State of the current focus session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusState {
    /// Work is in progress.
    Active,
    /// Work was explicitly paused.
    Paused,
}

/// Relevant reason for requesting a new plan proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplanReason {
    /// The current priority was explicitly postponed.
    PriorityPostponed,
    /// Every priority in the plan was finished.
    PlanExhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Immutable request for deterministic replanning.
pub struct ReplanSignal {
    day: LocalDate,
    reason: ReplanReason,
}
impl ReplanSignal {
    #[must_use]
    /// Returns this value.
    pub const fn day(&self) -> LocalDate {
        self.day
    }
    #[must_use]
    /// Returns this value.
    pub const fn reason(&self) -> ReplanReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Current user-controlled focus session.
pub struct FocusSession {
    task_id: TaskId,
    state: FocusState,
    revision: u64,
}
impl FocusSession {
    #[must_use]
    /// Returns this value.
    pub const fn task_id(&self) -> TaskId {
        self.task_id
    }
    #[must_use]
    /// Returns this value.
    pub const fn state(&self) -> FocusState {
        self.state
    }
    #[must_use]
    /// Returns this value.
    pub const fn revision(&self) -> u64 {
        self.revision
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Read model for the post-approval Agora state.
pub struct NowSnapshot {
    day: LocalDate,
    plan_id: PlanId,
    current: Option<TaskId>,
    remaining: Vec<TaskId>,
    session: Option<FocusSession>,
    revision: u64,
}
impl NowSnapshot {
    #[must_use]
    /// Returns this value.
    pub const fn day(&self) -> LocalDate {
        self.day
    }
    #[must_use]
    /// Returns this value.
    pub const fn plan_id(&self) -> PlanId {
        self.plan_id
    }
    #[must_use]
    /// Returns this value.
    pub const fn current(&self) -> Option<TaskId> {
        self.current
    }
    #[must_use]
    /// Returns the current snapshot value.
    pub fn remaining(&self) -> &[TaskId] {
        &self.remaining
    }
    #[must_use]
    /// Returns the current snapshot value.
    pub fn session(&self) -> Option<&FocusSession> {
        self.session.as_ref()
    }
    #[must_use]
    /// Returns this value.
    pub const fn revision(&self) -> u64 {
        self.revision
    }
}

#[derive(Debug, Clone)]
struct ActiveExecution {
    day: LocalDate,
    plan_id: PlanId,
    remaining: Vec<TaskId>,
    completed: BTreeSet<TaskId>,
    postponed: BTreeSet<TaskId>,
    session: Option<FocusSession>,
    revision: u64,
}

#[derive(Debug, Default)]
/// Owner of the active-plan execution state.
pub struct Execution {
    active: Option<ActiveExecution>,
}

impl Execution {
    /// Activates an approved plan. Exact replays are idempotent.
    /// # Errors
    /// Returns an active-plan conflict for a different plan.
    pub fn activate(&mut self, plan: &DailyPlan) -> Result<NowSnapshot, ExecutionError> {
        if let Some(active) = &self.active {
            return if active.plan_id == plan.id() && active.day == plan.day() {
                Ok(snapshot(active))
            } else {
                Err(ExecutionError::ActivePlanConflict)
            };
        }
        let active = ActiveExecution {
            day: plan.day(),
            plan_id: plan.id(),
            remaining: plan.priorities().to_vec(),
            completed: BTreeSet::new(),
            postponed: BTreeSet::new(),
            session: None,
            revision: 0,
        };
        let result = snapshot(&active);
        self.active = Some(active);
        Ok(result)
    }

    /// Starts focus only on the task currently presented by Agora.
    /// # Errors
    /// Returns a deterministic state, priority, or revision failure.
    pub fn start(
        &mut self,
        task: TaskId,
        expected_revision: u64,
    ) -> Result<NowSnapshot, ExecutionError> {
        let active = self.active.as_mut().ok_or(ExecutionError::NoActivePlan)?;
        if let Some(session) = &active.session {
            return if session.task_id == task && session.state == FocusState::Active {
                Ok(snapshot(active))
            } else {
                Err(ExecutionError::SessionAlreadyOpen)
            };
        }
        ensure_revision(active.revision, expected_revision)?;
        if active.remaining.first().copied() != Some(task) {
            return Err(ExecutionError::NotCurrentPriority);
        }
        active.session = Some(FocusSession {
            task_id: task,
            state: FocusState::Active,
            revision: 0,
        });
        active.revision += 1;
        Ok(snapshot(active))
    }

    /// Pauses the session.
    /// # Errors
    /// Returns a deterministic session or revision failure.
    pub fn pause(&mut self, expected_revision: u64) -> Result<NowSnapshot, ExecutionError> {
        self.change_focus_state(expected_revision, FocusState::Paused)
    }
    /// Resumes the session.
    /// # Errors
    /// Returns a deterministic session or revision failure.
    pub fn resume(&mut self, expected_revision: u64) -> Result<NowSnapshot, ExecutionError> {
        self.change_focus_state(expected_revision, FocusState::Active)
    }

    /// Completes the current item and advances Agora without interrupting another session.
    /// # Errors
    /// Returns a deterministic state, priority, or revision failure.
    pub fn complete(
        &mut self,
        task: TaskId,
        expected_revision: u64,
    ) -> Result<(NowSnapshot, Option<ReplanSignal>), ExecutionError> {
        self.finish(task, expected_revision, false)
    }
    /// Explicitly postpones the current item and requests replanning.
    /// # Errors
    /// Returns a deterministic state, priority, or revision failure.
    pub fn postpone(
        &mut self,
        task: TaskId,
        expected_revision: u64,
    ) -> Result<(NowSnapshot, Option<ReplanSignal>), ExecutionError> {
        self.finish(task, expected_revision, true)
    }
    #[must_use]
    /// Returns the current snapshot value.
    pub fn now(&self) -> Option<NowSnapshot> {
        self.active.as_ref().map(snapshot)
    }

    fn change_focus_state(
        &mut self,
        expected_revision: u64,
        destination: FocusState,
    ) -> Result<NowSnapshot, ExecutionError> {
        let active = self.active.as_mut().ok_or(ExecutionError::NoActivePlan)?;
        let session = active
            .session
            .as_mut()
            .ok_or(ExecutionError::NoOpenSession)?;
        if session.state == destination {
            return Ok(snapshot(active));
        }
        ensure_revision(active.revision, expected_revision)?;
        session.state = destination;
        session.revision += 1;
        active.revision += 1;
        Ok(snapshot(active))
    }

    fn finish(
        &mut self,
        task: TaskId,
        expected_revision: u64,
        postpone: bool,
    ) -> Result<(NowSnapshot, Option<ReplanSignal>), ExecutionError> {
        let active = self.active.as_mut().ok_or(ExecutionError::NoActivePlan)?;
        let already_done = if postpone {
            active.postponed.contains(&task)
        } else {
            active.completed.contains(&task)
        };
        if already_done {
            return Ok((snapshot(active), None));
        }
        ensure_revision(active.revision, expected_revision)?;
        if active.remaining.first().copied() != Some(task) {
            return Err(ExecutionError::NotCurrentPriority);
        }
        if active
            .session
            .as_ref()
            .is_some_and(|session| session.task_id != task)
        {
            return Err(ExecutionError::SessionAlreadyOpen);
        }
        active.remaining.remove(0);
        active.session = None;
        if postpone {
            active.postponed.insert(task);
        } else {
            active.completed.insert(task);
        }
        active.revision += 1;
        let reason = if postpone {
            Some(ReplanReason::PriorityPostponed)
        } else if active.remaining.is_empty() {
            Some(ReplanReason::PlanExhausted)
        } else {
            None
        };
        Ok((
            snapshot(active),
            reason.map(|reason| ReplanSignal {
                day: active.day,
                reason,
            }),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Deterministic execution failure.
pub enum ExecutionError {
    /// No approved plan was activated.
    NoActivePlan,
    /// A different plan is already active.
    ActivePlanConflict,
    /// Another focus session is already open.
    SessionAlreadyOpen,
    /// No focus session can be paused or resumed.
    NoOpenSession,
    /// The task is not the item currently shown by Agora.
    NotCurrentPriority,
    /// The caller changed an obsolete snapshot.
    RevisionConflict {
        /// Revision supplied by the caller.
        expected: u64,
        /// Current revision.
        actual: u64,
    },
}

fn ensure_revision(actual: u64, expected: u64) -> Result<(), ExecutionError> {
    if actual == expected {
        Ok(())
    } else {
        Err(ExecutionError::RevisionConflict { expected, actual })
    }
}
fn snapshot(active: &ActiveExecution) -> NowSnapshot {
    NowSnapshot {
        day: active.day,
        plan_id: active.plan_id,
        current: active.remaining.first().copied(),
        remaining: active.remaining.clone(),
        session: active.session.clone(),
        revision: active.revision,
    }
}
