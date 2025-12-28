# PHP Test Organization Analysis

## Current State:
- 29 test files (helper files not counted as tests)
- 570 total test methods across suite
- Well-distributed coverage across major domains
- Total LOC in tests: 10,803 lines
- Largest tests: ControllerMethodHandlerTest (1604 LOC), AppTest (1209 LOC), StreamingResponseTest (871 LOC)

### What's Well-Covered:

1. **App & Routing (AppTest, AppBehavioralTest, AppRegisterControllerTest)**
   - App immutability and builder pattern
   - Route registration and matching
   - Handler discovery via reflection attributes
   - Native route conversion
   - Config and hooks serialization
   - 200+ test methods focusing on core app functionality

2. **Parameter Resolution (ControllerMethodHandlerTest)**
   - Query, path, header, cookie, body parameter extraction
   - Multiple parameter types and implicit resolution
   - Type coercion (string→int, string→float)
   - Response conversion (array, string, scalar, object, null)
   - 80+ test cases covering all param sources

3. **Configuration (ServerConfigBuilderTest, ConfigTest)**
   - Compression, CORS, RateLimit, JWT, API Key auth
   - Static files, OpenAPI/Swagger configuration
   - All middleware config structures
   - Builder pattern and immutability

4. **Request/Response (RequestResponseTest)**
   - Request construction with all parameter types
   - Response creation and helpers (json, text)
   - Status codes, headers, cookies
   - Data type preservation

5. **Lifecycle Hooks (LifecycleHooksTest)**
   - Hook registration and execution
   - HookResult (continue/shortCircuit)
   - Callback preservation
   - Builder and direct constructor

6. **Cookie/Header Parameters (CookieHeaderTest)**
   - Default values and factories
   - Length constraints, patterns, schemas
   - Aliases and underscore conversion
   - 80+ test methods

7. **GraphQL Client (GraphQLTestClientTest)**
   - GraphQL query/mutation execution
   - Variable handling
   - Error extraction
   - Multiple status handling

### Coverage Gaps Identified:

#### 1. **ext-php-rs Boundary Testing** (HIGH PRIORITY)
- No explicit tests for FFI boundary crossing (Rust↔PHP)
- No tests for null pointer handling across FFI
- Missing: opaque type validation, memory safety edge cases
- Missing: exception translation from Rust panics to PHP exceptions
- No tests for large data serialization across boundary (>2MB)
- No tests for type mismatch error conditions

#### 2. **PSR-7 Interface Compliance** (MEDIUM-HIGH PRIORITY)
- Request class NOT implementing PSR-7 RequestInterface
- Response class NOT implementing PSR-7 ResponseInterface
- No tests for PSR-7 stream handling
- No tests for PSR-7 URI handling
- Missing PSR-7 compliance for headers (case-insensitivity)
- Missing ServerRequest interface implementation
- No tests comparing against PSR-7 spec

#### 3. **Error Translation & Type Safety** (CRITICAL)
- No dedicated error translation tests (Rust errors → PHP exceptions)
- Missing: validation error payload structure tests
- Missing: error code mapping across FFI
- No tests for stacktrace preservation
- Missing: circular reference detection in type conversion
- No tests for integer overflow on 32-bit systems
- Missing: Unicode handling in error messages

#### 4. **GraphQL Integration** (MEDIUM PRIORITY)
- GraphQL tests use mock routes, not actual GraphQL schema validation
- Missing: subscription support tests
- Missing: introspection query tests
- Missing: directive handling
- No federation/stitching tests

#### 5. **Type Safety Across FFI Boundary** (CRITICAL)
- No tests verifying Rust Result<T,E> → PHP throws conversion
- Missing: null safety checks (PHP7 null vs Option<T>)
- No tests for array key type consistency (string vs int)
- Missing: Enum type handling across boundary
- No tests for trait/interface type info loss

#### 6. **Missing Integration Test Files:**
- No PsrComplianceTest.php (PSR-7, PSR-12)
- No FfiTypeSafetyTest.php (ext-php-rs specific)
- No ErrorTranslationTest.php (Rust error → PHP exception)
- No BoundaryConditionsTest.php (FFI edge cases)
- No SerializationEdgeCasesTest.php (large payloads, circular refs)
- No CharacterEncodingTest.php (UTF-8, special chars in JSON)

#### 7. **Test Duplication Detected:**

**AppTest.php vs AppBehavioralTest.php overlap (CONSOLIDATE):**
- AppTest lines 140-187: Route registration via registerController
- AppBehavioralTest lines 97-159: Identical route registration tests
- AppTest lines 314-346: nativeRoutes() coverage
- AppBehavioralTest lines 244-298: Duplicate nativeRoutes() tests
- Recommendation: Merge into single AppControllerTest.php

**StreamingResponseTest vs StreamingResponseTestCase (ELIMINATE):**
- StreamingResponseTestCase is a base class, StreamingResponseTest is its child
- Recommendation: Consolidate duplicated test methods

**ConfigTest vs AppTest configuration sections:**
- AppTest has 80+ lines testing config conversion
- ConfigTest also tests config builders
- Recommendation: Move integration config tests to AppTest, keep ConfigTest focused on builder pattern

#### 8. **Incomplete Coverage Areas:**

**Middleware Integration:**
- Only route-level middleware tested (@Middleware attribute)
- No global middleware stack testing
- Missing: middleware ordering/composition tests
- No tests for error propagation through middleware chain

**WebSocket/SSE:**
- WebSocketSseTest is minimal (60 lines)
- Missing: WebSocket message ordering
- Missing: SSE error handling
- Missing: Connection lifecycle edge cases

**Dependency Injection:**
- DependencyContainerTest is minimal (130 lines)
- Missing: circular dependency detection
- Missing: lazy initialization
- Missing: factory with dependency resolution

**Background Tasks:**
- BackgroundTaskTest (478 lines) but uses mock implementation
- Missing: actual async execution verification
- Missing: error propagation from async context

#### 9. **Edge Cases Not Covered:**

- Empty/null body handling across FFI boundary
- Very large request bodies (>100MB)
- Special characters in headers (UTF-8, control chars)
- Concurrent request handling
- Resource cleanup (file handles, connections)
- Memory leaks from circular references
- Integer overflow in numeric parameters

---

## Recommendations:

### 1. **CONSOLIDATE: Merge AppTest.php + AppBehavioralTest.php**
   **Why**: 60+ test methods are duplicated (route registration, nativeRoutes)
   **Action**:
   - Keep AppTest.php as the unified authority
   - Move configuration conversion tests from AppBehavioralTest→AppTest
   - Remove duplicate route registration tests
   - Delete AppBehavioralTest.php
   **Impact**: Reduce file count from 29→28, eliminate ~150 LOC duplication

### 2. **CONSOLIDATE: ConfigTest.php + ServerConfigBuilderTest.php**
   **Why**: Both test ServerConfig, RateLimitConfig, etc.
   **Action**:
   - Keep ServerConfigBuilderTest.php as authority (more comprehensive)
   - Move ConfigTest validation tests → ServerConfigBuilderTest
   - Delete ConfigTest.php
   **Impact**: Single source of truth for config tests, simpler maintenance

### 3. **NEW TEST FILE: FfiTypeSafetyTest.php** (CRITICAL)
   **Behavior**: Validate type conversion at ext-php-rs boundary
   **Tests**: 12-15 tests
   ```
   - Null pointer handling across FFI
   - Integer type overflow/underflow (PHP 32-bit vs 64-bit)
   - Array key type preservation (int vs string)
   - Boolean false vs 0 distinction
   - Option<T> → nullable type mapping
   - Result<T,E> → exception thrown
   - Large struct serialization (>1MB)
   - Unicode in strings across boundary
   - Circular reference detection
   - Memory cleanup after error
   ```
   **Why**: Critical for runtime safety; prevents silent type corruption

### 4. **NEW TEST FILE: ErrorTranslationTest.php** (CRITICAL)
   **Behavior**: Verify Rust validation errors → PHP exceptions
   **Tests**: 10-12 tests
   ```
   - ValidationError payload structure (error, code, details)
   - HTTP status codes (400 vs 422 vs 500)
   - Stack trace preservation
   - Error code mapping consistency
   - Field-level error messages
   - Multiple validation errors in one response
   - Type mismatch error formatting
   - Header validation errors
   - Cookie validation errors
   - Custom error codes from handlers
   ```
   **Why**: Ensures error contract between Rust and PHP is honored

### 5. **NEW TEST FILE: BoundaryConditionsTest.php** (HIGH)
   **Behavior**: ext-php-rs edge cases and memory safety
   **Tests**: 10-12 tests
   ```
   - Null request handling
   - Empty request body across FFI
   - Maximum request size boundary
   - Handler returning null vs Response
   - Exception during native call
   - Shutdown without cleanup
   - Reusing closed app instance
   - Handler state isolation
   - Multiple concurrent app instances
   - Version compatibility checks
   ```
   **Why**: Prevent memory leaks, crashes, undefined behavior

### 6. **NEW TEST FILE: PsrComplianceTest.php** (MEDIUM-HIGH)
   **Behavior**: Verify PSR-7/PSR-12 compliance where applicable
   **Tests**: 8-10 tests
   ```
   - Header case-insensitivity
   - Response immutability (withStatus, withHeader)
   - Request readonly properties
   - Stream interface compatibility
   - URI parsing from request
   - Method normalization (post→POST)
   - Protocol version handling
   - Status phrase mapping
   ```
   **Note**: Spikard Request/Response may be intentionally non-PSR-7;
   if so, document why and skip tests or create compatibility shim tests

### 7. **NEW TEST FILE: SerializationEdgeCasesTest.php** (MEDIUM)
   **Behavior**: JSON serialization, large payloads, special characters
   **Tests**: 8-10 tests
   ```
   - JSON with emoji/special Unicode
   - Deeply nested arrays (100+ levels)
   - Large string values (10MB+ body)
   - Circular reference in object conversion
   - NaN/Infinity in floats
   - Raw control characters in strings
   - Mixed encoding in request
   - Binary data in JSON
   - Special object types (DateTime, stdClass)
   - Numeric key vs string key consistency
   ```
   **Why**: Catches serialization bugs before production

### 8. **SPLIT: ControllerMethodHandlerTest.php**
   **Why**: 1604 lines is too large; split into logical groups
   **Action**:
   - Keep ControllerMethodHandlerTest for basic parameter resolution (main cases)
   - Move type coercion tests → TypeCoercionTest.php (10-12 tests)
   - Move response conversion tests → ResponseConversionTest.php (12-15 tests)
   - Keep test fixtures in single namespace (Spikard\Tests)
   **Impact**: Easier to maintain, parallel test execution, clearer intent

### 9. **ENHANCE: WebSocketSseTest.php**
   **Current**: 60 lines (minimal)
   **Action**: Expand from 4 tests to 12-15 tests
   ```
   - WebSocket connection lifecycle
   - Message ordering and buffering
   - SSE heartbeat/keepalive
   - Connection error handling
   - Resource cleanup
   - Concurrent connections
   - Broadcast message delivery
   - Binary vs text frames
   ```

### 10. **ENHANCE: DependencyContainerTest.php**
   **Current**: 130 lines (minimal)
   **Action**: Expand from 8 tests to 15-20 tests
   ```
   - Circular dependency detection
   - Lazy factory initialization
   - Dependency resolution order
   - Overwrite existing binding
   - Get non-existent dependency error
   - Type-hinting resolution
   - Named bindings vs auto-wiring
   - Factory with dependencies
   - Singleton vs new instance
   ```

---

## Summary Table:

| File | Current State | Action | New Test Count | Rationale |
|------|--------------|--------|-----------------|-----------|
| AppTest + AppBehavioralTest | 1788 LOC duplicate | Consolidate | 80 total | Eliminate 200+ LOC duplication |
| ConfigTest + ServerConfigBuilderTest | 988 LOC split | Consolidate | 50 total | Single config source of truth |
| ControllerMethodHandlerTest | 1604 LOC monolithic | Split | 60 base + 40 new | Parallel execution, clarity |
| FfiTypeSafetyTest | MISSING | Create | 15 | Critical for type safety |
| ErrorTranslationTest | MISSING | Create | 12 | Critical for error contracts |
| BoundaryConditionsTest | MISSING | Create | 12 | Critical for memory safety |
| PsrComplianceTest | MISSING | Create | 10 | Compliance verification |
| SerializationEdgeCasesTest | MISSING | Create | 10 | Payload safety |
| WebSocketSseTest | 60 LOC sparse | Enhance | 15 total | Better coverage |
| DependencyContainerTest | 130 LOC sparse | Enhance | 20 total | DI edge cases |

---

## Migration Plan:

### Phase 1 (Week 1): Consolidation
1. Merge AppBehavioralTest → AppTest
2. Merge ConfigTest → ServerConfigBuilderTest
3. Update test imports, run full suite (verify 0 regressions)
4. Delete old files from version control

### Phase 2 (Week 2): New Critical Tests
1. Create FfiTypeSafetyTest.php
2. Create ErrorTranslationTest.php
3. Create BoundaryConditionsTest.php
4. Run CI to verify green

### Phase 3 (Week 3): Compliance & Edge Cases
1. Create PsrComplianceTest.php (or mark as N/A if non-PSR)
2. Create SerializationEdgeCasesTest.php
3. Enhance WebSocketSseTest (add 8+ tests)
4. Enhance DependencyContainerTest (add 10+ tests)

### Phase 4 (Week 4): Refactoring
1. Split ControllerMethodHandlerTest into 3 files
2. Update CI to run tests in parallel where possible
3. Run full coverage analysis
4. Update CLAUDE.md with new test patterns

---

## Coverage Targets Post-Reorganization:

- **Total test files**: 32 (from 29, net +3)
- **Total test methods**: 650+ (from 570)
- **Critical areas** (FFI, errors, boundaries): 100% coverage
- **Integration tests**: GraphQL, WebSocket, SSE, DI
- **Consolidation savings**: ~350 LOC removed duplication
- **New test LOC**: ~600 (3 new files × 200 LOC avg)

---

## Critical Notes:

1. **PSR-7**: Verify if Spikard intentionally does NOT implement PSR-7. If so, document and skip PsrComplianceTest. If intended, create adapter/compat layer.

2. **FFI Testing**: FfiTypeSafetyTest is the highest priority—type corruption across FFI boundary can cause silent data loss.

3. **Error Translation**: ErrorTranslationTest ensures Rust validation errors become PHP exceptions with correct structure for client error handling.

4. **Test Fixtures**: All new tests should reuse existing test helpers (make_request, RouteTestHelper) to maintain consistency.

5. **Parallel Execution**: After splitting ControllerMethodHandlerTest, confirm CI can run tests in parallel without state leakage.
