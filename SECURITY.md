# Security policy

Report security issues privately to the repository owner. Do not open a public issue
containing credentials, personal data, database material, prompts, or exploit detail.

Secrets must be supplied through an approved secret store once that subsystem is
implemented. Environment files are not an approved production secret store. Logs,
tests, fixtures, snapshots, and crash reports must use synthetic data and the
allowlist/redaction rules defined by the frozen architecture.

