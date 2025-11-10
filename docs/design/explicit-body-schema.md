# Explicit Body Schema Support

## Problem Statement

Currently, Spikard extracts validation schemas from Python type hints. This works well for typed models (dataclass, Pydantic, TypedDict, etc.) but fails for generic types like `dict[str, Any]`.

**Current behavior:**
```python
@post("/login/")
def login(body: dict[str, Any]) -> Any:
    return {"ok": True}
```

Schema extracted: `{'type': 'object'}` - NO validation rules!

## Solution Design

Allow decorators to accept explicit JSON Schema for validation:

```python
@post("/login/", body_schema={
    "type": "object",
    "required": ["username", "password"],
    "properties": {
        "username": {"type": "string"},
        "password": {"type": "string"}
    }
})
def login(body: dict[str, Any]) -> Any:
    return {"ok": True}
```

## Architecture Changes

### 1. Python Layer (`packages/python/spikard/`)

#### 1.1 Decorator Updates (`routing.py`)
**Change:** Add `body_schema` parameter to all route decorators

```python
def post(path: str, *, body_schema: dict[str, Any] | None = None) -> Callable:
    """POST route decorator.

    Args:
        path: URL path pattern
        body_schema: Optional explicit JSON Schema for request body validation
                     (useful when using dict[str, Any] or other generic types)
    """
    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        from spikard.app import Spikard
        app = Spikard._current_instance
        if app is None:
            raise RuntimeError("No Spikard app instance found")
        return app._register_route("POST", path, body_schema=body_schema)(func)
    return decorator
```

**Apply to:** `get`, `post`, `put`, `patch`, `delete`, `head`, `options`, `trace`

#### 1.2 App Updates (`app.py`)
**Change:** Update `_register_route` to accept and store explicit schema

```python
def _register_route(
    self,
    method: str,
    path: str,
    *,
    body_schema: dict[str, Any] | None = None
) -> Callable[..., Any]:
    """Internal method to register a route.

    Args:
        method: HTTP method
        path: URL path pattern
        body_schema: Optional explicit body schema (overrides type hint extraction)
    """
    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        # Extract schemas from type hints
        request_schema, response_schema = extract_schemas(func)

        # Explicit body_schema takes precedence over extracted request_schema
        if body_schema is not None:
            request_schema = body_schema

        parameter_schema = extract_parameter_schema(func, path)

        # ... rest of existing logic

        route = Route(
            method=method,
            path=path,
            handler=wrapped_func,
            handler_name=func.__name__,
            request_schema=request_schema,  # Now can be explicit
            response_schema=response_schema,
            parameter_schema=parameter_schema,
            is_async=inspect.iscoroutinefunction(func),
        )
```

### 2. Rust Layer (`crates/spikard-py/src/`)

**No changes needed!** The Rust code already accepts `request_schema` from Python's Route object. The explicit schema will flow through automatically.

### 3. Test Generator (`tools/test-generator/src/python_app.rs`)

#### 3.1 Update Handler Generation
**Change:** For PlainDict handlers, emit explicit `body_schema` parameter

```rust
// Add decorator
if let Some(ref schema) = body_schema {
    if body_type == BodyType::PlainDict {
        // For PlainDict, pass explicit schema to decorator
        let schema_json = serde_json::to_string(schema)
            .unwrap_or_else(|_| "{}".to_string());
        code.push_str(&format!(
            "@{}(\"{}\", body_schema={})\n",
            method.to_lowercase(),
            route,
            schema_json
        ));
    } else {
        // For typed models, schema is extracted from type hints
        code.push_str(&format!("@{}(\"{}\")\n", method.to_lowercase(), route));
    }
} else {
    code.push_str(&format!("@{}(\"{}\")\n", method.to_lowercase(), route));
}
```

## Implementation Steps

### Phase 1: Python API (No Breaking Changes)
1. ✅ Update all decorators in `routing.py` to accept `body_schema` parameter
2. ✅ Update `_register_route` in `app.py` to accept and prioritize explicit schema
3. ✅ Add tests for explicit schema usage

### Phase 2: Test Generator
4. ✅ Update `python_app.rs` to emit `body_schema` for PlainDict handlers
5. ✅ Regenerate test apps with explicit schemas

### Phase 3: Verification
6. ✅ Run full test suite
7. ✅ Verify all PlainDict handlers now validate properly
8. ✅ Verify typed handlers still work (no regression)

## Benefits

1. **Comprehensive Testing:** Can test validation with all Python type systems
2. **User Flexibility:** Users can choose validation without type conversion
3. **Performance:** PlainDict with validation is fastest (no Python object construction)
4. **Backward Compatible:** Existing code without explicit schema still works

## Example Use Cases

### Use Case 1: Testing Framework (our case)
```python
# Rotate through all type systems while maintaining validation
@post("/login/", body_schema=LOGIN_SCHEMA)  # Handler 0: dict
def login_dict(body: dict[str, Any]) -> Any: ...

@post("/login2/")  # Handler 1: TypedDict (schema from type)
def login_typed(body: LoginBody) -> Any: ...
```

### Use Case 2: Performance-Critical Endpoints
```python
# Skip Python object construction for speed
@post("/webhook/", body_schema={
    "type": "object",
    "required": ["event", "data"],
    "properties": {
        "event": {"type": "string"},
        "data": {"type": "object"}
    }
})
def webhook(body: dict[str, Any]) -> Any:
    # Validated in Rust, received as dict
    event = body["event"]  # Type checked, but no object overhead
    ...
```

### Use Case 3: Dynamic Schemas
```python
# Schema determined at runtime
schema = load_schema_from_config()

@post("/dynamic/", body_schema=schema)
def dynamic_endpoint(body: dict[str, Any]) -> Any:
    ...
```

## Testing Strategy

1. **Unit tests:** Test decorator accepts and stores explicit schema
2. **Integration tests:** Verify Rust validates against explicit schema
3. **Regression tests:** Ensure typed handlers still work
4. **Generator tests:** Verify PlainDict handlers get explicit schema

## Risk Assessment

**Low Risk:**
- Backward compatible (new optional parameter)
- No changes to Rust validation logic
- Type hints still work as before
- Minimal code changes

**Testing Coverage:**
- Existing tests cover typed handlers
- New tests cover explicit schema
- Generated tests cover all type systems
