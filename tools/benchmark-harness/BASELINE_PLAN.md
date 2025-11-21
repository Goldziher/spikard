# Baseline Benchmark Plan

## Objective

Create baseline (no-validation) versions of all framework benchmarks to measure the overhead of validation libraries.

## Approach

For each framework, create a baseline version that:
1. Parses JSON from the request
2. Returns the same JSON back
3. Does **NO** type validation or DTO construction

This measures pure framework overhead without validation costs.

## Implementation Plan

### 1. Spikard Python Baseline

**Location**: `tools/benchmark-harness/apps/spikard-python-baseline/`

**Changes**:
- Remove all msgspec.Struct models
- Handlers accept `dict` type hint
- Just return the dict directly
- No validation or type checking

**Example**:
```python
@app.post("/json/small")
def post_json_small(body: dict) -> dict:
    return body
```

### 2. FastAPI Baseline

**Location**: `tools/benchmark-harness/apps/fastapi-baseline/`

**Changes**:
- Remove all Pydantic BaseModel classes
- Use `Request.json()` to get raw dict
- Return dict with `jsonify()`
- No Pydantic validation

**Example**:
```python
@app.post("/json/small")
async def post_json_small(request: Request):
    body = await request.json()
    return JSONResponse(body)
```

### 3. Robyn Baseline

**Location**: `tools/benchmark-harness/apps/robyn-baseline/`

**Changes**:
- Remove all Pydantic BaseModel classes
- Use `request.json()` to get raw dict
- Return dict with `jsonify()`
- No validation

**Example**:
```python
@app.post("/json/small")
async def post_json_small(request: Request):
    body = request.json()
    return jsonify(body)
```

### 4. Spikard Rust Baseline

**Location**: `tools/benchmark-harness/apps/spikard-rust-baseline/`

**Changes**:
- Remove all typed structs (SmallJson, MediumJson, etc.)
- Use `serde_json::Value` for all endpoints
- Just parse and return JSON
- Minimal serde validation (only JSON syntax)

**Example**:
```rust
async fn post_json_small(ctx: Context) -> HandlerResult {
    let value: serde_json::Value = ctx.json()?;
    Ok(Response::new(StatusCode::OK).json(&value)?)
}
```

### 5. Spikard Node Baseline

**Location**: `tools/benchmark-harness/apps/spikard-node-baseline/`

**Changes**:
- Remove all Zod schemas
- Remove `.parse()` validation calls
- Just parse JSON and return
- No type checking

**Example**:
```typescript
async function post_json_small(requestJson: string): Promise<string> {
  const request = JSON.parse(requestJson);
  return JSON.stringify(request.body);
}
```

### 6. Spikard Ruby Baseline

**Location**: `tools/benchmark-harness/apps/spikard-ruby-baseline/`

**Changes**:
- Remove all validation classes (SmallPayload, Address, etc.)
- Just return raw request body hash
- No type checking

**Example**:
```ruby
app.post '/json/small', handler_name: 'post_json_small' do |request|
  request[:body]
end
```

## Benchmark Comparisons

With baseline versions, we can measure:

### Validation Overhead Per Framework

1. **Spikard Python**:
   - Baseline (dict) vs msgspec.Struct
   - Measures msgspec validation cost

2. **FastAPI**:
   - Baseline (dict) vs Pydantic
   - Measures Pydantic validation cost

3. **Robyn**:
   - Baseline (dict) vs Pydantic
   - Measures Pydantic validation cost in Robyn

4. **Spikard Rust**:
   - Baseline (serde_json::Value) vs typed structs (serde Deserialize)
   - Measures serde struct deserialization cost

5. **Spikard Node**:
   - Baseline (JSON.parse) vs Zod validation
   - Measures Zod runtime validation cost

6. **Spikard Ruby**:
   - Baseline (hash) vs custom validation classes
   - Measures Ruby class validation cost

### Cross-Framework Comparison

With baselines, we can also compare:
- Pure framework overhead (no validation)
- Rust vs Python vs Node vs Ruby baseline performance
- Impact of validation relative to baseline

## Expected Results

From previous benchmarks:

**With Validation** (current):
- Spikard Python: ~20,700 req/s (msgspec.Struct)
- FastAPI: ~12,700 req/s (Pydantic)

**Without Validation** (old invalid benchmarks):
- Spikard Python: ~112,000 req/s (5.4x faster!)
- FastAPI: ~57,700 req/s (4.5x faster!)

This shows validation overhead is **massive** (~80% throughput drop).

## Implementation Steps

1. ✅ Document the plan (this file)
2. Create baseline app directories
3. Copy existing apps and strip validation
4. Test each baseline app
5. Run benchmarks baseline vs validated
6. Document results
7. Update VALIDATION_STATUS.md with baseline findings

## Files to Create

- `tools/benchmark-harness/apps/spikard-python-baseline/server.py`
- `tools/benchmark-harness/apps/fastapi-baseline/server.py`
- `tools/benchmark-harness/apps/robyn-baseline/server.py`
- `tools/benchmark-harness/apps/spikard-rust-baseline/src/main.rs`
- `tools/benchmark-harness/apps/spikard-node-baseline/server.ts`
- `tools/benchmark-harness/apps/spikard-ruby-baseline/server.rb`

## Testing

For each baseline, verify:

1. **Accepts valid JSON**: Should work exactly like validated version
```bash
curl -X POST 'http://localhost:PORT/json/small' \
  -H 'Content-Type: application/json' \
  -d '{"name":"Foo","description":"Bar","price":35.4,"tax":3.2}'
```

2. **Accepts invalid JSON**: Should NOT reject (no validation)
```bash
curl -X POST 'http://localhost:PORT/json/small' \
  -H 'Content-Type: application/json' \
  -d '{"name":"Foo","description":"Bar","price":"NOT A NUMBER","tax":3.2}'
```

The second request should **succeed** in baseline (no validation) but **fail** in validated version.
