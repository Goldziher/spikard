# Changelog

All notable changes to the Spikard project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### New Features
- **`spikard init` command**: Project scaffolding for all 5 supported languages (Python, TypeScript, Ruby, PHP, Rust)
  - Language-specific project structure generation
  - Automatic dependency initialization (pip, npm/pnpm, gem, composer, cargo)
  - Example handler files with language-specific patterns
  - Interactive next steps guidance
  - Support for optional schema file integration

- **Quality validation framework**: Comprehensive code generation quality checks
  - Language-specific syntax validation
  - Type checking integration (mypy, TypeScript, Steep, PHPStan)
  - Linting with native tools (Ruff, Biome, Rubocop, PHP-CS-Fixer)
  - Structured validation reports with detailed error information
  - Support for Python, TypeScript, Ruby, PHP, and Rust

- **Shared codegen utilities**: DRY architecture for code generation
  - Case conversion utilities (snake_case, camelCase, PascalCase, kebab-case)
  - String escaping contexts for different syntaxes (JSON, GraphQL SDL, docstrings, template literals)
  - Identifier sanitization with language-specific rules
  - Common formatting helpers used across all generators

#### Generator Improvements
- **GraphQL generators**: Fully typed code generation with quality tool compliance
  - Auto-generated RBS type signatures for Ruby
  - Proper type imports and references across all languages
  - Elimination of Any types throughout generated code
  - DO NOT EDIT headers for better tooling integration
  - SDL reconstruction with proper schema validation

- **OpenAPI generators**: Critical bug fixes and quality improvements
  - Ruby: Fixed multi-line comment handling
  - PHP: Corrected parameter ordering violations
  - TypeScript: Resolved forward reference errors
  - All generators now use shared utilities for consistency

- **OpenRPC and AsyncAPI generators**: Systematic bug fixes
  - Critical issues resolved in all language-specific generators
  - Improved error handling and validation
  - Better fixture coverage and test integration

### Changed

#### Breaking Changes
- **Code generation architecture refactored**: All generators now use shared utilities
  - Moved case conversion, escaping, and sanitization to common module
  - Generators instantiate shared utilities instead of duplicating logic
  - Benefits: Consistency, maintainability, fewer bugs

#### Improvements
- Improved code generation quality across all languages through shared utilities
- Better error messages and validation diagnostics
- More consistent formatting and naming conventions across language generators
- Enhanced fixture-driven testing for all code generation scenarios

### Fixed

#### Critical Bugs
- **Ruby OpenAPI**: Multi-line comment handling that caused syntax errors
- **PHP OpenAPI**: Parameter ordering violations breaking API contracts
- **TypeScript OpenAPI**: Forward reference errors in type definitions
- **OpenRPC generators**: Serialization issues causing double JSON encoding
- **AsyncAPI generators**: Critical type mapping issues across all languages
- **Python GraphQL generator**: Fixed scalar type mapping tests to reflect actual implementation
- **Python GraphQL generator**: Corrected union type assertion to use `TypeAlias` annotation format

#### Quality Issues
- Eliminated duplicate code generation logic across language-specific modules
- Resolved type system inconsistencies in generated code
- Fixed identifier sanitization edge cases
- Corrected escape sequence handling in docstrings and comments
- Fixed doctest import paths in shared utilities to use full crate paths

#### Dependency Issues
- **Node.js examples**: Fixed incorrect workspace dependency reference in `examples/graphql-node`
  - Changed from `"spikard": "workspace:*"` to correct `"@spikard/node": "workspace:*"`
  - Resolved `task update` failures caused by missing workspace package

### Documentation

#### User-Facing
- **ADR-0004**: Updated architecture decision record to reflect current implementation
  - Documented DRY architecture with shared utilities
  - Added quality validation framework details
  - Included init command scaffolding patterns
- **CLI Usage Guide**: Enhanced with comprehensive command examples
  - Added `spikard init` usage and options
  - Documented all `spikard generate` subcommands (GraphQL, OpenAPI, AsyncAPI, OpenRPC)
  - Included schema integration examples
- **Init Command Guide**: Cleaned up implementation details, focused on user-facing features
  - Removed Rust implementation architecture section
  - Retained usage examples and language-specific scaffolds

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
