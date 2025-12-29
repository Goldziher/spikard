# Spikard AI-Rulez Quick Reference

A quick lookup guide for the migrated spikard configuration files.

## File Locations

```
/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/
├── custom-agents.yaml          # 14 spikard-specific agents
├── custom-rules.yaml           # 30 spikard-specific rules
├── custom-profiles.yaml        # 1 spikard-web-framework profile
├── MIGRATION_SUMMARY.md        # Detailed migration documentation
├── QUICK_REFERENCE.md          # This file
├── config.yaml                 # Base spikard config (unchanged)
├── mcp.yaml                    # MCP configuration (unchanged)
├── agents/                     # Original agent .md files (preserved)
├── rules/                      # Original rule .md files (preserved)
└── skills/                     # Skills directory
```

## Agent Quick Lookup

### Architecture & Design (Use Sonnet)
- **middleware-architect**: tower-http middleware stack, ServerConfig, lifecycle hooks
- **workspace-architect**: Cargo workspace structure, crate layering, FFI isolation
- **rust-polyglot-architect**: Handler trait design, error contracts, cross-language semantics

### Language-Specific HTTP Implementations (Use Haiku)
- **python-engineer**: crates/spikard-py, PyO3, pyo3_async_runtimes, msgspec
- **typescript-engineer**: crates/spikard-node, napi-rs, TypeScript types, ThreadsafeFunction
- **ruby-engineer**: crates/spikard-rb, magnus/rb-sys, RBS types, Steep
- **php-engineer**: crates/spikard-php, ext-php-rs, PSR compliance, PHPStan
- **wasm-engineer**: crates/spikard-wasm, wasm-bindgen, WASM optimization

### Testing & Quality (Use Haiku)
- **fixture-tester**: testing_data/ schemas, fixture creation, parametrized tests
- **integration-qa**: Cross-language fixture parity, regression testing, coverage enforcement

### Documentation (Use Haiku)
- **docs-strategist**: docs/adr/, architecture documentation, example currency
- **docs-scribe**: CLAUDE.md generation, agent handbooks, API documentation

### Build & Operations (Use Haiku)
- **build-and-ci-ops**: Taskfile.yaml, CI workflows, lock file management
- **interop-build-engineer**: Cargo.toml, pyproject.toml, package.json, Gemfile, composer.json

### Inherited from Shared ai-rulez
- **polyglot-architect**: Overall system design
- **rust-core-engineer**: HTTP core (crates/spikard-http)
- **test-automation-engineer**: E2E test generation
- **code-reviewer**: Multi-language code review
- **docs-writer**, **api-doc-writer**: General documentation

## Rule Quick Lookup

### Fixture-Driven Testing (CRITICAL)
- **fixture-driven-testing**: Every feature extends pytest suite with fixtures
- **fixture-first-testing**: Design via fixtures before implementation
- **fixture-backed-testing**: Keep testing_data/ and tests synchronized
- **fixture-aligned-error-handling**: Error serialization via fixture schemas

### Handler & HTTP Contracts (CRITICAL/HIGH)
- **handler-trait-abstraction**: Arc<dyn Handler> with Pin<Box<dyn Future>>
- **http-input-validation**: Schema-driven header, cookie, body validation
- **http-error-contracts**: Error translation to fixture-defined JSON
- **header-cookie-security**: Security defaults and validation
- **request-surface-security**: Comprehensive validation layer strategy

### FFI & Error Handling (CRITICAL)
- **cross-language-error-boundaries**: Error conversion at FFI boundaries
- **thin-binding-pattern-architecture**: No logic duplication across bindings
- **ext-php-rs-binding-configuration**: PHP-specific FFI configuration
- **php-psr-compliance-standards-enforcement**: PSR standards enforcement

### Performance & Serialization (CRITICAL/HIGH)
- **zero-copy-json-to-python-conversion**: Direct PyDict/PyList construction (30-40% faster)
- **pyo3-async-performance**: pyo3_async_runtimes and GIL release patterns
- **pyo3-extension-module-management**: Feature flag strategy for extension vs. binary
- **optimized-serialization-path**: msgspec and buffer reuse
- **async-friendly-performance**: spawn_blocking and allow_threads patterns
- **cross-target-performance**: Edge case stress testing

### Middleware & Configuration (HIGH)
- **tower-http-middleware-stack**: Middleware composition and ServerConfig
- **lifecycle-hooks-implementation**: Zero-cost lifecycle hook design

### Code Organization (HIGH/MEDIUM)
- **layered-code-organization**: Core in crates/spikard, adapters in bindings
- **workspace-separation**: Thin adapters prevent logic duplication
- **workspace-organization**: Module structure and Cargo.toml coordination
- **consistent-tooling**: cargo fmt, biome, uv enforcement
- **lint-formatting-discipline**: Biome/rustfmt/uv.lock coordination

## Quick Decision Matrix

### When to Use Which Agent?

| Situation | Agent | Model |
|-----------|-------|-------|
| Design HTTP middleware | middleware-architect | Sonnet |
| Plan crate structure | workspace-architect | Sonnet |
| Design Handler trait | rust-polyglot-architect | Sonnet |
| Implement Python bindings | python-engineer | Haiku |
| Implement Node/TS bindings | typescript-engineer | Haiku |
| Create/evolve fixtures | fixture-tester | Haiku |
| Validate cross-language parity | integration-qa | Haiku |
| Write ADRs | docs-strategist | Haiku |
| Generate CLAUDE.md | docs-scribe | Haiku |
| Coordinate builds | build-and-ci-ops | Haiku |
| Sync manifests | interop-build-engineer | Haiku |

### When to Apply Which Rule?

| Scenario | Rule | Priority |
|----------|------|----------|
| Add new feature | fixture-driven-testing | CRITICAL |
| Design validation | http-input-validation | HIGH |
| Implement error handling | fixture-aligned-error-handling | CRITICAL |
| Design async Python handler | pyo3-async-performance | CRITICAL |
| Create Rust handler | handler-trait-abstraction | CRITICAL |
| Add HTTP endpoints | request-surface-security | HIGH |
| Implement middleware | tower-http-middleware-stack | HIGH |
| Commit code | lint-formatting-discipline | MEDIUM |

## Profile Quick Start

### To Use spikard-web-framework Profile:

Add to spikard/.ai-rulez/config.yaml:
```yaml
profiles:
  - spikard-web-framework
```

Run ai-rulez tooling:
```bash
ai-rulez generate              # Create CLAUDE.md
ai-rulez validate              # Check configuration
ai-rulez list-agents           # See all available agents
```

### Profile Includes:

- **14 custom agents** from custom-agents.yaml
- **11 inherited agents** from shared web-framework.yaml
- **30 custom rules** from custom-rules.yaml
- **9 inherited rules** from shared web-framework.yaml
- **Rust 2024 + Python 3.10+ + TS 5.0+ + Ruby 3.2+ + PHP 8.2+ support**
- **Quality gates**: 95% Rust coverage, 80% language coverage, fixture parity
- **Toolchain**: cargo, uv, pnpm, bundler, composer, wasm-pack

## Common Workflows

### Adding a New HTTP Handler

1. **fixture-tester**: Create fixture(s) in testing_data/<domain>/
2. **fixture-tester**: Update testing_data/<domain>/schema.json
3. **rust-polyglot-architect**: Review Handler trait requirements
4. **rust-core-engineer**: Implement in crates/spikard-http
5. **fixture-tester**: Create parametrized test in packages/python/tests/
6. **integration-qa**: Validate cross-language fixture parity
7. **code-reviewer**: Multi-language code review
8. **build-and-ci-ops**: Ensure CI passes all targets

### Adding a New Language Binding

1. **rust-polyglot-architect**: Design FFI boundary
2. **workspace-architect**: Plan crate and manifest structure
3. **interop-build-engineer**: Coordinate manifest files
4. **{python,typescript,ruby,php,wasm}-engineer**: Implement binding
5. **fixture-tester**: Ensure all fixtures represented
6. **integration-qa**: Validate fixture parity with other languages
7. **docs-strategist**: Document binding in ADR
8. **build-and-ci-ops**: Add language-specific build tasks

### Fixing a Cross-Language Bug

1. **integration-qa**: Identify which fixtures fail
2. **fixture-tester**: Review fixture schema vs. implementation
3. **rust-polyglot-architect**: Review error contract or Handler contract
4. **{python,typescript,ruby,php,wasm}-engineer**: Fix language binding
5. **integration-qa**: Validate fix across all languages
6. **code-reviewer**: Multi-language review
7. **build-and-ci-ops**: Ensure CI passes all targets

### Optimizing Performance

1. **workspace-architect**: Review crate layering for bottlenecks
2. **cross-target-performance** rule: Use testing_data/edge_cases for benchmarks
3. **zero-copy-json-to-python-conversion** rule: Profile JSON conversion
4. **pyo3-async-performance** rule: Profile GIL usage in Python
5. **async-friendly-performance** rule: Review spawn_blocking patterns
6. **build-and-ci-ops**: Run task bench to measure improvements

## File Cross-References

### custom-agents.yaml
- See "Agent Coordination Matrix" for communication patterns
- See "Scope" sections for each agent's domain

### custom-rules.yaml
- See "Rule Coordination & Enforcement" for rule relationships
- See "Implementation" sections for guidance
- See "Scope" sections for affected domains

### custom-profiles.yaml
- See "Agent Coordination & Enforcement" for how agents work together
- See "Quality Gates" for success criteria
- See "Task Definitions" for Taskfile.yaml reference
- See "Coordination" section for meeting cadences

### MIGRATION_SUMMARY.md
- See "Domain Analysis" for shared vs. unique breakdown
- See "Integration with Shared ai-rulez" for composition model
- See "Next Steps" for onboarding guidance

## Key Concepts

### Handler Trait Pattern
Spikard's core abstraction for language-agnostic HTTP request handling:
```rust
pub trait Handler: Send + Sync {
    fn call(&self, request: HttpRequest)
        -> Pin<Box<dyn Future<Output = HandlerResult> + Send>>;
}
```

All language bindings implement this trait via Arc<dyn Handler> wrappers. The HTTP server never knows about language-specific code.

### Thin Binding Pattern
All binding crates (spikard-py, spikard-node, etc.) provide only:
- Type translation (language types ↔ Rust types)
- Configuration APIs (ServerConfig exposure)
- Error conversion (Rust Result → language exceptions)

Business logic, validation, middleware, and routing ALL live in Rust.

### Fixture-First Development
1. Define expected behavior as JSON fixtures (testing_data/)
2. Create schema (testing_data/*/schema.json)
3. Write parametrized tests loading fixtures
4. Run tests (should fail initially)
5. Implement handler to satisfy fixtures
6. Verify cross-language fixture parity

### Zero-Cost Lifecycle Hooks
Lifecycle hooks use Option<Arc<dyn Fn>> for efficiency:
- When not registered: Option.is_none() check (< 1ns)
- When registered: Execute async hook, allow short-circuiting
- Hooks: onRequest, preValidation, preHandler, onResponse, onError

## Links & References

- **Shared ai-rulez**: `/Users/naamanhirschfeld/workspace/kreuzberg-dev/ai-rulez/`
- **Web Framework Profile**: `/Users/naamanhirschfeld/workspace/kreuzberg-dev/ai-rulez/profiles/web-framework.yaml`
- **Schema**: https://raw.githubusercontent.com/Goldziher/ai-rulez/main/schema/ai-rules-v3.schema.json
- **ADRs**: spikard/docs/adr/
- **Examples**: spikard/examples/
- **Tests**: spikard/packages/python/tests/

## Maintenance Checklist

- [ ] Monthly: Regenerate CLAUDE.md via `ai-rulez generate`
- [ ] Per PR: Verify fixture coverage (fixture-driven-testing rule)
- [ ] Per PR: Validate cross-language parity (integration-qa)
- [ ] Per sprint: Sync ADRs with code changes (docs-strategist)
- [ ] Per release: Update example code (docs-strategist)
- [ ] Quarterly: Review quality gates from profile
- [ ] Quarterly: Update toolchain versions if needed

---

**Last Updated**: 2025-12-29
**Profile Version**: spikard-web-framework v1.0
**Schema**: ai-rules-v3
