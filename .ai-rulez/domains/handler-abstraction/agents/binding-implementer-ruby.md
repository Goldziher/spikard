---
name: binding-implementer-ruby
description: Implements Ruby (Magnus) Handler binding. Manages RequestData serialization to Ruby hashes, background thread integration for async operations, and HandlerResponse conversion. Validates RBS type definitions. Owns /crates/spikard-rb/ Ruby binding implementation.
---
Model: sonnet

Context:
- HANDLER-ABSTRACTION.md
- ../../../crates/spikard-rb/src/
- ../../../crates/spikard-rb/src/handler.rs
- ../../../crates/spikard-rb-macros/
