# GraphQL & Protobuf Library Research for Spikard v1.0

**Date:** 2025-12-27
**Status:** Comprehensive evaluation completed
**Purpose:** Identify suitable Rust libraries for GraphQL and Protobuf/gRPC support in Spikard

---

## Executive Summary

### GraphQL Recommendation: **async-graphql**
- **Integration Complexity:** 4/10 (Moderate - Straightforward)
- **License:** MIT OR Apache-2.0 ✓
- **Stars:** 3,600+
- **Status:** Production-ready, actively maintained (last commit: May 2025)
- **Verdict:** HIGHLY RECOMMENDED - Best fit for Spikard's architecture

### Protobuf/gRPC Recommendation: **tonic**
- **Integration Complexity:** 6/10 (Moderate - requires multi-language wrapper effort)
- **License:** MIT ✓
- **Stars:** 11,700+
- **Status:** Production-ready, actively maintained (v0.14.2, Sept 2025)
- **Verdict:** RECOMMENDED for gRPC - Skip prost-only approach

---

## GraphQL Libraries Evaluated

### 1. async-graphql (WINNER)

**Repository:** https://github.com/async-graphql/async-graphql
**Version:** 7.0.17 (May 2025)
**MSRV:** Rust 1.86.0

#### Strengths
✅ **Zero unsafe code** - `#![forbid(unsafe_code)]` across entire codebase
✅ **Native Axum integration** - `async-graphql-axum` crate (~400 LOC)
✅ **Apollo Federation v2 support** - Critical for microservices
✅ **Comprehensive features** - Subscriptions, introspection, tracing, security
✅ **Clean Handler abstraction** - Executor trait maps perfectly to Spikard's Handler
✅ **Tower-compatible** - Works seamlessly with tower-http middleware
✅ **Excellent documentation** - Book, rustdoc, multi-language guides

#### Integration Points
- **Executor trait** abstracts GraphQL execution (language-agnostic)
- Request/Response via `GraphQLRequest`/`GraphQLResponse` extractors
- WebSocket subscriptions via `GraphQLSubscription` service
- Zero architectural conflicts with Spikard's middleware stack

#### Minor Considerations
⚠️ Uses `serde_json::Value` (not msgspec) - requires conversion adapter for Python
⚠️ Brings own WebSocket protocols - use `execute_stream()` only, delegate protocol to Spikard

#### Integration Estimate
- MVP integration: 1-2 weeks
- Full multi-language support: 3-4 weeks
- Production hardening: 2-3 weeks
- **Total: 6-9 weeks**

#### Recommended Architecture
```rust
spikard-graphql (new crate)
├─ Wraps async-graphql::Schema
├─ Implements Handler trait for Executor
├─ Type conversions (Value ↔ JSON/msgspec)
└─ NO WebSocket protocols (delegated to spikard-http)

spikard-http (existing)
├─ Uses spikard-graphql Handler
├─ WebSocket protocol negotiation
└─ Tower-http middleware stack

Bindings (spikard-py, spikard-node, etc.)
├─ Expose Schema class
├─ Query/Mutation/Subscription decorators
└─ Conversion layer for Value ↔ msgspec
```

---

### 2. juniper (FALLBACK OPTION)

**Repository:** https://github.com/graphql-rust/juniper
**Version:** 0.17.0 (Nov 2025)
**MSRV:** Rust 1.85

#### Strengths
✅ Mature (since 2016), battle-tested
✅ Code-first with strong type safety
✅ Lightweight dependencies (~12 core)
✅ Clean Axum integration (`juniper_axum`)
✅ BSD 2-Clause license (permissive)

#### Weaknesses
❌ **No Apollo Federation support** - Major limitation for distributed GraphQL
❌ **No SDL-first approach** - Code-first only (or external `juniper-from-schema`)
❌ **Harder FFI binding** - RootNode generic over context, more complex for multi-language
⚠️ Incomplete directive validation (TODOs in code)

#### When to Choose Juniper
- Federation is NOT required
- Prefer mature, lightweight codebase
- Rust-first development (bindings are secondary)

#### Integration Complexity: 5/10

---

## Protobuf/gRPC Libraries Evaluated

### 3. tonic (WINNER for gRPC)

**Repository:** https://github.com/hyperium/tonic
**Version:** 0.14.2 (Sept 2025)
**MSRV:** Rust 1.75

#### Strengths
✅ **Production-grade gRPC** - Full HTTP/2, streaming (unary, server, client, bidirectional)
✅ **Tower-native** - Built on tower::Service, perfect middleware integration
✅ **Axum Routes support** - `Routes::into_axum_router()` for mixed gRPC+REST
✅ **Comprehensive examples** - 30+ examples covering all major features
✅ **Clean separation** - Server traits + transport layer
✅ **Feature-gated** - Granular control (compression, TLS, reflection, health)

#### Integration Points
- **tonic-build** generates server traits from `.proto` files
- Implements `Service<Request>` for tower compatibility
- Can coexist with REST handlers on same axum::Router
- gRPC-Web support via `tonic-web` (browser clients)

#### Multi-Language Strategy
Each binding (Python/Node/Ruby/PHP) needs wrapper code:
1. tonic codegen → Rust server trait
2. Convert `tonic::Request` → language types
3. Call language handler
4. Convert response → `tonic::Response`
5. Handle streaming via language async patterns

**Effort per language:** 1-2 weeks
**Total for all bindings:** 6-10 weeks

#### When to Use Separate Ports
- gRPC requires HTTP/2 (ALPN negotiation)
- REST typically uses HTTP/1.1
- **Recommendation:** gRPC on 50051, REST on 8080
- **Alternative:** gRPC-Web for browser clients (HTTP/1.1 compatible)

#### Integration Complexity: 6/10

---

### 4. prost (NOT RECOMMENDED for Spikard)

**Repository:** https://github.com/tokio-rs/prost
**Version:** 0.14.2 (Dec 2025)
**MSRV:** Rust 1.82

#### Why NOT Recommended
❌ **Rust-only codegen** - No multi-language support (tonic handles this better)
❌ **Protobuf over HTTP is awkward** - Requires custom extractors/responses
❌ **Adds complexity without benefit** - JSON+msgspec already optimized
❌ **Passively maintained** - Google's official Rust protobuf coming
❌ **No RPC support** - Would need to build entire RPC layer

#### When prost Makes Sense
- Internal Rust microservices (single language)
- Embedded systems (no_std support)
- As dependency of tonic (already included)

#### For Spikard
**Verdict:** Stick with JSON+msgspec for HTTP APIs. Use tonic if gRPC needed.

#### Integration Complexity: 4/10 (but not worthwhile)

---

## Comparison Matrix

| Feature | async-graphql | juniper | tonic | prost |
|---------|--------------|---------|-------|-------|
| **Primary Use** | GraphQL | GraphQL | gRPC | Protobuf |
| **Stars** | 3,600+ | 5,900+ | 11,700+ | 4,500+ |
| **License** | MIT/Apache-2.0 | BSD 2-Clause | MIT | Apache-2.0 |
| **Last Commit** | May 2025 | Nov 2025 | Sept 2025 | Dec 2025 |
| **Integration** | 4/10 | 5/10 | 6/10 | 4/10 |
| **Axum Native** | ✅ Excellent | ✅ Good | ✅ Routes | ⚠️ Manual |
| **Multi-Language** | ✅ Via FFI | ⚠️ Harder | ✅ Via protoc | ❌ Rust-only |
| **Federation** | ✅ v2 | ❌ None | N/A | N/A |
| **Streaming** | ✅ WebSocket | ✅ WebSocket | ✅ All types | N/A |
| **Tower Compat** | ✅ Perfect | ✅ Good | ✅ Native | ⚠️ Manual |
| **Recommendation** | **ADOPT** | Fallback | **ADOPT** | Skip |

---

## Implementation Roadmap

### Phase 1: GraphQL MVP (Weeks 1-3)
1. Create `crates/spikard-graphql` crate
   - Wrap async-graphql::Schema with builder
   - Implement Handler trait for Executor
   - Value ↔ JSON conversion
2. Integrate with `crates/spikard-http`
   - Route POST /graphql to GraphQL handler
   - WebSocket upgrade at /graphql/ws
3. Python binding prototype (`spikard-py`)
   - Schema class with decorators
   - msgspec ↔ Value adapter

### Phase 2: Multi-Language GraphQL (Weeks 4-7)
1. Node.js binding (`spikard-node`)
2. Ruby binding (`spikard-rb`)
3. PHP binding (`spikard-php`)
4. Fixture expansion (`testing_data/graphql/`)
5. Documentation (ADR, examples, guides)

### Phase 3: gRPC Support (Weeks 8-13)
1. Add `tonic` + `tonic-build` to workspace
2. Create `crates/spikard-grpc` crate
3. Python binding with grpcio integration
4. Node.js binding with @grpc/grpc-js
5. Ruby/PHP bindings
6. Separate port strategy (gRPC on 50051)

### Phase 4: Advanced Features (Weeks 14-18)
1. Apollo Federation support
2. GraphQL subscriptions over HTTP/2
3. gRPC streaming across bindings
4. Performance optimization
5. Comprehensive benchmarks

---

## Key Architectural Decisions

### 1. Middleware Strategy
**Decision:** All middleware stays in `crates/spikard-http` (tower-http)
**Rationale:** Both async-graphql and tonic are middleware-agnostic; Spikard controls the stack

### 2. WebSocket Handling
**Decision:** Use async-graphql's `execute_stream()`, Spikard handles protocols
**Rationale:** Maintains Spikard's lifecycle hook control while using async-graphql's query engine

### 3. JSON Serialization
**Decision:** Create Value ↔ msgspec adapter for Python
**Rationale:** Preserves Spikard's zero-copy story while integrating async-graphql

### 4. gRPC Port Strategy
**Decision:** Separate ports (gRPC: 50051, REST: 8080)
**Rationale:** HTTP/2 ALPN negotiation complexity; cleaner separation

### 5. Multi-Language Code Generation
**Decision:** Use standard protoc plugins for each language
**Rationale:** tonic generates Rust only; leverage existing ecosystem tooling

---

## Dependencies Added

### For GraphQL
```toml
[dependencies]
async-graphql = "7.0"
async-graphql-axum = "7.0"

[features]
graphql = ["dep:async-graphql", "dep:async-graphql-axum"]
```

### For gRPC
```toml
[dependencies]
tonic = { version = "0.14", features = ["transport", "codegen"] }
prost = "0.14"

[build-dependencies]
tonic-build = "0.14"

[features]
grpc = ["dep:tonic", "dep:prost"]
```

---

## Risk Assessment

### GraphQL (async-graphql)
| Risk | Severity | Mitigation |
|------|----------|-----------|
| Serialization mismatch (serde_json vs msgspec) | Medium | Create adapter layer |
| WebSocket protocol duplication | Low | Use execute_stream() only |
| Extension system overlap | Low | Document separation clearly |
| Dependency tree size | Low | Use feature flags |

### gRPC (tonic)
| Risk | Severity | Mitigation |
|------|----------|-----------|
| Multi-language FFI overhead | Medium | Benchmark early, optimize |
| HTTP/2 + HTTP/1.1 complexity | Medium | Separate ports |
| Protobuf not language-neutral | High | Use protoc ecosystem |
| Streaming Pin<Box> ergonomics | Low | Wrapper helpers |

---

## Next Steps

1. **Review this research** with stakeholders
2. **Prototype async-graphql** integration in branch
3. **Benchmark** GraphQL vs REST performance
4. **Document** integration patterns in ADR
5. **Decide** on gRPC adoption timeline
6. **Update** README roadmap with findings

---

## References

### Cloned Repositories (in /tmp)
- `/tmp/async-graphql` - async-graphql/async-graphql
- `/tmp/juniper` - graphql-rust/juniper
- `/tmp/tonic` - hyperium/tonic
- `/tmp/prost` - tokio-rs/prost

### Documentation
- [async-graphql Book](https://async-graphql.github.io/)
- [Juniper Book](https://graphql-rust.github.io/juniper/master/)
- [Tonic Guide](https://github.com/hyperium/tonic/tree/master/examples)
- [Prost Documentation](https://docs.rs/prost/)

### Related ADRs
- `docs/adr/0002-runtime-and-middleware.md` - Tower-HTTP middleware
- `docs/adr/0003-validation-and-fixtures.md` - Fixture-driven testing
- `docs/adr/0005-lifecycle-hooks.md` - Lifecycle hooks design

---

**Report Generated:** 2025-12-27
**Evaluation Depth:** Comprehensive (4 haiku agents, full codebase analysis)
**Confidence Level:** High (production-ready libraries, active maintenance verified)
