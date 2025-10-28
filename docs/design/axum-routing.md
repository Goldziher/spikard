# Design Decision: Axum Router with Path Parameter Extraction

## Context

Spikard needs a high-performance HTTP router that can:

1. Match incoming requests to handler functions based on path patterns
2. Extract path parameters (e.g., `/users/{user_id}`)
3. Handle query parameters and request bodies
4. Integrate seamlessly with our Rust/Python architecture

This document describes our routing architecture and path parameter extraction approach.

## Decision

We use **Axum** as our HTTP framework with its built-in **matchit-based router** and native **path parameter extraction**.

### Architecture

```
HTTP Request
    ↓
Axum Router (matchit algorithm)
    ↓
Path<HashMap<String, String>> extractor
    ↓
Query<HashMap<String, String>> extractor
    ↓
Request body extraction (for POST/PUT/PATCH)
    ↓
JSON Schema validation
    ↓
Python handler
```

## Implementation

### Router Setup

Located in `crates/spikard-http/src/server.rs`:

```rust
use axum::Router as AxumRouter;
use axum::extract::{Path, Query};

// Routes use {param} syntax, same as FastAPI
let path = "/users/{user_id}/posts/{post_id}";

// Axum extracts parameters automatically
axum::routing::get(
    move |path_params: Path<HashMap<String, String>>,
          query_params: Query<HashMap<String, String>>,
          req| async move {
        let request_data = RequestData {
            path_params: path_params.0,  // HashMap<String, String>
            query_params: query_params.0,
            body: None,
        };
        handler.call(req, request_data).await
    }
)
```

### Path Pattern Syntax

Spikard uses the same `{param}` syntax as FastAPI and other modern frameworks:

```python
@app.get("/users/{user_id}")
def get_user(user_id: int):
    ...

@app.get("/posts/{post_id}/comments/{comment_id}")
def get_comment(post_id: str, comment_id: int):
    ...
```

No conversion is needed - Axum natively supports this syntax.

### Parameter Extraction Flow

1. **Axum Router**: Uses matchit algorithm to find matching route
2. **Path Extractor**: Extracts `{param}` values into `HashMap<String, String>`
3. **Query Extractor**: Parses query string into `HashMap<String, String>`
4. **Body Extractor**: Parses JSON body (POST/PUT/PATCH only)
5. **Validation**: All parameters validated against JSON Schema in Rust
6. **Type Conversion**: Python converts validated strings to typed objects using msgspec

### Request Body Handling

For POST, PUT, and PATCH requests:

```rust
axum::routing::post(
    move |path_params: Path<HashMap<String, String>>,
          query_params: Query<HashMap<String, String>>,
          req: axum::extract::Request| async move {
        // Extract body
        let (parts, body) = req.into_parts();
        let body_bytes = body.collect().await?.to_bytes();

        let body_value = if !body_bytes.is_empty() {
            serde_json::from_slice::<Value>(&body_bytes)?
        } else {
            None
        };

        let request_data = RequestData {
            path_params: path_params.0,
            query_params: query_params.0,
            body: body_value,
        };

        handler.call(req, request_data).await
    }
)
```

## Alternatives Considered

### 1. Starlette/FastAPI Routing

**Approach**: Use Starlette's Python-based routing.

**Rejected because**:
- Python routing would require holding the GIL
- Slower than Rust-based routing
- Doesn't align with our "Rust core, Python interface" philosophy
- Would duplicate effort across language bindings

### 2. actix-web

**Approach**: Use actix-web's routing system.

**Rejected because**:
- Axum has cleaner extractor pattern
- Axum is built on tokio (which we already use)
- Axum has better community momentum
- actix-web's actor model is unnecessary complexity

### 3. Custom Router Implementation

**Approach**: Write our own router with matchit or regex.

**Rejected because**:
- Reinventing the wheel
- Axum's router is battle-tested and optimized
- Would require significant maintenance effort
- Missing middleware ecosystem

### 4. serde_qs for Path Parameters

**Note**: The user mentioned "custom path parameter support via serde_qs" but based on the code review, we're using **Axum's native Path extractor**, not serde_qs.

- **serde_qs** is designed for complex query string deserialization (nested objects, arrays)
- **Axum's Path extractor** handles simple path parameters natively
- We may use serde_qs for query parameters in the future if we need complex nested structures

## Benefits

1. **Performance**: matchit algorithm is extremely fast (trie-based, O(log n) lookup)
2. **Battle-tested**: Axum is widely used in production Rust applications
3. **Ecosystem**: Rich middleware and extractor ecosystem
4. **Type Safety**: Rust's type system prevents routing errors at compile time
5. **Async-first**: Built on tokio, fully async/await
6. **No GIL**: Routing happens entirely in Rust without Python overhead
7. **Familiar Syntax**: `{param}` syntax matches FastAPI, making migration easier

## Trade-offs

1. **Rust Dependency**: Routing logic is in Rust, not Python (acceptable given performance benefits)
2. **Path Extraction Limitations**: Only extracts to `HashMap<String, String>` (type conversion happens later in Python)
3. **Framework Lock-in**: Committed to Axum's patterns (mitigated by its stability and popularity)

## Performance Characteristics

### matchit Router (used by Axum)

- **Algorithm**: Radix trie (compressed prefix tree)
- **Lookup**: O(log n) where n is number of routes
- **Memory**: O(m) where m is total path pattern length
- **Strengths**:
  - Fast matching for both static and dynamic paths
  - No regex compilation overhead
  - Deterministic performance

### Comparison to Regex-based Routing

| Feature | matchit (Axum) | Regex |
|---------|----------------|-------|
| Lookup time | O(log n) | O(n) worst case |
| Compilation | None | Per-route overhead |
| Memory | Lower | Higher (compiled regexes) |
| Ambiguity | Clear precedence | Order-dependent |

## Path Parameter Types

All path parameters are extracted as strings initially. Type conversion happens in the Python layer using msgspec based on handler type hints.

Example:

```python
@app.get("/users/{user_id}")
def get_user(user_id: int):  # msgspec converts string -> int
    ...
```

Flow:
1. Axum extracts: `{"user_id": "123"}` (string)
2. Rust validates: JSON Schema checks it matches integer pattern
3. Python converts: msgspec converts `"123"` → `123` (int) based on type hint

## Related Files

- `crates/spikard-http/src/server.rs` - Axum router setup and path extraction
- `crates/spikard-http/src/handler.rs` - Handler invocation with extracted parameters
- `crates/spikard-http/src/parameters.rs` - Parameter validation
- `packages/python/spikard/routing.py` - Python decorator API

## Future Enhancements

1. **Regex path parameters**: Support for patterns like `/files/{path:.*}` using Axum's regex support
2. **Path parameter constraints**: Type hints in path patterns (e.g., `/users/{user_id:int}`)
3. **serde_qs integration**: For complex nested query parameters
4. **Route grouping**: Prefix-based route groups with shared middleware
5. **Route introspection**: API to query registered routes at runtime
