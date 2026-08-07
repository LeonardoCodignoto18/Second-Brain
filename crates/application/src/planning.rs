//! Draft-first daily planning and explicit user approval.

use std::collections::{BTreeMap, BTreeSet};

use crate::{DeterministicProposal, LocalDate, ProposedPriority, TaskId};

/// Stable draft identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DraftId(u64);
impl DraftId {
    /// Returns the stable local value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
    /// Creates a non-zero identifier.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }
}
/// Stable approved-plan identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlanId(u64);
impl PlanId {
    /// Returns the stable local value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
    /// Creates a non-zero identifier.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }
}
/// Stable idempotency key for an approval decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApprovalKey(u64);
impl ApprovalKey {
    /// Creates a non-zero key.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }
}
/// Fingerprint of the source snapshots used by a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextFingerprint(u64);
impl ContextFingerprint {
    /// Creates a fingerprint.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Lifecycle state of a plan draft.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftState {
    /// Awaiting a user decision.
    Pending,
    /// Explicitly rejected while preserving its content.
    Rejected,
    /// Converted to an approved plan.
    Approved,
}

/// User's atomic approval selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalSelection {
    /// Keep the current draft priorities.
    All,
    /// Approve the supplied subset or substitutions.
    Partial(Vec<TaskId>),
}

/// Plan draft that never has mutation authority by itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanDraft {
    id: DraftId,
    revision: u64,
    day: LocalDate,
    fingerprint: ContextFingerprint,
    priorities: Vec<ProposedPriority>,
    eligible: BTreeSet<TaskId>,
    removed: BTreeSet<TaskId>,
    missing_duration: Vec<TaskId>,
    context_complete: bool,
    state: DraftState,
    source_proposal: DeterministicProposal,
}
impl PlanDraft {
    /// Returns the identifier.
    #[must_use]
    pub const fn id(&self) -> DraftId {
        self.id
    }
    /// Returns the revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }
    /// Returns the operational date.
    #[must_use]
    pub const fn day(&self) -> LocalDate {
        self.day
    }
    /// Returns current proposed priorities.
    #[must_use]
    pub fn priorities(&self) -> &[ProposedPriority] {
        &self.priorities
    }
    /// Returns tasks whose duration is required before reliable planning.
    #[must_use]
    pub fn missing_duration(&self) -> &[TaskId] {
        &self.missing_duration
    }
    /// Returns every task eligible for explicit substitution.
    #[must_use]
    pub fn eligible_tasks(&self) -> impl Iterator<Item = TaskId> + '_ {
        self.eligible.iter().copied()
    }
    /// Returns whether the source context was complete.
    #[must_use]
    pub const fn context_complete(&self) -> bool {
        self.context_complete
    }
    /// Returns the draft state.
    #[must_use]
    pub const fn state(&self) -> DraftState {
        self.state
    }
}

/// Approved daily plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyPlan {
    id: PlanId,
    revision: u64,
    day: LocalDate,
    priorities: Vec<TaskId>,
    source_draft: DraftId,
}
impl DailyPlan {
    /// Returns the identifier.
    #[must_use]
    pub const fn id(&self) -> PlanId {
        self.id
    }
    /// Returns the revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }
    /// Returns the operational date.
    #[must_use]
    pub const fn day(&self) -> LocalDate {
        self.day
    }
    /// Returns zero to three approved priorities.
    #[must_use]
    pub fn priorities(&self) -> &[TaskId] {
        &self.priorities
    }
    /// Returns the source draft.
    #[must_use]
    pub const fn source_draft(&self) -> DraftId {
        self.source_draft
    }
}

/// Owner of draft and approved daily plan state.
#[derive(Debug, Clone, Default)]
pub struct DailyPlanning {
    drafts: BTreeMap<DraftId, PlanDraft>,
    active: BTreeMap<LocalDate, DailyPlan>,
    approvals: BTreeMap<ApprovalKey, (DraftId, DailyPlan)>,
}

impl DailyPlanning {
    /// Creates a draft from a validated deterministic proposal.
    ///
    /// # Errors
    /// Returns [`PlanningError::ConflictingCreate`] for a conflicting identifier replay.
    pub fn create_draft(
        &mut self,
        id: DraftId,
        day: LocalDate,
        fingerprint: ContextFingerprint,
        proposal: &DeterministicProposal,
    ) -> Result<PlanDraft, PlanningError> {
        let draft = PlanDraft {
            id,
            revision: 0,
            day,
            fingerprint,
            priorities: proposal.priorities().to_vec(),
            eligible: proposal.eligible_tasks().iter().copied().collect(),
            removed: BTreeSet::new(),
            missing_duration: proposal.missing_duration().to_vec(),
            context_complete: proposal.context_complete(),
            state: DraftState::Pending,
            source_proposal: proposal.clone(),
        };
        if let Some(existing) = self.drafts.get(&id) {
            return if existing.day == day
                && existing.fingerprint == fingerprint
                && existing.source_proposal == *proposal
            {
                Ok(existing.clone())
            } else {
                Err(PlanningError::ConflictingCreate)
            };
        }
        self.drafts.insert(id, draft.clone());
        Ok(draft)
    }

    /// Adjusts priorities without restarting the planning flow.
    ///
    /// # Errors
    /// Returns [`PlanningError`] for a missing/non-pending draft, obsolete revision, or invalid selection.
    pub fn adjust_draft(
        &mut self,
        id: DraftId,
        expected_revision: u64,
        selected: Vec<TaskId>,
    ) -> Result<PlanDraft, PlanningError> {
        let draft = self
            .drafts
            .get_mut(&id)
            .ok_or(PlanningError::DraftNotFound)?;
        if draft.state != DraftState::Pending {
            return Err(PlanningError::DraftNotPending);
        }
        let current = draft
            .priorities
            .iter()
            .map(ProposedPriority::task_id)
            .collect::<Vec<_>>();
        if current == selected {
            return Ok(draft.clone());
        }
        if draft.revision != expected_revision {
            return Err(PlanningError::RevisionConflict {
                expected: expected_revision,
                actual: draft.revision,
            });
        }
        let selected = validate_selection(draft, selected)?;
        draft
            .removed
            .extend(current.into_iter().filter(|task| !selected.contains(task)));
        draft.priorities = selected
            .into_iter()
            .map(ProposedPriority::deterministic_fit)
            .collect();
        draft.revision += 1;
        Ok(draft.clone())
    }
    /// Approves all, a subset, or valid substitutions atomically.
    ///
    /// # Errors
    /// Returns [`PlanningError`] for missing/rejected/stale drafts, invalid selections, obsolete revisions, duplicate active plans, or conflicting idempotency keys.
    pub fn approve(
        &mut self,
        draft_id: DraftId,
        expected_revision: u64,
        current_fingerprint: ContextFingerprint,
        plan_id: PlanId,
        key: ApprovalKey,
        selection: ApprovalSelection,
    ) -> Result<DailyPlan, PlanningError> {
        self.approve_internal(
            draft_id,
            expected_revision,
            current_fingerprint,
            plan_id,
            key,
            selection,
            None,
        )
    }

    /// Approves a replacement for the currently active plan after an execution trigger.
    ///
    /// # Errors
    /// Returns [`PlanningError`] for the same validation failures as [`Self::approve`],
    /// or when the supplied active plan is obsolete.
    #[expect(
        clippy::too_many_arguments,
        reason = "fields mirror the frozen approval command"
    )]
    pub fn approve_replan(
        &mut self,
        draft_id: DraftId,
        expected_revision: u64,
        current_fingerprint: ContextFingerprint,
        plan_id: PlanId,
        key: ApprovalKey,
        selection: ApprovalSelection,
        replaces: PlanId,
    ) -> Result<DailyPlan, PlanningError> {
        self.approve_internal(
            draft_id,
            expected_revision,
            current_fingerprint,
            plan_id,
            key,
            selection,
            Some(replaces),
        )
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "keeps approval validation atomic"
    )]
    fn approve_internal(
        &mut self,
        draft_id: DraftId,
        expected_revision: u64,
        current_fingerprint: ContextFingerprint,
        plan_id: PlanId,
        key: ApprovalKey,
        selection: ApprovalSelection,
        replaces: Option<PlanId>,
    ) -> Result<DailyPlan, PlanningError> {
        if let Some((used_draft, plan)) = self.approvals.get(&key) {
            return if *used_draft == draft_id && plan.id == plan_id {
                Ok(plan.clone())
            } else {
                Err(PlanningError::IdempotencyConflict)
            };
        }
        let draft = self
            .drafts
            .get(&draft_id)
            .ok_or(PlanningError::DraftNotFound)?;
        if draft.state != DraftState::Pending {
            return Err(PlanningError::DraftNotPending);
        }
        if draft.fingerprint != current_fingerprint {
            return Err(PlanningError::StaleProposal);
        }
        if draft.revision != expected_revision {
            return Err(PlanningError::RevisionConflict {
                expected: expected_revision,
                actual: draft.revision,
            });
        }
        let selected = match selection {
            ApprovalSelection::All => draft
                .priorities
                .iter()
                .map(ProposedPriority::task_id)
                .collect(),
            ApprovalSelection::Partial(tasks) => validate_selection(draft, tasks)?,
        };
        if let Some(active) = self.active.get(&draft.day) {
            if replaces != Some(active.id()) {
                return Err(PlanningError::ActivePlanExists);
            }
        } else if replaces.is_some() {
            return Err(PlanningError::ActivePlanChanged);
        }
        let plan = DailyPlan {
            id: plan_id,
            revision: 0,
            day: draft.day,
            priorities: selected,
            source_draft: draft_id,
        };
        let draft = self
            .drafts
            .get_mut(&draft_id)
            .ok_or(PlanningError::DraftNotFound)?;
        draft.state = DraftState::Approved;
        draft.revision += 1;
        self.active.insert(plan.day, plan.clone());
        self.approvals.insert(key, (draft_id, plan.clone()));
        Ok(plan)
    }

    /// Rejects a draft without changing an approved plan.
    ///
    /// # Errors
    /// Returns [`PlanningError`] for a missing draft or obsolete revision.
    pub fn reject(
        &mut self,
        id: DraftId,
        expected_revision: u64,
    ) -> Result<PlanDraft, PlanningError> {
        let draft = self
            .drafts
            .get_mut(&id)
            .ok_or(PlanningError::DraftNotFound)?;
        if draft.state == DraftState::Rejected {
            return Ok(draft.clone());
        }
        if draft.state != DraftState::Pending {
            return Err(PlanningError::DraftNotPending);
        }
        if draft.revision != expected_revision {
            return Err(PlanningError::RevisionConflict {
                expected: expected_revision,
                actual: draft.revision,
            });
        }
        draft.state = DraftState::Rejected;
        draft.revision += 1;
        Ok(draft.clone())
    }

    /// Returns a preserved draft so the user can decide later.
    #[must_use]
    pub fn draft(&self, id: DraftId) -> Option<PlanDraft> {
        self.drafts.get(&id).cloned()
    }
    /// Returns the single approved plan for a date.
    #[must_use]
    pub fn active_plan(&self, day: LocalDate) -> Option<DailyPlan> {
        self.active.get(&day).cloned()
    }
}

/// Deterministic daily-planning failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanningError {
    /// A draft does not exist.
    DraftNotFound,
    /// An identifier replay conflicts with the original draft.
    ConflictingCreate,
    /// The draft was already approved or rejected.
    DraftNotPending,
    /// Proposal source snapshots changed.
    StaleProposal,
    /// Another active plan already exists for the day.
    ActivePlanExists,
    /// The plan intended for replacement is no longer active.
    ActivePlanChanged,
    /// The approval key was previously used for another decision.
    IdempotencyConflict,
    /// More than three, duplicated, ineligible, or previously removed tasks were selected.
    InvalidSelection,
    /// The caller changed an obsolete draft snapshot.
    RevisionConflict {
        /// Revision supplied by the caller.
        expected: u64,
        /// Current draft revision.
        actual: u64,
    },
}

fn validate_selection(draft: &PlanDraft, tasks: Vec<TaskId>) -> Result<Vec<TaskId>, PlanningError> {
    if tasks.len() > 3 {
        return Err(PlanningError::InvalidSelection);
    }
    let unique = tasks.iter().copied().collect::<BTreeSet<_>>();
    if unique.len() != tasks.len()
        || !unique
            .iter()
            .all(|task| draft.eligible.contains(task) && !draft.removed.contains(task))
    {
        return Err(PlanningError::InvalidSelection);
    }
    Ok(tasks)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u64) -> TaskId {
        TaskId::new(value).expect("task")
    }
    fn proposal() -> DeterministicProposal {
        DeterministicProposal::from_parts_for_planning_tests(
            vec![id(1), id(2), id(3)],
            vec![id(1), id(2), id(3), id(4)],
        )
    }

    #[test]
    fn approves_partial_selection_once_and_replays_by_key() {
        let mut planning = DailyPlanning::default();
        let day = LocalDate::new(2026, 8, 6).expect("day");
        let draft = planning
            .create_draft(
                DraftId::new(1).expect("draft"),
                day,
                ContextFingerprint::new(7),
                &proposal(),
            )
            .expect("draft");
        let plan = planning
            .approve(
                draft.id(),
                0,
                ContextFingerprint::new(7),
                PlanId::new(1).expect("plan"),
                ApprovalKey::new(1).expect("key"),
                ApprovalSelection::Partial(vec![id(4), id(2)]),
            )
            .expect("approve");
        assert_eq!(plan.priorities(), &[id(4), id(2)]);
        assert_eq!(
            planning
                .approve(
                    draft.id(),
                    0,
                    ContextFingerprint::new(7),
                    plan.id(),
                    ApprovalKey::new(1).expect("key"),
                    ApprovalSelection::All
                )
                .expect("replay"),
            plan
        );
    }

    #[test]
    fn stale_proposal_and_invalid_partial_selection_are_rejected() {
        let mut planning = DailyPlanning::default();
        let day = LocalDate::new(2026, 8, 6).expect("day");
        let draft = planning
            .create_draft(
                DraftId::new(1).expect("draft"),
                day,
                ContextFingerprint::new(7),
                &proposal(),
            )
            .expect("draft");
        assert_eq!(
            planning.approve(
                draft.id(),
                0,
                ContextFingerprint::new(8),
                PlanId::new(1).expect("plan"),
                ApprovalKey::new(1).expect("key"),
                ApprovalSelection::All
            ),
            Err(PlanningError::StaleProposal)
        );
        assert_eq!(
            planning.approve(
                draft.id(),
                0,
                ContextFingerprint::new(7),
                PlanId::new(1).expect("plan"),
                ApprovalKey::new(2).expect("key"),
                ApprovalSelection::Partial(vec![id(9)])
            ),
            Err(PlanningError::InvalidSelection)
        );
        assert_eq!(planning.draft(draft.id()), Some(draft));
    }
    #[test]
    fn adjusts_a_draft_without_restarting_and_preserves_decide_later() {
        let mut planning = DailyPlanning::default();
        let day = LocalDate::new(2026, 8, 6).expect("day");
        let draft = planning
            .create_draft(
                DraftId::new(1).expect("draft"),
                day,
                ContextFingerprint::new(7),
                &proposal(),
            )
            .expect("draft");
        let adjusted = planning
            .adjust_draft(draft.id(), 0, vec![id(4), id(2)])
            .expect("adjust");
        assert_eq!(adjusted.revision(), 1);
        assert_eq!(
            adjusted
                .priorities()
                .iter()
                .map(ProposedPriority::task_id)
                .collect::<Vec<_>>(),
            vec![id(4), id(2)]
        );
        assert_eq!(planning.draft(draft.id()), Some(adjusted.clone()));
        assert_eq!(
            planning.adjust_draft(draft.id(), 0, vec![id(4), id(2)]),
            Ok(adjusted)
        );
    }

    #[test]
    fn adjustment_rejects_stale_writes_and_removed_priorities() {
        let mut planning = DailyPlanning::default();
        let day = LocalDate::new(2026, 8, 6).expect("day");
        let draft = planning
            .create_draft(
                DraftId::new(1).expect("draft"),
                day,
                ContextFingerprint::new(7),
                &proposal(),
            )
            .expect("draft");
        planning
            .adjust_draft(draft.id(), 0, vec![id(4), id(2)])
            .expect("adjust");
        assert_eq!(
            planning.adjust_draft(draft.id(), 0, vec![id(4)]),
            Err(PlanningError::RevisionConflict {
                expected: 0,
                actual: 1
            })
        );
        assert_eq!(
            planning.adjust_draft(draft.id(), 1, vec![id(1), id(4)]),
            Err(PlanningError::InvalidSelection)
        );
    }
}
