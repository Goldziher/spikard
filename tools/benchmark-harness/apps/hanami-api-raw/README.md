# Hanami API Raw (No Validation) Benchmark Server

This is a **raw performance** variant of the Hanami API benchmark server that **excludes all validation logic** to measure the framework's baseline performance without validation overhead.

## Purpose

This server implements the same 18 workload endpoints as the standard `hanami-api` server but:

- **NO** Dry::Schema validation
- **NO** type checking or coercion
- Accepts any JSON body and echoes it back
- Measures Hanami API's raw routing and response performance

## Differences from Standard hanami-api

| Feature | hanami-api | hanami-api-raw |
|---------|-----------|----------------|
| Validation | Dry::Schema | None |
| JSON parsing | ✓ | ✓ |
| Error handling | Structured validation errors | None |
| Dependencies | hanami-api, dry-schema, puma | hanami-api, puma |
| Response | Validated data | Raw echo |

## Endpoints

All 18 workload endpoints are implemented:

### JSON Body Endpoints
- `POST /json/small` - Small payload (no validation)
- `POST /json/medium` - Medium payload (no validation)
- `POST /json/large` - Large payload (no validation)
- `POST /json/very-large` - Very large payload (no validation)

### Multipart Form Endpoints
- `POST /multipart/small` - Stub implementation
- `POST /multipart/medium` - Stub implementation
- `POST /multipart/large` - Stub implementation

### URL-encoded Form Endpoints
- `POST /urlencoded/simple` - Echo params
- `POST /urlencoded/complex` - Echo params

### Path Parameter Endpoints
- `GET /path/simple/:id`
- `GET /path/multiple/:user_id/:post_id`
- `GET /path/deep/:org/:team/:project/:resource/:id`
- `GET /path/int/:id`
- `GET /path/uuid/:uuid`
- `GET /path/date/:date`

### Query Parameter Endpoints
- `GET /query/few`
- `GET /query/medium`
- `GET /query/many`

### Health Checks
- `GET /health`
- `GET /`

## Installation

```bash
cd apps/hanami-api-raw
bundle install
```

## Running

```bash
# Default port 8000
ruby server.rb

# Custom port
ruby server.rb 9000

# Or via Rack
bundle exec rackup -p 8000
```

## Usage

```bash
# JSON endpoint (accepts any JSON)
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{"any": "data", "works": true}'

# Path parameters
curl http://localhost:8000/path/simple/123

# Query parameters
curl "http://localhost:8000/query/few?q=test&page=1&limit=10"

# Health check
curl http://localhost:8000/health
```

## Benchmark Comparison

Use this server to compare:

1. **Raw Hanami API performance** (this server)
2. **Hanami API + Dry::Schema** (standard hanami-api server)
3. **Spikard Ruby** with validation
4. Other Ruby frameworks

This helps quantify the validation overhead vs. raw framework performance.

## Configuration

- Single-threaded Puma: `Threads: '1:1'`
- Binds to `0.0.0.0` for container compatibility
- Silent mode enabled (no request logging)
- Port configurable via command-line argument

## Notes

- This server accepts **any** JSON body without validation
- No error handling for malformed JSON (intentional for raw performance)
- Multipart endpoints are stubs (same as standard server)
- URL-encoded endpoints echo all params without validation
