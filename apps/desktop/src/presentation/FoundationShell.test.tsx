import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { FoundationShell } from "./FoundationShell";

const { invoke } = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke }));

const snapshot = {
  projects: [
    { id: 1, revision: 0, name: "Alpha", description: null, archived: false },
  ],
  tasks: [
    {
      id: 1,
      revision: 0,
      title: "Primeira ação",
      state: "inbox",
      projectId: 1,
      estimatedMinutes: 30,
    },
  ],
  storage: { cipherVersion: "4.14.0 community", schemaVersion: 1 },
};

describe("integrated desktop shell", () => {
  beforeEach(() => {
    invoke.mockReset();
    invoke.mockResolvedValue(snapshot);
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
});
