# Phase 5: Testing and Hardening

## Goal

Harden the engine so it can safely become the base for future editor features.

## Todo

- [x] Add property-style tests for edit sequences and range validity.
- [x] Add regression tests for newline handling.
- [x] Add regression tests for UTF-8 and non-ASCII content.
- [x] Add large-document tests for startup, editing, and save behavior.
- [ ] Measure slow paths in insert, delete, and line lookup operations.
- [x] Add structured logging around document open, edit, save, undo, and redo.
- [x] Define recovery expectations for unsaved edits and autosave integration.
- [x] Write a short architecture note for the chosen text buffer strategy.
- [ ] Record any follow-up ADRs if the engine design changes.
- [ ] Revisit deferred items only after the plain-text engine is stable.

## Exit Criteria

- The engine has a reliable automated test baseline.
- Performance and correctness risks are visible early.
- The project can build editor features on top of the engine without redesigning the core immediately.
