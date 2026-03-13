# Phase 3: Editing History and Selection

## Goal

Add undo/redo and a minimal selection model without expanding into advanced editor behavior too early.

## Todo

- [ ] Define a `Selection` model for a single caret or single range.
- [ ] Define cursor movement rules separately from storage logic.
- [ ] Implement transaction boundaries for grouped edits.
- [ ] Implement an `UndoManager`.
- [ ] Implement `undo` and `redo` using stored edit transactions.
- [ ] Ensure each edit produces a stable `ChangeSet`.
- [ ] Add tests for insert-undo-redo cycles.
- [ ] Add tests for delete-undo-redo cycles.
- [ ] Add tests for replace operations and empty selections.
- [ ] Document deferred features: multi-cursor, grapheme-aware movement, soft wrap, and rectangular selection.

## Exit Criteria

- The engine supports single-selection editing with undo and redo.
- Undo history is domain-owned, not frontend-owned.
- Deferred features are explicitly marked out of scope.
