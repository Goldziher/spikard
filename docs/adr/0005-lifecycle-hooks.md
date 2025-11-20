# ADR 0005: Lifecycle Hooks
**Status**: Accepted
**Date**: 2025-11-20

## Context
Spikard exposes request lifecycle hooks that must be consistent across bindings and cheap when unused. Hooks are used for logging, auth/rate limiting, response shaping, and error handling.

## Decision
- **Hook stages** (in order): `on_request` → `pre_validation` → `pre_handler` → handler → `on_response` → `on_error` (error path).
- **Runtime location**: Implemented in `crates/spikard-http`; hooks are stored as optional Arc’d callbacks. Fast-path skips allocation when no hooks are registered.
- **Binding surface**:
  - Python: callbacks registered through `Spikard` methods; async supported via `pyo3_async_runtimes`.
  - Node/WASM: functions registered via `Spikard` hook arrays; async supported through napi/wasm glue.
  - Ruby: blocks registered on `Spikard::App`; Magnus handles calling back into Ruby.
- **Short-circuiting**: Hooks may return a Response to stop the pipeline. Otherwise they return the (possibly mutated) request/response.
- **Testing**: Fixture-driven scenarios under `testing_data/lifecycle_hooks` with e2e coverage in all language suites.

## Consequences
- Hook registration must remain optional to keep the zero-overhead fast path.
- New hook behavior requires updating fixtures and cross-language e2e tests.
- Bindings must translate hook return types into the runtime’s `Continue|ShortCircuit` contract.

## References
- Runtime: `crates/spikard-http/src/lifecycle`
- Fixtures: `testing_data/lifecycle_hooks/*`
- Tests: `e2e/*/tests/*lifecycle_hooks*`
