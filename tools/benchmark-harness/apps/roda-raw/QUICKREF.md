# Roda-Raw Quick Reference

## One-Line Summary
Roda benchmark server with NO validation - measures raw Roda routing performance.

## File Structure

```
roda-raw/
├── server.rb          # Main server (149 lines)
├── config.ru          # Rack config (5 lines)
├── Gemfile            # Dependencies (8 lines, NO dry-schema)
├── .ruby-version      # Ruby 3.2.0
├── test.sh            # Test script (executable)
├── README.md          # Full documentation
├── COMPARISON.md      # vs validated roda
├── INSTALLATION.md    # Setup guide
├── ENDPOINTS.txt      # Endpoint inventory
└── QUICKREF.md        # This file
```

## Quick Commands

```bash
# Install
bundle install

# Run (port 8000)
ruby server.rb

# Run (custom port)
ruby server.rb 3000

# Test
./test.sh

# Syntax check
ruby -c server.rb
```

## All 18 Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| POST | /json/small | Small JSON (no validation) |
| POST | /json/medium | Medium JSON (no validation) |
| POST | /json/large | Large JSON (no validation) |
| POST | /json/very-large | Very large JSON (no validation) |
| POST | /multipart/small | Small file upload |
| POST | /multipart/medium | Medium file upload |
| POST | /multipart/large | Large file upload |
| POST | /urlencoded/simple | Simple form (no validation) |
| POST | /urlencoded/complex | Complex form (no validation) |
| GET | /path/simple/:id | Single path param |
| GET | /path/multiple/:user_id/:post_id | Multiple path params |
| GET | /path/deep/:org/:team/:project/:resource/:id | Deep nesting |
| GET | /path/int/:id | Integer conversion |
| GET | /path/uuid/:uuid | UUID param |
| GET | /path/date/:date | Date param |
| GET | /query/few | Few query params |
| GET | /query/medium | Medium query params |
| GET | /query/many | Many query params |

Plus: `GET /` and `GET /health` for health checks.

## Dependencies

```ruby
roda      # Routing framework
puma      # HTTP server
rack      # Rack interface
rackup    # Server runner
```

**Removed**: dry-schema (no validation)

## Roda Plugins

```ruby
plugin :json         # Auto JSON response
plugin :json_parser  # Auto JSON request parsing
plugin :all_verbs    # All HTTP methods
plugin :type_routing # Type-based routing
```

## Key Differences from `roda`

| Feature | roda | roda-raw |
|---------|------|----------|
| Validation | Dry::Schema | None |
| JSON Parsing | Manual (`JSON.parse`) | Automatic (`r.params`) |
| Error Handling | Validation errors | None |
| Dependencies | 5 gems | 4 gems (no dry-schema) |
| Response Time | Higher | Lower |
| Type Safety | Yes | No |

## Performance Expectations

**roda-raw should be faster** because:
- No schema validation
- No type checking
- No error message generation
- Fewer object allocations
- Less CPU per request

Benchmark to measure the **cost of validation**.

## Example Requests

```bash
# Health check
curl http://localhost:8000/health

# JSON (accepts ANY data)
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{"any":"fields","work":true}'

# Path params
curl http://localhost:8000/path/simple/abc123

# Query params
curl "http://localhost:8000/query/few?q=test&page=1&limit=10"

# Form data
curl -X POST http://localhost:8000/urlencoded/simple \
  -d "name=John&email=john@example.com"
```

## Architecture

```
Request → Puma → Rack → Roda Router
                         ↓
                    json_parser plugin
                         ↓
                    r.params (parsed)
                         ↓
                    Return r.params
                         ↓
                    json plugin
                         ↓
                  JSON Response
```

**No validation step** - just parse and echo.

## Use Cases

1. **Benchmarking**: Measure Roda baseline performance
2. **Comparison**: Calculate validation overhead
3. **Internal APIs**: Trusted clients, no validation needed
4. **Proxies**: High-throughput gateways

## Verification

```bash
# Should accept invalid data
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{"price":"not-a-number"}'

# Should return HTTP 200 with echoed data
# (validated roda would return HTTP 400)
```

## Notes

- Single-threaded (`Threads: '1:1'`) for consistent benchmarks
- Silent mode (`Silent: true`) to reduce overhead
- Frozen app (`BenchmarkApp.freeze.app`) for performance
- Port from `ARGV[0]` or default 8000
- Binds to `0.0.0.0` (all interfaces)

## Troubleshooting

```bash
# Port in use?
lsof -i :8000
kill -9 <PID>

# Dependencies?
bundle install

# Syntax error?
ruby -c server.rb
```

## Related Files

- `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/apps/roda/` - Validated version
- See `COMPARISON.md` for detailed architectural comparison
- See `INSTALLATION.md` for full setup guide
- See `README.md` for complete documentation

## Benchmarking

Use with wrk, ab, or Bombardier:

```bash
# wrk example
wrk -t4 -c100 -d30s --latency http://localhost:8000/health

# ab example
ab -n 10000 -c 100 http://localhost:8000/health

# Bombardier example
bombardier -c 100 -d 30s http://localhost:8000/health
```

Compare results with `roda` (validated) to measure validation cost.
