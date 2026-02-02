# ADR 0004: Code Generation Architecture
**Status**: Accepted
**Date**: 2024-12-28 (Updated)

## Context

Spikard generates type-safe, idiomatic code from schemas (OpenAPI, AsyncAPI, OpenRPC, GraphQL) for five languages (Python, TypeScript, Ruby, PHP, Rust). Generated code must pass strict quality tools (mypy --strict, tsc, steep, phpstan level max, clippy) and integrate with Spikard's runtime.

## Decision

### Architecture

**CLI Entry Point**: `spikard-cli` (`crates/spikard-cli/src/cli.rs`)
- `spikard generate <protocol>` - Generate handlers/types from schema
- `spikard init` - Scaffold new projects with idiomatic structure

**Generator Organization**: `crates/spikard-cli/src/codegen/`
```
codegen/
├── common/              # Shared utilities (case conversion, escaping, sanitization)
├── formatters/          # Language-specific code formatting
├── quality/             # Quality validation framework
├── graphql/generators/  # GraphQL: Python, TypeScript, Ruby, PHP, Rust
├── openapi/             # OpenAPI generators
├── asyncapi/            # AsyncAPI generators
└── openrpc/             # OpenRPC generators
```

**Core Principles**:
1. **DRY**: Extract common logic into `codegen/common/` - no duplication across generators
2. **Quality-First**: All generated code must pass language-specific quality tools
3. **Idiomatic Output**: Follow ecosystem conventions (uv for Python, pnpm for TS, bundler for Ruby, etc.)
4. **Zero Runtime Dependencies**: Generated code uses only Spikard runtime APIs

### Shared Utilities

**Case Conversion** (`common/case_conversion.rs`):
- `to_snake_case`, `to_camel_case`, `to_pascal_case`, `to_kebab_case`
- Handles acronyms, preserves underscores, consistent across all generators

**Escaping** (`common/escaping.rs`):
- Context-aware string escaping for Python, JavaScript, Ruby, PHP, Rust
- `escape_for_docstring`, `escape_quotes`, `escape_template_literal`

**Identifier Sanitization** (`common/identifier_sanitization.rs`):
- Language-specific keyword avoidance
- Invalid character handling per language rules

**Formatters** (`formatters/{python,typescript,ruby,php,rust}.rs`):
- Language-specific headers (shebang, strict mode, frozen_string_literal)
- Import organization and ordering
- Docstring/comment formatting

### Quality Validation

**QualityValidator** (`quality/validator.rs`):
- Runs language-specific quality tools on generated code
- Python: `mypy --strict`, `ruff check`
- TypeScript: `tsc --noEmit`, `biome check`
- Ruby: `ruby -c`, `steep check`
- PHP: `php -l`, `phpstan --level=max`
- Rust: `cargo check`, `cargo clippy`

**Test Coverage**: Fixture-driven quality tests for each protocol × language combination

### Protocol Generators

**GraphQL** (`codegen/graphql/`):
- SDL builder for schema reconstruction
- Type mapper for cross-language type mapping
- Generators produce resolvers, types, and schema bindings

**OpenAPI/AsyncAPI/OpenRPC**:
- Follow same shared utility patterns
- Protocol-specific parsers in respective modules
- Consistent output quality across all protocols

### Project Scaffolding

**Init Command** (`crates/spikard-cli/src/init/`):
- Trait-based scaffolder system
- Language-specific implementations: `PythonScaffolder`, `TypeScriptScaffolder`, etc.
- Generates idiomatic project structure with:
  - Package manager config (pyproject.toml, package.json, Gemfile, composer.json, Cargo.toml)
  - Testing framework setup (pytest, vitest, rspec, phpunit, cargo test)
  - Quality tools (mypy, biome, steep, phpstan, clippy)
  - Example health endpoint

**Schema Integration**: When provided with `--schema`, generates handlers in conventional locations

## Consequences

**Benefits**:
- Single source of truth for common operations (1,670 lines of duplication eliminated)
- Consistent quality across all generated code
- Easy to add new protocols (reuse shared utilities)
- Generated code passes strict type checkers out of the box

**Trade-offs**:
- Breaking changes to generator internals when updating shared utilities
- Must maintain quality tool compatibility as they evolve
- Pre-1.0: Breaking changes to generated code structure are expected

**Maintenance**:
- Update shared utilities when adding language-specific edge cases
- Keep quality validators in sync with tool versions
- Test fixtures must cover all protocol × language combinations

## References

- CLI: `crates/spikard-cli/src/cli.rs`
- Generators: `crates/spikard-cli/src/codegen/`
- Init scaffolders: `crates/spikard-cli/src/init/`
- Test fixtures: `testing_data/{openapi,asyncapi,openrpc,graphql}_schemas/`
- Quality tests: `crates/spikard-cli/tests/*_quality.rs`
