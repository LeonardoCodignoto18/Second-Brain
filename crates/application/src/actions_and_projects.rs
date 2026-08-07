//! Deterministic task lifecycle and minimal project ownership.

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Stable identifier of a project.
pub struct ProjectId(u64);

impl ProjectId {
    #[must_use]
    /// Creates a non-zero identifier.
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }

    #[must_use]
    /// Returns the numeric identifier value.
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Stable identifier of a task.
pub struct TaskId(u64);

impl TaskId {
    #[must_use]
    /// Creates a non-zero identifier.
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }

    #[must_use]
    /// Returns the numeric identifier value.
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Lifecycle state of a minimal MVP project.
pub enum ProjectState {
    /// Project accepts edits and task links.
    Active,
    /// Project is retained but no longer accepts changes or new links.
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Immutable snapshot of a minimal MVP project.
pub struct Project {
    id: ProjectId,
    revision: u64,
    name: String,
    description: Option<String>,
    state: ProjectState,
}

impl Project {
    #[must_use]
    /// Returns the entity identifier.
    pub const fn id(&self) -> ProjectId {
        self.id
    }
    #[must_use]
    /// Returns the optimistic-concurrency revision.
    pub const fn revision(&self) -> u64 {
        self.revision
    }
    #[must_use]
    /// Returns the project name.
    pub fn name(&self) -> &str {
        &self.name
    }
    #[must_use]
    /// Returns the optional project description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    #[must_use]
    /// Returns the lifecycle state.
    pub const fn state(&self) -> ProjectState {
        self.state
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Lifecycle state of a task in the MVP contract.
pub enum TaskState {
    /// Captured but not planned.
    Inbox,
    /// Selected for execution.
    Planned,
    /// Currently being executed.
    InProgress,
    /// Finished successfully.
    Completed,
    /// Explicitly deferred.
    Postponed,
    /// Explicitly cancelled.
    Cancelled,
}

/// Positive estimated task duration used by planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskDuration(u16);

impl TaskDuration {
    /// Creates a positive duration in minutes.
    #[must_use]
    pub const fn new(minutes: u16) -> Option<Self> {
        if minutes == 0 {
            None
        } else {
            Some(Self(minutes))
        }
    }

    /// Returns the duration in minutes.
    #[must_use]
    pub const fn minutes(self) -> u16 {
        self.0
    }
}
/// Immutable snapshot of a task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    id: TaskId,
    revision: u64,
    title: String,
    state: TaskState,
    project_id: Option<ProjectId>,
    estimated_duration: Option<TaskDuration>,
}

impl Task {
    #[must_use]
    /// Returns the entity identifier.
    pub const fn id(&self) -> TaskId {
        self.id
    }
    #[must_use]
    /// Returns the optimistic-concurrency revision.
    pub const fn revision(&self) -> u64 {
        self.revision
    }
    #[must_use]
    /// Returns the task title.
    pub fn title(&self) -> &str {
        &self.title
    }
    #[must_use]
    /// Returns the lifecycle state.
    pub const fn state(&self) -> TaskState {
        self.state
    }
    #[must_use]
    /// Returns the optional linked project identifier.
    pub const fn project_id(&self) -> Option<ProjectId> {
        self.project_id
    }
    /// Returns the optional estimated duration used by planning.
    #[must_use]
    pub const fn estimated_duration(&self) -> Option<TaskDuration> {
        self.estimated_duration
    }
}

#[derive(Debug, Default, Clone)]
/// In-memory owner of deterministic task and minimal-project operations.
pub struct ActionsAndProjects {
    projects: BTreeMap<ProjectId, Project>,
    tasks: BTreeMap<TaskId, Task>,
}

impl ActionsAndProjects {
    /// Creates a project or returns the original snapshot for an exact replay.
    ///
    /// # Errors
    /// Returns [``ActionsError``] when validation, identity, lifecycle, or revision checks fail.
    pub fn create_project(
        &mut self,
        id: ProjectId,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Result<Project, ActionsError> {
        let name = normalize_required(name.into(), Field::ProjectName)?;
        let description = normalize_optional(description, Field::ProjectDescription)?;
        if let Some(existing) = self.projects.get(&id) {
            if existing.name == name && existing.description == description {
                return Ok(existing.clone());
            }
            return Err(ActionsError::ConflictingCreate(Entity::Project));
        }
        let project = Project {
            id,
            revision: 0,
            name,
            description,
            state: ProjectState::Active,
        };
        self.projects.insert(id, project.clone());
        Ok(project)
    }

    /// Replaces editable project fields at the expected revision.
    ///
    /// # Errors
    /// Returns [``ActionsError``] when validation, identity, lifecycle, or revision checks fail.
    pub fn edit_project(
        &mut self,
        id: ProjectId,
        expected_revision: u64,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Result<Project, ActionsError> {
        let name = normalize_required(name.into(), Field::ProjectName)?;
        let description = normalize_optional(description, Field::ProjectDescription)?;
        let project = self
            .projects
            .get_mut(&id)
            .ok_or(ActionsError::NotFound(Entity::Project))?;
        if project.name == name && project.description == description {
            return Ok(project.clone());
        }
        if project.state == ProjectState::Archived {
            return Err(ActionsError::ArchivedProject);
        }
        ensure_revision(project.revision, expected_revision, Entity::Project)?;
        project.name = name;
        project.description = description;
        project.revision += 1;
        Ok(project.clone())
    }

    /// Archives a project at the expected revision.
    ///
    /// # Errors
    /// Returns [``ActionsError``] when validation, identity, lifecycle, or revision checks fail.
    pub fn archive_project(
        &mut self,
        id: ProjectId,
        expected_revision: u64,
    ) -> Result<Project, ActionsError> {
        let project = self
            .projects
            .get_mut(&id)
            .ok_or(ActionsError::NotFound(Entity::Project))?;
        if project.state == ProjectState::Archived {
            return Ok(project.clone());
        }
        ensure_revision(project.revision, expected_revision, Entity::Project)?;
        project.state = ProjectState::Archived;
        project.revision += 1;
        Ok(project.clone())
    }

    /// Creates an inbox task or returns the original snapshot for an exact replay.
    ///
    /// # Errors
    /// Returns [``ActionsError``] when validation, identity, lifecycle, or revision checks fail.
    pub fn create_task(
        &mut self,
        id: TaskId,
        title: impl Into<String>,
    ) -> Result<Task, ActionsError> {
        let title = normalize_required(title.into(), Field::TaskTitle)?;
        if let Some(existing) = self.tasks.get(&id) {
            if existing.title == title {
                return Ok(existing.clone());
            }
            return Err(ActionsError::ConflictingCreate(Entity::Task));
        }
        let task = Task {
            id,
            revision: 0,
            title,
            state: TaskState::Inbox,
            project_id: None,
            estimated_duration: None,
        };
        self.tasks.insert(id, task.clone());
        Ok(task)
    }

    /// Replaces a task title at the expected revision.
    ///
    /// # Errors
    /// Returns [``ActionsError``] when validation, identity, lifecycle, or revision checks fail.
    pub fn edit_task(
        &mut self,
        id: TaskId,
        expected_revision: u64,
        title: impl Into<String>,
    ) -> Result<Task, ActionsError> {
        let title = normalize_required(title.into(), Field::TaskTitle)?;
        let task = self
            .tasks
            .get_mut(&id)
            .ok_or(ActionsError::NotFound(Entity::Task))?;
        if task.title == title {
            return Ok(task.clone());
        }
        ensure_revision(task.revision, expected_revision, Entity::Task)?;
        task.title = title;
        task.revision += 1;
        Ok(task.clone())
    }

    /// Links or unlinks a task and project at the expected task revision.
    ///
    /// # Errors
    /// Returns [``ActionsError``] when validation, identity, lifecycle, or revision checks fail.
    pub fn link_task(
        &mut self,
        task_id: TaskId,
        expected_revision: u64,
        project_id: Option<ProjectId>,
    ) -> Result<Task, ActionsError> {
        let task = self
            .tasks
            .get(&task_id)
            .ok_or(ActionsError::NotFound(Entity::Task))?;
        if task.project_id == project_id {
            return Ok(task.clone());
        }
        if let Some(project_id) = project_id {
            let project = self
                .projects
                .get(&project_id)
                .ok_or(ActionsError::NotFound(Entity::Project))?;
            if project.state == ProjectState::Archived {
                return Err(ActionsError::ArchivedProject);
            }
        }
        let task = self
            .tasks
            .get_mut(&task_id)
            .ok_or(ActionsError::NotFound(Entity::Task))?;
        ensure_revision(task.revision, expected_revision, Entity::Task)?;
        task.project_id = project_id;
        task.revision += 1;
        Ok(task.clone())
    }

    /// Sets or clears the estimated duration used by planning.
    ///
    /// # Errors
    /// Returns [`ActionsError`] when the task is missing or the revision is obsolete.
    pub fn set_task_duration(
        &mut self,
        id: TaskId,
        expected_revision: u64,
        duration: Option<TaskDuration>,
    ) -> Result<Task, ActionsError> {
        let task = self
            .tasks
            .get_mut(&id)
            .ok_or(ActionsError::NotFound(Entity::Task))?;
        if task.estimated_duration == duration {
            return Ok(task.clone());
        }
        ensure_revision(task.revision, expected_revision, Entity::Task)?;
        task.estimated_duration = duration;
        task.revision += 1;
        Ok(task.clone())
    }

    /// Returns tasks whose lifecycle permits consideration by planning.
    #[must_use]
    pub fn eligible_tasks(&self) -> Vec<Task> {
        self.tasks
            .values()
            .filter(|task| {
                matches!(
                    task.state,
                    TaskState::Inbox | TaskState::Planned | TaskState::Postponed
                )
            })
            .cloned()
            .collect()
    }
    /// Applies an allowed task lifecycle transition at the expected revision.
    ///
    /// # Errors
    /// Returns [``ActionsError``] when validation, identity, lifecycle, or revision checks fail.
    pub fn transition_task(
        &mut self,
        id: TaskId,
        expected_revision: u64,
        destination: TaskState,
    ) -> Result<Task, ActionsError> {
        let task = self
            .tasks
            .get_mut(&id)
            .ok_or(ActionsError::NotFound(Entity::Task))?;
        if task.state == destination {
            return Ok(task.clone());
        }
        ensure_revision(task.revision, expected_revision, Entity::Task)?;
        if !transition_allowed(task.state, destination) {
            return Err(ActionsError::InvalidTransition {
                from: task.state,
                to: destination,
            });
        }
        task.state = destination;
        task.revision += 1;
        Ok(task.clone())
    }

    #[must_use]
    /// Returns a project snapshot when it exists.
    pub fn project(&self, id: ProjectId) -> Option<Project> {
        self.projects.get(&id).cloned()
    }
    #[must_use]
    /// Returns a task snapshot when it exists.
    pub fn task(&self, id: TaskId) -> Option<Task> {
        self.tasks.get(&id).cloned()
    }
    /// Returns every project snapshot, including archived projects.
    #[must_use]
    pub fn projects(&self) -> Vec<Project> {
        self.projects.values().cloned().collect()
    }

    /// Returns every task snapshot.
    #[must_use]
    pub fn tasks(&self) -> Vec<Task> {
        self.tasks.values().cloned().collect()
    }

    /// Returns active projects in stable identifier order.
    #[must_use]
    pub fn active_projects(&self) -> Vec<Project> {
        self.projects
            .values()
            .filter(|project| project.state == ProjectState::Active)
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Entity category reported by an operation error.
pub enum Entity {
    /// A minimal project.
    Project,
    /// A task/action.
    Task,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Text field reported by deterministic validation.
pub enum Field {
    /// Required project name.
    ProjectName,
    /// Optional project description.
    ProjectDescription,
    /// Required task title.
    TaskTitle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Deterministic failure returned by task and project operations.
pub enum ActionsError {
    /// The requested entity does not exist.
    NotFound(Entity),
    /// A create replay conflicts with the original data.
    ConflictingCreate(Entity),
    /// A text field failed deterministic validation.
    InvalidText(Field),
    /// An operation attempted to modify or link an archived project.
    ArchivedProject,
    /// The supplied revision does not match the current revision.
    RevisionConflict {
        /// Entity whose revision was checked.
        entity: Entity,
        /// Revision supplied by the caller.
        expected: u64,
        /// Current entity revision.
        actual: u64,
    },
    /// The requested task lifecycle edge is not allowed.
    InvalidTransition {
        /// Current task state.
        from: TaskState,
        /// Requested destination state.
        to: TaskState,
    },
}

fn normalize_required(value: String, field: Field) -> Result<String, ActionsError> {
    let normalized = value.trim();
    if normalized.is_empty() || normalized.chars().any(char::is_control) {
        return Err(ActionsError::InvalidText(field));
    }
    if normalized.len() == value.len() {
        Ok(value)
    } else {
        Ok(normalized.to_owned())
    }
}

fn normalize_optional(value: Option<String>, field: Field) -> Result<Option<String>, ActionsError> {
    value
        .map(|value| {
            let normalized = value.trim();
            if normalized.chars().any(char::is_control) {
                Err(ActionsError::InvalidText(field))
            } else if normalized.is_empty() {
                Ok(None)
            } else {
                Ok(Some(normalized.to_owned()))
            }
        })
        .transpose()
        .map(Option::flatten)
}

fn ensure_revision(actual: u64, expected: u64, entity: Entity) -> Result<(), ActionsError> {
    if actual == expected {
        Ok(())
    } else {
        Err(ActionsError::RevisionConflict {
            entity,
            expected,
            actual,
        })
    }
}

const fn transition_allowed(from: TaskState, to: TaskState) -> bool {
    matches!(
        (from, to),
        (
            TaskState::Inbox | TaskState::Postponed,
            TaskState::Planned | TaskState::Cancelled
        ) | (
            TaskState::Planned,
            TaskState::InProgress | TaskState::Postponed | TaskState::Cancelled
        ) | (
            TaskState::InProgress,
            TaskState::Completed | TaskState::Postponed | TaskState::Cancelled
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project_id() -> ProjectId {
        ProjectId::new(1).expect("valid project id")
    }
    fn task_id() -> TaskId {
        TaskId::new(1).expect("valid task id")
    }

    #[test]
    fn project_contract_is_minimal_and_create_is_idempotent() {
        let mut owner = ActionsAndProjects::default();
        let created = owner
            .create_project(project_id(), " Alpha ", Some(" MVP ".to_owned()))
            .expect("create");
        let replay = owner
            .create_project(project_id(), "Alpha", Some("MVP".to_owned()))
            .expect("replay");
        assert_eq!(created, replay);
        assert_eq!(created.revision(), 0);
        assert_eq!(created.name(), "Alpha");
        assert_eq!(created.description(), Some("MVP"));
        assert_eq!(created.state(), ProjectState::Active);
    }

    #[test]
    fn create_replays_remain_idempotent_after_later_changes() {
        let mut owner = ActionsAndProjects::default();
        owner
            .create_project(project_id(), "Alpha", None)
            .expect("project");
        owner.create_task(task_id(), "Ship MVP").expect("task");
        owner
            .link_task(task_id(), 0, Some(project_id()))
            .expect("link");
        owner.archive_project(project_id(), 0).expect("archive");

        let project_replay = owner
            .create_project(project_id(), "Alpha", None)
            .expect("project replay");
        let task_replay = owner
            .create_task(task_id(), "Ship MVP")
            .expect("task replay");
        let link_replay = owner
            .link_task(task_id(), 0, Some(project_id()))
            .expect("link replay");

        assert_eq!(project_replay.state(), ProjectState::Archived);
        assert_eq!(task_replay.project_id(), Some(project_id()));
        assert_eq!(link_replay, task_replay);
    }
    #[test]
    fn rejects_conflicting_create_and_obsolete_project_edit() {
        let mut owner = ActionsAndProjects::default();
        owner
            .create_project(project_id(), "Alpha", None)
            .expect("create");
        assert_eq!(
            owner.create_project(project_id(), "Beta", None),
            Err(ActionsError::ConflictingCreate(Entity::Project))
        );
        owner
            .edit_project(project_id(), 0, "Beta", None)
            .expect("edit");
        assert_eq!(
            owner.edit_project(project_id(), 0, "Gamma", None),
            Err(ActionsError::RevisionConflict {
                entity: Entity::Project,
                expected: 0,
                actual: 1
            })
        );
    }

    #[test]
    fn archive_is_idempotent_and_blocks_new_links() {
        let mut owner = ActionsAndProjects::default();
        owner
            .create_project(project_id(), "Alpha", None)
            .expect("project");
        owner.create_task(task_id(), "Ship MVP").expect("task");
        let archived = owner.archive_project(project_id(), 0).expect("archive");
        let replay = owner.archive_project(project_id(), 0).expect("replay");
        assert_eq!(archived, replay);
        assert_eq!(archived.state(), ProjectState::Archived);
        assert_eq!(
            owner.link_task(task_id(), 0, Some(project_id())),
            Err(ActionsError::ArchivedProject)
        );
        assert!(owner.active_projects().is_empty());
    }

    #[test]
    fn task_link_can_be_added_removed_and_replayed() {
        let mut owner = ActionsAndProjects::default();
        owner
            .create_project(project_id(), "Alpha", None)
            .expect("project");
        owner.create_task(task_id(), "Ship MVP").expect("task");
        let linked = owner
            .link_task(task_id(), 0, Some(project_id()))
            .expect("link");
        let replay = owner
            .link_task(task_id(), 0, Some(project_id()))
            .expect("replay");
        assert_eq!(linked, replay);
        assert_eq!(linked.project_id(), Some(project_id()));
        let unlinked = owner.link_task(task_id(), 1, None).expect("unlink");
        assert_eq!(unlinked.project_id(), None);
        assert_eq!(unlinked.revision(), 2);
    }

    #[test]
    fn planning_duration_is_optional_revision_safe_and_idempotent() {
        let mut owner = ActionsAndProjects::default();
        let task = owner.create_task(task_id(), "Ship MVP").expect("task");
        assert_eq!(task.estimated_duration(), None);
        assert_eq!(TaskDuration::new(0), None);
        let duration = TaskDuration::new(45).expect("duration");
        let updated = owner
            .set_task_duration(task_id(), 0, Some(duration))
            .expect("duration change");
        let replay = owner
            .set_task_duration(task_id(), 0, Some(duration))
            .expect("replay");
        assert_eq!(updated, replay);
        assert_eq!(updated.estimated_duration(), Some(duration));
        assert_eq!(
            owner.set_task_duration(task_id(), 0, None),
            Err(ActionsError::RevisionConflict {
                entity: Entity::Task,
                expected: 0,
                actual: 1
            })
        );
        assert_eq!(owner.eligible_tasks(), vec![updated]);
    }
    #[test]
    fn lifecycle_accepts_contract_path_and_rejects_invalid_transition() {
        let mut owner = ActionsAndProjects::default();
        owner.create_task(task_id(), "Ship MVP").expect("task");
        let planned = owner
            .transition_task(task_id(), 0, TaskState::Planned)
            .expect("plan");
        let started = owner
            .transition_task(task_id(), planned.revision(), TaskState::InProgress)
            .expect("start");
        let completed = owner
            .transition_task(task_id(), started.revision(), TaskState::Completed)
            .expect("complete");
        assert_eq!(completed.state(), TaskState::Completed);
        assert_eq!(
            owner.transition_task(task_id(), completed.revision(), TaskState::Planned),
            Err(ActionsError::InvalidTransition {
                from: TaskState::Completed,
                to: TaskState::Planned
            })
        );
    }

    #[test]
    fn task_changes_reject_obsolete_revisions_but_exact_replays_are_idempotent() {
        let mut owner = ActionsAndProjects::default();
        owner.create_task(task_id(), "Ship MVP").expect("task");
        owner.edit_task(task_id(), 0, "Release MVP").expect("edit");
        assert_eq!(
            owner.edit_task(task_id(), 0, "Another title"),
            Err(ActionsError::RevisionConflict {
                entity: Entity::Task,
                expected: 0,
                actual: 1
            })
        );
        let replay = owner
            .edit_task(task_id(), 0, "Release MVP")
            .expect("replay");
        assert_eq!(replay.revision(), 1);
    }
}
