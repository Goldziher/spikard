# Validation Architecture: JSON Schema vs Garde/Validator

**Date:** 2025-10-31
**Status:** Analysis
**Context:** Understanding interop between garde/validator and our JSON Schema-based API

## Executive Summary

**TL;DR:** Garde/validator and JSON Schema serve **different use cases** and are **complementary, not competitive**. Our current JSON Schema approach is correct for a **schema-driven API framework**. Garde/validator would only be useful for **internal Rust type safety**, not for validating external requests.

## Current Architecture

### How Spikard Works Today

```rust
// User provides JSON Schema at runtime (from fixtures, OpenAPI, etc.)
let schema = json!({
    "type": "object",
    "properties": {
        "email": {"type": "string", "format": "email"},
        "age": {"type": "integer", "minimum": 18}
    },
    "required": ["email"]
});

// We compile it once
let validator = SchemaValidator::new(schema)?;

// Then validate any incoming JSON data
let result = validator.validate(&incoming_data);
```

**Key characteristics:**
- ✅ **Dynamic:** Schemas provided at runtime (not compile-time)
- ✅ **Language-agnostic:** JSON Schema works across Python, Node, Rust, WASM
- ✅ **Framework-level:** Validates external data before it reaches user code
- ✅ **Industry standard:** JSON Schema is widely understood and tooled

**Used in:**
- `SchemaValidator` - validates request/response bodies
- `ParameterValidator` - validates query params, path params, cookies, headers
- Generated handlers in test suite
- Python bindings (`spikard-py`)
- Node bindings (`spikard-node`)

## What Are Garde/Validator?

### Garde (Modern, 2023+)

```rust
use garde::Validate;

#[derive(Validate)]
struct User {
    #[garde(email)]
    email: String,

    #[garde(range(min = 18))]
    age: u8,
}

// Validates at compile-time defined types
let user = User { email: "test".to_string(), age: 15 };
user.validate()?; // Returns validation errors
```

**Characteristics:**
- ✅ **Compile-time:** Validation rules defined in Rust structs via derive macros
- ✅ **Type-safe:** Leverages Rust's type system
- ✅ **Ergonomic:** Nice Rust API with good error messages
- ❌ **Static only:** Can't validate arbitrary JSON against runtime schemas
- ❌ **Rust-specific:** Doesn't work in Python/Node bindings

### Validator (Older, 2017+)

Similar to garde but older API:

```rust
use validator::Validate;

#[derive(Validate)]
struct User {
    #[validate(email)]
    email: String,

    #[validate(range(min = 18))]
    age: u8,
}
```

## The Fundamental Difference

### JSON Schema (What We Use)

**Use case:** "I have a JSON Schema (from OpenAPI, fixtures, user config) and need to validate arbitrary JSON data against it at runtime"

```rust
// Runtime schema (from file, API, user input)
let schema: Value = load_schema_from_somewhere();
let validator = jsonschema::validator_for(&schema)?;

// Runtime data (from HTTP request, file, etc.)
let data: Value = parse_incoming_request();

// Validate any shape against any schema
validator.validate(&data)?;
```

**Example:** Spikard validates HTTP requests using schemas defined in:
- Fixture files (`testing_data/*/`)
- OpenAPI specifications
- User-provided handler definitions

### Garde/Validator (Rust-internal validation)

**Use case:** "I have a known Rust struct at compile-time and want to validate instances of it"

```rust
// Compile-time type
#[derive(Validate)]
struct User {
    #[garde(email)]
    email: String,
}

// Only works with this specific type
let user = User { email: "invalid" };
user.validate()?;
```

**Example:** Django's model validators, FastAPI's Pydantic models

## Can They Work Together?

### Scenario 1: Using Garde for Internal Types ✅

```rust
// Internal Rust types could use garde
#[derive(Garde, Serialize, Deserialize)]
struct InternalConfig {
    #[garde(url)]
    api_endpoint: String,

    #[garde(range(min = 1, max = 100))]
    timeout_seconds: u32,
}

// But HTTP requests still use JSON Schema
let schema = json!({...}); // From OpenAPI/fixtures
let validator = SchemaValidator::new(schema)?;
```

**Works because:**
- Garde validates internal Rust structs (config, database models)
- JSON Schema validates external data (HTTP requests)
- They operate at different layers

### Scenario 2: Generating JSON Schema from Garde ❌

Some libraries (like `schemars`) can generate JSON Schema from Rust types:

```rust
#[derive(JsonSchema, Garde)]
struct User {
    #[garde(email)]
    #[schemars(regex = "email_regex")]
    email: String,
}

// Generate JSON Schema from the type
let schema = schemars::schema_for!(User);
```

**This DOESN'T help Spikard because:**
- ❌ We need **dynamic** schemas (from fixtures, OpenAPI, runtime config)
- ❌ We'd need to define **every possible request type** at compile-time
- ❌ Breaks our **language-agnostic** design (Python/Node can't use Rust types)
- ❌ Couples validation to Rust types instead of JSON Schema standard

## Why JSON Schema is Correct for Spikard

### 1. Dynamic Schema Requirements

Spikard is a **testing framework** where schemas come from:
- Test fixtures (`testing_data/json_bodies/*.json`)
- OpenAPI specifications
- User-provided handler definitions
- Runtime configuration

```rust
// User provides schema at runtime
let schema: Value = serde_json::from_str(&fixture_json)?;
let validator = SchemaValidator::new(schema)?;
```

**With garde/validator, you'd need:**
```rust
// ❌ Can't do this - must be compile-time!
#[derive(Validate)]
struct DynamicRequest {
    // How do we know what fields exist?
    // How do we know their validation rules?
    // Different for each test fixture!
}
```

### 2. Language Agnostic

Spikard has bindings for:
- Python (`crates/spikard-py`)
- Node.js (`crates/spikard-node`)
- WASM (`crates/spikard-wasm`)

JSON Schema works everywhere:
```python
# Python
validator = SchemaValidator({"type": "object", ...})
validator.validate(data)
```

```javascript
// Node.js
const validator = new SchemaValidator({type: "object", ...});
validator.validate(data);
```

**Garde/validator only works in Rust** - can't cross FFI boundaries.

### 3. Industry Standard

JSON Schema is understood by:
- OpenAPI/Swagger
- FastAPI (Python)
- NestJS (TypeScript)
- Ajv (JavaScript)
- Every major API toolchain

**Garde/validator are Rust-specific** - no interop with other ecosystems.

### 4. Framework vs Application Code

```
┌─────────────────────────────────┐
│  User Application Code          │
│  (Python/Node/Rust)             │
│  - Could use garde/validator    │  ← Garde useful here
│    for internal types           │
└─────────────────────────────────┘
         ↓ HTTP Request
┌─────────────────────────────────┐
│  Spikard HTTP Layer             │
│  - Validates using JSON Schema  │  ← JSON Schema correct here
│  - SchemaValidator              │
│  - ParameterValidator           │
└─────────────────────────────────┘
```

## When Would Garde/Validator Help?

### Valid Use Cases Within Spikard

1. **Internal Configuration Validation**
```rust
// crates/spikard-http/src/server.rs
#[derive(Garde, Deserialize)]
struct ServerConfig {
    #[garde(range(min = 1, max = 65535))]
    port: u16,

    #[garde(custom = validate_path)]
    static_dir: PathBuf,
}
```

2. **CLI Argument Validation**
```rust
// crates/spikard-cli/src/main.rs
#[derive(Garde, Parser)]
struct Args {
    #[garde(url)]
    #[arg(long)]
    api_url: String,

    #[garde(range(min = 1))]
    #[arg(long, default_value = "8080")]
    port: u16,
}
```

3. **Database Models** (if we add ORM)
```rust
#[derive(Garde, sqlx::FromRow)]
struct User {
    #[garde(email)]
    email: String,

    #[garde(length(min = 8))]
    password_hash: String,
}
```

### Where It Does NOT Help

❌ **HTTP Request Validation** - We need dynamic JSON Schema
❌ **OpenAPI Support** - We need runtime schema loading
❌ **Test Fixtures** - We need arbitrary schema shapes
❌ **Python/Node Bindings** - We need language-agnostic validation
❌ **Parameter Validation** - We need runtime schema compilation

## Comparison Matrix

| Feature | JSON Schema (Current) | Garde/Validator |
|---------|----------------------|------------------|
| Dynamic schemas | ✅ Yes | ❌ No (compile-time only) |
| Runtime validation | ✅ Yes | ❌ No (type must exist) |
| Language agnostic | ✅ Yes | ❌ No (Rust only) |
| OpenAPI compatible | ✅ Yes | ⚠️  Via schemars generation |
| Type safety | ⚠️  JSON only | ✅ Full Rust types |
| Error messages | ✅ Good | ✅ Excellent |
| Performance | ✅ Good (compiled) | ✅ Excellent (compile-time) |
| Test fixtures | ✅ Perfect fit | ❌ Can't express arbitrary schemas |
| FFI support | ✅ Works via PyO3/napi | ❌ Rust-only traits |

## Recommendation

### Keep JSON Schema for Core Validation ✅

**Why:**
- It's the right tool for dynamic, runtime schema validation
- Essential for our test fixture architecture
- Required for language-agnostic bindings
- Industry standard for HTTP APIs

**Current usage is correct:**
- `SchemaValidator` - validates request/response bodies
- `ParameterValidator` - validates params/cookies/headers
- Test generator - creates validators from fixture schemas

### Consider Garde for Internal Types (Low Priority)

**Potential additions:**
```toml
# crates/spikard-http/Cargo.toml
[dependencies]
garde = { version = "0.20", optional = true }

[features]
default = ["validation"]
validation = []
internal-validation = ["garde"]  # Optional, for internal use
```

**Use for:**
- Server configuration validation
- CLI argument validation
- Internal Rust data structures
- Database models (if added)

**Do NOT use for:**
- HTTP request validation ❌
- Response validation ❌
- Parameter validation ❌
- Anything that needs dynamic schemas ❌

## Conclusion

**Garde/validator and JSON Schema are complementary tools for different use cases:**

- **JSON Schema:** Validates external data against runtime-provided schemas (HTTP requests, API responses, test fixtures)
- **Garde/validator:** Validates internal Rust types with compile-time guarantees (config, models, arguments)

**For Spikard's core mission** (HTTP validation framework with test fixtures), **JSON Schema is the correct choice** and we should continue using it.

**Garde/validator could be useful** for internal Rust type safety, but it's a **low priority enhancement** that wouldn't improve our main validation pipeline.

## References

- [jsonschema crate](https://docs.rs/jsonschema) - Our current validator
- [garde crate](https://docs.rs/garde) - Modern Rust validation
- [validator crate](https://docs.rs/validator) - Older Rust validation
- [schemars](https://docs.rs/schemars) - Generate JSON Schema from Rust types
- [JSON Schema Draft 2020-12](https://json-schema.org/draft/2020-12/json-schema-core)

## Decision

**Status:** ✅ **Use JSON Schema for all HTTP validation**

**Rationale:**
1. Dynamic schemas required for test fixtures
2. Language-agnostic validation needed for Python/Node bindings
3. OpenAPI/industry standard compatibility
4. Runtime schema compilation essential for framework design

**Future consideration:** Add garde for internal Rust type validation (low priority)
