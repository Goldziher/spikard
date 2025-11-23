# Roda vs Roda-Raw Performance Comparison

This document explains the architectural differences between the two Roda implementations.

## Architecture Comparison

### Roda (with validation)

```ruby
require 'roda'
require 'dry/schema'

SmallPayloadSchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:description).filled(:string)
  required(:price).filled(:float)
  optional(:tax).maybe(:float)
end

class BenchmarkApp < Roda
  def validate_and_respond(schema, data)
    result = schema.call(data)
    if result.success?
      result.to_h
    else
      response.status = 400
      { errors: result.errors.to_h }
    end
  end

  route do |r|
    r.on 'json' do
      r.post 'small' do
        body = JSON.parse(request.body.read)
        validate_and_respond(SmallPayloadSchema, body)
      end
    end
  end
end
```

**Dependencies**: roda, dry-schema, puma, rack, rackup

**Process per request**:
1. Manual `JSON.parse(request.body.read)`
2. Schema validation via `Dry::Schema`
3. Type checking and coercion
4. Error message generation
5. Serialize validated hash to JSON response

### Roda-Raw (no validation)

```ruby
require 'roda'

class BenchmarkApp < Roda
  plugin :json
  plugin :json_parser

  route do |r|
    r.on 'json' do
      r.post 'small' do
        r.params  # Already parsed by json_parser plugin
      end
    end
  end
end
```

**Dependencies**: roda, puma, rack, rackup (NO dry-schema)

**Process per request**:
1. Automatic JSON parsing via `json_parser` plugin → `r.params`
2. Echo back params (no validation)
3. Serialize to JSON via `json` plugin

## Performance Implications

### What Roda-Raw Eliminates

1. **Dry::Schema overhead**
   - Schema definition parsing
   - Type checking and coercion
   - Nested validation logic
   - Error message generation

2. **Manual JSON parsing**
   - `JSON.parse(request.body.read)` replaced by automatic `r.params`

3. **Validation logic**
   - No `validate_and_respond` method calls
   - No success/failure branching
   - No error hash construction

### What Remains (Roda Core Overhead)

Both implementations share:
- Roda's tree-based routing
- Rack middleware stack
- Puma HTTP server
- JSON response serialization
- HTTP request/response handling

## Expected Performance Characteristics

### Roda (validated)
- Higher latency due to validation
- More CPU cycles per request
- Memory allocation for schema objects
- Better type safety and error handling

### Roda-Raw
- Lower latency (baseline Roda performance)
- Minimal CPU overhead
- Less memory allocation
- No type safety (accepts any JSON)

## Benchmark Metrics to Compare

1. **Throughput (req/sec)**: Roda-raw should show higher numbers
2. **Latency (p50, p90, p99)**: Roda-raw should show lower latency
3. **Memory usage**: Roda-raw should use less memory
4. **CPU utilization**: Roda should show higher CPU due to validation

## Use Cases

### When to use Roda (validated)
- Production APIs requiring type safety
- Public endpoints needing input validation
- Applications with complex validation rules

### When to use Roda-Raw
- Internal microservices with trusted clients
- High-throughput proxies/gateways
- Benchmarking Roda's baseline performance
- Applications with client-side validation

## Conclusion

The difference between these two implementations isolates the **cost of validation**.
By comparing their benchmark results, we can quantify the overhead of Dry::Schema
validation versus Roda's raw routing and JSON handling performance.
