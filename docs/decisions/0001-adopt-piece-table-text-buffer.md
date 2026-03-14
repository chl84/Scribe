# ADR 0001: Adopt a Piece Table for Plain-Text Editing

## Status

Accepted

## Context

The first document implementation stored the full document as a plain `String`.

That model was simple, but it was not a good long-term fit for editor workloads:

- inserts and deletes would increasingly copy large sections of text
- undo and redo would stay coupled to full-string mutation
- the backend would not have a clean foundation for larger documents

The project strategy explicitly prefers a maintainable editor core over a short-lived prototype.

## Decision

The backend document engine will use a piece table as its primary text buffer strategy for the plain-text phase.

The current shape is:

- an immutable original buffer
- an append-only add buffer
- a piece list describing visible text
- a separate `LineIndex` for offset and position mapping

The engine continues to use byte offsets as the internal source of truth. Line and column remain derived data.

## Alternatives Considered

### Plain `String`

Pros:

- minimal implementation cost
- straightforward for very small documents

Cons:

- poor fit for repeated insert and delete workloads
- weak foundation for undo and redo history
- encourages application code to lean on whole-document mutation

### Gap Buffer

Pros:

- strong locality for a single active cursor
- simpler than some tree-based structures

Cons:

- less flexible for future multi-selection and non-local editing
- more tightly shaped around one dominant cursor region

### Rope or Tree-Based Buffer

Pros:

- scalable for very large documents
- can support efficient segmented reads

Cons:

- significantly more implementation complexity for the current phase
- premature for the first plain-text engine milestone

## Consequences

Positive:

- editing operations now have a more editor-oriented storage model
- undo and redo integrate naturally with structural edits
- the engine has a clearer path toward larger documents

Negative:

- snapshots still materialize a full `String`
- the current implementation still needs performance measurement and future profiling
- the piece list may need a different internal representation if workloads become much larger

## Follow-Up

- Keep partial-read and profiling work outside the current milestone unless a concrete bottleneck appears.
- Revisit richer buffer structures only if the piece table becomes a measured bottleneck.
