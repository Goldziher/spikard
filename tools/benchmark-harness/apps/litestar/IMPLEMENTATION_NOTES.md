# Litestar Implementation Notes

## Key Differences from FastAPI

### 1. Validation Library
- **FastAPI**: Uses Pydantic BaseModel
- **Litestar**: Uses msgspec.Struct for 2.5x-5.4x better performance

```python
# FastAPI
class SmallPayload(BaseModel):
    name: str
    price: float

# Litestar
class SmallPayload(msgspec.Struct):
    name: str
    price: float
```

### 2. Route Decorator Syntax
- **FastAPI**: `@app.get()` / `@app.post()`
- **Litestar**: `@get()` / `@post()` with handlers registered in `Litestar(route_handlers=[...])`

```python
# FastAPI
@app.post("/json/small")
async def post_json_small(body: SmallPayload) -> SmallPayload:
    return body

# Litestar
@post("/json/small", sync_to_thread=False)
def post_json_small(data: SmallPayload) -> SmallPayload:
    return data
```

### 3. Thread Pool Control
- **FastAPI**: All sync handlers run in thread pool by default
- **Litestar**: Use `sync_to_thread=False` to avoid thread pool overhead for I/O-free handlers

### 4. Path Parameter Type Hints
- **FastAPI**: Type hints in function signature only
- **Litestar**: Explicit type declaration in path string for performance optimization

```python
# FastAPI
@app.get("/path/int/{id}")
async def get_path_int(id: int = PathParam()) -> dict[str, int]:
    return {"id": id}

# Litestar
@get("/path/int/{id:int}", sync_to_thread=False)
def get_path_int(id: int) -> dict[str, int]:
    return {"id": id}
```

### 5. Form Data Handling
- **FastAPI**: `Form(...)` parameters
- **Litestar**: `Body(media_type=RequestEncodingType.URL_ENCODED)` with msgspec.Struct

```python
# FastAPI
@app.post("/urlencoded/simple")
async def post_urlencoded_simple(
    username: str = Form(...),
    email: str = Form(...),
) -> dict[str, str]:
    return {"username": username, "email": email}

# Litestar
class SimpleForm(msgspec.Struct):
    username: str
    email: str

@post("/urlencoded/simple", sync_to_thread=False)
def post_urlencoded_simple(
    data: SimpleForm = Body(media_type=RequestEncodingType.URL_ENCODED),
) -> dict[str, str]:
    return {"username": data.username, "email": data.email}
```

### 6. Async vs Sync Handlers
- **FastAPI**: All handlers declared as `async def`
- **Litestar**: Sync handlers with `sync_to_thread=False` for better performance when no I/O

This eliminates unnecessary event loop overhead for handlers that just transform data.

## Performance Optimizations

1. **msgspec.Struct**: Zero-copy serialization, 2.5x-5.4x faster than Pydantic
2. **sync_to_thread=False**: Avoid thread pool dispatch overhead
3. **Type declarations in paths**: Compile-time optimization for path parameter parsing
4. **Single worker**: Eliminate inter-process communication overhead (matches FastAPI config)
5. **Disabled access logs**: Reduce I/O overhead (matches FastAPI config)

## Schema Compatibility

All endpoints match the FastAPI implementation:
- ✅ 4 JSON body endpoints (small, medium, large, very-large)
- ✅ 3 multipart form endpoints (small, medium, large)
- ✅ 2 URL-encoded form endpoints (simple, complex)
- ✅ 6 path parameter endpoints (simple, multiple, deep, int, uuid, date)
- ✅ 3 query parameter endpoints (few, medium, many)
- ✅ 1 health check endpoint

**Total: 19 endpoints** (identical to FastAPI)

## References

- [Litestar DTO Documentation](https://docs.litestar.dev/latest/usage/dto/0-basic-use.html)
- [msgspec Performance](https://jcristharif.com/msgspec/benchmarks.html)
- [Litestar Custom Types](https://docs.litestar.dev/2/usage/custom-types.html)
