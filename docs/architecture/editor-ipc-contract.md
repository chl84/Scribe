# Editor IPC Contract

This document describes the first stable backend contract for editor operations.

## Commands

- `create_document`
- `open_document`
- `get_document`
- `edit_document`
- `undo_document`
- `redo_document`
- `save_document`
- `close_document`

## Contract Shape

- Full document snapshots are returned for open and read operations.
- Edit, undo, and redo operations return `ChangeSet`-style responses.
- The backend remains the owner of document state, revision history, and undo/redo history.
- File I/O is handled through backend infrastructure, not through the domain model.

## Frontend Request Flow

- The frontend opens or creates a document and receives a full snapshot.
- The frontend reads existing document state through `get_document` when it needs a full refresh.
- The frontend sends edits as command DTOs through `edit_document`.
- The backend responds with incremental `ChangeSet` payloads for edit, undo, and redo.
- The frontend should treat snapshots as initialization and recovery paths, and `ChangeSet` responses as the normal live update path.

## Notes

- This contract is intentionally plain and backend-driven.
- It is suitable for the first editor slice but will likely evolve once the frontend starts rendering incremental document updates.
