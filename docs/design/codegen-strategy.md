# Code Generation Strategy

**Date**: 2025-01-31
**Status**: 🟢 Active
**Related**: [testing-strategy.md](./testing-strategy.md), [api-design.md](./api-design.md)

## Purpose

Code generation in Spikard is **test infrastructure tooling** - not a published feature. It exists to:

1. **Generate comprehensive test suites** from test fixtures across all target languages
2. **Generate minimal test applications** that validate fixture scenarios
3. **Keep tests synchronized** with fixture updates automatically
4. **Reduce manual test maintenance** burden across 3+ languages

**Future work**: Full-featured OpenAPI → production app codegen is planned but not in scope.

## Architecture

### Component Roles

```
testing_data/           # Source of truth: test fixtures
    ├── query_params/
    ├── path_params/
    └── ...

crates/spikard-codegen/ # Code generation library (NOT PUBLISHED)
    ├── src/
    │   ├── openapi/
    │   │   ├── from_fixtures.rs  # Fixtures → OpenAPI
    │   │   └── spec.rs            # OpenAPI 3.1 types
    │   └── lib.rs
    └── Cargo.toml

Taskfile.yaml           # Orchestrates generation

e2e/                    # Generated test infrastructure
    ├── rust/
    │   ├── app/        # Generated minimal Axum app
    │   └── tests/      # Generated test suite
    ├── python/
    │   ├── app/        # Generated minimal Spikard app
    │   ├── tests/      # Generated pytest suite
    │   └── conftest.py # Shared fixtures (NOT generated)
    └── typescript/
        ├── app/        # Generated minimal Fastify app
        ├── tests/      # Generated Vitest suite
        └── setup.ts    # Shared setup (NOT generated)
```

### Generation Pipeline

```
┌─────────────────────┐
│  testing_data/*.json│  Authoritative test scenarios
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ task generate:tests │  Orchestrates entire pipeline
└──────────┬──────────┘
           │
           ├─────────────────────────────────────┐
           │                                     │
           ▼                                     ▼
  ┌────────────────────┐            ┌───────────────────────┐
  │ Generate Test Apps │            │ Generate Test Suites  │
  │  (per language)    │            │   (per language)      │
  └────────┬───────────┘            └───────────┬───────────┘
           │                                    │
           ▼                                    ▼
    e2e/<lang>/app/                     e2e/<lang>/tests/
    - Minimal server                    - Parametrized tests
    - Route handlers                    - Per-fixture assertions
    - Echo validated params             - Shared utilities
```

## Generation Strategy

### Test Applications

**Purpose**: Minimal apps that validate Spikard correctly handles each fixture scenario

**Characteristics**:
- **Simple handlers**: Echo back validated parameters/body
- **No business logic**: Just proves validation worked
- **Idiomatic structure**: Follows each language's conventions
- **Self-contained**: Can run independently

**Example** (Rust):
```rust
// e2e/rust/app/src/routes.rs - GENERATED
#[get("/users")]
async fn list_users(
    Query(query): Query<ListUsersQuery>
) -> Json<Value> {
    // Echo back validated params
    json!({
        "page": query.page,
        "limit": query.limit,
        "search": query.search
    })
}
```

### Test Suites

**Purpose**: Comprehensive parametrized tests covering all fixture scenarios

**Characteristics**:
- **One test file per fixture category**: `test_query_params.rs`, `test_path_params.py`, etc.
- **Parametrized**: Each fixture becomes a test case
- **Assertion-rich**: Validates status, headers, body structure, error messages
- **Language-native patterns**: pytest parametrize, Rust `#[test_case]`, Vitest `describe.each`

**Example** (Python/pytest):
```python
# e2e/python/tests/test_query_params.py - GENERATED
import pytest
from .conftest import client, load_fixtures

@pytest.mark.parametrize("fixture", load_fixtures("query_params"))
def test_query_param_scenarios(client, fixture):
    """Test all query parameter validation scenarios."""
    response = client.get(
        fixture["request"]["path"],
        params=fixture["request"].get("query_params", {})
    )

    assert response.status_code == fixture["expected_response"]["status_code"]

    if fixture["expected_response"].get("validation_errors"):
        assert "detail" in response.json()
        # Validate error structure matches fixture
```

## What Gets Generated vs Manual

### Generated (Deletable/Regeneratable)

✅ **e2e/<lang>/app/** - Test applications
- Route handlers
- Models/schemas
- Server startup
- Cargo.toml / package.json / pyproject.toml

✅ **e2e/<lang>/tests/** - Test files
- Parametrized test functions
- Per-category test files
- Assertion logic

### Manual (Checked In)

❌ **e2e/<lang>/conftest.py** (Python) - Shared pytest fixtures
❌ **e2e/<lang>/common/mod.rs** (Rust) - Shared test utilities
❌ **e2e/<lang>/setup.ts** (TypeScript) - Vitest configuration
❌ **e2e/<lang>/README.md** - Documentation
❌ **.gitignore** - Ignore generated files

## Taskfile Commands

```yaml
generate:tests:
  desc: "Generate all test infrastructure from fixtures"
  cmds:
    - task: generate:tests:rust
    - task: generate:tests:python
    - task: generate:tests:typescript

generate:tests:rust:
  desc: "Generate Rust test app and suite"
  cmds:
    - rm -rf e2e/rust/app e2e/rust/tests
    - cargo run -p spikard-codegen --bin gen-tests -- --lang rust --fixtures testing_data --output e2e/rust

generate:tests:python:
  desc: "Generate Python test app and suite"
  cmds:
    - rm -rf e2e/python/app e2e/python/tests
    - cargo run -p spikard-codegen --bin gen-tests -- --lang python --fixtures testing_data --output e2e/python

clean:generated:
  desc: "Remove all generated test infrastructure"
  cmds:
    - rm -rf e2e/*/app e2e/*/tests
```

## spikard-codegen Crate Structure

```
crates/spikard-codegen/
├── Cargo.toml           # NOT published to crates.io
├── src/
│   ├── lib.rs           # Library interface
│   ├── bin/
│   │   └── gen-tests.rs # Test generation CLI
│   ├── openapi/
│   │   ├── from_fixtures.rs
│   │   └── spec.rs
│   ├── generators/
│   │   ├── rust.rs      # Rust app + test generation
│   │   ├── python.rs    # Python app + test generation
│   │   └── typescript.rs # TS app + test generation
│   └── templates/
│       ├── rust/        # Tera/Handlebars templates
│       ├── python/
│       └── typescript/
└── README.md           # "Internal tooling - not published"
```

### Key Design Decisions

1. **Separate binary** (`gen-tests`) - No mixing with spikard CLI
2. **Not published** - Internal tooling only
3. **Template-based** - Use Tera or Handlebars for flexibility
4. **Idiomatic output** - Each language follows its conventions

## Example: Adding a New Language

To add Go support:

1. Create `crates/spikard-codegen/src/generators/go.rs`
2. Add Go templates in `crates/spikard-codegen/src/templates/go/`
3. Create `e2e/go/` directory with manual support files
4. Add `generate:tests:go` task to Taskfile
5. Generate and verify tests pass

## Benefits

### For Development
- ✅ **Consistency**: All languages test the same scenarios
- ✅ **Coverage**: 367 fixtures = 367 test cases per language
- ✅ **Maintenance**: Update fixtures once, regenerate all tests
- ✅ **Confidence**: Cross-language validation catches binding bugs

### For CI/CD
- ✅ **Fast iteration**: `task generate:tests` after fixture changes
- ✅ **Catches regressions**: Generated tests fail if behavior changes
- ✅ **Documentation**: Generated tests show usage patterns

## Future: Production Codegen Feature

**Out of scope for initial release**, but planned:

```bash
# Future: Generate production applications from OpenAPI
spikard generate \
  --spec api.yaml \
  --lang python \
  --output src/generated \
  --framework spikard

# Would generate:
# - Full application structure
# - Database models
# - Authentication/authorization
# - Client SDKs
# - Documentation sites
```

This requires:
- Published `spikard-codegen` crate
- Production-quality templates
- Plugin system for customization
- OpenAPI extension support
- Database integration
- Auth patterns

## Non-Goals

❌ **Not** building a general-purpose codegen framework
❌ **Not** replacing handwritten application code
❌ **Not** generating complex business logic
❌ **Not** a competitor to OpenAPI Generator / Swagger Codegen

## Success Criteria

1. ✅ Running `task generate:tests` creates working test suites for Rust, Python, TypeScript
2. ✅ All generated tests pass in CI
3. ✅ Fixture changes automatically reflected in tests after regeneration
4. ✅ Generated code is readable and idiomatic
5. ✅ Zero manual test duplication across languages
