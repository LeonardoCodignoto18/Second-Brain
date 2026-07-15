# Candidate foundation milestone

- **Date:** 2026-07-15 (America/Sao_Paulo)
- **First commit:** `3dfb7176d78a6f0eaf25eb8153769706cbedf88d`
- **Status:** candidate foundation technically validated; not promoted as the official production foundation
- **Toolchains:** Rust 1.96.1 MSVC, Cargo 1.96.1, Node 24.14.0, pnpm 11.7.0, Tauri 2.11.x, React 19.2.7, TypeScript 6.0.3, Vite 8.1.3
- **Validation:** `scripts/check.ps1`, `cargo build --workspace --locked`, and `pnpm --filter @second-brain/desktop tauri build --no-bundle` completed successfully
- **Pending gates:** secondary SPK-01 reproduction, SPK-05, SPK-06, and SPK-07
- **Restriction:** no product functionality may be implemented before its corresponding gate and decision; experimental spike code must not migrate automatically into production
