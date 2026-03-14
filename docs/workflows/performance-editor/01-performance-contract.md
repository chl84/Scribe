# Phase 1: Performance Contract

## Goal

Define what "fastest" means for Scribe and make performance measurable before major rewrites begin.

## Todo

- [ ] Define the primary benchmark suite for:
  - cold start
  - open file
  - typing latency
  - scrolling
  - undo/redo
  - search
  - save
- [ ] Define one reference hardware profile and one minimum hardware profile.
- [ ] Record target budgets for small, medium, large, and huge files.
- [ ] Add machine-readable benchmark output so results can be compared over time.
- [ ] Add timing instrumentation for:
  - input received
  - edit applied
  - viewport built
  - IPC response returned
  - frame painted
- [ ] Capture the current baseline for the existing implementation.
- [ ] Document the first performance dashboard or report format in `docs/domain/` or `docs/architecture/`.

## Exit Criteria

- The project has an explicit performance contract.
- The team can compare future work against a stable baseline.
- "Fast" is defined with numbers, not descriptions.
