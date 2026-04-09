---
name: ruby-binding-threads
priority: high
---

Ruby binding must:

- Use background threads for async operations
- Convert RequestData to Ruby Hash
- Support Fiber/async_io patterns if available
- Handle Ruby exceptions -> HandlerError
- Maintain thread safety with Arc
- Load RBS type definitions for static checking
