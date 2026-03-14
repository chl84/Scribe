# Current Text Engine Hotspots

This note records the current measured hotspots in the plain-text engine before the core storage upgrade begins.

## Current Storage Shape

The current implementation uses:

- a piece table with a linear piece list
- full-string snapshot materialization for some read paths
- a line index that is updated incrementally

## Measured Hot Paths

The current baseline suite measures:

- repeated insert near the middle of the document
- repeated delete near the middle of the document
- repeated line lookup
- repeated snapshot materialization through `Document::text()`

## Current Interpretation

- Insert and replace are currently acceptable in the local baseline.
- Line lookup and reverse position lookup improved significantly with the piece tree.
- Delete is now the clearest hot path regression and should be treated as the first storage-level optimization target.
- Snapshot materialization remains a structural hotspot and is still in the millisecond range per call on the local machine.

## Why This Matters

The current model is a reasonable correctness-first foundation, but it is not likely to remain competitive once the editor moves toward:

- large-file mode
- viewport-based rendering
- frequent incremental refresh
- richer movement and search workloads

## Next Step

The next storage work should improve delete performance and reduce the need for full-document materialization during normal editing and rendering.
