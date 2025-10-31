# Test Generator

**Internal tool** for generating test infrastructure from fixtures.

## Purpose

This tool generates:
- OpenAPI specifications from test fixtures
- Test suites for Rust, Python, and TypeScript
- Minimal test applications to validate scenarios

**Not published** - Internal development tooling only.

## Usage

### Generate OpenAPI Spec

```bash
cargo run --manifest-path tools/test-generator/Cargo.toml -- openapi \
  --fixtures testing_data \
  --output openapi-specs/full.yaml \
  --title "Spikard Test API" \
  --version "1.0.0"
```

### Generate Test Suites

```bash
# Rust
cargo run --manifest-path tools/test-generator/Cargo.toml -- tests \
  --lang rust \
  --fixtures testing_data \
  --output e2e/rust

# Python
cargo run --manifest-path tools/test-generator/Cargo.toml -- tests \
  --lang python \
  --fixtures testing_data \
  --output e2e/python

# TypeScript
cargo run --manifest-path tools/test-generator/Cargo.toml -- tests \
  --lang typescript \
  --fixtures testing_data \
  --output e2e/typescript
```

### Using Taskfile (Recommended)

```bash
# Generate all tests
task generate:tests

# Generate for specific language
task generate:tests:rust
task generate:tests:python
task generate:tests:typescript

# Generate OpenAPI spec
task generate:openapi

# Clean all generated code
task clean:generated
```

## What Gets Generated

- `e2e/<lang>/app/` - Minimal test applications
- `e2e/<lang>/tests/` - Parametrized test suites
- `openapi-specs/` - OpenAPI 3.1 specifications

All generated code can be safely deleted and regenerated.

## Architecture

See [docs/design/codegen-strategy.md](../../docs/design/codegen-strategy.md) for complete design.
