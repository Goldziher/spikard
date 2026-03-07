---
name: di-handler-engineer
description: Implements DependencyInjectingHandler wrapper. Manages request-scoped dependency resolution and passes resolved dependencies to handlers via RequestData. Ensures zero-cost when DI feature disabled. Handles dependency graph compilation and cycle detection. Optional feature: di
---
Model: sonnet

Context:
- HANDLER-ABSTRACTION.md
- ../../../crates/spikard-http/src/di_handler.rs
- ../../../crates/spikard-core/src/di/
