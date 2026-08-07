import { invoke } from "@tauri-apps/api/core";

export interface ProjectDto {
  id: number;
  revision: number;
  name: string;
  description: string | null;
  archived: boolean;
}
export interface TaskDto {
  id: number;
  revision: number;
  title: string;
  state:
    | "inbox"
    | "planned"
    | "in_progress"
    | "completed"
    | "postponed"
    | "cancelled";
  projectId: number | null;
  estimatedMinutes: number | null;
}
export interface WorkspaceSnapshot {
  projects: ProjectDto[];
  tasks: TaskDto[];
  storage: { cipherVersion: string; schemaVersion: number };
}
export interface IpcError {
  code: string;
  message: string;
}

export const workspace = {
  load: () => invoke<WorkspaceSnapshot>("workspace_snapshot"),
  createProject: (name: string, description: string | null) =>
    invoke<WorkspaceSnapshot>("create_project", {
      request: { name, description },
    }),
  archiveProject: (project: ProjectDto) =>
    invoke<WorkspaceSnapshot>("archive_project", {
      request: { id: project.id, expectedRevision: project.revision },
    }),
  createTask: (
    title: string,
    projectId: number | null,
    estimatedMinutes: number | null,
  ) =>
    invoke<WorkspaceSnapshot>("create_task", {
      request: { title, projectId, estimatedMinutes },
    }),
  transitionTask: (task: TaskDto, destination: TaskDto["state"]) =>
    invoke<WorkspaceSnapshot>("transition_task", {
      request: { id: task.id, expectedRevision: task.revision, destination },
    }),
};
