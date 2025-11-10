# Spikard Design Documents

This directory contains all design documents for the Spikard framework.

## Document Status

| Status | Meaning |
|--------|---------|
| 🟢 **Active** | Current, implemented or in implementation |
| 🟡 **Draft** | Under review, not yet finalized |
| 🔵 **Research** | Research phase, decision pending |
| ⚪ **Reference** | Historical or reference material |

## Core Design Documents

### Foundation (Read First)

| Document | Status | Description |
|----------|--------|-------------|
| [summary.md](./summary.md) | 🟢 Active | High-level overview of Spikard architecture and design decisions |
| [architecture.md](./architecture.md) | 🟢 Active | Core architectural patterns and module organization |
| [TEMPLATE.md](./TEMPLATE.md) | ⚪ Reference | Standard template for new design documents |

### Validation & Testing

| Document | Status | Description |
|----------|--------|-------------|
| [validation-strategy.md](./validation-strategy.md) | 🟢 Active | JSON Schema-based validation approach across all bindings |
| [testing-strategy.md](./testing-strategy.md) | 🟢 Active | Fixture-driven testing methodology (367 comprehensive test scenarios) |
| [ecosystem-audit-2025.md](./ecosystem-audit-2025.md) | 🟢 Active | **⭐ Comprehensive ecosystem audit** - identifies 12 areas where we're reinventing the wheel (~770 lines of code to remove) |
| [ecosystem-audit-2025-summary.md](./ecosystem-audit-2025-summary.md) | 🟢 Active | Quick reference guide for ecosystem audit - critical issues and quick wins |
| [ecosystem-alternatives.md](./ecosystem-alternatives.md) | 🟢 Active | Detailed comparison of alternative crates (cookies, date/time, validation, etc.) |
| [metaprogramming-test-system.md](./metaprogramming-test-system.md) | 🟡 Draft | Dynamic test and handler generation from fixtures |

### API Design & Code Generation

| Document | Status | Description |
|----------|--------|-------------|
| [api-design.md](./api-design.md) | 🟢 Active | Complete API specifications for Python, TypeScript, and Rust |
| [unified-config-format.md](./unified-config-format.md) | 🟡 Draft | YAML/JSON schema for declarative server definition and code generation |
| [axum-routing.md](./axum-routing.md) | 🟢 Active | Axum-based routing implementation details |
| [codegen-strategy.md](./codegen-strategy.md) | 🟢 Active | Multi-language code generation from OpenAPI specs |

### Middleware & Lifecycle

| Document | Status | Description |
|----------|--------|-------------|
| [dependency-injection-middleware.md](./dependency-injection-middleware.md) | 🟡 Draft | Original DI and middleware design (superseded by newer docs) |
| [middleware-lifecycle-optimization.md](./middleware-lifecycle-optimization.md) | 🟢 Active | **⭐ Tower-http middleware (IMPLEMENTED) and Fastify-inspired lifecycle hooks (PENDING)** |
| [lifecycle-hooks-implementation.md](./lifecycle-hooks-implementation.md) | 🟡 **NEW** | **Detailed implementation guide for lifecycle hooks with examples and test plans** |
| [dependency-injection.md](./dependency-injection.md) | 🔵 Research | DI feasibility analysis and recommendation |

### Observability

| Document | Status | Description |
|----------|--------|-------------|
| [observability-openapi.md](./observability-openapi.md) | 🟢 Active | OpenTelemetry instrumentation and OpenAPI generation |

### Implementation Details

| Document | Status | Description |
|----------|--------|-------------|
| [msgspec-type-conversion.md](./msgspec-type-conversion.md) | 🟢 Active | msgspec integration for Python type conversion |

## Reading Paths

### For New Contributors
1. Start with [summary.md](./summary.md) - Get the big picture
2. Read [architecture.md](./architecture.md) - Understand the layers
3. Read [validation-strategy.md](./validation-strategy.md) - Core validation approach
4. Read [api-design.md](./api-design.md) - User-facing APIs

### For Addressing Technical Debt ⭐ NEW
1. [ecosystem-audit-2025-summary.md](./ecosystem-audit-2025-summary.md) - Start here for critical issues
2. [ecosystem-audit-2025.md](./ecosystem-audit-2025.md) - Full audit with migration strategy
3. [ecosystem-alternatives.md](./ecosystem-alternatives.md) - Compare alternative libraries

### For Implementing Middleware/Hooks ⭐ UPDATED
1. [middleware-lifecycle-optimization.md](./middleware-lifecycle-optimization.md) - **Phase 1 (Middleware) COMPLETE, Phase 2 (Hooks) PENDING**
2. [dependency-injection-middleware.md](./dependency-injection-middleware.md) - Background context

### For Implementing DI
1. [dependency-injection.md](./dependency-injection.md) - Analysis and decision
2. [dependency-injection-middleware.md](./dependency-injection-middleware.md) - Original ideas
3. Recommendation: **Binding-level DI** (language-native patterns)

### For Implementing Observability
1. [observability-openapi.md](./observability-openapi.md) - Complete OTEL design
2. [middleware-lifecycle-optimization.md](./middleware-lifecycle-optimization.md) - Middleware integration

### For Testing
1. [testing-strategy.md](./testing-strategy.md) - Fixture-driven approach
2. Check `testing_data/` directory for existing fixtures

## Design Principles

All design documents should follow these principles:

### 1. **Rust-First, Multi-Language**
- Core logic in Rust for performance and safety
- Language bindings provide idiomatic APIs
- Consistent behavior across all bindings

### 2. **JSON Schema as Contract**
- Universal validation format
- OpenAPI generation
- Cross-language type safety

### 3. **Zero-Cost Abstractions**
- Feature flags for optional components
- Conditional compilation
- No runtime overhead for unused features

### 4. **Battle-Tested Dependencies**
- Prefer mature, well-maintained crates (Axum, tower-http, OpenTelemetry)
- Permissive licenses only (MIT, Apache-2.0)
- Production-proven performance

### 5. **Developer Experience First**
- Type-safe APIs in all languages
- Comprehensive error messages
- Excellent documentation

## Document Format

All design documents should follow the structure in [TEMPLATE.md](./TEMPLATE.md):

1. **Header**: Title, date, status, related docs
2. **Executive Summary**: 2-3 sentence overview
3. **Overview**: Goals and non-goals
4. **Background/Context**: Research and prior art
5. **Design**: Detailed technical design
6. **API Design**: Language-specific APIs
7. **Implementation Strategy**: Phased rollout plan
8. **Performance**: Metrics and benchmarks
9. **Testing**: Test strategy
10. **References**: Specs, libraries, prior art
11. **Key Takeaway**: 1-2 sentence summary

## Contributing

When creating a new design document:

1. Copy [TEMPLATE.md](./TEMPLATE.md)
2. Use descriptive filenames without number prefixes (e.g., `new-feature.md`)
3. Set status to 🟡 **Draft** initially
4. Update this README.md with a link and description
5. Mark as 🟢 **Active** once implemented/finalized

## Research Sources (2024-2025)

Our design documents are informed by:

- **Axum ecosystem**: tower-http, axum-tracing-opentelemetry
- **Python frameworks**: Litestar, FastAPI, Robyn
- **TypeScript frameworks**: Fastify, NestJS, Express
- **OpenTelemetry**: Official Rust, Python, JS implementations
- **Validation**: JSON Schema, Pydantic, Zod, msgspec

## Decision Log

| Date | Decision | Document | Status |
|------|----------|----------|--------|
| 2025-01 | Use Litestar-style route decorators (@get, not app.get) | api-design.md | ✅ Decided |
| 2025-01 | All middleware in Rust (tower-http) | middleware-lifecycle-optimization.md | ✅ **IMPLEMENTED** |
| 2025-01 | Fastify-inspired lifecycle hooks with zero-cost | middleware-lifecycle-optimization.md | 🟡 Designed, pending implementation |
| 2025-01 | OpenTelemetry optional feature with full instrumentation | observability-openapi.md | ✅ Decided |
| 2025-01 | OpenAPI 3.1 generation from JSON Schema | observability-openapi.md | 🟡 Pending implementation |
| 2025-01 | Binding-level DI (not Rust-based) | dependency-injection.md | ✅ Decided |
| 2025-01 | TypeScript functional API (no decorators) | api-design.md | ✅ Decided |
| 2025-10-31 | Replace manual cookie parsing with `cookie` crate | ecosystem-audit-2025.md | 🔴 High Priority |
| 2025-10-31 | Replace manual date/time validation with `jiff` or `chrono` | ecosystem-audit-2025.md | 🔴 High Priority |
| 2025-10-31 | Add `garde` or `validator` for declarative validation | ecosystem-audit-2025.md | 🔴 High Priority |
| 2025-11 | Implement all tower-http middleware with typed ServerConfig | middleware-lifecycle-optimization.md | ✅ **IMPLEMENTED** |

---

**Questions?** Open an issue or discussion on GitHub.
