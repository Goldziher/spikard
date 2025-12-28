# Spikard

A Rust-centric multi-language toolkit for building and validating typed web services across Python, TypeScript, Ruby, PHP, and WebAssembly.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3.10%2B-blue?logo=python)](https://www.python.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.x-blue?logo=typescript)](https://www.typescriptlang.org/)
[![Ruby](https://img.shields.io/badge/ruby-3.2%2B-red?logo=ruby)](https://www.ruby-lang.org/)
[![PHP](https://img.shields.io/badge/php-8.2%2B-purple?logo=php)](https://www.php.net/)

---

## Features

### Core Capabilities

- **Multi-Language Code Generation**: Generate type-safe handlers for Python, TypeScript, Ruby, PHP, and Rust from API specifications
- **Project Scaffolding**: `spikard init` creates new projects with language-specific structure and examples
- **Quality Validation Framework**: Automated syntax, type, and linting checks across all languages
- **Specification Support**:
  - OpenAPI 3.0.x
  - GraphQL SDL
  - AsyncAPI 2.x
  - OpenRPC 1.x
- **Zero-Copy Bindings**: FFI layers for Python (PyO3), Node.js (napi-rs), Ruby (magnus), PHP (ext-php-rs)
- **Tower-HTTP Middleware Stack**: Complete runtime with compression, rate limiting, auth, CORS
- **Fixture-Driven Testing**: Comprehensive test suites using JSON-based test fixtures

### Language Support

| Language | Status | Package Manager | Min Version |
|----------|--------|-----------------|-------------|
| **Python** | ✅ Production | pip/uv | 3.10+ |
| **TypeScript** | ✅ Production | npm/pnpm/yarn | 5.x |
| **Ruby** | ✅ Production | bundler | 3.2+ |
| **PHP** | ✅ Production | Composer | 8.2+ |
| **Rust** | ✅ Production | cargo | 1.75+ |
| **WebAssembly** | ✅ Supported | wasm-pack | Latest |

---

## Quick Start

### Installation

```bash
cargo install spikard-cli
```

Or clone and build from source:
```bash
git clone https://github.com/spikard/spikard.git
cd spikard
task setup    # Install all dependencies
task build    # Build all language bindings
```

### Create Your First Project

```bash
# Create a new Python project
spikard init --name my_api --language python

cd my_api
uv sync                    # Install dependencies
python -m my_api.app       # Start the app
```

### Generate from API Specification

```bash
# Generate Python handlers from OpenAPI spec
spikard codegen \
  --spec openapi.json \
  --language python \
  --output ./generated

# Or from GraphQL schema
spikard codegen \
  --spec schema.graphql \
  --language typescript \
  --output ./src/generated
```

---

## Spikard Init Command

The `spikard init` command scaffolds new projects with language-specific structure:

```bash
spikard init --name PROJECT_NAME --language LANGUAGE [--schema PATH]
```

### Supported Languages

#### Python
```bash
spikard init --name user_service --language python
```
Creates a uv-based project with handlers, models, and tests.

#### TypeScript
```bash
spikard init --name user-service --language typescript
```
Creates a pnpm project with strict TypeScript and Biome linting.

#### Ruby
```bash
spikard init --name UserService --language ruby
```
Creates a bundler-managed project with RBS type signatures.

#### PHP
```bash
spikard init --name UserService --language php
```
Creates a Composer project with PHPStan type checking.

#### Rust
```bash
spikard init --name user_service --language rust
```
Creates a Cargo project with full type safety.

### With Existing Schema

```bash
spikard init \
  --name my_api \
  --language python \
  --schema ./openapi.json
```

See [Init Command Guide](docs/init-command.md) for complete documentation.

---

## Examples

### Python

```python
# src/my_api/handlers.py
from spikard import Handler, Request, Response
from .models import User

@Handler("/users/{id}")
async def get_user(request: Request) -> Response:
    user_id = request.path_params["id"]
    user = User(id=user_id, name="Alice")
    return Response(user.to_dict())
```

### TypeScript

```typescript
// src/handlers.ts
import { Handler, Request, Response } from "spikard";

export const getUser: Handler = async (request: Request) => {
    const userId = request.params.id;
    const user = { id: userId, name: "Alice" };
    return new Response(user);
};
```

### Ruby

```ruby
# lib/my_api/handlers.rb
class GetUserHandler
  def initialize(request)
    @request = request
  end

  def call
    user_id = @request.path_params["id"]
    { id: user_id, name: "Alice" }
  end
end
```

### PHP

```php
<?php declare(strict_types=1);

namespace MyApi;

class Handlers {
    public function getUser(Request $request): Response {
        $userId = $request->pathParams['id'];
        $user = ['id' => $userId, 'name' => 'Alice'];
        return new Response($user);
    }
}
```

### Rust

```rust
// src/handlers.rs
use spikard::{Handler, Request, Response};

pub async fn get_user(request: Request) -> Response {
    let user_id = request.path_params.get("id");
    Response::json(json!({
        "id": user_id,
        "name": "Alice"
    }))
}
```

---

## Project Structure

```
spikard/
├── crates/                                 # Rust workspace
│   ├── spikard-cli/                       # CLI tool & code generation
│   │   ├── src/codegen/                   # Code generators
│   │   │   ├── common/                    # Shared utilities
│   │   │   ├── quality/                   # Quality validation
│   │   │   ├── graphql/                   # GraphQL generators
│   │   │   ├── openapi.rs                 # OpenAPI generators
│   │   │   └── init/                      # Project scaffolding
│   │   └── tests/                         # Integration tests
│   ├── spikard/                           # Core library
│   ├── spikard-http/                      # HTTP runtime & middleware
│   ├── spikard-py/                        # Python bindings (PyO3)
│   ├── spikard-node/                      # Node.js bindings (napi-rs)
│   ├── spikard-rb/                        # Ruby bindings (magnus)
│   ├── spikard-php/                       # PHP bindings (ext-php-rs)
│   └── spikard-wasm/                      # WebAssembly bindings
│
├── docs/                                   # Documentation
│   ├── adr/                               # Architecture Decision Records
│   ├── init-command.md                    # Init command guide
│   ├── codegen-modernization.md           # Codegen architecture
│   └── guides/                            # Language-specific guides
│
├── testing_data/                          # Fixture-driven test data
│   ├── graphql/                           # GraphQL fixtures
│   ├── openapi/                           # OpenAPI fixtures
│   ├── headers/                           # Header validation fixtures
│   └── cookies/                           # Cookie validation fixtures
│
├── examples/                              # Language examples
├── packages/                              # Multi-language packages
└── Taskfile.yaml                          # Task automation
```

---

## Code Generation

### Supported Specifications

| Spec | Status | Generators | Test Coverage |
|------|--------|-----------|---------------|
| OpenAPI 3.0.x | ✅ | Python, TypeScript, Ruby, PHP | 95%+ |
| GraphQL SDL | ✅ | Python, TypeScript, Ruby, PHP, Rust | 95%+ |
| AsyncAPI 2.x | ✅ | Python, TypeScript, Ruby, PHP | 85%+ |
| OpenRPC 1.x | ✅ | Python, TypeScript, Ruby, PHP | 80%+ |

### Quality Validation

Generated code is automatically validated:

```bash
# Syntax validation
python3 -m py_compile      # Python
tsc --noEmit               # TypeScript
php -l                     # PHP
ruby -c                    # Ruby
cargo check                # Rust

# Type checking
mypy --strict              # Python
tsc --strict               # TypeScript
phpstan --level=max        # PHP
steep check                # Ruby

# Linting
ruff check                 # Python
biome check                # TypeScript
php-cs-fixer               # PHP
rubocop                    # Ruby
```

---

## Development

### Local Setup

```bash
# Install dependencies for all languages
task setup

# Build all language bindings
task build

# Run all tests
task test

# Format code
task format

# Lint with all language tools
task lint
```

### Building Specific Language Bindings

```bash
task build:python    # Build Python bindings
task build:node      # Build Node.js bindings
task build:ruby      # Build Ruby bindings
task build:php       # Build PHP bindings
task build:wasm      # Build WebAssembly
```

### Running Tests

```bash
# All tests across all languages
task test

# Language-specific
task test:rust       # Rust tests
task test:python     # Python tests
task test:js         # TypeScript/Node.js tests
task test:ruby       # Ruby tests
task test:php        # PHP tests
```

---

## Documentation

### Getting Started
- [Init Command Guide](docs/init-command.md) - Project scaffolding
- [Code Generation Guide](docs/guides/code-generation.md) - Generate from specifications

### Architecture
- [ADR-0001: Architecture & Principles](docs/adr/0001-architecture-and-principles.md)
- [ADR-0002: Runtime & Middleware](docs/adr/0002-runtime-and-middleware.md)
- [ADR-0004: Code Generation](docs/adr/0004-code-generation.md)

### Advanced
- [Codegen Modernization](docs/codegen-modernization.md) - 3-phase architecture refactor
- [Quality Framework](docs/codegen-modernization.md#phase-2-quality-validation-framework)
- [Shared Utilities](docs/codegen-modernization.md#phase-1-foundation--shared-utilities)

### Language Bindings
- [Python Bindings (PyO3)](docs/bindings/python.md)
- [TypeScript Bindings (napi-rs)](docs/bindings/typescript.md)
- [Ruby Bindings (magnus)](docs/bindings/ruby.md)
- [PHP Bindings (ext-php-rs)](docs/bindings/php.md)
- [WebAssembly (wasm-bindgen)](docs/bindings/wasm.md)

---

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for all releases and changes.

### Recent Updates
- ✅ `spikard init` command for project scaffolding (all 5 languages)
- ✅ Quality validation framework with language-specific tools
- ✅ Shared codegen utilities (case conversion, escaping, sanitization)
- ✅ Critical bug fixes in GraphQL, OpenAPI, OpenRPC, AsyncAPI generators
- ✅ Full type safety across all language bindings

---

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Code Quality Standards
- Rust: clippy -D warnings, 95%+ test coverage
- Python: mypy --strict, 95%+ test coverage
- TypeScript: Strict mode, 80%+ test coverage
- Ruby: Steep, 80%+ test coverage
- PHP: PHPStan level max, 80%+ test coverage

### Pull Request Process
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `task lint` and `task test`
5. Update CHANGELOG.md
6. Submit a PR with clear description

---

## Architecture Highlights

### Thin Binding Pattern
All language bindings follow the "thin binding" pattern:
- Heavy lifting in Rust core (`crates/spikard`)
- Bindings translate to/from language types
- No business logic duplication
- Consistent behavior across all platforms

### Zero-Copy Serialization
- Direct PyO3 type construction (no JSON round-trips)
- serde-based serialization
- msgspec integration for Python
- 30-40% performance improvement vs. naive JSON conversion

### Quality-First Code Generation
- Shared utilities eliminate code duplication
- Automatic quality validation (syntax, types, linting)
- Fixture-driven test coverage
- 95%+ test coverage on core generators

---

## Performance

### Benchmarks

Generation performance (source → output):

| Spec Type | Size | Python | TypeScript | Ruby | PHP | Time |
|-----------|------|--------|-----------|------|-----|------|
| OpenAPI | 100 endpoints | ✅ | ✅ | ✅ | ✅ | <500ms |
| GraphQL | 50 types | ✅ | ✅ | ✅ | ✅ | <300ms |
| AsyncAPI | 30 channels | ✅ | ✅ | ✅ | ✅ | <400ms |

Runtime overhead from bindings: <1ms per request

---

## License

Licensed under the MIT License. See LICENSE file for details.

---

## Support & Community

- **Documentation**: https://docs.spikard.dev
- **GitHub Issues**: https://github.com/spikard/spikard/issues
- **Discussions**: https://github.com/spikard/spikard/discussions
- **Twitter**: [@spikard_dev](https://twitter.com/spikard_dev)

---

## Acknowledgments

Built with:
- 🦀 Rust & tokio for core async runtime
- 🐍 PyO3 for Python bindings
- 📘 napi-rs for Node.js bindings
- 💎 magnus for Ruby bindings
- 🐘 ext-php-rs for PHP bindings
- 🕸️ wasm-bindgen for WebAssembly

Special thanks to the Rust, Python, TypeScript, Ruby, and PHP communities.
