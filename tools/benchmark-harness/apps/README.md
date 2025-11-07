# Benchmark Test Applications

This directory contains minimal HTTP servers for benchmarking different frameworks and language bindings.

## Structure

```
apps/
├── spikard-python/    # Python bindings server
├── spikard-node/      # Node.js bindings server
├── spikard-ruby/      # Ruby bindings server
├── spikard-rust/      # Pure Rust server
├── fastapi/           # FastAPI comparison
└── fastify/           # Fastify comparison
```

## Endpoints

All servers implement the same set of endpoints for fair comparison:

### Health Check
```
GET /health
Response: {"status": "ok"}
```

### Simple Query Parameter
```
GET /query?query=test
Response: "foo bar test"
```

### Create User (JSON Body)
```
POST /users
Body: {"name": "Alice", "email": "alice@example.com"}
Response: {"id": 1, "name": "Alice", "email": "alice@example.com"}
```

### Get User (Path Parameter)
```
GET /users/{user_id}
Response: {"id": 123, "name": "User 123"}
```

### Validated Input
```
POST /validated
Body: {"value": 42}
Response: {"value": 42}
Note: value must be > 0
```

## Running Servers

### Spikard Python
```bash
cd apps/spikard-python
python server.py 8000
```

### Spikard Node
```bash
cd apps/spikard-node
node server.js 8000
```

### FastAPI
```bash
cd apps/fastapi
python server.py 8000
```

## Implementation Notes

- All servers use port from command line argument (default: 8000)
- All servers run single-process mode for fair comparison
- Logging is disabled or minimized for benchmarking
- All servers implement identical validation logic

## Adding New Frameworks

1. Create directory: `apps/my-framework/`
2. Implement the 5 standard endpoints
3. Accept port as CLI argument
4. Add `/health` endpoint
5. Update this README
