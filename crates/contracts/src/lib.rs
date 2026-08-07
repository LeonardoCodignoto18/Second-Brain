//! Canonical, versioned types that may cross an application boundary.
//!
//! This crate owns representation only. It contains no domain policy, transport,
//! persistence, provider, operating-system, or framework dependency.

use serde::{Deserialize, Serialize};

/// Current major version of the foundation IPC contract.
pub const FOUNDATION_CONTRACT_VERSION: u16 = 1;

/// Read-only metadata used to prove the desktop-to-core boundary is operational.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FoundationStatus {
    /// Human-readable product name.
    pub product_name: String,
    /// Version of the native application.
    pub application_version: String,
    /// Version of this response contract.
    pub contract_version: u16,
}

/// Health facts for the local encrypted store.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StorageHealthDto {
    /// Runtime `SQLCipher` version.
    pub cipher_version: String,
    /// Current embedded schema version.
    pub schema_version: u32,
}

/// Minimal project representation crossing IPC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProjectDto {
    /// Stable local identifier.
    /// Stable local identifier.
    pub id: u64,
    /// Optimistic concurrency revision.
    pub revision: u64,
    /// Required project name.
    /// Required project name.
    pub name: String,
    /// Optional description.
    /// Optional project description.
    pub description: Option<String>,
    /// Whether the project is archived.
    pub archived: bool,
}

/// Minimal task representation crossing IPC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TaskDto {
    /// Stable local identifier.
    /// Stable local identifier.
    pub id: u64,
    /// Optimistic concurrency revision.
    pub revision: u64,
    /// Required task title.
    /// Required task title.
    pub title: String,
    /// Canonical lifecycle state.
    pub state: String,
    /// Optional linked project.
    /// Optional project link.
    pub project_id: Option<u64>,
    /// Optional estimated duration in minutes.
    /// Optional positive estimate.
    pub estimated_minutes: Option<u16>,
}

/// Complete local projection needed by the initial integrated interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WorkspaceSnapshot {
    /// Active and archived projects.
    pub projects: Vec<ProjectDto>,
    /// All current tasks.
    pub tasks: Vec<TaskDto>,
    /// Verified persistence health.
    pub storage: StorageHealthDto,
}

/// Input for creating a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreateProjectRequest {
    /// Required project name.
    pub name: String,
    /// Optional project description.
    pub description: Option<String>,
}

/// Input for creating a task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreateTaskRequest {
    /// Required task title.
    pub title: String,
    /// Optional project link.
    pub project_id: Option<u64>,
    /// Optional positive estimate.
    pub estimated_minutes: Option<u16>,
}

/// Input for a revision-safe task transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransitionTaskRequest {
    /// Stable local identifier.
    pub id: u64,
    /// Optimistic concurrency revision.
    pub expected_revision: u64,
    /// Canonical destination state.
    pub destination: String,
}

/// Input for archiving a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ArchiveProjectRequest {
    /// Stable local identifier.
    pub id: u64,
    /// Optimistic concurrency revision.
    pub expected_revision: u64,
}

/// Stable safe error returned across IPC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct IpcError {
    /// Stable machine-readable code.
    pub code: String,
    /// Safe user-facing summary.
    pub message: String,
}
