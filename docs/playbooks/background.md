# Background Tasks

Offload non-critical work from request handlers.

## Enqueue work

=== "Python"

    --8<-- "snippets/python/background_task.md"

=== "TypeScript"

    --8<-- "snippets/typescript/background_task.md"

=== "Ruby"

    ```ruby
    # Background task helpers are planned. Use external job runners (Sidekiq/Resque)
    # or a queue to defer work; avoid heavy work inside request handlers.
    ```

=== "Rust"

    ```rust
    use spikard::prelude::*;
    use tokio::task;

    app.route(post("/signup"), |ctx: Context| async move {
        let user: serde_json::Value = ctx.json()?;
        task::spawn(async move {
            // send email or enqueue to external system
        });
        Ok(Json(user))
    })?;
    ```

## Tips
- Keep request handlers fast; enqueue email/notifications/ETL jobs instead of blocking responses.
- Prefer durable queues (Redis/SQS) over in-process threads for production workloads; the built-in helpers are best-effort and in-process.
- Ensure idempotency when retrying background tasks.
