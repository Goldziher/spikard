# Generated Test Architecture

**Date**: 2025-01-30
**Status**: 🟡 Draft
**Related**: [02-testing-strategy.md](./02-testing-strategy.md), [09-unified-config-format.md](./09-unified-config-format.md)

## Executive Summary

A code generation pipeline that converts test fixtures → OpenAPI 3.1 → language-specific test apps and test suites. Generated code lives in `e2e/<lang>/` and can be regenerated via Taskfile commands.

## Architecture

```
testing_data/*.json (Fixtures)
        ↓
   [Aggregator]
        ↓
openapi-specs/*.yaml (OpenAPI 3.1)
        ↓
   [Generators]
        ↓
    ┌──────┬──────┬──────┐
    ↓      ↓      ↓      ↓
  e2e/   e2e/   e2e/   e2e/
  rust/  python/ ts/   other/
    ├── app/          (generated server)
    ├── tests/        (generated tests)
    └── README.md     (generation metadata)
```

## Directory Structure

```
spikard/
├── testing_data/              # Source of truth: fixtures
│   ├── query_params/
│   ├── path_params/
│   └── ...
├── openapi-specs/             # Generated OpenAPI specs
│   ├── full.yaml             # All fixtures → one spec
│   ├── query_params.yaml     # Category-specific
│   ├── path_params.yaml
│   └── ...
├── e2e/                       # Generated tests and apps
│   ├── rust/
│   │   ├── app/              # Generated Axum app
│   │   │   ├── src/
│   │   │   │   ├── main.rs
│   │   │   │   ├── routes.rs
│   │   │   │   └── models.rs
│   │   │   └── Cargo.toml
│   │   ├── tests/            # Generated test suite
│   │   │   ├── query_params_tests.rs
│   │   │   ├── path_params_tests.rs
│   │   │   └── common/
│   │   │       └── client.rs
│   │   └── README.md
│   ├── python/
│   │   ├── app/              # Generated FastAPI/Spikard app
│   │   │   ├── main.py
│   │   │   ├── routes.py
│   │   │   └── models.py
│   │   ├── tests/            # Generated pytest suite
│   │   │   ├── test_query_params.py
│   │   │   └── conftest.py
│   │   └── README.md
│   └── typescript/
│       ├── app/              # Generated Express/Fastify app
│       │   ├── src/
│       │   │   ├── index.ts
│       │   │   └── routes.ts
│       │   └── package.json
│       ├── tests/            # Generated Vitest suite
│       │   └── *.test.ts
│       └── README.md
└── Taskfile.yaml             # Generation commands
```

## Generation Pipeline

### Phase 1: Fixture → OpenAPI

```rust
// crates/spikard-codegen/src/openapi/from_fixtures.rs

/// Convert test fixtures to OpenAPI 3.1 specification
pub fn fixtures_to_openapi(
    fixtures: Vec<Fixture>,
    options: OpenApiOptions,
) -> Result<OpenApiSpec> {
    let mut spec = OpenApiSpec::new("3.1.0");

    // Group fixtures by route
    let routes = group_by_route(&fixtures);

    for (path, methods) in routes {
        let path_item = build_path_item(methods)?;
        spec.paths.insert(path, path_item);
    }

    // Extract schemas from fixtures
    let schemas = extract_schemas(&fixtures)?;
    spec.components.schemas = schemas;

    Ok(spec)
}

/// Group fixtures by route path and method
fn group_by_route(fixtures: &[Fixture]) -> HashMap<String, Vec<Fixture>> {
    // Route -> [Fixtures]
    // Merge multiple fixtures for same route
}

/// Extract all unique schemas from fixtures
fn extract_schemas(fixtures: &[Fixture]) -> Result<HashMap<String, Schema>> {
    // Collect all body_schema, response schemas
    // Deduplicate by schema content
    // Generate schema names
}
```

### Phase 2: OpenAPI → Language-Specific Code

```rust
// crates/spikard-codegen/src/generators/rust_from_openapi.rs

/// Generate Rust server from OpenAPI spec
pub fn generate_rust_server(
    spec: &OpenApiSpec,
    output_dir: &Path,
) -> Result<()> {
    // Generate Cargo.toml
    generate_cargo_toml(output_dir)?;

    // Generate models from schemas
    let models = generate_models(&spec.components.schemas)?;
    fs::write(output_dir.join("src/models.rs"), models)?;

    // Generate route handlers
    let routes = generate_routes(&spec.paths)?;
    fs::write(output_dir.join("src/routes.rs"), routes)?;

    // Generate main.rs
    let main = generate_main(&spec)?;
    fs::write(output_dir.join("src/main.rs"), main)?;

    Ok(())
}

/// Generate Rust tests from fixtures + OpenAPI
pub fn generate_rust_tests(
    fixtures: Vec<Fixture>,
    spec: &OpenApiSpec,
    output_dir: &Path,
) -> Result<()> {
    // Group by category
    let by_category = group_by_category(&fixtures);

    for (category, fixtures) in by_category {
        let test_code = generate_test_file(category, fixtures, spec)?;
        let filename = format!("{}_tests.rs", category);
        fs::write(output_dir.join(filename), test_code)?;
    }

    // Generate common test utilities
    generate_test_common(output_dir)?;

    Ok(())
}
```

## Generated Code Examples

### Rust: Generated Route Handler

```rust
// e2e/rust/app/src/routes.rs (generated)
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetUserParams {
    pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct GetUserQuery {
    #[serde(default)]
    pub include_profile: bool,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

/// GET /users/{user_id}
/// Generated from: query_params/01_basic_param.json
pub async fn get_user(
    Path(params): Path<GetUserParams>,
    Query(query): Query<GetUserQuery>,
) -> Result<Json<User>, StatusCode> {
    // Echo back the validated params
    Ok(Json(User {
        id: params.user_id,
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    }))
}
```

### Rust: Generated Test

```rust
// e2e/rust/tests/query_params_tests.rs (generated)
use common::TestClient;

#[tokio::test]
async fn test_01_basic_param() {
    let client = TestClient::new().await;

    // From fixture: query_params/01_basic_param.json
    let response = client
        .get("/users/42")
        .query(&[("include_profile", "true")])
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["id"], 42);
}

#[tokio::test]
async fn test_02_required_string_missing() {
    let client = TestClient::new().await;

    // From fixture: query_params/02_required_string_missing.json
    let response = client
        .get("/users")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 422);

    let body: serde_json::Value = response.json().await.unwrap();
    let errors = body["detail"].as_array().unwrap();
    assert_eq!(errors[0]["type"], "validation_error");
    assert_eq!(errors[0]["loc"], ["query", "user_id"]);
}
```

### Python: Generated Route Handler

```python
# e2e/python/app/routes.py (generated)
from fastapi import APIRouter, Query
from pydantic import BaseModel

router = APIRouter()

class User(BaseModel):
    id: int
    name: str
    email: str

@router.get("/users/{user_id}")
async def get_user(
    user_id: int,
    include_profile: bool = Query(False)
) -> User:
    """
    Generated from: query_params/01_basic_param.json
    """
    return User(
        id=user_id,
        name="Test User",
        email="test@example.com"
    )
```

### TypeScript: Generated Route Handler

```typescript
// e2e/typescript/app/src/routes.ts (generated)
import { FastifyInstance } from 'fastify';
import { z } from 'zod';

const UserSchema = z.object({
  id: z.number(),
  name: z.string(),
  email: z.string().email()
});

export function registerRoutes(app: FastifyInstance) {
  app.get('/users/:user_id', {
    schema: {
      params: z.object({
        user_id: z.number()
      }),
      querystring: z.object({
        include_profile: z.boolean().default(false)
      }),
      response: {
        200: UserSchema
      }
    }
  }, async (request, reply) => {
    // Generated from: query_params/01_basic_param.json
    return {
      id: request.params.user_id,
      name: "Test User",
      email: "test@example.com"
    };
  });
}
```

## Taskfile Commands

```yaml
# Taskfile.yaml additions

generate:
  desc: "Generate all e2e tests and apps"
  deps: [generate:openapi, generate:rust, generate:python, generate:typescript]

generate:openapi:
  desc: "Generate OpenAPI specs from fixtures"
  cmds:
    - cargo run -p spikard-codegen -- fixtures-to-openapi \
        --fixtures testing_data/ \
        --output openapi-specs/full.yaml
    - |
      for category in query_params path_params json_bodies headers cookies; do
        cargo run -p spikard-codegen -- fixtures-to-openapi \
          --fixtures testing_data/$category/ \
          --output openapi-specs/$category.yaml
      done

generate:rust:
  desc: "Generate Rust e2e tests and app"
  deps: [generate:openapi]
  cmds:
    - rm -rf e2e/rust
    - cargo run -p spikard-codegen -- openapi-to-rust \
        --spec openapi-specs/full.yaml \
        --fixtures testing_data/ \
        --output e2e/rust/app \
        --tests e2e/rust/tests
    - cd e2e/rust/app && cargo fmt

generate:python:
  desc: "Generate Python e2e tests and app"
  deps: [generate:openapi]
  cmds:
    - rm -rf e2e/python
    - cargo run -p spikard-codegen -- openapi-to-python \
        --spec openapi-specs/full.yaml \
        --fixtures testing_data/ \
        --output e2e/python/app \
        --tests e2e/python/tests
    - cd e2e/python && ruff format .

generate:typescript:
  desc: "Generate TypeScript e2e tests and app"
  deps: [generate:openapi]
  cmds:
    - rm -rf e2e/typescript
    - cargo run -p spikard-codegen -- openapi-to-typescript \
        --spec openapi-specs/full.yaml \
        --fixtures testing_data/ \
        --output e2e/typescript/app \
        --tests e2e/typescript/tests
    - cd e2e/typescript && npm run format

test:e2e:
  desc: "Run all e2e tests"
  deps: [test:e2e:rust, test:e2e:python, test:e2e:typescript]

test:e2e:rust:
  desc: "Run Rust e2e tests"
  dir: e2e/rust
  cmds:
    - cargo test

test:e2e:python:
  desc: "Run Python e2e tests"
  dir: e2e/python
  cmds:
    - pytest tests/

test:e2e:typescript:
  desc: "Run TypeScript e2e tests"
  dir: e2e/typescript
  cmds:
    - npm test
```

## Implementation Phases

### Phase 1: Fixtures → OpenAPI (Week 1)
- Implement `fixtures_to_openapi()` in `spikard-codegen`
- Handle all fixture formats
- Generate valid OpenAPI 3.1 specs
- Add CLI command: `spikard fixtures-to-openapi`

### Phase 2: OpenAPI → Rust (Week 2)
- Implement Rust server generator
- Implement Rust test generator
- Generate Axum routes with validation
- Generate parametrized tests
- Add CLI command: `spikard openapi-to-rust`

### Phase 3: OpenAPI → Python (Week 3)
- Implement Python server generator
- Implement Python test generator
- Generate FastAPI/Spikard routes
- Generate pytest tests
- Add CLI command: `spikard openapi-to-python`

### Phase 4: OpenAPI → TypeScript (Week 4)
- Implement TypeScript server generator
- Implement TypeScript test generator
- Generate Fastify routes with Zod
- Generate Vitest tests
- Add CLI command: `spikard openapi-to-typescript`

### Phase 5: Integration & CI (Week 5)
- Integrate with Taskfile
- Add to CI pipeline
- Document generation workflow
- Add regeneration checks

## Benefits

1. **Single Source of Truth**: Fixtures drive everything
2. **Consistency**: Same fixtures test all languages
3. **No Manual Maintenance**: Regenerate when fixtures change
4. **OpenAPI as Bridge**: Universal format for code generation
5. **Idiomatic Code**: Generated code follows language conventions
6. **Type Safety**: Full type coverage in generated code

## Related Files

- `crates/spikard-codegen/src/openapi/from_fixtures.rs`
- `crates/spikard-codegen/src/generators/rust_from_openapi.rs`
- `crates/spikard-codegen/src/generators/python_from_openapi.rs`
- `crates/spikard-codegen/src/generators/typescript_from_openapi.rs`
- `Taskfile.yaml` - Generation commands

## Key Takeaway

By treating fixtures as the source of truth and OpenAPI as an intermediate format, we can automatically generate type-safe test applications and comprehensive test suites for any language, ensuring perfect consistency across the entire Spikard ecosystem.
