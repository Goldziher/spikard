---
name: lifecycle-hook-execution
priority: high
---

Lifecycle hooks (onRequest, preValidation, preHandler, onResponse, onError) must execute
in proper order per docs/adr/0005-lifecycle-hooks.md. Hooks are zero-cost when None.
Async hooks must work correctly with Python (pyo3_async_runtimes) and TypeScript (ThreadsafeFunction).
Hook errors must not prevent response transmission; must be logged and included in response metadata.
