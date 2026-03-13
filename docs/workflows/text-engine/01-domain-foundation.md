# Phase 1: Domain Foundation

## Goal

Define the core document types and invariants before implementing storage details.

## Todo

- [x] Create `DocumentId` and `RevisionId` value types.
- [x] Create `TextOffset`, `TextRange`, and `Position` types.
- [x] Decide and document the newline policy for opened and saved files.
- [x] Decide and document that byte offsets are the internal source of truth.
- [x] Define a `Document` aggregate for text state and revision tracking.
- [x] Define `Edit` variants for insert, delete, and replace.
- [x] Define a `ChangeSet` model that describes what changed after an edit.
- [x] Add domain invariants for valid ranges, empty documents, and revision updates.
- [x] Add unit tests for ranges, offsets, and invalid input handling.

## Exit Criteria

- Core types are implemented in `backend/src/domain/document/`.
- Domain invariants are explicit and tested.
- No UI or IPC concerns leak into the document model.
