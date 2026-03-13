# Contributing

Contributions must follow the architecture principles in [AI_ENGINEERING_STRATEGY.md](./AI_ENGINEERING_STRATEGY.md).

Ground rules:

- Keep `frontend/` free of domain logic, filesystem operations, and backend implementation details.
- Keep `backend/src/domain/` free of UI, IPC, and framework dependencies.
- Let `backend/src/application/` orchestrate use cases and coordinate `domain/` and `infrastructure/`.
- Treat `backend/src/interface/` as a transport and integration layer, not as the home for core business logic.
- Use `docs/decisions/` to document important architecture choices as ADRs.

For new features:

1. Document module boundaries and important trade-offs.
2. Implement a small vertical slice.
3. Add appropriate tests in `tests/` or `backend/src/tests/`.
