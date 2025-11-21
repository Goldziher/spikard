# Benchmark TODO - Proper Validation Testing

## CRITICAL ISSUE: Current Benchmarks Are Invalid

**Problem**: Current benchmarks don't test framework features properly:
- ❌ No proper DTO/schema validation (just passing `dict[str, Any]`)
- ❌ Not testing typed query parameters with validation
- ❌ Not actually parsing multipart files
- ❌ Not testing URL-encoded form parsing
- ❌ Not following framework best practices

**Impact**: Current benchmarks measure raw HTTP throughput, not realistic framework usage with validation overhead.

---

## Phase 1: Fix Existing Benchmarks (URGENT)

### 1.1 Spikard Python - CRITICAL ISSUE FOUND
**Status**: ⚠️  **BLOCKER**: msgspec.Struct return serialization is broken

**Problem Discovered (2025-11-21)**:
- Spikard Python bindings incorrectly serialize msgspec.Struct responses as `{}`
- Bug is in `crates/spikard-py/src/handler.rs` or response serialization path
- Test: `POST /json/small` with msgspec.Struct returns empty object

**ADR 0003 Clarification**:
- Validation happens at **Rust layer** (`crates/spikard-http`) using JSON Schema
- Python types are just **hints** - Rust does the actual validation
- Current benchmarks ARE testing validation (Rust side), just not Python typing

**Temporary Solution**:
- Use Python `dataclass` instead of msgspec.Struct for now
- Still provides typed DTOs and type hints
- Rust validation layer still exercises properly
- Benchmarks remain valid for measuring validation overhead

```python
from dataclasses import dataclass

@dataclass
class UserCreate:
    name: str
    email: str
    age: int

@post("/json/small")
async def handler(body: UserCreate) -> dict[str, Any]:
    # Rust validates against JSON Schema
    # Python provides typed input
    return {"name": body.name, "email": body.email, "age": body.age}
```

**Tasks**:
- [ ] **FIX CORE BUG**: Fix msgspec.Struct serialization in Spikard
- [ ] Use dataclass as workaround for benchmarks
- [ ] Add proper Query() type hints for query params (already done)
- [ ] Add UploadFile handling for multipart workloads
- [ ] Add form parsing for URL-encoded workloads

### 1.2 Spikard Node - Add Zod/TypeBox Validation
**Status**: ❌ Currently just JSON.parse() with no validation

**Requirements**:
- Use Zod or TypeBox for schema validation
- Define TypeScript interfaces
- Validate query params, path params, bodies

```typescript
import { z } from 'zod';

const UserSchema = z.object({
  name: z.string(),
  email: z.string().email(),
  age: z.number().int().positive()
});

app.post('/json/small', async (request) => {
  const user = UserSchema.parse(request.body);
  return user;
});
```

**Tasks**:
- [ ] Choose validation library (Zod recommended)
- [ ] Define schemas for all JSON body workloads
- [ ] Add query parameter validation
- [ ] Add multipart file parsing

### 1.3 Spikard Ruby - Add Validation
**Status**: ❌ Currently just returning raw hashes

**Requirements**:
- Use dry-validation or similar
- Define Ruby classes/structs
- Validate all inputs

**Tasks**:
- [ ] Choose validation library
- [ ] Define classes for all workloads
- [ ] Add query/path param validation
- [ ] Add multipart handling

### 1.4 Spikard Rust - Verify Validation
**Status**: ⚠️  Need to verify

**Tasks**:
- [ ] Check if Rust server properly validates
- [ ] Ensure using serde validation
- [ ] Add typed structs for all workloads

### 1.5 FastAPI - Add Pydantic Models
**Status**: ❌ Currently just `request.json()` - no validation

**Requirements**:
- Use Pydantic v2 BaseModel (FastAPI standard)
- Add Field() validators
- Proper query parameter typing

```python
from pydantic import BaseModel, Field

class UserCreate(BaseModel):
    name: str = Field(min_length=1)
    email: str = Field(pattern=r'^[\w\.-]+@')
    age: int = Field(gt=0, lt=150)

@app.post("/json/small")
async def handler(user: UserCreate) -> UserCreate:
    return user  # Auto validation + serialization
```

**Tasks**:
- [ ] Define Pydantic models for all JSON workloads
- [ ] Add Query() params with validation
- [ ] Add UploadFile for multipart
- [ ] Add Form() for URL-encoded

### 1.6 Robyn - Add Validation
**Status**: ❌ Currently just `request.json()` - no validation

**Tasks**:
- [ ] Research Robyn validation best practices
- [ ] Add proper typing
- [ ] Validate all inputs

---

## Phase 2: Align Workloads with testing_data/ Fixtures

Per ADR 0003, benchmarks should use same data as tests.

### 2.1 JSON Bodies
**Source**: `testing_data/json_bodies/`

**Tasks**:
- [ ] Review existing fixtures (01_simple_object_success.json, etc.)
- [ ] Create benchmark-specific fixtures for size variants
- [ ] Define schemas that match testing_data patterns
- [ ] Ensure all frameworks validate same structures

**Size Variants**:
- Small: ~100-500 bytes (simple object, 3-5 fields)
- Medium: ~1-10KB (nested objects, 10-20 fields)
- Large: ~10-100KB (arrays with many items)
- Very Large: ~100KB-1MB (deep nesting + large arrays)

### 2.2 Query Parameters
**Source**: `testing_data/query_params/`

**Tasks**:
- [ ] Define typed query param handlers:
  - Few (1-3 params): `q: str, page: int = 1, limit: int = 10`
  - Medium (5-10 params): filtering, sorting, pagination
  - Many (15-30 params): complex filters, multiple arrays
- [ ] Add validation (min/max, patterns, enums)
- [ ] Test with actual query strings in workload definitions

### 2.3 Path Parameters
**Source**: `testing_data/path_params/`

**Tasks**:
- [ ] Simple: `/users/{id}` - string
- [ ] Int: `/posts/{post_id}` - validate integer
- [ ] UUID: `/resources/{uuid}` - validate format
- [ ] Multiple: `/users/{user_id}/posts/{post_id}`
- [ ] Deep: `/{org}/{team}/{project}/{resource}/{id}`
- [ ] Add type conversion + validation

### 2.4 Multipart Forms
**Source**: `testing_data/multipart/`

**Tasks**:
- [ ] Actually parse file uploads (not just return metadata)
- [ ] Small: 1 file, ~1KB
- [ ] Medium: 2-3 files, ~10KB
- [ ] Large: 5 files, ~100KB
- [ ] Measure file parsing overhead

### 2.5 URL-Encoded Forms
**Source**: `testing_data/url_encoded/`

**Tasks**:
- [ ] Parse `application/x-www-form-urlencoded`
- [ ] Simple: 3-5 fields
- [ ] Complex: 20+ fields, nested if supported
- [ ] Validate parsed data

---

## Phase 3: Workload Definition Updates

### 3.1 Update WorkloadDef Schema
**File**: `tools/benchmark-harness/src/schema/workload.rs`

**Tasks**:
- [ ] Add validation_required: bool field
- [ ] Add expected_schema: Option<JsonSchema>
- [ ] Add validates_types: bool
- [ ] Add parses_files: bool for multipart

### 3.2 Synthetic Data Generation
**File**: `tools/benchmark-harness/src/profile/runner.rs`

**Tasks**:
- [ ] Generate valid JSON matching schemas
- [ ] Generate proper query strings with types
- [ ] Generate multipart boundaries with actual files
- [ ] Generate URL-encoded forms

---

## Phase 4: Documentation

### 4.1 README Updates
**Tasks**:
- [ ] Document validation requirements
- [ ] Show example DTOs for each framework
- [ ] Explain why validation matters for benchmarks
- [ ] Link to ADR 0003

### 4.2 Per-Framework Documentation
**Tasks**:
- [ ] Document Spikard msgspec usage
- [ ] Document FastAPI Pydantic patterns
- [ ] Document Node validation library choice
- [ ] Document Ruby validation approach

---

## Success Criteria

Benchmarks are valid when:
- ✅ All frameworks define typed DTOs/schemas
- ✅ All frameworks validate query parameters
- ✅ All frameworks parse multipart files
- ✅ All frameworks parse URL-encoded forms
- ✅ Validation overhead is measurable
- ✅ Follows each framework's documented best practices
- ✅ Aligns with Spikard ADR 0003 and testing_data/ fixtures

---

## Priority Order

1. **CRITICAL** (Do Immediately):
   - Fix Spikard Python (msgspec DTOs)
   - Fix FastAPI (Pydantic models)
   - Fix query parameter validation across all frameworks

2. **HIGH** (Do Next):
   - Add multipart file parsing
   - Add URL-encoded form parsing
   - Fix Node and Ruby validation

3. **MEDIUM** (Do After):
   - Align with testing_data fixtures
   - Update workload definitions
   - Documentation

---

## Related Files

- ADR: `docs/adr/0003-validation-and-fixtures.md`
- Fixtures: `testing_data/`
- Workload Schema: `tools/benchmark-harness/src/schema/workload.rs`
- Runner: `tools/benchmark-harness/src/profile/runner.rs`
- Servers: `tools/benchmark-harness/apps/*/server.*`
