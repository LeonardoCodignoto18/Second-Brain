# Official foundation milestone

- **Candidate baseline:** `818a35964574cdc0848f7ce0550eaeb59aa5d174`
- **Promotion date:** 2026-08-06 (America/Sao_Paulo)
- **ARR-02 decision:** PROMOTE THE FOUNDATION
- **Closed gates:** SPK-01 through SPK-07; full foundation CI on a clean Windows runner; independent secondary SPK-01 reproduction
- **Independent evidence:** GitHub Actions run `31077973605`, jobs `92540081759` (foundation CI) and `92540081765` (SPK-01), both successful on Windows Server 2025 x64 (`windows-2025-vs2026`, image `20260803.193.1`, runner `2.336.0`)
- **Validated SPK-01 chain:** Rust/Cargo 1.96.1 MSVC; Perl 5.42.0 x64; `rusqlite` 0.40.1; `libsqlite3-sys` 0.38.1; `openssl-sys` 0.9.117; Tauri CLI 2.11.4; vendored static SQLCipher/OpenSSL
- **SPK-01 result:** 9 tests passed and 0 failed; release probe and NSIS installer built; native linkage inspected; no SQLCipher, SQLite, OpenSSL, `libcrypto`, or `libssl` DLL shipped
- **Binary SHA-256:** `06CCE244688915D6EA3659201615AC26FB18D15C09710B57D2162683A55205C8`
- **NSIS SHA-256:** `1930D1C256A0FD058DFB4505614E8580D9975878C85188A4A8C50646732AA566`
- **Short-lived evidence artifact:** `8959692555`, digest `sha256:553d7f5209a76729d3e66392a5aff5745bc6bd47ffeb7e86246e3c3437e10c57`
- **ADR-004 condition:** closed by the approved independent secondary reproduction; SQLCipher 4.14.0 remains the pilot-validated version, not a permanent pin, and `sqlcipher_export()` remains prohibited in the MVP
- **Repository state:** temporary PR closed without merge; disposable branch and harness removed; no experimental code promoted
- **Authorization:** functional implementation of the frozen MVP is authorized. No new architecture round is required without concrete technical evidence.
