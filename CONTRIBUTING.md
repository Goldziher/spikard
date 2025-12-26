# Contributing to Spikard

Thank you for your interest in contributing to Spikard! This document provides guidelines and information for contributors.

## Table of Contents

- [Development Setup](#development-setup)
- [Repository Structure](#repository-structure)
- [Testing: Fixture-Driven Development](#testing-fixture-driven-development)
- [Benchmarking](#benchmarking)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [How to Contribute](#how-to-contribute)
- [Language Binding Development](#language-binding-development)

## Development Setup

### Prerequisites

**Required:**
- **Rust** 1.80+ via [rustup](https://rustup.rs/)
- **Python** 3.11+ (recommend using [uv](https://github.com/astral-sh/uv))
- **Node.js** 20+ and [pnpm](https://pnpm.io/) 10+
- **Ruby** 3.2+ via rbenv
- **Git** for version control

**Optional:**
- **Docker** for isolated testing environments
- **PostgreSQL** for integration tests (if using database fixtures)
- **macOS only (PHP extension)**: Homebrew LLVM (`brew install llvm`) so `LIBCLANG_PATH` and `BINDGEN_EXTRA_CLANG_ARGS` resolve when building `spikard-php` with `--all-features`; the Taskfile auto-sets these on macOS if LLVM is present.

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/your-org/spikard.git
cd spikard

# Install Node.js dependencies
pnpm install

# Install Ruby dependencies
cd packages/ruby
bundle install
cd ../..

# Install Python dependencies (using uv)
cd packages/python
uv sync
cd ../..

# Build all Rust crates and bindings
task build

# Verify installation
task test
```

### Using Taskfile

Spikard uses [Task](https://taskfile.dev/) for automation. Install it:

```bash
# macOS
brew install go-task

# Linux
sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b ~/.local/bin

# Windows
choco install go-task
```

**Common tasks:**

```bash
task setup          # Install all dependencies and build bindings
task build          # Build all bindings
task build:rust     # Build Rust workspace
task build:py       # Build Python bindings (maturin)
task build:node     # Build Node.js bindings (napi-rs)
task build:ruby     # Build Ruby bindings (rb-sys)
task build:wasm     # Build WebAssembly bindings

task test           # Run all tests
task test:rust      # Rust tests only
task test:python    # Python tests only
task test:node      # Node.js tests only
task test:ruby      # Ruby tests only

task lint           # Run all linters
task format         # Format all code

task benchmark:profile   # Profile Spikard implementations
task benchmark:compare   # Compare with other frameworks
```

See `Taskfile.yaml` for the complete list of available tasks.

## Repository Structure

```
spikard/
├── crates/                    # Rust workspace
│   ├── spikard-core/          # Core types, validation, routing
│   ├── spikard-http/          # HTTP server, middleware (Axum + Tower)
│   ├── spikard-cli/           # Command-line interface
│   ├── spikard-codegen/       # Code generation logic
│   ├── spikard-py/            # Python bindings (PyO3)
│   ├── spikard-node/          # Node.js bindings (napi-rs)
│   ├── spikard-rb/            # Ruby bindings (Magnus)
│   └── spikard-wasm/          # WebAssembly bindings (wasm-bindgen)
│
├── packages/                  # Language packages
│   ├── python/                # Python package (published to PyPI)
│   │   ├── spikard/           # Python source
│   │   └── tests/             # Integration tests
│   ├── node/                  # TypeScript package (published to npm)
│   │   ├── src/               # TypeScript source
│   │   └── tests/             # Integration tests
│   └── ruby/                  # Ruby gem (published to RubyGems)
│       ├── lib/               # Ruby source
│       ├── sig/               # RBS type definitions
│       └── spec/              # RSpec tests
│
├── testing_data/              # Fixture-driven test scenarios
│   ├── 00-FIXTURE-SCHEMA.json # Meta-schema
│   ├── json_bodies/           # JSON body fixtures (52)
│   ├── query_params/          # Query parameter fixtures (74)
│   ├── path_params/           # Path parameter fixtures (43)
│   ├── headers/               # Header fixtures (36)
│   ├── cookies/               # Cookie fixtures (29)
│   ├── multipart/             # Multipart upload fixtures (26)
│   ├── validation_errors/     # Validation error fixtures (25)
│   ├── lifecycle_hooks/       # Lifecycle hook fixtures (15)
│   ├── websockets/            # WebSocket fixtures (9)
│   └── ... (27 categories total)
│
├── tools/                     # Development tools
│   ├── benchmark-harness/     # Performance benchmarking
│   ├── test-generator/        # Generate tests from fixtures
│   └── app-generator/         # Generate apps from fixtures
│
├── docs/                      # Documentation
│   └── adr/                   # Architecture Decision Records
│       ├── 0001-architecture.md
│       ├── 0002-runtime-and-middleware.md
│       ├── 0003-validation-and-fixtures.md
│       ├── 0004-code-generation.md
│       ├── 0005-lifecycle-hooks.md
│       ├── 0006-streaming-sse-websockets.md
│       └── 0007-observability.md
│
├── examples/                  # Example applications (coming soon)
├── Taskfile.yaml              # Task automation
├── Cargo.toml                 # Rust workspace manifest
├── Cargo.lock                 # Rust dependency lock (committed)
├── package.json               # Root package.json (workspaces)
├── pnpm-lock.yaml             # Node.js dependency lock (committed)
└── uv.lock                    # Python dependency lock (committed)
```

### Key Design Principles

**Layered Architecture:**
```
Language Packages (Python/Node/Ruby/WASM)
    ↓ (PyO3, napi-rs, Magnus, wasm-bindgen)
Binding Crates (thin FFI adapters)
    ↓ (Handler trait abstraction)
HTTP Server (Axum + Tower middleware)
    ↓
Core Library (validation, routing, schemas)
```

**Separation of Concerns:**
- **spikard-core**: Transport-agnostic primitives (no HTTP knowledge)
- **spikard-http**: HTTP server, middleware, WebSocket/SSE (no FFI knowledge)
- **Binding crates**: Thin FFI adapters implementing `Handler` trait
- **Language packages**: Idiomatic APIs wrapping binding crates

**Handler Trait Abstraction:**

All language bindings implement the same `Handler` trait from `spikard-http`:

```rust
pub trait Handler: Send + Sync {
    fn call(
        &self,
        request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>>;
}
```

This enables **zero FFI dependencies** in the HTTP layer—all Python/Node/Ruby/WASM code stays isolated in binding crates.

## Testing: Fixture-Driven Development

### Philosophy

Spikard follows a **spec-first, fixture-driven approach** where behavior is defined in JSON fixtures before implementation.

**Workflow:**
1. Define behavior in `testing_data/` fixtures
2. Update `schema.json` with validation rules
3. Generate tests from fixtures
4. Implement handlers to satisfy fixtures
5. All languages must pass identical fixture tests

### The testing_data/ Directory

**Structure:**
- 27 fixture categories
- 400+ individual test scenarios
- 15 `schema.json` files defining validation contracts

**Each fixture contains:**
- `schema.json` - JSON Schema Draft 2020-12 definition
- `*.json` files - Test scenarios with expected inputs/outputs

**Example:**
```
testing_data/json_bodies/
├── schema.json           # Validation rules for this category
├── simple-object.json    # Test: basic JSON object
├── nested-object.json    # Test: deeply nested structure
├── large-array.json      # Test: 1000+ item array
└── ... (52 fixtures total)
```

### Running Tests

**All tests:**
```bash
task test
```

**Language-specific:**
```bash
task test:rust      # Rust unit + integration tests
task test:python    # pytest suite
task test:node      # vitest suite
task test:ruby      # RSpec suite
```

**Specific test files:**
```bash
# Python
cd packages/python
uv run pytest tests/test_all_fixtures.py -v

# Node.js
cd packages/node
pnpm test

# Ruby
cd packages/ruby
bundle exec rspec spec/integration_spec.rb

# Rust
cargo test -p spikard-core
cargo test -p spikard-http
```

### Coverage Requirements

- **Rust core:** 95% minimum (enforced by tarpaulin)
- **Language bindings:** 80%+ minimum
- CI fails if coverage drops below threshold

**Check coverage:**
```bash
# Rust
cargo tarpaulin --workspace --exclude spikard-cli --out Html

# Python
cd packages/python
uv run pytest --cov=spikard --cov-report=html

# Node.js
cd packages/node
pnpm test:coverage
```

### Three-Tier Testing Strategy

**Tier 1: Unit Tests**
- Pure functions, fast execution
- No I/O, no network
- Rust: `cargo test -p spikard-core`
- Python: Function-based tests with `pytest`

**Tier 2: Integration Tests**
- Real infrastructure (databases if needed)
- Fixture loading from `testing_data/`
- HTTP client tests against test servers
- Python: `packages/python/tests/`
- Rust: Integration test crates

**Tier 3: End-to-End Tests**
- Full HTTP stack
- All language bindings
- Multi-language client/server combinations
- Generated from fixtures via `test-generator`

**Tier 4: Published Package Tests**
- Validate published packages from registries
- Test apps in `tests/test_apps/`
- Executed after releases in CI
- Catches registry-specific issues (binary distribution, platform compatibility)

### Adding New Fixtures

1. **Create fixture file:**
   ```bash
   cd testing_data/json_bodies
   # Create new-feature.json with request/response data
   ```

2. **Update schema.json:**
   ```json
   {
     "categories": {
       "json_bodies": {
         "fixtures": [
           "new-feature"
         ]
       }
     }
   }
   ```

3. **Regenerate tests:**
   ```bash
   task generate:tests
   ```

4. **Implement handlers:**
   - Update Rust core if needed
   - Update language bindings
   - All must pass the new fixture test

5. **Verify:**
   ```bash
   task test
   ```

## Benchmarking

### Benchmark Harness

Located in `tools/benchmark-harness/`, provides two modes:

**Profile Mode:**
- Deep analysis of Spikard implementations
- Language-specific profilers (py-spy, Node profiler, stackprof)
- GIL contention, V8 optimization, GC pause analysis
- Baseline comparison against pure Rust

**Compare Mode:**
- Framework comparisons within ecosystems
- Python: Spikard vs FastAPI vs Robyn vs Litestar
- Node: Spikard vs Fastify vs Hono vs Express
- Ruby: Spikard vs Roda vs Hanami-API

### Running Benchmarks

**Profile Spikard implementation:**
```bash
cd tools/benchmark-harness
cargo run --release -- profile \
  --framework spikard-python \
  --app-dir apps/spikard-python \
  --suite all \
  --output results/python-profile.json
```

**Compare with other frameworks:**
```bash
cargo run --release -- compare \
  --frameworks spikard-python,fastapi,robyn \
  --apps apps/ \
  --suite json-bodies \
  --output results/comparison.json
```

### Workload Suites

- `all` (15 workloads) - Complete benchmark suite
- `json-bodies` (4) - Small, medium, large, very-large JSON
- `path-params` (6) - Simple, multiple, deep, int, uuid, date
- `query-params` (3) - Few, medium, many parameters
- `forms` (2) - URL-encoded simple and complex

### Benchmark Applications

**Auto-generated apps** (DO NOT manually edit):
```bash
cd tools/app-generator
cargo build --release

./target/release/app-generator generate \
  --framework spikard-python \
  --fixtures ../../testing_data \
  --output ../benchmark-harness/apps/spikard-python \
  --categories json_bodies,multipart,url_encoded
```

**Baseline apps** (hand-written for comparison):
- `apps/fastapi/` - FastAPI baseline
- `apps/fastify/` - Fastify baseline
- `apps/roda/` - Roda baseline

### Performance Expectations

Spikard aims for:
- **Rust:** Best-in-class performance (165k+ req/s)
- **Python:** Match or exceed FastAPI/Litestar (15k+ req/s)
- **Node:** Competitive with Fastify/Hono (80k+ req/s)
- **Ruby:** Match or exceed Roda/Hanami (20k+ req/s)

## Development Workflow

### Making Changes

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Add fixtures if adding behavior:**
   ```bash
   cd testing_data/my-category
   # Create new fixture JSON files
   # Update schema.json
   ```

3. **Implement in Rust core:**
   ```bash
   cd crates/spikard-core
   # Add implementation
   cargo test
   ```

4. **Update language bindings:**
   ```bash
   # Python
   cd crates/spikard-py
   # Update FFI adapter

   cd packages/python
   # Update high-level API
   uv run pytest
   ```

5. **Lint and format:**
   ```bash
   task lint
   task format
   ```

6. **Run full test suite:**
   ```bash
   task test
   ```

7. **Commit and push:**
   ```bash
   git add .
   git commit -m "feat: add new feature"
   git push origin feature/my-feature
   ```

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks
- `ci:` - CI/CD changes

**Examples:**
```
feat(python): add dependency injection support
fix(http): correct CORS preflight handling
docs(adr): add ADR for GraphQL design
test(fixtures): add edge case for nested arrays
perf(py): optimize JSON to Python conversion
```

## Code Standards

### Rust (2024 Edition)

**Requirements:**
- Rust 2024 edition
- `cargo fmt` with project `rustfmt.toml`
- `cargo clippy -- -D warnings` (zero tolerance)
- 95% test coverage minimum

**Best practices:**
- Use `Result<T, E>` for errors (no panics in production code)
- `thiserror` for custom error types
- NEVER `.unwrap()` in production paths (`.expect()` with message in tests only)
- Explicit lifetimes when needed
- SAFETY comments for any `unsafe` code
- Async: Tokio 1.x with `'static` and `Send + Sync` bounds

**Example:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid schema: {0}")]
    InvalidSchema(String),
}

pub fn validate(data: &Value) -> Result<(), ValidationError> {
    // Use ? for error propagation, never unwrap
    Ok(())
}
```

### Python (3.11+)

**Requirements:**
- Python 3.11+ (for match/case, union types)
- Full type hints with `mypy --strict`
- msgspec for validation (NOT Pydantic unless explicitly needed)
- Fully async (anyio, httpx AsyncClient)
- Function-based tests only (`*_test.py`)

**Best practices:**
- Use `X | Y` syntax for unions, never `Optional[T]`
- `ParamSpec`, `TypeVar`, `Generic[T]` for proper typing
- NEVER use `Any` type
- Pure functions, composition, immutability
- pytest fixtures for test setup

**Example:**
```python
from msgspec import Struct
from typing import TypeVar

T = TypeVar('T')

class User(Struct, frozen=True):
    id: int
    name: str

async def get_user(user_id: int) -> User | None:
    # Fully typed, async, immutable
    return User(id=user_id, name="Alice")
```

### TypeScript (5.x)

**Requirements:**
- TypeScript 5.x with strictest settings
- Enable ALL strict flags in `tsconfig.json`
- Ban `any` and `object` types (use `unknown` with guards)
- vitest for testing (`.spec.ts` next to source)
- Biome for linting and formatting

**Best practices:**
- Generics with constraints
- `satisfies` operator for type narrowing
- `const` assertions for literals
- Functional: map/filter/reduce, immutability, `readonly`
- NEVER use non-null assertions (`!`)
- NEVER use `||` for defaults (use `??`)

**Example:**
```typescript
interface User {
  readonly id: number;
  readonly name: string;
}

async function getUser(id: number): Promise<User | null> {
  // Properly typed, immutable, null handling
  return { id, name: "Alice" };
}
```

### Ruby (3.2+)

**Requirements:**
- Ruby 3.2+ with `.ruby-version`
- RBS type definitions in `sig/` directory
- Steep for type checking
- RSpec for testing (80%+ coverage)
- Rubocop with auto-fix

**Best practices:**
- Methods <10 lines
- Guard clauses for early returns
- Modules for mixins
- YARD documentation
- Line length ≤120 characters

**Example:**
```ruby
# sig/user.rbs
class User
  attr_reader id: Integer
  attr_reader name: String

  def initialize: (id: Integer, name: String) -> void
end

# lib/user.rb
class User
  attr_reader :id, :name

  def initialize(id:, name:)
    @id = id
    @name = name
  end
end
```

### WebAssembly

**Requirements:**
- wasm-bindgen for FFI
- wasm-pack for bundling
- No blocking operations (all async)
- Minimize binary size (`opt-level=z`, `lto=true`)

**Best practices:**
- NEVER spawn threads (limited WASM support)
- No blocking I/O
- Proper JS/WASM type boundaries
- Use `console_error_panic_hook` for debugging

## How to Contribute

### Reporting Issues

**Before opening an issue:**
1. Search existing issues
2. Check if it's already fixed in `main`
3. Verify with latest version

**When reporting:**
- Spikard version
- Language binding and version
- Operating system and version
- Minimal reproduction code
- Expected vs. actual behavior
- Stack trace if applicable

### Contributing Code

**Good first issues:**
- Add new fixtures to `testing_data/`
- Improve documentation
- Add examples
- Fix typos
- Add validation library integration

**Larger contributions:**
- Discuss in an issue first
- Follow the development workflow above
- Include tests and benchmarks
- Update relevant ADRs if architecture changes

### Testing Published Packages

After releasing a new version, update and test the test apps:

1. **Update versions across all test apps:**
   ```bash
   task test:apps:update-versions VERSION=0.7.0
   ```

2. **Update lock files:**
   ```bash
   cd tests/test_apps/python && uv sync && cd ../../..
   cd tests/test_apps/node && pnpm install && cd ../../..
   cd tests/test_apps/ruby && bundle install && cd ../../..
   cd tests/test_apps/php && composer install && cd ../../..
   cd tests/test_apps/rust && cargo update && cd ../../..
   cd tests/test_apps/wasm && pnpm install && cd ../../..
   ```

3. **Run all test apps:**
   ```bash
   task test:apps:all
   ```

4. **Commit changes:**
   ```bash
   git add tests/test_apps/
   git commit -m "chore: update test apps to v0.7.0"
   ```

**Note**: CI automatically runs test apps after successful releases, but manual testing before release is recommended.

### Contributing Language Bindings

See [Language Binding Development](#language-binding-development) below.

## Language Binding Development

### Adding a New Language Binding

**Prerequisites:**
1. Language has mature FFI support
2. Active community and ecosystem
3. Good interop with Rust (C ABI or similar)

**Recommended FFI options:**
- **C ABI languages:** Use FFI crate (like `cbindgen`)
- **Python-like:** PyO3
- **Node.js-like:** napi-rs
- **Ruby-like:** Magnus
- **Go:** Explore cgo or similar
- **JVM languages:** JNI
- **.NET:** C# interop

**Steps:**

1. **Create binding crate:**
   ```bash
   cargo new --lib crates/spikard-yourlang
   cd crates/spikard-yourlang
   ```

2. **Add dependencies:**
   ```toml
   [dependencies]
   spikard-http = { path = "../spikard-http" }
   # Your FFI library (napi-rs, PyO3, etc.)
   ```

3. **Implement Handler trait:**
   ```rust
   use spikard_http::Handler;

   pub struct YourLangHandler {
       // FFI callback storage
   }

   impl Handler for YourLangHandler {
       fn call(&self, req: Request<Body>, data: RequestData)
           -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>>
       {
           // Call into your language
       }
   }
   ```

4. **Create language package:**
   ```bash
   mkdir -p packages/yourlang
   # Add package manifest (package.json, Gemfile, etc.)
   ```

5. **Add to workspace:**
   Update root `Cargo.toml` and `Taskfile.yaml`

6. **Add tests:**
   - Reuse fixtures from `testing_data/`
   - Ensure all 400+ fixtures pass
   - Add to `task test`

7. **Document:**
   - Create `packages/yourlang/README.md`
   - Add examples
   - Update root README

### Binding Design Guidelines

**DO:**
- Follow language idioms (decorators in Python, builders in Rust, etc.)
- Integrate with popular validation libraries
- Keep FFI layer thin
- Let Rust handle heavy lifting
- Provide test clients

**DON'T:**
- Reimplement middleware in bindings
- Duplicate validation logic
- Block the event loop
- Panic across FFI boundaries
- Add language-specific features not in spec

### Testing Your Binding

1. **Pass all fixtures:**
   ```bash
   task test:yourlang
   ```

2. **Benchmark:**
   ```bash
   cd tools/benchmark-harness
   cargo run --release -- profile \
     --framework spikard-yourlang \
     --suite all
   ```

3. **Compare with ecosystem:**
   Find popular framework in your language and compare performance

4. **Document performance:**
   Add results to benchmark comparison

## Questions?

- **GitHub Issues:** Bug reports and feature requests
- **GitHub Discussions:** Questions and general discussion
- **ADRs:** See `docs/adr/` for architecture decisions

Thank you for contributing to Spikard!
