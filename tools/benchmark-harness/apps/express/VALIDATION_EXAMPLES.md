# Express Zod Validation Examples

## Valid Requests

### Small Payload (Valid)
```bash
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Widget",
    "description": "A useful widget",
    "price": 19.99,
    "tax": 1.50
  }'
```

Response:
```json
{
  "name": "Widget",
  "description": "A useful widget",
  "price": 19.99,
  "tax": 1.50
}
```

### Medium Payload (Valid)
```bash
curl -X POST http://localhost:8000/json/medium \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": 123,
    "username": "johndoe",
    "email": "john@example.com",
    "is_active": true,
    "address": {
      "street": "123 Main St",
      "city": "Springfield",
      "state": "IL",
      "zip_code": "62701"
    },
    "tags": ["premium", "verified"]
  }'
```

Response:
```json
{
  "user_id": 123,
  "username": "johndoe",
  "email": "john@example.com",
  "is_active": true,
  "address": {
    "street": "123 Main St",
    "city": "Springfield",
    "state": "IL",
    "zip_code": "62701"
  },
  "tags": ["premium", "verified"]
}
```

## Invalid Requests (Zod Validation Errors)

### Missing Required Field
```bash
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Widget",
    "price": 19.99
  }'
```

Response (400 Bad Request):
```json
{
  "error": "Validation failed",
  "details": [
    {
      "code": "invalid_type",
      "expected": "string",
      "received": "undefined",
      "path": ["description"],
      "message": "Required"
    }
  ]
}
```

### Wrong Type
```bash
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Widget",
    "description": "A useful widget",
    "price": "nineteen dollars"
  }'
```

Response (400 Bad Request):
```json
{
  "error": "Validation failed",
  "details": [
    {
      "code": "invalid_type",
      "expected": "number",
      "received": "string",
      "path": ["price"],
      "message": "Expected number, received string"
    }
  ]
}
```

### Nested Object Validation Error
```bash
curl -X POST http://localhost:8000/json/medium \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": 123,
    "username": "johndoe",
    "email": "john@example.com",
    "is_active": true,
    "address": {
      "street": "123 Main St",
      "city": "Springfield"
    },
    "tags": ["premium", "verified"]
  }'
```

Response (400 Bad Request):
```json
{
  "error": "Validation failed",
  "details": [
    {
      "code": "invalid_type",
      "expected": "string",
      "received": "undefined",
      "path": ["address", "state"],
      "message": "Required"
    },
    {
      "code": "invalid_type",
      "expected": "string",
      "received": "undefined",
      "path": ["address", "zip_code"],
      "message": "Required"
    }
  ]
}
```

### Array Type Mismatch
```bash
curl -X POST http://localhost:8000/json/medium \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": 123,
    "username": "johndoe",
    "email": "john@example.com",
    "is_active": true,
    "address": {
      "street": "123 Main St",
      "city": "Springfield",
      "state": "IL",
      "zip_code": "62701"
    },
    "tags": "premium"
  }'
```

Response (400 Bad Request):
```json
{
  "error": "Validation failed",
  "details": [
    {
      "code": "invalid_type",
      "expected": "array",
      "received": "string",
      "path": ["tags"],
      "message": "Expected array, received string"
    }
  ]
}
```

### Extra Unknown Fields (Passed Through)
```bash
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Widget",
    "description": "A useful widget",
    "price": 19.99,
    "unknown_field": "ignored"
  }'
```

Response (200 OK - Zod strips unknown fields by default):
```json
{
  "name": "Widget",
  "description": "A useful widget",
  "price": 19.99
}
```

## Path Parameter Examples

### Simple Path
```bash
curl http://localhost:8000/path/simple/abc123
```

Response:
```json
{
  "id": "abc123"
}
```

### Integer Path
```bash
curl http://localhost:8000/path/int/42
```

Response:
```json
{
  "id": 42
}
```

### Multiple Path Parameters
```bash
curl http://localhost:8000/path/multiple/user-456/post-789
```

Response:
```json
{
  "user_id": "user-456",
  "post_id": "post-789"
}
```

### Deep Nested Paths
```bash
curl http://localhost:8000/path/deep/acme/engineering/webapp/api/v1
```

Response:
```json
{
  "org": "acme",
  "team": "engineering",
  "project": "webapp",
  "resource": "api",
  "id": "v1"
}
```

## Query Parameter Examples

### Few Parameters
```bash
curl "http://localhost:8000/query/few?page=1&limit=10"
```

Response:
```json
{
  "page": "1",
  "limit": "10"
}
```

### Medium Parameters
```bash
curl "http://localhost:8000/query/medium?name=john&age=30&city=NYC&active=true&role=admin"
```

Response:
```json
{
  "name": "john",
  "age": "30",
  "city": "NYC",
  "active": "true",
  "role": "admin"
}
```

### Many Parameters
```bash
curl "http://localhost:8000/query/many?a=1&b=2&c=3&d=4&e=5&f=6&g=7&h=8&i=9&j=10"
```

Response:
```json
{
  "a": "1",
  "b": "2",
  "c": "3",
  "d": "4",
  "e": "5",
  "f": "6",
  "g": "7",
  "h": "8",
  "i": "9",
  "j": "10"
}
```

## URL-Encoded Form Examples

### Simple Form
```bash
curl -X POST http://localhost:8000/urlencoded/simple \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "name=John&email=john@example.com"
```

Response:
```json
{
  "name": "John",
  "email": "john@example.com"
}
```

### Complex Form
```bash
curl -X POST http://localhost:8000/urlencoded/complex \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "user[name]=John&user[age]=30&user[email]=john@example.com&tags[]=admin&tags[]=premium"
```

Response (Express parses nested structures):
```json
{
  "user": {
    "name": "John",
    "age": "30",
    "email": "john@example.com"
  },
  "tags": ["admin", "premium"]
}
```

## Multipart Form Examples

Note: These endpoints currently return mock data since multipart parsing requires additional middleware (e.g., `multer`).

### Small Multipart
```bash
curl -X POST http://localhost:8000/multipart/small \
  -F "file=@small.txt"
```

Response:
```json
{
  "files_received": 1,
  "total_bytes": 1024
}
```

## Testing with HTTPie (Alternative to curl)

HTTPie provides a more user-friendly CLI for testing:

```bash
# Install HTTPie
pip install httpie

# POST JSON
http POST localhost:8000/json/small \
  name=Widget \
  description="A useful widget" \
  price:=19.99 \
  tax:=1.50

# GET with query params
http GET localhost:8000/query/few page==1 limit==10

# URL-encoded form
http --form POST localhost:8000/urlencoded/simple \
  name=John \
  email=john@example.com
```

## Zod Error Codes Reference

Common Zod error codes you'll encounter:

- `invalid_type` - Value is wrong type (e.g., string instead of number)
- `invalid_literal` - Value doesn't match literal type
- `unrecognized_keys` - Extra fields (when `.strict()` is used)
- `invalid_union` - None of the union types matched
- `invalid_enum_value` - Value not in enum
- `invalid_string` - String validation failed (email, url, etc.)
- `too_small` - Array/string/number below minimum
- `too_big` - Array/string/number above maximum
- `not_multiple_of` - Number is not multiple of specified value

## Schema Customization

To add stricter validation, modify schemas in `server.ts`:

```typescript
// Email validation
const MediumPayloadSchema = z.object({
  email: z.string().email(), // Validates email format
  // ...
});

// String length constraints
const SmallPayloadSchema = z.object({
  name: z.string().min(1).max(100),
  description: z.string().min(10).max(500),
  // ...
});

// Number ranges
const ItemSchema = z.object({
  price: z.number().min(0).max(10000),
  quantity: z.number().int().positive(),
  // ...
});

// Strict mode (reject unknown fields)
const StrictSchema = z.object({
  name: z.string(),
}).strict(); // Throws error on unknown fields
```

## Testing Validation in Integration Tests

Example pytest test:

```python
def test_express_validation_error():
    response = requests.post(
        "http://localhost:8000/json/small",
        json={"name": "Widget", "price": 19.99}  # Missing description
    )
    assert response.status_code == 400
    data = response.json()
    assert data["error"] == "Validation failed"
    assert any(err["path"] == ["description"] for err in data["details"])
```
