# Validation Strategy

## Overview

All validation happens in **Rust** for maximum performance, while maintaining excellent developer experience in Python and TypeScript by leveraging existing schema libraries.

## Core Strategy

**Pragmatic approach:** Don't reinvent Pydantic or Zod. Use JSON Schema as the interchange format.

```
Pydantic/msgspec/Zod → JSON Schema → Rust Validator → Validated Data
```

## Request Body Validation

### Python - Multiple Schema Sources

```python
from pydantic import BaseModel
import msgspec

# Option 1: Pydantic v2
class CreateUser(BaseModel):
    name: str
    email: str
    age: int

# Option 2: msgspec
class CreateUserMsg(msgspec.Struct):
    name: str
    email: str
    age: int

# Option 3: Raw JSON Schema
user_schema = {
    "type": "object",
    "properties": {
        "name": {"type": "string"},
        "email": {"type": "string", "format": "email"},
        "age": {"type": "integer", "minimum": 0}
    },
    "required": ["name", "email", "age"]
}

# All work the same way
@post("/users")
async def create_user(data: CreateUser) -> User:
    # data is validated by Rust, deserialized by Pydantic
    return await db.create_user(data)
```

### TypeScript - Zod and Others

```typescript
import { z } from 'zod';

const CreateUserSchema = z.object({
    name: z.string(),
    email: z.string().email(),
    age: z.number().int().min(0).max(150)
});

@Post('/users')
@ValidateBody(CreateUserSchema)
async createUser(@Body() data: z.infer<typeof CreateUserSchema>): Promise<User> {
    // data is validated by Rust, typed by Zod inference
    return db.createUser(data);
}
```

### Schema Extraction

Python extracts JSON Schema at route registration time:

```python
def extract_json_schema(schema_source: Union[Type, dict]) -> dict:
    """Extract JSON Schema from various sources"""

    # Already a dict - assume it's JSON Schema
    if isinstance(schema_source, dict):
        return schema_source

    # Pydantic v2
    if hasattr(schema_source, 'model_json_schema'):
        return schema_source.model_json_schema()

    # msgspec
    if hasattr(schema_source, '__json_schema__'):
        return schema_source.__json_schema__()

    # attrs
    if hasattr(schema_source, '__attrs_attrs__'):
        return attrs_to_json_schema(schema_source)

    # Dataclasses
    if is_dataclass(schema_source):
        return dataclass_to_json_schema(schema_source)

    raise ValueError(f"Cannot extract JSON Schema from {schema_source}")
```

### Rust Validation Implementation

```rust
use jsonschema::JSONSchema;
use serde_json::Value;

pub struct SchemaValidator {
    // Pre-compiled JSON Schema validator
    compiled: JSONSchema,
    schema: Value,
}

impl SchemaValidator {
    pub fn new(schema: Value) -> Result<Self, String> {
        let compiled = JSONSchema::compile(&schema)
            .map_err(|e| format!("Invalid JSON Schema: {}", e))?;
        Ok(Self { compiled, schema })
    }

    pub fn validate_json(&self, json_bytes: &[u8]) -> Result<Value, ValidationError> {
        // Parse JSON (zero-copy where possible)
        let value: Value = serde_json::from_slice(json_bytes)?;

        // Validate against schema
        self.compiled.validate(&value)
            .map_err(|errors| ValidationError::from_errors(errors.collect()))?;

        Ok(value)
    }
}
```

## Response Validation

Same approach for responses - extract schema from return type annotation:

```python
class User(BaseModel):
    id: int
    name: str
    email: str

@get("/users/{user_id}")
async def get_user(user_id: int) -> User:  # ← Response schema
    user = await db.get_user(user_id)
    return user  # Rust validates this matches User schema
```

Response validation is **optional** and primarily useful for:
- Development/debugging
- API contract enforcement
- OpenAPI documentation generation

## Parameter Validation

Parameters use a **simple type system** - no need for JSON Schema complexity.

### Path Parameters

```python
@get("/users/{user_id}")
async def get_user(user_id: int) -> User:  # ← Rust validates as int
    return await db.get_user(user_id)

from uuid import UUID

@get("/sessions/{session_id}")
async def get_session(session_id: UUID) -> Session:  # ← Rust validates UUID format
    return await db.get_session(session_id)
```

Supported types: `str`, `int`, `float`, `bool`, `UUID`

### Query Parameters

```python
@get("/users")
async def list_users(
    page: int = 1,              # Default value
    limit: int = 10,            # Default value
    search: Optional[str] = None,  # Optional
    active: bool = True         # Bool with default
) -> list[User]:
    return await db.get_users(page=page, limit=limit, search=search, active=active)

# Array parameters: /items?tags=python&tags=rust
@get("/items")
async def list_items(tags: list[str] = []) -> list[Item]:
    return await db.get_items_by_tags(tags)
```

### Header Parameters

```python
from typing import Annotated

@get("/protected")
async def protected_route(
    authorization: Annotated[str, Header()],  # Required
    x_api_key: Annotated[Optional[str], Header()] = None  # Optional
) -> dict:
    return {"auth": authorization}
```

## Error Messages

Clear, actionable error messages for validation failures:

```json
{
  "detail": [
    {
      "type": "missing_field",
      "loc": ["body", "email"],
      "msg": "Field required"
    },
    {
      "type": "value_error",
      "loc": ["body", "age"],
      "msg": "Value must be >= 0, got -5"
    },
    {
      "type": "string_format",
      "loc": ["body", "email"],
      "msg": "Invalid email format"
    }
  ]
}
```

For parameters:

```json
{
  "detail": [
    {
      "type": "type_error",
      "loc": ["path", "user_id"],
      "msg": "Value must be an integer, got 'abc'"
    },
    {
      "type": "missing",
      "loc": ["query", "page"],
      "msg": "Required query parameter 'page' is missing"
    }
  ]
}
```

## Performance Characteristics

- **Schema compilation**: Once at route registration (~1ms per schema)
- **Validation**: ~500k validations/sec on commodity hardware
- **JSON parsing**: SIMD-accelerated via `serde_json`
- **Zero-copy**: Request body bytes stay in Rust until validated
- **Streaming**: Large payloads can be validated in chunks

## Benefits

✅ **Leverage existing ecosystems** - Pydantic, msgspec, Zod, etc.
✅ **Zero learning curve** - Use tools developers already know
✅ **Maximum performance** - All validation in Rust
✅ **Type safety** - Full IDE support and type checking
✅ **OpenAPI generation** - JSON Schema → OpenAPI schemas
✅ **Flexible** - Support any library that outputs JSON Schema
✅ **Clear errors** - Detailed validation error messages

## Why Not Reinvent Validation?

**Pydantic-core** is already extremely fast (~100k validations/sec in pure Python). By moving to Rust with `jsonschema-rs`, we get:

1. **5x faster validation** - Rust + SIMD optimizations
2. **Zero GIL contention** - Validation happens outside Python
3. **Zero-copy parsing** - No duplicate JSON parsing
4. **Streaming support** - Handle large payloads efficiently

**But we reuse:**
- Pydantic's schema generation
- Zod's schema generation
- Existing type systems developers know

This is the **pragmatic approach** - leverage existing tools, optimize the critical path.
