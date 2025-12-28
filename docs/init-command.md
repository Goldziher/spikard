# Spikard Init Command Guide

## Overview

The `spikard init` command scaffolds new Spikard projects with language-specific structure, dependencies, and example handlers. It supports five programming languages with idiomatic patterns for each ecosystem.

**Quick Start:**
```bash
spikard init --name my_api --language python
```

---

## Installation

The init command is built into `spikard`. Ensure you have the latest version:

```bash
cargo install spikard-cli
# or
spikard --version
```

---

## Supported Languages

| Language | Scaffolder | Package Manager | Init Time |
|----------|-----------|-----------------|-----------|
| Python | `PythonScaffolder` | pip/uv | ~2s |
| TypeScript | `TypeScriptScaffolder` | npm/pnpm/yarn | ~3s |
| Ruby | `RubyScaffolder` | bundler (gem) | ~2s |
| PHP | `PhpScaffolder` | Composer | ~2s |
| Rust | `RustScaffolder` | cargo | ~3s |

---

## Command Syntax

### Basic Usage
```bash
spikard init --name PROJECT_NAME --language LANGUAGE
```

### With Optional Schema
```bash
spikard init \
  --name my_api \
  --language python \
  --schema /path/to/openapi.json
```

### Directory Specification
```bash
spikard init \
  --name my_api \
  --language typescript \
  --project-dir ./projects
```

## Command Options

| Option | Short | Required | Description | Example |
|--------|-------|----------|-------------|---------|
| `--name` | `-n` | Yes | Project name (identifier-safe) | `my_api` |
| `--language` | `-l` | Yes | Target language | `python`, `typescript`, `ruby`, `php`, `rust` |
| `--project-dir` | `-d` | No | Directory to create project in | `./projects` |
| `--schema` | `-s` | No | Path to existing API schema | `./openapi.json` |
| `--help` | `-h` | No | Show help message | — |

---

## Usage Examples

### Python Project

```bash
spikard init --name user_service --language python
```

Creates:
```
user_service/
├── pyproject.toml              # Project metadata & dependencies
├── uv.lock                      # Locked dependencies
├── .python-version              # Python 3.10+
├── src/
│   └── user_service/
│       ├── __init__.py
│       ├── app.py              # Main application
│       ├── handlers.py          # Example handlers
│       └── models.py            # Data models
├── tests/
│   ├── __init__.py
│   └── test_handlers.py         # Example tests
├── README.md
└── .gitignore
```

**Next Steps:**
```bash
cd user_service
uv sync                         # Install dependencies
python -m user_service.app      # Run the app
uv run pytest tests/            # Run tests
```

### TypeScript Project

```bash
spikard init --name user_service --language typescript
```

Creates:
```
user_service/
├── package.json                # Project metadata & dependencies
├── pnpm-lock.yaml              # Locked dependencies
├── tsconfig.json               # TypeScript configuration
├── biome.json                  # Code quality config
├── src/
│   ├── app.ts                  # Main application
│   ├── handlers.ts             # Example handlers
│   └── models.ts               # Type definitions
├── tests/
│   └── handlers.spec.ts        # Example tests
├── dist/                       # Compiled output (after build)
├── README.md
└── .gitignore
```

**Next Steps:**
```bash
cd user_service
pnpm install                    # Install dependencies
pnpm run dev                    # Start development server
pnpm test                       # Run tests
pnpm build                      # Build for production
```

### Ruby Project

```bash
spikard init --name user_service --language ruby
```

Creates:
```
user_service/
├── Gemfile                     # Dependency specification
├── Gemfile.lock                # Locked dependencies
├── .ruby-version               # Ruby 3.2+
├── lib/
│   └── user_service/
│       ├── version.rb          # Version constant
│       ├── app.rb              # Main application
│       ├── handlers.rb         # Example handlers
│       └── models.rb           # Data models
├── sig/
│   └── user_service.rbs        # Type signatures
├── spec/
│   ├── spec_helper.rb
│   └── user_service_spec.rb    # Example tests
├── README.md
└── .gitignore
```

**Next Steps:**
```bash
cd user_service
bundle install                  # Install dependencies
bundle exec ruby lib/user_service/app.rb  # Run the app
bundle exec rspec spec/         # Run tests
```

### PHP Project

```bash
spikard init --name UserService --language php
```

Creates:
```
UserService/
├── composer.json               # Project metadata & dependencies
├── composer.lock               # Locked dependencies
├── .php-version                # PHP 8.2+
├── src/
│   └── UserService/
│       ├── App.php             # Main application
│       ├── Handlers.php        # Example handlers
│       └── Models.php          # Data models
├── tests/
│   ├── bootstrap.php
│   └── HandlersTest.php        # Example tests
├── phpstan.neon                # Type checking config
├── .php-cs-fixer.php           # Code style config
├── README.md
└── .gitignore
```

**Next Steps:**
```bash
cd UserService
composer install                # Install dependencies
php src/UserService/App.php     # Run the app
composer test                   # Run tests (PHPUnit)
```

### Rust Project

```bash
spikard init --name user_service --language rust
```

Creates:
```
user_service/
├── Cargo.toml                  # Project manifest
├── Cargo.lock                  # Locked dependencies
├── src/
│   ├── main.rs                 # Binary entry point
│   ├── lib.rs                  # Library root
│   ├── handlers.rs             # Example handlers
│   └── models.rs               # Data models
├── tests/
│   └── integration_test.rs      # Integration tests
├── examples/
│   └── basic.rs                # Example usage
├── README.md
└── .gitignore
```

**Next Steps:**
```bash
cd user_service
cargo build                     # Build the project
cargo run                       # Run the application
cargo test                      # Run tests
cargo doc --open               # View documentation
```

---

## Project Names & Conventions

The init command validates project names according to language conventions:

### Python
- Must be a valid Python identifier: `user_service`, `api_v2`
- Conventionally snake_case
- Converted to snake_case if mixed case provided
- Examples: ✅ `user_service`, ✅ `my_api`, ❌ `User-Service`

### TypeScript/JavaScript
- Must be a valid npm package name
- Conventionally lowercase with hyphens: `user-service`, `api-v2`
- Converted to kebab-case if snake_case provided
- Examples: ✅ `user-service`, ✅ `my-api`, ✅ `api`

### Ruby
- Must be a valid constant: `UserService`, `ApiV2`
- Conventionally PascalCase
- Converted to PascalCase if snake_case provided
- Examples: ✅ `UserService`, ✅ `MyApi`, ❌ `user-service`

### PHP
- Must be a valid class name: `UserService`, `ApiV2`
- Conventionally PascalCase (PSR-4 compliance)
- Converted to PascalCase if snake_case provided
- Examples: ✅ `UserService`, ✅ `MyApi`, ❌ `user_service` (converted to UserService)

### Rust
- Must be a valid crate name: `user_service`, `api_v2`
- Conventionally snake_case
- Converted to snake_case if mixed case provided
- Examples: ✅ `user_service`, ✅ `my_api`, ❌ `UserService` (converted to user_service)

---

## Project Structure Documentation

### Common Elements (All Languages)

#### README.md
Auto-generated with:
- Project description
- Quick start instructions
- Available commands
- Basic usage examples

#### .gitignore
Language-specific ignores:
- Build artifacts
- Dependency directories
- IDE configuration
- Environment files

#### Example Handlers
Each scaffolder includes:
- **GET /health** - Health check endpoint
- **POST /echo** - Echo request body (demonstration)
- Type definitions for request/response
- Error handling examples

#### Example Tests
Each scaffolder includes:
- Health check test
- Echo handler test
- Pattern for adding more tests
- Test configuration

### Language-Specific Structure

#### Python Projects
```
pyproject.toml          # PEP 517 standard format
├── [project]          # Project metadata
├── [build-system]     # uv/pip configuration
├── [tool.mypy]        # Type checking config
├── [tool.ruff]        # Linting config
└── [tool.pytest]      # Testing config

src/package/
├── __init__.py        # Package initialization
├── app.py             # Entry point & routing
├── handlers.py        # Request handlers
├── models.py          # Data models (msgspec.Struct)
└── errors.py          # Error definitions
```

#### TypeScript Projects
```
tsconfig.json           # TypeScript strict mode
├── compilerOptions
│   ├── strict: true
│   ├── noUncheckedIndexedAccess: true
│   └── exactOptionalPropertyTypes: true
└── include: ["src/**/*"]

biome.json             # Linting & formatting
├── linter { rules }
└── formatter { options }

src/
├── app.ts             # Express/Fastify app setup
├── handlers.ts        # Route handlers
└── models.ts          # Types (strict TypeScript)
```

#### Ruby Projects
```
Gemfile                 # Dependency specification
├── source "https://rubygems.org"
├── gem "spikard"
├── gem "rspec", groups: [:development, :test]
└── gem "rubocop", groups: [:development]

.ruby-version           # Ruby version (3.2+)

sig/                    # Type signatures (RBS)
├── project.rbs        # Type definitions
└── generated/         # Generated signatures

lib/package/
├── version.rb         # Version constant
├── app.rb             # Entry point
├── handlers.rb        # Handlers
└── models.rb          # Models
```

#### PHP Projects
```
composer.json           # PSR-4 autoloading
├── name: "vendor/package"
├── autoload { psr-4 }
├── require { php-version }
└── require-dev { dev-tools }

phpstan.neon            # PHPStan config
├── level: max
└── paths

src/Namespace/
├── App.php            # Main application (strict_types=1)
├── Handlers.php       # Handlers (typed properties/returns)
└── Models.php         # Models (typed)

tests/
├── bootstrap.php      # PHPUnit bootstrap
└── *Test.php          # Test classes
```

#### Rust Projects
```
Cargo.toml              # Package manifest
├── [package]
├── [dependencies]
└── [dev-dependencies]

src/
├── lib.rs             # Library root (if library)
├── main.rs            # Binary entry point
├── handlers.rs        # Handler implementations
└── models.rs          # Data structures
```

---

## Schema Integration

The init command can optionally integrate an existing API schema:

```bash
spikard init \
  --name my_api \
  --language python \
  --schema ./openapi.json
```

When a schema is provided:

1. **Validation**: Schema is validated against OpenAPI 3.0 spec
2. **Integration**: Schema is copied to `schemas/` directory
3. **Documentation**: Handler stubs are generated from schema
4. **Next Steps**: Guidance on code generation is provided

**Supported Schema Formats:**
- OpenAPI 3.0.x (JSON or YAML)
- GraphQL SDL (`.graphql` files)
- AsyncAPI 2.x (future enhancement)
- OpenRPC 1.x (future enhancement)

**Generated Files:**
```
my_api/
├── schemas/
│   └── openapi.json         # Your uploaded schema
├── handlers.py              # Auto-stubs for each endpoint
├── models.py                # Models extracted from schema
└── README.md                # Updated with schema info
```

---

## Validation Rules

### Project Name Validation

The init command validates project names:

| Rule | Applies To | Error Example |
|------|-----------|----------------|
| Valid identifier | All languages | `123invalid` → InvalidProjectName |
| Not reserved keyword | Python, Ruby, PHP | `class` → InvalidProjectName |
| ASCII alphanumeric + `_-` | All languages | `café_api` → InvalidProjectName |
| 3-50 characters | All languages | `a` → InvalidProjectName |

### Directory Validation

```bash
# Creates: ./my_api/
spikard init --name my_api --language python
# ✅ OK if ./my_api/ doesn't exist

# ❌ Fails if ./my_api/ already exists
spikard init --name my_api --language python
# DirectoryAlreadyExists: Directory './my_api' already exists

# ✅ OK with different directory
spikard init --name my_api --language python --project-dir ./projects
# Creates: ./projects/my_api/
```

### Schema Path Validation

```bash
# ✅ OK if file exists
spikard init --name my_api --language python --schema ./openapi.json

# ❌ Fails if file doesn't exist
spikard init --name my_api --language python --schema ./missing.json
# SchemaPathNotFound: Schema file not found: ./missing.json
```

---

## Error Handling

### Common Errors & Solutions

#### `InvalidProjectName: "my-api" is not a valid Python identifier`
- Python projects must use snake_case
- Solution: Use `my_api` instead of `my-api`

#### `DirectoryAlreadyExists: Directory './my_api' already exists`
- The project directory already exists
- Solution: Use a different name or remove the existing directory

#### `SchemaPathNotFound: Schema file not found`
- The provided schema file doesn't exist
- Solution: Check the path and try again

#### `LanguageNotSupported: Python is not yet supported`
- This language doesn't support init yet (shouldn't happen with current release)
- Solution: Choose from: python, typescript, ruby, php, rust

---

## Next Steps Guidance

After initialization, the command provides language-specific guidance:

### Python
```
✅ Project created: user_service

Next steps:
  1. cd user_service
  2. uv sync                 # Install dependencies
  3. python -m user_service.app  # Run the application
  4. uv run pytest tests/    # Run tests

Documentation:
  - Project structure: README.md
  - Type checking: mypy src/
  - Code quality: ruff check src/

For more info: https://docs.spikard.dev
```

### TypeScript
```
✅ Project created: user-service

Next steps:
  1. cd user-service
  2. pnpm install           # Install dependencies
  3. pnpm run dev           # Start dev server
  4. pnpm test              # Run tests

Documentation:
  - Project structure: README.md
  - Build: pnpm build
  - Type checking: pnpm run type-check

For more info: https://docs.spikard.dev
```

### Ruby
```
✅ Project created: UserService

Next steps:
  1. cd UserService
  2. bundle install         # Install dependencies
  3. bundle exec ruby lib/user_service/app.rb
  4. bundle exec rspec spec/  # Run tests

Documentation:
  - Project structure: README.md
  - Type checking: bundle exec steep check
  - Code quality: bundle exec rubocop lib/

For more info: https://docs.spikard.dev
```

### PHP
```
✅ Project created: UserService

Next steps:
  1. cd UserService
  2. composer install       # Install dependencies
  3. php src/UserService/App.php
  4. composer test          # Run tests

Documentation:
  - Project structure: README.md
  - Type checking: vendor/bin/phpstan analyze
  - Code quality: vendor/bin/php-cs-fixer fix src/

For more info: https://docs.spikard.dev
```

### Rust
```
✅ Project created: user_service

Next steps:
  1. cd user_service
  2. cargo build            # Build the project
  3. cargo run              # Run the application
  4. cargo test             # Run tests

Documentation:
  - Project structure: README.md
  - Documentation: cargo doc --open
  - Code quality: cargo clippy

For more info: https://docs.spikard.dev
```

---

## Development Workflow

### Adding New Handlers

#### Python
```python
# src/user_service/handlers.py
from spikard import Handler, Request, Response

@Handler("/api/users/{id}")
async def get_user(request: Request) -> Response:
    user_id = request.path_params["id"]
    return Response({"id": user_id, "name": "Alice"})
```

#### TypeScript
```typescript
// src/handlers.ts
import { Handler, Request, Response } from "spikard";

export const getUser: Handler = async (request: Request) => {
    const userId = request.params.id;
    return new Response({ id: userId, name: "Alice" });
};
```

#### Ruby
```ruby
# lib/user_service/handlers.rb
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

#### PHP
```php
<?php declare(strict_types=1);

namespace UserService;

class Handlers {
    public function getUser(Request $request): Response {
        $userId = $request->pathParams['id'];
        return new Response(['id' => $userId, 'name' => 'Alice']);
    }
}
```

#### Rust
```rust
// src/handlers.rs
use spikard::{Handler, Request, Response};

pub async fn get_user(request: Request) -> Response {
    let user_id = request.path_params.get("id");
    Response::json(json!({"id": user_id, "name": "Alice"}))
}
```

### Running Tests

#### Python
```bash
# Run all tests
uv run pytest tests/

# Run specific test file
uv run pytest tests/test_handlers.py

# With coverage
uv run pytest --cov=src tests/
```

#### TypeScript
```bash
# Run all tests
pnpm test

# Watch mode
pnpm test --watch

# With coverage
pnpm test --coverage
```

#### Ruby
```bash
# Run all tests
bundle exec rspec spec/

# Run specific file
bundle exec rspec spec/user_service_spec.rb

# With coverage
bundle exec rspec --require coverage spec/
```

#### PHP
```bash
# Run all tests
composer test

# Run specific test
composer test -- tests/HandlersTest.php

# With coverage
composer test -- --coverage
```

#### Rust
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_get_user

# With coverage
cargo tarpaulin
```

---

## Advanced Topics

### Customizing Project Templates

To customize generated files, modify the scaffolder implementations in:
```
crates/spikard-cli/src/init/[language].rs
```

### Adding New Language Support

To add support for a new language:

1. Create `crates/spikard-cli/src/init/new_language.rs`
2. Implement `ProjectScaffolder` trait
3. Add to `TargetLanguage` enum
4. Register in `InitEngine::scaffolder_for_language()`
5. Add tests in `crates/spikard-cli/tests/`

### CI/CD Integration

Example GitHub Actions workflow:

```yaml
name: New Project Validation
on: [workflow_dispatch]

jobs:
  test-init:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        language: [python, typescript, ruby, php, rust]
    steps:
      - uses: actions/checkout@v3
      - name: Initialize project
        run: |
          spikard init \
            --name test_project \
            --language ${{ matrix.language }} \
            --project-dir ./test_projects
      - name: Build and test
        run: |
          cd ./test_projects/test_project
          # Language-specific build & test commands
```

---

## Architecture Reference

### InitEngine Flow

```
1. Parse CLI Arguments
   ├── Validate project name
   ├── Validate language
   └── Validate paths

2. Select Scaffolder
   ├── Python → PythonScaffolder
   ├── TypeScript → TypeScriptScaffolder
   ├── Ruby → RubyScaffolder
   ├── PHP → PhpScaffolder
   └── Rust → RustScaffolder

3. Generate Files
   ├── Create directory structure
   ├── Generate manifests (pyproject.toml, package.json, etc.)
   ├── Generate source code (app.rs, handlers.py, etc.)
   ├── Generate tests
   └── Generate documentation (README.md)

4. Return Response
   ├── List of created files
   └── Next steps for user
```

### ProjectScaffolder Trait

```rust
pub trait ProjectScaffolder {
    fn scaffold(&self, request: &InitRequest) -> Result<Vec<ScaffoldedFile>>;

    fn validate_name(&self, name: &str) -> Result<()>;
}

pub struct ScaffoldedFile {
    pub path: PathBuf,
    pub content: String,
}
```

---

## Related Documentation

- [Code Generation Guide](../guides/code-generation.md)
- [Codegen Modernization](./codegen-modernization.md)
- [ADR-0004: Code Generation](../adr/0004-code-generation.md)
- [API Documentation](../reference/api.md)

---

## FAQ

**Q: Can I initialize a project in an existing directory?**
A: No, init creates a new directory to avoid conflicts. To use an existing directory, manually copy the generated files.

**Q: What if my project name has special characters?**
A: The init command will convert them to valid identifiers (e.g., `my-api` → `my_api` for Python).

**Q: Can I use init with a monorepo structure?**
A: Use `--project-dir` to specify the parent directory:
```bash
spikard init --name service1 --language python --project-dir ./services
spikard init --name service2 --language typescript --project-dir ./services
```

**Q: Are the generated files production-ready?**
A: The scaffolded structure is ready for development. You'll need to add business logic, additional handlers, and configuration appropriate for your use case.

**Q: How do I update the scaffolded project later?**
A: The init command is for initial scaffolding. For code generation from schemas, use `spikard codegen`.

**Q: Can I contribute custom scaffolders?**
A: Yes! See the Architecture Reference section for implementing custom scaffolders. Submit PR to the repository.

---

## Support

For issues or questions:
- GitHub Issues: https://github.com/spikard/spikard/issues
- Documentation: https://docs.spikard.dev
- Discussions: https://github.com/spikard/spikard/discussions
