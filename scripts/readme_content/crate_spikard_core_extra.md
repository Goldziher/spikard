## Core Types

- `Request` - HTTP request model with headers, cookies, body, and path parameters
- `Response` - HTTP response model with status, headers, and body
- `HandlerResult` - Standard result type for handlers
- `ValidationError` - Structured validation errors with field-level details
- `RequestContext` - Request execution context with metadata
- `RouteConfig` - Route configuration with validation schemas

## Architecture

`spikard-core` sits at the foundation of the Spikard architecture:

```
┌─────────────────────────────────────┐
│  Language Bindings                  │
│  (Python, Node, Ruby, PHP, Elixir)  │
└──────────────┬──────────────────────┘
               │ implements
┌──────────────▼──────────────────────┐
│  spikard-http (Axum Runtime)        │
└──────────────┬──────────────────────┘
               │ uses
┌──────────────▼──────────────────────┐
│  spikard-core (Primitives)          │
└─────────────────────────────────────┘
```

All language bindings depend on `spikard-core` to ensure consistent request/response handling across platforms.
