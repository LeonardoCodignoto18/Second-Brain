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
export interface DailyAvailabilityDto {
  day: string;
  startMinute: number;
  endMinute: number;
  revision: number;
}
export interface PlanDraftDto {
  id: number;
  revision: number;
  priorityTaskIds: number[];
  eligibleTaskIds: number[];
  missingDurationTaskIds: number[];
  contextComplete: boolean;
  replanning: boolean;
}
export interface NowDto {
  day: string;
  planId: number;
  revision: number;
  currentTaskId: number | null;
  remainingTaskIds: number[];
  focusState: "active" | "paused" | null;
  replanReason: "priority_postponed" | "plan_exhausted" | null;
}
export interface WorkspaceSnapshot {
  projects: ProjectDto[];
  tasks: TaskDto[];
  storage: { cipherVersion: string; schemaVersion: number };
  dailyCycle: {
    availability: DailyAvailabilityDto | null;
    draft: PlanDraftDto | null;
    now: NowDto | null;
  };
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
  configureAvailability: (
    day: string,
    startMinute: number,
    endMinute: number,
    expectedRevision: number,
  ) =>
    invoke<WorkspaceSnapshot>("configure_daily_availability", {
      request: { day, startMinute, endMinute, expectedRevision },
    }),
  proposePlan: (day: string) =>
    invoke<WorkspaceSnapshot>("propose_daily_plan", { request: { day } }),
  approvePlan: (draft: PlanDraftDto, selectedTaskIds: number[] | null) =>
    invoke<WorkspaceSnapshot>("approve_daily_plan", {
      request: {
        draftId: draft.id,
        expectedRevision: draft.revision,
        selectedTaskIds,
      },
    }),
  startFocus: (now: NowDto) =>
    invoke<WorkspaceSnapshot>("start_focus", {
      request: { expectedRevision: now.revision },
    }),
  completeCurrent: (now: NowDto) =>
    invoke<WorkspaceSnapshot>("complete_current", {
      request: { expectedRevision: now.revision },
    }),
  postponeCurrent: (now: NowDto) =>
    invoke<WorkspaceSnapshot>("postpone_current", {
      request: { expectedRevision: now.revision },
    }),
};
