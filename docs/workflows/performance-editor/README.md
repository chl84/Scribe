# Performance Editor Plan

This workflow breaks the long-term editor performance work into concrete phases.

Recommended order:

1. [01-performance-contract.md](./01-performance-contract.md)
2. [02-core-engine-upgrade.md](./02-core-engine-upgrade.md)
3. [03-runtime-and-ipc.md](./03-runtime-and-ipc.md)
4. [04-viewport-rendering.md](./04-viewport-rendering.md)
5. [05-large-files-search-and-save.md](./05-large-files-search-and-save.md)
6. [06-regression-gates.md](./06-regression-gates.md)

Scope rules for this plan:

- Optimize for a balanced, general-purpose text editor.
- Keep the current Rust + Tauri + Svelte stack.
- Treat backend performance and frontend rendering as one end-to-end system.
- Use measured budgets and repeatable benchmarks, not anecdotal speed claims.
- Keep the editor core in Rust and keep the frontend as a thin UI/input layer.
- Defer plugins, collaboration, rich text, and syntax-heavy features until the baseline is stable.
