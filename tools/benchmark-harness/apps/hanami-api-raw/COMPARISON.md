# Hanami API: Validated vs Raw Performance Comparison

This document explains the differences between the two Hanami API benchmark servers and how to use them for performance comparison.

## Server Variants

### 1. `hanami-api` (Standard - With Validation)
- **Location:** `apps/hanami-api/`
- **Dependencies:** hanami-api, dry-schema, puma, rack, rackup
- **Validation:** Full Dry::Schema validation on all JSON/form endpoints
- **Error Handling:** Structured validation errors returned on failure
- **Use Case:** Measures real-world Hanami API performance with validation

### 2. `hanami-api-raw` (Raw - No Validation)
- **Location:** `apps/hanami-api-raw/`
- **Dependencies:** hanami-api, puma, rack, rackup (NO dry-schema)
- **Validation:** None - accepts any JSON body
- **Error Handling:** None - raw echo
- **Use Case:** Measures Hanami API baseline routing/response performance

## Key Differences

| Aspect | hanami-api | hanami-api-raw |
|--------|-----------|----------------|
| JSON parsing | ✓ JSON.parse | ✓ JSON.parse |
| Schema validation | ✓ Dry::Schema | ✗ None |
| Type coercion | ✓ Via schemas | ✗ None |
| Error handling | ✓ Structured | ✗ None |
| Response | Validated hash | Raw echo |
| Dependencies | 5 gems | 4 gems |
| Gem count | ~20 (with deps) | ~12 (with deps) |

## Code Comparison

### hanami-api (With Validation)
```ruby
post '/json/small' do
  body = JSON.parse(request.body.read)
  result = SmallPayloadSchema.call(body)

  if result.success?
    json(result.to_h)
  else
    halt 400, json({ errors: result.errors.to_h })
  end
end
```

### hanami-api-raw (No Validation)
```ruby
post '/json/small' do
  body = JSON.parse(env['rack.input'].read)
  json(body)
end
```

## Performance Testing

### Running Both Servers

```bash
# Terminal 1: Standard with validation
cd apps/hanami-api
bundle install
ruby server.rb 8000

# Terminal 2: Raw without validation
cd apps/hanami-api-raw
bundle install
ruby server.rb 8001
```

### Benchmark Example

```bash
# Test validated endpoint
wrk -t2 -c100 -d30s --latency \
  -s post-json.lua \
  http://localhost:8000/json/small

# Test raw endpoint
wrk -t2 -c100 -d30s --latency \
  -s post-json.lua \
  http://localhost:8001/json/small
```

### Expected Results

The raw version should show:
- **Lower latency** (no validation overhead)
- **Higher throughput** (fewer CPU cycles per request)
- **Less memory usage** (no schema objects)

The difference quantifies the **validation overhead** of Dry::Schema.

## Benchmark Scenarios

### 1. JSON Body Validation Overhead

Test both `/json/small`, `/json/medium`, `/json/large`, `/json/very-large` endpoints to see how validation overhead scales with payload size.

**Hypothesis:** Larger payloads show greater absolute validation overhead but similar relative overhead.

### 2. Routing Performance (No Validation Difference)

Test path parameter endpoints like `/path/simple/:id` - both servers should perform identically since there's no validation on either.

**Hypothesis:** Identical performance proves validation is the only difference.

### 3. URL-encoded Form Overhead

Test `/urlencoded/simple` and `/urlencoded/complex` to measure Dry::Schema::Params overhead.

**Hypothesis:** URL-encoded validation may have different overhead characteristics than JSON.

## Integration with Benchmark Harness

Both servers implement the same 18 workload endpoints:

- JSON: small, medium, large, very-large
- Multipart: small, medium, large (stubs)
- URL-encoded: simple, complex
- Path params: simple, multiple, deep, int, uuid, date
- Query params: few, medium, many
- Health: /, /health

This allows direct comparison with:
- `spikard-ruby` (Rust-backed with validation)
- `roda-raw` (Pure Ruby, no validation)
- `sinatra-raw` (Pure Ruby, no validation)

## Measurement Recommendations

### Key Metrics

1. **Requests/second** - Higher is better
2. **Latency p50/p95/p99** - Lower is better
3. **Memory usage** - Lower is better (RSS)
4. **CPU usage** - Lower is better (% utilization)

### Test Configuration

- Single-threaded Puma (`Threads: '1:1'`)
- Same wrk configuration for all tests
- Warmup period before measurement
- Multiple runs for statistical significance

### Analysis

Calculate **validation overhead percentage**:

```
overhead_pct = ((raw_rps - validated_rps) / raw_rps) * 100
```

For example:
- Raw: 10,000 req/s
- Validated: 7,000 req/s
- Overhead: 30%

This quantifies how much Dry::Schema validation costs for Hanami API.

## Future Comparisons

With both servers, you can answer:

1. **How much does Dry::Schema cost?** (hanami-api vs hanami-api-raw)
2. **How does Hanami compare to Roda?** (hanami-api-raw vs roda-raw)
3. **What's the Rust advantage?** (spikard-ruby vs hanami-api)
4. **Is validation the bottleneck?** (compare overhead % across frameworks)

## Notes

- Both servers use identical Puma configuration
- Both parse JSON with the same method
- The ONLY difference is Dry::Schema validation
- This provides a clean measurement of validation overhead
- Results help optimize validation strategies
