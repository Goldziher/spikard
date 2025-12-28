# Codegen Modernization: 3-Phase Architecture Refactor

## Overview

The Spikard code generation system has undergone a comprehensive 3-phase modernization to eliminate code duplication, improve maintainability, and ensure consistency across all language targets (Python, TypeScript, Ruby, PHP, Rust).

This document describes the architecture, shared utilities, quality framework, and best practices for the modernized system.

---

## Phase 1: Foundation & Shared Utilities

### Objective
Eliminate code duplication by extracting common code generation logic into shared utilities.

### Implemented Features

#### Case Conversion (`codegen/common/case_conversion.rs`)
Unified case conversion utilities used by all generators:

```
Input: "user_profile_handler"

to_camel_case()      → "userProfileHandler"
to_pascal_case()     → "UserProfileHandler"
to_snake_case()      → "user_profile_handler"
to_kebab_case()      → "user-profile-handler"
```

**Usage Example:**
```rust
use spikard_cli::codegen::common::{to_camel_case, to_pascal_case};

let class_name = to_pascal_case("user_dto");      // "UserDto"
let field_name = to_camel_case("is_active");      // "isActive"
```

#### String Escaping (`codegen/common/escaping.rs`)
Context-aware escaping for different syntax requirements:

| Context | Purpose | Example |
|---------|---------|---------|
| `EscapeContext::DoubleQuotes` | JSON/docstring escaping | `hello"world` → `hello\"world` |
| `EscapeContext::GraphQLSDL` | GraphQL schema strings | Handles `"""` blocks |
| `EscapeContext::TemplateLiteral` | Template string escaping | JavaScript/TypeScript |
| `EscapeContext::Docstring` | Language-specific docs | Handles `"""`, `///`, `/**` |

**Usage Example:**
```rust
use spikard_cli::codegen::common::{EscapeContext, escape_quotes};

let json_value = escape_quotes("hello\"world", EscapeContext::DoubleQuotes);
let graphql_doc = escape_graphql_sdl_description("A \"special\" type");
```

#### Identifier Sanitization (`codegen/common/identifier_sanitization.rs`)
Language-aware sanitization for reserved keywords and invalid identifiers:

```rust
use spikard_cli::codegen::common::{
    sanitize_identifier,
    TargetLanguage
};

// Python: Avoids reserved keywords like 'class', 'def', etc.
sanitize_identifier("class", TargetLanguage::Python)
    → "class_"

// TypeScript: Preserves camelCase while sanitizing
sanitize_identifier_camel_case("invalid-id", TargetLanguage::TypeScript)
    → "invalidId"

// Ruby: Handles snake_case patterns with reserved words
sanitize_identifier_snake_case("private_key", TargetLanguage::Ruby)
    → "private_key_"
```

### Architecture Diagram

```
┌─────────────────────────────────────────────────────┐
│             Shared Utilities (Phase 1)              │
├─────────────────────────────────────────────────────┤
│  • Case Conversion (snake, camel, pascal, kebab)   │
│  • String Escaping (JSON, GraphQL, Docstring)      │
│  • Identifier Sanitization (per language)          │
│  • Formatting Helpers                              │
└─────────────────────────────────────────────────────┘
                        ↑
        ┌───────────────┼───────────────┐
        │               │               │
        ↓               ↓               ↓
   ┌──────────┐  ┌──────────┐  ┌──────────┐
   │ GraphQL  │  │ OpenAPI  │  │ OpenRPC  │
   │Generator │  │Generator │  │Generator │
   └──────────┘  └──────────┘  └──────────┘
        ↓               ↓               ↓
   ┌──────────────────────────────────────┐
   │  Language-Specific Generators        │
   │  (Python, TypeScript, Ruby, PHP)     │
   └──────────────────────────────────────┘
```

---

## Phase 2: Quality Validation Framework

### Objective
Ensure all generated code meets language-specific quality standards before output.

### Quality Validator Architecture

The `QualityValidator` provides language-specific validation:

```rust
use spikard_cli::codegen::{TargetLanguage, quality::QualityValidator};

let validator = QualityValidator::new(TargetLanguage::Python);
let report = validator.validate_all("generated_code_here")?;

if !report.is_valid() {
    for error in report.errors {
        eprintln!("Quality check failed: {}", error.message);
        eprintln!("  Type: {}", error.check_type);
        eprintln!("  Line: {}", error.line_number.unwrap_or(0));
    }
}
```

### Language-Specific Quality Gates

#### Python
- **Syntax**: `python3 -m py_compile`
- **Types**: `mypy --strict` (no Any types, full type hints)
- **Linting**: `ruff check` (consistent with project standards)
- **Standards**: PEP 8, type hints on all functions

#### TypeScript
- **Syntax**: `tsc --noEmit` (strict mode)
- **Types**: Full type inference, no `any` or `object`
- **Linting**: `biome check` (strict ESLint rules)
- **Standards**: ESLint strict config, no unchecked index access

#### Ruby
- **Syntax**: `ruby -c` (compile check)
- **Types**: `steep check` (gradual typing with RBS)
- **Linting**: `rubocop` (style consistency)
- **Standards**: RBS type signatures in `sig/` directory

#### PHP
- **Syntax**: `php -l` (lint)
- **Types**: `phpstan --level=max` (maximum strictness)
- **Linting**: `php-cs-fixer` (PSR-12 compliance)
- **Standards**: PSR-4, PSR-12, strict_types=1

#### Rust
- **Syntax**: `cargo check`
- **Types**: Type system verification
- **Linting**: `cargo clippy -- -D warnings` (no warnings)
- **Standards**: Clippy, rustfmt compliance

### Validation Report Structure

```rust
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<QualityError>,
    pub warnings: Vec<String>,
    pub checks_passed: usize,
    pub checks_failed: usize,
}

#[derive(Debug, Clone)]
pub struct QualityError {
    pub message: String,
    pub check_type: String,      // "syntax", "type", "lint"
    pub line_number: Option<usize>,
    pub severity: ErrorSeverity,
}
```

### Usage in Generators

```rust
// After generating code
let generated_code = generator.generate(&schema)?;

// Validate quality
let validator = QualityValidator::new(target_language);
let report = validator.validate_all(&generated_code)?;

if !report.is_valid() {
    return Err(anyhow::anyhow!(
        "Generated code failed quality checks:\n{:?}",
        report.errors
    ));
}

// Safe to write
fs::write(output_path, generated_code)?;
```

---

## Phase 3: Generator Refactoring

### Objective
Refactor all generators (GraphQL, OpenAPI, OpenRPC, AsyncAPI) to use shared utilities and pass quality gates.

### Before & After Comparison

#### Before Modernization (Duplicated Code)
```rust
// codegen/graphql/generators/python.rs
fn to_camel_case(s: &str) -> String {
    // Duplicate implementation
}

fn escape_string(s: &str) -> String {
    // Duplicate implementation
}

// codegen/openapi/generators/python.rs
fn to_camel_case(s: &str) -> String {
    // Another duplicate implementation
}

fn escape_string(s: &str) -> String {
    // Another duplicate implementation
}
```

#### After Modernization (Shared Utilities)
```rust
// codegen/graphql/generators/python.rs
use crate::codegen::common::{to_camel_case, escape_quotes, EscapeContext};

// Now all generators use the same, tested implementations
let field_name = to_camel_case(original_name);
let escaped = escape_quotes(docstring, EscapeContext::Docstring);
```

### Generator Implementation Pattern

Each generator now follows this pattern:

```rust
pub struct MyLanguageGenerator;

impl MyLanguageGenerator {
    pub fn generate(schema: &SchemaSpec) -> Result<String> {
        let mut output = String::new();

        // Use shared utilities
        for field in schema.fields() {
            let sanitized_name = sanitize_identifier(
                field.name,
                TargetLanguage::MyLanguage
            )?;
            let camel_name = to_camel_case(&sanitized_name);

            output.push_str(&format!(
                "{}: {}",
                camel_name,
                self.generate_type(&field.type_)?
            ));
        }

        // Validate quality before returning
        let validator = QualityValidator::new(TargetLanguage::MyLanguage);
        let report = validator.validate_all(&output)?;

        if !report.is_valid() {
            return Err(anyhow::anyhow!("Quality validation failed"));
        }

        Ok(output)
    }
}
```

### Generator Improvements Summary

| Generator | Key Improvements |
|-----------|------------------|
| **GraphQL - Ruby** | Auto-generated RBS type signatures, fixed reference handling |
| **GraphQL - TypeScript** | Eliminated Any types, proper forward reference resolution |
| **GraphQL - PHP** | Added root class generation, fixed inheritance chains |
| **OpenAPI - Ruby** | Fixed multi-line comment handling in descriptions |
| **OpenAPI - PHP** | Corrected parameter ordering to match API contracts |
| **OpenAPI - TypeScript** | Resolved forward reference errors in type definitions |
| **OpenRPC** | Fixed serialization (eliminated double JSON encoding) |
| **AsyncAPI** | Improved type mapping, fixed subscription handling |

---

## Quality Framework Usage Guide

### Running Quality Validation

#### Programmatic Usage
```rust
use spikard_cli::codegen::{TargetLanguage, quality::QualityValidator};

fn generate_and_validate(schema: &str, lang: TargetLanguage) -> Result<String> {
    // Generate code
    let generated = generate(schema, lang)?;

    // Create validator
    let validator = QualityValidator::new(lang);

    // Run all checks
    let report = validator.validate_all(&generated)?;

    // Check results
    if report.is_valid {
        Ok(generated)
    } else {
        Err(anyhow::anyhow!(
            "Quality checks failed:\n{}",
            report.errors.iter()
                .map(|e| format!("  - {}: {}", e.check_type, e.message))
                .collect::<Vec<_>>()
                .join("\n")
        ))
    }
}
```

#### CLI Usage
```bash
# Generate with quality validation (built-in)
spikard codegen --schema openapi.json --language python --output ./generated

# The CLI automatically validates generated code
# Exits with error if quality gates fail
```

### Customizing Quality Checks

To disable specific checks for a language:

```rust
let mut validator = QualityValidator::new(TargetLanguage::Python);
validator.disable_check("type");  // Skip mypy checks
let report = validator.validate_all(&code)?;
```

To add custom validation rules:

```rust
// Extend QualityValidator with custom checks
trait CustomValidation {
    fn validate_custom_rules(&self, code: &str) -> Result<Vec<QualityError>>;
}

impl CustomValidation for QualityValidator {
    fn validate_custom_rules(&self, code: &str) -> Result<Vec<QualityError>> {
        // Custom validation logic
        Ok(vec![])
    }
}
```

---

## Best Practices for Code Generation

### 1. Always Use Shared Utilities
```rust
// Good
use crate::codegen::common::to_pascal_case;
let class_name = to_pascal_case(&type_name);

// Bad - Don't duplicate
fn to_pascal_case(s: &str) -> String { ... }
```

### 2. Validate Generated Code
```rust
// Good
let validator = QualityValidator::new(target_lang);
let report = validator.validate_all(&generated_code)?;
if !report.is_valid() { return Err(...); }

// Bad - Skip validation
fs::write(output_path, generated_code)?;
```

### 3. Use Proper Escaping Contexts
```rust
// Good - Context-aware
use crate::codegen::common::{EscapeContext, escape_quotes};
let docstring = escape_quotes(doc, EscapeContext::Docstring);
let json = escape_quotes(data, EscapeContext::DoubleQuotes);

// Bad - Wrong context
let always_escaped = escape_quotes(doc, EscapeContext::DoubleQuotes);
```

### 4. Sanitize Identifiers
```rust
// Good
use crate::codegen::common::{sanitize_identifier, TargetLanguage};
let safe_name = sanitize_identifier(name, TargetLanguage::Python)?;

// Bad - Assuming names are safe
let unsafe_name = name;  // Might be a reserved keyword!
```

### 5. Add Tests for New Generators
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_generation() {
        let schema = r#"{ ... }"#;
        let result = MyGenerator::generate(schema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_quality_validation() {
        let schema = r#"{ ... }"#;
        let result = MyGenerator::generate(schema).unwrap();

        let validator = QualityValidator::new(TargetLanguage::MyLanguage);
        let report = validator.validate_all(&result).unwrap();
        assert!(report.is_valid);
    }
}
```

### 6. Document Generated Code
Add "DO NOT EDIT" headers to all generated code:

```python
# This file is auto-generated by Spikard. DO NOT EDIT manually.
# Schema: openapi.json
# Generated: 2025-12-28T10:30:00Z
```

---

## Migration Guide

### Updating Existing Generators

If you have a generator using duplicate code, here's the migration path:

1. **Remove duplicate utilities**
   ```rust
   // Remove these functions
   - fn to_camel_case(...)
   - fn escape_string(...)
   - fn sanitize_identifier(...)
   ```

2. **Add shared utility imports**
   ```rust
   use crate::codegen::common::{
       to_camel_case, escape_quotes, sanitize_identifier,
       EscapeContext, TargetLanguage,
   };
   ```

3. **Replace calls**
   ```rust
   // Before
   let name = self.to_camel_case(original_name);

   // After
   let name = to_camel_case(original_name);
   ```

4. **Add quality validation**
   ```rust
   let validator = QualityValidator::new(self.target_language);
   let report = validator.validate_all(&output)?;
   if !report.is_valid() {
       return Err(anyhow::anyhow!("Quality validation failed"));
   }
   ```

5. **Update tests**
   ```rust
   #[test]
   fn test_quality_gates() {
       let generated = generate_test_code();
       let validator = QualityValidator::new(TargetLanguage::MyLanguage);
       let report = validator.validate_all(&generated).unwrap();
       assert!(report.is_valid);
   }
   ```

---

## File Organization

### New Structure
```
crates/spikard-cli/src/codegen/
│
├── common/                          # Phase 1: Shared Utilities
│   ├── mod.rs                      # Public API
│   ├── case_conversion.rs          # to_camel_case, to_snake_case, etc.
│   ├── escaping.rs                 # Context-aware string escaping
│   └── identifier_sanitization.rs  # Language-aware name sanitization
│
├── quality/                         # Phase 2: Quality Validation
│   ├── mod.rs                      # Public API
│   └── validator.rs                # QualityValidator implementation
│
├── formatters/                      # Language-specific formatters
│   ├── mod.rs
│   ├── python.rs
│   ├── typescript.rs
│   ├── ruby.rs
│   └── php.rs
│
├── graphql/                         # Phase 3a: GraphQL Generators
│   ├── generators/
│   │   ├── base.rs
│   │   ├── python.rs
│   │   ├── typescript.rs
│   │   ├── ruby.rs
│   │   └── php.rs
│   └── ...
│
├── openapi.rs                       # OpenAPI generators
├── openrpc/                         # OpenRPC generators
├── asyncapi/                        # AsyncAPI generators
│
└── engine.rs                        # Main codegen engine
```

---

## Testing Strategy

### Unit Tests
Test shared utilities in isolation:
```rust
#[test]
fn test_camel_case_conversion() {
    assert_eq!(to_camel_case("user_profile"), "userProfile");
    assert_eq!(to_camel_case("HTTPResponse"), "httpResponse");
}
```

### Integration Tests
Test generators with quality validation:
```rust
#[test]
fn test_graphql_python_generation() {
    let schema = read_test_schema("graphql/test_schema.graphql");
    let result = GraphQLPythonGenerator::generate(&schema).unwrap();

    // Verify quality
    let validator = QualityValidator::new(TargetLanguage::Python);
    let report = validator.validate_all(&result).unwrap();
    assert!(report.is_valid);
}
```

### Fixture-Driven Tests
Use fixtures for comprehensive coverage:
```rust
#[test]
fn test_all_fixtures() {
    for fixture_file in glob("testing_data/graphql/*.json").unwrap() {
        let schema = load_schema(&fixture_file);

        for lang in [Python, TypeScript, Ruby, PHP] {
            let result = generate(&schema, lang).unwrap();
            let validator = QualityValidator::new(lang);
            assert!(validator.validate_all(&result).unwrap().is_valid);
        }
    }
}
```

---

## Performance Considerations

### Caching
The quality validator caches tool availability checks to avoid repeated subprocess calls:

```rust
// First call: Executes "which python3"
validator.validate_syntax(...)?;

// Subsequent calls: Uses cached result
validator.validate_syntax(...)?;
```

### Parallelization
For large schemas, validate multiple generators in parallel:

```rust
use rayon::prelude::*;

let languages = vec![Python, TypeScript, Ruby, PHP];
let results: Vec<_> = languages.par_iter()
    .map(|lang| generate_and_validate(&schema, *lang))
    .collect();
```

---

## Troubleshooting

### Quality Validation Fails
1. Check the specific error message: Is it syntax, type, or lint?
2. Run the native tool manually: `mypy`, `tsc`, `steep`, `phpstan`
3. Review the generated code for issues
4. Check CLAUDE.md for language-specific standards

### Missing Utilities
If a generator needs a new utility function:

1. Implement it in `codegen/common/`
2. Add tests for the utility
3. Export from `codegen/common/mod.rs`
4. Update generators to use it

### Custom Quality Rules
For project-specific quality requirements:

1. Extend `QualityValidator` with custom checks
2. Document the requirements in ADR-0004
3. Add fixtures to `testing_data/`
4. Update CLAUDE.md standards

---

## Related Documentation

- [ADR-0004: Code Generation](../adr/0004-code-generation.md)
- [Init Command Guide](./init-command.md)
- [Project Initialization](../init-command.md)
- [API Specification Guides](../guides/)

---

## Summary

The Spikard codegen modernization follows three phases:

1. **Phase 1**: Extract shared utilities (case conversion, escaping, sanitization)
2. **Phase 2**: Build quality validation framework (syntax, types, linting)
3. **Phase 3**: Refactor all generators to use Phase 1 & 2

This ensures:
- No duplicated code across generators
- Consistent output across all language targets
- High code quality with automated validation
- Easier maintenance and feature additions
- Better error detection and reporting
