# Infrastructure adapters

This directory will contain concrete adapters only when an approved use case needs
one. The frozen technical architecture reserves these adapter groups: persistence,
AI transport/providers, Windows integration, security, backup, and observability.

Infrastructure may depend on application/domain ports and third-party mechanisms.
Application and domains must never depend on infrastructure. No placeholder adapter,
empty service, or speculative trait belongs here.

