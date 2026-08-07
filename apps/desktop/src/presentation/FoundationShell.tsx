import { FormEvent, useEffect, useMemo, useState } from "react";

import {
  type IpcError,
  type TaskDto,
  type WorkspaceSnapshot,
  workspace,
} from "../application/workspace";

type View = "today" | "inbox" | "projects";

export function FoundationShell() {
  const [data, setData] = useState<WorkspaceSnapshot | null>(null);
  const [view, setView] = useState<View>("today");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    void run(workspace.load);
  }, []);

  async function run(operation: () => Promise<WorkspaceSnapshot>) {
    setBusy(true);
    setError(null);
    try {
      setData(await operation());
    } catch (cause) {
      setError(readError(cause));
    } finally {
      setBusy(false);
    }
  }

  const activeTasks = useMemo(
    () =>
      data?.tasks.filter(
        (task) => !["completed", "cancelled"].includes(task.state),
      ) ?? [],
    [data],
  );
  const now =
    activeTasks.find((task) => task.state === "in_progress") ??
    activeTasks.find((task) => task.state === "planned");

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand">
          <span className="brand-mark">S</span>
          <div>
            <strong>Second Brain</strong>
            <small>Seu dia, em ordem</small>
          </div>
        </div>
        <nav aria-label="Navegação principal">
          <NavButton
            active={view === "today"}
            onClick={() => setView("today")}
            icon="◎"
          >
            Agora
          </NavButton>
          <NavButton
            active={view === "inbox"}
            onClick={() => setView("inbox")}
            icon="⌁"
          >
            Ações <span>{activeTasks.length}</span>
          </NavButton>
          <NavButton
            active={view === "projects"}
            onClick={() => setView("projects")}
            icon="◇"
          >
            Projetos
          </NavButton>
        </nav>
        <div className="privacy">
          <span className="status-dot" />
          Local First
          <small>
            {data
              ? `SQLCipher ${data.storage.cipherVersion}`
              : "Abrindo dados locais…"}
          </small>
        </div>
      </aside>

      <main aria-label="Second Brain OS">
        <header>
          <div>
            <p className="eyebrow">QUINTA-FEIRA · 06 DE AGOSTO</p>
            <h1>
              {view === "today"
                ? "O que importa agora"
                : view === "inbox"
                  ? "Suas ações"
                  : "Seus projetos"}
            </h1>
          </div>
          <span className="calm-status">Tudo sob controle</span>
        </header>
        {error && (
          <div className="error" role="alert">
            {error}
            <button onClick={() => void run(workspace.load)}>
              Tentar novamente
            </button>
          </div>
        )}
        {!data ? (
          <Loading />
        ) : view === "today" ? (
          <Today data={data} now={now} busy={busy} run={run} />
        ) : view === "inbox" ? (
          <Inbox data={data} busy={busy} run={run} />
        ) : (
          <Projects data={data} busy={busy} run={run} />
        )}
      </main>
    </div>
  );
}

function Today({
  data,
  now,
  busy,
  run,
}: {
  data: WorkspaceSnapshot;
  now: TaskDto | undefined;
  busy: boolean;
  run: (operation: () => Promise<WorkspaceSnapshot>) => Promise<void>;
}) {
  const next = data.tasks.filter(
    (task) => task.state === "planned" && task.id !== now?.id,
  );
  return (
    <section className="today-grid">
      <article className="now-card">
        <p className="section-label">AGORA</p>
        {now ? (
          <>
            <h2>{now.title}</h2>
            <p>
              {now.estimatedMinutes
                ? `${now.estimatedMinutes} minutos reservados`
                : "Duração ainda não definida"}
            </p>
            <div className="actions">
              {now.state === "planned" && (
                <button
                  className="primary"
                  disabled={busy}
                  onClick={() =>
                    void run(() => workspace.transitionTask(now, "in_progress"))
                  }
                >
                  Iniciar foco
                </button>
              )}
              {now.state === "in_progress" && (
                <button
                  className="primary"
                  disabled={busy}
                  onClick={() =>
                    void run(() => workspace.transitionTask(now, "completed"))
                  }
                >
                  Concluir
                </button>
              )}
              <button
                className="quiet"
                disabled={busy}
                onClick={() =>
                  void run(() => workspace.transitionTask(now, "postponed"))
                }
              >
                Adiar
              </button>
            </div>
          </>
        ) : (
          <Empty
            title="Seu Agora está livre"
            text="Planeje uma ação para transformar intenção em direção."
          />
        )}
      </article>
      <article className="panel">
        <div className="panel-title">
          <h3>Depois</h3>
          <span>{next.length}</span>
        </div>
        {next.length ? (
          next.map((task) => <TaskRow key={task.id} task={task} data={data} />)
        ) : (
          <p className="muted">Nenhuma outra prioridade planejada.</p>
        )}
      </article>
    </section>
  );
}

function Inbox({
  data,
  busy,
  run,
}: {
  data: WorkspaceSnapshot;
  busy: boolean;
  run: (operation: () => Promise<WorkspaceSnapshot>) => Promise<void>;
}) {
  const [title, setTitle] = useState("");
  const [minutes, setMinutes] = useState("");
  const [project, setProject] = useState("");
  const tasks = data.tasks.filter(
    (task) => !["completed", "cancelled"].includes(task.state),
  );
  function submit(event: FormEvent) {
    event.preventDefault();
    const clean = title.trim();
    if (!clean) return;
    void run(() =>
      workspace.createTask(
        clean,
        project ? Number(project) : null,
        minutes ? Number(minutes) : null,
      ),
    ).then(() => {
      setTitle("");
      setMinutes("");
    });
  }
  return (
    <section>
      <form className="capture" onSubmit={submit}>
        <input
          aria-label="Nova ação"
          placeholder="Capture o que precisa ser feito…"
          value={title}
          onChange={(event) => setTitle(event.target.value)}
        />
        <select
          aria-label="Projeto"
          value={project}
          onChange={(event) => setProject(event.target.value)}
        >
          <option value="">Sem projeto</option>
          {data.projects
            .filter((item) => !item.archived)
            .map((item) => (
              <option key={item.id} value={item.id}>
                {item.name}
              </option>
            ))}
        </select>
        <input
          className="minutes"
          aria-label="Minutos"
          type="number"
          min="1"
          placeholder="min"
          value={minutes}
          onChange={(event) => setMinutes(event.target.value)}
        />
        <button className="primary" disabled={busy}>
          Adicionar
        </button>
      </form>
      <div className="list-panel">
        {tasks.length ? (
          tasks.map((task) => (
            <div className="task-line" key={task.id}>
              <TaskRow task={task} data={data} />
              <div className="row-actions">
                {["inbox", "postponed"].includes(task.state) && (
                  <button
                    disabled={busy}
                    onClick={() =>
                      void run(() => workspace.transitionTask(task, "planned"))
                    }
                  >
                    Planejar
                  </button>
                )}
                {task.state === "planned" && (
                  <button
                    disabled={busy}
                    onClick={() =>
                      void run(() =>
                        workspace.transitionTask(task, "in_progress"),
                      )
                    }
                  >
                    Iniciar
                  </button>
                )}
                {task.state === "in_progress" && (
                  <button
                    disabled={busy}
                    onClick={() =>
                      void run(() =>
                        workspace.transitionTask(task, "completed"),
                      )
                    }
                  >
                    Concluir
                  </button>
                )}
              </div>
            </div>
          ))
        ) : (
          <Empty
            title="Nada pendente"
            text="Capture uma ação e libere espaço mental."
          />
        )}
      </div>
    </section>
  );
}

function Projects({
  data,
  busy,
  run,
}: {
  data: WorkspaceSnapshot;
  busy: boolean;
  run: (operation: () => Promise<WorkspaceSnapshot>) => Promise<void>;
}) {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  function submit(event: FormEvent) {
    event.preventDefault();
    const clean = name.trim();
    if (!clean) return;
    void run(() =>
      workspace.createProject(clean, description.trim() || null),
    ).then(() => {
      setName("");
      setDescription("");
    });
  }
  return (
    <section>
      <form className="capture project-capture" onSubmit={submit}>
        <input
          aria-label="Nome do projeto"
          placeholder="Novo projeto"
          value={name}
          onChange={(event) => setName(event.target.value)}
        />
        <input
          aria-label="Descrição"
          placeholder="Descrição opcional"
          value={description}
          onChange={(event) => setDescription(event.target.value)}
        />
        <button className="primary" disabled={busy}>
          Criar
        </button>
      </form>
      <div className="project-grid">
        {data.projects.map((project) => (
          <article
            className={`project-card ${project.archived ? "archived" : ""}`}
            key={project.id}
          >
            <div className="project-symbol">◇</div>
            <h3>{project.name}</h3>
            <p>{project.description ?? "Sem descrição"}</p>
            <small>
              {
                data.tasks.filter(
                  (task) =>
                    task.projectId === project.id && task.state !== "completed",
                ).length
              }{" "}
              ações abertas
            </small>
            {!project.archived && (
              <button
                className="quiet"
                disabled={busy}
                onClick={() =>
                  void run(() => workspace.archiveProject(project))
                }
              >
                Arquivar
              </button>
            )}
          </article>
        ))}
      </div>
    </section>
  );
}

function TaskRow({ task, data }: { task: TaskDto; data: WorkspaceSnapshot }) {
  const project = data.projects.find((item) => item.id === task.projectId);
  return (
    <div className="task">
      <span className={`task-state ${task.state}`} />
      <div>
        <strong>{task.title}</strong>
        <small>
          {[
            project?.name,
            task.estimatedMinutes ? `${task.estimatedMinutes} min` : null,
          ]
            .filter(Boolean)
            .join(" · ") || "Sem contexto"}
        </small>
      </div>
    </div>
  );
}
function NavButton({
  active,
  onClick,
  icon,
  children,
}: {
  active: boolean;
  onClick: () => void;
  icon: string;
  children: React.ReactNode;
}) {
  return (
    <button className={active ? "active" : ""} onClick={onClick}>
      <b>{icon}</b>
      {children}
    </button>
  );
}
function Empty({ title, text }: { title: string; text: string }) {
  return (
    <div className="empty">
      <span>✓</span>
      <h3>{title}</h3>
      <p>{text}</p>
    </div>
  );
}
function Loading() {
  return (
    <div className="loading">
      <span />
      <p>Organizando seu contexto local…</p>
    </div>
  );
}
function readError(cause: unknown) {
  if (typeof cause === "object" && cause !== null && "message" in cause)
    return String((cause as IpcError).message);
  return "Não foi possível concluir a operação.";
}
