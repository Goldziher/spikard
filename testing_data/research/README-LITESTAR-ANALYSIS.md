# Litestar Test Analysis Documentation Index

This directory contains a comprehensive analysis of Litestar's test suite for replication in Spikard.

## Quick Navigation

### For Getting Started (Start Here)
1. Read: **This file** (you are here)
2. Read: [03-litestar-test-analysis.md](./03-litestar-test-analysis.md) - Overview of all test scenarios
3. Review: [05-litestar-implementation-roadmap.md](./05-litestar-implementation-roadmap.md) - Implementation plan

### For Implementation
- [04-litestar-test-code-examples.md](./04-litestar-test-code-examples.md) - Copy-paste ready code patterns
- [05-litestar-implementation-roadmap.md](./05-litestar-implementation-roadmap.md) - Fixture templates and phase breakdown

### For Reference During Development
- [03-litestar-test-analysis.md](./03-litestar-test-analysis.md) - Look up specific constraints
- [04-litestar-test-code-examples.md](./04-litestar-test-code-examples.md) - Code patterns

## Document Descriptions

### 03-litestar-test-analysis.md (13 KB)
**Complete breakdown of Litestar's test suite**

Contains:
- Test-by-test analysis of all 9 test modules
- 45+ test cases organized by location (query, path, header, cookie, body)
- 13+ type conversions detailed for path parameters
- Unique validation scenarios for each test
- Edge cases and complex scenarios
- Summary tables by category
- Validation pattern frequency analysis

**Best for:** Understanding what Litestar tests and how to think about test scenarios

### 04-litestar-test-code-examples.md (15 KB)
**Ready-to-use code examples from Litestar**

Contains:
- Concrete handler definitions for each parameter type
- Test case parameters with expected results
- Fixture JSON structure templates
- Parameter aliasing patterns
- All 13 path parameter type conversions with examples
- Edge case examples (special characters, encoding, etc.)
- Validation constraint reference
- Fixture naming conventions

**Best for:** When you need to create a fixture or understand how a constraint works

### 05-litestar-implementation-roadmap.md (16 KB)
**Step-by-step implementation guide for Spikard**

Contains:
- Executive summary of test findings
- Implementation priority tiers (Phase 1-4)
- Complete fixture creation roadmap (45+ files)
- Constraint implementation checklist
- Test case mapping (Litestar → Spikard fixtures)
- JSON schema structure template
- Implementation timeline
- Validation coverage matrix
- 5 quick-start fixtures with full examples

**Best for:** Planning implementation and tracking progress

## Key Statistics

### Test Coverage
- **Total Parametrized Cases:** 45+
- **Query Parameters:** 15 test cases
- **Path Parameters:** 19 test cases
- **Headers:** 10 test cases
- **Cookies:** 5 test cases
- **Body Data:** 20+ test cases (JSON, URL-encoded, multipart)
- **Validation:** 8+ test cases

### Constraints to Implement
**Must Implement (10 constraints):**
- le (<=) - 7 occurrences
- gt (>) - 5 occurrences
- ge (>=) - 4 occurrences
- min_length - 4 occurrences
- max_length - 4 occurrences
- min_items - 2 occurrences
- max_items - 2 occurrences
- required - 8 occurrences
- default - 5 occurrences
- Type coercion - 13+ occurrences

**Should Implement (3 constraints):**
- lt (<)
- pattern (regex)
- Parameter aliasing

**Future (3 constraints):**
- unique_items
- Custom validators
- Enum validation

### Type Conversions (Path Parameters)
13 different type conversions tested:
str, int, float, uuid (2 formats), Decimal, date, time, datetime, timedelta (2 formats), Path

## Implementation Roadmap Summary

### Phase 1: Core Constraints (1-2 weeks)
**Status:** Not started
- Implement: gt, ge, le, min_length, max_length
- Create: ~30 fixtures (query + path params)

### Phase 2: Headers & Cookies (1 week)
**Status:** Depends on Phase 1
- Implement: Header/cookie parameter parsing
- Create: ~20 fixtures

### Phase 3: Body Data (2 weeks)
**Status:** Depends on Phase 1 & 2
- Implement: JSON, URL-encoded, multipart parsing
- Create: ~30 fixtures

### Phase 4: Polish (1 week)
**Status:** Depends on Phase 1-3
- Add: lt, pattern, parameter aliasing
- Create: ~10+ edge case fixtures

**Total Estimated Timeline:** 5-7 weeks for full coverage

## How to Use These Documents

### If You're Implementing a New Constraint
1. Find the constraint in 03-litestar-test-analysis.md
2. Check 04-litestar-test-code-examples.md for patterns
3. Find the corresponding fixtures to create in 05-litestar-implementation-roadmap.md
4. Create the fixtures using the JSON template

### If You're Creating Fixtures
1. Determine the parameter location (query, path, header, cookie, body)
2. Look up valid test patterns in 04-litestar-test-code-examples.md
3. Check the fixture roadmap in 05-litestar-implementation-roadmap.md for naming
4. Use the JSON schema template from 05-litestar-implementation-roadmap.md
5. Create both valid and invalid test cases (for validation failures)

### If You're Adding Edge Cases
1. Review "Edge Cases Identified" section in 03-litestar-test-analysis.md
2. Check if already covered in 05-litestar-implementation-roadmap.md
3. Look at 04-litestar-test-code-examples.md for similar patterns
4. Create fixture following the established naming conventions

## Original Litestar Test Files

For reference, the original test files are at:
```
/tmp/litestar/tests/unit/test_kwargs/
├── test_query_params.py
├── test_path_params.py
├── test_header_params.py
├── test_cookie_params.py
├── test_json_data.py
├── test_url_encoded_data.py
├── test_multipart_data.py
├── test_validations.py
└── test_defaults.py
```

## Fixture Directory Structure (Target)

Once implemented, Spikard's testing_data/ should mirror this structure:

```
testing_data/
├── query_params/          (15+ fixtures)
├── path_params/           (15+ fixtures)
├── headers/               (10+ fixtures)
├── cookies/               (9+ fixtures)
├── json_bodies/           (8+ fixtures)
├── url_encoded/           (5+ fixtures)
├── multipart/             (17+ fixtures)
└── validation/            (10+ fixtures)
```

Total: 80+ fixture files

## Validation Coverage

| Constraint | Query | Path | Header | Cookie | Body |
|-----------|-------|------|--------|--------|------|
| gt        | ✓     | ✓    | ✓      | ✗      | ✗    |
| ge        | ✗     | ✗    | ✓      | ✓      | ✓    |
| lt        | ✗     | ✗    | ✗      | ✗      | ✓    |
| le        | ✓     | ✓    | ✓      | ✓      | ✓    |
| min_len   | ✓     | ✓    | ✓      | ✗      | ✗    |
| max_len   | ✓     | ✓    | ✓      | ✗      | ✗    |
| min_items | ✓     | ✗    | ✗      | ✗      | ✗    |
| max_items | ✓     | ✗    | ✗      | ✗      | ✗    |
| required  | ✓     | ✓    | ✓      | ✓      | ✓    |
| default   | ✓     | ✓    | ✓      | ✓      | ✓    |
| Type crc  | ✓     | ✓    | ✓      | ✓      | ✓    |

Legend: ✓ = Tested, ✗ = Not tested in Litestar

## Key Insights

1. **Consistency Across Locations**
   - Same validation patterns applied to query, path, header, and cookie parameters
   - Body data has its own validation logic

2. **Type Coercion is Critical**
   - Headers and cookies are strings that get converted to target types
   - Validation happens AFTER type conversion
   - Example: "123" (string) → 123 (int) → validate ge/le

3. **Parameter Aliasing is Common**
   - Parameters have different names in request vs handler code
   - Requires mapping mechanism (e.g., query="pageSize" maps to page_size parameter)

4. **Optional Fields Need Careful Handling**
   - Three patterns: Optional[type], required=False with default, required=True
   - Missing optional fields should not cause validation errors
   - Defaults must still satisfy constraints

5. **File Uploads are Complex**
   - Support single and multiple files
   - Support mixed files and form fields
   - Preserve file metadata (name, content type)
   - Support optional file fields

6. **Edge Cases Matter**
   - Empty strings vs null
   - Special characters in values
   - Encoding/escaping of values
   - Invalid format handling

## Next Steps

1. Start with 03-litestar-test-analysis.md to understand the scope
2. Review 05-litestar-implementation-roadmap.md for Phase 1 fixtures
3. Use 04-litestar-test-code-examples.md as a reference while implementing
4. Create fixtures in order (query → path → headers → cookies → body)
5. Run tests and iterate

## Questions?

Refer back to the specific analysis document:
- "How do I validate a list with min/max items?" → 04-litestar-test-code-examples.md
- "What constraints need what priority?" → 05-litestar-implementation-roadmap.md
- "What edge cases should I test?" → 03-litestar-test-analysis.md
