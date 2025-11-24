# Background Tasks

Offload non-critical work from request handlers.

## Enqueue work

=== "Python"

    --8<-- "snippets/python/background_task.md"

=== "TypeScript"

    --8<-- "snippets/typescript/background_task.md"

=== "Ruby"

    ```ruby
    # Background task helpers are planned. Today, use external job runners (Sidekiq/Resque)
    # or spawn threads carefully; prefer external queues for production.
    ```

=== "Rust"

    ```rust
    // Use Tokio tasks or external queues; a dedicated background task API is planned.
    ```

## Tips
- Keep request handlers fast; enqueue email/notifications/ETL jobs instead of blocking responses.
- Prefer durable queues (Redis/SQS) over in-process threads for production workloads.
- Ensure idempotency when retrying background tasks.
