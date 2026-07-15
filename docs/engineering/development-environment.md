# Development environment

> **Status:** Candidate foundation, technically validated but not promoted as the official production foundation. Promotion remains gated by the secondary SPK-01 reproduction and SPK-05, SPK-06, and SPK-07. No product functionality is implemented, and experimental spike code must never migrate automatically into production.

## Official root

The only local development root is `D:\Projetos\Second Brain`. All commands must be
started there. Project scripts reject another local root. Caches, temporary files,
compiled output, JavaScript stores, and TypeScript build metadata are redirected to
`.tooling` or `target` below the repository and ignored by Git.

Hosted CI is the sole path exception: its disposable checkout root is selected by the
runner, contains no personal data, and is destroyed after the job. It does not change
the official developer root.

## Required baseline

- Windows 11 24H2 or later, supported by Microsoft
- Visual Studio Build Tools with Desktop development with C++ and Windows SDK
- WebView2 Evergreen Runtime
- Rust 1.96.1 MSVC with rustfmt and Clippy
- Node 24 LTS
- pnpm 11.7.0

Versions selected by the frozen architecture are pinned in toolchain files and
manifests; resolved transitive versions are pinned in lockfiles.

TypeScript 6.0.3 is the activated, exact fallback authorized by the Technical
Architecture: 7.0.2 compiled the source but was incompatible with the stable typed
ESLint toolchain during Foundation-01 validation.

## Supported commands

| Command | Result |
|---|---|
| `scripts/bootstrap.ps1` | checks tools and installs locked dependencies into project-local stores |
| `scripts/check.ps1` | architecture, formatting, lint, tests, and builds |
| `scripts/dev.ps1` | Tauri + Vite hot-reload loop |

Do not invoke package managers from subdirectories. Do not store production secrets
in `.env`, shell history, IDE launch configuration, fixtures, or scripts.

## Debug and diagnostics

Rust debugging starts at `apps/desktop/src-tauri`; WebView debugging is development
only. Production diagnostics will follow the frozen structured logging and redaction
contract when implemented. Foundation-01 intentionally contains no collector.

