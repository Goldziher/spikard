## Middleware Stack

The default middleware stack (in order):

1. **Compression** - gzip/brotli compression (configurable)
2. **Request ID** - Unique request tracking
3. **Timeout** - Request timeout enforcement
4. **Rate Limit** - Per-IP rate limiting (if configured)
5. **Authentication** - JWT/Bearer token validation (if configured)
6. **User-Agent** - User agent parsing and validation
7. **CORS** - Cross-origin resource sharing (if configured)
8. **Handler** - Your application logic

See `ServerConfig` documentation for detailed configuration options.

## Validation

Validate requests against JSON schemas:

```rust
use spikard_http::validation::ValidateRequest;
use serde_json::json;

let schema = json!({
    "type": "object",
    "properties": {
        "name": { "type": "string" },
        "age": { "type": "integer", "minimum": 0 }
    },
    "required": ["name"]
});

request.validate_body(&schema)?;
```
