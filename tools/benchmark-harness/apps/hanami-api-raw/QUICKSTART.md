# Hanami API Raw - Quick Start Guide

## Installation

```bash
cd /Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/apps/hanami-api-raw
bundle install
```

## Run Server

```bash
# Default port 8000
ruby server.rb

# Custom port
ruby server.rb 9000

# Via rackup
bundle exec rackup -p 8000
```

## Test Endpoints

```bash
# Health check
curl http://localhost:8000/health
# Response: {"status":"ok"}

# JSON endpoint (accepts ANY JSON)
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{"name":"test","price":99.99,"extra_field":"will_appear","anything":true}'
# Response: {"name":"test","price":99.99,"extra_field":"will_appear","anything":true}

# Path parameters
curl http://localhost:8000/path/simple/42
# Response: {"id":"42"}

# Multiple path parameters
curl http://localhost:8000/path/multiple/123/456
# Response: {"user_id":"123","post_id":"456"}

# Query parameters
curl "http://localhost:8000/query/few?q=search&page=1&limit=20"
# Response: {"q":"search","page":1,"limit":20}

# URL-encoded form
curl -X POST http://localhost:8000/urlencoded/simple \
  -d "name=test&email=test@example.com&extra=field"
# Response: {"name":"test","email":"test@example.com","extra":"field"}
```

## All Endpoints (18 total)

### JSON Body (4 endpoints)
- `POST /json/small` - Small payload
- `POST /json/medium` - Medium payload  
- `POST /json/large` - Large payload
- `POST /json/very-large` - Very large payload

### Multipart Form (3 endpoints - stubs)
- `POST /multipart/small`
- `POST /multipart/medium`
- `POST /multipart/large`

### URL-encoded Form (2 endpoints)
- `POST /urlencoded/simple`
- `POST /urlencoded/complex`

### Path Parameters (6 endpoints)
- `GET /path/simple/:id`
- `GET /path/multiple/:user_id/:post_id`
- `GET /path/deep/:org/:team/:project/:resource/:id`
- `GET /path/int/:id`
- `GET /path/uuid/:uuid`
- `GET /path/date/:date`

### Query Parameters (3 endpoints)
- `GET /query/few`
- `GET /query/medium`
- `GET /query/many`

### Health Checks (2 endpoints)
- `GET /`
- `GET /health`

## Key Features

- **NO validation** - Accepts any JSON body
- **Zero overhead** - No Dry::Schema dependency
- **Raw performance** - Measures Hanami API baseline
- **Complete workload coverage** - All 18 endpoints
- **Single-threaded Puma** - Consistent benchmarking

## Benchmark Example

```bash
# Install wrk
brew install wrk

# Create JSON payload
cat > /tmp/small.json << 'JSON'
{"name":"test","price":99.99,"description":"benchmark"}
JSON

# Benchmark
wrk -t2 -c100 -d30s --latency \
  -H "Content-Type: application/json" \
  --script=/tmp/post.lua \
  http://localhost:8000/json/small
```

## Compare with Validated Version

```bash
# Terminal 1: Run raw version
cd apps/hanami-api-raw
ruby server.rb 8000

# Terminal 2: Run validated version
cd apps/hanami-api
ruby server.rb 8001

# Terminal 3: Compare
wrk ... http://localhost:8000/json/small  # Raw
wrk ... http://localhost:8001/json/small  # Validated
```

## Expected Performance Characteristics

### vs hanami-api (validated)
- **Higher RPS** - No validation overhead
- **Lower latency** - Fewer CPU cycles
- **Less memory** - No schema objects

### vs spikard-ruby
- **Lower RPS** - Pure Ruby vs Rust core
- **Higher latency** - No zero-copy optimization
- **More memory** - Ruby objects vs Rust structs

## Dependencies

```ruby
# Gemfile
gem 'hanami-api', '~> 0.3'    # Routing framework
gem 'puma', '~> 7.1'          # HTTP server
gem 'rack', '~> 3.2'          # Rack interface
gem 'rackup', '~> 2.2'        # Server launcher
# NO dry-schema - that's the point!
```

Total: 13 gems (vs 20+ with dry-schema)

## Configuration

Server configured in `server.rb`:

```ruby
handler.run(
  BenchmarkApp.new,
  Port: port,
  Host: '0.0.0.0',      # Container-friendly
  Threads: '1:1',       # Single-threaded
  Silent: true          # No request logging
)
```

## Troubleshooting

### Port already in use
```bash
# Find and kill process
lsof -i :8000
kill -9 <PID>

# Or use different port
ruby server.rb 9000
```

### Bundle install fails
```bash
# Update bundler
gem install bundler

# Clear and reinstall
rm Gemfile.lock
bundle install
```

### Server not responding
```bash
# Check server is running
curl http://localhost:8000/health

# Check logs (if not in silent mode)
tail -f log/puma.stderr.log
```

## Files

- `server.rb` - Main server implementation (140 lines)
- `config.ru` - Rack configuration (3 lines)
- `Gemfile` - Dependencies (4 gems)
- `Gemfile.lock` - Locked versions (37 lines)
- `README.md` - Detailed documentation
- `COMPARISON.md` - Performance comparison guide
- `.ruby-version` - Ruby version (3.4.7)

## Next Steps

1. Start server: `ruby server.rb`
2. Test endpoints: `curl http://localhost:8000/health`
3. Run benchmarks: See COMPARISON.md
4. Compare results: vs hanami-api, spikard-ruby, roda-raw

For detailed performance analysis, see `COMPARISON.md`.
