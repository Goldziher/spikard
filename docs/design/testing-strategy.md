# Testing Strategy

## Overview

Spikard uses a **fixture-driven testing approach** where all test scenarios are defined as JSON fixtures that can be consumed by tests in any language (Python, Rust, TypeScript).

## Core Philosophy

### Write Once, Test Everywhere

Each test scenario is a JSON fixture that defines:
- Request details (method, path, query params, headers, cookies, body)
- Expected response (status code, headers, body)
- Metadata (description, source test from FastAPI/Starlette/Litestar)

### Test Fixtures → Spikard Development (Not Test App Development)

**Critical Principle**: The fixtures are for developing and testing **Spikard's internals**, not for building complex test applications.

**What this means:**
- **Fixtures define Spikard's behavior specification** - each fixture documents how Spikard should handle a specific scenario
- **Test apps must be MINIMAL** - they contain no business logic, no "smart" routing, no special case handling
- **Spikard does the work** - validation, routing, error handling, status codes all handled by Spikard's engine
- **Test apps are scaffolding** - they exist only to exercise Spikard's functionality

**Example of what NOT to do:**
```rust
// ❌ BAD: Test app contains business logic
async fn handle_status_test(Path(code): Path<String>) -> Response {
    // Parsing status code from path - this is business logic!
    let status = code.parse::<u16>().unwrap_or(200);
    StatusCode::from_u16(status).unwrap()
}
```

**Example of what TO do:**
```rust
// ✓ GOOD: Minimal handler, let Spikard do validation
async fn handle_items(query: Query<ItemParams>) -> Response {
    // Spikard validated query params - just return success
    Json(json!({"items": []}))
}
```

**The Goal:**
- Use fixtures to drive TDD development of Spikard's Rust engine
- Ensure consistent behavior across all language bindings (Rust, Python, TypeScript, Ruby)
- Test that Spikard correctly handles validation, errors, routing, status codes
- Keep test apps as simple as possible - they're not the code under test!

## Fixture Structure

All fixtures live in `testing_data/` organized by category:

```
testing_data/
├── path_params/           # Path parameter handling
├── query_params/          # Query string parameters
├── headers/               # Request header handling
├── cookies/               # Cookie parsing and validation
├── json_bodies/           # JSON request body validation
├── multipart/             # Multipart form data & file uploads
├── url_encoded/           # URL-encoded form data
├── content_types/         # Content-Type negotiation
├── status_codes/          # HTTP status code handling
├── validation_errors/     # Validation error responses
├── edge_cases/            # Edge cases and error conditions
└── cors/                  # CORS handling
```

Each directory contains:
- `schema.json` - JSON Schema defining the fixture format
- `*.json` - Individual test fixtures

## Fixture Format

### Example: Query Parameter Test

```json
{
  "name": "Required integer parameter",
  "description": "Tests validation when required int parameter is provided",
  "source": {
    "framework": "fastapi",
    "test_file": "tests/test_query.py",
    "test_function": "test_query_int"
  },
  "request": {
    "method": "GET",
    "path": "/items",
    "query_params": {
      "item_id": "42"
    }
  },
  "expected_response": {
    "status_code": 200,
    "body": {
      "item_id": 42
    }
  }
}
```

### Example: Validation Error

```json
{
  "name": "Missing required field",
  "description": "Returns 422 when required field is missing",
  "source": {
    "framework": "fastapi",
    "test_file": "tests/test_body.py",
    "test_function": "test_post_body_missing_required"
  },
  "request": {
    "method": "POST",
    "path": "/users",
    "headers": {
      "Content-Type": "application/json"
    },
    "body": {
      "email": "alice@example.com"
    }
  },
  "expected_response": {
    "status_code": 422,
    "body": {
      "detail": [
        {
          "type": "missing",
          "loc": ["body", "name"],
          "msg": "Field required",
          "input": ""
        }
      ]
    }
  }
}
```

## Test Implementation

### Test Generator (E2E Tests)

For end-to-end testing, we use a **test generator** that creates minimal test applications from fixtures:

**Location**: `tools/test-generator/`

**Purpose**:
- Generate minimal Rust and Python test applications from fixtures
- Create test suites that verify Spikard handles each scenario correctly
- Output lives in `e2e/rust/` and `e2e/python/`

**Key Principle**: The generator creates the **simplest possible handlers** that:
1. Register routes from fixtures
2. Use Spikard's validation (what we're testing!)
3. Return success responses when validation passes
4. Let Spikard handle all errors, validation failures, status codes

**What the generator should NOT do:**
- Add business logic to handle different scenarios
- Parse parameters to determine behavior (e.g., parsing status codes from paths)
- Implement smart routing or conditional responses
- Duplicate logic that Spikard should handle

**What the generator SHOULD do:**
- Create one handler per unique route/method from fixtures
- Set up Spikard's validators with schemas from fixtures
- Return simple success responses
- Trust Spikard to handle validation, errors, and edge cases

**Example generated handler:**
```rust
// Simple handler - Spikard does all the validation work
async fn handle_items_get(
    uri: axum::http::Uri,
    headers: axum::http::HeaderMap,
) -> impl axum::response::IntoResponse {
    // Schema from fixture
    let schema: Value = serde_json::from_str(SCHEMA_JSON).unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Extract and validate - Spikard handles this!
    match validator.validate_and_extract(&query_params, ...) {
        Ok(validated) => {
            // Success - return 200 with validated data
            (StatusCode::OK, Json(validated))
        },
        Err(err) => {
            // Spikard already formatted the error - just return it
            (StatusCode::UNPROCESSABLE_ENTITY, Json(err.to_json()))
        }
    }
}
```

## Benefits

### 1. Language-Agnostic Tests

The same fixture validates behavior across:
- Python bindings (PyO3)
- TypeScript/Node bindings (napi-rs)
- Rust core
- WebAssembly bindings

### 2. FastAPI Compatibility

Fixtures are derived from FastAPI/Starlette tests, ensuring behavior parity:
- Request handling matches FastAPI conventions
- Error formats match FastAPI exactly
- Parameter parsing is identical

### 3. Regression Prevention

When a test passes, the fixture documents the expected behavior permanently:
- Adding features won't break existing behavior
- Refactoring is safe with comprehensive coverage
- Easy to bisect when regressions occur

### 4. Documentation

Fixtures serve as **executable documentation**:
- Each fixture documents a specific behavior
- Source attribution links back to reference implementations
- Examples can be used in user documentation

## Fixture Development Process

### 1. Extract from Reference Frameworks

We systematically extract test scenarios from:
- **FastAPI**: `tests/` directory
- **Starlette**: `tests/` directory
- **Litestar**: `tests/` directory

Process:
1. Identify test case in reference framework
2. Run the test to capture request/response
3. Create fixture JSON with all details
4. Add source attribution (framework, file, function)

### 2. Add Custom Scenarios

For Spikard-specific features or edge cases:
1. Write fixture describing the scenario
2. Implement the feature
3. Verify tests pass

### 3. Update Schema

When fixture format changes:
1. Update `schema.json` in the category directory
2. Run schema validation on all fixtures
3. Fix any validation errors

## Current Coverage

**Total Fixtures**: 367 comprehensive test scenarios

### Fixture Categories

| Category | Fixtures | Coverage |
|----------|----------|----------|
| Query Parameters | 71 | Format validators, arrays, separators, validation |
| JSON Bodies | 49 | Composition (oneOf/anyOf/not), nesting, constraints |
| Path Parameters | 36 | UUID versions, dates, string constraints |
| Headers | 33 | Bearer tokens, API keys, validation |
| Cookies | 26 | Security attributes (SameSite, Secure, HttpOnly) |
| Status Codes | 23 | HTTP status code handling |
| URL-Encoded | 22 | Form validation, nested objects, arrays |
| Multipart | 22 | File uploads, magic numbers, MIME validation |
| Validation Errors | 22 | Structured error responses |
| Edge Cases | 20 | Unicode, floats, boundaries, security |
| Content Types | 20 | MIME types, charset, parameters |
| HTTP Methods | 12 | Method handling |
| CORS | 10 | Preflight, origins, headers |

### Validation Features Covered

**String Constraints:**
- `minLength`, `maxLength`, `pattern` (regex)
- Format validators: `email`, `uuid`, `ipv4`, `ipv6`, `uri`, `hostname`
- UUID version enforcement (v3, v4, v5)
- Date/time formats: `date`, `time`, `date-time`, `duration`

**Numeric Constraints:**
- `minimum`, `maximum`, `exclusiveMinimum`, `exclusiveMaximum`
- `multipleOf` (step validation)
- Scientific notation, negative zero handling

**Array Constraints:**
- `minItems`, `maxItems`, `uniqueItems`
- Custom separators (pipe, semicolon, space)
- Sparse array validation

**Object Constraints:**
- `minProperties`, `maxProperties`
- `dependencies` (conditional required fields)
- `additionalProperties` (strict validation)
- `const` (literal value matching)

**Schema Composition:**
- `allOf` (intersection)
- `oneOf` (exclusive union)
- `anyOf` (union)
- `not` (negation)
- `$ref` and `definitions` (schema reuse)

**Security Features:**
- File magic number validation (PNG, JPEG, PDF)
- MIME type spoofing detection
- Bearer token format validation
- API key pattern validation
- Cookie security attributes
- Null byte injection prevention
- DoS prevention (nesting depth, payload size)

## Validation Error Format

All validation errors follow FastAPI's format with consistent structure:

```json
{
  "detail": [
    {
      "type": "int_parsing",           // Error type
      "loc": ["query", "item_id"],     // Parameter location
      "msg": "Input should be a valid integer",  // Human message
      "input": "abc",                  // Actual input value
      "ctx": {                         // Optional context
        "expected_type": "integer"
      }
    }
  ]
}
```

### Error Types

Common error types include:
- `missing` - Required parameter/field not provided
- `int_parsing` - Failed to parse integer
- `float_parsing` - Failed to parse float
- `bool_parsing` - Failed to parse boolean
- `uuid_parsing` - Invalid UUID format
- `string_too_short` - String length < minLength
- `string_too_long` - String length > maxLength
- `string_pattern_mismatch` - Regex pattern didn't match
- `enum` - Value not in allowed enum values
- `greater_than` - Numeric value too small (exclusive)
- `greater_than_equal` - Numeric value too small (inclusive)
- `less_than` - Numeric value too large (exclusive)
- `less_than_equal` - Numeric value too large (inclusive)

## Build & Test Commands

```bash
# Build Python bindings
task build:py

# Run all Python tests
task test:python

# Run specific test category
task test:python -k "path_params"
task test:python -k "query_params"

# Run single fixture test
task test:python -k "path_params::01_string_path_param"
```

## Design Decisions

### Single-Pass Validation

**Decision**: Validate once in Rust, never re-validate in Python/TypeScript.

**Rationale**:
- Rust validation is faster (no GIL, SIMD optimizations)
- Avoids double-validation overhead
- Universal validation across all language bindings
- JSON Schema as universal validation contract

### Always Include `input` Field

**Decision**: Always return the `input` field in validation errors, even when empty.

**Rationale**:
- Consistent error format regardless of error type
- Helpful for debugging (see what value was actually received)
- Matches Pydantic v2 behavior
- Makes error messages more actionable

### Fixture-First Development

**Decision**: Write fixtures before implementing features.

**Rationale**:
- Forces thinking about API contract first
- Prevents implementation bias in tests
- Ensures comprehensive test coverage
- Creates executable documentation

## Related Files

- `testing_data/` - All test fixtures
- `packages/python/tests/test_all_fixtures.py` - Parametrized test runner
- `packages/python/tests/conftest.py` - Test fixtures and helpers
- `crates/spikard-http/src/validation.rs` - Validation implementation
- `packages/python/spikard/_internal/json_schema.py` - Schema generation
