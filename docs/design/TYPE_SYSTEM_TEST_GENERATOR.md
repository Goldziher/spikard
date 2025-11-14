# Type System Test Generator Architecture

**Date:** November 14, 2025
**Status:** DESIGN PHASE
**Goal:** Comprehensive type system testing across entire API surface for all languages

## Overview

Current fixture-based test generator only tests plain `dict`/`Hash`/object types. We need parameterized tests that cover **all supported type systems** across **all API surfaces** (HTTP request/response bodies, WebSocket messages, SSE events) for **all languages** (Python, Node.js, Ruby, Rust).

## Current State

### What Works
- ✅ Python HTTP handlers with plain `dict`
- ✅ Python WebSocket/SSE with plain `dict`
- ✅ Python schema extraction from 6 type systems (TypedDict, dataclass, NamedTuple, Pydantic, msgspec, JSON Schema)
- ✅ Ruby schema extraction for Dry-Schema and Dry-Types
- ✅ Manual type system tests created

### What's Missing
- ❌ Parameterized test generation for all type systems
- ❌ HTTP POST/PUT/PATCH request body type system tests
- ❌ HTTP response body type system tests
- ❌ Node.js schema extraction (Zod, TypeBox, io-ts)
- ❌ Test generator producing type-system-aware tests
- ❌ Fixture metadata specifying type system variants

## API Surface Coverage

### 1. HTTP Request Bodies
**Routes:** POST, PUT, PATCH endpoints
**Validation Point:** Request body JSON → Typed object
**Type Systems:**
- Python: TypedDict, dataclass, NamedTuple, Pydantic, msgspec, JSON Schema
- Node.js: Zod, TypeBox, io-ts, JSON Schema
- Ruby: Dry-Schema, Dry-Types, JSON Schema
- Rust: serde types

**Example:**
```python
# Python with Pydantic
class UserCreate(BaseModel):
    email: str
    age: int

@app.post("/users", request_schema=UserCreate)
async def create_user(user: UserCreate) -> dict:
    return {"id": 123, "email": user.email}
```

### 2. HTTP Response Bodies
**Routes:** All HTTP endpoints
**Validation Point:** Handler return → JSON validation
**Type Systems:** Same as request bodies

**Example:**
```python
class UserResponse(BaseModel):
    id: int
    email: str
    created_at: str

@app.post("/users", response_schema=UserResponse)
async def create_user(data: dict) -> UserResponse:
    return UserResponse(id=123, email=data["email"], created_at="2025-01-01")
```

### 3. WebSocket Messages
**Routes:** WebSocket endpoints
**Validation Points:**
- Incoming message JSON → Typed object
- Outgoing response object → JSON validation

**Example:**
```python
class ChatMessage(TypedDict):
    user: str
    text: str

@websocket("/chat")
async def handler(message: ChatMessage) -> dict:
    return {"echo": message["text"]}
```

### 4. SSE Events
**Routes:** SSE endpoints
**Validation Point:** Event data → JSON validation
**Type Systems:** Same as above

**Example:**
```python
class StatusEvent(msgspec.Struct):
    status: str
    timestamp: int

@sse("/status")
async def handler() -> AsyncIterator[StatusEvent]:
    yield StatusEvent(status="ok", timestamp=123)
```

## Test Generator Architecture

### Fixture Structure Enhancement

**Current:**
```json
{
  "name": "Create User",
  "handler": {
    "route": "/users",
    "method": "POST"
  },
  "request_body": {
    "email": "test@example.com",
    "age": 25
  },
  "expected_response": {
    "status_code": 201,
    "body": {"id": 123}
  }
}
```

**Enhanced:**
```json
{
  "name": "Create User",
  "handler": {
    "route": "/users",
    "method": "POST",
    "request_schema": {
      "type": "object",
      "properties": {
        "email": {"type": "string", "format": "email"},
        "age": {"type": "integer", "minimum": 0}
      },
      "required": ["email", "age"]
    },
    "response_schema": {
      "type": "object",
      "properties": {
        "id": {"type": "integer"},
        "email": {"type": "string"}
      },
      "required": ["id", "email"]
    }
  },
  "request_body": {
    "email": "test@example.com",
    "age": 25
  },
  "expected_response": {
    "status_code": 201,
    "body": {"id": 123, "email": "test@example.com"}
  }
}
```

### Code Generation Strategy

#### Python Test Generator

**File:** `tools/test-generator/src/python_tests.rs`

Generate parameterized tests using `pytest.mark.parametrize`:

```python
import pytest
from typing import TypedDict
from dataclasses import dataclass
from pydantic import BaseModel
import msgspec

# Define all type system variants for the schema
class UserCreateTypedDict(TypedDict):
    email: str
    age: int

@dataclass
class UserCreateDataclass:
    email: str
    age: int

class UserCreatePydantic(BaseModel):
    email: str
    age: int

class UserCreateMsgspec(msgspec.Struct):
    email: str
    age: int

# Parameterized test
@pytest.mark.parametrize("type_system,schema_class", [
    ("typeddict", UserCreateTypedDict),
    ("dataclass", UserCreateDataclass),
    ("pydantic", UserCreatePydantic),
    ("msgspec", UserCreateMsgspec),
    ("json_schema", None),  # Uses plain dict + JSON Schema
])
async def test_create_user(type_system, schema_class):
    """Test POST /users with {type_system} type system."""
    app = create_app_with_type_system(type_system, schema_class)
    client = TestClient(app)

    response = await client.post("/users", json={
        "email": "test@example.com",
        "age": 25
    })

    assert response.status_code == 201
    assert response.json()["email"] == "test@example.com"
```

#### Node.js Test Generator

**File:** `tools/test-generator/src/node_tests.rs`

Generate tests with Zod/TypeBox variants:

```typescript
import { z } from 'zod';
import { Type } from '@sinclair/typebox';
import * as t from 'io-ts';

describe.each([
  ['zod', z.object({ email: z.string().email(), age: z.number().int().min(0) })],
  ['typebox', Type.Object({ email: Type.String({ format: 'email' }), age: Type.Integer({ minimum: 0 }) })],
  ['io-ts', t.type({ email: t.string, age: t.number })],
  ['json-schema', { type: 'object', properties: { email: { type: 'string' }, age: { type: 'integer' } } }],
])('POST /users with %s', (typeSystem, schema) => {
  it('creates user with validation', async () => {
    const app = createAppWithTypeSystem(typeSystem, schema);
    const client = new TestClient(app);

    const response = await client.post('/users', {
      body: { email: 'test@example.com', age: 25 }
    });

    expect(response.status).toBe(201);
    expect(response.body.email).toBe('test@example.com');
  });
});
```

#### Ruby Test Generator

**File:** `tools/test-generator/src/ruby_tests.rs`

Generate RSpec tests with Dry-Schema/Dry-Types variants:

```ruby
require 'dry-schema'
require 'dry-types'

Dry::Schema.load_extensions(:json_schema)

RSpec.describe 'POST /users' do
  let(:type_systems) do
    {
      'dry-schema' => Dry::Schema.JSON do
        required(:email).filled(:str?)
        required(:age).filled(:int?, gt?: 0)
      end,
      'dry-types' => UserCreateStruct,  # Dry::Struct defined elsewhere
      'json-schema' => {
        'type' => 'object',
        'properties' => {
          'email' => { 'type' => 'string' },
          'age' => { 'type' => 'integer' }
        }
      }
    }
  end

  type_systems.each do |type_system, schema|
    context "with #{type_system}" do
      let(:app) { create_app_with_type_system(type_system, schema) }
      let(:client) { Spikard::TestClient.new(app) }

      it 'creates user with validation' do
        response = client.post('/users', json: {
          'email' => 'test@example.com',
          'age' => 25
        })

        expect(response.status).to eq(201)
        expect(response.body['email']).to eq('test@example.com')
      end
    end
  end
end
```

### Test Generator Implementation

**Key Changes to `tools/test-generator/`:**

1. **Add schema metadata to fixtures:**
   - Parse `request_schema` and `response_schema` from fixtures
   - Generate type definitions for each supported type system

2. **Generate type definitions:**
   - Python: TypedDict, dataclass, NamedTuple, Pydantic, msgspec
   - Node.js: Zod, TypeBox, io-ts schemas
   - Ruby: Dry-Schema, Dry-Types definitions

3. **Generate parameterized tests:**
   - Use `pytest.mark.parametrize` for Python
   - Use `describe.each` for Node.js/Jest
   - Use RSpec contexts for Ruby

4. **Generate app factory functions:**
   - Create separate app instances for each type system variant
   - Register handlers with appropriate schema extraction

## Implementation Plan

### Phase 1: Fixture Enhancement ✅ READY
- Add `request_schema` and `response_schema` to fixture format
- Update existing fixtures with schema definitions
- Document fixture schema format

### Phase 2: Python Implementation
1. Update `tools/test-generator/src/python_tests.rs`:
   - Generate type definitions from JSON Schema
   - Generate parameterized test functions
   - Generate app factory with type system variants

2. Test coverage:
   - HTTP POST/PUT/PATCH with all 6 type systems
   - HTTP response validation with all 6 type systems
   - WebSocket with all 6 type systems ✅ DONE MANUALLY
   - SSE with all 6 type systems ✅ DONE MANUALLY

### Phase 3: Node.js Implementation
1. Implement Node.js schema extraction:
   - Create `packages/node/src/schema.ts`
   - Add Zod schema → JSON Schema conversion
   - Add TypeBox schema → JSON Schema conversion
   - Add io-ts schema → JSON Schema conversion

2. Update `tools/test-generator/src/node_tests.rs`:
   - Generate Zod/TypeBox/io-ts schemas
   - Generate parameterized tests
   - Generate app factory with type system variants

### Phase 4: Ruby Implementation
1. Update `tools/test-generator/src/ruby_tests.rs`:
   - Generate Dry-Schema definitions ✅ DONE
   - Generate Dry-Types class definitions
   - Generate RSpec parameterized tests
   - Generate app factory with type system variants

### Phase 5: Rust Native Implementation
1. Generate native Rust handlers with serde types
2. Test type safety at compile time

## Success Criteria

- [ ] All fixtures have `request_schema` and `response_schema` metadata
- [ ] Python: HTTP handlers tested with all 6 type systems
- [ ] Python: WebSocket handlers tested with all 6 type systems
- [ ] Python: SSE handlers tested with all 6 type systems
- [ ] Node.js: HTTP handlers tested with Zod, TypeBox, io-ts, JSON Schema
- [ ] Node.js: WebSocket handlers tested with all supported types
- [ ] Node.js: SSE handlers tested with all supported types
- [ ] Ruby: HTTP handlers tested with Dry-Schema, Dry-Types, JSON Schema
- [ ] Ruby: WebSocket handlers tested with all supported types
- [ ] Ruby: SSE handlers tested with all supported types
- [ ] All tests pass with proper validation errors for invalid data
- [ ] Documentation updated with type system usage examples

## Benefits

1. **Comprehensive Coverage:** Every API surface tested with every type system
2. **Confidence:** Type marshaling works correctly across all languages
3. **Regression Prevention:** Changes to schema extraction caught immediately
4. **Documentation:** Generated tests serve as usage examples
5. **Developer Experience:** Users can choose their preferred type system with confidence

## References

- Type Support Summary: `docs/design/TYPE_SUPPORT_SUMMARY.md`
- Python schema extraction: `packages/python/spikard/schema.py`
- Ruby schema extraction: `packages/ruby/lib/spikard/schema.rb`
- Node.js schema extraction: `packages/node/src/schema.ts` (TO BE CREATED)
- Test generator: `tools/test-generator/src/`
