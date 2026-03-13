# Phase 2: Buffer and Line Index

## Goal

Implement the internal text storage and the line index needed for fast editor operations.

## Todo

- [ ] Define a `TextBuffer` interface for reading slices and applying edits.
- [ ] Implement a `PieceTable` with original and add buffers.
- [ ] Support `insert`, `delete`, and `replace` on top of the piece table.
- [ ] Implement piece splitting and piece merging rules.
- [ ] Define how snapshots or borrowed reads are exposed safely.
- [ ] Add a `LineIndex` that maps `offset -> line/column`.
- [ ] Add reverse mapping from `line/column -> offset`.
- [ ] Update the line index incrementally after edits.
- [ ] Add tests for large inserts, deletes across line boundaries, and empty-line cases.
- [ ] Add tests for UTF-8 edge cases so offset math does not corrupt text.

## Exit Criteria

- The document can apply edits without using a plain `String` as the main storage model.
- Offset and line mapping work for realistic multi-line documents.
- Buffer and line-index tests cover both normal and edge-case edits.
