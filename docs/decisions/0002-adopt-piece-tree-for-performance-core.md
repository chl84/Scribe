# ADR 0002: Adopt a Piece Tree for the Performance Editor Core

## Status

Accepted

## Context

ADR 0001 chose a piece table for the plain-text engine because it was the safest correctness-first starting point.

That decision was appropriate for the first engine milestone, but the current performance plan now has stricter requirements:

- lower and more stable latency on larger documents
- faster line and offset operations under sustained editing
- less dependence on full-document materialization
- a data structure that can support viewport-driven rendering later

The current measurements show that the plain piece-list model is no longer the desired long-term storage shape for the performance phase.

## Decision

The next core storage implementation will move from a linear piece list to a balanced piece tree.

The piece tree should preserve the existing editor model:

- immutable original buffer
- append-only add buffer
- byte offsets as the internal source of truth
- line and column derived from indexed metadata

The tree nodes should carry cached subtree metadata needed for fast operations:

- total byte length
- line count
- newline boundary information
- optional lightweight hashes if later change detection needs them

## Alternatives Considered

### Keep the Current Piece List

Pros:

- lowest implementation risk
- no storage migration effort

Cons:

- linear hot paths remain visible in measurements
- snapshot-heavy operations will keep scaling poorly
- weak fit for future viewport-driven rendering

### Rope

Pros:

- good asymptotic behavior for large documents
- mature general-purpose text structure

Cons:

- less directly aligned with the current piece-table semantics
- can require broader adaptation around edit history and metadata layout

### Gap Buffer

Pros:

- strong for localized single-cursor editing
- simpler than some tree-based structures

Cons:

- weaker fit for broader general-purpose workloads
- less suitable for non-local edits and future viewport/index layering

## Consequences

Positive:

- the editor core gets a structure that can scale beyond the current milestone
- subtree metadata can support faster offset, line, and rendering-related operations
- the change keeps the current Rust domain model direction intact

Negative:

- implementation complexity increases materially
- balancing logic and metadata maintenance add more invariants to test
- migration must preserve current undo/redo and offset correctness

## Follow-Up

- Keep the public `Document` abstraction stable while swapping internal storage.
- Add stress tests against the current correctness model before replacing the implementation.
- Revisit snapshot-heavy APIs after the piece tree lands.
