# JSON-RPC Validation Edge Case Fixtures

Comprehensive test fixtures for JSON-RPC 2.0 parameter validation in Spikard testing framework.

## Overview

This directory contains 5 fixture files with 59 total test cases covering JSON Schema validation edge cases. All fixtures follow the JSON-RPC 2.0 specification with error code `-32602 (Invalid params)` for validation failures.

## Fixture Files

### 1. schema_validation.json (13 test cases)

Tests field-level JSON Schema validation constraints:
- **Valid inputs**: All required/optional fields with valid values
- **String validation**: minLength (≥2), maxLength (≤50)
- **Number validation**: minimum (≥0), maximum (≤150)
- **Email validation**: Format validation
- **Array validation**: minItems (≥1), item type checking
- **Multiple errors**: Comprehensive error aggregation

**Schema Example**:
```json
{
  "name": {"type": "string", "minLength": 2, "maxLength": 50},
  "age": {"type": "integer", "minimum": 0, "maximum": 150},
  "email": {"type": "string", "format": "email"},
  "tags": {"type": "array", "items": {"type": "string"}, "minItems": 1}
}
```

**Key Test Cases**:
- ✓ valid_input_all_fields
- ✓ valid_input_required_only
- ✗ name_too_short (1 char)
- ✗ name_too_long (51+ chars)
- ✗ age_negative
- ✗ age_over_maximum
- ✗ invalid_email_no_at
- ✗ empty_tags_array
- ✗ tags_with_non_string_items
- ✗ missing_required_name
- ✗ missing_required_email
- ✗ multiple_validation_errors (4 errors)

### 2. type_coercion.json (11 test cases)

Tests strict type checking without implicit coercion:
- String to number rejection
- Number to string rejection
- String boolean ("true") rejection
- Array/object type confusion
- Null handling
- Float for integer rejection
- Edge cases (zero, false, empty string)

**Key Test Cases**:
- ✓ valid_types
- ✗ string_number_rejected ("25" for integer)
- ✗ number_string_rejected (123 for string)
- ✗ string_boolean_rejected ("true" for boolean)
- ✗ array_as_object_rejected
- ✗ object_as_array_rejected
- ✗ null_vs_missing_field
- ✗ float_for_integer_rejected (25.5)
- ✗ empty_string_validation
- ✓ number_zero_valid
- ✓ boolean_false_valid

### 3. required_fields.json (10 test cases)

Tests required field enforcement:
- All required fields present
- Single missing field
- Multiple missing fields
- All fields missing
- Null values for required fields
- Empty string for required fields
- Optional fields handling

**Schema**:
```json
{
  "required": ["id", "name", "email"],
  "properties": {
    "id": {"type": "string"},
    "name": {"type": "string"},
    "email": {"type": "string"},
    "phone": {"type": "string"}
  }
}
```

**Key Test Cases**:
- ✓ all_required_present
- ✓ required_only_present
- ✗ missing_id
- ✗ missing_name
- ✗ missing_email
- ✗ missing_two_required_fields
- ✗ all_required_fields_missing (3 errors)
- ✗ required_field_is_null
- ✗ required_field_is_empty_string

### 4. extra_fields.json (10 test cases)

Tests additionalProperties constraint handling:
- Valid exact fields (no extras)
- Single extra field rejection
- Multiple extra fields rejection
- Typos creating required field misses + extras
- Nested object extras
- Different schema configurations (false/true/default)
- Special characters in field names

**Schema Variants**:
- `additionalPropertiesFalse`: Strict, no extra fields allowed
- `additionalPropertiesTrue`: Permissive, extras allowed
- `additionalPropertiesDefault`: Default JSON Schema behavior

**Key Test Cases**:
- ✓ valid_exact_fields (additionalPropertiesFalse)
- ✓ valid_with_optional_field (additionalPropertiesTrue)
- ✗ one_extra_field_rejected
- ✗ multiple_extra_fields_rejected (3 errors)
- ✗ typo_missing_required_and_extra (2 errors)
- ✗ extra_nested_object_rejected
- ✓ schema_with_additional_properties_true
- ✓ schema_without_additional_properties_specified
- ✗ special_characters_in_extra_field_name

### 5. nested_objects.json (15 test cases)

Tests deeply nested object validation:
- Valid nested structures (3+ levels deep)
- Missing required nested fields with path tracking
- Invalid patterns in nested values
- Null nested objects
- Arrays of nested objects
- Nested arrays with item validation
- Multiple nested errors with paths
- 6-level deep nesting

**Schema**:
```json
{
  "type": "object",
  "properties": {
    "user": {
      "type": "object",
      "properties": {
        "profile": {
          "type": "object",
          "properties": {
            "address": {
              "type": "object",
              "properties": {
                "street": {"type": "string"},
                "zip": {"type": "string", "pattern": "^[0-9]{5}$"}
              },
              "required": ["street", "zip"]
            }
          },
          "required": ["address"]
        }
      },
      "required": ["profile"]
    }
  },
  "required": ["user"]
}
```

**Key Test Cases**:
- ✓ valid_nested_structure
- ✓ valid_nested_with_optional_fields
- ✗ missing_nested_street
- ✗ invalid_zip_pattern
- ✗ invalid_zip_with_letters
- ✗ missing_intermediate_profile
- ✗ null_for_nested_object
- ✓ array_of_nested_objects_valid
- ✗ array_of_nested_objects_invalid_item
- ✓ nested_array_with_item_validation
- ✗ nested_array_invalid_items
- ✗ multiple_nested_validation_errors (2 errors)
- ✓ deep_nesting_six_levels
- ✗ deep_nesting_invalid_at_level_four
- ✗ missing_required_at_root_level

## Error Format

All validation errors follow the JSON-RPC 2.0 error response format:

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32602,
    "message": "Invalid params",
    "data": {
      "fieldPath": "user.profile.address.zip",
      "constraint": "pattern",
      "value": "123",
      "details": "Value does not match pattern: ^[0-9]{5}$"
    }
  },
  "id": "request-id"
}
```

### Error Data Fields

- **fieldPath** (string): Dot-notation path to the field (e.g., `user.profile.address.zip`, `tags[1]`)
- **constraint** (string): JSON Schema constraint that failed
- **value** (any): The actual value that failed validation
- **details** (string): Human-readable explanation of the failure

### Multiple Errors

When multiple fields fail, error response includes an `errors` array:

```json
{
  "data": {
    "errors": [
      {
        "fieldPath": "name",
        "constraint": "minLength",
        "details": "String length must be at least 2"
      },
      {
        "fieldPath": "age",
        "constraint": "maximum",
        "details": "Value must be at most 150"
      }
    ]
  }
}
```

## Validation Constraints Covered

| Constraint | Description | Example |
|------------|-------------|---------|
| `type` | Type mismatch | "25" instead of 25 |
| `required` | Missing required field | omitted name |
| `minLength` | String too short | "A" (min 2) |
| `maxLength` | String too long | 51+ chars (max 50) |
| `minimum` | Number too small | -5 (min 0) |
| `maximum` | Number too large | 200 (max 150) |
| `pattern` | Regex pattern mismatch | "123" (pattern ^[0-9]{5}$) |
| `format` | Format validation | "noatsign" (email format) |
| `minItems` | Array too small | [] (min 1) |
| `maxItems` | Array too large | 6+ items (max 5) |
| `uniqueItems` | Duplicate items | [1, 2, 1] |
| `additionalProperties` | Extra fields not allowed | extra_field: true |

## Integration with Tests

Use these fixtures in Spikard test suites:

### Python (packages/python/tests/test_all_fixtures.py)

```python
import json
import pytest

@pytest.fixture
def validation_fixtures():
    with open("testing_data/jsonrpc/validation/schema_validation.json") as f:
        return json.load(f)

@pytest.mark.parametrize("test_case", validation_fixtures, ids=lambda t: t["name"])
def test_schema_validation(client, test_case):
    response = client.jsonrpc_call(test_case["method"], test_case["params"])
    if test_case["expectedSuccess"]:
        assert response["result"] is not None
    else:
        assert response["error"]["code"] == "-32602"
        assert test_case["expectedError"] in response["error"]["data"]
```

### Rust (crates/spikard-http/tests/)

```rust
#[test]
fn test_schema_validation_from_fixtures() {
    let fixtures = load_fixtures("testing_data/jsonrpc/validation/schema_validation.json");
    for test_case in fixtures {
        let result = validate_params(&test_case.params, &test_case.schema);
        assert_eq!(result.is_ok(), test_case.expected_success);
    }
}
```

## File Sizes

```
schema_validation.json    5.7 KB  (13 test cases)
type_coercion.json        4.1 KB  (11 test cases)
required_fields.json      4.6 KB  (10 test cases)
extra_fields.json         5.3 KB  (10 test cases)
nested_objects.json       7.0 KB  (15 test cases)
schema.json               3.7 KB  (JSON Schema definition)
───────────────────────────────────────────────
Total                    30.4 KB  (59 test cases)
```

## Best Practices

1. **Field Paths**: Always use dot notation for nested fields
   - Good: `user.profile.address.zip`
   - Bad: `user['profile']['address']['zip']`

2. **Array Indices**: Use bracket notation for array items
   - Good: `tags[1]` for second element
   - Bad: `tags.1`

3. **Error Aggregation**: Group related errors in a single response
   - Include all validation failures, not just the first
   - Use `errors` array for multiple failures

4. **Realistic Data**: Use realistic field names and values
   - Email addresses with proper format
   - Zip codes with 5-digit pattern
   - Age with reasonable min/max

5. **Schema Alignment**: Keep test cases aligned with actual JSON Schema
   - Match constraint names to JSON Schema Draft 7+
   - Use consistent error messages

## Extending the Fixtures

To add new test cases:

1. Choose appropriate fixture file based on validation type
2. Add test case object with required fields: `name`, `method`, `params`, `expectedSuccess`
3. For failures, include `expectedError` with `code`, `message`, and `data`
4. Use descriptive snake_case names for test cases
5. Validate JSON syntax: `python3 -m json.tool fixture_file.json`
6. Update this README if adding new constraint types

## Schema Validation File Structure

Each fixture file is a JSON array of test case objects:

```json
[
  {
    "name": "unique_test_case_name",
    "method": "module.method",
    "params": { /* JSON-RPC params object */ },
    "expectedSuccess": true,
    "expectedError": {
      "code": "-32602",
      "message": "Invalid params",
      "data": { /* error details */ }
    }
  }
]
```

All fixtures follow JSON Schema Draft 7+ standards and use ISO 8601 datetime formats where applicable.
