# Python E2E Tests - Next Session Plan

## Current Status
- **Tests**: 366 total, 33 passing (9%), 333 failing (91%)
- **Commits**:
  - `cd62889` - Initial Python e2e infrastructure
  - `829a6e2` - Generator produces lint-clean code
  - `e978a55` - Added cookies support + parameter normalization

## Completed This Session ✅
1. **Cookies Support**: All HTTP methods now accept `cookies` parameter
2. **Parameter Normalization**: Strip underscores from Python param names (_id_ → id)
3. **Generator Fixes**: PascalCase classes, unused param prefixes, Python builtins avoidance

## Critical Issue to Fix: Body Validation Failure

### Symptom
```python
# Test expects 200, gets 422
json_data = {"in_stock": True, "name": "Item", "price": 42.0}
response = await client.post("/items/", headers=headers, json=json_data)
assert response.status_code == 200  # FAILS: status=422
```

### Error Message
```json
{
  "type": "type_error",
  "msg": "Input should be a valid unknown",
  "loc": ["body", "body"]
}
```

### Investigation So Far
1. **Generator creates**: `@post("/items/", body_schema={...})`
2. **app.py:61**: Sets `request_schema = body_schema` (explicit takes precedence)
3. **Route object**: Created with `request_schema` field
4. **Rust validator**: Gets schema but reports "unknown" type

### Root Cause Hypothesis
The JSON Schema being passed to Rust is likely malformed or incompatible with jsonschema crate's expectations.

**Check**:
```python
# In e2e/python/app/main.py, find:
@post("/items/", body_schema={...})
def post_items(_body: PostItemsBody, _limit: int) -> Any:
    ...

# The body_schema dict might be:
# 1. Missing required JSON Schema fields
# 2. Using wrong Draft version
# 3. Has type incompatibilities
```

### Debug Steps for Next Session

1. **Print the actual schema being registered**:
```python
# Add debug logging in packages/python/spikard/app.py around line 95:
print(f"DEBUG: Registering route {method} {path}")
print(f"DEBUG: request_schema = {request_schema}")
```

2. **Check Rust side schema reception**:
```bash
# Run with debug logging:
RUST_LOG=spikard_http=debug pytest e2e/python/tests/test_json_bodies.py::test_boolean_field__success -xvs 2>&1 | grep "schema"
```

3. **Test with minimal schema**:
```python
# Create a test handler with simple schema:
@post("/test", body_schema={"type": "object", "properties": {"name": {"type": "string"}}, "required": ["name"]})
def test_handler(body: dict[str, Any]) -> Any:
    return {"ok": True}
```

4. **Check JSON Schema Draft compatibility**:
- Rust uses jsonschema crate (Draft 2020-12)
- Python Pydantic uses Draft 2020-12
- Verify schemas match expected format

## Secondary Issues (After Body Validation Fixed)

### Status Codes (~50 tests)
**Issue**: Handlers always return 200, tests expect 201, 204, 404, etc.

**Fix**: Generator should use `expected_response.status_code` from fixtures:
```python
# In tools/test-generator/src/python_app.rs:
if let Some(body_json) = response_body {
    let status_code = fixtures[0].expected_response.status_code;
    code.push_str(&format!("    return {}, {}\n", body_json, status_code));
}
```

### Parameter Validation (~40 tests)
**Status**: Should work now with normalization fix, needs testing

### Edge Cases (~43 tests)
- Content-Type edge cases
- CORS handling
- Multipart/URL-encoded forms

## Commands for Next Session

```bash
# Rebuild bindings:
PYTHONPATH=packages/python uv run maturin develop --manifest-path crates/spikard-py/Cargo.toml

# Run failing test with debug:
PYTHONPATH=packages/python RUST_LOG=spikard_http=debug \
  .venv/bin/pytest e2e/python/tests/test_json_bodies.py::test_boolean_field__success -xvs

# Check pass rate:
PYTHONPATH=packages/python .venv/bin/pytest e2e/python/tests/ -v --tb=no 2>&1 | tail -3

# Test specific categories:
PYTHONPATH=packages/python .venv/bin/pytest e2e/python/tests/test_cookies.py -v --tb=no
PYTHONPATH=packages/python .venv/bin/pytest e2e/python/tests/test_path_params.py -v --tb=no
```

## Files to Focus On
- `packages/python/spikard/app.py` - Route registration, schema handling
- `crates/spikard-py/src/lib.rs` - Python→Rust bridge for schemas
- `crates/spikard-http/src/validation.rs` - Rust validation logic
- `tools/test-generator/src/python_app.rs` - Test generation

## Goal
Get to 366/366 tests passing (100%)
