# Phase 1: Performance Contract

## Goal

Define what "fastest" means for Scribe and make performance measurable before major rewrites begin.

## Todo

- [x] Define the primary benchmark suite for:
  - cold start
  - open file
  - typing latency
  - scrolling
  - undo/redo
  - search
  - save
- [x] Define one reference hardware profile and one minimum hardware profile.
- [x] Record target budgets for small, medium, large, and huge files.
- [x] Add machine-readable benchmark output so results can be compared over time.
- [ ] Add timing instrumentation for:
  - input received
  - edit applied
  - viewport built
  - IPC response returned
  - frame painted
- [x] Capture the current baseline for the existing implementation.
- [x] Document the first performance dashboard or report format in `docs/domain/` or `docs/architecture/`.

## Exit Criteria

- The project has an explicit performance contract.
- The team can compare future work against a stable baseline.
- "Fast" is defined with numbers, not descriptions.

## Notes

- The performance contract is documented in `docs/architecture/performance-contract.md`.
- The first report format is documented in `docs/domain/performance-report-format.md`.
- The existing text-engine baseline remains the first captured measurement entry point.
