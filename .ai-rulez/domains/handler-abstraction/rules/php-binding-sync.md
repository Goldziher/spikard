---
name: php-binding-sync
priority: high
---
PHP binding must:
- Use thread pool for concurrent PHP execution
- Convert RequestData to PHP array
- Support both sync and async-php patterns
- Handle PHP errors/exceptions -> HandlerError
- Validate phpstan type definitions
- Optimize for request-response cycles
