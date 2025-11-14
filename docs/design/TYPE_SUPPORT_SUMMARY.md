# Type Support Summary - WebSocket & SSE

**Date:** November 14, 2025
**Status:** Python ✅ | Node.js 🟡 | Ruby 🟡

## Overview

This document outlines type system support for WebSocket and SSE handlers across Python, Node.js, and Ruby bindings.

## Python - ✅ **COMPLETE**

**Schema Extraction:** `packages/python/spikard/schema.py::extract_json_schema()`

### Supported Type Systems

1. **TypedDict** - Built-in Python typing
   ```python
   from typing import TypedDict

   class ChatMessage(TypedDict):
       text: str
       user: str

   @websocket("/chat")
   async def handler(message: ChatMessage) -> dict:
       return {"echo": message["text"]}
   ```

2. **dataclass** - Standard library
   ```python
   from dataclasses import dataclass

   @dataclass
   class ChatMessage:
       text: str
       user: str

   @websocket("/chat")
   async def handler(message: ChatMessage) -> dict:
       return {"echo": message.text}
   ```

3. **NamedTuple** - Standard library
   ```python
   from typing import NamedTuple

   class ChatMessage(NamedTuple):
       text: str
       user: str

   @websocket("/chat")
   async def handler(message: ChatMessage) -> dict:
       return {"echo": message.text}
   ```

4. **Pydantic v1/v2** - Popular validation library
   ```python
   from pydantic import BaseModel

   class ChatMessage(BaseModel):
       text: str
       user: str

   @websocket("/chat")
   async def handler(message: ChatMessage) -> dict:
       return {"echo": message.text}
   ```

5. **msgspec.Struct** - High-performance serialization
   ```python
   import msgspec

   class ChatMessage(msgspec.Struct):
       text: str
       user: str

   @websocket("/chat")
   async def handler(message: ChatMessage) -> dict:
       return {"echo": message.text}
   ```

6. **Plain JSON Schema** - Direct specification
   ```python
   message_schema = {
       "type": "object",
       "properties": {"text": {"type": "string"}},
       "required": ["text"]
   }

   @websocket("/chat", message_schema=message_schema)
   async def handler(message: dict) -> dict:
       return {"echo": message["text"]}
   ```

### Testing Requirements

**Current State:** Tests use plain `dict` types
**Required:** Tests for each type system

- [ ] WebSocket with TypedDict
- [ ] WebSocket with dataclass
- [ ] WebSocket with NamedTuple
- [ ] WebSocket with Pydantic
- [ ] WebSocket with msgspec.Struct
- [ ] SSE with TypedDict
- [ ] SSE with dataclass
- [ ] SSE with NamedTuple
- [ ] SSE with Pydantic
- [ ] SSE with msgspec.Struct

## Node.js - 🟡 **NEEDS ENHANCEMENT**

**Schema Extraction:** Manual - schemas passed as properties

### Currently Supported

1. **Plain JSON Schema** - Direct specification
   ```typescript
   const messageSchema = {
       type: "object",
       properties: { text: { type: "string" } },
       required: ["text"]
   };
   ```

### Needs Implementation

1. **Zod** - TypeScript-first schema validation
   ```typescript
   import { z } from 'zod';

   const ChatMessage = z.object({
       text: z.string(),
       user: z.string()
   });
   type ChatMessage = z.infer<typeof ChatMessage>;

   // Should auto-extract schema from Zod
   ```

2. **TypeBox** - JSON Schema Type Builder
   ```typescript
   import { Type } from '@sinclair/typebox';

   const ChatMessage = Type.Object({
       text: Type.String(),
       user: Type.String()
   });
   ```

3. **io-ts** - Runtime type checking
   ```typescript
   import * as t from 'io-ts';

   const ChatMessage = t.type({
       text: t.string,
       user: t.string
   });
   ```

4. **AJV** - JSON Schema validator
   ```typescript
   // Use existing JSON Schema with AJV
   ```

5. **Yup** - Schema validation
   ```typescript
   import * as yup from 'yup';

   const chatMessage = yup.object({
       text: yup.string().required(),
       user: yup.string().required()
   });
   ```

### Testing Requirements

- [ ] WebSocket with Zod
- [ ] WebSocket with TypeBox
- [ ] WebSocket with io-ts
- [ ] SSE with Zod
- [ ] SSE with TypeBox
- [ ] SSE with io-ts

## Ruby - 🟡 **RECOMMENDED: Dry-Schema**

**Schema Extraction:** Manual - schemas passed as instance variables
**Recommendation:** Support **Dry-Schema** as primary type system

### Research Summary

After evaluating Ruby type systems for JSON Schema integration:

| Type System | JSON Schema Support | Implementation Effort | Ecosystem | Status |
|-------------|-------------------|---------------------|-----------|---------|
| **Dry-Schema** ✅ | Native via `:json_schema` extension | **LOW** | Modern, fast | **IMPLEMENT** |
| **Dry-Types** 🟡 | Via Dry-Schema integration | **MEDIUM** | Modern, fast | Optional |
| **Plain JSON Schema** ✅ | Direct specification | **ZERO** | Universal | **SUPPORTED** |

### RECOMMENDED: Dry-Schema

**Why Dry-Schema:**
- Native JSON Schema conversion via `:json_schema` extension
- Low implementation effort (just call `.json_schema` method)
- Modern, performant, actively maintained (v1.14.1 as of 2025)
- Part of dry-rb ecosystem (widely used)
- Multiple times faster than ActiveModel validations

**Implementation Example:**
```ruby
require 'dry-schema'

# User writes validation schema
Dry::Schema.load_extensions(:json_schema)

ChatMessageSchema = Dry::Schema.JSON do
  required(:text).filled(:str?, min_size?: 1)
  required(:user).filled(:str?)
end

# We extract JSON Schema
schema_hash = ChatMessageSchema.json_schema
# Returns valid JSON Schema hash ready for Rust validation
```


### Currently Supported

1. **Plain JSON Schema** - Direct specification (Hash)
   ```ruby
   message_schema = {
       "type" => "object",
       "properties" => { "text" => { "type" => "string" } },
       "required" => ["text"]
   }
   ```

### Implementation Plan: Dry-Schema Support

1. **Dry-Schema** - RECOMMENDED (Native JSON Schema support)
   ```ruby
   require 'dry-schema'

   Dry::Schema.load_extensions(:json_schema)

   ChatMessageSchema = Dry::Schema.JSON do
     required(:text).filled(:str?)
     required(:user).filled(:str?)
   end

   @websocket("/chat", message_schema: ChatMessageSchema.json_schema)
   def handler(message)
     { echo: message[:text] }
   end
   ```

2. **Dry-Types** - OPTIONAL (Via Dry-Schema)
   ```ruby
   require 'dry-types'
   require 'dry-schema'

   module Types
     include Dry.Types()
   end

   class ChatMessage < Dry::Struct
     attribute :text, Types::String
     attribute :user, Types::String
   end

   # Convert via Dry-Schema
   schema = Dry::Schema.JSON { required(:text).filled(:str?); required(:user).filled(:str?) }
   ```

### Testing Requirements

- [ ] Implement Dry-Schema integration in `packages/ruby/lib/spikard/schema.rb`
- [ ] WebSocket with Dry-Schema
- [ ] WebSocket with Plain JSON Schema
- [ ] SSE with Dry-Schema
- [ ] SSE with Plain JSON Schema
- [ ] Document Dry-Schema usage patterns

## Implementation Priority

### Phase 1: Python Tests (Highest Priority)
**Why:** Python already has full schema support, just needs comprehensive tests

1. Create WebSocket test suite with all 6 type systems
2. Create SSE test suite with all 6 type systems
3. Verify schema extraction works correctly
4. Test validation errors for each type system

### Phase 2: Node.js Schema Extraction (High Priority)
**Why:** TypeScript ecosystem heavily uses Zod and TypeBox

1. Implement Zod schema extraction
2. Implement TypeBox schema extraction
3. Add type system detection in Node bindings
4. Create comprehensive test suite
5. Document usage patterns

### Phase 3: Ruby Type Support (Medium Priority)
**Why:** Ruby ecosystem needs research on preferred patterns

1. Research Ruby type system preferences
2. Choose primary type systems (Dry-Types? Dry-Schema?)
3. Implement schema extraction helper
4. Create comprehensive test suite
5. Document patterns

## Schema Extraction Architecture

### Current Flow

```
┌─────────────────┐
│  Type Hint      │ → TypedDict, Pydantic, msgspec, etc.
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│  extract_json   │ → Python: schema.py::extract_json_schema()
│  _schema()      │ → Node: Manual (needs implementation)
└────────┬────────┘ → Ruby: Manual (needs implementation)
         │
         ↓
┌─────────────────┐
│  JSON Schema    │ → Standard JSON Schema dict
│  Dict           │
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│  Rust Layer     │ → jsonschema::validator_for()
│  Validation     │ → Validates at runtime
└─────────────────┘
```

### Required Changes for Node.js

1. Create `packages/node/src/schema.ts` with:
   - `extractJsonSchema(type)` function
   - Zod schema extraction
   - TypeBox schema extraction
   - io-ts schema extraction

2. Update `crates/spikard-node/src/websocket.rs`:
   - Check for `_messageSchema` and `_responseSchema`
   - Already implemented ✅

3. Update `crates/spikard-node/src/sse.rs`:
   - Check for `_eventSchema`
   - Already implemented ✅

### Required Changes for Ruby

1. Create `packages/ruby/lib/spikard/schema.rb` with:
   - `extract_json_schema(type)` function
   - Dry-Types extraction (if chosen)
   - Dry-Schema extraction (if chosen)

2. Update Ruby decorators to call schema extraction

3. Bindings already support schema passing ✅

## Next Steps

1. **Immediate:** Create Python test suite with all type systems
2. **Next:** Research and implement Node.js Zod integration
3. **Then:** Research Ruby type system preferences
4. **Finally:** Document best practices for each language

## References

- Python schema extraction: `packages/python/spikard/schema.py`
- Node.js WebSocket binding: `crates/spikard-node/src/websocket.rs`
- Node.js SSE binding: `crates/spikard-node/src/sse.rs`
- Ruby WebSocket binding: `crates/spikard-rb/src/websocket.rs`
- Ruby SSE binding: `crates/spikard-rb/src/sse.rs`
