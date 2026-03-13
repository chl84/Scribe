# Phase 2: Buffer and Line Index

## Goal

Implement the internal text storage and the line index needed for fast editor operations.

## Todo

- [x] Define a `TextBuffer` interface for reading slices and applying edits.
- [x] Implement a `PieceTable` with original and add buffers.
- [x] Support `insert`, `delete`, and `replace` on top of the piece table.
- [x] Implement piece splitting and piece merging rules.
- [x] Define how snapshots or borrowed reads are exposed safely.
- [x] Add a `LineIndex` that maps `offset -> line/column`.
- [x] Add reverse mapping from `line/column -> offset`.
- [x] Update the line index incrementally after edits.
- [x] Add tests for large inserts, deletes across line boundaries, and empty-line cases.
- [x] Add tests for UTF-8 edge cases so offset math does not corrupt text.

## Exit Criteria

- The document can apply edits without using a plain `String` as the main storage model.
- Offset and line mapping work for realistic multi-line documents.
- Buffer and line-index tests cover both normal and edge-case edits.
