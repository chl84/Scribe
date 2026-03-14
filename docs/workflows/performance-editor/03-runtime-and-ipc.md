# Phase 3: Runtime and IPC

## Goal

Remove backend coordination bottlenecks and redesign editor IPC for high-frequency operations.

## Todo

- [x] Replace the single shared mutex around editor state with a dedicated editor runtime.
- [x] Move document operations onto a dedicated backend thread or task runtime.
- [ ] Introduce explicit document sessions and viewport sessions.
- [x] Make document commands revision-aware.
- [ ] Redesign IPC around high-frequency commands:
  - `apply_edit`
  - `move_cursor`
  - `set_selection`
  - `get_viewport`
  - `scroll_viewport`
  - `undo`
  - `redo`
  - `search`
- [ ] Stop using full-document refreshes for routine editing.
- [ ] Return incremental payloads and stale-revision errors where appropriate.
- [x] Cache immutable snapshots per revision when repeated reads would otherwise rebuild identical data.
- [ ] Add integration tests for revision mismatches, concurrent command ordering, and repeated viewport reads.

## Exit Criteria

- The backend no longer serializes all editing through one coarse lock.
- IPC is shaped for high-frequency editor traffic.
- The frontend can request small, revisioned updates instead of full state refreshes.

## Notes

- The backend now routes Tauri commands through `application/runtime/editor_runtime.rs` instead of holding `EditorService` behind a shared `Mutex`.
- The runtime currently keeps the existing request-response IPC surface while moving execution to a dedicated editor thread.
- Repeated `get_document` reads now reuse cached snapshots until a document mutation invalidates the revision.
- `edit_document`, `undo_document`, `redo_document`, and `save_document` now accept optional expected revisions and reject stale callers explicitly.
- Runtime tests now cover concurrent command ordering, repeated snapshot reads, and stale-revision propagation.
