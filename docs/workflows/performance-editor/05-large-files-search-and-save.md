# Phase 5: Large Files, Search, and Save

## Goal

Make the editor perform well on large documents without sacrificing normal editing behavior.

## Todo

- [ ] Add explicit large-file scenarios to the benchmark suite.
- [ ] Stream or chunk file open where full eager loading becomes a bottleneck.
- [ ] Add lazy or background indexing for large files.
- [ ] Ensure large-file mode still supports reliable navigation and editing.
- [ ] Add a fast literal search path over structured snapshots or chunks.
- [ ] Add a path for incremental search results and paging.
- [ ] Optimize save so large writes do not require unnecessary full-string rebuilding.
- [ ] Measure memory usage and peak allocations for large files.
- [ ] Document the large-file operating model and any temporary limitations.

## Exit Criteria

- Large files are navigable quickly and remain editable.
- Search and save have explicit performance targets and measured results.
- Memory behavior is understood and tracked, not guessed.
