# Background Tasks

Offload non-critical work from request handlers to maintain fast response times and improve reliability.

## Enqueue work

=== "Python"

    --8<-- "snippets/python/background_task.md"

=== "TypeScript"

    --8<-- "snippets/typescript/background_task.md"

=== "Ruby"

    --8<-- "snippets/ruby/background_task.md"

=== "PHP"

    --8<-- "snippets/php/background_task.md"

=== "Rust"

    --8<-- "snippets/rust/background_task.md"

## Error Recovery Patterns

Handle failures gracefully with retries and dead letter queues.

=== "Python"

    --8<-- "snippets/python/background_error_recovery.md"

=== "TypeScript"

    --8<-- "snippets/typescript/background_error_recovery.md"

=== "Ruby"

    --8<-- "snippets/ruby/background_error_recovery.md"

=== "PHP"

    --8<-- "snippets/php/background_error_recovery.md"

=== "Rust"

    --8<-- "snippets/rust/background_error_recovery.md"

## Queue Monitoring

Monitor job health and status in production.

=== "Python"

    --8<-- "snippets/python/background_monitoring.md"

=== "TypeScript"

    --8<-- "snippets/typescript/background_monitoring.md"

=== "Ruby"

    --8<-- "snippets/ruby/background_monitoring.md"

=== "PHP"

    --8<-- "snippets/php/background_monitoring.md"

=== "Rust"

    --8<-- "snippets/rust/background_monitoring.md"

## Testing Background Jobs

Test asynchronous jobs reliably in your test suite.

=== "Python"

    --8<-- "snippets/python/background_testing.md"

=== "TypeScript"

    --8<-- "snippets/typescript/background_testing.md"

=== "Ruby"

    --8<-- "snippets/ruby/background_testing.md"

=== "PHP"

    --8<-- "snippets/php/background_testing.md"

=== "Rust"

    --8<-- "snippets/rust/background_testing.md"

## Tips

- **Keep request handlers fast**: Enqueue email, notifications, and ETL jobs instead of blocking responses.
- **Use durable queues**: Prefer Redis, SQS, or RabbitMQ over in-process threads for production workloads.
- **Ensure idempotency**: Jobs should be safe to retry. Check if work was already completed before processing.
- **Monitor queue depth**: Alert when queues grow too large or latency increases.
- **Set appropriate timeouts**: Prevent jobs from running indefinitely.
- **Use dead letter queues**: Capture failed jobs for manual investigation.
- **Test both success and failure paths**: Verify retry logic and error handling work as expected.
