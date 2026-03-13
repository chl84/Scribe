# Project Structure

This structure is intended to keep presentation, application logic, domain logic, and infrastructure clearly separated.

## Top-Level Responsibilities

- `frontend/`: UI, presentation, user interaction, and the IPC client.
- `backend/`: application layer, domain logic, infrastructure, and the IPC server.
- `docs/`: architecture documentation, ADRs, domain knowledge, and workflows.
- `tests/`: end-to-end tests, integration tests, and shared fixtures.
- `scripts/`: project-specific tooling and automation.

## Frontend Rules

- `frontend/src/features/` is organized by functional area.
- `frontend/src/shared/ui/` contains reusable UI building blocks.
- `frontend/src/shared/ipc/` contains contracts, client adapters, and mappings to the backend.
- The frontend must not own the document model, filesystem logic, or any other editor core logic.

## Backend Rules

- `backend/src/domain/` contains the core concepts and rules for the editor, documents, and workspace.
- `backend/src/application/` orchestrates use cases, commands, and application state.
- `backend/src/infrastructure/` implements filesystem access, persistence, autosave, recovery, and logging.
- `backend/src/interface/` translates between external interfaces and internal use cases.

## Dependency Direction

- `frontend` may depend on IPC contracts, but not on backend internals.
- `interface` may depend on `application`.
- `application` may depend on `domain` and `infrastructure`.
- `domain` must not depend on `application`, `interface`, or the frontend.

## Tauri Note

The project intentionally uses `backend/` as the backend root to make the architectural split clearer than the default `src-tauri/` layout.
When Tauri is fully integrated, the toolchain and configuration must explicitly point to this directory.
