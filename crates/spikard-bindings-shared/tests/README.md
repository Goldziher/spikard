# spikard-bindings-shared Test Suite

This directory contains comprehensive integration tests for the spikard-bindings-shared crate, achieving 97.96% code coverage.

## Test Structure

### 1. Unit Tests (in `src/` modules)
Each module includes inline unit tests covering basic functionality:
- **102 unit tests** total across all modules
- Test basic API surface, happy paths, and common edge cases
- Located in `#[cfg(test)] mod tests` blocks within each module

### 2. Integration Test Suites

#### `handler_base_integration.rs` (10 tests)
Comprehensive integration tests for the `handler_base` module:
- **Coverage achieved**: 40/40 lines (100%)
- Tests validation error paths
- Tests parameter validation integration
- Tests handler execution failures (prepare, invoke, interpret)
- Tests builder pattern with multiple validators

Key scenarios:
- Validation errors from SchemaValidator
- Parameter validation failures
- Handler execution error paths
- Builder pattern chaining

#### `error_response_edge_cases.rs` (19 tests)
Edge case tests for the `error_response` module:
- **Coverage achieved**: 44/48 lines (91.67%)
- Tests all status code convenience methods
- Tests complex nested error details
- Tests validation errors with multiple fields
- Tests ProblemDetails with extensions and instance

Key scenarios:
- Unicode and special characters in error messages
- Null values and complex JSON structures
- All HTTP status code builders
- RFC 9457 Problem Details format

#### `full_coverage.rs` (12 tests)
Full-coverage tests across all modules:
- Tests all remaining code paths
- Comprehensive lifecycle hook tests
- Full validation helper coverage
- DI trait integration tests
- Conversion trait implementations

Key scenarios:
- All lifecycle hook types
- Header and body validation with all field types
- Test client configuration and metadata
- DI value and factory adapters
- Custom type conversions

## Coverage Summary

| Module | Lines Covered | Coverage |
|--------|--------------|----------|
| conversion_traits.rs | 6/6 | 100.00% |
| di_traits.rs | 16/16 | 100.00% |
| error_response.rs | 44/48 | 91.67% |
| handler_base.rs | 40/40 | 100.00% |
| lifecycle_base.rs | 8/8 | 100.00% |
| response_builder.rs | 19/19 | 100.00% |
| test_client_base.rs | 22/22 | 100.00% |
| validation_helpers.rs | 37/37 | 100.00% |
| **TOTAL** | **192/196** | **97.96%** |

## Uncovered Lines

Only 4 lines remain uncovered, all in `error_response.rs`:
- Lines 131-132: Serialization fallback for ValidationError
- Lines 156-157: Serialization fallback for ProblemDetails

These are intentionally difficult to test as they only trigger when `serde_json` serialization fails, which is extremely rare and would require creating malformed types that can't be serialized. The fallback strings are hardcoded valid JSON to ensure the API never returns invalid responses.

## Running Tests

```bash
# Run all tests
cargo test -p spikard-bindings-shared

# Run specific test suite
cargo test -p spikard-bindings-shared --test handler_base_integration

# Run with coverage
cargo tarpaulin --package spikard-bindings-shared --out Html
```

## Test Philosophy

These tests follow spikard project conventions:
- ✅ Use `pretty_assertions` for clear diffs
- ✅ Test both success and error paths
- ✅ Use fixtures from `testing_data/` where applicable
- ✅ Follow naming: `test_<what>_<scenario>`
- ✅ Target 90-95% coverage per module
- ✅ Keep tests focused and maintainable

## Maintenance Notes

- All tests pass consistently
- No flaky tests
- Integration tests use mock implementations to avoid external dependencies
- Tests are independent and can run in any order
- Coverage target: maintain >95% for new changes
