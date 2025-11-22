# Litestar Benchmark Application

This directory contains a Litestar HTTP server implementation for workload benchmarking against spikard-python, FastAPI, and the pure Rust baseline.

## Architecture

- **Framework**: [Litestar](https://litestar.dev/) - A lightweight, high-performance ASGI framework
- **Validation**: msgspec.Struct for zero-overhead serialization and validation
- **Server**: Uvicorn with single worker for fair comparison
- **Python Version**: 3.10+

## Key Features

### DTOs with msgspec.Struct

Following Litestar best practices, all request/response DTOs use `msgspec.Struct` instead of Pydantic for maximum performance:

```python
class SmallPayload(msgspec.Struct):
    name: str
    description: str
    price: float
    tax: float | None = None
```

### Zero Thread Overhead

All handlers use `sync_to_thread=False` to avoid unnecessary thread pool dispatch:

```python
@post("/json/small", sync_to_thread=False)
def post_json_small(data: SmallPayload) -> SmallPayload:
    return data
```

### Type-Safe Path Parameters

Path parameters use type hints for automatic validation and conversion:

```python
@get("/path/int/{id:int}", sync_to_thread=False)
def get_path_int(id: int) -> dict[str, int]:
    return {"id": id}
```

## Endpoints

### JSON Body Endpoints
- `POST /json/small` - Small payload (~100-500 bytes)
- `POST /json/medium` - Medium payload (~1-10KB) with nested objects
- `POST /json/large` - Large payload (~10-100KB) with deep nesting
- `POST /json/very-large` - Very large payload (~100KB-1MB) with arrays

### Multipart Form Endpoints
- `POST /multipart/small` - Small multipart form (~1KB)
- `POST /multipart/medium` - Medium multipart form (~10KB)
- `POST /multipart/large` - Large multipart form (~100KB)

### URL-Encoded Form Endpoints
- `POST /urlencoded/simple` - Simple form (3-5 fields)
- `POST /urlencoded/complex` - Complex form (10-20 fields)

### Path Parameter Endpoints
- `GET /path/simple/{id}` - Single path parameter
- `GET /path/multiple/{user_id}/{post_id}` - Multiple path parameters
- `GET /path/deep/{org}/{team}/{project}/{resource}/{id}` - Deep path (5 levels)
- `GET /path/int/{id}` - Integer path parameter with validation
- `GET /path/uuid/{uuid}` - UUID path parameter
- `GET /path/date/{date}` - Date path parameter

### Query Parameter Endpoints
- `GET /query/few` - Few query parameters (1-3)
- `GET /query/medium` - Medium query parameters (5-10)
- `GET /query/many` - Many query parameters (15+)

### Health Check
- `GET /health` - Health check endpoint

## Installation

Using uv (recommended):

```bash
cd /Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/apps/litestar
uv pip install -e .
```

Using pip:

```bash
cd /Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/apps/litestar
pip install -e .
```

## Running the Server

```bash
# Default port (8000)
python server.py

# Custom port
python server.py 8080
```

The server listens on `0.0.0.0` (not `127.0.0.1`) for fair comparison across different frameworks.

## Configuration

- **Host**: 0.0.0.0 (all interfaces)
- **Port**: Configurable via CLI argument (default: 8000)
- **Workers**: 1 (single worker for fair comparison)
- **Access Logs**: Disabled for performance
- **Log Level**: Error only

## Performance Optimizations

1. **msgspec.Struct**: Zero-copy serialization (2.5x-5.4x faster than Pydantic)
2. **sync_to_thread=False**: Avoid thread pool overhead for sync handlers
3. **Single worker**: Eliminate inter-process communication overhead
4. **Disabled access logs**: Reduce I/O overhead
5. **Type-safe validation**: Compile-time optimized validation via msgspec

## Schema Compatibility

All DTOs match the JSON schemas in `/Users/naamanhirschfeld/workspace/spikard/testing_data/`:
- `json_bodies/*.json` - Request/response body schemas
- `path_params/*.json` - Path parameter validation scenarios
- `query_params/*.json` - Query parameter validation scenarios

## References

- [Litestar Documentation](https://docs.litestar.dev/latest/)
- [Litestar DTO Best Practices](https://docs.litestar.dev/latest/usage/dto/0-basic-use.html)
- [msgspec Documentation](https://jcristharif.com/msgspec/)
