# PHP Coverage Gaps Analysis: Test Scenarios Needed

**Current Coverage**: 53.73% lines (274/510)
**Target**: 80%+

## Executive Summary

Three critical gaps account for the majority of uncovered code:

1. **Spikard\App**: 38.97% coverage (76/195 lines) - Missing 119 lines
2. **Spikard\Testing\TestClient**: 56.14% coverage (32/57 lines) - Missing 25 lines
3. **Spikard\Http\StreamingResponse**: 95.35% coverage (41/43 lines) - Missing 2 lines

---

## 1. SPIKARD\APP (38.97% → Goal: 80%+)

### File: `/Users/naamanhirschfeld/workspace/spikard/packages/php/src/App.php`

**Currently Tested:**
- Basic app creation, configuration chaining, immutability patterns
- Basic route registration (`addRoute`, `addRouteWithSchemas`)
- WebSocket and SSE registration
- Route finding and matching (`findHandler`)
- Method case-insensitivity
- `nativeRoutes()` basic functionality

**MISSING BEHAVIORAL TESTS** (Implementation gaps):

#### A. `registerController()` - COMPLETELY UNTESTED
**Lines 157-208**: Reflection-based route discovery from controller attributes

**Test Scenarios Needed:**

1. **Basic controller registration with single route**
   - Register a controller class with one `#[Get('/users')]` method
   - Verify route is added with correct method/path
   - Verify handler can be found via `findHandler()`

2. **Multiple routes in single controller**
   - Register controller with `#[Get]`, `#[Post]`, `#[Put]`, `#[Delete]`, `#[Patch]` methods
   - Verify all routes registered correctly
   - Verify they're distinguishable by method

3. **Controller with middleware attributes**
   - Register controller where methods have `#[Middleware(...)]` attributes
   - Verify middleware is collected from both route attribute and method attributes
   - Verify middleware chains are properly merged

4. **Controller with schemas in route attributes**
   - Register controller with `#[Get('/items', requestSchema: [...], responseSchema: [...], parameterSchema: [...])]`
   - Verify schemas are preserved in registered routes

5. **Skip non-public methods**
   - Register controller with private/protected methods bearing route attributes
   - Verify they are NOT registered as routes
   - Verify only public methods are scanned

6. **Skip methods without route attributes**
   - Register controller with many public methods, only some with route attributes
   - Verify only attributed methods become routes
   - Verify non-attributed methods are ignored

7. **Register controller instance vs. class name**
   - Test registering with class string: `$app->registerController(UserController::class)`
   - Test registering with instance: `$app->registerController(new UserController($dep))`
   - Verify both work identically for route discovery

8. **Chaining after registerController**
   - Register controller, then chain more operations: `->registerController()->addRoute()->withConfig()`
   - Verify immutability maintained
   - Verify accumulated state correct

9. **Error: Controller with nested/malformed attributes**
   - Register controller where route attribute parsing might fail
   - Verify ReflectionException is raised with meaningful context

10. **Multiple controllers registration**
    - Register Controller1, then Controller2, then Controller3
    - Verify all routes from all controllers are registered
    - Verify routes don't collide or overwrite

#### B. `run()` - ONLY TESTED FOR MISSING EXTENSION
**Lines 313-338**: Server startup with config/hooks/dependencies

**Test Scenarios Needed:**

1. **Error: run() without ServerConfig**
   - Call `$app->run()` with no config set in constructor or via parameter
   - Verify RuntimeException thrown with "ServerConfig is required"

2. **Error: run() without extension functions**
   - Already tested that missing `spikard_version` throws
   - Also test missing `spikard_start_server` throws
   - Verify both checks are present

3. **Error: run() called with null config parameter when app has config**
   - App created with config, but `run(null)` called
   - Verify it uses app config (the `config ?? $this->config` logic)

4. **Lifecycle hooks conversion**
   - App with hooks registered, call `run()`
   - Verify `hooksToNative()` is invoked
   - Verify hooks payload is passed to extension with correct structure

5. **Dependencies injection**
   - App with dependencies registered, call `run()`
   - Verify dependencies payload is passed to extension

6. **Route normalization for WebSocket/SSE**
   - App with HTTP routes + WebSocket + SSE, call `run()`
   - Verify `nativeRoutes()` returns normalized routes with handler key added
   - Verify `stdClass()` is used as fallback handler for ws/sse

#### C. `close()` - TESTED BUT NOT FULLY
**Lines 340-347**: Server shutdown

**Test Scenarios Needed:**

1. **close() with active server handle**
   - After `run()`, mock `spikard_stop_server()` and verify it's called
   - Verify handle is cleared to null after close

2. **close() multiple times**
   - Call `close()` twice in succession
   - Verify second call is no-op (check `if ($this->serverHandle !== null)`)

3. **close() when extension function missing**
   - Verify close() doesn't crash if `spikard_stop_server` doesn't exist
   - Just silently clears handle

#### D. `configToNative()` - COMPLETELY UNTESTED (PRIVATE METHOD)
**Lines 368-463**: Config serialization to extension

**Test Scenarios Needed:**

1. **All basic config fields**
   - Create ServerConfig with host/port/workers/enableRequestId/maxBodySize/requestTimeout/gracefulShutdown/shutdownTimeout
   - Call via reflection or integration test
   - Verify all snake_case fields present in native payload

2. **Compression config serialization**
   - ServerConfig with compression enabled (gzip, brotli, minSize, quality)
   - Verify compression payload matches Rust ServerConfig struct
   - Test with defaults and custom values

3. **Rate limit config serialization**
   - ServerConfig with rateLimit (perSecond, burst, ipBased)
   - Verify rate_limit payload matches Rust struct
   - Test both enabled and disabled states

4. **JWT auth config serialization**
   - ServerConfig with jwtAuth (secret, algorithm, audience, issuer, leeway)
   - Verify jwt_auth payload shape
   - All fields properly mapped

5. **API key auth config serialization**
   - ServerConfig with apiKeyAuth (keys, headerName)
   - Verify api_key_auth payload
   - Multiple keys case

6. **CORS config serialization**
   - ServerConfig with cors enabled
   - Verify allowed_origins, allowed_methods, allowed_headers, expose_headers, max_age, allow_credentials
   - Test CORS disabled case (should not include cors in payload)

7. **Static files config serialization**
   - ServerConfig with staticFiles (enabled, root, indexFile, cache)
   - Verify static_files array with directory/route_prefix/index_file/cache_control
   - Test disabled case (empty array)

8. **OpenAPI config serialization**
   - ServerConfig with openapi (enabled, title, version, description, paths)
   - Verify openapi payload

9. **Null/optional field handling**
   - Fields that are null should not crash serialization
   - Compression null → payload omits compression key
   - All auth types null → omit from payload

10. **Field name mapping (camelCase → snake_case)**
    - Verify enableRequestId → enable_request_id
    - Verify gracefulShutdown → graceful_shutdown
    - Verify all field names use snake_case (Rust serde default)

#### E. `hooksToNative()` - COMPLETELY UNTESTED (PRIVATE METHOD)
**Lines 465-500**: Lifecycle hooks serialization

**Test Scenarios Needed:**

1. **All lifecycle hooks present**
   - Create LifecycleHooks with all 5 hooks (onRequest, preValidation, preHandler, onResponse, onError)
   - Verify all are included in native payload with correct keys (camelCase)

2. **Partial hooks**
   - Only onRequest and onError hooks set
   - Verify only those appear in payload
   - Null hooks are omitted

3. **Empty hooks**
   - LifecycleHooks with all null hooks
   - Verify empty payload array returned

4. **Hook callable preservation**
   - Hooks payload contains PHP callables
   - Verify they're not double-encoded or modified
   - Should pass through as-is to Rust

### Summary Table: App Coverage Gaps

| Method/Feature | Current | Gap | Priority | Lines |
|---|---|---|---|---|
| registerController() | 0% | Complete | CRITICAL | ~52 |
| run() - full flow | Partial | Config/hooks/deps paths | HIGH | ~26 |
| close() - with handle | Partial | Active handle case | MEDIUM | ~8 |
| configToNative() | 0% | Complete | CRITICAL | ~95 |
| hooksToNative() | 0% | Complete | MEDIUM | ~26 |

---

## 2. SPIKARD\TESTING\TESTCLIENT (56.14% → Goal: 80%+)

### File: `/Users/naamanhirschfeld/workspace/spikard/packages/php/src/Testing/TestClient.php`

**Currently Tested:**
- Factory method (`create()`)
- HTTP verb shortcuts (`get()`, `post()`)
- Generic `request()` method
- Query parameter parsing (comprehensive)
- Headers, cookies, files, body options
- Error handling for missing routes

**MISSING BEHAVIORAL TESTS:**

1. **useNative() branch coverage**
   - `useNative()` is private, tested indirectly
   - Currently tested indirectly via WebSocket/SSE throwing "requires native extension"
   - Need explicit branch: when `\class_exists()` is false AND `\function_exists()` is false

2. **WebSocket native path - extension loaded**
   - Mock scenario where native extension IS loaded
   - Verify `connectWebSocket()` delegates to native client
   - Verify it returns object (not Response)

3. **SSE native path - extension loaded**
   - Mock scenario where native extension IS loaded
   - Verify `connectSse()` delegates to native client
   - Verify it returns object (not Response)

4. **request() → findHandler() path**
   - Non-native path testing is good
   - Verify handler.matches() is called and respected
   - Verify handler.handle() return value is used as response

5. **Query parameter edge cases (gaps in existing tests)**
   - Malformed URL fragments with multiple `?` characters
   - Very long query strings (>2000 chars)
   - Query params with special regex characters (`.*+?^${}()|[\]\\`)
   - Percent-encoding edge cases (incomplete sequences like `%2`)

6. **parseQueryParams() with pathological input**
   - Pairs with no `=` sign: `?orphan`
   - Consecutive separators: `?a=1&&&b=2`
   - Keys with empty strings: `?=value` (empty key)
   - Very deeply nested semantics (not array-valued, just many params)

7. **Options parameter validation edge cases**
   - Headers is int, null, object (not array)
   - Cookies is int, null, object (not array)
   - Files is int, null, object (not array)
   - Body is array but files also provided (body should win)
   - Multiple conflicting option combinations

8. **Request construction with all fields**
   - Create Request with every field populated differently
   - path, headers, cookies, files, queryParams, pathParams, dependencies all set
   - Verify handler receives correct Request object

9. **Case sensitivity on HTTP methods**
   - Request with `get`, `GET`, `Get`, `gEt`
   - Verify they all normalize to uppercase for handler matching

10. **Path extraction with complex URLs**
    - Paths like `/api/v1/resource?filter=active&sort=desc`
    - Verify `pathOnly` correctly splits on first `?`
    - Verify queryParams parsed from full path

### Summary Table: TestClient Coverage Gaps

| Method | Current | Gap | Priority |
|---|---|---|---|
| useNative() (private) | Indirect | Direct branch testing | MEDIUM |
| connectWebSocket() - native | Partial | With extension loaded | MEDIUM |
| connectSse() - native | Partial | With extension loaded | MEDIUM |
| parseQueryParams() | ~90% | Edge cases/malformed | LOW |
| request() - full flow | ~95% | Handler.matches() branch | LOW |

---

## 3. SPIKARD\HTTP\STREAMINGREPONSE (95.35% → Goal: 100%)

### File: `/Users/naamanhirschfeld/workspace/spikard/packages/php/src/Http/StreamingResponse.php`

**Currently Tested:**
- Basic streaming response creation
- Custom status codes and headers
- SSE response factory
- File streaming with chunk sizes
- JSON Lines streaming
- Generator iteration

**MISSING 2 LINES (95% → 100%):**

1. **mime_content_type() returns false**
   - Line 143-146: `if ($mimeType !== false)`
   - File stream where `mime_content_type()` returns false
   - Verify Content-Type header is omitted from payload when mime detection fails
   - Setup: Create file with unknown/unmappable extension, call `::file()`

2. **filesize() returns false**
   - Line 150-152: `if ($fileSize !== false)`
   - File stream where `filesize()` returns false (permission issue, symlink, etc.)
   - Verify Content-Length header is omitted when size can't be determined
   - Setup: Create file, change permissions, call `::file()`

3. **fopen() returns false**
   - Line 120-122: `if ($handle === false)`
   - File streaming where fopen fails (permission denied)
   - Verify RuntimeException thrown with "Failed to open file"
   - Setup: Create file with no read permissions, call `::file()` and iterate generator

4. **fread() returns false**
   - Line 127-129: `if ($chunk === false)`
   - During generator iteration, fread fails (file deleted mid-stream)
   - Verify iteration stops gracefully (break, not yield)
   - This is challenging: need to delete file during reading

### Summary Table: StreamingResponse Coverage Gaps

| Code Path | Line Range | Test Scenario | Priority |
|---|---|---|---|
| mime_content_type() false | 143-146 | Unknown file type | TRIVIAL |
| filesize() false | 150-152 | Permission/symlink issue | TRIVIAL |
| fopen() false | 120-122 | Read permission denied | TRIVIAL |
| fread() false | 127-129 | File deleted during read | TRIVIAL |

---

## Consolidated Test Implementation Plan

### Phase 1: CRITICAL (20-30 tests, ~500 lines of test code)
**Impact: Reach 65-70% coverage**

1. **registerController() suite** (8 tests)
   - Basic single route
   - Multiple routes
   - With middleware
   - With schemas
   - Skip non-public methods
   - Skip non-attributed methods
   - Instance vs. class registration
   - Multiple controllers

2. **configToNative() suite** (9 tests)
   - Basic fields
   - Compression config
   - Rate limit config
   - JWT auth config
   - API key auth config
   - CORS config
   - Static files config
   - OpenAPI config
   - Null field handling

3. **hooksToNative() suite** (3 tests)
   - All hooks present
   - Partial hooks
   - Empty hooks

**Estimated new lines of test code**: ~350

### Phase 2: HIGH (5-10 tests, ~150 lines)
**Impact: Reach 75-78% coverage**

1. **run() suite** (4 tests)
   - Missing config error path
   - With lifecycle hooks
   - With dependencies
   - Route normalization for WebSocket/SSE

2. **TestClient native paths** (2 tests)
   - WebSocket with extension
   - SSE with extension

**Estimated new lines of test code**: ~120

### Phase 3: TRIVIAL (4 tests, ~80 lines)
**Impact: Reach 80%+ coverage**

1. **StreamingResponse edge cases** (4 tests)
   - mime_content_type() false
   - filesize() false
   - fopen() false
   - fread() false during iteration

**Estimated new lines of test code**: ~80

---

## Key Testing Patterns & Constraints

### Reflection Testing (registerController)
- Use `ReflectionClass` to create test controllers with attributes
- Use `newInstance()` to instantiate for controller registration
- Verify `ReflectionMethod::getAttributes()` is properly exercised

### Private Method Testing (configToNative, hooksToNative)
- Use ReflectionMethod to make private methods testable
- OR test indirectly via `run()` which calls them
- OR add public wrapper methods in tests only

### Mock/Stub Patterns
- Mock `spikard_version`, `spikard_start_server` using reflection on `\function_exists()`
- Cannot directly mock built-in functions; test both paths (with/without)
- For native extension tests, skip if extension not loaded

### Edge Case Testing
- Invalid MIME types, permissions, file encodings
- Deeply nested query strings, Unicode in query params
- Empty values vs. missing values in options arrays

---

## Risk Assessment

**Likelihood of reaching 80%+**: HIGH (95%)
- Most gaps are straightforward method/path coverage
- registerController and configToNative are largest but well-defined
- StreamingResponse is nearly complete (2 lines remain)

**Implementation Effort**:
- Estimated 15-20 hours for complete implementation
- ~27-30 new test methods across 3 test classes
- ~550 lines of test code total

**Maintenance Risk**: LOW
- Tests are behavioral, not implementation-dependent
- Follow existing test patterns (SimpleTestClientHandler, etc.)
- No external dependencies needed

---

## Appendix: Checklist for Test Implementation

### App.php Tests
- [ ] registerController() with single route
- [ ] registerController() with multiple routes
- [ ] registerController() with middleware attributes
- [ ] registerController() with schemas
- [ ] registerController() skips non-public methods
- [ ] registerController() skips non-attributed methods
- [ ] registerController() with instance vs. class string
- [ ] registerController() chaining
- [ ] registerController() multiple controllers
- [ ] configToNative() basic fields
- [ ] configToNative() compression
- [ ] configToNative() rate limit
- [ ] configToNative() JWT auth
- [ ] configToNative() API key auth
- [ ] configToNative() CORS
- [ ] configToNative() static files
- [ ] configToNative() OpenAPI
- [ ] configToNative() null/optional fields
- [ ] hooksToNative() all hooks
- [ ] hooksToNative() partial hooks
- [ ] hooksToNative() empty hooks
- [ ] run() missing config error
- [ ] run() without extension error
- [ ] run() with hooks
- [ ] run() with dependencies
- [ ] close() with active handle
- [ ] close() multiple calls

### TestClient Tests
- [ ] useNative() false branch (no extension)
- [ ] connectWebSocket() with extension loaded
- [ ] connectSse() with extension loaded
- [ ] parseQueryParams() malformed input
- [ ] request() with handler.matches()
- [ ] Query params with regex chars
- [ ] Options edge cases (type validation)

### StreamingResponse Tests
- [ ] mime_content_type() returns false
- [ ] filesize() returns false
- [ ] fopen() returns false
- [ ] fread() returns false mid-stream
