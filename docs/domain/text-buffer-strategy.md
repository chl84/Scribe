# Text Buffer Strategy

The current text storage model uses a piece tree backed by:

- an immutable original buffer
- an append-only add buffer
- tree nodes describing the visible document through pieces

## Current Behavior

- The `Document` aggregate no longer stores its content as a plain `String`.
- Edits are applied through a piece tree.
- Read access is exposed through snapshots.
- Offset validation remains UTF-8 aware.
- A separate `LineIndex` tracks line starts and supports offset and position mapping.
- Tree nodes cache subtree byte lengths and newline counts.

## Current Trade-Off

- This is a better editor-oriented foundation than a plain mutable string.
- It removes the old linear piece-list representation from the active storage layer.
- Snapshot reads still materialize a full string when needed.
- Snapshot reads still remain an explicit hotspot that must be reduced later.

## Deferred Work

- More efficient partial reads
- Undo transaction integration
- Richer cursor movement rules
- Performance profiling under larger edit workloads

## Performance Direction

The active implementation now follows the balanced piece-tree direction chosen for the performance phase.

The next follow-up is to reduce full snapshot materialization and tighten slow edit paths that still show up in the benchmark suite.
