# Litestar Test Scenarios Analysis

Analysis of test files in `/tmp/litestar/tests/unit/test_kwargs/` to identify replicable test scenarios.

---

## 1. QUERY PARAMS (test_query_params.py)

### Test Case: `test_query_params`
**Parametrize Count:** 9 test cases

**Validation Scenarios:**
1. Valid params: page=1, pageSize=1, brands=["Nike", "Adidas"] (2 brands) ✓
2. Valid params: page=1, pageSize=1, brands=["Nike", "Adidas", "Rebok"] (3 brands) ✓
3. **Required missing**: brands not provided (should fail) ✗
4. **List max_items exceeded**: 4 brands (max=3) ✗
5. **Numeric gt validation exceeded**: pageSize=101 (le=100) ✗
6. **List min_items violated**: empty brands=[] (min=1) ✗
7. Valid with optional datetime: from_date provided ✓
8. Valid with both optional datetimes: from_date and to_date provided ✓
9. Valid with single brand and both datetimes ✓

**Constraints Used:**
- `gt=0` (page, implicit minimum)
- `le=100` (pageSize max)
- `min_items=1` (brands list minimum)
- `max_items=3` (brands list maximum)
- Optional fields with `None` default
- Query parameter aliasing: `pageSize` query name maps to `page_size` param

**Complex Scenarios:**
- List of strings with min/max items validation
- Optional datetime parameters
- Query parameter name aliasing

---

### Test Case: `test_query_param_arrays`
**Parametrize Count:** 2 test cases

**Validation Scenarios:**
1. Union[int, list[int]] with value [1, 2, 3] ✓
2. Union[int, list[int]] with value [1] ✓

**Complex Scenarios:**
- Union types: single value OR list
- Array handling in query params

---

### Test Case: `test_query_kwarg`
**What It Tests:**
- Multiple values for same query parameter key: `a=["foo", "bar"]` and `b=["qux"]`
- MultiDict access pattern for raw query params
- List parsing for repeated query parameters

---

### Test Case: `test_query_parsing_of_escaped_values`
**Parametrize Count:** 4 test cases

**Edge Cases:**
- Email addresses with special characters: `x@test.com`, `&@A.ac`, `a@A.ac&`, `a@A&.ac`
- Escaped value parsing and preservation

---

### Test Case: `test_query_param_dependency_with_alias`
**What It Tests:**
- Query parameter validation in dependency injection
- Parameter aliasing through dependencies

---

### Test Case: `test_query_params_with_post`
**What It Tests:**
- Missing required query parameter in POST request (should fail with 400)
- Query params on POST method

---

## 2. PATH PARAMS (test_path_params.py)

### Test Case: `test_path_params`
**Parametrize Count:** 5 test cases

**Path Pattern:** `{version:float}/{service_id:int}/{user_id:str}/{order_id:uuid}`

**Validation Scenarios:**
1. Valid: version=1.0, service_id=1, user_id="abc", uuid valid ✓
2. **Numeric gt validation**: version=4.1 (gt=0.1, le=4.0) fails ✗
3. **Numeric validation failed**: version=0.2, service_id=101 (le=100) fails ✗
4. **String max_length exceeded**: user_id="abcdefghijklm" (max=10) fails ✗
5. Valid with uuid1 instead of uuid4 ✓

**Constraints Used:**
- `gt=0.1` (version minimum exclusive)
- `le=4.0` (version maximum inclusive)
- `gt=0` (service_id minimum)
- `le=100` (service_id maximum)
- `min_length=1` (user_id minimum)
- `max_length=10` (user_id maximum)
- Path param type converters: float, int, str, uuid

**Complex Scenarios:**
- Multiple type conversions in single path
- UUID validation (uuid4 and uuid1 support)
- Numeric boundary validation (gt, le)
- String length constraints on path params

---

### Test Case: `test_path_param_type_resolution`
**Parametrize Count:** 13 test cases

**Types Tested:**
- str: "abc" → "abc"
- int: "1" → 1
- float: "1.01" → 1.01
- uuid: hex without dashes → UUID object
- uuid: standard UUID format → UUID object
- decimal: "1.00001" → Decimal("1.00001")
- date: "2023-07-15" → date object
- time: "01:02:03" → time object
- datetime: ISO format → datetime object
- timedelta: numeric seconds "86400.0" → 1 day
- timedelta: ISO 8601 "P1D" → 1 day
- timedelta: ISO 8601 "PT1H1S" → 1 hour 1 second
- path: "/1/2/3/4/some-file.txt" → Path object
- path: relative "1/2/3/4/some-file.txt" → Path with leading slash

**Edge Cases:**
- Multiple UUID formats (with and without dashes)
- Multiple timedelta formats (numeric, ISO 8601 duration)
- Path normalization (relative to absolute)

---

### Test Case: `test_optional_path_parameter`
**What It Tests:**
- Optional path parameter: "/" and "/{message:str}" routes
- Default value behavior when parameter not provided

---

### Test Cases: Validation Tests
- `test_path_param_validation`: Invalid path syntax tests
- `test_duplicate_path_param_validation`: Duplicate parameter detection
- `test_path_param_defined_in_layered_params_error`: Ambiguity detection

---

## 3. HEADER PARAMS (test_header_params.py)

### Test Case: `test_header_params`
**Parametrize Count:** 8 test cases

**Validation Scenarios:**
1. **String valid**: "123" with min_length=1, max_length=3 ✓
2. **String max_length exceeded**: "123" with max_length=2 ✗
3. **Required header missing**: empty dict with required header ✗
4. **Optional header missing**: empty dict with required=False, default=None ✓
5. **Integer valid**: "123" with ge=100, le=201 ✓
6. **Integer validation failed**: "123" with le=120 ✗
7. **Required int header missing** ✗
8. **Optional int header missing**: required=False, default=None ✓

**Constraints Used:**
- `min_length` (string minimum)
- `max_length` (string maximum)
- `ge` (greater than or equal)
- `le` (less than or equal)
- `required=False` (optional headers)
- `default=None` (optional default)
- Type coercion: header string → int

**Edge Cases:**
- Header name aliasing: `special_header` parameter name
- Type conversion: string to int in headers
- Optional with explicit None default

---

### Test Case: `test_header_param_with_post`
**What It Tests:**
- Missing required header in POST request (should fail with 400)
- Headers with POST method

---

## 4. COOKIE PARAMS (test_cookie_params.py)

### Test Case: `test_cookie_params`
**Parametrize Count:** 5 test cases

**Validation Scenarios:**
1. **Optional cookie missing**: empty dict with required=False, default=None ✓
2. **Integer valid**: "123" with ge=100, le=201 ✓
3. **Integer validation failed**: "123" with le=120 ✗
4. **Required cookie missing**: empty dict, required ✗
5. **Optional int cookie missing**: required=False, default=None ✓

**Constraints Used:**
- `ge` (greater than or equal)
- `le` (less than or equal)
- `required=False` (optional cookies)
- `default=None` (optional default)
- Type coercion: cookie string → int

**Edge Cases:**
- Cookie name aliasing: `special_cookie` parameter name
- Type conversion: string to int in cookies
- Similar validation patterns to headers

---

### Test Case: `test_cookie_param_with_post`
**What It Tests:**
- Missing required cookie in POST request (should fail with 400)
- Cookies with POST method

---

## 5. JSON BODY DATA (test_json_data.py)

### Test Case: `test_request_body_json`
**What It Tests:**
- JSON body parsing into dataclass Form
- Form structure: name: str, age: int, programmer: bool, value: str
- asdict conversion for test data

---

### Test Case: `test_empty_dict_allowed`
**What It Tests:**
- Empty JSON object ({}) accepted for dict body
- No validation failures on empty payload

---

### Test Case: `test_no_body_with_default`
**What It Tests:**
- POST with no body but default value provided
- Default value returned when body is empty

---

## 6. URL-ENCODED DATA (test_url_encoded_data.py)

### Test Case: `test_request_body_url_encoded`
**What It Tests:**
- URL-encoded form body parsing into Form dataclass
- Equivalent behavior to JSON for same data structure

---

### Test Case: `test_optional_request_body_url_encoded`
**What It Tests:**
- Optional URL-encoded body: Optional[Form]
- Empty body treated as None for optional fields

---

## 7. MULTIPART DATA (test_multipart_data.py)

### Complex Scenarios:

#### File Upload Tests:
1. **Basic file upload**: UploadFile handling
2. **Multiple files**: list[UploadFile]
3. **File with content type**: ("filename", file_obj, "content/type")
4. **Files in dataclass**: Optional fields in file lists
5. **Mixed files and form data**: Files + regular fields in same request
6. **Multipart with validation**: lt=10, ge=1 on structured data

#### Mixed Field Types:
- `test_request_body_multi_part_mixed_field_content_types`:
  - image: UploadFile
  - tags: list[int]
  - Validates type coercion in multipart (strings to ints)

#### Optional Fields Handling:
- `test_multipart_handling_of_optional_values`:
  - name: str (required)
  - int_field: int (required)
  - options: str (required)
  - optional_without_default: Optional[float]
  - optional_with_default: Optional[int] = None

#### Edge Cases:
- Empty strings preservation
- UTF-8 filename encoding
- Percent encoding in form data
- Boundary parsing
- Invalid multipart format rejection

#### Validation Tests:
- Parametrize test on form_type: URL_ENCODED vs MULTI_PART
- Same validation behavior for both encodings
- Struct-based validation: `lt=10, ge=1` on msgspec.Struct

---

## 8. VALIDATION TESTS (test_validations.py)

### Ambiguity Detection Tests:
1. Path param + aliased Parameter with same key (fails)
2. Path param + Dependency with same key (fails)
3. Dependency + aliased Parameter with same key (fails)

### Reserved Kwargs Tests:
- Cannot use RESERVED_KWARGS as path params
- Cannot use RESERVED_KWARGS as dependencies
- Cannot use RESERVED_KWARGS as aliased parameters

### Body Media Type Validation:
- JSON, URL_ENCODED, MULTI_PART conflicts
- Cannot mix different media types in same handler
- Dependency body type must match handler body type

---

## 9. DEFAULTS TESTS (test_defaults.py)

### Test Case: `test_params_default`
**What It Tests:**
- Parameter with default value: `default=10`
- Query param works when provided: `?pageSize=10`
- Query param uses default when omitted
- Default value satisfies validation: ge=0, le=100

---

## SUMMARY BY CATEGORY

### Total Parametrize Test Cases: ~45+

### Query Params:
- Total parametrized cases: 15
- Validation constraints: min_items, max_items, gt, le
- Complex scenarios: Union types, arrays, optional datetimes
- Edge cases: Special characters in values, parameter aliasing

### Path Params:
- Total parametrized cases: 19 (5 + 13 + more)
- Validation constraints: gt, le, min_length, max_length
- Type converters: str, int, float, uuid, decimal, date, time, datetime, timedelta, path
- Edge cases: Multiple UUID formats, timedelta formats, path normalization

### Headers:
- Total parametrized cases: 10
- Validation constraints: min_length, max_length, ge, le
- Complex scenarios: Type coercion (string to int), optional headers
- Edge cases: Header name aliasing, type conversion

### Cookies:
- Total parametrized cases: 5
- Validation constraints: ge, le
- Similar to headers with cookie-specific behavior

### Body Data:
- JSON: Basic parsing
- URL-Encoded: Alternative encoding support
- Multipart: Files, mixed types, optional fields
- Validation: Full struct validation with constraints

### Critical Constraints Not Yet Covered:
- `pattern` (regex validation)
- `min_length`/`max_length` on body fields
- `gt`/`ge`/`lt`/`le` on body fields
- Nested objects and arrays
- Enum validation
- Custom validators

---

## RECOMMENDATIONS FOR SPIKARD

### Priority 1 - Core Validation Constraints:
1. Implement all numeric constraints: gt, ge, lt, le
2. Implement string length constraints: min_length, max_length
3. Implement list constraints: min_items, max_items
4. Implement pattern (regex) validation

### Priority 2 - Parameter Locations:
1. Query parameters with all constraints
2. Path parameters with type conversion
3. Headers with type conversion
4. Cookies with validation

### Priority 3 - Body Types:
1. JSON body validation
2. URL-encoded form validation
3. Multipart form with files
4. Mixed field types in forms

### Priority 4 - Complex Scenarios:
1. Union types (single value or list)
2. Optional fields with defaults
3. Nested structures
4. Array/list handling
5. Type coercion across boundaries

### Priority 5 - Edge Cases:
1. Parameter aliasing
2. Special characters in values
3. Empty values
4. Type conversion failures
5. Missing required parameters

---

## VALIDATION PATTERN FREQUENCY

Based on Litestar tests:

| Constraint | Frequency | Locations |
|-----------|-----------|-----------|
| le (<=) | 7 | query, path, header, cookie |
| gt (>) | 5 | query, path, header |
| ge (>=) | 4 | header, cookie, body |
| min_length | 4 | query, path, header |
| max_length | 4 | query, path, header |
| min_items | 2 | query |
| max_items | 2 | query |
| lt (<) | 1 | body (msgspec) |
| pattern | 0 | (not in tests) |
| required | 8 | query, header, cookie |
| default | 5 | query, header, cookie |
| Type coercion | 13 | headers, cookies, path, body |
