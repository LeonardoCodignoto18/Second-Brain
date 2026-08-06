# Engineering conventions

> **Status:** Official foundation, promoted on 2026-08-06 after ARR-02 closure. SPK-01 through SPK-07 and the clean Windows CI gate are approved. Functional implementation is authorized; experimental spike code remains excluded from production.

## Language and names

Code identifiers and filesystem names use English `snake_case` in Rust and
`camelCase`/`PascalCase` in TypeScript. User-facing language is localized and never
embedded in domain identifiers. Frozen Portuguese domain names remain authoritative;
English filesystem names are stable mappings, not redefinitions.

| Concept | Convention | Example |
|---|---|---|
| Rust crate | `second-brain-<layer-or-mechanism>` | `second-brain-contracts` |
| Rust module/file | singular `snake_case` by responsibility | `plan_version.rs` |
| Trait | capability or role, no `I` prefix | `PlanRepository` |
| Adapter | mechanism + role | `SqlitePlanRepository` |
| Application handler | verb + noun + `Handler` | `ApprovePlanHandler` |
| Command/query | imperative command; observational query | `ApprovePlan`, `GetActivePlan` |
| Domain event | completed fact in past tense | `PlanApproved` |
| DTO | semantic name plus boundary suffix only when useful | `ApprovePlanRequest` |
| TypeScript component | one `PascalCase.tsx` component per file | `NowPanel.tsx` |
| Test | behavior and outcome, not implementation | `rejects_expired_proposal` |

## Commands, queries, events, and proposals

- Commands request a state change and return a typed result or stable safe error.
- Queries observe an owner and never mutate state.
- Events are immutable completed facts, emitted post-commit only when another
  consumer, audit, or consistency rule requires them.
- Proposals are inert, versioned data. AI has no mutation path; accepted proposals
  become explicit user-authorized commands after deterministic validation.
- Every cross-boundary contract carries its schema/contract version when evolution
  requires compatibility. Breaking semantic changes create a new major version.

## Errors and logging

Errors crossing IPC use stable codes and safe details; internal causes remain local.
Logs are structured and allowlisted. They never contain user text, prompts, model
responses, API keys, encryption keys, full chosen paths, or database payloads.

## Dependency injection

Constructor injection is the default. A trait exists only at a real substitution or
mechanism boundary and is owned by the consumer. Global service locators, mutable
singletons, and framework containers are prohibited. The Tauri host is the process
composition root.

## Versioning

The application uses SemVer. `0.y.z` denotes the private implementation phase; `1.0`
requires the MVP product contract. Cargo and npm packages share the application
version unless a generated contract explicitly needs independent schema versioning.
Lockfiles are committed. Dependency/toolchain changes use dedicated reviewable PRs.

