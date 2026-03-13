# Scribe

Scribe is a cross-platform desktop text editor for Linux and Windows, designed for long-term maintainability, strong core editing behavior, and room to grow into a full product.

The project is being built with:

- `Rust` for the backend and editor core
- `Tauri` for desktop application packaging and system integration
- `Svelte` and `Vite` for the frontend

## Development

Install frontend dependencies from the repository root:

```bash
npm run setup
```

Start the desktop app from the repository root:

```bash
npm run dev
```

The root scripts are the primary entrypoint for local development so the same commands work on Linux and Windows.

## Product Direction

Scribe is intended to become a robust, extensible editor rather than a throwaway prototype.
The architecture is meant to support:

- a reliable document model
- core editing workflows
- undo and redo
- filesystem integration
- a consistent experience across Linux and Windows
- long-term feature growth without collapsing the codebase structure

Early development is intentionally focused on the essentials.
More advanced capabilities such as plugins, richer formatting, and collaboration are expected to come later.

## Engineering Approach

The project is organized to keep a clear separation between frontend, backend, and supporting documentation:

- `frontend/` contains the Svelte/Vite user interface
- `backend/` contains the Rust application layer, domain logic, and system integration
- `docs/` contains architecture notes, ADRs, domain documentation, and workflows
- `tests/` contains end-to-end and integration tests

The architecture and AI engineering principles are defined in [AI_ENGINEERING_STRATEGY.md](./AI_ENGINEERING_STRATEGY.md).
See [docs/architecture/project-structure.md](./docs/architecture/project-structure.md) for folder responsibilities and dependency rules.
