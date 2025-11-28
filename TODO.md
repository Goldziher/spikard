# Spikard PHP Bindings - Feature Parity TODO

**Generated:** 2025-11-28
**Status:** PHP bindings are ~65% complete (missing critical features, documentation, and tooling)
**Goal:** Achieve 95%+ parity with Python, Node.js, and Ruby bindings

---

## Executive Summary

The PHP bindings have solid foundation but require 5-9 weeks of focused work to reach production readiness:

| Area | Current Status | Target | Effort |
|------|---------------|--------|--------|
| **Core Features** | 65% (missing background tasks, DI, streaming) | 95% | 3-4 weeks |
| **Documentation** | 20% (no examples, guides, or API docs) | 95% | 2-3 weeks |
| **CLI Codegen** | 65% (no AsyncAPI test apps) | 95% | 1 week |
| **Benchmarks** | 0% (no apps or harness integration) | 100% | 3-4 weeks |
| **CI/CD** | 60% (no publishing, disabled native build) | 95% | 1-2 weeks |
| **Taskfile** | 80% (missing benchmark tasks) | 95% | 3-5 days |

**Total Estimated Effort:** 10-18 weeks (can be parallelized)

---

## Priority 0: Critical Blockers (3-4 weeks)

These features block production use and prevent feature parity.

### 1. Background Tasks Implementation

**Files:**
- `crates/spikard-php/src/php/mod.rs`
- `crates/spikard-php/src/php/background.rs` (new)
- `packages/php/src/Background/BackgroundTask.php` (new)
- `packages/php/src/Config/BackgroundTaskConfig.php` (new)

**Tasks:**
- [ ] Add `spikard_background_run()` FFI function
- [ ] Implement `PhpBackgroundTask` struct in Rust
- [ ] Create PHP `BackgroundTask` class
- [ ] Expose `BackgroundTaskConfig` to PHP
- [ ] Add task spawning from handlers
- [ ] Test with async jobs (email, logging, etc.)
- [ ] Document usage patterns

**Acceptance Criteria:**
```php
use Spikard\Background\BackgroundTask;

$app->addRoute('POST', '/signup', function($request) {
    // Synchronous work
    $user = createUser($request->body);

    // Background work
    BackgroundTask::run(function() use ($user) {
        sendWelcomeEmail($user);
        updateAnalytics($user);
    });

    return Response::json(['user' => $user]);
});
```

**Effort:** 1-2 weeks
**Priority:** P0 - Critical

---

### 2. Dependency Injection System

**Files:**
- `crates/spikard-php/src/php/start.rs` (line 277: remove `di_container: None`)
- `crates/spikard-php/src/php/di.rs` (new)
- `packages/php/src/DI/DependencyContainer.php` (update - currently stub)

**Tasks:**
- [ ] Enable DI in `start.rs` (remove `None` stub)
- [ ] Implement value dependency resolution
- [ ] Implement factory dependency resolution
- [ ] Add singleton caching
- [ ] Handler parameter injection
- [ ] Nested dependency resolution
- [ ] Test with complex dependency trees

**Acceptance Criteria:**
```php
use Spikard\DI\DependencyContainer;

$container = new DependencyContainer();
$container->singleton(Database::class, fn() => new PDO(...));
$container->factory(EmailService::class, fn($db) => new EmailService($db));

$app = new App($config, dependencies: $container);

// Handlers receive dependencies automatically
$app->addRoute('GET', '/users', function(Database $db) {
    return $db->query('SELECT * FROM users');
});
```

**Effort:** 1-2 weeks
**Priority:** P0 - Critical

---

### 3. Streaming Response Support

**Files:**
- `packages/php/src/Http/StreamingResponse.php` (new)
- `crates/spikard-php/src/php/handler.rs` (update to handle streaming)

**Tasks:**
- [ ] Create `StreamingResponse` class
- [ ] Support PHP Generator as stream source
- [ ] Implement chunked encoding
- [ ] Handle backpressure
- [ ] Test with large file streaming
- [ ] Test with real-time data streams

**Acceptance Criteria:**
```php
use Spikard\Http\StreamingResponse;

$app->addRoute('GET', '/large-file', function() {
    return new StreamingResponse(function() {
        $file = fopen('large-file.csv', 'r');
        while ($line = fgets($file)) {
            yield $line;
        }
        fclose($file);
    });
});
```

**Effort:** 1 week
**Priority:** P0 - Critical

---

## Priority 1: High-Impact Features (2-3 weeks)

These features significantly improve developer experience.

### 4. Parameter Extraction Helpers

**Files:**
- `packages/php/src/Http/Query.php` (new)
- `packages/php/src/Http/Path.php` (new)
- `packages/php/src/Http/Header.php` (new)
- `packages/php/src/Http/Cookie.php` (new)
- `packages/php/src/Http/Body.php` (new)

**Tasks:**
- [ ] Create `Query`, `Path`, `Header`, `Cookie`, `Body` helper classes
- [ ] Integrate with handler signature parsing (via reflection)
- [ ] Generate parameter schemas from PHP type hints
- [ ] Validate extracted parameters
- [ ] Test with complex parameter combinations

**Acceptance Criteria:**
```php
use Spikard\Http\{Query, Path, Header, Body};

$app->addRoute('GET', '/users/{id}',
    function(int $id = Path(), ?string $search = Query(), string $auth = Header('Authorization')) {
        // Parameters auto-extracted and validated
        return User::find($id, $search);
    }
);
```

**Effort:** 1-2 weeks
**Priority:** P1 - High

---

### 5. Route Attributes (Decorator Pattern)

**Files:**
- `packages/php/src/Attributes/Route.php` (new)
- `packages/php/src/Attributes/Get.php` (new)
- `packages/php/src/Attributes/Post.php` (new)
- `packages/php/src/App.php` (update to scan attributes)

**Tasks:**
- [ ] Create PHP 8.0+ Attribute classes
- [ ] Scan controller classes for route attributes
- [ ] Generate routes from attributes at runtime
- [ ] Support middleware attributes
- [ ] Test attribute-based routing

**Acceptance Criteria:**
```php
use Spikard\Attributes\{Get, Post};

class UserController
{
    #[Get('/users')]
    public function listUsers(): array {
        return User::all();
    }

    #[Post('/users')]
    public function createUser(#[Body] CreateUserRequest $request): User {
        return User::create($request);
    }
}

$app->registerController(UserController::class);
```

**Effort:** 1-2 weeks
**Priority:** P1 - High

---

## Priority 2: Documentation (2-3 weeks)

### 6. Code Examples & Snippets

**Files:**
- `examples/php/` (new directory)
- `docs/snippets/php/` (new directory)

**Tasks:**
- [ ] Create 16 snippet files (match Python/Node/Ruby)
  - [ ] `01-hello-world.php`
  - [ ] `02-json-request-response.php`
  - [ ] `03-query-parameters.php`
  - [ ] `04-path-parameters.php`
  - [ ] `05-headers.php`
  - [ ] `06-cookies.php`
  - [ ] `07-api-key-auth.php`
  - [ ] `08-jwt-auth.php`
  - [ ] `09-cors.php`
  - [ ] `10-compression.php`
  - [ ] `11-rate-limiting.php`
  - [ ] `12-lifecycle-hooks.php`
  - [ ] `13-websocket.php`
  - [ ] `14-sse.php`
  - [ ] `15-background-tasks.php`
  - [ ] `16-testing.php`

- [ ] Create 4 example applications
  - [ ] `examples/php/hello-world/` - Basic server
  - [ ] `examples/php/todo-api/` - REST API with auth
  - [ ] `examples/php/chat-websocket/` - WebSocket chat
  - [ ] `examples/php/notifications-sse/` - SSE notifications

**Effort:** 1 week
**Priority:** P2 - High

---

### 7. User Documentation

**Files:**
- `docs/bindings/php.md` (new)
- `packages/php/README.md` (expand)
- `docs/guides/php-getting-started.md` (new)

**Tasks:**
- [ ] Create comprehensive binding guide (`docs/bindings/php.md`)
  - Architecture overview
  - Installation (Composer, extension build)
  - Configuration reference
  - Handler patterns
  - Middleware usage
  - Testing guide

- [ ] Expand `packages/php/README.md`
  - Match Python README depth (~300 lines)
  - Installation instructions
  - Quick start example
  - Feature list
  - API reference links

- [ ] Create getting started guide
  - Prerequisites
  - Installation steps
  - First app tutorial
  - Deployment guide

- [ ] Update root `README.md`
  - Promote PHP from "Future" to "Current Bindings"
  - Add PHP installation snippet
  - Add PHP to feature matrix

**Effort:** 1-2 weeks
**Priority:** P2 - High

---

### 8. ADR Updates & API Reference

**Files:**
- `docs/adr/0001-architecture-overview.md`
- `docs/adr/0002-runtime-and-middleware.md`
- `docs/adr/0003-validation-and-fixtures.md`
- `docs/adr/0005-lifecycle-hooks.md`
- `docs/adr/0006-async-streaming.md`
- `docs/adr/0007-php-ffi-patterns.md` (new)
- `phpDocumentor.xml` (new)

**Tasks:**
- [ ] Add PHP examples to all 5 existing ADRs
- [ ] Create new ADR for PHP FFI patterns (ext-php-rs)
- [ ] Configure phpDocumentor for API docs generation
- [ ] Generate API reference documentation
- [ ] Host documentation (GitHub Pages or docs.rs equivalent)

**Effort:** 3-5 days
**Priority:** P2 - Medium

---

## Priority 3: CLI & Codegen (1 week)

### 9. AsyncAPI Test App Generation

**Files:**
- `crates/spikard-cli/src/codegen/asyncapi/mod.rs`
- `crates/spikard-cli/src/codegen/asyncapi/generators/php.rs`
- `crates/spikard-cli/src/codegen/engine.rs`

**Tasks:**
- [ ] Implement `generate_php_test_app()` function
- [ ] Generate PHP WebSocket/SSE test clients
- [ ] Add PHP to AsyncAPI bundle generation
- [ ] Update `features` command to mention PHP
- [ ] Add CLI tests for PHP AsyncAPI test apps

**Acceptance Criteria:**
```bash
# Should work (currently fails)
spikard testing asyncapi test-app schema.yaml --lang php --output app.php

# Should include PHP (currently omitted)
spikard testing asyncapi all schema.yaml --output ./generated
```

**Effort:** 3-5 days
**Priority:** P3 - Medium

---

### 10. AsyncAPI Message Schema DTOs

**Files:**
- `crates/spikard-cli/src/codegen/asyncapi/generators/php.rs`

**Tasks:**
- [ ] Generate readonly class DTOs for AsyncAPI messages
- [ ] Add JSON serialization helpers
- [ ] Mirror Python's msgspec.Struct pattern
- [ ] Test with complex message schemas

**Acceptance Criteria:**
```php
// Generated from AsyncAPI schema
readonly class ChatMessage
{
    public function __construct(
        public string $type,
        public string $body,
        public string $userId
    ) {}

    public static function fromJson(string $json): self {
        $data = json_decode($json, true);
        return new self($data['type'], $data['body'], $data['userId']);
    }
}
```

**Effort:** 1 week
**Priority:** P3 - Medium

---

## Priority 4: Benchmarking (3-4 weeks)

### 11. App Generator

**Files:**
- `tools/app-generator/src/generators/spikard_php.rs` (new)
- `Taskfile.yaml` (add PHP app generator task)

**Tasks:**
- [ ] Create PHP app generator (mirror Python generator)
- [ ] Generate routes from fixture metadata
- [ ] Extract ServerConfig from fixtures
- [ ] Generate handlers with expected responses
- [ ] Test with all fixture categories
- [ ] Add `task php:generate:app` to Taskfile

**Acceptance Criteria:**
```bash
task php:generate:app FIXTURE_DIR=testing_data OUTPUT=generated-php-app
```

**Effort:** 1-2 weeks
**Priority:** P4 - High

---

### 12. Benchmark Apps

**Files:**
- `tools/benchmark-harness/apps/spikard-php/` (new)
- `tools/benchmark-harness/apps/phalcon/` (new)

**Tasks:**
- [ ] Create Spikard PHP benchmark app
  - [ ] `GET /` - Hello World
  - [ ] `GET /json` - JSON response
  - [ ] `POST /echo` - Echo request body
  - [ ] `GET /query?foo=bar` - Query params
  - [ ] Production configuration (opcache, JIT)

- [ ] Create Phalcon reference app
  - [ ] Same endpoints as Spikard PHP
  - [ ] Fair comparison setup
  - [ ] Production configuration

- [ ] Document setup instructions
- [ ] Test both apps run correctly

**Effort:** 1-2 weeks
**Priority:** P4 - High

---

### 13. Benchmark Harness Integration

**Files:**
- `tools/benchmark-harness/src/main.rs` (or equivalent)
- `tools/benchmark-harness/Taskfile.yaml`
- `Taskfile.yaml` (root)

**Tasks:**
- [ ] Add PHP app detection to benchmark harness
- [ ] Configure PHP-FPM or CLI runner
- [ ] Ensure opcache and JIT enabled
- [ ] Test harness runs PHP apps correctly

- [ ] Add Taskfile tasks:
  - [ ] `task bench:install:spikard-php`
  - [ ] `task bench:install:phalcon`
  - [ ] `task bench:run:spikard-php`
  - [ ] `task bench:run:phalcon`
  - [ ] `task bench:compare:php`

**Effort:** 1 week
**Priority:** P4 - High

---

### 14. Benchmark CI Workflow

**Files:**
- `.github/workflows/benchmarks.yml` (update)

**Tasks:**
- [ ] Add PHP to benchmark matrix
- [ ] Install PHP, Composer, extensions
- [ ] Run PHP benchmarks in CI
- [ ] Generate comparison reports
- [ ] Publish benchmark results

**Effort:** 3-5 days
**Priority:** P4 - Medium

---

## Priority 5: CI/CD & Publishing (1-2 weeks)

### 15. Native Extension Build

**Files:**
- `Taskfile.yaml`
- `.cargo/config.toml`
- `crates/spikard-php/build.rs`

**Tasks:**
- [ ] Fix ext-php-rs ARM bindgen issues
  - Current blocker: NEON intrinsics on ARM
  - Investigate: `--features force-bindgen`
  - Alternative: Pre-generate bindings for ARM

- [ ] Enable `task php:build` in Taskfile
- [ ] Test extension build on macOS ARM64
- [ ] Test extension build on Linux x86_64
- [ ] Test extension build on Linux ARM64
- [ ] Document build requirements

**Effort:** 1 week
**Priority:** P5 - High

---

### 16. Publishing Workflow

**Files:**
- `.github/workflows/publish.yml` (update)
- `packages/php/composer.json`

**Tasks:**
- [ ] Add PHP to release workflow
- [ ] Publish Composer package to Packagist
- [ ] Build extension binaries for multiple platforms
- [ ] Publish binaries to GitHub Releases
- [ ] Update Composer package version on release
- [ ] Generate changelog for PHP package

**Acceptance Criteria:**
```bash
# Users can install via Composer
composer require spikard/spikard

# Package available on Packagist
https://packagist.org/packages/spikard/spikard
```

**Effort:** 1 week
**Priority:** P5 - High

---

### 17. CI/CD Enhancements

**Files:**
- `.github/workflows/ci.yml`

**Tasks:**
- [ ] Test PHP 8.2, 8.3, 8.4 in CI
- [ ] Test on Ubuntu, macOS, Windows (if supported)
- [ ] Add PHPUnit test coverage reporting
- [ ] Add PHPStan to CI (currently passing locally)
- [ ] Add Psalm static analysis (optional)

**Effort:** 3-5 days
**Priority:** P5 - Medium

---

## Priority 6: Enhanced Features (3-4 weeks)

### 18. Async Integration (ReactPHP/Amp/Swoole)

**Files:**
- `packages/php/src/Runtime/AsyncAdapter.php` (new)

**Tasks:**
- [ ] Research ReactPHP integration
- [ ] Research Amp integration
- [ ] Research Swoole integration
- [ ] Implement async adapter layer
- [ ] Test non-blocking I/O
- [ ] Document async usage

**Note:** PHP lacks native async/await. This would be adapter-based.

**Effort:** 2-3 weeks
**Priority:** P6 - Low (nice to have)

---

### 19. Enhanced Error Messages

**Files:**
- `crates/spikard-php/src/php/handler.rs`
- `packages/php/src/Exceptions/` (new)

**Tasks:**
- [ ] Match Python's error detail level
- [ ] Include stack traces in errors
- [ ] Better validation error messages
- [ ] Custom exception classes
- [ ] Test error handling edge cases

**Effort:** 1 week
**Priority:** P6 - Low

---

### 20. Testing Expansion

**Files:**
- `e2e/php/tests/`
- `packages/php/tests/`

**Tasks:**
- [ ] Add property-based testing (Pest framework)
- [ ] Expand E2E test scenarios
- [ ] Performance benchmarks
- [ ] Integration tests for all middleware
- [ ] Edge case testing

**Effort:** 1-2 weeks
**Priority:** P6 - Low

---

## Effort Summary

| Priority | Description | Tasks | Effort | Can Parallelize? |
|----------|-------------|-------|--------|------------------|
| P0 | Critical blockers | 3 | 3-4 weeks | Yes (different devs) |
| P1 | High-impact features | 2 | 2-3 weeks | Yes |
| P2 | Documentation | 3 | 2-3 weeks | Yes |
| P3 | CLI & Codegen | 2 | 1 week | Yes |
| P4 | Benchmarking | 4 | 3-4 weeks | Yes (separate from P0-P3) |
| P5 | CI/CD & Publishing | 3 | 1-2 weeks | Depends on P0 completion |
| P6 | Enhanced features | 3 | 3-4 weeks | Yes (optional) |

**Total Sequential:** 15-21 weeks
**Total Parallelized:** 5-9 weeks (with 3-4 developers working concurrently)

---

## Recommended Phasing

### Phase 1 (Weeks 1-3): Core Features
- P0: Background tasks, DI, streaming responses
- **Deliverable:** Feature-complete bindings

### Phase 2 (Weeks 2-4): Documentation & CLI
- P1: Parameter extraction, route attributes
- P2: Examples, guides, API docs
- P3: AsyncAPI test apps, message DTOs
- **Deliverable:** Production-ready with full docs

### Phase 3 (Weeks 3-7): Benchmarking & Publishing
- P4: App generator, benchmark apps, harness integration
- P5: Native extension build, CI/CD, publishing
- **Deliverable:** Published package with performance metrics

### Phase 4 (Weeks 5-9): Polish & Enhancements
- P6: Async integration, enhanced errors, expanded tests
- **Deliverable:** Best-in-class PHP framework experience

---

## Success Metrics

### Feature Parity
- [ ] 95%+ implementation parity with Python bindings
- [ ] All core features functional
- [ ] Background tasks working
- [ ] DI system complete
- [ ] Streaming responses supported

### Documentation
- [ ] 16 code snippets (match Python)
- [ ] 4 example applications
- [ ] Comprehensive binding guide
- [ ] API reference generated
- [ ] PHP mentioned in all ADRs

### Tooling
- [ ] CLI codegen fully supports PHP
- [ ] App generator produces working apps
- [ ] Benchmark harness runs PHP apps
- [ ] CI/CD covers all PHP workflows

### Performance
- [ ] Benchmarks published comparing Spikard PHP vs Phalcon
- [ ] Performance within 10-20% of pure PHP
- [ ] JIT and opcache optimizations documented

### Publishing
- [ ] Package on Packagist
- [ ] Binaries on GitHub Releases
- [ ] Automated publishing workflow
- [ ] Versioning synchronized with core

---

## Dependencies & Blockers

### External Dependencies
- ext-php-rs ARM bindgen fix (P5 blocker)
- Phalcon framework setup knowledge (P4)
- ReactPHP/Amp/Swoole integration research (P6)

### Internal Dependencies
- Background tasks needed for P2 examples
- DI needed for P1 parameter extraction
- Native extension needed for full benchmarks

### Resource Requirements
- 3-4 developers for parallel work
- macOS ARM64 machine for ext-php-rs debugging
- CI/CD credits for PHP workflows

---

## Tracking

**Created:** 2025-11-28
**Last Updated:** 2025-11-28
**Owner:** TBD
**Milestone:** PHP Bindings GA (General Availability)
**Target:** Q2 2025

---

## Notes

- Current PHP bindings are functional but incomplete
- Test suite is passing (441/441 tests) but limited in scope
- Core infrastructure is solid, mainly missing ergonomic features
- Documentation gap is the biggest barrier to adoption
- Benchmarking will demonstrate performance vs Phalcon
