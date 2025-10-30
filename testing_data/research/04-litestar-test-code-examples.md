# Litestar Test Code Examples - Ready to Replicate

This document contains concrete code examples from Litestar tests that can be directly replicated in Spikard's fixture system.

---

## QUERY PARAMS - CORE TEST PATTERN

### Pattern 1: List with Min/Max Items + Numeric Validation

```python
# Handler Definition
@get(path="/test")
def test_method(
    page: int,
    page_size: int = Parameter(query="pageSize", gt=0, le=100),
    brands: list[str] = Parameter(min_items=1, max_items=3),
    from_date: Optional[datetime] = None,
    to_date: Optional[datetime] = None,
) -> None:
    pass

# Test Cases:
Test Cases:
1. Valid: {"page": 1, "pageSize": 1, "brands": ["Nike", "Adidas"]}
   Expected: 200 OK

2. Valid max items: {"page": 1, "pageSize": 1, "brands": ["Nike", "Adidas", "Rebok"]}
   Expected: 200 OK

3. Missing required: {"page": 1, "pageSize": 1}
   Expected: 400 BAD_REQUEST (brands required, min_items=1)

4. Exceeds max_items: {"page": 1, "pageSize": 1, "brands": ["Nike", "Adidas", "Rebok", "Polgat"]}
   Expected: 400 BAD_REQUEST (4 items > max 3)

5. Exceeds numeric le: {"page": 1, "pageSize": 101, "brands": ["Nike", "Adidas"]}
   Expected: 400 BAD_REQUEST (101 > 100)

6. List empty (min_items=1): {"page": 1, "pageSize": 1, "brands": []}
   Expected: 400 BAD_REQUEST (0 < min 1)

7. Valid with optional: {"page": 1, "pageSize": 1, "brands": ["Nike", "Adidas"], "from_date": timestamp}
   Expected: 200 OK

8. Valid with both optionals: {..., "from_date": timestamp, "to_date": timestamp}
   Expected: 200 OK
```

### Fixture Structure (JSON):
```json
{
  "path": "/test",
  "method": "GET",
  "query_params": {
    "page": 1,
    "pageSize": 1,
    "brands": ["Nike", "Adidas", "Rebok"],
    "from_date": 1234567890.0,
    "to_date": 1234567890.0
  },
  "expected_status": 200,
  "expected_response": {
    "status": "success"
  },
  "validation_rules": {
    "page": { "type": "integer", "required": true },
    "pageSize": { "type": "integer", "query": "pageSize", "gt": 0, "le": 100 },
    "brands": { "type": "array", "items": { "type": "string" }, "minItems": 1, "maxItems": 3 },
    "from_date": { "type": "number", "required": false },
    "to_date": { "type": "number", "required": false }
  }
}
```

---

## PATH PARAMS - TYPE CONVERSION TEST PATTERN

### Pattern 1: Multiple Types with Numeric Boundaries

```python
# Handler Definition
@get(path="{version:float}/{service_id:int}/{user_id:str}/{order_id:uuid}")
def test_method(
    order_id: UUID,
    version: float = Parameter(gt=0.1, le=4.0),
    service_id: int = Parameter(gt=0, le=100),
    user_id: str = Parameter(min_length=1, max_length=10),
) -> None:
    pass

# Test Cases:
1. Valid: version=1.0, service_id=1, user_id="abc", order_id=uuid4()
   Expected: 200 OK

2. Exceeds float gt: version=4.1
   Expected: 400 BAD_REQUEST (4.1 > le 4.0)

3. Service ID exceeds le: service_id=101
   Expected: 400 BAD_REQUEST (101 > 100)

4. User ID exceeds max_length: user_id="abcdefghijklm"
   Expected: 400 BAD_REQUEST (13 chars > 10)

5. Valid with uuid1: order_id=uuid1()
   Expected: 200 OK
```

### Fixture Structure (JSON):
```json
{
  "path": "/{version:float}/{service_id:int}/{user_id:str}/{order_id:uuid}",
  "method": "GET",
  "path_params": {
    "version": 1.0,
    "service_id": 1,
    "user_id": "abc",
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

### Type Conversions to Test:
```python
# 13 type conversion cases from Litestar:
1. str: "abc" → "abc"
2. int: "1" → 1
3. float: "1.01" → 1.01
4. uuid (hex): "0fcb1054c56e4dd4a127f70a97d1fc21" → UUID
5. uuid (standard): "542226d1-7199-41a0-9cba-aaa6d85932a3" → UUID
6. Decimal: "1.00001" → Decimal
7. date: "2023-07-15" → date
8. time: "01:02:03" → time
9. datetime: "2023-07-15T15:45:34.073314" → datetime
10. timedelta (seconds): "86400.0" → timedelta(days=1)
11. timedelta (ISO 8601): "P1D" → timedelta(days=1)
12. timedelta (ISO 8601): "PT1H1S" → timedelta(hours=1, seconds=1)
13. Path (absolute): "/1/2/3/4/some-file.txt" → Path
14. Path (relative): "1/2/3/4/some-file.txt" → Path (normalized)
```

---

## HEADER PARAMS - TYPE COERCION PATTERN

### Pattern 1: Headers with Type Conversion and Validation

```python
# Handler Definition
@get(path="/test")
def test_method(
    special_header: str = Parameter(header="special-header", min_length=1, max_length=3)
) -> None:
    pass

# Test Cases:
1. Valid string: headers={"special-header": "123"}
   Expected: 200 OK (length 3)

2. String max_length exceeded: headers={"special-header": "123"}
   With max_length=2
   Expected: 400 BAD_REQUEST

3. Required header missing: headers={}
   Expected: 400 BAD_REQUEST

4. Optional header missing: headers={}
   Parameter(required=False, default=None)
   Expected: 200 OK

5. Integer conversion: headers={"special-header": "123"}
   type: int, ge=100, le=201
   Expected: 200 OK (converts "123" to 123)

6. Integer validation failed: headers={"special-header": "123"}
   type: int, le=120
   Expected: 400 BAD_REQUEST (123 > 120)
```

### Fixture Structure (JSON):
```json
{
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

---

## COOKIE PARAMS - VALIDATION PATTERN

### Pattern 1: Cookies with Type Coercion

```python
# Handler Definition
@get(path="/test")
def test_method(
    special_cookie: int = Parameter(cookie="special-cookie", ge=100, le=201)
) -> None:
    pass

# Test Cases:
1. Valid integer cookie: cookies={"special-cookie": "123"}
   Expected: 200 OK

2. Cookie validation failed: cookies={"special-cookie": "123"}
   With le=120
   Expected: 400 BAD_REQUEST

3. Required cookie missing: cookies={}
   Expected: 400 BAD_REQUEST

4. Optional cookie missing: cookies={}
   Parameter(required=False, default=None)
   Expected: 200 OK

5. Optional with value: cookies={"special-cookie": "123"}
   Parameter(required=False, default=None)
   Expected: 200 OK
```

### Fixture Structure (JSON):
```json
{
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

---

## JSON BODY - BASIC PATTERN

### Pattern 1: Structured Data Parsing

```python
# Handler Definition
@post(path="/test")
def test_method(data: Form) -> None:
    # Form: name: str, age: int, programmer: bool, value: str
    pass

# Test Cases:
1. Valid JSON:
   POST /test
   {"name": "Moishe Zuchmir", "age": 30, "programmer": true, "value": "100"}
   Expected: 201 CREATED

2. Empty dict:
   POST /test
   {}
   Expected: 201 CREATED or 400 BAD_REQUEST (depends on required fields)

3. With defaults:
   POST /test (no body)
   Default provided
   Expected: 201 CREATED
```

### Fixture Structure (JSON):
```json
{
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

## URL-ENCODED DATA - PATTERN

### Pattern 1: Form Data Parsing

```python
# Handler Definition
@post(path="/test")
def test_method(data: Form = Body(media_type=RequestEncodingType.URL_ENCODED)) -> None:
    pass

# Test Cases:
1. Valid URL-encoded:
   POST /test
   Content-Type: application/x-www-form-urlencoded
   name=Moishe+Zuchmir&age=30&programmer=true&value=100
   Expected: 201 CREATED

2. Optional body empty:
   POST /test
   (empty body)
   data: Optional[Form]
   Expected: 201 CREATED
```

### Fixture Structure (JSON):
```json
{
  "path": "/test",
  "method": "POST",
  "content_type": "application/x-www-form-urlencoded",
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

## MULTIPART DATA - ADVANCED PATTERN

### Pattern 1: Files with Mixed Field Types

```python
# Handler Definition
@dataclass
class MultiPartFormWithMixedFields:
    image: UploadFile
    tags: list[int]

@post(path="/form")
async def test_method(data: MultiPartFormWithMixedFields = Body(media_type=RequestEncodingType.MULTI_PART)) -> None:
    pass

# Test Cases:
1. Valid multipart with file and list:
   POST /form
   ---boundary---
   Content-Disposition: form-data; name="image"; filename="test.png"
   Content-Type: image/png

   (binary data)
   ---boundary---
   Content-Disposition: form-data; name="tags"

   1
   ---boundary---
   Content-Disposition: form-data; name="tags"

   2
   ---boundary---
   Content-Disposition: form-data; name="tags"

   3
   ---boundary---

   Expected: 201 CREATED
   Result: image: UploadFile, tags: [1, 2, 3]
```

### Pattern 2: Optional Fields in Multipart

```python
# Handler Definition
@dataclass
class ProductForm:
    name: str
    int_field: int
    options: str
    optional_without_default: Optional[float]
    optional_with_default: Optional[int] = None

@post(path="/")
def handler(data: ProductForm = Body(media_type=RequestEncodingType.MULTI_PART)) -> dict:
    pass

# Test Cases:
1. With all fields:
   POST /
   name: "moishe zuchmir"
   int_field: 1
   options: "[1,2,3,4]"
   optional_without_default: 3.14
   optional_with_default: 42
   Expected: 201 CREATED

2. Optional fields empty:
   POST /
   name: "moishe zuchmir"
   int_field: 1
   options: "[1,2,3,4]"
   (optional fields omitted)
   Expected: 201 CREATED or 400 (depends on required)
```

### Fixture Structure (JSON):
```json
{
  "path": "/form",
  "method": "POST",
  "content_type": "multipart/form-data",
  "body": {
    "image": {
      "filename": "test.png",
      "content_type": "image/png",
      "content": "<binary data>"
    },
    "tags": [1, 2, 3]
  },
  "expected_status": 201,
  "validation_rules": {
    "image": { "type": "file", "required": true },
    "tags": { "type": "array", "items": { "type": "integer" }, "required": true }
  }
}
```

---

## PARAMETER ALIASING - PATTERN

### Pattern 1: Query Parameter with Different Name

```python
# Handler Definition
@get("/")
def handler(
    page_size: int = Parameter(query="pageSize", gt=0, le=100)
) -> None:
    pass

# Test Case:
GET /?pageSize=50
Expected: 200 OK
Handler receives: page_size=50
```

### Fixture Structure:
```json
{
  "path": "/",
  "method": "GET",
  "query_params": {
    "pageSize": 50
  },
  "expected_status": 200,
  "validation_rules": {
    "page_size": {
      "type": "integer",
      "query": "pageSize",
      "gt": 0,
      "le": 100,
      "required": true
    }
  }
}
```

---

## DEFAULTS - PATTERN

### Pattern 1: Default Value with Validation

```python
# Handler Definition
@get(path="/test")
def test_method(
    page_size: int = Parameter(query="pageSize", gt=0, le=100, default=10)
) -> None:
    pass

# Test Cases:
1. With value: GET /test?pageSize=10
   Expected: 200 OK, page_size=10

2. Default: GET /test
   Expected: 200 OK, page_size=10 (default)

3. Invalid with default: GET /test?pageSize=101
   Expected: 400 BAD_REQUEST (101 > 100, validation runs even on explicit values)
```

### Fixture Structure:
```json
{
  "path": "/test",
  "method": "GET",
  "query_params": {},
  "expected_status": 200,
  "validation_rules": {
    "page_size": {
      "type": "integer",
      "query": "pageSize",
      "gt": 0,
      "le": 100,
      "default": 10
    }
  }
}
```

---

## EDGE CASES TO TEST

### 1. Special Characters in Query Values
```
Query: ?first=x@test.com&second=aaa
       ?first=&@A.ac&second=aaa
       ?first=a@A.ac&&second=aaa
       ?first=a@A&.ac&second=aaa

Should parse and preserve characters correctly
```

### 2. Type Coercion Failures
```
Path: /123abc/ (int expected, string provided)
Expected: 400 BAD_REQUEST

Header: Content-Length: "not_a_number"
Expected: 400 BAD_REQUEST
```

### 3. Empty Strings in Forms
```
Multipart/URL-encoded with: field=""
Should preserve empty string, not treat as null
```

### 4. Percent Encoding
```
URL-encoded: name=John%20Doe&age=30
Should decode: name="John Doe", age=30
```

### 5. Required vs Optional Combinations
```
Handler signature variations:
1. param: int (required)
2. param: Optional[int] (optional, None if missing)
3. param: int = Parameter(required=False, default=None) (optional)
4. param: int = Parameter(default=10) (optional with default)
5. param: int = Parameter(required=True) (required)
```

---

## VALIDATION CONSTRAINT REFERENCE

### Numeric Constraints
- `gt`: Greater than (exclusive)
- `ge`: Greater than or equal (inclusive)
- `lt`: Less than (exclusive)
- `le`: Less than or equal (inclusive)

### String Constraints
- `min_length`: Minimum character count
- `max_length`: Maximum character count
- `pattern`: Regex pattern matching (not in Litestar tests, but common)

### Array/List Constraints
- `min_items`: Minimum number of items
- `max_items`: Maximum number of items
- `unique_items`: All items must be unique (not tested)

### General Constraints
- `required`: Parameter must be present
- `default`: Default value when not provided
- Type conversion: Automatic string → target type

### Special Cases
- Parameter aliasing: Different name in request vs handler
- Type coercion: Headers and cookies are strings, converted to target type
- Union types: Parameter can be single value or array
- Optional types: None is valid value

---

## FIXTURE FILE NAMING CONVENTIONS (from Spikard)

Based on existing test files:

- `{location}/{scenario_number}_{description}.json`

Examples:
```
testing_data/query_params/01_valid_required_params.json
testing_data/query_params/02_required_string_missing.json
testing_data/query_params/13_list_required_missing.json
testing_data/path_params/03_int_path_param_invalid.json
testing_data/path_params/07_int_validation_gt_fail.json
testing_data/headers/01_valid_header.json
testing_data/cookies/04_required_cookie_missing.json
testing_data/json_bodies/02_required_field_missing.json
testing_data/multipart/16_required_file_missing.json
```
