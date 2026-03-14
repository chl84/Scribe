# Current Text Engine Hotspots

This note records the current measured hotspots in the plain-text engine after the initial piece-tree upgrade.

## Current Storage Shape

The current implementation uses:

- a piece tree with subtree metadata
- direct range extraction for delete and replace paths
- full-string snapshot materialization for explicit full-text reads
- a line index that is updated incrementally

## Measured Hot Paths

The current baseline suite measures:

- repeated insert near the middle of the document
- repeated delete near the middle of the document
- repeated replace near the middle of the document
- repeated line lookup
- repeated reverse position lookup
- repeated snapshot materialization through `Document::text()`

## Current Interpretation

- Insert and replace are currently acceptable in the local baseline.
- Delete improved after removing full snapshot materialization from common edit paths, but it is still the clearest storage-level hotspot.
- Snapshot materialization remains a structural hotspot and is still in the millisecond range per call on the local machine.
- Line lookup and reverse position lookup remain fast enough to continue with the current tree shape, but they still need to be watched as viewport work begins.

## Why This Matters

The current model is a reasonable correctness-first foundation, but it is not likely to remain competitive once the editor moves toward:

- large-file mode
- viewport-based rendering
- frequent incremental refresh
- richer movement and search workloads

## Next Step

The next storage work should focus on delete-path optimization and on keeping viewport work off the full snapshot path.
