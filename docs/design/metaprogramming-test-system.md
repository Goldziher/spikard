# Metaprogramming Test System

**Date**: 2025-01-30
**Status**: 🟡 Draft
**Related**: [02-testing-strategy.md](./02-testing-strategy.md), [09-unified-config-format.md](./09-unified-config-format.md)

## Executive Summary

A metaprogramming system that dynamically generates test functions, route handlers, and test applications from JSON/YAML fixtures, eliminating boilerplate and ensuring perfect fixture-to-implementation alignment across Python, TypeScript, and Rust.

## Goals

1. **Zero Boilerplate**: Generate tests and handlers automatically from fixtures
2. **Type Safety**: Leverage language type systems for generated code
3. **Perfect Alignment**: Guarantee test applications match fixture expectations
4. **Cross-Language**: Share generation patterns across Python/TypeScript/Rust
5. **Configuration-Driven**: Define entire servers via YAML/JSON schemas

## Non-Goals

- Runtime code generation (prefer build-time or import-time)
- Supporting arbitrary custom handler logic (focus on validation/routing)
- Replacing hand-written tests entirely (metaprogramming for repetitive scenarios)

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Fixture Files (JSON)                     │
│  testing_data/query_params/01_basic_param.json              │
└────────────────────┬────────────────────────────────────────┘
                     │
          ┌──────────┴──────────┐
          │                     │
┌─────────▼──────────┐  ┌──────▼─────────┐
│  Test Generator    │  │ App Generator  │
│  (Python/TS/Rust)  │  │ (Python/TS)    │
└─────────┬──────────┘  └──────┬─────────┘
          │                     │
   ┌──────┴──────┐      ┌──────┴──────┐
   │             │      │             │
┌──▼──┐    ┌───▼───┐ ┌─▼──┐    ┌───▼───┐
│ Pytest│    │ Vitest│ │ App│    │FastAPI│
│ Tests │    │ Tests │ │ Cfg│    │  App  │
└───────┘    └───────┘ └────┘    └───────┘
```

## Design: Python Metaprogramming

### 1. Fixture-Driven Route Generation

```python
# testing_data/meta/fixture_loader.py
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List
import json

@dataclass
class FixtureRoute:
    """Represents a route derived from fixture handler spec."""
    path: str
    method: str
    schema: Dict[str, Any]
    response_schema: Dict[str, Any]
    handler_name: str

    @classmethod
    def from_fixture(cls, fixture: Dict[str, Any]) -> "FixtureRoute":
        handler = fixture["handler"]
        return cls(
            path=handler["route"],
            method=handler["method"],
            schema=handler.get("body_schema") or handler.get("parameters", {}),
            response_schema=fixture["expected_response"],
            handler_name=f"handler_{fixture['name']}"
        )

def discover_routes(fixture_dir: Path) -> List[FixtureRoute]:
    """Scan fixtures and extract unique routes."""
    routes = {}
    for fixture_file in fixture_dir.rglob("*.json"):
        if fixture_file.name.startswith("00-"):
            continue

        with open(fixture_file) as f:
            fixture = json.load(f)

        route = FixtureRoute.from_fixture(fixture)
        route_key = (route.path, route.method)

        # Merge schemas if same route appears in multiple fixtures
        if route_key in routes:
            routes[route_key].merge_schema(route.schema)
        else:
            routes[route_key] = route

    return list(routes.values())
```

### 2. Dynamic App Generation

```python
# packages/python/tests/fixture_app.py
from spikard import Spikard, Request, Response
from spikard.validation import validate_request
from testing_data.meta.fixture_loader import discover_routes
from pathlib import Path
import msgspec

def create_fixture_app() -> Spikard:
    """Generate Spikard app with all routes from fixtures."""
    app = Spikard()
    fixtures_path = Path(__file__).parent.parent.parent / "testing_data"

    # Discover all routes from fixtures
    routes = discover_routes(fixtures_path)

    # Generate handlers dynamically
    for route in routes:
        handler = create_handler(route)
        app.route(route.path, methods=[route.method])(handler)

    return app

def create_handler(route: FixtureRoute):
    """Create a handler function for a given route."""
    schema = route.schema
    response_schema = route.response_schema

    async def handler(request: Request) -> Response:
        # Extract and validate parameters based on schema
        params = {}

        if "parameters" in schema:
            if "query" in schema["parameters"]:
                params["query"] = request.query_params
            if "path" in schema["parameters"]:
                params["path"] = request.path_params
            if "headers" in schema["parameters"]:
                params["headers"] = dict(request.headers)
            if "cookies" in schema["parameters"]:
                params["cookies"] = request.cookies

        if "body_schema" in schema:
            body = await request.json()
            params["body"] = body

        # Validation happens in Rust layer automatically
        # Handler just echoes back the validated params
        return Response(
            status_code=response_schema.get("status_code", 200),
            body=params
        )

    handler.__name__ = route.handler_name
    return handler
```

### 3. Parametrized Test Generation

```python
# packages/python/tests/test_all_fixtures.py
import pytest
from pathlib import Path
from httpx import AsyncClient
import json
from .fixture_app import create_fixture_app

def discover_fixtures():
    """Find all fixture files."""
    fixtures_dir = Path(__file__).parent.parent.parent / "testing_data"
    for fixture_file in fixtures_dir.rglob("[0-9]*.json"):
        yield pytest.param(
            fixture_file,
            id=f"{fixture_file.parent.name}::{fixture_file.stem}"
        )

@pytest.fixture(scope="session")
def app():
    """Create the fixture-driven test app once per session."""
    return create_fixture_app()

@pytest.fixture
async def client(app):
    """HTTP client for the test app."""
    async with AsyncClient(app=app, base_url="http://test") as ac:
        yield ac

@pytest.mark.parametrize("fixture_file", discover_fixtures())
async def test_fixture(fixture_file: Path, client: AsyncClient):
    """Universal fixture-based test."""
    with open(fixture_file) as f:
        fixture = json.load(f)

    # Build request from fixture
    request_spec = fixture["request"]
    response = await client.request(
        method=request_spec["method"],
        url=request_spec["path"],
        params=request_spec.get("query_params"),
        headers=request_spec.get("headers"),
        cookies=request_spec.get("cookies"),
        json=request_spec.get("body"),
        files=build_multipart(request_spec.get("files"))
    )

    # Assert against expected response
    expected = fixture["expected_response"]
    assert response.status_code == expected["status_code"]

    if "body" in expected:
        actual = response.json()
        assert_equal_recursive(actual, expected["body"])

    if "headers" in expected:
        for header, value in expected["headers"].items():
            assert response.headers.get(header) == value

    if "validation_errors" in expected:
        actual_errors = response.json()["detail"]
        expected_errors = expected["validation_errors"]
        assert_validation_errors_match(actual_errors, expected_errors)

def assert_equal_recursive(actual, expected):
    """Deep equality check with helpful diff messages."""
    if isinstance(expected, dict):
        assert isinstance(actual, dict), f"Expected dict, got {type(actual)}"
        for key, expected_value in expected.items():
            assert key in actual, f"Missing key: {key}"
            assert_equal_recursive(actual[key], expected_value)
    elif isinstance(expected, list):
        assert isinstance(actual, list), f"Expected list, got {type(actual)}"
        assert len(actual) == len(expected), f"Length mismatch: {len(actual)} != {len(expected)}"
        for i, expected_item in enumerate(expected):
            assert_equal_recursive(actual[i], expected_item)
    else:
        assert actual == expected, f"Value mismatch: {actual} != {expected}"

def assert_validation_errors_match(actual_errors, expected_errors):
    """Compare validation error structures."""
    assert len(actual_errors) == len(expected_errors)
    for actual, expected in zip(actual_errors, expected_errors):
        assert actual["type"] == expected["type"]
        assert actual["loc"] == expected["loc"]
        assert actual["msg"] == expected["msg"]
        if "ctx" in expected:
            assert "ctx" in actual
            for key, value in expected["ctx"].items():
                assert actual["ctx"][key] == value
```

## Design: TypeScript Metaprogramming

### 1. Code Generation at Build Time

```typescript
// testing_data/meta/generate-tests.ts
import { readdir, readFile, writeFile } from 'fs/promises';
import { join } from 'path';

interface Fixture {
  name: string;
  request: {
    method: string;
    path: string;
    query_params?: Record<string, string>;
    headers?: Record<string, string>;
    body?: unknown;
  };
  expected_response: {
    status_code: number;
    body?: unknown;
    headers?: Record<string, string>;
  };
}

async function generateTests() {
  const fixturesDir = join(__dirname, '../../testing_data');
  const categories = await readdir(fixturesDir, { withFileTypes: true });

  const testCode: string[] = [
    `import { test, expect } from 'vitest';`,
    `import { createFixtureApp } from './fixture-app';`,
    `import type { Fixture } from './types';`,
    ``,
    `const app = createFixtureApp();`,
    ``
  ];

  for (const category of categories) {
    if (!category.isDirectory()) continue;

    const categoryPath = join(fixturesDir, category.name);
    const files = await readdir(categoryPath);

    for (const file of files) {
      if (!file.match(/^\d+.*\.json$/)) continue;

      const fixturePath = join(categoryPath, file);
      const fixture: Fixture = JSON.parse(
        await readFile(fixturePath, 'utf-8')
      );

      const testName = `${category.name}::${fixture.name}`;
      testCode.push(generateTestCase(fixture, testName));
    }
  }

  await writeFile(
    join(__dirname, '../tests/generated.test.ts'),
    testCode.join('\n')
  );
}

function generateTestCase(fixture: Fixture, testName: string): string {
  return `
test('${testName}', async () => {
  const response = await app.inject({
    method: '${fixture.request.method}',
    url: '${fixture.request.path}',
    query: ${JSON.stringify(fixture.request.query_params)},
    headers: ${JSON.stringify(fixture.request.headers)},
    payload: ${JSON.stringify(fixture.request.body)}
  });

  expect(response.statusCode).toBe(${fixture.expected_response.status_code});
  ${fixture.expected_response.body ? `
  expect(response.json()).toEqual(${JSON.stringify(fixture.expected_response.body, null, 2)});
  ` : ''}
});`;
}

generateTests().catch(console.error);
```

### 2. Runtime App Generation

```typescript
// packages/node/tests/fixture-app.ts
import { Spikard } from '@spikard/node';
import { readdirSync, readFileSync } from 'fs';
import { join } from 'path';
import type { Fixture } from './types';

export function createFixtureApp(): Spikard {
  const app = new Spikard();
  const fixturesPath = join(__dirname, '../../../testing_data');

  // Discover all unique routes
  const routes = discoverRoutes(fixturesPath);

  // Register handlers
  for (const route of routes) {
    app.route(route.path, {
      method: route.method,
      schema: route.schema,
      handler: createHandler(route)
    });
  }

  return app;
}

function discoverRoutes(fixturesPath: string): RouteSpec[] {
  const routes = new Map<string, RouteSpec>();

  const categories = readdirSync(fixturesPath, { withFileTypes: true });
  for (const category of categories) {
    if (!category.isDirectory()) continue;

    const categoryPath = join(fixturesPath, category.name);
    const files = readdirSync(categoryPath);

    for (const file of files) {
      if (!file.match(/^\d+.*\.json$/)) continue;

      const fixture: Fixture = JSON.parse(
        readFileSync(join(categoryPath, file), 'utf-8')
      );

      const routeKey = `${fixture.handler.method}:${fixture.handler.route}`;
      if (!routes.has(routeKey)) {
        routes.set(routeKey, {
          path: fixture.handler.route,
          method: fixture.handler.method,
          schema: fixture.handler.body_schema || fixture.handler.parameters
        });
      }
    }
  }

  return Array.from(routes.values());
}

function createHandler(route: RouteSpec) {
  return async (request: Request) => {
    // Extract validated params (validation done by Rust layer)
    const params: Record<string, unknown> = {};

    if (request.query) params.query = request.query;
    if (request.params) params.path = request.params;
    if (request.headers) params.headers = request.headers;
    if (request.body) params.body = request.body;

    // Echo back validated params
    return { status: 200, body: params };
  };
}
```

## Design: Rust Macro-Based Generation

```rust
// crates/spikard-test-macros/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Generate test functions from fixture directory
///
/// Usage:
/// ```
/// generate_fixture_tests!("testing_data/query_params");
/// ```
#[proc_macro]
pub fn generate_fixture_tests(input: TokenStream) -> TokenStream {
    let fixture_dir = parse_macro_input!(input as LitStr).value();

    let test_functions = discover_and_generate_tests(&fixture_dir);

    let expanded = quote! {
        #(#test_functions)*
    };

    TokenStream::from(expanded)
}

fn discover_and_generate_tests(dir: &str) -> Vec<proc_macro2::TokenStream> {
    let mut tests = Vec::new();

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.extension().map_or(false, |e| e == "json") {
            continue;
        }

        let fixture: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&path).unwrap()
        ).unwrap();

        let test_name = format_ident!("test_{}", fixture["name"].as_str().unwrap());
        let fixture_path = path.to_str().unwrap();

        tests.push(quote! {
            #[tokio::test]
            async fn #test_name() {
                let fixture = load_fixture(#fixture_path);
                let app = create_test_app();

                let response = app
                    .oneshot(build_request(&fixture))
                    .await
                    .unwrap();

                assert_response_matches(&response, &fixture);
            }
        });
    }

    tests
}
```

## Next Steps: Unified Configuration Format

See [09-unified-config-format.md](./09-unified-config-format.md) for the schema-driven server configuration design that extends this metaprogramming approach to full application generation.

## Benefits

1. **DRY Principle**: Single source of truth (fixtures) for tests and implementations
2. **Consistency**: Impossible for test app to diverge from test expectations
3. **Coverage**: Every fixture automatically becomes a test case
4. **Maintainability**: Add new test scenarios by adding fixtures, no code changes
5. **Cross-Language**: Same generation patterns work across Python/TS/Rust

## Implementation Strategy

### Phase 1: Python Prototype (2 weeks)
- Implement `fixture_loader.py` with route discovery
- Build `create_fixture_app()` with dynamic handler generation
- Migrate existing tests to parametrized fixture-driven approach
- Validate 100% fixture coverage

### Phase 2: TypeScript Generation (2 weeks)
- Build-time code generation script
- Runtime app generation for Vitest
- Port Python patterns to TypeScript/Fastify style
- Cross-validate with Python test results

### Phase 3: Rust Macros (2 weeks)
- Procedural macro for test generation
- Integration with existing Rust test suite
- Benchmark-driven validation of zero-cost abstractions

### Phase 4: Configuration-Driven Apps (see doc 09)
- Extend to full server generation from YAML/JSON
- Multi-protocol support (HTTP, gRPC, queues)
- OpenAPI/Protobuf integration

## Testing

- **Meta-tests**: Tests for the test generator itself
- **Fixture validation**: JSON Schema validation for all fixtures
- **Cross-language consistency**: Same fixtures must pass in all bindings
- **Performance**: Generated code should match hand-written performance

## References

- [pytest parametrize](https://docs.pytest.org/en/stable/how-to/parametrize.html)
- [Vitest programmatic API](https://vitest.dev/guide/api.html)
- [Rust proc macros](https://doc.rust-lang.org/reference/procedural-macros.html)
- [msgspec for Python speed](https://jcristharif.com/msgspec/)

## Key Takeaway

Metaprogramming transforms static fixture files into living test suites and test applications, ensuring perfect alignment between specifications and implementations while eliminating boilerplate across all language bindings.
