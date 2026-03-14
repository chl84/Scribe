# Phase 6: Regression Gates

## Goal

Turn performance work into an enforceable engineering standard.

## Todo

- [ ] Decide which benchmarks run on every branch and which run in a slower dedicated lane.
- [ ] Add regression comparison output to CI artifacts.
- [ ] Define threshold rules for blocking regressions.
- [ ] Add a documented exception process for temporary regressions.
- [ ] Record benchmark trends over time.
- [ ] Add release-time performance validation for Linux and Windows builds.
- [ ] Review deferred performance-heavy features only after the baseline remains stable over several iterations.
- [ ] Write one ADR if the runtime, rendering model, or buffer strategy materially changes from the current plan.

## Exit Criteria

- Performance regressions are visible and actionable.
- Important slowdowns can fail CI instead of being discovered later.
- Performance remains a maintained property of the product, not a one-time sprint.
