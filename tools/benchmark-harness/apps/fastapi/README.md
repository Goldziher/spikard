# FastAPI Benchmark Server

Complete FastAPI implementation matching all spikard-python workload endpoints for fair performance comparison.

## Features

- **Full endpoint parity**: 19 endpoints matching spikard-python exactly
- **Production-ready configuration**: Uvicorn with optimized settings
- **Pydantic validation**: Automatic request/response validation using Pydantic BaseModel
- **Async handlers**: All endpoints use async/await for optimal performance
- **Schema alignment**: Exact field-level match with spikard-python msgspec.Struct schemas

## Endpoints

### JSON Body Workloads (POST)
- `/json/small` - Small payload (~100-500 bytes) with SmallPayload validation
- `/json/medium` - Medium payload (~1-10KB) with nested Seller object
- `/json/large` - Large payload (~10-100KB) with deep Address/Country nesting
- `/json/very-large` - Very large payload (~100KB-1MB) with Tag arrays

### Path Parameter Workloads (GET)
- `/path/simple/{id}` - Single string parameter
- `/path/multiple/{user_id}/{post_id}` - Multiple parameters
- `/path/deep/{org}/{team}/{project}/{resource}/{id}` - Deep nesting (5 levels)
- `/path/int/{id}` - Integer parameter with type validation
- `/path/uuid/{uuid}` - UUID parameter
- `/path/date/{date}` - Date parameter

### Query Parameter Workloads (GET)
- `/query/few` - Few parameters (1-3): q, page, limit
- `/query/medium` - Medium parameters (5-10): category, tags, pricing, sorting
- `/query/many` - Many parameters (15+): param1-param15

### Form Workloads (POST)
- `/urlencoded/simple` - Simple form (3 fields): username, email, password
- `/urlencoded/complex` - Complex form (15 fields): user details, address, preferences

### Multipart Workloads (POST)
- `/multipart/small` - Small multipart (~1KB)
- `/multipart/medium` - Medium multipart (~10KB)
- `/multipart/large` - Large multipart (~100KB)

### Health Check (GET)
- `/health` - Returns `{"status": "ok"}`

## Installation

```bash
pip install -r requirements.txt
```

## Running

```bash
# Default port 8000
python server.py

# Custom port
python server.py 8080
```

## Configuration

The server is configured for fair benchmarking:

- **Host**: `0.0.0.0` (not `127.0.0.1`) for proper network access
- **Workers**: Single worker for consistent comparison
- **Logging**: Error level only, no access logs
- **Validation**: Pydantic validation enabled (production mode)

## Schema Design

All schemas mirror spikard-python's msgspec.Struct definitions:

```python
# SmallPayload matches testing_data/json_bodies/01_simple_object_success.json
class SmallPayload(BaseModel):
    name: str
    description: str
    price: float
    tax: float | None = None

# Nested structures match testing_data/json_bodies/04_nested_object_success.json
class Seller(BaseModel):
    name: str
    email: str | None = None

class MediumPayload(BaseModel):
    name: str
    description: str
    price: float
    seller: Seller
```

## Validation Strategy

FastAPI automatically validates all requests using Pydantic:
- **Type checking**: Automatic conversion and validation
- **Required fields**: Enforced by Pydantic
- **Nested objects**: Full recursive validation
- **Error responses**: Automatic 422 Unprocessable Entity with details

This represents production usage and enables fair "with validation" comparisons against spikard-python.

## Best Practices Applied

Based on FastAPI official documentation:

1. **Async handlers**: All endpoints use `async def` for I/O efficiency
2. **Type hints**: Full typing with Python 3.10+ union syntax (`str | None`)
3. **Pydantic models**: BaseModel for all structured data
4. **Path/Query separation**: Explicit `Path()` and `Query()` usage
5. **Production server**: Uvicorn with standard extensions for HTTP/1.1 and WebSocket support

## Performance Notes

- Single worker mode ensures consistent CPU usage for benchmarking
- Pydantic validation overhead is included (production-realistic)
- All responses are JSON-serialized through Pydantic's optimized encoder
- No middleware overhead beyond FastAPI defaults

## Comparison with spikard-python

| Aspect | FastAPI | spikard-python |
|--------|---------|----------------|
| Validation | Pydantic BaseModel | msgspec.Struct |
| Server | Uvicorn (ASGI) | Spikard (Tokio/Hyper/Tower) |
| Endpoints | 19 | 19 |
| Schema match | ✅ Exact | - |
| Async | ✅ async/await | ✅ async/await |
| Workers | 1 | 1 |

## Dependencies

- `fastapi>=0.109.0` - Web framework
- `uvicorn[standard]>=0.27.0` - ASGI server with HTTP/1.1 support
- `pydantic>=2.0.0` - Data validation
- `python-multipart>=0.0.9` - Form data parsing

## Resources

- [FastAPI Official Documentation](https://fastapi.tiangolo.com/)
- [Pydantic V2 Documentation](https://docs.pydantic.dev/)
- [Uvicorn Documentation](https://www.uvicorn.org/)
