# Scribe

Scribe is structured to keep a clear separation between frontend, backend, and documentation.

The project uses:

- `frontend/` for the Svelte/Vite user interface
- `backend/` for the Rust application layer, domain logic, and system integration
- `docs/` for architecture notes, ADRs, domain documentation, and workflows
- `tests/` for end-to-end and integration tests

The architecture and AI principles are defined in [AI_ENGINEERING_STRATEGY.md](./AI_ENGINEERING_STRATEGY.md).

See [docs/architecture/project-structure.md](./docs/architecture/project-structure.md) for folder responsibilities and dependency rules.

Note: if the project is wired directly into the Tauri toolchain, that setup must be configured to use `backend/` instead of the default `src-tauri/` directory.
