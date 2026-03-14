# Phase 3: Editing History and Selection

## Goal

Add undo/redo and a minimal selection model without expanding into advanced editor behavior too early.

## Todo

- [x] Define a `Selection` model for a single caret or single range.
- [x] Define cursor movement rules separately from storage logic.
- [x] Implement transaction boundaries for grouped edits.
- [x] Implement an `UndoManager`.
- [x] Implement `undo` and `redo` using stored edit transactions.
- [x] Ensure each edit produces a stable `ChangeSet`.
- [x] Add tests for insert-undo-redo cycles.
- [x] Add tests for delete-undo-redo cycles.
- [x] Add tests for replace operations and empty selections.
- [x] Document deferred features: multi-cursor, grapheme-aware movement, soft wrap, and rectangular selection.

## Exit Criteria

- The engine supports single-selection editing with undo and redo.
- Undo history is domain-owned, not frontend-owned.
- Deferred features are explicitly marked out of scope.
