# Roda Raw Benchmark Server

This is a **raw performance** Roda server that implements all benchmark workload endpoints **WITHOUT validation**. It measures Roda's baseline performance without Dry::Schema validation overhead.

## Purpose

This server is used to measure the performance difference between:
- **roda**: Full validation with Dry::Schema
- **roda-raw**: No validation, just JSON parsing and echo

## Endpoints

All 18 workload endpoints are implemented:

### JSON Body Endpoints
- `POST /json/small` - Small JSON payload (no validation)
- `POST /json/medium` - Medium JSON payload (no validation)
- `POST /json/large` - Large JSON payload (no validation)
- `POST /json/very-large` - Very large JSON payload (no validation)

### Multipart Form Endpoints
- `POST /multipart/small` - Small file upload
- `POST /multipart/medium` - Medium file upload
- `POST /multipart/large` - Large file upload

### URL-Encoded Form Endpoints
- `POST /urlencoded/simple` - Simple form data (no validation)
- `POST /urlencoded/complex` - Complex nested form data (no validation)

### Path Parameter Endpoints
- `GET /path/simple/:id` - Single path parameter
- `GET /path/multiple/:user_id/:post_id` - Multiple path parameters
- `GET /path/deep/:org/:team/:project/:resource/:id` - Deep path nesting
- `GET /path/int/:id` - Integer path parameter
- `GET /path/uuid/:uuid` - UUID path parameter
- `GET /path/date/:date` - Date path parameter

### Query Parameter Endpoints
- `GET /query/few?q=...&page=...&limit=...` - Few query parameters
- `GET /query/medium?...` - Medium number of query parameters
- `GET /query/many?...` - Many query parameters

### Health Check
- `GET /` - Root health check
- `GET /health` - Health check endpoint

## Dependencies

```ruby
gem 'roda'       # Routing framework
gem 'puma'       # HTTP server
gem 'rack'       # Rack interface
gem 'rackup'     # Rack server runner
```

**Note**: NO dry-schema dependency - validation is completely removed.

## Installation

```bash
bundle install
```

## Running

```bash
# Default port 8000
ruby server.rb

# Custom port
ruby server.rb 3000

# Via config.ru
rackup -p 8000
```

## Roda Plugins Used

1. **json** - Automatic JSON response encoding
2. **json_parser** - Automatic JSON request body parsing into `r.params`
3. **all_verbs** - Support for all HTTP verbs (GET, POST, PUT, DELETE, etc.)
4. **type_routing** - Type-based path parameter matching

## Performance Characteristics

- **No validation overhead** - Just parse JSON and echo back
- **Minimal middleware stack** - Only essential Roda plugins
- **Direct parameter access** - `r.params` already contains parsed JSON
- **Fast routing** - Roda's tree-based routing
- **Single-threaded Puma** - `Threads: '1:1'` for consistent benchmarking

## Comparison with Validated Server

The validated `roda` server includes:
- Dry::Schema validation for all JSON and form endpoints
- Type checking and coercion
- Nested schema validation
- Error message generation

This raw server removes all of that to isolate Roda's routing and JSON parsing performance.

## Expected Performance

This server should demonstrate:
1. Roda's baseline routing performance
2. JSON parsing overhead (json_parser plugin)
3. Response serialization cost (json plugin)
4. Puma single-threaded overhead

The difference between `roda` and `roda-raw` benchmarks shows the cost of Dry::Schema validation.
