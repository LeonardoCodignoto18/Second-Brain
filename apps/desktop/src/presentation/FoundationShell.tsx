import { FormEvent, useEffect, useMemo, useState } from "react";

import {
  type IpcError,
  type PlanDraftDto,
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
  const today = localDay();

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
            icon="◉"
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
            <p className="eyebrow">{humanDay(today).toUpperCase()}</p>
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
          <Today data={data} day={today} busy={busy} run={run} />
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
  day,
  busy,
  run,
}: {
  data: WorkspaceSnapshot;
  day: string;
  busy: boolean;
  run: (operation: () => Promise<WorkspaceSnapshot>) => Promise<void>;
}) {
  const { draft, now } = data.dailyCycle;
  const current = data.tasks.find((task) => task.id === now?.currentTaskId);
  const next =
    now?.remainingTaskIds
      .slice(1)
      .map((id) => data.tasks.find((task) => task.id === id))
      .filter((task): task is TaskDto => Boolean(task)) ?? [];

  if (now && now.day !== day) {
    return (
      <section className="planning-stage">
        <article className="now-card replan-card">
          <p className="section-label">NOVO DIA</p>
          <h2>O plano anterior ainda está aberto</h2>
          <p>
            As ações não concluídas serão adiadas conscientemente antes de você
            planejar hoje.
          </p>
          <button
            className="primary"
            disabled={busy}
            onClick={() => void run(() => workspace.startNewDay(day, now))}
          >
            Encerrar dia anterior
          </button>
        </article>
      </section>
    );
  }
  if (draft) {
    return <PlanProposal data={data} draft={draft} busy={busy} run={run} />;
  }

  if (!now) {
    return <PlanSetup data={data} day={day} busy={busy} run={run} />;
  }

  if (now.replanReason) {
    return (
      <section className="today-grid">
        <article className="now-card replan-card">
          <p className="section-label">REORGANIZAR</p>
          <h2>O plano precisa de uma nova decisão</h2>
          <p>
            {now.replanReason === "priority_postponed"
              ? "A prioridade foi adiada. Vamos escolher conscientemente o próximo passo."
              : "As prioridades planejadas terminaram. Você decide se o dia continua."}
          </p>
          <div className="actions">
            <button
              className="primary"
              disabled={busy}
              onClick={() => void run(() => workspace.proposePlan(now.day))}
            >
              Replanejar agora
            </button>
            <button
              className="quiet"
              disabled={busy}
              onClick={() => void run(() => workspace.dismissReplan(now))}
            >
              {current ? "Continuar plano atual" : "Encerrar por agora"}
            </button>
          </div>
        </article>
        <AfterPanel data={data} tasks={next} />
      </section>
    );
  }

  return (
    <section className="today-grid">
      <article className="now-card">
        <p className="section-label">AGORA</p>
        {current ? (
          <>
            <h2>{current.title}</h2>
            <p>
              {current.estimatedMinutes
                ? `${current.estimatedMinutes} minutos reservados`
                : "Duração ainda não definida"}
            </p>
            <div className="actions">
              {!now.focusState && (
                <button
                  className="primary"
                  disabled={busy}
                  onClick={() => void run(() => workspace.startFocus(now))}
                >
                  Iniciar foco
                </button>
              )}
              {now.focusState === "active" && (
                <button
                  className="primary"
                  disabled={busy}
                  onClick={() => void run(() => workspace.completeCurrent(now))}
                >
                  Concluir
                </button>
              )}
              <button
                className="quiet"
                disabled={busy}
                onClick={() => void run(() => workspace.postponeCurrent(now))}
              >
                Adiar
              </button>
            </div>
          </>
        ) : (
          <Empty
            title="Plano concluído"
            text="O sistema está preparando a próxima decisão do dia."
          />
        )}
      </article>
      <AfterPanel data={data} tasks={next} />
    </section>
  );
}

function PlanSetup({
  data,
  day,
  busy,
  run,
}: {
  data: WorkspaceSnapshot;
  day: string;
  busy: boolean;
  run: (operation: () => Promise<WorkspaceSnapshot>) => Promise<void>;
}) {
  const availability = data.dailyCycle.availability;
  const [start, setStart] = useState(
    availability ? minuteToTime(availability.startMinute) : "09:00",
  );
  const [end, setEnd] = useState(
    availability ? minuteToTime(availability.endMinute) : "18:00",
  );
  const eligible = data.tasks.filter(
    (task) =>
      ["inbox", "planned", "postponed"].includes(task.state) &&
      task.estimatedMinutes,
  );

  function submit(event: FormEvent) {
    event.preventDefault();
    void run(async () => {
      await workspace.configureAvailability(
        day,
        timeToMinute(start),
        timeToMinute(end),
        availability?.revision ?? 0,
      );
      return workspace.proposePlan(day);
    });
  }

  return (
    <section className="planning-stage">
      <article className="now-card plan-setup">
        <p className="section-label">ANTES DE COMEÇAR</p>
        <h2>Quanto do seu dia está realmente disponível?</h2>
        <p>
          O plano usa somente ações com duração definida que cabem nessa janela.
        </p>
        <form className="availability-form" onSubmit={submit}>
          <label>
            Começo
            <input
              aria-label="Início da disponibilidade"
              type="time"
              required
              value={start}
              onChange={(event) => setStart(event.target.value)}
            />
          </label>
          <label>
            Fim
            <input
              aria-label="Fim da disponibilidade"
              type="time"
              required
              value={end}
              onChange={(event) => setEnd(event.target.value)}
            />
          </label>
          <button className="primary" disabled={busy || !eligible.length}>
            Criar plano do dia
          </button>
        </form>
        {!eligible.length && (
          <p className="planning-hint">
            Capture ao menos uma ação com duração para criar um plano confiável.
          </p>
        )}
      </article>
    </section>
  );
}

function PlanProposal({
  data,
  draft,
  busy,
  run,
}: {
  data: WorkspaceSnapshot;
  draft: PlanDraftDto;
  busy: boolean;
  run: (operation: () => Promise<WorkspaceSnapshot>) => Promise<void>;
}) {
  const [selected, setSelected] = useState(draft.priorityTaskIds);
  const eligible = draft.eligibleTaskIds
    .map((id) => data.tasks.find((task) => task.id === id))
    .filter((task): task is TaskDto => Boolean(task));

  function toggle(id: number) {
    setSelected((current) =>
      current.includes(id)
        ? current.filter((value) => value !== id)
        : current.length < 3
          ? [...current, id]
          : current,
    );
  }

  return (
    <section className="planning-stage">
      <article className="now-card proposal-card">
        <p className="section-label">
          {draft.replanning ? "NOVO PLANO" : "PROPOSTA DO DIA"}
        </p>
        <h2>
          {draft.replanning
            ? "Como você quer continuar?"
            : "Estas são as prioridades que cabem no seu dia"}
        </h2>
        <div className="proposal-list">
          {eligible.map((task) => (
            <label className="proposal-item" key={task.id}>
              <input
                type="checkbox"
                checked={selected.includes(task.id)}
                onChange={() => toggle(task.id)}
              />
              <TaskRow task={task} data={data} />
            </label>
          ))}
        </div>
        {draft.missingDurationTaskIds.length > 0 && (
          <p className="planning-hint">
            {draft.missingDurationTaskIds.length} ação(ões) ficaram fora por não
            terem duração definida.
          </p>
        )}
        <div className="actions">
          <button
            className="primary"
            disabled={busy || selected.length === 0}
            onClick={() =>
              void run(() => workspace.approvePlan(draft, selected))
            }
          >
            {selected.length
              ? `Aprovar ${selected.length}`
              : "Selecione uma prioridade"}
          </button>
          <button
            className="quiet"
            disabled={busy || draft.priorityTaskIds.length === 0}
            onClick={() => void run(() => workspace.approvePlan(draft, null))}
          >
            Aceitar sugestão
          </button>
          <button
            className="quiet"
            disabled={busy}
            onClick={() => void run(() => workspace.rejectPlan(draft))}
          >
            {draft.replanning ? "Manter plano atual" : "Agora não"}
          </button>
        </div>
      </article>
    </section>
  );
}

function AfterPanel({
  data,
  tasks,
}: {
  data: WorkspaceSnapshot;
  tasks: TaskDto[];
}) {
  return (
    <article className="panel">
      <div className="panel-title">
        <h3>Depois</h3>
        <span>{tasks.length}</span>
      </div>
      {tasks.length ? (
        tasks.map((task) => <TaskRow key={task.id} task={task} data={data} />)
      ) : (
        <p className="muted">Nenhuma outra prioridade planejada.</p>
      )}
    </article>
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
function localDay() {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, "0");
  const day = String(now.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}
function humanDay(day: string) {
  const [year = 1970, month = 1, date = 1] = day.split("-").map(Number);
  return new Intl.DateTimeFormat("pt-BR", {
    weekday: "long",
    day: "2-digit",
    month: "long",
  }).format(new Date(year, month - 1, date));
}
function timeToMinute(value: string) {
  const [hour = 0, minute = 0] = value.split(":").map(Number);
  return hour * 60 + minute;
}
function minuteToTime(value: number) {
  return `${String(Math.floor(value / 60)).padStart(2, "0")}:${String(value % 60).padStart(2, "0")}`;
}
