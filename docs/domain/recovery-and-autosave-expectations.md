# Recovery and Autosave Expectations

The current engine does not implement autosave or crash recovery yet, but the expected boundaries are already clear.

## Expectations

- Unsaved edits remain owned by the backend document state.
- Autosave should observe document revisions rather than frontend events.
- Recovery data should be written by infrastructure code, not by the document domain model.
- Recovery should restore document content and revision state before the frontend attempts to render editor state.

## Non-Goals For The Current Phase

- No persistence format for recovery snapshots is defined yet.
- No crash-recovery replay logic is implemented yet.
- No frontend recovery UX is defined yet.

These concerns remain deferred until the plain-text engine and application service layer are stable enough to persist safely.
