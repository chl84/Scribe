# Text Document Foundation

This note documents the first domain-level decisions for the text engine.

## Source of Truth

- Byte offsets are the internal source of truth.
- Line and column are derived views on top of text storage.
- UI layers must not treat line and column as canonical document coordinates.

## Newline Policy

- Opened documents detect their preferred newline mode from existing content.
- Documents preserve the detected newline style when possible.
- `LF` is the default when no `CRLF` sequences are present.
- Newline normalization is deferred until the persistence layer is designed.

## Current Scope

- The current implementation defines document identity, revisions, ranges, edits, and change sets.
- The current implementation validates ranges and UTF-8 boundaries.
- The current implementation is a domain foundation, not the final storage engine.
- Piece table storage, line indexing, undo management, and IPC integration are still separate phases.
