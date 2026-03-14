# Phase 2: Core Engine Upgrade

## Goal

Replace the current text-buffer hot path with a structure that can scale to very large documents and high edit throughput.

## Todo

- [x] Evaluate the current piece-table hot paths with actual measurements.
- [x] Design the replacement structure as a balanced piece tree or equivalent editor-oriented tree.
- [x] Keep byte offsets as the internal source of truth.
- [x] Store subtree metadata needed for fast operations:
  - byte length
  - line count
  - newline boundaries
  - optional lightweight hashes for change detection
- [x] Preserve the current `Document` abstraction while swapping internal storage.
- [x] Remove full-document snapshot materialization from common edit paths where practical.
- [x] Keep undo/redo transaction-based and backend-owned.
- [x] Add stress tests comparing the new engine to a simple string model.
- [x] Benchmark insert, delete, replace, navigation, and line mapping after the engine swap.

## Exit Criteria

- Core editing operations no longer depend on a linear piece list in hot paths.
- Line and offset operations scale predictably on large documents.
- Correctness and undo/redo parity are preserved through the engine change.

## Notes

- The current hotspot summary is documented in `docs/domain/current-text-engine-hotspots.md`.
- The measurement suite now includes snapshot materialization so the current full-text read cost stays visible.
- The storage direction is recorded in `docs/decisions/0002-adopt-piece-tree-for-performance-core.md`.
- The active implementation now uses a piece tree.
- Delete and replace now extract removed text directly from the buffer instead of materializing a full document snapshot first.
- Full snapshot materialization is still intentionally measured because viewport and export work can still regress on it later.
