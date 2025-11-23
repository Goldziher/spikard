# Hono RAW Benchmark Server

This is a **no-validation** version of the Hono benchmark server designed to measure Hono's raw performance without the overhead of Zod validation.

## Purpose

Measures the baseline performance of:
- Hono framework routing and request handling
- @hono/node-server adapter
- JSON parsing and serialization
- WITHOUT Zod validation overhead

## Key Differences from Standard Hono

- **NO Zod validation**: Accepts any JSON body
- **NO @hono/zod-validator**: Direct JSON parsing
- **Echo responses**: Simply returns the parsed request body
- **Same endpoints**: All 18 workload endpoints implemented

## Endpoints

### JSON Body Workloads (NO VALIDATION)
- `POST /json/small` - Small payload echo
- `POST /json/medium` - Medium payload echo
- `POST /json/large` - Large payload echo
- `POST /json/very-large` - Very large payload echo

### Multipart Form Workloads
- `POST /multipart/small` - Mock 1 file, 1KB
- `POST /multipart/medium` - Mock 2 files, 10KB
- `POST /multipart/large` - Mock 5 files, 100KB

### URL Encoded Form Workloads
- `POST /urlencoded/simple` - Simple form echo
- `POST /urlencoded/complex` - Complex form echo

### Path Parameter Workloads
- `GET /path/simple/:id` - Single path param
- `GET /path/multiple/:user_id/:post_id` - Two path params
- `GET /path/deep/:org/:team/:project/:resource/:id` - Five path params
- `GET /path/int/:id` - Integer path param
- `GET /path/uuid/:uuid` - UUID path param
- `GET /path/date/:date` - Date path param

### Query Parameter Workloads
- `GET /query/few` - Few query params
- `GET /query/medium` - Medium query params
- `GET /query/many` - Many query params

### Health Check
- `GET /health` - Returns `{"status": "ok"}`
- `GET /` - Returns `{"status": "ok"}`

## Usage

```bash
# Install dependencies
pnpm install

# Start server (default port 8000)
pnpm start

# Start on custom port
pnpm start 9000

# Development mode with watch
pnpm dev
```

## Performance Comparison

Compare with the standard Hono server to measure validation overhead:

```bash
# Standard Hono (with Zod validation)
cd ../hono && pnpm start 8000

# Hono RAW (no validation)
cd ../hono-raw && pnpm start 8001
```

Run benchmarks against both servers to quantify the performance impact of Zod validation.

## Implementation Notes

- Uses TypeScript with tsx for development
- ESM modules (`"type": "module"`)
- Direct JSON parsing via `c.req.json()`
- No validation schemas or runtime checks
- Minimal dependencies: only hono and @hono/node-server
