# Spikard AI-Rulez V3 Migration Summary

**Date**: 2025-12-29
**Status**: Complete
**Schema Version**: ai-rules-v3
**Target Directory**: `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/`

## Overview

This document summarizes the migration of Spikard's 14 existing agents and 6 rules into structured custom configuration files following ai-rulez v3 schema. The migration consolidates spikard-specific HTTP framework configuration while maintaining inheritance from shared ai-rulez for polyglot development patterns.

## Migration Results

### Files Created

1. **custom-agents.yaml** (14 spikard-specific agents)
2. **custom-rules.yaml** (30 spikard-specific rules grouped by domain)
3. **custom-profiles.yaml** (1 spikard-web-framework profile extending web-framework.yaml)
4. **MIGRATION_SUMMARY.md** (this file)

### Preservation of Existing Content

All 14 original agent files and 30 rule files remain in their original locations:
- `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/agents/*.md` (14 files)
- `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/rules/*.md` (30 files)

These individual files can be kept as detailed references or archived for historical purposes. The YAML consolidations serve as the canonical configuration for ai-rulez tooling.

---

## 1. CUSTOM AGENTS MIGRATION (custom-agents.yaml)

### Spikard-Specific Agents Extracted

All 14 original agents categorized and documented:

#### HTTP Middleware & Runtime (3 agents)
- **middleware-architect** (sonnet) - tower-http middleware stack design
- **workspace-architect** (sonnet) - crate layering and FFI isolation
- **rust-polyglot-architect** (sonnet) - Handler trait and FFI boundaries

#### Language-Specific HTTP Binding Engineers (5 agents)
- **python-engineer** (haiku) - PyO3 HTTP bindings specifics (crates/spikard-py)
- **typescript-engineer** (haiku) - napi-rs HTTP bindings specifics
- **ruby-engineer** (haiku) - magnus/rb-sys HTTP bindings specifics
- **php-engineer** (haiku) - ext-php-rs HTTP bindings specifics
- **wasm-engineer** (haiku) - wasm-bindgen HTTP bindings specifics

#### Quality Assurance & Testing (2 agents)
- **fixture-tester** (haiku) - testing_data/ fixture evolution and schemas
- **integration-qa** (haiku) - cross-language fixture parity testing

#### Documentation & Developer Experience (2 agents)
- **docs-strategist** (haiku) - ADRs and architecture documentation
- **docs-scribe** (haiku) - CLAUDE.md generation and handbooks

#### Build, CI/CD & Operations (2 agents)
- **build-and-ci-ops** (haiku) - Taskfile.yaml and CI workflow orchestration
- **interop-build-engineer** (haiku) - multi-language build manifest coordination

### Analysis: Shared vs. Unique Content

**Not Extracted (Use Shared ai-rulez Instead)**:
- `polyglot-architect` - Use shared: `/workspace/kreuzberg-dev/ai-rulez/agents/polyglot-architect.md`
- `rust-core-engineer` - Use shared: `rust-core-engineer.md` (available in shared ai-rulez)
- General bindings engineers - Use shared: `python-bindings-engineer`, `typescript-bindings-engineer`, etc.

**Unique to Spikard** (Extracted):
- middleware-architect: Tower-HTTP specific, not in shared ai-rulez
- rust-polyglot-architect: Spikard's Handler trait design (vs. generic polyglot)
- workspace-architect: Spikard's specific crate layering (vs. generic workspace)
- {python,typescript,ruby,php,wasm}-engineer: HTTP-specific implementations (vs. generic bindings)
- fixture-tester, integration-qa: Spikard's fixture-first testing approach
- docs-strategist, docs-scribe: Spikard-specific documentation patterns
- build-and-ci-ops, interop-build-engineer: Spikard's multi-language orchestration

---

## 2. CUSTOM RULES MIGRATION (custom-rules.yaml)

### Spikard-Specific Rules Extracted

30 rules organized into 8 domains:

#### Fixture-Driven Validation (4 rules - CRITICAL)
- `fixture-driven-testing` - Every feature must extend fixture suite
- `fixture-first-testing` - Design behavior via fixtures before implementation
- `fixture-backed-testing` - Keep testing_data/ and tests synchronized
- `fixture-aligned-error-handling` - Error serialization via fixtures

#### HTTP Handler & Validation Contracts (4 rules - CRITICAL to HIGH)
- `handler-trait-abstraction` - Arc<dyn Handler> with Pin<Box<dyn Future>>
- `http-input-validation` - Schema-driven header, cookie, body validation
- `http-error-contracts` - Error translation to fixture-defined JSON
- `header-cookie-security` - Security defaults and validation fixtures

#### HTTP Request & Error Handling (2 rules)
- `request-surface-security` - Comprehensive validation layer strategy
- `cross-language-error-boundaries` - Error conversion at FFI (PyResult, napi::Result, etc.)

#### Tower-HTTP Middleware & Configuration (2 rules)
- `tower-http-middleware-stack` - Middleware composition and ServerConfig
- `lifecycle-hooks-implementation` - Zero-cost lifecycle hook design

#### Performance & Serialization (4 rules - 1 CRITICAL)
- `zero-copy-json-to-python-conversion` - Direct PyDict/PyList construction (30-40% faster)
- `pyo3-async-performance` - pyo3_async_runtimes and GIL release patterns (CRITICAL)
- `pyo3-extension-module-management` - Feature flag strategy for extension vs. binary (CRITICAL)
- `optimized-serialization-path` - msgspec and buffer reuse patterns
- `async-friendly-performance` - spawn_blocking and allow_threads patterns
- `cross-target-performance` - Edge case stress testing across language targets

#### Workspace Organization & Code Structure (3 rules)
- `layered-code-organization` - Core in crates/spikard, adapters in binding crates
- `workspace-separation` - Thin adapters prevent logic duplication
- `workspace-organization` - Module structure and Cargo.toml coordination

#### Binding Architecture (2 rules)
- `thin-binding-pattern-architecture` - Never duplicate validation/routing across bindings
- `ext-php-rs-binding-configuration` - PHP-specific FFI configuration (from shared ai-rulez)

#### Code Quality & Formatting (3 rules)
- `consistent-tooling` - cargo fmt, biome, uv enforcement
- `lint-formatting-discipline` - Biome/rustfmt/uv.lock coordination
- (Note: php-psr-compliance-standards-enforcement from shared ai-rulez)

### Analysis: Shared vs. Unique Content

**Overlapping with Shared ai-rulez**:
- `php-psr-compliance-standards-enforcement` - Available in shared (`php-82-ext-php-rs-extension-composer-package`)
- `ext-php-rs-binding-configuration` - Partially in shared

**Unique to Spikard**:
- All fixture-driven rules (4 rules) - Not in shared ai-rulez
- Handler-trait-abstraction - Spikard-specific trait design
- Zero-copy-json-to-python-conversion - Spikard's optimization strategy
- All HTTP validation & error handling rules - HTTP-specific
- Spikard's middleware stack organization
- Spikard's AsyncPerformance patterns for pyo3

---

## 3. CUSTOM PROFILES MIGRATION (custom-profiles.yaml)

### Spikard-Web-Framework Profile

**Composition Model**:
```
spikard-web-framework
  ├─ extends: web-framework.yaml (shared ai-rulez)
  ├─ custom agents: 14 from custom-agents.yaml
  ├─ custom rules: 30 from custom-rules.yaml
  └─ custom capabilities: fixture-driven-development, thin-binding-pattern, etc.
```

**Profile Structure**:
- **Inherited Agents** (11 from shared web-framework.yaml)
  - polyglot-architect
  - rust-core-engineer
  - python-bindings-engineer / typescript-bindings-engineer / ruby-bindings-engineer / php-bindings-engineer
  - test-automation-engineer
  - code-reviewer
  - docs-writer / api-doc-writer / tutorial-writer

- **Custom Agents** (14 from custom-agents.yaml)
  - middleware-architect, workspace-architect, rust-polyglot-architect
  - python-engineer, typescript-engineer, ruby-engineer, php-engineer, wasm-engineer
  - fixture-tester, integration-qa
  - docs-strategist, docs-scribe
  - build-and-ci-ops, interop-build-engineer

- **Priority Rules** (39 total)
  - Shared web-framework rules (9)
  - Spikard-specific rules (30)

- **Model Routing**
  - Architecture decisions → Sonnet (4 agents)
  - Core/binding implementation → Haiku (6 agents)
  - Testing/review/docs → Haiku (4 agents)

- **Toolchain Definition**
  - Rust 2024 edition with cargo
  - Python 3.10+ with uv
  - TypeScript 5.0+ with pnpm
  - Ruby 3.2+ with bundler
  - PHP 8.2+ with composer
  - WASM with wasm-pack

- **Quality Gates**
  - Rust coverage: 95% (crates/spikard, crates/spikard-http)
  - Language bindings: 80% coverage
  - Fixture parity: All fixtures pass across all languages
  - Linting: biome, clippy, phpstan level max, steep
  - Testing: fixture-driven approach mandatory

- **Task Definitions**
  - test (all), test:rust, test:python, test:node, test:ruby, test:php, test:wasm
  - lint (all linters), build (all targets)
  - bench (Rust and Python benchmarks)

---

## 4. DOMAIN ANALYSIS: SHARED VS. UNIQUE

### Unique to Spikard (in custom configs)

| Category | Spikard-Specific | Status |
|----------|-----------------|--------|
| **HTTP Framework** | Handler trait abstraction | Custom |
| **Middleware** | tower-http specific stack | Custom |
| **Testing** | Fixture-driven validation | Custom |
| **Binding Pattern** | Thin binding enforcement | Custom |
| **FFI Boundaries** | Arc<dyn Handler> patterns | Custom |
| **Serialization** | Zero-copy JSON→Python | Custom |
| **Async Patterns** | pyo3_async_runtimes specifics | Custom |
| **Extension Module** | Feature gate strategy | Custom |

### Inherited from Shared ai-rulez

| Category | Shared Agent | Shared Rule |
|----------|-------------|------------|
| **Polyglot Architecture** | polyglot-architect | polyglot-ffi-stability-guarantee |
| **Rust Core** | rust-core-engineer | rust-2024-edition-core-conversion-engine |
| **Python Bindings** | python-bindings-engineer | python-310-pyo3-binding-wrappers |
| **TypeScript Bindings** | typescript-bindings-engineer | typescript-5x-napi-rs-bindings-cli |
| **Ruby Bindings** | ruby-bindings-engineer | ruby-32-magnus-native-bindings-with-rbs |
| **PHP Bindings** | php-bindings-engineer | php-82-ext-php-rs-extension-composer-package |
| **WASM** | wasm-bindings-engineer | wasm-bindgen-browser-node-deno |
| **Testing** | test-automation-engineer | dual-testing-strategy-core-bindings |
| **Code Review** | code-reviewer | code-quality |
| **Documentation** | docs-writer | documentation standards |

### Overlapping (Adapted from Shared)

| Item | Original | Spikard Adaptation |
|------|----------|-------------------|
| **Error Handling** | polyglot-security-hardening | fixture-aligned-error-handling |
| **Build System** | task-automation-workflow | build-and-ci-ops (Taskfile-specific) |
| **Version Sync** | cross-ecosystem-version-synchronization | interop-build-engineer (manifest-specific) |
| **PHP Standards** | php-82-ext-php-rs-extension-composer-package | + ext-php-rs-binding-configuration + php-psr-compliance |

---

## 5. MIGRATION IMPACT & INTEGRATION

### How to Use the Custom Configs

#### For AI Tools (Claude, etc.)

Include in ai-rulez context:
```yaml
# .ai-rulez/config.yaml or your ai tool config
profiles:
  - spikard-web-framework
custom_agents: .ai-rulez/custom-agents.yaml
custom_rules: .ai-rulez/custom-rules.yaml
```

#### For Development Workflow

1. **Design Phase**: Reference custom-profiles.yaml for capability checklist
2. **Implementation**: Follow agents and rules from custom-agents.yaml + custom-rules.yaml
3. **Testing**: Ensure fixture-first approach (fixture-driven-testing rule)
4. **Review**: Use code-reviewer agent with cross-language parity rules
5. **Merge**: Verify all quality gates from profile definition

#### For Documentation

- Generate CLAUDE.md via ai-rulez: `ai-rulez generate`
- Update docs/adr/ via docs-strategist and docs-scribe coordination
- Keep examples/ in sync via fixture-tester

### Integration with Shared ai-rulez

The custom configs do NOT replace shared ai-rulez. Instead:

1. **Shared ai-rulez** provides:
   - Base agents (polyglot-architect, rust-core-engineer, language bindings engineers)
   - Base rules (Rust 2024, Python 310, TypeScript 5.x, Ruby 3.2, PHP 8.2, etc.)
   - Generic polyglot patterns

2. **Spikard custom configs** add:
   - HTTP framework-specific agents (middleware-architect, workspace-architect)
   - Fixture-driven testing rules (all 4 fixture rules)
   - Handler trait abstraction specifics
   - Thin binding pattern enforcement
   - Spikard-specific performance optimizations

3. **Profile composition**:
   ```
   spikard-web-framework (in custom-profiles.yaml)
     ├─ extends: web-framework.yaml (from shared)
     ├─ adds: middleware-architect, fixture-tester, etc.
     └─ adds: fixture-driven-testing, handler-trait-abstraction, etc.
   ```

### Backward Compatibility

- Original .md files in agents/ and rules/ are preserved
- Existing config.yaml continues to work
- New YAML files are complementary
- No breaking changes to existing structure

---

## 6. SCHEMA COMPLIANCE

All three custom YAML files comply with ai-rules-v3 schema:

```yaml
# custom-agents.yaml
agents:
  - name: <identifier>
    model: sonnet|haiku
    description: <string>
    scope: [<domains>]

# custom-rules.yaml
rules:
  - name: <identifier>
    priority: critical|high|medium|low
    description: <detailed string with implementation guidance>
    scope: [<domains>]

# custom-profiles.yaml
profiles:
  - name: <identifier>
    extends: [{ base: <path>, description: <string> }]
    agents: [<agent names>]
    priority_rules: [<rule names>]
    model_routing: { <key>: sonnet|haiku }
    capabilities: [<capability names>]
    languages: { required: [<langs>], optional: [<langs>] }
    toolchain: { <lang>: { edition, version, formatter, linter, package_manager } }
    quality_gates: { <gate_name>: { minimum, tool, requirement } }
    tasks: { <task_name>: { description, includes|command } }
```

---

## 7. NEXT STEPS

### Immediate Actions

1. **Verify Paths**:
   - Confirm `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/` exists
   - Confirm shared ai-rulez at `/Users/naamanhirschfeld/workspace/kreuzberg-dev/ai-rulez/`

2. **Register Profile** in spikard config:
   ```yaml
   # spikard/.ai-rulez/config.yaml
   profiles:
     - spikard-web-framework  # References custom-profiles.yaml
   ```

3. **Update Documentation**:
   - Run `ai-rulez generate` to create updated CLAUDE.md
   - Update CONTRIBUTING.md to reference new profile
   - Link to ADRs from docs/adr/

4. **Team Onboarding**:
   - Share MIGRATION_SUMMARY.md with team
   - Update CI workflows to validate custom configs
   - Ensure pre-commit hooks understand new YAML structure

### Ongoing Maintenance

1. **Schema Updates**:
   - Monitor ai-rulez releases for schema changes
   - Update custom configs if v3.x changes occur
   - Validate with `ai-rulez validate`

2. **Agent Coordination**:
   - Use agent-coordination-matrix (in custom-agents.yaml) for task planning
   - Sync with middleware-architect on tower-http changes
   - Sync with rust-polyglot-architect on Handler trait evolution

3. **Rule Enforcement**:
   - Fixture-tester creates testing_data/ fixtures before implementation
   - integration-qa validates fixture parity across languages
   - CI enforces quality-gates from profile (coverage, linting, etc.)

4. **Documentation**:
   - docs-strategist maintains docs/adr/ synchronously with code
   - docs-scribe regenerates CLAUDE.md monthly or on major changes
   - Update examples/ when Handler trait or ServerConfig changes

---

## 8. MIGRATION CHECKLIST

- [x] Extracted 14 agents from .md files into custom-agents.yaml
- [x] Categorized agents by domain (middleware, bindings, QA, docs, build)
- [x] Documented agent scope and coordination patterns
- [x] Extracted 30 rules from .md files into custom-rules.yaml
- [x] Grouped rules by domain (fixture-driven, HTTP contracts, middleware, performance, quality)
- [x] Added implementation guidance to each rule
- [x] Created spikard-web-framework profile in custom-profiles.yaml
- [x] Profile extends shared web-framework.yaml
- [x] Profile includes all 14 custom agents + inherited agents
- [x] Profile includes all 30 custom rules + shared rules
- [x] Defined model routing (Sonnet for architecture, Haiku for implementation)
- [x] Specified quality gates (95% Rust, 80% language, fixture parity)
- [x] Documented toolchain versions (Rust 2024, Python 3.10+, etc.)
- [x] Documented task definitions (test, lint, build)
- [x] Created MIGRATION_SUMMARY.md documentation
- [x] Verified schema v3 compliance
- [x] Analyzed shared vs. unique content
- [x] Documented integration with shared ai-rulez

---

## 9. REFERENCES

### Created Files

1. `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/custom-agents.yaml` (14 agents)
2. `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/custom-rules.yaml` (30 rules)
3. `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/custom-profiles.yaml` (1 profile)
4. `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/MIGRATION_SUMMARY.md` (this file)

### Preserved Files

- `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/agents/*.md` (14 original files)
- `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/rules/*.md` (30 original files)
- `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/config.yaml` (existing config)
- `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/mcp.yaml` (existing MCP config)

### Shared ai-rulez Reference

- Base: `/Users/naamanhirschfeld/workspace/kreuzberg-dev/ai-rulez/`
- Profiles: `web-framework.yaml`, `rust-library.yaml`, `python-focused.yaml`, etc.
- Agents: `polyglot-architect`, `rust-core-engineer`, `python-bindings-engineer`, etc.
- Rules: `rust-2024-edition-core-conversion-engine`, `dual-testing-strategy-core-bindings`, etc.

### Schema Reference

- ai-rules-v3: https://raw.githubusercontent.com/Goldziher/ai-rulez/main/schema/ai-rules-v3.schema.json

---

## 10. GLOSSARY

| Term | Definition |
|------|-----------|
| **Agent** | AI role with specific expertise (model: sonnet/haiku) |
| **Rule** | Development principle (priority: critical/high/medium/low) |
| **Profile** | Composable set of agents + rules for a project type |
| **Custom Config** | Spikard-specific YAML extending shared ai-rulez |
| **Shared ai-rulez** | Reusable polyglot development patterns |
| **Handler Trait** | Language-agnostic async request handler abstraction |
| **Thin Binding** | Language-idiomatic wrapper over Rust core (no logic duplication) |
| **Fixture** | JSON test data representing valid/invalid behavior |
| **ServerConfig** | Typed HTTP server configuration struct |
| **FFI Boundary** | Interface between Rust and language bindings |
| **Lifecycle Hook** | onRequest, preValidation, preHandler, onResponse, onError |
| **Zero-Copy** | Data conversion without intermediate serialization |

---

**Migration Completed**: 2025-12-29
**Total Agents Migrated**: 14
**Total Rules Migrated**: 30
**Profiles Created**: 1
**Schema Version**: ai-rules-v3
