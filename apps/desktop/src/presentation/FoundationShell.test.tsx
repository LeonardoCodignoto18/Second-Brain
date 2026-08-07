import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { FoundationShell } from "./FoundationShell";

const { invoke } = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke }));

const task = {
  id: 1,
  revision: 0,
  title: "Primeira ação",
  state: "inbox",
  projectId: 1,
  estimatedMinutes: 30,
};
const base = {
  projects: [
    { id: 1, revision: 0, name: "Alpha", description: null, archived: false },
  ],
  tasks: [task],
  storage: { cipherVersion: "4.14.0 community", schemaVersion: 1 },
  dailyCycle: { availability: null, draft: null, now: null },
};

const proposal = {
  ...base,
  dailyCycle: {
    availability: {
      day: "2026-08-07",
      startMinute: 540,
      endMinute: 1080,
      revision: 1,
    },
    draft: {
      id: 1,
      revision: 0,
      priorityTaskIds: [1],
      eligibleTaskIds: [1],
      missingDurationTaskIds: [],
      contextComplete: true,
      replanning: false,
    },
    now: null,
  },
};

const active = {
  ...proposal,
  tasks: [{ ...task, revision: 1, state: "planned" }],
  dailyCycle: {
    ...proposal.dailyCycle,
    draft: null,
    now: {
      day: "2026-08-07",
      planId: 1,
      revision: 0,
      currentTaskId: 1,
      remainingTaskIds: [1],
      focusState: null,
      replanReason: null,
    },
  },
};

describe("integrated desktop shell", () => {
  beforeEach(() => {
    invoke.mockReset();
    invoke.mockResolvedValue(base);
  });

  it("loads the encrypted local projection and captures a task through IPC", async () => {
    render(<FoundationShell />);
    expect(await screen.findByText("O que importa agora")).toBeInTheDocument();
    expect(screen.getByText(/SQLCipher 4\.14\.0/)).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Ações/ }));
    fireEvent.change(screen.getByLabelText("Nova ação"), {
      target: { value: "Preparar entrega" },
    });
    fireEvent.change(screen.getByLabelText("Projeto"), {
      target: { value: "1" },
    });
    fireEvent.change(screen.getByLabelText("Minutos"), {
      target: { value: "45" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Adicionar" }));

    await waitFor(() =>
      expect(invoke).toHaveBeenCalledWith("create_task", {
        request: {
          title: "Preparar entrega",
          projectId: 1,
          estimatedMinutes: 45,
        },
      }),
    );
  });

  it("drives planning approval and Agora through the typed IPC boundary", async () => {
    invoke
      .mockResolvedValueOnce(base)
      .mockResolvedValueOnce({
        ...base,
        dailyCycle: {
          availability: {
            day: "2026-08-07",
            startMinute: 540,
            endMinute: 1080,
            revision: 1,
          },
          draft: null,
          now: null,
        },
      })
      .mockResolvedValueOnce(proposal)
      .mockResolvedValueOnce(active)
      .mockResolvedValueOnce({
        ...active,
        tasks: [{ ...task, revision: 2, state: "in_progress" }],
        dailyCycle: {
          ...active.dailyCycle,
          now: { ...active.dailyCycle.now, revision: 1, focusState: "active" },
        },
      });

    render(<FoundationShell />);
    expect(await screen.findByText(/Quanto do seu dia/)).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Criar plano do dia" }));
    expect(await screen.findByText("Primeira ação")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Aprovar 1" }));
    expect(
      await screen.findByRole("button", { name: "Iniciar foco" }),
    ).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Iniciar foco" }));

    await waitFor(() =>
      expect(invoke).toHaveBeenCalledWith("start_focus", {
        request: { expectedRevision: 0 },
      }),
    );
  });
  it("keeps incomplete proposals non-mutating and allows explicit rejection", async () => {
    const incomplete = {
      ...base,
      tasks: [{ ...task, estimatedMinutes: null }],
      dailyCycle: {
        availability: {
          day: "2026-08-07",
          startMinute: 540,
          endMinute: 1080,
          revision: 1,
        },
        draft: {
          id: 7,
          revision: 0,
          priorityTaskIds: [],
          eligibleTaskIds: [],
          missingDurationTaskIds: [1],
          contextComplete: true,
          replanning: false,
        },
        now: null,
      },
    };
    invoke.mockResolvedValueOnce(incomplete).mockResolvedValueOnce(base);

    render(<FoundationShell />);
    expect(await screen.findByText(/ficaram fora/)).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Selecione uma prioridade" }),
    ).toBeDisabled();
    expect(
      screen.getByRole("button", { name: "Aceitar sugestão" }),
    ).toBeDisabled();
    fireEvent.click(screen.getByRole("button", { name: "Agora não" }));

    await waitFor(() =>
      expect(invoke).toHaveBeenCalledWith("reject_daily_plan", {
        request: { draftId: 7, expectedRevision: 0 },
      }),
    );
  });
  it("requires explicit closure when Agora belongs to an earlier day", async () => {
    const previousDay = {
      ...active,
      dailyCycle: {
        ...active.dailyCycle,
        now: {
          ...active.dailyCycle.now,
          day: "2026-08-06",
        },
      },
    };
    invoke.mockResolvedValueOnce(previousDay).mockResolvedValueOnce(base);

    render(<FoundationShell />);
    const close = await screen.findByRole("button", {
      name: "Encerrar dia anterior",
    });
    fireEvent.click(close);

    await waitFor(() =>
      expect(invoke).toHaveBeenCalledWith("start_new_day", {
        request: { day: "2026-08-07", expectedRevision: 0 },
      }),
    );
  });
});
