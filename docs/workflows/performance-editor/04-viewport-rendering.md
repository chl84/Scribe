# Phase 4: Viewport Rendering

## Goal

Replace the current full-text surface with a rendering model that only paints what the user can see.

## Todo

- [ ] Remove the full-document editor `textarea` as the primary editing surface.
- [ ] Keep a hidden input bridge only for text input, shortcuts, clipboard, and IME support.
- [ ] Add a viewport state model in the frontend:
  - visible rows
  - overscan rows
  - scroll position
  - caret and selection ranges
  - current revision
- [ ] Render only visible lines plus overscan.
- [ ] Choose one rendering strategy and commit to it:
  - canvas-backed text rendering
  - or a tightly controlled minimal DOM renderer
- [ ] Add viewport request and refresh flows over IPC.
- [ ] Add frontend instrumentation for input-to-paint latency.
- [ ] Add stress tests for scrolling, rapid cursor movement, and repeated viewport invalidation.
- [ ] Ensure Linux and Windows keyboard, clipboard, and IME paths continue to work.

## Exit Criteria

- The editor no longer depends on full-document DOM state.
- Frontend rendering cost scales with viewport size, not document size.
- Input-to-paint latency is measurable and stable under normal workloads.
