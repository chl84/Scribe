# Phase 3: Runtime and IPC

## Goal

Remove backend coordination bottlenecks and redesign editor IPC for high-frequency operations.

## Todo

- [ ] Replace the single shared mutex around editor state with a dedicated editor runtime.
- [ ] Move document operations onto a dedicated backend thread or task runtime.
- [ ] Introduce explicit document sessions and viewport sessions.
- [ ] Make document commands revision-aware.
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
- [ ] Cache immutable snapshots per revision when repeated reads would otherwise rebuild identical data.
- [ ] Add integration tests for revision mismatches, concurrent command ordering, and repeated viewport reads.

## Exit Criteria

- The backend no longer serializes all editing through one coarse lock.
- IPC is shaped for high-frequency editor traffic.
- The frontend can request small, revisioned updates instead of full state refreshes.
