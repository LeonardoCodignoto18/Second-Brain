# Testing strategy

> **Status:** Candidate foundation, technically validated but not promoted as the official production foundation. Promotion remains gated by the secondary SPK-01 reproduction and SPK-05, SPK-06, and SPK-07. No product functionality is implemented, and experimental spike code must never migrate automatically into production.

| Layer | Location | Purpose | Tooling baseline |
|---|---|---|---|
| Unit | beside Rust/React source | deterministic policy and component behavior | Rust test harness; Vitest/Testing Library |
| Integration | `tests/integration` | application plus concrete controlled adapters | Rust integration harness |
| Contract | `tests/contracts` | IPC schemas, canonical AI/provider contracts, compatibility | golden canonical payloads and generated binding drift checks |
| Architecture | `scripts/check-architecture.ps1`, later `tests/architecture` | forbidden dependencies, source ownership, boundary shape | Cargo metadata plus focused compiled checks |
| Golden | within the owning contract suite | stable serialized representations and migrations | reviewed fixtures, explicit update command |
| Snapshot | React/component or diagnostic owner | high-signal presentation/diagnostic shape only | Vitest snapshots; never domain truth |
| Windows E2E | `tests/e2e-windows` | packaged lifecycle, IPC, installer, tray and OS adapters | supported Windows/WebView2 matrix |

No empty test executable is created. Each first use case adds its unit test; each new
boundary adds its contract/architecture test in the same commit. Snapshots and
goldens must be deterministic, synthetic, reviewable, and never blindly regenerated.

The full local gate is `scripts/check.ps1`. Every commit must pass it. CI uses the
same script rather than reimplementing checks in workflow YAML.

