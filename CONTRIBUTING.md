# Contributing

## Ground rules

- Frozen product and architecture documents are normative.
- A change must compile and pass `scripts/check.ps1` before commit.
- Never commit secrets, personal data, generated build output, or local databases.
- A domain owns its state. Cross-domain collaboration uses approved application
  commands, queries, or relevant post-commit events.
- UI imports only generated contracts and frontend presentation code.
- Infrastructure implements ports owned by application or domain code; it does not
  define product policy.
- Do not add an abstraction, crate, service, event, or feature flag without a current
  consumer and an architectural requirement.

## Commits

Use Conventional Commits: `type(scope): imperative summary`. Allowed foundation
types are `build`, `ci`, `docs`, `chore`, `refactor`, `test`, and `fix`. Product work
may use `feat` only after the relevant implementation phase begins.

Keep each commit independently buildable and reviewable. Breaking contract changes
require an explicit version change and migration plan.

## Pull requests

Describe the normative source, affected boundary, verification performed, and any
security or privacy impact. Do not combine formatting-only and behavioral changes.

