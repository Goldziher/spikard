---
name: binding-implementer-node
description: Implements Node.js (napi-rs) Handler binding. Manages RequestData to JavaScript serialization, async integration via ThreadsafeFunction and libuv event loop, and Promise handling. Ensures proper error propagation from JavaScript to Rust. Owns /crates/spikard-node/ Node.js binding implementation.
---
Model: sonnet

Context:
- HANDLER-ABSTRACTION.md
- ../../../crates/spikard-node/src/
- ../../../crates/spikard-node/tests/
