# Text Engine Plan

This workflow breaks the text engine work into small, reviewable phases.

Recommended order:

1. [01-domain-foundation.md](./01-domain-foundation.md)
2. [02-buffer-and-index.md](./02-buffer-and-index.md)
3. [03-editing-history-and-selection.md](./03-editing-history-and-selection.md)
4. [04-application-and-ipc-integration.md](./04-application-and-ipc-integration.md)
5. [05-testing-and-hardening.md](./05-testing-and-hardening.md)

Scope rules for the first implementation:

- Build a plain-text engine, not rich text.
- Keep the engine in the Rust backend domain layer.
- Use byte offsets as the internal source of truth.
- Treat line and column as derived data.
- Defer plugins, collaboration, syntax highlighting, and multi-cursor support.
