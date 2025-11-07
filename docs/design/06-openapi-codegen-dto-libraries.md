# OpenAPI Code Generator: DTO Library Selection Strategy

**Date:** November 2025
**Status:** Active
**Related:** [02-unified-cli.md](./02-unified-cli.md), [codegen-strategy.md](./codegen-strategy.md), [03-python-type-systems.md](./03-python-type-systems.md)

## Executive Summary

This document defines the DTO library selection strategy for Spikard's OpenAPI code generator across all language bindings. We establish `msgspec.Struct` for Python, Zod v4 for TypeScript, `dry-struct` for Ruby, `spiral/json-schema-generator` for PHP, and native `serde` for Rust as the primary, optimized patterns for each language binding, prioritizing JSON Schema generation capabilities and type safety.

## 1. Overview

Spikard's OpenAPI code generator must produce idiomatic, type-safe code for each language binding. A critical decision is which DTO (Data Transfer Object) library to use as the "blessed" or default pattern for each language. These libraries must provide:

1. **Strong typing** with compile-time or runtime validation
2. **JSON Schema generation** from type definitions
3. **Performance** suitable for high-throughput HTTP services
4. **Ergonomics** matching language idioms and best practices
5. **Maintenance** with active development and community support

### Goals

- Select one primary DTO library per language as the default/recommended pattern
- Ensure all selected libraries support JSON Schema generation
- Establish consistent patterns across language bindings
- Enable automated generation of benchmark applications from a single OpenAPI schema
- Support future extensibility for alternative DTO libraries

### Non-Goals

- Supporting every possible DTO library in each language (initially)
- Creating custom validation frameworks
- Replacing existing hand-written Spikard applications
- Runtime serialization performance (handled by Rust core)

## 2. Background / Context

Spikard performs all serialization in the Rust core layer. Language bindings define route handlers and parameter extraction but delegate the actual HTTP parsing and JSON serialization to Rust. Therefore, we need libraries that:

- Define **schema/type information** (not serialization)
- Generate **JSON Schema** for documentation and validation
- Provide **type safety** for developer ergonomics
- Integrate well with **Spikard's parameter extraction** (Path, Query, Body)

### Research (2024-2025)

#### Python
- **msgspec** (2025): Fastest validation library, 10-80x faster than alternatives, built-in JSON Schema support, immutable structs
- **pydantic v2** (2025): Most popular, Rust-based core, extensive ecosystem, JSON Schema generation
- **dataclasses** (stdlib): No validation, no JSON Schema, but simple and ubiquitous

**Decision**: msgspec.Struct for performance and JSON Schema support

#### TypeScript
- **Zod v4** (2025): Industry standard for runtime validation, excellent TypeScript integration, JSON Schema via `zod-to-json-schema`
- **yup**: Older, less type-safe
- **joi**: Schema-first, verbose API
- **TypeBox**: JSON Schema first, less ergonomic

**Decision**: Zod v4 for type inference and ecosystem support

#### Ruby
- **dry-struct** (2025): Immutable typed objects from dry-rb ecosystem, integrates with `dry-types-json-schema` gem (v0.0.5, May 2024)
- **dry-schema**: More functional, less OOP
- **ActiveModel**: Rails-specific, heavy dependencies
- **Virtus**: Unmaintained since 2016

**Decision**: dry-struct + dry-types-json-schema for type safety and JSON Schema generation

#### PHP
- **spiral/json-schema-generator** (PHP 8.3+, 2025): Modern attributes-based approach, generates JSON Schema from typed properties, active development
- **psx/schema** (2025): More comprehensive, multi-language generation, TypeSchema support
- **symfony/serializer**: No type safety, no JSON Schema generation
- **JMS Serializer**: Deprecated approach, slow

**Decision**: spiral/json-schema-generator for modern PHP 8.3+ features and simplicity

#### Rust
- **serde** (ubiquitous): De facto standard, zero-cost, compile-time
- **schemars**: JSON Schema generation from serde types
- No alternatives seriously considered

**Decision**: Native serde + schemars

## 3. Design

### 3.1 Library Selection Matrix

| Language   | Primary Library                  | Version     | JSON Schema Support    | Status        |
|------------|----------------------------------|-------------|------------------------|---------------|
| Python     | `msgspec.Struct`                 | Latest      | ✅ Built-in            | Selected      |
| TypeScript | `zod`                            | v4.x        | ✅ Via zod-to-json-schema | Selected   |
| Ruby       | `dry-struct`                     | ~1.6        | ✅ Via dry-types-json-schema | Selected |
| PHP        | `spiral/json-schema-generator`   | Latest      | ✅ Built-in            | Selected      |
| Rust       | `serde` + `schemars`             | Latest      | ✅ Via schemars        | Selected      |

### 3.2 Python - msgspec.Struct

**Library**: `msgspec` (https://jcristharif.com/msgspec/)

**Rationale**:
- 10-80x faster than alternatives (including pydantic)
- Built-in JSON Schema generation
- Immutable struct types similar to Rust/dataclasses
- Minimal memory overhead
- Type-safe with Python 3.10+ union types

**Example**:
```python
import msgspec

class Pet(msgspec.Struct):
    id: int | None = None
    name: str
    tag: str | None = None

# JSON Schema generation
schema = msgspec.json.schema(Pet)
```

**Alternatives considered**:
- **pydantic v2**: More popular but slower, heavier runtime. Consider supporting as alternative in Phase 2.
- **dataclasses**: No validation, no JSON Schema

**Integration with Spikard**:
```python
from spikard import Spikard, route, Body, Path
import msgspec

app = Spikard()

class Pet(msgspec.Struct):
    name: str
    age: int

@route("/pets/{id}", methods=["PUT"])
def update_pet(id: Path[int], pet: Body[Pet]):
    # Rust validates JSON against Pet schema
    return {"id": id, "pet": pet}
```

### 3.3 TypeScript - Zod v4

**Library**: `zod` v4 (https://zod.dev/)

**Rationale**:
- Industry standard for TypeScript validation
- Excellent type inference (`z.infer<typeof Schema>`)
- Composable schema builders
- JSON Schema generation via `zod-to-json-schema`
- Large ecosystem and active development

**Example**:
```typescript
import { z } from 'zod';

const PetSchema = z.object({
  id: z.number().int().optional(),
  name: z.string(),
  tag: z.string().optional(),
});

type Pet = z.infer<typeof PetSchema>;

// JSON Schema generation
import { zodToJsonSchema } from 'zod-to-json-schema';
const jsonSchema = zodToJsonSchema(PetSchema);
```

**Alternatives considered**:
- **yup**: Less type-safe, older
- **joi**: More verbose, schema-first approach less ergonomic
- **TypeBox**: JSON Schema first, but less TypeScript-idiomatic

**Integration with Spikard**:
```typescript
import { Spikard, route, Body, Path } from '@spikard/node';
import { z } from 'zod';

const app = new Spikard();

const PetSchema = z.object({
  name: z.string(),
  age: z.number().int(),
});

app.route('/pets/:id', 'PUT', (req, id: Path<number>, pet: Body<z.infer<typeof PetSchema>>) => {
  // Rust validates JSON against PetSchema
  return { id, pet };
});
```

### 3.4 Ruby - dry-struct

**Library**: `dry-struct` + `dry-types-json-schema` (https://dry-rb.org/gems/dry-struct/)

**Rationale**:
- Immutable typed objects with compile-time guarantees
- Part of mature dry-rb ecosystem
- `dry-types-json-schema` gem generates JSON Schema from dry-types
- Functional programming approach aligns with Rust philosophy
- Active maintenance (last update March 2025)

**Example**:
```ruby
require 'dry-struct'
require 'dry-types-json-schema'

module Types
  include Dry.Types()
end

class Pet < Dry::Struct
  attribute :id, Types::Integer.optional
  attribute :name, Types::String
  attribute :tag, Types::String.optional
end

# JSON Schema generation
schema = Pet.to_json_schema
```

**Alternatives considered**:
- **dry-schema**: More functional but less OOP, harder to integrate
- **ActiveModel**: Rails-specific, heavy dependencies
- **Virtus**: Unmaintained

**Integration with Spikard**:
```ruby
require 'spikard'
require 'dry-struct'

app = Spikard::App.new

class Pet < Dry::Struct
  attribute :name, Types::String
  attribute :age, Types::Integer
end

app.route('/pets/:id', methods: [:put]) do |req, id:, pet:|
  # Rust validates JSON against Pet schema
  { id: id, pet: pet }
end
```

### 3.5 PHP - spiral/json-schema-generator

**Library**: `spiral/json-schema-generator` (https://github.com/spiral/json-schema-generator)

**Rationale**:
- Modern PHP 8.3+ approach using attributes
- Generates JSON Schema from typed DTO classes
- Supports union types, nullable types
- Lightweight and focused
- Active development (2025)

**Example**:
```php
<?php

use Spiral\JsonSchemaGenerator\Attribute\Field;
use Spiral\JsonSchemaGenerator\SchemaGenerator;

class Pet
{
    public function __construct(
        #[Field(description: "Pet ID")]
        public ?int $id = null,

        #[Field(description: "Pet name")]
        public string $name,

        #[Field(description: "Pet tag")]
        public ?string $tag = null,
    ) {}
}

// JSON Schema generation
$generator = new SchemaGenerator();
$schema = $generator->generate(Pet::class);
```

**Alternatives considered**:
- **psx/schema**: More features but more complex, overkill for most use cases
- **symfony/serializer**: No JSON Schema generation, no type safety
- **JMS Serializer**: Deprecated approach

**Integration with Spikard**:
```php
<?php

require 'vendor/autoload.php';

use Spikard\App;
use Spikard\Route;

$app = new App();

class Pet {
    public function __construct(
        public string $name,
        public int $age,
    ) {}
}

$app->route('/pets/{id}', 'PUT', function($req, int $id, Pet $pet) {
    // Rust validates JSON against Pet schema
    return ['id' => $id, 'pet' => $pet];
});
```

### 3.6 Rust - serde + schemars

**Library**: `serde` + `schemars` (https://serde.rs/, https://docs.rs/schemars/)

**Rationale**:
- serde is the de facto Rust standard
- Zero-cost abstractions
- Compile-time type safety
- schemars generates JSON Schema from serde types
- No alternatives seriously considered

**Example**:
```rust
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Pet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

// JSON Schema generation
let schema = schemars::schema_for!(Pet);
```

**Integration with Spikard**:
```rust
use spikard_http::{route, Path, Body};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Pet {
    name: String,
    age: i32,
}

#[route("/pets/{id}", method = "PUT")]
fn update_pet(id: Path<i64>, pet: Body<Pet>) -> Pet {
    pet
}
```

## 4. API Design

### 4.1 Code Generator CLI

```bash
# Generate with default library for each language
spikard generate api.yaml -l python -o server.py
spikard generate api.yaml -l typescript -o server.ts
spikard generate api.yaml -l ruby -o server.rb
spikard generate api.yaml -l php -o server.php
spikard generate api.yaml -l rust -o server.rs

# Future: Support alternative libraries
spikard generate api.yaml -l python --dto-library pydantic -o server.py
```

### 4.2 Generated Code Structure

All generators produce:
1. **DTO type definitions** using the selected library
2. **Route handlers** with type-safe parameter extraction
3. **JSON Schema metadata** (embedded or separate file)
4. **Imports/requires** for the DTO library
5. **Application bootstrap** code

## 5. Implementation Strategy

### Phase 0: Complete Ruby and PHP Bindings (5-10 days)

**Critical prerequisite**: We need working Ruby and PHP bindings before we can generate code for them.

#### Ruby Binding (spikard-rb) - 3-5 days
- [ ] Complete crates/spikard-rb implementation
  - [ ] Route registration and extraction
  - [ ] Parameter handling (Path, Query, Body)
  - [ ] Error handling aligned with Rust core
  - [ ] Integration with rb-sys
- [ ] Create packages/ruby/ with Ruby gem structure
- [ ] Write minimal test application
- [ ] Test with Spikard CLI `run` command
- [ ] Document Ruby API

#### PHP Binding (spikard-php) - 5-7 days
- [ ] Create crates/spikard-php (using FFI or php-ext-rs)
- [ ] Implement core binding features:
  - [ ] Route registration
  - [ ] Parameter extraction (Path, Query, Body)
  - [ ] JSON request/response handling
  - [ ] Error propagation from Rust
- [ ] Create packages/php/ with Composer package structure
- [ ] Write minimal test application
- [ ] Test with Spikard CLI `run` command
- [ ] Document PHP API

**Estimated Total**: 8-12 days for both bindings

**Note**: Existing Python (spikard-py) and Node.js (spikard-node) bindings are complete and can be used immediately.

### Phase 1: Update Python Generator (2-3 days)
- [x] Research complete
- [ ] Replace `dataclass` with `msgspec.Struct` in python.rs
- [ ] Update type mapping (use union types: `int | None`)
- [ ] Add JSON Schema generation to output
- [ ] Test with existing Python bindings

### Phase 2: Implement TypeScript Generator (3-4 days)
- [x] Research complete
- [ ] Create `typescript.rs` in codegen module
- [ ] Generate Zod schemas from OpenAPI
- [ ] Map types to Zod validators
- [ ] Add `zod-to-json-schema` for schema generation
- [ ] Test with Node.js bindings

### Phase 3: Implement Ruby Generator (3-4 days)
- [x] Research complete
- [ ] Validate dry-struct + dry-types-json-schema integration
- [ ] Create `ruby.rs` in codegen module
- [ ] Generate dry-struct classes
- [ ] Map types to dry-types
- [ ] Test JSON Schema generation

### Phase 4: Implement PHP Generator (3-4 days)
- [x] Research complete
- [ ] Validate spiral/json-schema-generator with PHP 8.3+
- [ ] Create `php.rs` in codegen module
- [ ] Generate typed DTO classes with attributes
- [ ] Test JSON Schema generation

### Phase 5: Implement Rust Generator (2-3 days)
- [ ] Create `rust.rs` in codegen module
- [ ] Generate serde structs
- [ ] Add schemars derives
- [ ] Test with spikard-http

### Phase 6: Comprehensive Test Schema (2-3 days)
- [ ] Design single OpenAPI schema covering all features
- [ ] Validate schema with openapiv3
- [ ] Generate code for all languages
- [ ] Verify JSON Schema output consistency

### Phase 7: Benchmark Applications (3-4 days)
- [ ] Generate benchmark apps from test schema
- [ ] Integrate with benchmark harness
- [ ] Run cross-language performance tests

## 6. Performance Considerations

| Language   | Library                | Validation Speed     | Memory Overhead | Schema Gen Speed |
|------------|------------------------|----------------------|-----------------|------------------|
| Python     | msgspec                | 10-80x faster        | Minimal         | Fast             |
| TypeScript | zod                    | Fast (runtime)       | Low             | Fast             |
| Ruby       | dry-struct             | Medium               | Low             | Medium           |
| PHP        | spiral                 | Fast (PHP 8.3+)      | Low             | Fast             |
| Rust       | serde                  | Zero-cost            | None            | Compile-time     |

**Note**: Spikard handles actual serialization in Rust, so these metrics reflect type definition and schema generation only, not runtime serialization.

## 7. Testing Strategy

### Unit Tests
- Test type mapping for each OpenAPI type → DTO library type
- Test optional vs required fields
- Test nested objects and arrays
- Test JSON Schema generation output

### Integration Tests
- Generate code from minimal OpenAPI schema
- Compile/parse generated code
- Verify imports and dependencies
- Validate JSON Schema output

### Cross-Language Tests
- Same OpenAPI schema generates equivalent schemas in each language
- JSON Schema output is compatible across languages
- Validation rules work consistently

**Example Test**:
```rust
#[test]
fn test_python_msgspec_generation() {
    let schema = r#"
    components:
      schemas:
        Pet:
          type: object
          required: [name]
          properties:
            id: {type: integer}
            name: {type: string}
    "#;

    let generated = generate_python_from_openapi(schema);

    assert!(generated.contains("import msgspec"));
    assert!(generated.contains("class Pet(msgspec.Struct):"));
    assert!(generated.contains("name: str"));
    assert!(generated.contains("id: int | None = None"));
}
```

## 8. Migration Guide (if applicable)

### For Existing Applications

If you have hand-written Spikard applications:

**No migration required.** These library choices only affect code generated from OpenAPI schemas. Hand-written applications can use any DTO library or none at all.

### From Generated dataclasses (Current Python Generator)

If you already generated Python code with the current generator:

```python
# Old (dataclass)
from dataclasses import dataclass

@dataclass
class Pet:
    id: Optional[int] = None
    name: str

# New (msgspec)
import msgspec

class Pet(msgspec.Struct):
    id: int | None = None
    name: str
```

**Migration Steps**:
1. Re-generate code from OpenAPI schema
2. Update imports: `import msgspec` instead of `from dataclasses import dataclass`
3. No runtime behavior changes (Rust handles serialization)

## 9. Open Questions

- [x] Python: msgspec vs pydantic? **Decision: msgspec primary, pydantic future alternative**
- [x] Ruby: dry-struct vs dry-schema? **Decision: dry-struct for OOP consistency**
- [x] PHP: spiral vs psx/schema? **Decision: spiral for simplicity, evaluate psx later**
- [ ] Should we support multiple DTO libraries per language via `--dto-library` flag?
  - **Recommendation**: Yes, but implement primary libraries first
- [ ] How to handle library-specific features (e.g., pydantic validators)?
  - **Recommendation**: Document as "advanced usage", focus on common subset
- [ ] Should generated code include installation instructions / requirements files?
  - **Recommendation**: Yes, generate requirements.txt / package.json / etc.

## 10. References

### Specifications
- OpenAPI 3.0 Spec: https://spec.openapis.org/oas/v3.0.3
- JSON Schema Draft 2020-12: https://json-schema.org/draft/2020-12/

### Libraries/Crates

#### Python
- msgspec: https://jcristharif.com/msgspec/ (Primary)
- pydantic: https://docs.pydantic.dev/ (Alternative consideration)

#### TypeScript
- zod: https://zod.dev/ (Primary)
- zod-to-json-schema: https://github.com/StefanTerdell/zod-to-json-schema

#### Ruby
- dry-struct: https://dry-rb.org/gems/dry-struct/ (Primary)
- dry-types-json-schema: https://rubygems.org/gems/dry-types-json-schema (v0.0.5, May 2024)

#### PHP
- spiral/json-schema-generator: https://github.com/spiral/json-schema-generator (Primary, PHP 8.3+)
- psx/schema: https://phpsx.org/docs/components/schema (Alternative consideration)

#### Rust
- serde: https://serde.rs/
- schemars: https://docs.rs/schemars/

### Prior Art
- FastAPI: Uses pydantic models for request/response validation
- NestJS: Uses class-validator with decorators
- OpenAPI Generator: Generates code for 50+ languages but no validation library strategy
- swagger-codegen: Similar to OpenAPI Generator

### Related Documents
- [02-unified-cli.md](./02-unified-cli.md) - CLI architecture
- [codegen-strategy.md](./codegen-strategy.md) - General codegen approach
- [03-python-type-systems.md](./03-python-type-systems.md) - Python typing analysis

---

**Key Takeaway:** Spikard's OpenAPI code generator uses best-in-class DTO libraries for each language (msgspec for Python, Zod for TypeScript, dry-struct for Ruby, spiral for PHP, serde for Rust), all of which provide JSON Schema generation capabilities and strong typing, enabling consistent API contracts and automated benchmark generation across all language bindings.
