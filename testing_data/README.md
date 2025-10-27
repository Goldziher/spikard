# Testing Data

Language-agnostic test fixtures for comprehensive HTTP API testing across Python, Rust, and TypeScript.

## 📁 Directory Structure

```
testing_data/
├── README.md                 # This file
├── multipart/                # Multipart form data fixtures ✅
│   ├── schema.json
│   ├── files/               # Test file contents
│   └── *.json               # Test fixtures
├── query_params/            # Query parameter fixtures ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
├── headers/                 # HTTP header fixtures ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
├── cookies/                 # Cookie fixtures ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
├── json_bodies/             # JSON request body fixtures ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
├── url_encoded/             # URL-encoded form fixtures ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
├── path_params/             # Path parameter fixtures ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
├── status_codes/            # HTTP status code fixtures ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
├── content_types/           # Media type fixtures ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
├── validation_errors/       # Validation error fixtures ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
├── http_methods/            # HTTP methods (PUT, PATCH, DELETE, OPTIONS, HEAD) fixtures ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
├── cors/                    # CORS (Cross-Origin Resource Sharing) fixtures ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
├── edge_cases/              # Edge cases and boundary conditions ✅
│   ├── schema.json
│   └── *.json               # Test fixtures
└── scripts/                 # Utility scripts
    ├── validate.py          # Validate fixtures against schemas
    └── loader.py            # Helper to load fixtures in tests
```

## 🎯 Purpose

These fixtures enable:

1. **Cross-language testing** - Same test cases in Python, Rust, TypeScript
2. **Framework parity** - Extracted from FastAPI, Starlette, Litestar
3. **Comprehensive coverage** - 100+ fixtures covering entire HTTP API surface
4. **Validation** - JSON Schema for type safety and consistency
5. **Documentation** - Each fixture documents its source and purpose

## 🚀 Quick Start

### Python (pytest)

```python
from pathlib import Path
import json

FIXTURES_DIR = Path(__file__).parent.parent / "testing_data"

def load_fixture(category: str, name: str):
    """Load a test fixture by category and name."""
    path = FIXTURES_DIR / category / f"{name}.json"
    with open(path) as f:
        return json.load(f)

def test_multipart_upload(test_client):
    fixture = load_fixture("multipart", "01_simple_file_upload")

    # Build request from fixture
    files = {
        f["field_name"]: (
            f.get("filename"),
            f["content"],
            f.get("content_type")
        )
        for f in fixture["request"]["files"]
    }

    response = test_client.post(
        fixture["request"]["path"],
        files=files
    )

    assert response.status_code == fixture["expected_response"]["status_code"]
    assert response.json() == fixture["expected_response"]["body"]
```

### Rust

```rust
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn load_fixture(category: &str, name: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("testing_data")
        .join(category)
        .join(format!("{}.json", name));

    let content = fs::read_to_string(path).unwrap();
    serde_json::from_str(&content).unwrap()
}

#[test]
fn test_multipart_upload() {
    let fixture = load_fixture("multipart", "01_simple_file_upload");

    // Use fixture data
    let method = fixture["request"]["method"].as_str().unwrap();
    let path = fixture["request"]["path"].as_str().unwrap();
    // ... build request
}
```

### TypeScript (vitest)

```typescript
import { readFileSync } from 'fs';
import { join } from 'path';

const FIXTURES_DIR = join(__dirname, '../../testing_data');

function loadFixture(category: string, name: string) {
  const path = join(FIXTURES_DIR, category, `${name}.json`);
  return JSON.parse(readFileSync(path, 'utf-8'));
}

test('multipart upload', async () => {
  const fixture = loadFixture('multipart', '01_simple_file_upload');

  // Build request from fixture
  const formData = new FormData();
  for (const file of fixture.request.files) {
    formData.append(
      file.field_name,
      new Blob([file.content], { type: file.content_type }),
      file.filename
    );
  }

  const response = await fetch(fixture.request.path, {
    method: fixture.request.method,
    body: formData
  });

  expect(response.status).toBe(fixture.expected_response.status_code);
});
```

## 📋 Fixture Format

Each fixture is a JSON file following this pattern:

```json
{
  "name": "Human-readable test case name",
  "description": "What this test validates",
  "source": {
    "framework": "starlette|fastapi|litestar",
    "test_file": "path/to/original/test.py",
    "test_function": "test_function_name"
  },
  "request": {
    "method": "GET|POST|PUT|PATCH|DELETE",
    "path": "/api/endpoint",
    "headers": {},
    "query_params": {},
    "body": {}
  },
  "expected_response": {
    "status_code": 200,
    "body": {},
    "headers": {}
  }
}
```

## ✅ Validation

All fixtures must validate against their category's `schema.json`:

```bash
# Validate all fixtures
python testing_data/scripts/validate.py

# Validate specific category
python testing_data/scripts/validate.py multipart
```

## 🔗 Categories

### Multipart Form Data ✅ COMPLETE
- **Fixtures:** 16
- **Coverage:** File uploads (single, multiple, optional, required), form data only, mixed files and data, custom headers, file lists (arrays), empty files, various content types (text, image/jpeg, application/pdf), validation (size limits, content-type restrictions), files without filenames, multiple values same field
- **Status:** Ready for use

### Query Parameters ✅ COMPLETE
- **Fixtures:** 40
- **Coverage:** All basic types (str, int, float, bool, UUID, date, datetime), validation constraints (ge, le, lt, gt, min_length, max_length, regex patterns), enums, arrays (empty, single, multiple), optional/required, defaults, URL encoding (spaces, special chars), multiple params of different types
- **Status:** Ready for use

### Headers ✅ COMPLETE
- **Fixtures:** 28
- **Coverage:** Standard headers (Content-Type, Accept, Accept-Language, Accept-Encoding, Host, Origin, Referer), custom headers (X-*), authentication (Bearer, Basic, APIKey), case insensitivity, multiple values, validation constraints (min_length, max_length, regex patterns), underscore conversion
- **Status:** Ready for use

### Cookies ✅ COMPLETE
- **Fixtures:** 22
- **Coverage:** Request cookies (optional, required, multiple), response cookies (set, delete, multiple), authentication (APIKeyCookie), validation constraints (min_length, max_length, regex patterns), attributes (max_age, domain, path, secure, httponly), SameSite variations (strict, lax, none), session cookies
- **Status:** Ready for use

### JSON Bodies ✅ COMPLETE
- **Fixtures:** 27
- **Coverage:** Simple objects, nested objects (deeply nested), arrays (of objects, of primitives), required/optional fields, field type validation, string validation (min_length, max_length, regex patterns), numeric validation (ge, le), boolean fields, null values, date/datetime fields, UUID fields, enum fields, empty objects/arrays, body with query params, PATCH partial updates, extra fields (ignored)
- **Status:** Ready for use

### URL-Encoded Forms ✅ COMPLETE
- **Fixtures:** 12
- **Coverage:** Simple form submission, required/optional fields, validation (min_length, max_length, regex patterns), special character encoding (spaces, &, etc.), multiple values for same field, empty string values, OAuth2 password grant flow, type conversion (numeric, boolean)
- **Status:** Ready for use

### Path Parameters ✅ COMPLETE
- **Fixtures:** 20
- **Coverage:** All basic types (str, int, float, UUID, bool, date), all validation constraints (gt, ge, lt, le, min_length, max_length, combined), enums, path type, multiple params
- **Status:** Ready for use

### Status Codes ✅ COMPLETE
- **Fixtures:** 18
- **Coverage:** Success codes (200 OK, 201 Created, 202 Accepted, 204 No Content, 206 Partial Content), redirect codes (301 Moved Permanently, 302 Found, 304 Not Modified, 307 Temporary Redirect), client error codes (400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, 408 Request Timeout, 422 Validation Error, 429 Too Many Requests), server error codes (500 Internal Server Error, 503 Service Unavailable)
- **Status:** Ready for use

### Content Types ✅ COMPLETE
- **Fixtures:** 12
- **Coverage:** JSON response (application/json), plain text (text/plain), HTML (text/html), XML (application/xml), binary data (application/octet-stream), PDF files (application/pdf), JPEG images (image/jpeg), PNG images (image/png), CSV exports (text/csv), UTF-8 charset, Accept header content negotiation, 415 Unsupported Media Type
- **Status:** Ready for use

### Validation Errors ✅ COMPLETE
- **Fixtures:** 20
- **Coverage:** Type validation errors (int, float, string, bool, UUID, datetime), numeric constraint violations (gt, ge, lt, le), string constraint violations (min_length, max_length, regex patterns), required field errors (query, body, header), enum validation, UUID format validation, datetime format validation, array constraints (min_items, max_items, item validation), multiple validation errors, nested object validation errors, header validation, malformed JSON
- **Status:** Ready for use

### HTTP Methods ✅ COMPLETE
- **Fixtures:** 12
- **Coverage:** PUT (complete replacement, create if not exists, idempotency, validation errors, with response body), PATCH (partial updates, multiple fields, validation errors), DELETE (resource removal, with response body, not found), OPTIONS (CORS preflight with headers), HEAD (metadata without body content)
- **Status:** Ready for use

### CORS ✅ COMPLETE
- **Fixtures:** 5
- **Coverage:** Simple CORS request (Origin header, Access-Control-Allow-Origin), OPTIONS preflight request (Access-Control-Request-Method, Access-Control-Request-Headers, Access-Control-Allow-Methods, Access-Control-Max-Age), credentials support (Access-Control-Allow-Credentials), wildcard origin (*), blocked request (missing CORS headers)
- **Status:** Ready for use

### Edge Cases ✅ COMPLETE
- **Fixtures:** 6
- **Coverage:** Unicode and emoji handling (UTF-8, multi-byte characters, Japanese, German), large integer boundaries (max safe integer, 64-bit int limits, BigInt), floating-point precision (0.1 + 0.2 != 0.3, rounding), deeply nested structures (10+ levels), special string values (empty, whitespace, tabs, newlines, quotes, backslashes, unicode escapes, special characters), empty and null value handling (null vs empty vs missing, zero, false)
- **Status:** Ready for use

## 📖 Documentation

Each category contains:
- `schema.json` - JSON Schema defining fixture structure
- `*.json` - Individual test fixtures

All documentation is centralized in this README.

## 🤝 Contributing

When adding fixtures:

1. Follow the category's JSON Schema
2. Extract from real framework test cases
3. Document the source (framework, file, function)
4. Add clear name and description
5. Include expected behavior
6. Validate before committing

## 📚 References

- FastAPI: `/tmp/framework-tests/fastapi/`
- Starlette: `/tmp/framework-tests/starlette/`
- Litestar: `/tmp/framework-tests/litestar/`
- Main plan: `TESTING_TODO.md`

## 📊 Progress

- ✅ Multipart: 16/16 fixtures (100%)
- ✅ Query Params: 40/40 fixtures (100%)
- ✅ Headers: 28/28 fixtures (100%)
- ✅ Cookies: 22/22 fixtures (100%)
- ✅ Path Params: 20/20 fixtures (100%)
- ✅ JSON Bodies: 27/27 fixtures (100%)
- ✅ URL-Encoded: 12/12 fixtures (100%)
- ✅ Status Codes: 18/18 fixtures (100%)
- ✅ Content Types: 12/12 fixtures (100%)
- ✅ Validation Errors: 20/20 fixtures (100%)
- ✅ HTTP Methods: 12/12 fixtures (100%)
- ✅ CORS: 5/5 fixtures (100%)
- ✅ Edge Cases: 6/6 fixtures (100%)

**Total:** 238/238 fixtures (100%) 🎉
