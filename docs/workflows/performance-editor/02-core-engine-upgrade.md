# Phase 2: Core Engine Upgrade

## Goal

Replace the current text-buffer hot path with a structure that can scale to very large documents and high edit throughput.

## Todo

- [ ] Evaluate the current piece-table hot paths with actual measurements.
- [ ] Design the replacement structure as a balanced piece tree or equivalent editor-oriented tree.
- [ ] Keep byte offsets as the internal source of truth.
- [ ] Store subtree metadata needed for fast operations:
  - byte length
  - line count
  - newline boundaries
  - optional lightweight hashes for change detection
- [ ] Preserve the current `Document` abstraction while swapping internal storage.
- [ ] Remove full-document snapshot materialization from common edit paths where practical.
- [ ] Keep undo/redo transaction-based and backend-owned.
- [ ] Add stress tests comparing the new engine to a simple string model.
- [ ] Benchmark insert, delete, replace, navigation, and line mapping after the engine swap.

## Exit Criteria

- Core editing operations no longer depend on a linear piece list in hot paths.
- Line and offset operations scale predictably on large documents.
- Correctness and undo/redo parity are preserved through the engine change.
