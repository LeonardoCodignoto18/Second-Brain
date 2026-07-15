# Domain modules

This directory is the physical home reserved for the 17 domains frozen by the
Conceptual and Logical Architectures:

1. Daily Planning
2. Orientation and Now
3. Intelligent Assistance
4. Experience Orchestration
5. Capture and Inbox
6. Actions and Projects
7. Calendar and Availability
8. Weekly Objective
9. Execution and Focus
10. Preferences and Configuration
11. Memory and Learning
12. Deterministic Orientation
13. Consent and Data Boundary
14. History and Audit
15. Backup, Export, and Restore
16. Notifications
17. Validation Metrics

No crate or module is created until its first approved use case exists. When created,
each domain is a module with private internals and an explicit public application
surface. Domains never depend on Tauri, React, SQLite, SQLCipher, OpenAI, provider
SDKs, Windows APIs, or another domain's repository.

The names above are English filesystem identifiers for the exact Portuguese domain
names in the frozen documents; this README does not rename or redefine them.

