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
- repeated offset-to-position lookups across many lines
- repeated full snapshot materialization through `Document::text()`

## Interpretation Rules

- The numbers are local and indicative, not CI gates.
- The baseline is meant to expose regressions and obvious algorithmic problems early.
- If the workload changes significantly, update this note with a new baseline and the date it was captured.

## Current Baseline

Captured on 2026-03-14 on the local development machine.

- Insert workload: `111.46ms` total over `1000` operations, `111.47µs` average
- Delete workload: `265.69ms` total over `1000` operations, `265.69µs` average
- Line lookup workload: `4.13s` total over `10000` operations, `412.56µs` average
- Snapshot materialization workload: `655.36ms` total over `200` operations, `3.28ms` average

These values are useful as a first regression baseline, not as acceptance thresholds.
