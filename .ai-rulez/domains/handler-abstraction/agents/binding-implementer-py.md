---
name: binding-implementer-py
description: Implements Python (PyO3) Handler binding. Manages RequestData serialization to Python objects, async integration with pyo3_async_runtimes, and HandlerResponse deserialization. Ensures raw_body is preferred for performance. Handles Python exception conversion to HandlerError. Owns /crates/spikard-py/ Python binding implementation.
---
Model: sonnet

Context:
- HANDLER-ABSTRACTION.md
- ../../../crates/spikard-py/src/
- ../../../crates/spikard-py/src/handler.rs
- ../../../crates/spikard-py/tests/
