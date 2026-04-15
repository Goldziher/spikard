---
priority: critical
---

# Fixture-Driven Testing

All features must be tested against JSON fixtures in `testing_data/`. The same fixtures drive tests across all language bindings to ensure behavioral consistency.

## Workflow

1. Add or update fixture files in the appropriate `testing_data/` subdirectory
2. Extend parametrized test suites (e.g., `packages/python/tests/test_all_fixtures.py`)
3. Run `task test` to verify Rust, Python, and JS checks pass
4. Run per-language tests: `task test:rust`, `task test:python`, `task test:node`, `task test:ruby`, `task test:php`
5. Run `task generate:e2e` for cross-language test generation from fixtures

## Fixture Directories

- `testing_data/http_methods/` - HTTP method handling
- `testing_data/validation_errors/` - Error response schemas
- `testing_data/headers/`, `testing_data/cookies/` - Header/cookie validation
- `testing_data/openapi_schemas/`, `testing_data/graphql/` - Spec parsing
- `testing_data/edge_cases/` - Large/nested payload stress tests

## Never

- Ship without running `task test`
- Add behavior without corresponding fixtures
- Test language-specific behavior in Rust tests (use binding test suites)
