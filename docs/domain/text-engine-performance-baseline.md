# Text Engine Performance Baseline

This note records the first repeatable measurement entry point for the plain-text engine.

## Command

Run the local baseline from the backend directory:

```bash
cargo run --example text_engine_metrics
```

Machine-readable output:

```bash
cargo run --example text_engine_metrics -- --json
```

The example exercises three slow-path candidates:

- repeated inserts near the middle of a large document
- repeated deletes near the middle of a large document
- repeated replaces near the middle of a large document
- repeated offset-to-position lookups across many lines
- repeated position-to-offset lookups across many lines
- repeated full snapshot materialization through `Document::text()`

## Interpretation Rules

- The numbers are local and indicative, not CI gates.
- The baseline is meant to expose regressions and obvious algorithmic problems early.
- If the workload changes significantly, update this note with a new baseline and the date it was captured.

## Current Baseline

Captured on 2026-03-14 on the local development machine after switching to the piece tree and removing full snapshot materialization from common delete and replace paths.

- Insert workload: `109.39ms` total over `1000` operations, `109.39µs` average
- Delete workload: `5.75s` total over `1000` operations, `5.75ms` average
- Replace workload: `97.30ms` total over `1000` operations, `97.30µs` average
- Line lookup workload: `858.24ms` total over `10000` operations, `85.82µs` average
- Position lookup workload: `552.32ms` total over `10000` operations, `55.23µs` average
- Snapshot materialization workload: `563.80ms` total over `200` operations, `2.82ms` average

These values are useful as a first regression baseline, not as acceptance thresholds.
