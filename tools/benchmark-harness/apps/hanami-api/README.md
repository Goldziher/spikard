# Hanami API Benchmark Application

Complete Hanami API implementation with Dry::Schema validation for benchmarking against spikard-ruby and other frameworks.

## Framework Overview

**Hanami API** is a minimal, extremely fast Ruby framework optimized for HTTP APIs.

- **Performance**: 14,290+ req/s with 10,000 routes
- **Memory**: 53,988 bytes for 10,000 routes
- **Architecture**: Block-based routing with minimal overhead
- **Validation**: Dry::Schema for request validation (matching Pydantic patterns)
- **Server**: Puma with Rack interface

## Installation

```bash
cd tools/benchmark-harness/apps/hanami-api
bundle install
```

## Running

```bash
# Default port 8000
./server.rb

# Custom port
./server.rb 9000

# Using Rackup (alternative)
rackup config.ru -p 8000
```

## Implemented Endpoints

### JSON Bodies (POST)
- `/json/small` - Small payload with Dry::Schema validation
- `/json/medium` - Medium payload with nested address
- `/json/large` - Large payload with items array
- `/json/very-large` - Very large payload with metadata

### Multipart Forms (POST)
- `/multipart/small` - Small file upload
- `/multipart/medium` - Medium file upload
- `/multipart/large` - Large file upload

### URL-Encoded Forms (POST)
- `/urlencoded/simple` - Simple form with name/email
- `/urlencoded/complex` - Nested form with user/preferences

### Path Parameters (GET)
- `/path/simple/:id` - Single parameter
- `/path/multiple/:user_id/:post_id` - Multiple parameters
- `/path/deep/:org/:team/:project/:resource/:id` - Deep nesting
- `/path/int/:id` - Integer conversion
- `/path/uuid/:uuid` - UUID validation
- `/path/date/:date` - Date parameter

### Query Parameters (GET)
- `/query/few` - Few parameters (q, page, limit)
- `/query/medium` - Medium parameter count
- `/query/many` - Many parameters

### Health Checks
- `/health` - Health check
- `/` - Root endpoint

## Configuration

- **Host**: 0.0.0.0 (all interfaces)
- **Port**: Command-line argument (default: 8000)
- **Threads**: Single-threaded (1:1) for consistent benchmarking
- **Logging**: Silent mode enabled
- **Validation**: Dry::Schema for all request bodies

## Validation Strategy

### Dry::Schema Integration

All POST endpoints use Dry::Schema for validation matching Python Pydantic patterns:

```ruby
SmallPayloadSchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:description).filled(:string)
  required(:price).filled(:float)
  optional(:tax).maybe(:float)
end
```

**Validation Flow:**
1. Parse JSON body with `JSON.parse(env['rack.input'].read)`
2. Validate with `result = Schema.call(data)`
3. Check `result.success?`
4. Return 400 with `{errors: result.errors.to_h}` if invalid
5. Return validated data if successful

### Schema Patterns

- **JSON Schemas**: Use `Dry::Schema.JSON` for JSON body endpoints
- **Form Schemas**: Use `Dry::Schema.Params` for URL-encoded forms
- **Nested Objects**: Use `.hash(SubSchema)` for nested validation
- **Arrays**: Use `.array(:type)` or `.array(SubSchema)` for arrays
- **Optional Fields**: Use `.maybe(:type)` for nullable fields

## Schema Alignment

All schemas match Python Pydantic patterns for fair comparison:
- Field names use snake_case (Ruby convention)
- Type mappings: `str → :string`, `int → :integer`, `float → :float`, `bool → :bool`
- Optional fields handled with `optional(:field).maybe(:type)`
- Nested validation with inline schemas or schema composition
- Array validation with type constraints

## Testing

```bash
# Test health endpoint
curl http://localhost:8000/health

# Test JSON validation (valid)
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{"name":"Widget","description":"Test","price":19.99}'

# Test JSON validation (invalid - returns 400)
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{"name":"Widget"}'

# Test path parameters
curl http://localhost:8000/path/simple/123

# Test query parameters
curl 'http://localhost:8000/query/few?q=test&page=1&limit=10'
```

## Performance Notes

- **Zero-overhead routing**: Hanami::API uses block-based routing with minimal abstraction
- **Efficient validation**: Dry::Schema compiled schemas are highly optimized
- **Single-threaded**: Configured for 1:1 threads to match benchmark requirements
- **Silent logging**: Logging disabled to minimize I/O overhead in benchmarks
