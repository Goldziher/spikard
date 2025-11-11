# Body Size Limit Test Fixtures

This directory contains test fixtures for validating body size limit middleware behavior.

## Overview

The body size limit middleware enforces maximum request body sizes to prevent resource exhaustion attacks and handle upload limits appropriately. These fixtures test various scenarios across different content types and configuration patterns.

## Fixture Structure

All fixtures follow the schema defined in `schema.json` and include:

- **Handler configuration**: Route, method, and middleware settings
- **Request details**: Method, path, headers (including Content-Type and Content-Length), and simulated body size
- **Expected response**: Status code and response body (RFC 9457 format for errors)
- **Tags**: Categorization for test filtering

## Test Coverage

### Basic Scenarios (3 fixtures)
- `01_body_size_within_limit.json` - 1KB body with 10KB limit → 200 success
- `02_body_size_exceeds_limit.json` - 20KB body with 10KB limit → 413 error
- `03_body_size_exact_boundary.json` - Body at exact 10KB limit → 200 success

### Configuration Variations (3 fixtures)
- `04_small_limit_text_endpoint.json` - 1KB limit for text endpoints → 413 when exceeded
- `05_large_limit_upload_endpoint.json` - 100MB limit for file uploads → 200 success
- `06_per_route_limits.json` - Different limits per route (1KB text, 50MB upload) → 200 success

### Content Type Scenarios (4 fixtures)
- `07_json_body_exceeds_limit.json` - JSON payload exceeds 5KB limit → 413
- `08_form_data_exceeds_limit.json` - URL-encoded form exceeds 2KB limit → 413
- `09_multipart_upload_exceeds_limit.json` - Multipart upload exceeds 1MB limit → 413
- `10_streaming_body_exceeds_limit.json` - Chunked transfer exceeds 8KB limit → 413

## Error Format (RFC 9457)

413 Payload Too Large responses follow RFC 9457 problem details format:

```json
{
  "type": "https://spikard.dev/errors/payload-too-large",
  "title": "Payload Too Large",
  "status": 413,
  "detail": "Request body size (20480 bytes) exceeds maximum allowed size (10240 bytes)"
}
```

## Middleware Configuration

```json
{
  "body_size_limit": {
    "max_body_size_bytes": 10240,
    "per_route_limits": {
      "/api/text": 1024,
      "/api/upload": 52428800
    }
  }
}
```

## Integration with Tower-HTTP

These fixtures align with the tower-http middleware implementation documented in:
- `docs/design/middleware-lifecycle-optimization.md`
- `docs/design/tower-http-middleware.md`

The body size limit layer runs early in the middleware stack to reject oversized payloads before expensive processing.

## Testing Usage

Load these fixtures in Python integration tests:

```python
import pytest
from pathlib import Path
import json

@pytest.mark.parametrize("fixture_file", Path("testing_data/body_size_limit").glob("*.json"))
def test_body_size_limit_fixture(fixture_file, test_client):
    if fixture_file.name == "schema.json":
        pytest.skip("Skip schema file")

    with open(fixture_file) as f:
        fixture = json.load(f)

    # Configure middleware from fixture.handler.middleware
    # Make request from fixture.request
    # Assert response matches fixture.expected_response
```

## Size Reference

Common size limits:
- Text/JSON APIs: 1KB - 10KB
- Form submissions: 2KB - 100KB
- Avatar uploads: 1MB - 5MB
- Document uploads: 10MB - 100MB
- Video uploads: 100MB - 5GB

All sizes in bytes for precision.
