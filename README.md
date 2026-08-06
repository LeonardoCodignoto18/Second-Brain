# Second Brain OS

> **Status:** Official foundation, promoted on 2026-08-06 after ARR-02 closure. SPK-01 through SPK-07 and the clean Windows CI gate are approved. Functional implementation is authorized; experimental spike code remains excluded from production.

Second Brain OS is a local-first Windows desktop product whose purpose is to reduce
mental load and help the user act with clarity. Product behavior is defined by the
frozen PRD and architecture documents in this repository.

This repository currently contains only the engineering foundation. It intentionally
contains no product domain, AI, persistence, planning, task, notification, scheduler,
backup, or security implementation.

## Start here

1. Use Windows 11 24H2 or later with the MSVC build tools and WebView2.
2. Run `scripts/bootstrap.ps1` from the repository root.
3. Run `scripts/check.ps1` to validate the complete foundation.
4. Run `scripts/dev.ps1` for the desktop development loop.

Every generated cache and artifact is redirected below this repository. See
`docs/engineering/development-environment.md` for the exact contract.

## Normative sources

- `PRD-Second-Brain-OS-v1.2.md`
- `Arquitetura-Conceitual-Second-Brain-OS-v1.0.md`
- `Arquitetura-Logica-Second-Brain-OS-v1.0.md`
- `Arquitetura-Tecnica-Second-Brain-OS-v1.1.md`
- `AI-Integration-Contract-v1.0.md`

## Repository map

- `apps/desktop`: React/Vite presentation and its Tauri host.
- `crates/contracts`: provider-neutral and transport-neutral boundary types.
- `crates/application`: application composition and dispatch boundary.
- `crates/domains`: reserved home of the 17 domain modules; no empty modules exist.
- `crates/infrastructure`: reserved adapter home; no empty adapters exist.
- `packages`: generated or shared frontend packages only.
- `tests`: cross-boundary test suites and their placement rules.
- `scripts`: deterministic developer entry points.
- `docs/engineering`: executable engineering contract for the foundation.

