# Requests & Responses

Handlers receive a context object tailored to each binding but backed by the same Rust data model.

## Request Data
- **Path params**: type-coerced when a converter is provided (e.g., `:id:int`).
- **Query params**: parsed into dicts/objects; validate with schemas or DTOs.
- **Headers & cookies**: accessible via the context; add middleware for cross-cutting auth/trace IDs.
- **Bodies**: JSON by default with optional form/multipart/file helpers depending on binding.

## Responses
- Return plain values (auto-serialized JSON) or explicit response types (JSON, stream, redirect, error).
- Streaming responses can yield chunks/frames and integrate with SSE/WebSockets.
- Use typed DTOs to keep schemas consistent across languages.

## Examples
### Python
```python
@app.post("/orders")
async def create_order(order: Order) -> Order:
    return order
```

### Rust
```rust
app.post("/orders", |ctx: Context| async move {
    let order: Order = ctx.json()?;
    Ok(Json(order))
});
```

### Error Handling
- Raise/return typed errors; the runtime maps them to RFC 9457-style payloads.
- Middleware can standardize error envelopes and logging.

See [Types reference](../reference/types.md) for the canonical shapes across bindings.
