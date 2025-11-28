# Schema & Examples Implementation Checklist

**Project:** Spikard Examples Suite
**Date:** November 28, 2025
**Status:** Design Complete ✓

---

## Design Phase - COMPLETED

### Schema Design

- [x] **Todo API** (`todo-api.openapi.yaml`)
  - [x] 6 REST endpoints (CRUD + health)
  - [x] Full request/response validation
  - [x] Pagination, filtering, sorting
  - [x] Multiple status codes (8 variants)
  - [x] Error responses (RFC 9457)
  - [x] Authentication (Bearer token)
  - [x] Rate limiting headers
  - [x] Lifecycle hooks integration
  - [x] Comprehensive examples (10+)
  - [x] YAML validation ✓

- [x] **File Service** (`file-service.openapi.yaml`)
  - [x] 7 endpoints (upload, list, download, delete, info, verify, quota)
  - [x] Multipart form-data upload
  - [x] Binary streaming response
  - [x] File validation (MIME type, size)
  - [x] Conditional requests (ETag, If-Modified-Since)
  - [x] Range requests (206 Partial Content)
  - [x] Hash verification (SHA-256)
  - [x] Status codes (11 variants)
  - [x] Comprehensive examples (8+)
  - [x] YAML validation ✓

- [x] **Chat Service** (`chat-service.asyncapi.yaml`)
  - [x] WebSocket protocol (wss/ws)
  - [x] 10 message types defined
  - [x] Bidirectional messaging
  - [x] User presence tracking
  - [x] Message acknowledgments
  - [x] Typing indicators
  - [x] Connection state
  - [x] Error handling
  - [x] Comprehensive examples (12+)
  - [x] YAML validation ✓

- [x] **Events Stream** (`events-stream.asyncapi.yaml`)
  - [x] Server-Sent Events (SSE) protocol
  - [x] 6 event types defined
  - [x] Event filtering (type, severity, user)
  - [x] Heartbeat messages
  - [x] Reconnection with catch-up
  - [x] Batch operations
  - [x] Query parameter validation
  - [x] Error handling
  - [x] Comprehensive examples (10+)
  - [x] YAML validation ✓

- [x] **Auth Service** (`auth-service.openapi.yaml`)
  - [x] 9 endpoints (keys, tokens, OAuth)
  - [x] API key management
  - [x] JWT token management
  - [x] OAuth 2.0 flow
  - [x] Token refresh
  - [x] Scope-based authorization
  - [x] Multiple grant types
  - [x] Token verification
  - [x] Comprehensive examples (10+)
  - [x] YAML validation ✓

### Documentation

- [x] **Schema Suite README** (`schemas/README.md`)
  - [x] Overview of all 5 schemas
  - [x] Feature checklist
  - [x] Design patterns documented
  - [x] Generation commands
  - [x] Schema features matrix
  - [x] References to standards

- [x] **Examples Structure Guide** (`STRUCTURE.md`)
  - [x] Directory layout (all 5 languages)
  - [x] Language-specific stacks
  - [x] Per-language patterns
  - [x] File organization
  - [x] Running examples
  - [x] Testing strategy
  - [x] Features by example matrix

- [x] **Main Examples README** (`README.md`)
  - [x] Quick start guide
  - [x] Getting started (5 languages)
  - [x] Schema overviews
  - [x] Feature highlights
  - [x] Testing guide
  - [x] Development workflow
  - [x] Common tasks

- [x] **Design Summary** (`SCHEMA_DESIGN_SUMMARY.md`)
  - [x] Executive summary
  - [x] All schemas described
  - [x] Spikard features coverage
  - [x] Quality checklist
  - [x] Next steps
  - [x] Statistics

- [x] **This Checklist** (`IMPLEMENTATION_CHECKLIST.md`)
  - [x] Design completeness
  - [x] Code generation tasks
  - [x] Implementation tasks
  - [x] Testing tasks
  - [x] Deployment tasks

---

## Phase 1: Code Generation - TODO

### OpenAPI Code Generation

#### Python

- [ ] Generate Flask server stub
  ```bash
  openapi-generator-cli generate \
    -i examples/schemas/todo-api.openapi.yaml \
    -g python-flask \
    -o python/todo-api/generated/
  ```

- [ ] Generate msgspec models
  - [ ] Request models
  - [ ] Response models
  - [ ] Error models

- [ ] Generate for each service:
  - [ ] todo-api
  - [ ] file-service
  - [ ] auth-service

#### Node.js/TypeScript

- [ ] Generate TypeScript types
  ```bash
  openapi-generator-cli generate \
    -i examples/schemas/todo-api.openapi.yaml \
    -g typescript-axios \
    -o node/todo-api/generated/
  ```

- [ ] Generate for each service:
  - [ ] todo-api
  - [ ] file-service
  - [ ] auth-service

#### Ruby

- [ ] Generate RBS type definitions
- [ ] Generate handler stubs
- [ ] Generate for each service:
  - [ ] todo-api
  - [ ] file-service
  - [ ] auth-service

#### PHP

- [ ] Generate PSR-compliant classes
- [ ] Generate models
- [ ] Generate for each service:
  - [ ] todo-api
  - [ ] file-service
  - [ ] auth-service

### AsyncAPI Code Generation

#### Python

- [ ] Generate message schemas
  ```bash
  asyncapi generate fromTemplate \
    examples/schemas/chat-service.asyncapi.yaml \
    @asyncapi/python-pydantic-schema \
    -o python/chat-service/generated/
  ```

- [ ] Generate for:
  - [ ] chat-service
  - [ ] events-stream

#### Node.js/TypeScript

- [ ] Generate TypeScript interfaces
- [ ] Generate for:
  - [ ] chat-service
  - [ ] events-stream

#### Ruby/PHP/WASM

- [ ] Generate language-specific types

---

## Phase 2: Implementation - TODO

### Python Implementation

- [ ] **todo-api**
  - [ ] Create `python/todo-api/` directory structure
  - [ ] Implement handlers in `src/handlers.py`
  - [ ] Create models in `src/models.py`
  - [ ] Implement lifecycle hooks in `src/lifecycle.py`
  - [ ] Write tests in `tests/test_handlers.py`
  - [ ] Create `pyproject.toml`
  - [ ] Create `README.md`

- [ ] **file-service**
  - [ ] Create `python/file-service/` directory
  - [ ] Implement upload handler (multipart)
  - [ ] Implement download handler (streaming)
  - [ ] Create storage layer in `src/storage.py`
  - [ ] Implement validation in `src/validation.py`
  - [ ] Write tests

- [ ] **chat-service**
  - [ ] Create `python/chat-service/` directory
  - [ ] Implement WebSocket handler
  - [ ] Create room management in `src/rooms.py`
  - [ ] Implement message handlers
  - [ ] Write tests

- [ ] **events-stream**
  - [ ] Create `python/events-stream/` directory
  - [ ] Implement SSE handler
  - [ ] Create event queue in `src/event_queue.py`
  - [ ] Implement subscriber management
  - [ ] Write tests

- [ ] **auth-service**
  - [ ] Create `python/auth-service/` directory
  - [ ] Implement token management in `src/tokens.py`
  - [ ] Implement API key management in `src/keys.py`
  - [ ] Implement OAuth flow in `src/oauth.py`
  - [ ] Write tests

### Node.js/TypeScript Implementation

- [ ] **todo-api**
  - [ ] Create `node/todo-api/` directory
  - [ ] Setup TypeScript configuration
  - [ ] Implement handlers in `src/handlers.ts`
  - [ ] Create types in `src/models.ts`
  - [ ] Implement lifecycle hooks
  - [ ] Write tests with Vitest
  - [ ] Create `package.json`
  - [ ] Create `tsconfig.json`
  - [ ] Create `README.md`

- [ ] **file-service**
  - [ ] Create `node/file-service/` directory
  - [ ] Implement upload/download handlers
  - [ ] Create storage layer
  - [ ] Write tests

- [ ] **chat-service**
  - [ ] Create `node/chat-service/` directory
  - [ ] Implement WebSocket handlers
  - [ ] Create room management
  - [ ] Write tests

- [ ] **events-stream**
  - [ ] Create `node/events-stream/` directory
  - [ ] Implement SSE handlers
  - [ ] Create event queue
  - [ ] Write tests

- [ ] **auth-service**
  - [ ] Create `node/auth-service/` directory
  - [ ] Implement token/key handlers
  - [ ] Implement OAuth flow
  - [ ] Write tests

### Ruby Implementation

- [ ] Create all 5 services under `ruby/`
- [ ] For each service:
  - [ ] Create RBS type definitions in `sig/`
  - [ ] Implement handlers in `lib/`
  - [ ] Write RSpec tests
  - [ ] Create `Gemfile`
  - [ ] Create `.ruby-version`
  - [ ] Create `README.md`

### PHP Implementation

- [ ] Create all 5 services under `php/`
- [ ] For each service:
  - [ ] Create PSR-4 autoloaded classes
  - [ ] Implement handlers
  - [ ] Write PHPUnit tests
  - [ ] Create `composer.json`
  - [ ] Create `.php-version`
  - [ ] Create `README.md`

### WebAssembly Implementation

- [ ] Create all services under `wasm/`
- [ ] For each service:
  - [ ] Implement handlers in Rust
  - [ ] Create wasm-bindgen FFI
  - [ ] Write integration tests
  - [ ] Create `Cargo.toml`
  - [ ] Create `wasm-pack.toml`
  - [ ] Create `README.md`

---

## Phase 3: Testing - TODO

### Fixture Preparation

- [ ] Create fixtures in `testing_data/`:
  - [ ] Headers fixtures
  - [ ] Cookies fixtures
  - [ ] JSON bodies fixtures
  - [ ] Validation errors
  - [ ] Status codes
  - [ ] Rate limiting scenarios

### Test Implementation

#### Python

- [ ] todos tests
  - [ ] CRUD operations
  - [ ] Pagination
  - [ ] Filtering
  - [ ] Sorting
  - [ ] Error cases

- [ ] Files tests
  - [ ] Upload
  - [ ] Download
  - [ ] Validation

- [ ] Chat tests
  - [ ] Message send/receive
  - [ ] Presence tracking
  - [ ] Connection state

- [ ] Events tests
  - [ ] Event streaming
  - [ ] Filtering
  - [ ] Reconnection

- [ ] Auth tests
  - [ ] Token management
  - [ ] API keys
  - [ ] OAuth flow

#### Node.js/TypeScript

- [ ] Same test suites as Python

#### Ruby, PHP, WASM

- [ ] Same test suites as Python

### Fixture-Driven Testing

- [ ] `task test:fixtures` - Validate all examples against fixtures
- [ ] `task test:python` - Run Python test suite
- [ ] `task test:node` - Run Node test suite
- [ ] `task test:ruby` - Run Ruby test suite
- [ ] `task test:php` - Run PHP test suite
- [ ] `task test:wasm` - Run WASM test suite

### Cross-Language Validation

- [ ] Ensure all languages produce identical responses
- [ ] Validate against fixture schemas
- [ ] Test error handling consistency

---

## Phase 4: Documentation - TODO

### API Documentation

- [ ] Generate Swagger UI docs
  ```bash
  docker run -p 8080:80 \
    -e SWAGGER_JSON=/schemas/todo-api.openapi.yaml \
    -v $(pwd)/schemas:/schemas \
    swaggerapi/swagger-ui
  ```

- [ ] Generate ReDoc docs
  ```bash
  docker run -p 8080:80 \
    -e SPEC_URL=/schemas/todo-api.openapi.yaml \
    -v $(pwd)/schemas:/schemas \
    redocly/redoc
  ```

### Example Documentation

- [ ] Update each example `README.md` with:
  - [ ] Quick start
  - [ ] Features demonstrated
  - [ ] Running the example
  - [ ] Testing
  - [ ] API reference
  - [ ] Code examples

### Spikard Documentation

- [ ] Create example links in main docs
- [ ] Add schema references to ADRs
- [ ] Document code generation workflow
- [ ] Create contributing guide

---

## Phase 5: Deployment & CI/CD - TODO

### CI/CD Setup

- [ ] **GitHub Actions Workflows**
  - [ ] Python tests and linting
  - [ ] Node tests and linting
  - [ ] Ruby tests and linting
  - [ ] PHP tests and linting
  - [ ] WASM tests and bundling
  - [ ] Schema validation

- [ ] **Local Development**
  - [ ] `task setup` - Install all dependencies
  - [ ] `task build` - Build all bindings
  - [ ] `task lint` - Lint all code
  - [ ] `task format` - Format all code
  - [ ] `task test` - Test all languages

### Docker Support

- [ ] Create Dockerfile for Python example
- [ ] Create Dockerfile for Node example
- [ ] Create docker-compose.yml for multi-service demo

### Package Publishing

- [ ] **Python:** Publish to PyPI (if applicable)
- [ ] **Node:** Publish to npm (if applicable)
- [ ] **Ruby:** Publish to RubyGems (if applicable)
- [ ] **PHP:** Publish to Packagist (if applicable)
- [ ] **WASM:** Publish to npm registry

---

## Quality Assurance - TODO

### Code Quality

- [ ] **Python**
  - [ ] mypy --strict validation
  - [ ] pytest coverage (95%+)
  - [ ] Black formatting

- [ ] **Node.js/TypeScript**
  - [ ] TypeScript strict compilation
  - [ ] Biome linting
  - [ ] Vitest coverage (80%+)

- [ ] **Ruby**
  - [ ] Steep type checking
  - [ ] RSpec coverage (80%+)
  - [ ] Rubocop linting

- [ ] **PHP**
  - [ ] PHPStan level max
  - [ ] PHPUnit coverage (80%+)

- [ ] **WASM**
  - [ ] Rust clippy checks
  - [ ] Minimal binary size
  - [ ] Browser/Node compatibility

### Security

- [ ] [ ] Validate authentication implementation
- [ ] [ ] Check authorization controls
- [ ] [ ] Verify error handling (no info leaks)
- [ ] [ ] Test rate limiting
- [ ] [ ] Check input validation
- [ ] [ ] Verify HTTPS requirements

### Performance

- [ ] [ ] Benchmark each example
- [ ] [ ] Measure response times
- [ ] [ ] Check memory usage
- [ ] [ ] Validate streaming efficiency
- [ ] [ ] Test concurrent connections

---

## Deliverables Checklist

### Phase 1: Complete ✓

- [x] 5 comprehensive schemas (4,256 lines)
- [x] Todo API OpenAPI schema
- [x] File Service OpenAPI schema
- [x] Chat Service AsyncAPI schema
- [x] Events Stream AsyncAPI schema
- [x] Auth Service OpenAPI schema

### Phase 2: Complete ✓

- [x] Comprehensive documentation (2,000+ lines)
- [x] Schema Suite Guide (schemas/README.md)
- [x] Examples Structure Guide (STRUCTURE.md)
- [x] Main Examples README (README.md)
- [x] Design Summary (SCHEMA_DESIGN_SUMMARY.md)
- [x] Implementation Checklist (this file)

### Phase 3-5: Pending

- [ ] Code generation (5 schemas × 5 languages = 25 generated codebases)
- [ ] Multi-language implementations (25 complete services)
- [ ] Comprehensive test suites (125+ test files)
- [ ] CI/CD workflows
- [ ] API documentation (Swagger UI, ReDoc)
- [ ] Example documentation
- [ ] Docker support

---

## Success Criteria

### Design Phase ✓

- [x] All 5 schemas designed and validated
- [x] Production-quality specifications
- [x] All Spikard features demonstrated
- [x] Comprehensive documentation
- [x] Ready for code generation

### Implementation Phase

- [ ] All 25 implementations complete (5 services × 5 languages)
- [ ] All implementations tested
- [ ] Code quality standards met
- [ ] Security validated
- [ ] Documentation complete

### Release Phase

- [ ] CI/CD configured and passing
- [ ] Docker images built and tested
- [ ] Packages published
- [ ] Documentation deployed
- [ ] Examples available for download

---

## Estimated Effort

### Code Generation
- 5-10 hours (using automation tools)

### Implementation per Language
- **Python:** 40-50 hours (async, msgspec, fixture-driven)
- **Node.js:** 40-50 hours (TypeScript strict, Vitest)
- **Ruby:** 30-40 hours (RBS, Steep, RSpec)
- **PHP:** 30-40 hours (PSR, PHPStan, PHPUnit)
- **WASM:** 20-30 hours (Rust, minimal binary size)

**Total Implementation:** ~170-210 hours

### Testing
- 30-40 hours (fixture preparation, cross-validation)

### Documentation & CI/CD
- 20-30 hours (API docs, deployment setup)

**Total Project:** ~225-280 hours

---

## Dependencies & Tools

### Code Generation
- openapi-generator-cli
- asyncapi CLI
- Swagger/OpenAPI validation tools

### Python
- maturin (PyO3 bindings)
- msgspec
- pytest
- mypy
- uv

### Node.js
- napi-rs
- TypeScript
- Vitest
- Biome
- pnpm

### Ruby
- magnus (Rust FFI)
- RBS
- Steep
- RSpec

### PHP
- ext-php-rs
- Composer
- PHPStan
- PHPUnit

### WASM
- wasm-bindgen
- wasm-pack

---

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| Schema completeness | ✓ All features covered (tested) |
| Code generation accuracy | Use mature generators, manual review |
| Cross-language consistency | Fixture-driven tests, CI/CD validation |
| Performance issues | Benchmarking, profiling, optimization |
| Security vulnerabilities | Security review, static analysis, testing |
| Documentation gaps | Comprehensive README per example |

---

## Sign-Off

**Design Phase Status:** ✅ **COMPLETE**

**Next Phase:** Code Generation & Multi-Language Implementation

**Ready to proceed with:** openapi-generator, asyncapi generate, and language-specific tooling

---

## References

- OpenAPI 3.1.0: https://spec.openapis.org/oas/v3.1.0
- AsyncAPI 3.0.0: https://www.asyncapi.com/en/docs/specifications/latest
- RFC 9457: https://tools.ietf.org/html/rfc9457
- Tower-HTTP: https://github.com/tower-rs/tower-http
- Spikard GitHub: https://github.com/spikard/spikard

---

**Last Updated:** November 28, 2025
**Status:** Ready for Implementation Phase
