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
| [00-summary.md](./00-summary.md) | 🟢 Active | High-level overview of Spikard architecture and design decisions |
| [00-architecture.md](./00-architecture.md) | 🟢 Active | Core architectural patterns and module organization |
| [00-TEMPLATE.md](./00-TEMPLATE.md) | ⚪ Reference | Standard template for new design documents |

### Validation & Testing

| Document | Status | Description |
|----------|--------|-------------|
| [01-validation-strategy.md](./01-validation-strategy.md) | 🟢 Active | JSON Schema-based validation approach across all bindings |
| [02-testing-strategy.md](./02-testing-strategy.md) | 🟢 Active | Fixture-driven testing methodology (367 comprehensive test scenarios) |
| [03-ecosystem-audit-2025.md](./03-ecosystem-audit-2025.md) | 🟢 Active | **⭐ Comprehensive ecosystem audit** - identifies 12 areas where we're reinventing the wheel (~770 lines of code to remove) |
| [03-ecosystem-audit-2025-summary.md](./03-ecosystem-audit-2025-summary.md) | 🟢 Active | Quick reference guide for ecosystem audit - critical issues and quick wins |
| [03-ecosystem-alternatives.md](./03-ecosystem-alternatives.md) | 🟢 Active | Detailed comparison of alternative crates (cookies, date/time, validation, etc.) |
| [08-metaprogramming-test-system.md](./08-metaprogramming-test-system.md) | 🟡 Draft | Dynamic test and handler generation from fixtures |

### API Design & Code Generation

| Document | Status | Description |
|----------|--------|-------------|
| [03-api-design.md](./03-api-design.md) | 🟢 Active | Complete API specifications for Python, TypeScript, and Rust |
| [09-unified-config-format.md](./09-unified-config-format.md) | 🟡 Draft | YAML/JSON schema for declarative server definition and code generation |
| [axum-routing.md](./axum-routing.md) | 🟢 Active | Axum-based routing implementation details |

### Middleware & Lifecycle

| Document | Status | Description |
|----------|--------|-------------|
| [04-dependency-injection-middleware.md](./04-dependency-injection-middleware.md) | 🟡 Draft | Original DI and middleware design (superseded by 05 and 07) |
| [05-middleware-lifecycle-optimization.md](./05-middleware-lifecycle-optimization.md) | 🟢 Active | Tower-http middleware and Fastify-inspired lifecycle hooks |
| [07-dependency-injection-analysis.md](./07-dependency-injection-analysis.md) | 🔵 Research | DI feasibility analysis and recommendation |

### Observability

| Document | Status | Description |
|----------|--------|-------------|
| [06-observability-openapi.md](./06-observability-openapi.md) | 🟢 Active | OpenTelemetry instrumentation and OpenAPI generation |

### Implementation Details

| Document | Status | Description |
|----------|--------|-------------|
| [msgspec-type-conversion.md](./msgspec-type-conversion.md) | 🟢 Active | msgspec integration for Python type conversion |

## Reading Paths

### For New Contributors
1. Start with [00-summary.md](./00-summary.md) - Get the big picture
2. Read [00-architecture.md](./00-architecture.md) - Understand the layers
3. Read [01-validation-strategy.md](./01-validation-strategy.md) - Core validation approach
4. Read [03-api-design.md](./03-api-design.md) - User-facing APIs

### For Addressing Technical Debt ⭐ NEW
1. [03-ecosystem-audit-2025-summary.md](./03-ecosystem-audit-2025-summary.md) - Start here for critical issues
2. [03-ecosystem-audit-2025.md](./03-ecosystem-audit-2025.md) - Full audit with migration strategy
3. [03-ecosystem-alternatives.md](./03-ecosystem-alternatives.md) - Compare alternative libraries

### For Implementing Middleware/Hooks
1. [05-middleware-lifecycle-optimization.md](./05-middleware-lifecycle-optimization.md) - Primary design
2. [04-dependency-injection-middleware.md](./04-dependency-injection-middleware.md) - Background context

### For Implementing DI
1. [07-dependency-injection-analysis.md](./07-dependency-injection-analysis.md) - Analysis and decision
2. [04-dependency-injection-middleware.md](./04-dependency-injection-middleware.md) - Original ideas
3. Recommendation: **Binding-level DI** (language-native patterns)

### For Implementing Observability
1. [06-observability-openapi.md](./06-observability-openapi.md) - Complete OTEL design
2. [05-middleware-lifecycle-optimization.md](./05-middleware-lifecycle-optimization.md) - Middleware integration

### For Testing
1. [02-testing-strategy.md](./02-testing-strategy.md) - Fixture-driven approach
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

All design documents should follow the structure in [00-TEMPLATE.md](./00-TEMPLATE.md):

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

1. Copy [00-TEMPLATE.md](./00-TEMPLATE.md)
2. Follow the numbering scheme (e.g., `08-new-feature.md`)
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
| 2025-01 | Use Litestar-style route decorators (@get, not app.get) | 03-api-design.md | ✅ Decided |
| 2025-01 | All middleware in Rust (tower-http) | 05-middleware-lifecycle-optimization.md | ✅ Decided |
| 2025-01 | Fastify-inspired lifecycle hooks with zero-cost | 05-middleware-lifecycle-optimization.md | ✅ Decided |
| 2025-01 | OpenTelemetry optional feature with full instrumentation | 06-observability-openapi.md | ✅ Decided |
| 2025-01 | OpenAPI 3.1 generation from JSON Schema | 06-observability-openapi.md | ✅ Decided |
| 2025-01 | Binding-level DI (not Rust-based) | 07-dependency-injection-analysis.md | ✅ Decided |
| 2025-01 | TypeScript functional API (no decorators) | 03-api-design.md | ✅ Decided |
| 2025-10-31 | Replace manual cookie parsing with `cookie` crate | 03-ecosystem-audit-2025.md | 🔴 High Priority |
| 2025-10-31 | Replace manual date/time validation with `jiff` or `chrono` | 03-ecosystem-audit-2025.md | 🔴 High Priority |
| 2025-10-31 | Add `garde` or `validator` for declarative validation | 03-ecosystem-audit-2025.md | 🔴 High Priority |

---

**Questions?** Open an issue or discussion on GitHub.
