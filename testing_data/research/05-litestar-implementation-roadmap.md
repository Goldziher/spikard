# Litestar Test Scenarios Implementation Roadmap for Spikard

Based on analysis of `/tmp/litestar/tests/unit/test_kwargs/`, this document outlines which test scenarios should be replicated in Spikard's testing framework.

---

## EXECUTIVE SUMMARY

Litestar's test suite in `test_kwargs/` contains approximately **45+ parametrized test cases** covering:

1. **Query Parameters** - Lists, numeric validation, optional/defaults
2. **Path Parameters** - Type conversion (13+ types), numeric validation
3. **Headers** - Type coercion, string/numeric validation
4. **Cookies** - Similar to headers with cookie-specific behavior
5. **Body Data** - JSON, URL-encoded, multipart with files
6. **Advanced Scenarios** - Mixed types, optional fields, file uploads

### Key Validation Constraints Tested:
- Numeric: `gt`, `ge`, `lt`, `le` (most frequent)
- String: `min_length`, `max_length`
- Array: `min_items`, `max_items`
- Meta: `required`, `default`, type coercion

---

## IMPLEMENTATION PRIORITY

### Phase 1: Core Query & Path Validation (Foundation)
**Goal:** Replicate 15+ basic validation scenarios

**Query Params Test Fixtures to Create:**

```
testing_data/query_params/
├── 01_valid_required_int.json
├── 02_required_int_missing.json
├── 03_int_validation_le_fail.json
├── 04_required_list_missing.json
├── 05_list_max_items_exceeded.json
├── 06_list_min_items_violated.json
├── 07_list_valid_with_max_items.json
├── 08_int_with_gt_validation.json
├── 09_optional_params_omitted.json
├── 10_optional_params_provided.json
├── 11_param_aliasing_query_name.json
├── 12_special_chars_in_values.json
├── 13_list_required_missing.json
├── 14_union_type_single_value.json
└── 15_union_type_array_value.json
```

**Path Params Test Fixtures to Create:**

```
testing_data/path_params/
├── 01_valid_multiple_types.json
├── 02_uuid_conversion_standard.json
├── 03_uuid_conversion_hex.json
├── 04_int_validation_gt_fail.json
├── 05_int_validation_le_fail.json
├── 06_float_validation_boundaries.json
├── 07_string_minlength_validation_fail.json
├── 08_string_maxlength_validation_fail.json
├── 09_decimal_type_conversion.json
├── 10_date_type_conversion.json
├── 11_time_type_conversion.json
├── 12_datetime_type_conversion.json
├── 13_timedelta_seconds_format.json
├── 14_timedelta_iso8601_format.json
└── 15_path_type_conversion.json
```

---

### Phase 2: Headers & Cookies (Similar Pattern)
**Goal:** Replicate 8-10 header/cookie scenarios

**Headers Test Fixtures:**

```
testing_data/headers/
├── 01_valid_string_header.json
├── 02_header_validation_minlength_fail.json
├── 03_header_validation_maxlength_fail.json
├── 04_required_header_missing.json
├── 05_optional_header_missing.json
├── 06_int_header_type_conversion.json
├── 07_int_header_validation_ge_fail.json
├── 08_int_header_validation_le_fail.json
├── 09_header_aliasing_custom_name.json
└── 10_header_with_post_method.json
```

**Cookies Test Fixtures:**

```
testing_data/cookies/
├── 01_valid_int_cookie.json
├── 02_cookie_type_conversion.json
├── 03_cookie_validation_ge_fail.json
├── 04_required_cookie_missing.json
├── 05_optional_cookie_missing.json
├── 06_cookie_with_default_value.json
├── 07_cookie_aliasing_custom_name.json
├── 08_cookie_validation_min_length_fail.json
└── 09_cookie_with_post_method.json
```

---

### Phase 3: Body Data (JSON, URL-Encoded, Multipart)
**Goal:** Replicate 10-15 body parsing scenarios

**JSON Body Fixtures:**

```
testing_data/json_bodies/
├── 01_valid_structured_data.json
├── 02_required_field_missing.json
├── 03_empty_dict_allowed.json
├── 04_with_default_values.json
├── 05_string_field_required.json
├── 06_int_field_required.json
├── 07_bool_field_required.json
└── 08_nested_object_parsing.json
```

**URL-Encoded Fixtures:**

```
testing_data/url_encoded/
├── 01_valid_form_data.json
├── 02_required_field_missing.json
├── 03_optional_form_empty.json
├── 04_percent_encoding_handling.json
└── 05_special_chars_in_values.json
```

**Multipart Fixtures:**

```
testing_data/multipart/
├── 01_single_file_upload.json
├── 02_multiple_file_upload.json
├── 03_file_with_content_type.json
├── 04_mixed_files_and_form_data.json
├── 05_multipart_with_list_field.json
├── 06_multipart_with_optional_fields.json
├── 07_empty_string_preservation.json
├── 08_utf8_filename_encoding.json
├── 09_multipart_validation_constraints.json
├── 10_file_required_missing.json
├── 11_multipart_vs_urlencoded_consistency.json
├── 12_invalid_multipart_format.json
├── 13_multipart_part_limit.json
├── 14_optional_file_upload.json
├── 15_file_with_custom_headers.json
├── 16_required_file_missing.json
└── 17_mixed_file_types.json
```

---

### Phase 4: Validation Constraints & Edge Cases
**Goal:** Ensure comprehensive validation coverage

**Additional Constraint Tests:**

```
testing_data/validation/
├── 01_pattern_regex_validation.json (NOT in Litestar)
├── 02_enum_validation.json (NOT in Litestar)
├── 03_custom_validator_failure.json (NOT in Litestar)
├── 04_multiple_constraints_combined.json
├── 05_type_coercion_failure.json
├── 06_nested_array_validation.json
├── 07_deeply_nested_objects.json
├── 08_union_type_validation.json
└── 09_discriminated_union.json
```

---

## CONSTRAINT IMPLEMENTATION CHECKLIST

Based on Litestar test frequency analysis:

### Must Implement (High Priority):
- [ ] `le` (less than or equal) - 7 occurrences
- [ ] `gt` (greater than) - 5 occurrences
- [ ] `ge` (greater than or equal) - 4 occurrences
- [ ] `min_length` - 4 occurrences
- [ ] `max_length` - 4 occurrences
- [ ] `min_items` - 2 occurrences
- [ ] `max_items` - 2 occurrences
- [ ] `required` flag - 8 occurrences
- [ ] `default` values - 5 occurrences
- [ ] Type coercion - 13+ occurrences

### Should Implement (Medium Priority):
- [ ] `lt` (less than) - 1 occurrence
- [ ] `pattern` (regex) - 0 in Litestar, but common
- [ ] Parameter aliasing (query, header, cookie names)
- [ ] Type conversion for 13+ path param types
- [ ] Union types (single value OR array)

### Future Enhancement (Low Priority):
- [ ] `unique_items` - Not in Litestar tests
- [ ] Custom validators - Not in Litestar tests
- [ ] Enum validation - Not in Litestar tests
- [ ] Nested validation - Limited in Litestar tests

---

## TEST CASE MAPPING

### Query Params - Direct Mapping

| Litestar Test | Scenario | Fixture Name | Status |
|---|---|---|---|
| test_query_params (case 1) | Valid required list | query_params/01_valid_required_list.json | To create |
| test_query_params (case 3) | Required missing | query_params/04_required_list_missing.json | Similar |
| test_query_params (case 4) | max_items exceeded | query_params/05_list_max_items_exceeded.json | To create |
| test_query_params (case 5) | le validation fail | query_params/03_int_validation_le_fail.json | To create |
| test_query_params (case 6) | min_items violated | query_params/06_list_min_items_violated.json | To create |
| test_query_param_arrays | Union types | query_params/14_union_type_single_value.json | To create |
| test_query_param_dependency_with_alias | Aliasing | query_params/11_param_aliasing_query_name.json | To create |

### Path Params - Direct Mapping

| Litestar Test | Scenario | Fixture Name | Status |
|---|---|---|---|
| test_path_params (case 1) | Valid multiple types | path_params/01_valid_multiple_types.json | To create |
| test_path_params (case 2) | gt validation fail | path_params/04_int_validation_gt_fail.json | To create |
| test_path_params (case 3) | le validation fail | path_params/05_int_validation_le_fail.json | To create |
| test_path_params (case 4) | max_length fail | path_params/08_string_maxlength_validation_fail.json | Similar |
| test_path_param_type_resolution | 13 types | path_params/02-15_*.json | To create |

### Headers - Direct Mapping

| Litestar Test | Scenario | Fixture Name | Status |
|---|---|---|---|
| test_header_params (case 1) | Valid string | headers/01_valid_string_header.json | To create |
| test_header_params (case 2) | max_length fail | headers/03_header_validation_maxlength_fail.json | To create |
| test_header_params (case 3) | Required missing | headers/04_required_header_missing.json | Similar |
| test_header_params (case 5) | Int conversion | headers/06_int_header_type_conversion.json | To create |
| test_header_params (case 6) | Validation fail | headers/07_int_header_validation_ge_fail.json | To create |

### Cookies - Direct Mapping

| Litestar Test | Scenario | Fixture Name | Status |
|---|---|---|---|
| test_cookie_params (case 1) | Optional missing | cookies/05_optional_cookie_missing.json | Similar |
| test_cookie_params (case 2) | Valid int | cookies/01_valid_int_cookie.json | To create |
| test_cookie_params (case 3) | Validation fail | cookies/03_cookie_validation_ge_fail.json | To create |
| test_cookie_params (case 4) | Required missing | cookies/04_required_cookie_missing.json | Similar |

### Body Data - Partial Mapping

| Litestar Test | Scenario | Fixture Name | Status |
|---|---|---|---|
| test_request_body_json | Valid JSON | json_bodies/01_valid_structured_data.json | To create |
| test_empty_dict_allowed | Empty dict | json_bodies/03_empty_dict_allowed.json | To create |
| test_request_body_url_encoded | Valid form | url_encoded/01_valid_form_data.json | To create |
| test_request_body_multi_part | File upload | multipart/01_single_file_upload.json | To create |
| test_multipart_handling_of_optional_values | Optional fields | multipart/06_multipart_with_optional_fields.json | To create |
| test_multipart_form_part_limit | Part limit | multipart/13_multipart_part_limit.json | To create |

---

## SCHEMA STRUCTURE TEMPLATE

For each fixture file, use this JSON schema:

```json
{
  "name": "Descriptive test case name",
  "description": "What this test validates",
  "path": "/api/endpoint",
  "method": "GET|POST|PUT|DELETE",
  "query_params": {
    "param_name": "value or array"
  },
  "path_params": {
    "id": "123"
  },
  "headers": {
    "header-name": "value"
  },
  "cookies": {
    "cookie-name": "value"
  },
  "body": {
    "field": "value"
  },
  "expected_status": 200,
  "expected_response": {
    "status": "success"
  },
  "validation_rules": {
    "param_name": {
      "type": "string|integer|number|array|object|boolean",
      "required": true,
      "minLength": 1,
      "maxLength": 100,
      "minimum": 0,
      "maximum": 100,
      "gt": 0,
      "ge": 0,
      "lt": 100,
      "le": 100,
      "minItems": 1,
      "maxItems": 10,
      "pattern": "^[a-z]+$",
      "enum": ["value1", "value2"],
      "default": "value",
      "items": {
        "type": "string"
      }
    }
  }
}
```

---

## IMPLEMENTATION PHASES TIMELINE

### Phase 1: Foundation (Week 1-2)
- Implement `gt`, `ge`, `le` constraints
- Implement `min_length`, `max_length` constraints
- Create 15 query param fixtures
- Create 15 path param fixtures
- Tests should pass: query_params/*, path_params/*

### Phase 2: Headers & Cookies (Week 3)
- Implement header parameter validation
- Implement cookie parameter validation
- Create 10 header fixtures
- Create 9 cookie fixtures
- Tests should pass: headers/*, cookies/*

### Phase 3: Body Data (Week 4-5)
- Implement JSON body parsing and validation
- Implement URL-encoded form parsing
- Implement multipart form parsing with files
- Create 8 JSON body fixtures
- Create 5 URL-encoded fixtures
- Create 17 multipart fixtures
- Tests should pass: json_bodies/*, url_encoded/*, multipart/*

### Phase 4: Polish & Edge Cases (Week 6)
- Add parameter aliasing support
- Add type coercion error handling
- Add edge case fixtures
- Comprehensive validation of all constraints
- Document known limitations

---

## VALIDATION COVERAGE MATRIX

| Constraint | Query | Path | Header | Cookie | JSON Body | URL Form | Multipart |
|-----------|-------|------|--------|--------|-----------|----------|-----------|
| gt | YES | YES | YES | NO | NO | NO | NO |
| ge | NO | NO | YES | YES | YES | NO | YES |
| lt | NO | NO | NO | NO | YES | NO | NO |
| le | YES | YES | YES | YES | YES | NO | YES |
| min_length | YES | YES | YES | NO | NO | NO | NO |
| max_length | YES | YES | YES | NO | NO | NO | NO |
| min_items | YES | NO | NO | NO | NO | NO | YES |
| max_items | YES | NO | NO | NO | NO | NO | YES |
| pattern | NO | NO | NO | NO | NO | NO | NO |
| required | YES | YES | YES | YES | YES | YES | YES |
| default | YES | YES | YES | YES | YES | YES | YES |
| Type coercion | YES | YES | YES | YES | YES | YES | YES |

Legend: YES = Should test, NO = Not in Litestar

---

## QUICK START - CREATE FIRST 5 FIXTURES

To begin implementation immediately, create these 5 fixtures:

### 1. testing_data/query_params/01_valid_required_list.json
```json
{
  "description": "Valid query parameters with list",
  "path": "/test",
  "method": "GET",
  "query_params": {
    "page": 1,
    "pageSize": 50,
    "brands": ["Nike", "Adidas"]
  },
  "expected_status": 200,
  "validation_rules": {
    "page": { "type": "integer", "required": true },
    "pageSize": { "type": "integer", "gt": 0, "le": 100 },
    "brands": { "type": "array", "items": { "type": "string" }, "minItems": 1, "maxItems": 3 }
  }
}
```

### 2. testing_data/path_params/01_valid_multiple_types.json
```json
{
  "description": "Multiple path parameter types with validation",
  "path": "/{version:float}/{service_id:int}/{user_id:str}/{order_id:uuid}",
  "method": "GET",
  "path_params": {
    "version": 1.5,
    "service_id": 10,
    "user_id": "alice",
    "order_id": "550e8400-e29b-41d4-a716-446655440000"
  },
  "expected_status": 200,
  "validation_rules": {
    "version": { "type": "number", "gt": 0.1, "le": 4.0 },
    "service_id": { "type": "integer", "gt": 0, "le": 100 },
    "user_id": { "type": "string", "minLength": 1, "maxLength": 10 },
    "order_id": { "type": "string", "format": "uuid" }
  }
}
```

### 3. testing_data/headers/01_valid_string_header.json
```json
{
  "description": "Valid string header with length validation",
  "path": "/test",
  "method": "GET",
  "headers": {
    "special-header": "123"
  },
  "expected_status": 200,
  "validation_rules": {
    "special_header": {
      "type": "string",
      "header": "special-header",
      "minLength": 1,
      "maxLength": 3,
      "required": true
    }
  }
}
```

### 4. testing_data/cookies/01_valid_int_cookie.json
```json
{
  "description": "Valid integer cookie with range validation",
  "path": "/test",
  "method": "GET",
  "cookies": {
    "special-cookie": "123"
  },
  "expected_status": 200,
  "validation_rules": {
    "special_cookie": {
      "type": "integer",
      "cookie": "special-cookie",
      "ge": 100,
      "le": 201,
      "required": true
    }
  }
}
```

### 5. testing_data/json_bodies/01_valid_structured_data.json
```json
{
  "description": "Valid JSON body with structured data",
  "path": "/test",
  "method": "POST",
  "body": {
    "name": "Moishe Zuchmir",
    "age": 30,
    "programmer": true,
    "value": "100"
  },
  "expected_status": 201,
  "validation_rules": {
    "name": { "type": "string", "required": true },
    "age": { "type": "integer", "required": true },
    "programmer": { "type": "boolean", "required": true },
    "value": { "type": "string", "required": true }
  }
}
```

---

## REFERENCES

- **Litestar Analysis**: See `03-litestar-test-analysis.md`
- **Code Examples**: See `04-litestar-test-code-examples.md`
- **Original Test Files**: `/tmp/litestar/tests/unit/test_kwargs/`
