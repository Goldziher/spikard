# Changelog

All notable changes to the Spikard project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0] - 2025-12-30

### Added

- **GraphQL code generation**: Full support for generating typed GraphQL server code from schema files
  - Supports all 5 languages: Python, TypeScript, Ruby, PHP, Rust
  - Generates three output types: types, resolvers, and schema
  - Type-safe resolver signatures with proper parent/context/info parameters
  - Automatic RBS type definitions for Ruby
  - Strict type checking compliance (no `Any` types in Python, TypeScript)
  - Quality validation with mypy, TypeScript compiler, Steep, PHPStan
  - SDL schema reconstruction for runtime validation

- **`spikard init` command**: Project scaffolding for new Spikard projects
  - Supports all 5 languages: Python, TypeScript, Ruby, PHP, Rust
  - Language-specific project structure generation
  - Automatic dependency initialization (pip, npm/pnpm, gem, composer, cargo)
  - Example handler files following language-specific patterns
  - Optional schema file integration for code generation

- **Quality validation framework**: Automated validation of generated code
  - Language-specific syntax validation
  - Type checking integration (mypy, TypeScript, Steep, PHPStan)
  - Linting with native tools (Ruff, Biome, Rubocop, PHP-CS-Fixer)
  - Structured validation reports with detailed error messages

### Changed

- **Code generation architecture refactored**: All generators now use shared utilities
  - Centralized case conversion (snake_case, camelCase, PascalCase, kebab-case)
  - Unified string escaping for different contexts (JSON, GraphQL SDL, docstrings)
  - Consistent identifier sanitization with language-specific rules
  - Improved code quality and consistency across all generators

### Fixed

- **OpenAPI generators**: Critical bug fixes affecting generated code quality
  - Ruby: Fixed multi-line comment handling causing syntax errors
  - PHP: Corrected parameter ordering violations
  - TypeScript: Resolved forward reference errors in type definitions

- **OpenRPC generators**: Fixed serialization issues causing double JSON encoding

- **AsyncAPI generators**: Fixed critical type mapping issues across all languages

## [0.6.2] - 2025-12-28

### Fixed
- Version bump and test app updates for consistency

## [0.6.1] - Previous Release

See git history for detailed changes.

---

## Project Structure

### Codegen Modules
```
crates/spikard-cli/src/codegen/
├── common/              # Shared utilities (case conversion, escaping, sanitization)
├── quality/             # Quality validation framework
├── formatters/          # Language-specific formatters
├── graphql/             # GraphQL schema generators
├── openapi.rs           # OpenAPI generators
├── openrpc/             # OpenRPC generators
├── asyncapi/            # AsyncAPI generators
└── [language].rs        # Individual language generators
```

### Init Module
```
crates/spikard-cli/src/init/
├── engine.rs            # Core initialization orchestration
├── scaffolder.rs        # ProjectScaffolder trait
└── [language].rs        # Language-specific scaffolders
```

## Contributing

When adding new features or generators:

1. Use shared utilities in `codegen/common/` for case conversion and escaping
2. Validate generated code using `codegen/quality/QualityValidator`
3. Add fixtures to `testing_data/` for new scenarios
4. Update this changelog with all changes
5. Run `task test` to ensure quality gates pass

For detailed guidelines, see:
- [Code Generation Architecture](docs/adr/0004-code-generation.md)
- [Project Initialization Guide](docs/init-command.md)
- [Codegen Modernization](docs/codegen-modernization.md)
