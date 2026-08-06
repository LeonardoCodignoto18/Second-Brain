//! Deterministic priority proposal constrained by task state, duration, and availability.

use crate::{DailyAvailability, Task, TaskId};

/// Reason attached to a deterministic priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityReason {
    /// The task has a known duration that fits a remaining free window.
    FitsAvailableWindow,
}

/// One proposed task priority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProposedPriority {
    task_id: TaskId,
    reason: PriorityReason,
}

impl ProposedPriority {
    /// Returns the referenced task.
    #[must_use]
    pub const fn task_id(&self) -> TaskId {
        self.task_id
    }
    pub(crate) const fn deterministic_fit(task_id: TaskId) -> Self {
        Self {
            task_id,
            reason: PriorityReason::FitsAvailableWindow,
        }
    }
    /// Returns the deterministic explanation.
    #[must_use]
    pub const fn reason(&self) -> PriorityReason {
        self.reason
    }
}

/// Provider-independent deterministic proposal consumed by Planning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicProposal {
    priorities: Vec<ProposedPriority>,
    eligible_tasks: Vec<TaskId>,
    missing_duration: Vec<TaskId>,
    context_complete: bool,
}

impl DeterministicProposal {
    /// Returns zero to three proposed priorities.
    #[must_use]
    pub fn priorities(&self) -> &[ProposedPriority] {
        &self.priorities
    }
    /// Returns all tasks eligible as manual substitutes.
    #[must_use]
    pub fn eligible_tasks(&self) -> &[TaskId] {
        &self.eligible_tasks
    }
    /// Returns tasks excluded because planning needs an estimated duration.
    #[must_use]
    pub fn missing_duration(&self) -> &[TaskId] {
        &self.missing_duration
    }
    /// Returns whether availability context was complete.
    #[must_use]
    pub const fn context_complete(&self) -> bool {
        self.context_complete
    }

    #[cfg(test)]
    pub(crate) fn from_parts_for_planning_tests(
        priorities: Vec<TaskId>,
        eligible_tasks: Vec<TaskId>,
    ) -> Self {
        Self {
            priorities: priorities
                .into_iter()
                .map(|task_id| ProposedPriority {
                    task_id,
                    reason: PriorityReason::FitsAvailableWindow,
                })
                .collect(),
            eligible_tasks,
            missing_duration: Vec::new(),
            context_complete: true,
        }
    }
}

/// Stateless deterministic Decision Engine.
#[derive(Debug, Default)]
pub struct DecisionEngine;

impl DecisionEngine {
    /// Produces a stable proposal without filling three positions artificially.
    #[must_use]
    pub fn propose(tasks: &[Task], availability: &DailyAvailability) -> DeterministicProposal {
        if !availability.is_complete() {
            return DeterministicProposal {
                priorities: Vec::new(),
                eligible_tasks: Vec::new(),
                missing_duration: tasks
                    .iter()
                    .filter(|task| task.estimated_duration().is_none())
                    .map(Task::id)
                    .collect(),
                context_complete: false,
            };
        }
        let mut ordered = tasks.iter().collect::<Vec<_>>();
        ordered.sort_by_key(|task| task.id());
        let mut eligible_tasks = Vec::new();
        let mut missing_duration = Vec::new();
        for task in ordered {
            let Some(duration) = task.estimated_duration() else {
                missing_duration.push(task.id());
                continue;
            };
            if availability
                .windows()
                .iter()
                .any(|window| window.duration_minutes() >= duration.minutes())
            {
                eligible_tasks.push(task.id());
            }
        }
        let priorities = eligible_tasks
            .iter()
            .take(3)
            .copied()
            .map(|task_id| ProposedPriority {
                task_id,
                reason: PriorityReason::FitsAvailableWindow,
            })
            .collect();
        DeterministicProposal {
            priorities,
            eligible_tasks,
            missing_duration,
            context_complete: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ActionsAndProjects, LocalDate, MinuteOfDay, Schedule, TaskDuration, TimeWindow, Weekday,
    };

    fn availability() -> DailyAvailability {
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
        schedule
            .availability(LocalDate::new(2026, 8, 6).expect("date"))
            .expect("derived")
    }

    #[test]
    fn proposes_only_tasks_with_duration_that_fit() {
        let mut actions = ActionsAndProjects::default();
        for value in 1..=4 {
            actions
                .create_task(TaskId::new(value).expect("id"), format!("Task {value}"))
                .expect("task");
        }
        actions
            .set_task_duration(TaskId::new(1).expect("id"), 0, TaskDuration::new(60))
            .expect("duration");
        actions
            .set_task_duration(TaskId::new(2).expect("id"), 0, TaskDuration::new(180))
            .expect("duration");
        actions
            .set_task_duration(TaskId::new(3).expect("id"), 0, TaskDuration::new(30))
            .expect("duration");
        let proposal = DecisionEngine::propose(&actions.eligible_tasks(), &availability());
        assert_eq!(
            proposal.eligible_tasks(),
            &[TaskId::new(1).expect("id"), TaskId::new(3).expect("id")]
        );
        assert_eq!(proposal.missing_duration(), &[TaskId::new(4).expect("id")]);
        assert_eq!(proposal.priorities().len(), 2);
    }
}
