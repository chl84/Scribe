# Editing History and Selection

The current editing model supports:

- a single caret
- a single contiguous selection range
- grouped edit transactions
- document-owned undo and redo history

## Current Scope

- Each edit still produces a stable `ChangeSet`.
- Undo and redo are implemented inside the document domain model.
- Cursor movement rules are defined separately from the text storage implementation.
- Non-empty selections collapse before horizontal movement.

## Deferred Features

- Multi-cursor editing
- Grapheme-aware cursor movement
- Soft wrap aware navigation
- Rectangular selection
- Selection affinity and bidi text behavior

These are intentionally deferred until the plain-text engine is more stable.
