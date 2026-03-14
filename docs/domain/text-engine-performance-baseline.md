# Text Engine Performance Baseline

This note records the first repeatable measurement entry point for the plain-text engine.

## Command

Run the local baseline from the backend directory:

```bash
cargo run --example text_engine_metrics
```

The example exercises three slow-path candidates:

- repeated inserts near the middle of a large document
- repeated deletes near the middle of a large document
- repeated offset-to-position lookups across many lines

## Interpretation Rules

- The numbers are local and indicative, not CI gates.
- The baseline is meant to expose regressions and obvious algorithmic problems early.
- If the workload changes significantly, update this note with a new baseline and the date it was captured.

## Current Baseline

Captured on 2026-03-14 on the local development machine.

- Insert workload: `121.21ms` total over `1000` operations, `121.21µs` average
- Delete workload: `251.73ms` total over `1000` operations, `251.73µs` average
- Line lookup workload: `1.20s` total over `10000` operations, `120.03µs` average

These values are useful as a first regression baseline, not as acceptance thresholds.
