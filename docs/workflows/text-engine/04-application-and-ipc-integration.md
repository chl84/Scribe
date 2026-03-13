# Phase 4: Application and IPC Integration

## Goal

Expose the text engine through backend application services and stable IPC contracts.

## Todo

- [ ] Add application services for document open, close, read, edit, save, undo, and redo.
- [ ] Define document lifecycle state in `backend/src/application/state/`.
- [ ] Add DTOs for editor commands and document snapshots.
- [ ] Add IPC handlers for edit commands.
- [ ] Return `ChangeSet`-style responses instead of full document payloads where possible.
- [ ] Define how the frontend requests initial content and incremental updates.
- [ ] Keep file I/O in `infrastructure/` and out of the domain layer.
- [ ] Add integration tests for opening a document, editing it, and saving it.
- [ ] Add integration tests for undo and redo through the application layer.
- [ ] Document the first stable IPC contract for the editor feature.

## Exit Criteria

- The frontend can open a document and perform basic edits through IPC.
- The backend owns the text engine and editor state.
- Document updates cross the boundary through explicit contracts.
