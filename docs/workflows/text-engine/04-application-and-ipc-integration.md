# Phase 4: Application and IPC Integration

## Goal

Expose the text engine through backend application services and stable IPC contracts.

## Todo

- [x] Add application services for document open, close, read, edit, save, undo, and redo.
- [x] Define document lifecycle state in `backend/src/application/state/`.
- [x] Add DTOs for editor commands and document snapshots.
- [x] Add IPC handlers for edit commands.
- [x] Return `ChangeSet`-style responses instead of full document payloads where possible.
- [x] Define how the frontend requests initial content and incremental updates.
- [x] Keep file I/O in `infrastructure/` and out of the domain layer.
- [x] Add integration tests for opening a document, editing it, and saving it.
- [x] Add integration tests for undo and redo through the application layer.
- [x] Document the first stable IPC contract for the editor feature.

## Exit Criteria

- The frontend can open a document and perform basic edits through IPC.
- The backend owns the text engine and editor state.
- Document updates cross the boundary through explicit contracts.
