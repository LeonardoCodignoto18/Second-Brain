# Foundation completion and first engineering sequence

> **Status:** Official foundation, promoted on 2026-08-06 after ARR-02 closure. SPK-01 through SPK-07 and the clean Windows CI gate are approved. Functional implementation is authorized; experimental spike code remains excluded from production.

This sequence is technical, not a product-scope change. Each numbered item is one or
more small commits that must compile independently.

## Recommended implementation order

1. **Repository policy:** root manifests, toolchain, ignore/line-ending policy,
   ownership, license, contribution rules.
2. **Core boundaries:** contracts and application crates with the one foundation
   status query that proves dependency direction.
3. **Desktop shell:** React/Vite process, Tauri composition root, restrictive main
   capability, no product screen.
4. **Executable governance:** architecture check, formatting, lint, test/build scripts,
   project-local cache enforcement.
5. **Locked builds:** generate and commit Cargo/pnpm lockfiles; reproduce from clean
   project-local stores.
6. **CI gate:** run the exact local check on the supported Windows runner.
7. **First vertical product slice:** only after the remaining technical gates and the corresponding promotion decision, select the
   first already-approved use case and add its domain module, application handler,
   port/adapter only where actually consumed, contract, and tests together.

## First weeks

| Window | Engineering outcome | Exit evidence |
|---|---|---|
| Week 1 | Foundation reviewed, lockfiles reproducible, desktop shell builds and starts | clean `scripts/check.ps1`; manual shell launch |
| Week 2 | First approved thin vertical slice establishes the domain-module pattern | unit + IPC contract + architecture checks; no boundary exception |
| Week 3 | First required local persistence slice establishes migration/transaction pattern using approved SQLCipher decisions | clean migration/integrity tests and spike conditions preserved |
| Week 4 | First approved daily-cycle integration establishes query invalidation and error/diagnostic pattern | deterministic fallback and consent boundaries tested where relevant |

The product backlog determines which frozen use case is selected; this roadmap does
not add or reprioritize functionality.

## Recommended first commit

`build(foundation): establish candidate modular desktop workspace`

It should contain this complete, compiling foundation because the manifests, scripts,
and shell are mutually required to prove the baseline. Subsequent work must return to
small vertical commits; spike code must never be copied into production.

## Definition of ready

- [ ] Repository is Git-initialized on `main` at the official D: root.
- [ ] Frozen documents and spike evidence are unchanged.
- [ ] Rust toolchain and JavaScript dependencies are exactly locked.
- [ ] Rust formatting, Clippy, tests, and workspace build pass.
- [ ] TypeScript typecheck, ESLint, formatting, tests, and Vite build pass.
- [ ] Tauri native host compiles and the empty presentation shell starts manually.
- [ ] Architecture check rejects every currently forbidden workspace edge.
- [ ] UI has no database, filesystem, network, OS, provider, or domain access.
- [ ] No domain logic, persistence, AI, scheduler, notification, backup, or secret
  implementation exists.
- [ ] No empty service, speculative trait, unused feature, or placeholder test exists.
- [ ] Caches and artifacts remain below the official repository root.
- [ ] README and developer commands allow a clean machine to bootstrap.
- [ ] CI runs the same full gate on the supported Windows baseline.
- [ ] Repository license and ownership are explicit.

