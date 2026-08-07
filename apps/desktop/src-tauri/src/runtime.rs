#![allow(clippy::needless_pass_by_value)]

//! Native transaction boundary connecting domains to the encrypted local journal.

use second_brain_application::{
    ActionsAndProjects, ProjectId, ProjectState, TaskDuration, TaskId, TaskState,
};
use second_brain_contracts::{
    ArchiveProjectRequest, CreateProjectRequest, CreateTaskRequest, IpcError, ProjectDto,
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
}

#[derive(Debug)]
pub(crate) struct Runtime {
    store: LocalStore,
    actions: ActionsAndProjects,
    next_project_id: u64,
    next_task_id: u64,
}

impl Runtime {
    pub(crate) fn open(directory: &std::path::Path) -> Result<Self, IpcError> {
        let store = LocalStore::open(directory).map_err(storage_error)?;
        let mut runtime = Self {
            store,
            actions: ActionsAndProjects::default(),
            next_project_id: 1,
            next_task_id: 1,
        };
        for stored in runtime.store.operations().map_err(storage_error)? {
            let operation: Operation = serde_json::from_str(&stored.payload).map_err(|_| {
                safe_error(
                    "storage.invalid_operation",
                    "Os dados locais nao puderam ser reconstruidos.",
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
        }
    }

    pub(crate) fn create_project(
        &mut self,
        request: CreateProjectRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        let id = self.next_project_id;
        let operation = Operation::CreateProject {
            id,
            name: request.name,
            description: request.description,
        };
        let mut candidate = self.actions.clone();
        apply(&mut candidate, &operation)?;
        self.commit(&operation)?;
        self.actions = candidate;
        self.next_project_id += 1;
        Ok(self.snapshot())
    }

    pub(crate) fn archive_project(
        &mut self,
        request: ArchiveProjectRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        let operation = Operation::ArchiveProject {
            id: request.id,
            expected_revision: request.expected_revision,
        };
        let mut candidate = self.actions.clone();
        apply(&mut candidate, &operation)?;
        self.commit(&operation)?;
        self.actions = candidate;
        Ok(self.snapshot())
    }

    pub(crate) fn create_task(
        &mut self,
        request: CreateTaskRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        let id = self.next_task_id;
        let operation = Operation::CreateTask {
            id,
            title: request.title,
            project_id: request.project_id,
            estimated_minutes: request.estimated_minutes,
        };
        let mut candidate = self.actions.clone();
        apply(&mut candidate, &operation)?;
        self.commit(&operation)?;
        self.actions = candidate;
        self.next_task_id += 1;
        Ok(self.snapshot())
    }

    pub(crate) fn transition_task(
        &mut self,
        request: TransitionTaskRequest,
    ) -> Result<WorkspaceSnapshot, IpcError> {
        let operation = Operation::TransitionTask {
            id: request.id,
            expected_revision: request.expected_revision,
            destination: request.destination,
        };
        let mut candidate = self.actions.clone();
        apply(&mut candidate, &operation)?;
        self.commit(&operation)?;
        self.actions = candidate;
        Ok(self.snapshot())
    }

    fn replay(&mut self, operation: Operation) -> Result<(), IpcError> {
        apply(&mut self.actions, &operation)?;
        match operation {
            Operation::CreateProject { id, .. } => {
                self.next_project_id = self.next_project_id.max(id + 1);
            }
            Operation::CreateTask { id, .. } => self.next_task_id = self.next_task_id.max(id + 1),
            Operation::ArchiveProject { .. } | Operation::TransitionTask { .. } => {}
        }
        Ok(())
    }

    fn commit(&mut self, operation: &Operation) -> Result<(), IpcError> {
        let payload = serde_json::to_string(operation).map_err(|_| {
            safe_error(
                "storage.serialization",
                "A alteraÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â§ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â£o nÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â£o pÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â´de ser preparada.",
            )
        })?;
        let kind = match operation {
            Operation::CreateProject { .. } => "project.created",
            Operation::ArchiveProject { .. } => "project.archived",
            Operation::CreateTask { .. } => "task.created",
            Operation::TransitionTask { .. } => "task.transitioned",
        };
        self.store.append(kind, &payload).map_err(storage_error)?;
        Ok(())
    }
}

fn apply(actions: &mut ActionsAndProjects, operation: &Operation) -> Result<(), IpcError> {
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
                        "A duraÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â§ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â£o deve ser maior que zero.",
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
    }
    Ok(())
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
fn project_id(value: u64) -> Result<ProjectId, IpcError> {
    ProjectId::new(value).ok_or_else(|| {
        safe_error(
            "validation.identifier",
            "Identificador invÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¡lido.",
        )
    })
}
fn task_id(value: u64) -> Result<TaskId, IpcError> {
    TaskId::new(value).ok_or_else(|| {
        safe_error(
            "validation.identifier",
            "Identificador invÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¡lido.",
        )
    })
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
            "TransiÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â§ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â£o de estado invÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¡lida.",
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
        &format!("AlteraÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â§ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â£o rejeitada: {error:?}"),
    )
}
fn storage_error(error: crate::persistence::StoreError) -> IpcError {
    safe_error(
        "storage.unavailable",
        &format!("Armazenamento local indisponÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â­vel: {error:?}"),
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

    fn directory() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("target/runtime-tests")
            .join(format!("restart-{}", std::process::id()))
    }

    #[test]
    fn reconstructs_domain_state_after_restart() {
        let directory = directory();
        if directory.exists() {
            std::fs::remove_dir_all(&directory).expect("remove stale data");
        }
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
        {
            let runtime = Runtime::open(&directory).expect("reopen runtime");
            let snapshot = runtime.snapshot();
            assert_eq!(snapshot.projects[0].name, "Alpha");
            assert_eq!(snapshot.tasks[0].state, "planned");
            assert_eq!(snapshot.tasks[0].estimated_minutes, Some(45));
        }
        std::fs::remove_dir_all(directory).expect("remove test data");
    }
}
