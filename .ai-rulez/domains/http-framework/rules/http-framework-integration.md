---
name: http-framework-integration
priority: high
---
HTTP framework must integrate seamlessly with Spikard-Core types (Route, Router, Method).
ServerConfig builder must expose all middleware options with sensible defaults.
Request/response lifecycle must support both synchronous and asynchronous handlers.
All middleware errors must be caught and converted to ProblemDetails responses.
