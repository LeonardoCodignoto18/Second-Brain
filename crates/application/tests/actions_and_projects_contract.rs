//! Public semantic contract tests for the Actions and Projects domain.

use second_brain_application::{
    ActionsAndProjects, ActionsError, Entity, ProjectId, ProjectState, TaskId, TaskState,
};

fn project_id() -> ProjectId {
    ProjectId::new(10).expect("non-zero project id")
}

fn task_id() -> TaskId {
    TaskId::new(20).expect("non-zero task id")
}

#[test]
fn public_contract_supports_the_minimal_project_and_task_lifecycle() {
    let mut owner = ActionsAndProjects::default();

    let project = owner
        .create_project(project_id(), "Second Brain", Some("Alpha".to_owned()))
        .expect("create project");
    let task = owner
        .create_task(task_id(), "Build core")
        .expect("create task");
    let linked = owner
        .link_task(task.id(), task.revision(), Some(project.id()))
        .expect("link task");
    let planned = owner
        .transition_task(linked.id(), linked.revision(), TaskState::Planned)
        .expect("plan task");
    let started = owner
        .transition_task(planned.id(), planned.revision(), TaskState::InProgress)
        .expect("start task");
    let completed = owner
        .transition_task(started.id(), started.revision(), TaskState::Completed)
        .expect("complete task");
    let archived = owner
        .archive_project(project.id(), project.revision())
        .expect("archive project");

    assert_eq!(completed.state(), TaskState::Completed);
    assert_eq!(completed.project_id(), Some(project.id()));
    assert_eq!(archived.state(), ProjectState::Archived);
}

#[test]
fn public_contract_replays_exact_changes_and_rejects_obsolete_differences() {
    let mut owner = ActionsAndProjects::default();
    let task = owner
        .create_task(task_id(), "Build core")
        .expect("create task");
    let edited = owner
        .edit_task(task.id(), task.revision(), "Validate core")
        .expect("edit task");

    let replay = owner
        .edit_task(task.id(), task.revision(), "Validate core")
        .expect("exact replay");
    let stale = owner.edit_task(task.id(), task.revision(), "Replace core");

    assert_eq!(replay, edited);
    assert_eq!(
        stale,
        Err(ActionsError::RevisionConflict {
            entity: Entity::Task,
            expected: task.revision(),
            actual: edited.revision(),
        })
    );
}
