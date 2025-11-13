# Background Task Runtime

**Date:** February 2025
**Status:** Draft
**Related:** [streaming-responses](./streaming-responses.md), [lifecycle-hooks-api-design](./lifecycle-hooks-api-design.md)

## Executive Summary

We need a portable, language-agnostic background task API so handlers can launch non-blocking work (audit logging, webhook fan-out, etc.) without holding request threads. This document proposes a Tokio-backed supervisor with bounded concurrency, deterministic shutdown, and async-first bindings for Python, Node.js, and Ruby.

## 1. Overview

Handlers frequently need to trigger follow-up work (write to data lakes, enqueue emails) where latency isn’t user-critical. Today they must spawn their own tasks; that’s error-prone, impossible from Python/Node/Ruby, and bypasses our graceful shutdown. We’ll provide a first-party API with predictable semantics.

### Goals
- ✅ Fire-and-forget execution that never blocks request handlers.
- ✅ Cross-language API parity (Python/Node/Ruby) anchored in a single Rust executor.
- ✅ Deterministic shutdown: drain or cancel jobs when the server stops, with metrics and error hooks.

### Non-Goals
- ❌ Durable persistence or distributed queues (no Redis/SQS abstraction).
- ❌ Cron/scheduled jobs—they can compose later but not part of this MVP.
- ❌ Strict ordering guarantees across requests; each job runs best-effort in FIFO submission order.

## 2. Background / Context

Frameworks like FastAPI expose `BackgroundTasks`, Fastify offers “onResponse” hooks + worker queues, and NestJS leans on BullMQ. These inspire the ergonomics we want, but we’ll keep things lightweight and in-process.

### Research (2024-2025)
- FastAPI’s `background.add_task(...)` uses Starlette tasks and drains them during shutdown—simple but Python-only.
- Fastify’s `@fastify/sensible` `request.runInBackground` wraps `setImmediate`; concurrency is unbounded unless users add plugins.
- Tokio’s `JoinSet` gives us a cheap way to manage thousands of concurrent tasks with cancellation support (Tokio blog, 2024).

## 3. Design

### 3.1 Runtime Supervisor

- Central struct `BackgroundExecutor` lives in `spikard-http`.
- Components:
  - `mpsc::Sender<BackgroundJob>` for submissions (bounded, default 1024).
  - `JoinSet<()>` to drive actual futures, limited by `max_concurrent_jobs`.
  - Metrics counters (queued, running, failed) exposed via `ServerMetrics`.
- Flow:
  1. Handler pushes jobs via `BackgroundHandle::spawn(future)`.
  2. Executor receives job, either schedules immediately (if below limit) or stores in a FIFO waiting queue.
  3. Each job runs on Tokio, capturing logs/panics; failure triggers `BackgroundErrorHook`.
  4. On shutdown, executor stops accepting new jobs, waits up to configurable timeout, and cancels leftovers.

**Rationale:** Single supervisor guarantees we can drain tasks and avoids each language creating unmanaged threads.

**Alternatives considered:** letting each binding spawn tasks directly (impossible to coordinate) or leaning on third-party job libraries (too heavy; introduces new infra dependencies).

### 3.2 Language Bindings

- **Rust:** `BackgroundHandle` injected into handlers (`HandlerContext`) with `spawn`.
- **Python:** expose `background.run(coro)`; on servers it enqueues into the shared executor, and in unit tests it falls back to `asyncio.create_task` (or a dedicated worker thread when no loop exists) so semantics stay identical.
- **Node.js:** `background.run(async () => { ... })`; uses napi `ThreadsafeFunction` to serialize closure into Rust.
- **Ruby:** `background.run { ... }`; procs are enqueued onto an internal `Queue` + worker thread so MRI never runs user code on foreign threads, but the API stays identical regardless of environment.
- All APIs return immediately; no join unless explicitly awaited (for tests).

**Rationale:** Async-first ensures parity with existing handler model and keeps language layers thin.

**Alternatives considered:** synchronous APIs that block until completion—would defeat the purpose and is hard to implement for Python/Node without re-entering event loops.

### 3.3 Observability & Shutdown

- Each job logs `task_id`, `request_id`, enqueue/start/finish timestamps.
- Expose metrics: `background_jobs_queued`, `background_jobs_running`, `background_jobs_failed_total`.
- Shutdown path integrates with existing `ServerConfig.shutdown_timeout`; new option `background_drain_timeout`.

**Rationale:** Operators must know if tasks were dropped; metrics align with our existing monitoring story.

**Alternatives considered:** letting tasks run indefinitely on shutdown—introduces undefined behavior and unbounded process lifetime.

## 4. API Design

### 4.1 Python API

```python
from spikard import background

@app.post("/events")
async def ingest(event: Event, request: Request):
    background.run(process_event(event, request.state["trace_id"]))
    return {"status": "accepted"}
```

### 4.2 TypeScript API

```typescript
import { background } from "@spikard/node";

app.post("/events", async (req) => {
  background.run(async () => {
    await auditStore.append(req.body);
  });
  return { status: "accepted" };
});
```

### 4.3 Rust API

```rust
pub async fn ingest(req: RequestData, bg: BackgroundHandle) -> HandlerResponse {
    bg.spawn(async move {
        audit::write(&req.body).await?;
        Ok(())
    });
    HandlerResponse::empty()
}
```

## 5. Implementation Strategy

### Phase 1: Runtime & Metrics (2 weeks)
- [ ] Add `BackgroundExecutor` + config knobs in `spikard-http`.
- [ ] Wire shutdown + metrics exposure.
- [ ] Unit tests for queue overflow, cancellation, panic propagation.

### Phase 2: Language Bindings (2 weeks)
- [ ] Rust API injection + Handle object.
- [ ] Python `background` module (PyO3).
- [ ] Node napi binding + TS typings.
- [ ] Ruby wrapper using magnus.

### Phase 3: Fixtures & Docs (1 week)
- [ ] Add `testing_data/background/*.json`.
- [ ] Generate e2e tests for each language (success/failure/drain).
- [ ] Docs + README updates.

## 6. Performance Considerations

| Metric | Target | Actual/Expected | Notes |
|--------|--------|-----------------|-------|
| Task submit overhead | < 5 µs | ~2 µs | MPSC send + cheap struct copy. |
| Max concurrent jobs | Configurable (default 128) | Bound by CPU | ensures no starvation. |
| Shutdown drain | ≤ shutdown timeout | Configurable | cancels remaining tasks. |

## 7. Testing Strategy

- Unit tests for executor (Tokio `#[tokio::test]`): overflow, queue ordering, panic logging.
- E2E fixtures verifying:
  - Handler returns immediately while job runs (`background_fire_and_forget`).
  - Failed job triggers hook but doesn’t crash server.
  - Shutdown waits for jobs (simulate long-running tasks).

```rust
#[tokio::test]
async fn cancels_after_timeout() {
    let exec = BackgroundExecutor::new(limit=1, timeout=Duration::from_millis(50));
    exec.spawn(async { tokio::time::sleep(Duration::from_secs(1)).await; });
    assert!(exec.shutdown().await.is_err());
}
```

## 8. Migration Guide (if applicable)

N/A – new capability.

## 9. Open Questions

- [ ] Should we expose named queues so apps can opt into ordering constraints?
- [ ] Do we need per-task priority, or is FIFO good enough for v1?
- [ ] How should we surface tracing spans—create dedicated “background” span or reuse request span?

## 10. References

### Specifications
- [Tokio JoinSet](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html)

### Libraries/Crates
- [tokio](https://crates.io/crates/tokio)
- [futures](https://crates.io/crates/futures)

### Prior Art
- [FastAPI BackgroundTasks](https://fastapi.tiangolo.com/tutorial/background-tasks/)
- [Fastify Background Jobs RFC](https://github.com/fastify/fastify/issues/4238)
- [NestJS + BullMQ](https://docs.nestjs.com/techniques/queues)

### Related Documents
- [streaming-responses](./streaming-responses.md)
- [lifecycle-hooks-api-design](./lifecycle-hooks-api-design.md)

---

**Key Takeaway:** A single, bounded Tokio supervisor—surfaced through async-friendly helpers in every language—gives us predictable, observable background job execution without introducing external infrastructure.
