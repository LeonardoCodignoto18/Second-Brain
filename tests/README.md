# Cross-boundary tests

Keep unit tests next to their code. Use this directory only for tests that cross a
package, crate, process, or platform boundary:

- `architecture`: forbidden dependency and repository-shape checks;
- `contracts`: Rust/TypeScript schema compatibility and golden payloads;
- `integration`: application plus real or controlled adapters;
- `e2e-windows`: packaged Windows behavior on the supported matrix;
- `fixtures-ai`: synthetic, redacted provider fixtures for contract tests.

Suites are created with their first real test. Empty test programs are prohibited.

