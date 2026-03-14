# Performance Contract

This note defines the first explicit performance contract for Scribe.

The contract is intentionally practical: it optimizes for a balanced, general-purpose text editor running on the fixed Rust + Tauri + Svelte stack.

## Benchmark Suite

The primary benchmark suite must cover:

- cold start to first editable frame
- open file to editable state
- typing and edit latency
- scrolling and viewport refresh
- undo and redo
- search first-result latency
- save latency

## Hardware Profiles

### Reference Profile

Use this profile for primary performance targets:

- modern desktop-class CPU
- SSD storage
- 32 GB RAM
- Linux or Windows developer workstation

### Minimum Profile

Use this profile to catch regressions that would make the editor feel slow on less capable machines:

- mid-range laptop-class CPU
- SSD storage
- 16 GB RAM
- Linux or Windows workstation

The exact machine specs should later be recorded in the benchmark harness metadata once dedicated performance runners exist.

## File Size Classes

The benchmark suite should keep these document classes stable:

- small: up to 100 KB
- medium: 1 MB
- large: 10 MB
- huge: 100 MB

## Initial Target Budgets

These budgets are the first engineering targets and can be revised only with an explicit note or ADR.

- cold start to first editable frame:
  - reference: `<= 250 ms`
  - minimum: `<= 400 ms`
- open 1 MB file to editable state:
  - reference: `<= 50 ms`
  - minimum: `<= 90 ms`
- open 10 MB file to editable state:
  - reference: `<= 200 ms`
  - minimum: `<= 350 ms`
- open 100 MB file to first navigable state:
  - reference: `<= 1200 ms`
  - minimum: `<= 2000 ms`
- typing pipeline p99:
  - small/medium: `<= 8 ms`
  - large: `<= 16 ms`
- viewport or scroll frame p99:
  - `<= 16 ms`
- undo/redo p99 on 10 MB:
  - `<= 20 ms`
- search first-result latency:
  - 10 MB: `<= 100 ms`
  - 100 MB: `<= 400 ms`

## Measurement Rules

- Prefer machine-readable benchmark output.
- Keep benchmark fixtures stable across iterations.
- Record whether a result was measured on the reference or minimum profile.
- Track p50, p95, and p99 where interactive latency matters.
- Treat regressions as product regressions, not only implementation details.
