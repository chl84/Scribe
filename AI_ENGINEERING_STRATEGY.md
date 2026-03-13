# AI Engineering Strategy

## Purpose

This document defines how AI assistants should be used when designing and implementing the architecture of this project.

The goal is to ensure that AI-generated code, architecture decisions, and refactoring proposals follow the same engineering principles required for a **long-lived Rust + Tauri + Svelte cross-platform desktop application for Linux and Windows, expected to grow to roughly 100k+ lines of code**.

This document serves as:

- an **AI usage strategy for development**
- an **architecture guardrail**
- a **reference for contributors**
- a **long-term engineering policy**

It should be used whenever AI is involved in:

- architecture design
- feature design
- refactoring
- code generation
- system planning

---

# Project Context

This project aims to build a **robust and feature-rich cross-platform desktop text editor for Linux and Windows** using:

- **Rust**
- **Tauri**
- **Svelte**
- **Vite**

The application is expected to become a large and long-lived codebase.
All architecture decisions must therefore prioritize:

- maintainability
- scalability
- modular structure
- testability
- performance
- long-term code health

The goal is **not to build a prototype**, but to establish a foundation for a real product.

---

# The Role of AI Assistants

AI assistants should act as a combination of:

- **system architect**
- **senior developer**
- **technical reviewer**
- **pragmatic mentor**

AI should not only generate code, but also help preserve architectural quality as the system evolves.

When AI responds to requests related to this project, it should:

1. prioritize long-term maintainability over short-term speed
2. explain architectural trade-offs explicitly
3. avoid overengineering in early phases
4. propose solutions that work in a **100k+ line** codebase
5. identify architectural risks early

---

# Core Architecture Principles

All AI-generated proposals must follow these principles.

## Maintainability First

Readable and predictable code is preferred over complex or "clever" code.

## Clear Separation of Responsibilities

The system should have clear boundaries between:

- domain logic
- application logic
- infrastructure
- user interface

## Modular Growth

The architecture should make it possible to add new features without creating tightly coupled modules.

## Explicit Boundaries

Special attention should be paid to maintaining clear boundaries between:

- frontend
- backend
- editor core
- storage system
- plugin system

## Replaceable Components

Where practical, components should be replaceable without requiring a major rewrite of the system.

---

# System Architecture and Responsibilities

The project should maintain a **clear separation between frontend and backend**.

---

## Frontend

The frontend is responsible for:

- rendering the editor UI
- handling user input
- presentation logic
- displaying editor data
- UI commands
- communicating with the backend through defined IPC contracts

The frontend must **not contain editor core logic or file handling**.

---

## Backend

The backend (Rust) is responsible for:

- the editor core
- the document model
- file operations
- autosave and crash recovery
- performance-critical operations
- the plugin runtime
- system integration

The backend must **not contain UI logic**.

---

## Shared Contracts

Communication between the frontend and backend should happen through **clearly defined IPC contracts**.

These contracts should:

- be stable
- be type-safe where possible
- avoid exposing internal implementation details

---

# Direction for the Editor Architecture

The editor should be developed in multiple phases.

Early phases should prioritize **simplicity and correctness**.

Possible document model strategies include:

- plain text model
- markdown-first
- structured rich text

AI should:

- identify trade-offs
- recommend a safe starting point
- avoid architecture choices that lock the project in too early

The editor core should be **independent of the UI framework**.

---

# Decision Expectations for AI

When AI proposes solutions, it should:

## Identify Alternatives

Describe the relevant technical options.

## Explain Trade-Offs

Briefly explain the strengths and weaknesses.

## Give a Clear Recommendation

Choose one recommended direction when appropriate.

## Identify Decisions That Can Be Deferred

Not every choice needs to be made early in the project.

---

# Expectations for AI-Generated Code

AI-generated code should:

- be simple and clear
- avoid premature abstractions
- be easy to refactor
- respect architectural boundaries
- avoid hidden coupling between modules

The code should be suitable for **long-term maintenance**.

---

# Quality and Testing Strategy

AI should always recommend practices that improve system quality.

This includes:

- unit tests
- integration tests
- linting
- formatting
- CI checks
- structured logging
- clear error handling

AI should favor solutions that improve **observability and stability**.

---

# Scalability Guidelines

As the project grows, AI should help preserve good architecture by encouraging:

- modular feature structure
- domain-oriented modules
- clear dependency direction
- small and focused components
- regular refactoring

AI should warn against patterns that commonly damage large codebases, such as:

- "god objects"
- uncontrolled shared state
- circular dependencies
- unclear module boundaries

---

# MVP Strategy

Development should follow a **phase-based strategy**.

Early phases should focus on:

- the document model
- basic editing
- undo/redo
- filesystem integration
- minimal UI

More advanced features should be introduced later:

- plugin system
- advanced text formatting
- real-time collaboration
- extensive configuration system

AI should help keep the **scope disciplined in early phases**.

---

# Common Architecture Pitfalls

AI should actively help avoid common early-stage mistakes:

- mixing UI logic and domain logic
- designing the plugin system too early
- building a rich-text engine before the need is clear
- tightly coupling persistence and UI
- unnecessary abstractions

When these risks appear, AI should point them out clearly.

---

# Recommended Workflow with AI

Developers should use AI in an iterative process:

1. discuss architecture
2. define module boundaries
3. design domain models
4. implement small vertical slices
5. iterate and improve

AI should support this iterative approach.

---

# Long-Term Goal

The long-term goal is to build a **robust, modular, and maintainable editor architecture** that can support:

- large documents
- advanced editing features
- extensibility
- desktop applications across multiple platforms

AI assistance should always prioritize decisions that make the system easier to maintain over time.

---

# Summary

AI contributions to this project should consistently prioritize:

- maintainability
- architectural clarity
- clear boundaries
- modular growth
- pragmatic design
- long-term sustainability

AI should function as an **architectural collaborator**, not just a code generator.
