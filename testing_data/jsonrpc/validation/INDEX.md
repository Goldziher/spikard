# JSON-RPC Validation Fixtures - Index

Quick navigation guide for the validation edge case fixtures.

## Files Overview

### Fixture Files (Test Cases)

1. **schema_validation.json** (13 test cases)
   - JSON Schema constraint validation
   - String length, number range, email format, array constraints
   - Start here: [schema_validation.json](schema_validation.json)

2. **type_coercion.json** (11 test cases)
   - Strict type checking without implicit coercion
   - Type mismatch detection, null handling, edge cases
   - Start here: [type_coercion.json](type_coercion.json)

3. **required_fields.json** (10 test cases)
   - Required field enforcement
   - Missing fields, null values, empty strings
   - Start here: [required_fields.json](required_fields.json)

4. **extra_fields.json** (10 test cases)
   - additionalProperties constraint handling
   - Extra field rejection, schema variants (true/false/default)
   - Start here: [extra_fields.json](extra_fields.json)

5. **nested_objects.json** (15 test cases)
   - Deeply nested object validation (3-6+ levels)
   - Nested required fields, pattern validation, arrays of objects
   - Start here: [nested_objects.json](nested_objects.json)

### Schema & Documentation

- **schema.json** - JSON Schema definition for fixture structure
- **README.md** - Comprehensive documentation and integration guide
- **FIXTURES_OVERVIEW.txt** - Complete reference documentation
- **INDEX.md** - This file

## Quick Stats

```
Total Test Cases:     59
Success Cases:        23 (39%)
Failure Cases:        36 (61%)
Constraints Covered:  10
Nesting Depth:        6+ levels
Total Size:           60 KB
```

## Error Response Format

All validation errors use the JSON-RPC 2.0 format with code `-32602`:

```json
{
  "code": "-32602",
  "message": "Invalid params",
  "data": {
    "fieldPath": "user.profile.address.zip",
    "constraint": "pattern",
    "value": "invalid",
    "details": "Human-readable error explanation"
  }
}
```

## Integration Guide

### Python
```python
import json
with open("testing_data/jsonrpc/validation/schema_validation.json") as f:
    fixtures = json.load(f)
```

### Rust
```rust
let fixtures: Vec<TestCase> = serde_json::from_str(
    include_str!("../../../testing_data/jsonrpc/validation/schema_validation.json")
)?;
```

### JavaScript
```javascript
const fixtures = require('./testing_data/jsonrpc/validation/schema_validation.json');
```

## Fixture Categories

### By Constraint Type
- **String**: minLength, maxLength (in schema_validation.json)
- **Number**: minimum, maximum (in schema_validation.json)
- **Type**: type checking, coercion (in type_coercion.json)
- **Format**: email validation (in schema_validation.json)
- **Array**: minItems, item validation (in schema_validation.json, nested_objects.json)
- **Object**: required fields, additionalProperties (in required_fields.json, extra_fields.json)
- **Nested**: deep validation (in nested_objects.json)

### By Test Outcome
- **Passing (✓)**: 23 test cases across all files
- **Failing (✗)**: 36 test cases with error validation

### By Complexity
- **Simple**: 1-level objects (schema_validation.json, type_coercion.json)
- **Medium**: 2-3 level nesting (required_fields.json, extra_fields.json)
- **Complex**: 4-6+ level nesting (nested_objects.json)

## Field Path Examples

- Simple: `name`, `age`, `email`
- Nested: `user.profile.address.zip`
- Array: `tags[1]`, `addresses[0].zip`
- Deep: `user.level1.level2.level3.level4.value`

## Getting Started

1. **Read** [README.md](README.md) for complete documentation
2. **Start** with [schema_validation.json](schema_validation.json) for basic constraints
3. **Explore** [type_coercion.json](type_coercion.json) for type safety
4. **Test** [nested_objects.json](nested_objects.json) for complex scenarios
5. **Reference** [FIXTURES_OVERVIEW.txt](FIXTURES_OVERVIEW.txt) for detailed breakdown

## File References

| File | Test Cases | Focus | Real-world Use |
|------|------------|-------|-----------------|
| schema_validation.json | 13 | Schema constraints | Forms, user data |
| type_coercion.json | 11 | Type checking | API contracts, security |
| required_fields.json | 10 | Required fields | Data validation |
| extra_fields.json | 10 | Extra properties | Schema enforcement |
| nested_objects.json | 15 | Deep nesting | Complex domains |

## Validation Constraints

| Constraint | Example | File |
|------------|---------|------|
| type | "25" for int | type_coercion.json |
| required | missing name | required_fields.json |
| minLength | "A" (min 2) | schema_validation.json |
| maxLength | 51+ chars | schema_validation.json |
| minimum | -5 (min 0) | schema_validation.json |
| maximum | 200 (max 150) | schema_validation.json |
| pattern | "123" for ^[0-9]{5}$ | nested_objects.json |
| format | invalid email | schema_validation.json |
| minItems | [] (min 1) | schema_validation.json |
| additionalProperties | extra field | extra_fields.json |

## Next Steps

### For Implementation
1. Load fixtures in test infrastructure
2. Create parametrized tests for each fixture
3. Implement validation handlers
4. Verify consistent error responses

### For Extension
1. Follow template in README.md
2. Add new test cases to appropriate file
3. Validate JSON syntax
4. Update documentation

## Support

For questions about specific test cases, see the detailed descriptions in:
- [README.md](README.md) - Overview and integration
- [FIXTURES_OVERVIEW.txt](FIXTURES_OVERVIEW.txt) - Complete reference

## Version Info

- **Created**: 2025-12-10
- **Standard**: JSON-RPC 2.0
- **Schema**: JSON Schema Draft 7+
- **Error Code**: -32602 (Invalid params)

---

**Location**: `/Users/naamanhirschfeld/workspace/spikard/testing_data/jsonrpc/validation/`

All fixtures are ready for immediate use in Spikard test suites.
