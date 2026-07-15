# Repository structure â€” Foundation-01

> **Status:** Candidate foundation, technically validated but not promoted as the official production foundation. Promotion remains gated by the secondary SPK-01 reproduction and SPK-05, SPK-06, and SPK-07. No product functionality is implemented, and experimental spike code must never migrate automatically into production.

Status: normative engineering convention for the repository foundation.

## Physical tree

```text
Second Brain/
â”œâ”€ .cargo/                    Cargo workspace configuration
â”œâ”€ .github/                   ownership, review template, future CI gate
â”œâ”€ .vscode/                   optional editor recommendations
â”œâ”€ apps/
â”‚  â””â”€ desktop/
â”‚     â”œâ”€ src/                 presentation-only React process
â”‚     â””â”€ src-tauri/           native composition root and IPC adapter
â”œâ”€ config/                    versioned non-secret configuration only
â”œâ”€ crates/
â”‚  â”œâ”€ application/            use-case composition and dispatch boundary
â”‚  â”œâ”€ contracts/              canonical boundary representations
â”‚  â”œâ”€ domains/                documented home of the 17 domain modules
â”‚  â””â”€ infrastructure/         documented home of concrete adapters
â”œâ”€ docs/
â”‚  â””â”€ engineering/            executable engineering conventions
â”œâ”€ packages/                  generated/reused frontend packages when real
â”œâ”€ resources/                 versioned local runtime/installer assets
â”œâ”€ scripts/                   supported developer entry points
â”œâ”€ spikes/                    historical evidence; never production source
â”œâ”€ tests/                     cross-boundary test suites
â”œâ”€ Cargo.toml                 Rust workspace and global policy
â”œâ”€ package.json               JavaScript workspace orchestration
â””â”€ pnpm-workspace.yaml        frontend package boundary
```

## Why each directory exists

| Directory | Problem solved | Why this form | Rejected alternative | Authority |
|---|---|---|---|---|
| `apps/desktop` | Separates the shipped desktop entry point from reusable core code | Co-locates the WebView presentation and its Tauri host while retaining a hard language/process boundary | Root-level mixed React/Rust source | Technical Architecture Â§Â§2, 8, 17; ADR-002/003 |
| `crates/contracts` | Prevents IPC representation drift | Small framework-neutral Rust source of canonical boundary types | TypeScript-owned or Tauri-owned contracts | Technical Architecture Â§Â§8.7, 17 |
| `crates/application` | Gives commands/queries one composition and dispatch boundary | Framework-neutral library usable by Tauri and tests | Handlers embedded in Tauri commands | Technical Architecture Â§Â§8.2, 8.8 |
| `crates/domains` | Fixes the physical home and naming policy for all 17 domains | Modules are added incrementally without speculative crates | 17 empty crates or one crate per conceptual domain | Conceptual Architecture Â§4; Technical Architecture Â§Â§17, 24 |
| `crates/infrastructure` | Keeps mechanisms behind owned ports | Adapters can be grouped by mechanism only when implemented | Global utilities or infrastructure imported by domains | Technical Architecture Â§Â§8, 17 |
| `packages` | Provides a home for generated UI bindings without making TS canonical | Created packages require generation or multiple consumers | Manually duplicated DTO package | Technical Architecture Â§8.7 |
| `tests` | Separates cross-boundary suites from colocated unit tests | Test location communicates scope and runtime | One undifferentiated test folder | Technical Architecture Â§Â§16, 17 |
| `scripts` | Gives developers and CI identical, reviewable entry points | PowerShell matches the Windows-first MVP | Ad-hoc local commands | PRD platform decision; Technical Architecture Â§21 |
| `config` | Distinguishes non-secret runtime mechanics from product preferences | Schema/defaults can be reviewed without secrets | `.env` as product configuration | PRD Local First; Technical Architecture security sections |
| `resources` | Isolates shipped assets and license notices | Versioned runtime inputs stay distinct from source/generated output | Assets scattered through adapters | Technical Architecture packaging/licensing decisions |
| `spikes` | Preserves approved evidence without contaminating production | Read-only historical evidence | Copying spike source into official crates | Technical Architecture Â§27 |

The architecture requires an in-process event dispatcher and durable outbox when
real cross-domain events exist. Foundation-01 deliberately creates neither: there is
no event, consumer, transaction, or persistence implementation yet. Creating an
empty bus would be dead infrastructure and would violate the rule that events exist
only for real reaction, audit, or consistency needs. The same rule applies to jobs,
observability collectors, feature flags, and secret stores.

