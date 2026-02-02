# Contributing to Spikard

Thanks for your interest in contributing to Spikard! This guide covers how to set up your environment, run tests, and submit changes.

For full documentation, visit [spikard.dev](https://spikard.dev).

## Prerequisites

- Rust 1.85+ (edition 2024)
- Node.js 20+
- Python 3.10+
- Ruby 3.2+
- PHP 8.2+ (optional)
- [Task](https://taskfile.dev/) (task runner)

## Setup

```bash
git clone https://github.com/goldziher/spikard.git
cd spikard
task setup
```

This installs all language dependencies and pre-commit hooks.

## Development Workflow

### Building

```bash
task build              # Build everything
task build:rust         # Rust core only
task build:python       # Python bindings
task build:node         # Node.js bindings
task build:ruby         # Ruby bindings
task build:php          # PHP bindings
```

### Testing

```bash
task test               # Run all test suites
task test:rust          # Rust tests
task test:python        # Python tests
task test:js            # TypeScript tests
```

### Linting & Formatting

```bash
task lint               # Check all code quality
task format             # Apply formatting
```

Run these before opening a PR. The CI pipeline enforces them.

## Code Quality Standards

| Language | Linter / Type Checker | Coverage Threshold |
|----------|----------------------|--------------------|
| Rust | `clippy -D warnings` | 95%+ |
| Python | `mypy --strict`, `ruff` | 80%+ |
| TypeScript | Strict mode, `biome` | 80%+ |
| Ruby | `rubocop`, `steep` | 80%+ |
| PHP | `phpstan` level max | 80%+ |

## Architecture

Spikard follows a **thin binding pattern**: all business logic lives in the Rust core (`crates/spikard` and `crates/spikard-http`). Language bindings are thin wrappers that handle type conversion and expose idiomatic APIs. Never duplicate logic across bindings.

Spikard supports code generation from OpenAPI, GraphQL, gRPC/Protobuf, AsyncAPI, and JSON-RPC specifications.

See [Architecture Decision Records](docs/adr/) for design rationale.

## Adding Features

1. Implement the feature in the Rust core first
2. Expose it through the binding crates (spikard-py, spikard-node, spikard-rb, spikard-php)
3. Add fixture-driven tests under `testing_data/` and corresponding test cases
4. Update examples if applicable

## Submitting a Pull Request

1. Fork the repo and create a feature branch from `main`
2. Make your changes
3. Run `task lint` and `task test` — both must pass
4. Update `CHANGELOG.md` if applicable
5. Submit a PR with a clear description of the change

## Reporting Issues

Open an issue at [github.com/goldziher/spikard/issues](https://github.com/goldziher/spikard/issues).

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
