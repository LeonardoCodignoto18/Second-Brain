# Crate boundaries

> **Status:** Official foundation, promoted on 2026-08-06 after ARR-02 closure. SPK-01 through SPK-07 and the clean Windows CI gate are approved. Functional implementation is authorized; experimental spike code remains excluded from production.

## Current crate matrix

| Crate | Responsibility | Public surface | Internal surface | Allowed workspace dependencies | Forbidden dependencies |
|---|---|---|---|---|---|
| `second-brain-contracts` | Stable, versioned request/response/event/proposal representations that cross boundaries | Versioned DTOs, envelopes, stable error codes, schema metadata as they become real | Serialization helpers and validation details | none | application, desktop, domains, infrastructure, Tauri, React, database, network, provider SDK, Windows API |
| `second-brain-application` | Composition, command/query dispatch, use-case transaction boundary, domain port coordination | Use-case inputs/outputs and composition entry points | handlers, policies, transaction orchestration | contracts; future domain modules owned inside the application/core boundary | desktop/Tauri, concrete infrastructure, React, provider SDKs, direct OS/database mechanisms |
| `second-brain-desktop` | Tauri lifecycle, allowlisted IPC translation, native process composition | executable only; no reusable product API | command functions and adapter wiring | application, contracts, Tauri; future concrete adapters required by composition | React source, domain repositories, provider-specific objects crossing IPC |

## Why three crates

Three is the smallest split that makes the frozen dependency direction mechanically
visible. A single crate would let Tauri and mechanisms leak into product code. A
crate per domain would add 17 manifests, public APIs, compile boundaries, and false
independence before any use case exists. Separate infrastructure crates will be
created only when a real adapter's build dependency or replacement boundary makes
the isolation valuable.

## Dependency direction

```text
desktop host â”€â”€> application â”€â”€> contracts
      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€> contracts

future infrastructure â”€â”€> ports owned by application/domain
future domain module â”€â”€â”€â”€> contracts only when data truly crosses its boundary
```

Forbidden direction is the inverse of every arrow. A domain may not import another
domain's repository. The application layer mediates approved commands and queries;
post-commit events are introduced only for named consumers or audit/consistency.

`scripts/check-architecture.ps1` enforces the current workspace graph and scans the
presentation source for direct native/infrastructure access. The allowlist must be
updated in the same commit that adds an architecturally approved crate.

## Features and visibility

- Default features remain empty until an optional mechanism has two required build
  modes. Product behavior is never selected by a Cargo feature.
- Crates expose the smallest type/function surface required by another crate.
- Modules begin private; `pub(crate)` is preferred for collaboration inside a crate.
- `pub` requires an external workspace consumer or a canonical boundary contract.
- Provider, transport, MCP, database, Windows, and encryption types never appear in
  domain or IPC public signatures.

