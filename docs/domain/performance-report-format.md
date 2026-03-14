# Performance Report Format

This note defines the first report format for Scribe performance measurements.

## Purpose

The report format should be simple enough to produce from local tools today and structured enough to feed CI comparisons later.

## Required Fields

Each report should include:

- suite name
- hardware profile label
- workload name
- iteration count
- total duration
- average duration

Future versions should add:

- fixture name
- file size class
- platform and OS version
- build profile
- p50, p95, and p99 values for interactive workloads

## Current JSON Shape

The current `text_engine_metrics` example emits JSON in this shape:

```json
{
  "suite": "text_engine_baseline",
  "hardware_profile": "local-dev-machine",
  "workloads": [
    {
      "name": "insert",
      "iterations": 1000,
      "total_nanos": 0,
      "average_nanos": 0
    }
  ]
}
```

## Commands

Human-readable output:

```bash
cargo run --example text_engine_metrics
```

Machine-readable output:

```bash
cargo run --example text_engine_metrics -- --json
```

## Interpretation Rules

- The current report is a baseline tool, not a CI gate.
- The JSON format should remain backward-compatible once CI starts consuming it.
- If the format changes materially, record that change in an ADR or architecture note.
