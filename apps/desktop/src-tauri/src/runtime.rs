#![allow(clippy::needless_pass_by_value)]

//! Native transaction boundary connecting domains to the encrypted local journal.

use second_brain_application::{
    ActionsAndProjects, ApprovalKey, ApprovalSelection, AvailabilityOverride, ContextFingerprint,
    DailyPlanning, DecisionEngine, DraftId, Execution, FocusState, LocalDate, MinuteOfDay, PlanId,
    ProjectId, ProjectState, ReplanReason, Schedule, TaskDuration, TaskId, TaskState, TimeWindow,
};
use second_brain_contracts::{
    ApproveDailyPlanRequest, ArchiveProjectRequest, ConfigureDailyAvailabilityRequest,
    CreateProjectRequest, CreateTaskRequest, DailyAvailabilityDto, DailyCycleDto,
    ExecuteNowRequest, IpcError, NowDto, PlanDraftDto, ProjectDto, ProposeDailyPlanRequest,
    StorageHealthDto, TaskDto, TransitionTaskRequest, WorkspaceSnapshot,
};
use serde::{Deserialize, Serialize};

use crate::persistence::LocalStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Operation {
    CreateProject {
        id: u64,
        name: String,
        description: Option<String>,
    },
    ArchiveProject {
        id: u64,
        expected_revision: u64,
    },
    CreateTask {
        id: u64,
        title: String,
        project_id: Option<u64>,
        estimated_minutes: Option<u16>,
    },
    TransitionTask {
        id: u64,
        expected_revision: u64,
        destination: String,
    },
    ConfigureDailyAvailability {
        day: String,
        start_minute: u16,
        end_minute: u16,
        expected_revision: u64,
    },
    ProposeDailyPlan {
        draft_id: u64,
        day: String,
        fingerprint: u64,
        replanning: bool,
    },
    ApproveDailyPlan {
        draft_id: u64,
        expected_revision: u64,
        plan_id: u64,
        approval_key: u64,
        selected_task_ids: Option<Vec<u64>>,
        replaces_plan_id: Option<u64>,
    },
    StartFocus {
        expected_revision: u64,
    },
    CompleteCurrent {
        expected_revision: u64,
    },
    PostponeCurrent {
        expected_revision: u64,
    },
}

#[derive(Debug, Clone)]
struct ConfiguredAvailability {
    day: LocalDate,
    start_minute: u16,
    end_minute: u16,
    revision: u64,
}

#[derive(Debug, Clone)]
struct PendingDraft {
    id: DraftId,
    replanning: bool,
}

#[derive(Debug, Clone, Default)]
struct CycleState {
    schedule: Schedule,
    planning: DailyPlanning,
    execution: Execution,
    availability: Option<ConfiguredAvailability>,
    pending: Option<PendingDraft>,
    next_draft_id: u64,
    next_plan_id: u64,
    next_approval_key: u64,
}

impl CycleState {
    fn initialized() -> Self {
        Self {
            next_draft_id: 1,
            next_plan_id: 1,
            next_approval_key: 1,
            ..Self::default()
        }
    }
}

#[derive(Debug)]
pub(crate) struct Runtime {
    store: LocalStore,
    actions: ActionsAndProjects,
    cycle: CycleState,
    next_project_id: u64,
    next_task_id: u64,
}

impl Runtime {
    pub(crate) fn open(directory: &std::path::Path) -> Result<Self, IpcError> {
        let store = LocalStore::open(directory).map_err(storage_error)?;
        let mut runtime = Self {
            store,
            actions: ActionsAndProjects::default(),
            cycle: CycleState::initialized(),
            next_project_id: 1,
            next_task_id: 1,
        };
        for stored in runtime.store.operations().map_err(storage_error)? {
            let operation: Operation = serde_json::from_str(&stored.payload).map_err(|_| {
                safe_error(
                    "storage.invalid_operation",
                    "Os dados locais nÃƒÂ£o puderam ser reconstruÃƒÂ­dos.",
                )
            })?;
            runtime.replay(operation)?;
        }
        Ok(runtime)
    }

    pub(crate) fn snapshot(&self) -> WorkspaceSnapshot {
        let mut projects = self
            .actions
            .projects()
            .iter()
            .map(project_dto)
            .collect::<Vec<_>>();
        let mut tasks = self
            .actions
            .tasks()
            .iter()
            .map(task_dto)
            .collect::<Vec<_>>();
        projects.sort_by_key(|project| project.id);
        tasks.sort_by_key(|task| task.id);
        WorkspaceSnapshot {
            projects,
            tasks,
            storage: StorageHealthDto {
                cipher_version: self.store.health().cipher_version.clone(),
                schema_version: self.store.health().schema_version,
            },
            daily_cycle: cycle_dto(&self.cycle),
        }
    }

    pub(crate) fn create_project(
        &mut self,
        request: CreateProjectRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        let operation = Operation::CreateProject {
            id: self.next_project_id,
            name: request.name,
            description: request.description,
        };
        self.apply_transaction(operation)
    }

    pub(crate) fn archive_project(
        &mut self,
        request: ArchiveProjectRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        self.apply_transaction(Operation::ArchiveProject {
            id: request.id,
            expected_revision: request.expected_revision,
        })
    }

    pub(crate) fn create_task(
        &mut self,
        request: CreateTaskRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        let operation = Operation::CreateTask {
            id: self.next_task_id,
            title: request.title,
            project_id: request.project_id,
            estimated_minutes: request.estimated_minutes,
        };
        self.apply_transaction(operation)
    }

    pub(crate) fn transition_task(
        &mut self,
        request: TransitionTaskRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        self.apply_transaction(Operation::TransitionTask {
            id: request.id,
            expected_revision: request.expected_revision,
            destination: request.destination,
        })
    }

    pub(crate) fn configure_daily_availability(
        &mut self,
        request: ConfigureDailyAvailabilityRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        self.apply_transaction(Operation::ConfigureDailyAvailability {
            day: request.day,
            start_minute: request.start_minute,
            end_minute: request.end_minute,
            expected_revision: request.expected_revision,
        })
    }

    pub(crate) fn propose_daily_plan(
        &mut self,
        request: ProposeDailyPlanRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        let replanning = self
            .cycle
            .execution
            .now()
            .is_some_and(|now| now.replan_reason().is_some());
        let id = self.cycle.next_draft_id;
        self.apply_transaction(Operation::ProposeDailyPlan {
            draft_id: id,
            day: request.day,
            fingerprint: id,
            replanning,
        })
    }

    pub(crate) fn approve_daily_plan(
        &mut self,
        request: ApproveDailyPlanRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        let replaces_plan_id = self
            .cycle
            .pending
            .as_ref()
            .filter(|pending| pending.replanning)
            .and_then(|_| self.cycle.execution.now().map(|now| now.plan_id().value()));
        self.apply_transaction(Operation::ApproveDailyPlan {
            draft_id: request.draft_id,
            expected_revision: request.expected_revision,
            plan_id: self.cycle.next_plan_id,
            approval_key: self.cycle.next_approval_key,
            selected_task_ids: request.selected_task_ids,
            replaces_plan_id,
        })
    }

    pub(crate) fn start_focus(
        &mut self,
        request: ExecuteNowRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        self.apply_transaction(Operation::StartFocus {
            expected_revision: request.expected_revision,
        })
    }

    pub(crate) fn complete_current(
        &mut self,
        request: ExecuteNowRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        self.apply_transaction(Operation::CompleteCurrent {
            expected_revision: request.expected_revision,
        })
    }

    pub(crate) fn postpone_current(
        &mut self,
        request: ExecuteNowRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        self.apply_transaction(Operation::PostponeCurrent {
            expected_revision: request.expected_revision,
        })
    }

    fn apply_transaction(&mut self, operation: Operation) -> Result<WorkspaceSnapshot, IpcError> {
        let mut actions = self.actions.clone();
        let mut cycle = self.cycle.clone();
        apply(&mut actions, &mut cycle, &operation)?;
        self.commit(&operation)?;
        self.actions = actions;
        self.cycle = cycle;
        self.update_sequences(&operation);
        Ok(self.snapshot())
    }

    fn replay(&mut self, operation: Operation) -> Result<(), IpcError> {
        apply(&mut self.actions, &mut self.cycle, &operation)?;
        self.update_sequences(&operation);
        Ok(())
    }

    fn update_sequences(&mut self, operation: &Operation) {
        match operation {
            Operation::CreateProject { id, .. } => {
                self.next_project_id = self.next_project_id.max(id + 1)
            }
            Operation::CreateTask { id, .. } => self.next_task_id = self.next_task_id.max(id + 1),
            Operation::ProposeDailyPlan { draft_id, .. } => {
                self.cycle.next_draft_id = self.cycle.next_draft_id.max(draft_id + 1);
            }
            Operation::ApproveDailyPlan {
                plan_id,
                approval_key,
                ..
            } => {
                self.cycle.next_plan_id = self.cycle.next_plan_id.max(plan_id + 1);
                self.cycle.next_approval_key = self.cycle.next_approval_key.max(approval_key + 1);
            }
            _ => {}
        }
    }

    fn commit(&mut self, operation: &Operation) -> Result<(), IpcError> {
        let payload = serde_json::to_string(operation).map_err(|_| {
            safe_error(
                "storage.serialization",
                "A alteraÃƒÂ§ÃƒÂ£o nÃƒÂ£o pÃƒÂ´de ser preparada.",
            )
        })?;
        let kind = match operation {
            Operation::CreateProject { .. } => "project.created",
            Operation::ArchiveProject { .. } => "project.archived",
            Operation::CreateTask { .. } => "task.created",
            Operation::TransitionTask { .. } => "task.transitioned",
            Operation::ConfigureDailyAvailability { .. } => "availability.configured",
            Operation::ProposeDailyPlan { .. } => "plan.proposed",
            Operation::ApproveDailyPlan {
                replaces_plan_id: Some(_),
                ..
            } => "plan.replanned",
            Operation::ApproveDailyPlan { .. } => "plan.approved",
            Operation::StartFocus { .. } => "focus.started",
            Operation::CompleteCurrent { .. } => "focus.completed",
            Operation::PostponeCurrent { .. } => "focus.postponed",
        };
        self.store
            .append(kind, &payload)
            .map(|_| ())
            .map_err(storage_error)
    }
}

fn apply(
    actions: &mut ActionsAndProjects,
    cycle: &mut CycleState,
    operation: &Operation,
) -> Result<(), IpcError> {
    match operation {
        Operation::CreateProject {
            id,
            name,
            description,
        } => {
            actions
                .create_project(project_id(*id)?, name.clone(), description.clone())
                .map_err(domain_error)?;
        }
        Operation::ArchiveProject {
            id,
            expected_revision,
        } => {
            actions
                .archive_project(project_id(*id)?, *expected_revision)
                .map_err(domain_error)?;
        }
        Operation::CreateTask {
            id,
            title,
            project_id: linked,
            estimated_minutes,
        } => {
            let id = task_id(*id)?;
            let mut task = actions
                .create_task(id, title.clone())
                .map_err(domain_error)?;
            if let Some(project) = linked {
                task = actions
                    .link_task(id, task.revision(), Some(project_id(*project)?))
                    .map_err(domain_error)?;
            }
            if let Some(minutes) = estimated_minutes {
                let duration = TaskDuration::new(*minutes).ok_or_else(|| {
                    safe_error(
                        "validation.duration",
                        "A duraÃƒÂ§ÃƒÂ£o deve ser maior que zero.",
                    )
                })?;
                actions
                    .set_task_duration(id, task.revision(), Some(duration))
                    .map_err(domain_error)?;
            }
        }
        Operation::TransitionTask {
            id,
            expected_revision,
            destination,
        } => {
            actions
                .transition_task(task_id(*id)?, *expected_revision, parse_state(destination)?)
                .map_err(domain_error)?;
        }
        Operation::ConfigureDailyAvailability {
            day,
            start_minute,
            end_minute,
            expected_revision,
        } => {
            let day = parse_date(day)?;
            let start = MinuteOfDay::new(*start_minute).map_err(preference_error)?;
            let end = MinuteOfDay::new(*end_minute).map_err(preference_error)?;
            let window = TimeWindow::new(start, end).map_err(schedule_error)?;
            let revision = cycle
                .schedule
                .configure_availability_exception(
                    day,
                    AvailabilityOverride::Window(window),
                    *expected_revision,
                )
                .map_err(schedule_error)?;
            cycle.availability = Some(ConfiguredAvailability {
                day,
                start_minute: *start_minute,
                end_minute: *end_minute,
                revision,
            });
        }
        Operation::ProposeDailyPlan {
            draft_id,
            day,
            fingerprint,
            replanning,
        } => {
            let day = parse_date(day)?;
            if cycle.availability.as_ref().map(|value| value.day) != Some(day) {
                return Err(safe_error(
                    "planning.availability_required",
                    "Defina sua disponibilidade antes de planejar.",
                ));
            }
            let availability = cycle.schedule.availability(day).map_err(schedule_error)?;
            let mut tasks = actions.eligible_tasks();
            if *replanning {
                tasks.retain(|task| task.state() != TaskState::Postponed);
            }
            let proposal = DecisionEngine::propose(&tasks, &availability);
            let id = draft_id_from(*draft_id)?;
            cycle
                .planning
                .create_draft(id, day, ContextFingerprint::new(*fingerprint), &proposal)
                .map_err(planning_error)?;
            cycle.pending = Some(PendingDraft {
                id,
                replanning: *replanning,
            });
        }
        Operation::ApproveDailyPlan {
            draft_id,
            expected_revision,
            plan_id,
            approval_key,
            selected_task_ids,
            replaces_plan_id,
        } => {
            let draft_id = draft_id_from(*draft_id)?;
            if cycle.pending.as_ref().map(|pending| pending.id) != Some(draft_id) {
                return Err(safe_error(
                    "planning.draft_changed",
                    "A proposta jÃƒÂ¡ nÃƒÂ£o ÃƒÂ© a proposta atual.",
                ));
            }
            let selection =
                match selected_task_ids {
                    Some(ids) => ApprovalSelection::Partial(
                        ids.iter()
                            .map(|id| task_id(*id))
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                    None => ApprovalSelection::All,
                };
            let fingerprint = ContextFingerprint::new(draft_id.value());
            let plan_id = plan_id_from(*plan_id)?;
            let key = ApprovalKey::new(*approval_key).ok_or_else(invalid_identifier)?;
            let plan = if let Some(replaces) = replaces_plan_id {
                cycle.planning.approve_replan(
                    draft_id,
                    *expected_revision,
                    fingerprint,
                    plan_id,
                    key,
                    selection,
                    plan_id_from(*replaces)?,
                )
            } else {
                cycle.planning.approve(
                    draft_id,
                    *expected_revision,
                    fingerprint,
                    plan_id,
                    key,
                    selection,
                )
            }
            .map_err(planning_error)?;
            for id in plan.priorities() {
                let task = actions.task(*id).ok_or_else(|| {
                    safe_error(
                        "domain.task_missing",
                        "Uma aÃƒÂ§ÃƒÂ£o da proposta nÃƒÂ£o existe mais.",
                    )
                })?;
                if matches!(task.state(), TaskState::Inbox | TaskState::Postponed) {
                    actions
                        .transition_task(*id, task.revision(), TaskState::Planned)
                        .map_err(domain_error)?;
                }
            }
            if replaces_plan_id.is_some() {
                let revision = cycle
                    .execution
                    .now()
                    .ok_or_else(|| {
                        safe_error(
                            "execution.not_active",
                            "NÃƒÂ£o existe plano ativo para reorganizar.",
                        )
                    })?
                    .revision();
                cycle
                    .execution
                    .replan(&plan, revision)
                    .map_err(execution_error)?;
            } else {
                cycle.execution.activate(&plan).map_err(execution_error)?;
            }
            cycle.pending = None;
        }
        Operation::StartFocus { expected_revision } => {
            let current = current_task(cycle)?;
            cycle
                .execution
                .start(current, *expected_revision)
                .map_err(execution_error)?;
            let task = actions.task(current).ok_or_else(|| {
                safe_error(
                    "domain.task_missing",
                    "A aÃƒÂ§ÃƒÂ£o atual nÃƒÂ£o existe mais.",
                )
            })?;
            actions
                .transition_task(current, task.revision(), TaskState::InProgress)
                .map_err(domain_error)?;
        }
        Operation::CompleteCurrent { expected_revision } => {
            let current = current_task(cycle)?;
            cycle
                .execution
                .complete(current, *expected_revision)
                .map_err(execution_error)?;
            let task = actions.task(current).ok_or_else(|| {
                safe_error(
                    "domain.task_missing",
                    "A aÃƒÂ§ÃƒÂ£o atual nÃƒÂ£o existe mais.",
                )
            })?;
            actions
                .transition_task(current, task.revision(), TaskState::Completed)
                .map_err(domain_error)?;
        }
        Operation::PostponeCurrent { expected_revision } => {
            let current = current_task(cycle)?;
            cycle
                .execution
                .postpone(current, *expected_revision)
                .map_err(execution_error)?;
            let task = actions.task(current).ok_or_else(|| {
                safe_error(
                    "domain.task_missing",
                    "A aÃƒÂ§ÃƒÂ£o atual nÃƒÂ£o existe mais.",
                )
            })?;
            actions
                .transition_task(current, task.revision(), TaskState::Postponed)
                .map_err(domain_error)?;
        }
    }
    Ok(())
}

fn current_task(cycle: &CycleState) -> Result<TaskId, IpcError> {
    cycle
        .execution
        .now()
        .and_then(|now| now.current())
        .ok_or_else(|| {
            safe_error(
                "execution.no_current",
                "NÃƒÂ£o existe uma aÃƒÂ§ÃƒÂ£o no Agora.",
            )
        })
}

fn cycle_dto(cycle: &CycleState) -> DailyCycleDto {
    let draft = cycle.pending.as_ref().and_then(|pending| {
        cycle.planning.draft(pending.id).map(|draft| PlanDraftDto {
            id: draft.id().value(),
            revision: draft.revision(),
            priority_task_ids: draft
                .priorities()
                .iter()
                .map(|item| item.task_id().value())
                .collect(),
            eligible_task_ids: draft.eligible_tasks().map(TaskId::value).collect(),
            missing_duration_task_ids: draft
                .missing_duration()
                .iter()
                .copied()
                .map(TaskId::value)
                .collect(),
            context_complete: draft.context_complete(),
            replanning: pending.replanning,
        })
    });
    DailyCycleDto {
        availability: cycle
            .availability
            .as_ref()
            .map(|value| DailyAvailabilityDto {
                day: format_date(value.day),
                start_minute: value.start_minute,
                end_minute: value.end_minute,
                revision: value.revision,
            }),
        draft,
        now: cycle.execution.now().map(|now| NowDto {
            day: format_date(now.day()),
            plan_id: now.plan_id().value(),
            revision: now.revision(),
            current_task_id: now.current().map(TaskId::value),
            remaining_task_ids: now.remaining().iter().copied().map(TaskId::value).collect(),
            focus_state: now.session().map(|session| {
                match session.state() {
                    FocusState::Active => "active",
                    FocusState::Paused => "paused",
                }
                .to_owned()
            }),
            replan_reason: now.replan_reason().map(|reason| {
                match reason {
                    ReplanReason::PriorityPostponed => "priority_postponed",
                    ReplanReason::PlanExhausted => "plan_exhausted",
                }
                .to_owned()
            }),
        }),
    }
}

fn project_dto(project: &second_brain_application::Project) -> ProjectDto {
    ProjectDto {
        id: project.id().value(),
        revision: project.revision(),
        name: project.name().to_owned(),
        description: project.description().map(str::to_owned),
        archived: project.state() == ProjectState::Archived,
    }
}
fn task_dto(task: &second_brain_application::Task) -> TaskDto {
    TaskDto {
        id: task.id().value(),
        revision: task.revision(),
        title: task.title().to_owned(),
        state: state_name(task.state()).to_owned(),
        project_id: task.project_id().map(ProjectId::value),
        estimated_minutes: task.estimated_duration().map(TaskDuration::minutes),
    }
}
fn parse_date(value: &str) -> Result<LocalDate, IpcError> {
    let mut parts = value.split('-');
    let year = parts.next().and_then(|part| part.parse::<u16>().ok());
    let month = parts.next().and_then(|part| part.parse::<u8>().ok());
    let day = parts.next().and_then(|part| part.parse::<u8>().ok());
    if parts.next().is_some() {
        return Err(invalid_date());
    }
    year.zip(month)
        .zip(day)
        .and_then(|((year, month), day)| LocalDate::new(year, month, day).ok())
        .ok_or_else(invalid_date)
}
fn format_date(value: LocalDate) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        value.year(),
        value.month(),
        value.day()
    )
}
fn invalid_date() -> IpcError {
    safe_error("validation.date", "Data operacional invÃƒÂ¡lida.")
}
fn invalid_identifier() -> IpcError {
    safe_error("validation.identifier", "Identificador invÃƒÂ¡lido.")
}
fn project_id(value: u64) -> Result<ProjectId, IpcError> {
    ProjectId::new(value).ok_or_else(invalid_identifier)
}
fn task_id(value: u64) -> Result<TaskId, IpcError> {
    TaskId::new(value).ok_or_else(invalid_identifier)
}
fn draft_id_from(value: u64) -> Result<DraftId, IpcError> {
    DraftId::new(value).ok_or_else(invalid_identifier)
}
fn plan_id_from(value: u64) -> Result<PlanId, IpcError> {
    PlanId::new(value).ok_or_else(invalid_identifier)
}
fn parse_state(value: &str) -> Result<TaskState, IpcError> {
    match value {
        "planned" => Ok(TaskState::Planned),
        "in_progress" => Ok(TaskState::InProgress),
        "completed" => Ok(TaskState::Completed),
        "postponed" => Ok(TaskState::Postponed),
        "cancelled" => Ok(TaskState::Cancelled),
        _ => Err(safe_error(
            "validation.state",
            "TransiÃƒÂ§ÃƒÂ£o de estado invÃƒÂ¡lida.",
        )),
    }
}
const fn state_name(value: TaskState) -> &'static str {
    match value {
        TaskState::Inbox => "inbox",
        TaskState::Planned => "planned",
        TaskState::InProgress => "in_progress",
        TaskState::Completed => "completed",
        TaskState::Postponed => "postponed",
        TaskState::Cancelled => "cancelled",
    }
}
fn domain_error(error: second_brain_application::ActionsError) -> IpcError {
    safe_error(
        "domain.rejected",
        &format!("AlteraÃƒÂ§ÃƒÂ£o rejeitada: {error:?}"),
    )
}
fn preference_error(error: second_brain_application::PreferenceError) -> IpcError {
    safe_error(
        "validation.availability",
        &format!("Horario invalido: {error:?}"),
    )
}
fn schedule_error(error: second_brain_application::ScheduleError) -> IpcError {
    safe_error(
        "schedule.rejected",
        &format!("Disponibilidade rejeitada: {error:?}"),
    )
}
fn planning_error(error: second_brain_application::PlanningError) -> IpcError {
    safe_error(
        "planning.rejected",
        &format!("Planejamento rejeitado: {error:?}"),
    )
}
fn execution_error(error: second_brain_application::ExecutionError) -> IpcError {
    safe_error(
        "execution.rejected",
        &format!("ExecuÃƒÂ§ÃƒÂ£o rejeitada: {error:?}"),
    )
}
fn storage_error(error: crate::persistence::StoreError) -> IpcError {
    safe_error(
        "storage.unavailable",
        &format!("Armazenamento local indisponÃƒÂ­vel: {error:?}"),
    )
}
fn safe_error(code: &str, message: &str) -> IpcError {
    IpcError {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn directory(name: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("target/runtime-tests")
            .join(format!("{name}-{}", std::process::id()))
    }

    fn clean(directory: &std::path::Path) {
        if directory.exists() {
            std::fs::remove_dir_all(directory).expect("remove stale data");
        }
    }

    #[test]
    fn reconstructs_domain_state_after_restart() {
        let directory = directory("restart");
        clean(&directory);
        {
            let mut runtime = Runtime::open(&directory).expect("runtime");
            runtime
                .create_project(CreateProjectRequest {
                    name: "Alpha".to_owned(),
                    description: None,
                })
                .expect("project");
            runtime
                .create_task(CreateTaskRequest {
                    title: "Ship slice".to_owned(),
                    project_id: Some(1),
                    estimated_minutes: Some(45),
                })
                .expect("task");
            let task = runtime.snapshot().tasks[0].clone();
            runtime
                .transition_task(TransitionTaskRequest {
                    id: task.id,
                    expected_revision: task.revision,
                    destination: "planned".to_owned(),
                })
                .expect("plan task");
        }
        let runtime = Runtime::open(&directory).expect("reopen runtime");
        let snapshot = runtime.snapshot();
        assert_eq!(snapshot.projects[0].name, "Alpha");
        assert_eq!(snapshot.tasks[0].state, "planned");
        assert_eq!(snapshot.tasks[0].estimated_minutes, Some(45));
        drop(runtime);
        clean(&directory);
    }

    #[test]
    fn daily_cycle_survives_completion_postponement_and_replanning() {
        let directory = directory("daily-cycle");
        clean(&directory);
        {
            let mut runtime = Runtime::open(&directory).expect("runtime");
            for title in ["First", "Second", "Replacement"] {
                runtime
                    .create_task(CreateTaskRequest {
                        title: title.to_owned(),
                        project_id: None,
                        estimated_minutes: Some(30),
                    })
                    .expect("task");
            }
            runtime
                .configure_daily_availability(ConfigureDailyAvailabilityRequest {
                    day: "2026-08-07".to_owned(),
                    start_minute: 540,
                    end_minute: 720,
                    expected_revision: 0,
                })
                .expect("availability");
            let proposed = runtime
                .propose_daily_plan(ProposeDailyPlanRequest {
                    day: "2026-08-07".to_owned(),
                })
                .expect("proposal");
            let draft = proposed.daily_cycle.draft.expect("draft");
            let approved = runtime
                .approve_daily_plan(ApproveDailyPlanRequest {
                    draft_id: draft.id,
                    expected_revision: draft.revision,
                    selected_task_ids: Some(vec![1, 2]),
                })
                .expect("approve");
            let now = approved.daily_cycle.now.expect("now");
            runtime
                .start_focus(ExecuteNowRequest {
                    expected_revision: now.revision,
                })
                .expect("start");
            let current = runtime.snapshot().daily_cycle.now.expect("active");
            runtime
                .complete_current(ExecuteNowRequest {
                    expected_revision: current.revision,
                })
                .expect("complete");
            let next = runtime.snapshot().daily_cycle.now.expect("next");
            runtime
                .postpone_current(ExecuteNowRequest {
                    expected_revision: next.revision,
                })
                .expect("postpone");
            let pending = runtime
                .propose_daily_plan(ProposeDailyPlanRequest {
                    day: "2026-08-07".to_owned(),
                })
                .expect("replan proposal");
            let draft = pending.daily_cycle.draft.expect("draft");
            runtime
                .approve_daily_plan(ApproveDailyPlanRequest {
                    draft_id: draft.id,
                    expected_revision: draft.revision,
                    selected_task_ids: Some(vec![3]),
                })
                .expect("approve replan");
        }
        let runtime = Runtime::open(&directory).expect("reopen runtime");
        let snapshot = runtime.snapshot();
        assert_eq!(snapshot.tasks[0].state, "completed");
        assert_eq!(snapshot.tasks[1].state, "postponed");
        assert_eq!(
            snapshot.daily_cycle.now.expect("now").current_task_id,
            Some(3)
        );
        drop(runtime);
        clean(&directory);
    }
}
