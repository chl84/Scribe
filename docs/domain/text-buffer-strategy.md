# Text Buffer Strategy

The current text storage model uses a piece table backed by:

- an immutable original buffer
- an append-only add buffer
- a piece list describing the visible document

## Current Behavior

- The `Document` aggregate no longer stores its content as a plain `String`.
- Edits are applied through the piece table.
- Read access is exposed through snapshots.
- Offset validation remains UTF-8 aware.
- A separate `LineIndex` tracks line starts and supports offset and position mapping.

## Current Trade-Off

- This is a better editor-oriented foundation than a plain mutable string.
- Snapshot reads still materialize a full string when needed.
- That is acceptable at this phase because correctness and boundaries matter more than premature optimization.

## Deferred Work

- More efficient partial reads
- Undo transaction integration
- Richer cursor movement rules
- Performance profiling under larger edit workloads
