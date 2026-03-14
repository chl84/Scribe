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

- Insert and delete are acceptable for the current milestone but still depend on a linear piece list.
- Line lookup is already materially slower than insert in the current baseline and should be watched closely.
- Snapshot materialization is the clearest structural hotspot in the current suite and is already in the millisecond range per call on the local machine.

## Why This Matters

The current model is a reasonable correctness-first foundation, but it is not likely to remain competitive once the editor moves toward:

- large-file mode
- viewport-based rendering
- frequent incremental refresh
- richer movement and search workloads

## Next Step

The next storage design should eliminate linear hot paths where possible and reduce the need for full-document materialization during normal editing and rendering.
