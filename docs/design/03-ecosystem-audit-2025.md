# Spikard Rust Implementation: Ecosystem Audit (2025)

**Date:** 2025-10-31
**Author:** Claude Code
**Purpose:** Identify areas where Spikard is reinventing the wheel instead of leveraging mature Rust ecosystem solutions

---

## Executive Summary

This audit identified **12 major areas** where Spikard could benefit from adopting mature ecosystem libraries instead of custom implementations. The recommendations are prioritized by impact (maintenance burden reduction, bug reduction, feature richness) and ease of migration.

**Key Findings:**
- ✅ **Already using ecosystem solutions well:** `jsonschema` crate, `serde_qs` for bracket notation, `mime` crate
- ⚠️ **Reinventing wheels:** Cookie parsing, date/time validation, query string parsing (partially), header parsing
- 🔴 **Missing validation infrastructure:** No `validator` or `garde` integration, manual validation logic throughout

---

## High Priority Recommendations

### 1. Cookie Parsing - Use `cookie` crate

**Current Implementation:**
```rust
// crates/spikard-http/src/server.rs:107-125
fn extract_cookies(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE)
        && let Ok(cookie_str) = cookie_header.to_str()
    {
        // Parse cookie header: "name1=value1; name2=value2"
        for cookie_pair in cookie_str.split(';') {
            let cookie_pair = cookie_pair.trim();
            if let Some((name, value)) = cookie_pair.split_once('=') {
                cookies.insert(name.trim().to_string(), value.trim().to_string());
            }
        }
    }
    cookies
}
```

**Issues:**
- ❌ No percent-decoding of cookie values
- ❌ No support for quoted values
- ❌ No validation of cookie names/values per RFC 6265
- ❌ Missing edge case handling (empty values, special characters)
- ❌ Cannot set cookies with attributes (Secure, HttpOnly, SameSite, Path, Domain)

**Recommended Solution:**
```rust
use cookie::{Cookie, CookieJar};

fn extract_cookies(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE)
        && let Ok(cookie_str) = cookie_header.to_str()
    {
        // Cookie::split_parse_encoded() handles percent-decoding
        for result in Cookie::split_parse_encoded(cookie_str) {
            if let Ok(cookie) = result {
                cookies.insert(cookie.name().to_string(), cookie.value().to_string());
            }
        }
    }
    cookies
}
```

**Benefits:**
- ✅ RFC 6265 compliant parsing
- ✅ Automatic percent-decoding with `split_parse_encoded()`
- ✅ Proper handling of quoted values and edge cases
- ✅ Battle-tested library (used by actix-web, rocket, warp)
- ✅ Support for signed cookies (via `cookie::SignedJar`) when needed
- ✅ Support for encrypted cookies (via `cookie::PrivateJar`) when needed

**Migration Difficulty:** 🟢 Easy (drop-in replacement)

**Priority:** 🔴 High (security implications)

**Dependencies to add:**
```toml
cookie = "0.18"  # Latest stable version
```

---

### 2. Replace `tower-cookies` with `axum-extra` Cookie Extractors

**Current Approach:**
Manual cookie extraction in every route handler

**Recommended Solution:**
```rust
use axum_extra::extract::cookie::{Cookie, CookieJar};

// In handler:
async fn my_handler(jar: CookieJar) -> impl IntoResponse {
    // Access cookies easily
    if let Some(cookie) = jar.get("session") {
        let value = cookie.value();
        // ...
    }

    // Set cookies
    let cookie = Cookie::new("name", "value");
    jar.add(cookie);

    // ...
}
```

**Benefits:**
- ✅ Built on top of the `cookie` crate
- ✅ Integrates seamlessly with Axum extractors
- ✅ Supports signed and private cookies out of the box
- ✅ Type-safe cookie handling
- ✅ Part of the axum ecosystem (consistent patterns)

**Migration Difficulty:** 🟡 Medium (requires handler signature changes)

**Priority:** 🔴 High (better architecture)

**Dependencies to add:**
```toml
axum-extra = { version = "0.9", features = ["cookie", "cookie-private"] }
```

---

### 3. Date/Time Validation - Use `jiff` or `chrono`

**Current Implementation:**
```rust
// crates/spikard-http/src/parameters.rs:435-556
// ~120 lines of manual date/time validation code
fn validate_date_format(value: &str) -> Result<(), String> {
    // Manual parsing of YYYY-MM-DD
    if value.len() != 10 { return Err(...); }
    let parts: Vec<&str> = value.split('-').collect();
    // ... manual validation of year, month, day
}

fn validate_datetime_format(value: &str) -> Result<(), String> {
    // Manual parsing of ISO 8601 datetime
    if !value.contains('T') { return Err(...); }
    // ... manual validation
}

fn validate_time_format(value: &str) -> Result<(), String> {
    // Manual parsing of HH:MM:SS
    // ... manual validation
}

fn validate_duration_format(value: &str) -> Result<(), String> {
    // Accepts anything starting with P or digits
    if value.starts_with('P') || /* ... */ { Ok(()) } else { Err(...) }
}
```

**Issues:**
- ❌ No actual parsing - just format validation
- ❌ Doesn't validate semantic correctness (e.g., month=13, day=32)
- ❌ Duration validation is a no-op (accepts invalid ISO 8601 durations)
- ❌ No timezone validation
- ❌ No leap year handling for dates
- ❌ Doesn't catch "2023-02-30" as invalid

**Recommended Solution - Option A (Modern): `jiff`**
```rust
use jiff::{civil::Date, civil::Time, Timestamp, Span};

fn validate_date_format(value: &str) -> Result<(), String> {
    Date::strptime("%Y-%m-%d", value)
        .map(|_| ())
        .map_err(|e| format!("Invalid date: {}", e))
}

fn validate_datetime_format(value: &str) -> Result<(), String> {
    Timestamp::from_str(value)
        .map(|_| ())
        .map_err(|e| format!("Invalid datetime: {}", e))
}

fn validate_time_format(value: &str) -> Result<(), String> {
    Time::strptime("%H:%M:%S", value)
        .map(|_| ())
        .map_err(|e| format!("Invalid time: {}", e))
}

fn validate_duration_format(value: &str) -> Result<(), String> {
    Span::from_str(value)
        .map(|_| ())
        .map_err(|e| format!("Invalid duration: {}", e))
}
```

**Recommended Solution - Option B (Established): `chrono`**
```rust
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, DateTime};

fn validate_date_format(value: &str) -> Result<(), String> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map(|_| ())
        .map_err(|e| format!("Invalid date: {}", e))
}

fn validate_datetime_format(value: &str) -> Result<(), String> {
    DateTime::parse_from_rfc3339(value)
        .map(|_| ())
        .map_err(|e| format!("Invalid datetime: {}", e))
}

fn validate_time_format(value: &str) -> Result<(), String> {
    NaiveTime::parse_from_str(value, "%H:%M:%S")
        .map(|_| ())
        .map_err(|e| format!("Invalid time: {}", e))
}
```

**Benefits:**
- ✅ Actual parsing, not just format checking
- ✅ Semantic validation (invalid dates like "2023-02-30" are rejected)
- ✅ Proper ISO 8601 duration parsing
- ✅ Timezone support (jiff has better IANA timezone support)
- ✅ ~100 lines of code removed
- ✅ Battle-tested libraries

**Recommendation:** Use `jiff` for new code (modern, designed for success), or `chrono` if you need immediate stability.

**Migration Difficulty:** 🟢 Easy (drop-in replacement)

**Priority:** 🔴 High (correctness issues)

**Dependencies to add:**
```toml
# Option A (recommended for new projects):
jiff = "0.1"

# Option B (established, stable):
chrono = { version = "0.4", features = ["serde"] }
```

---

### 4. Validation Infrastructure - Add `validator` or `garde`

**Current Implementation:**
Custom validation logic scattered throughout codebase:
- `crates/spikard-http/src/validation.rs`: 380+ lines of custom JSON Schema error mapping
- `crates/spikard-http/src/parameters.rs`: 580+ lines of custom parameter validation
- Manual type coercion, format validation, constraint checking

**Issues:**
- ❌ Validation logic is not reusable across the codebase
- ❌ No declarative validation for Rust structs
- ❌ Can't validate request parameters before they reach handlers
- ❌ Custom error format mapping is verbose and error-prone
- ❌ No built-in validators for common patterns (email, URL, IP address)

**Recommended Solution - Option A (Modern): `garde`**
```rust
use garde::Validate;

#[derive(Debug, Validate, Deserialize)]
struct QueryParams {
    #[garde(length(min = 3, max = 50))]
    name: String,

    #[garde(range(min = 0, max = 120))]
    age: i32,

    #[garde(email)]
    email: String,

    #[garde(url)]
    website: Option<String>,

    #[garde(length(min = 1))]
    #[garde(dive)]  // Validate each item in the vec
    tags: Vec<String>,
}

// In handler:
async fn create_user(
    ValidatedQuery(params): ValidatedQuery<QueryParams>
) -> impl IntoResponse {
    // params is already validated!
    // ...
}
```

**Recommended Solution - Option B (Established): `validator`**
```rust
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct QueryParams {
    #[validate(length(min = 3, max = 50))]
    name: String,

    #[validate(range(min = 0, max = 120))]
    age: i32,

    #[validate(email)]
    email: String,

    #[validate(url)]
    website: Option<String>,
}

// With axum-validator:
use axum_valid::Valid;

async fn create_user(
    Valid(Query(params)): Valid<Query<QueryParams>>
) -> impl IntoResponse {
    // params is already validated!
    // ...
}
```

**Benefits:**
- ✅ Declarative validation with derive macros
- ✅ Built-in validators for common patterns
- ✅ Composable validation rules
- ✅ Better error messages
- ✅ Can validate nested structs
- ✅ Integration with Axum via extractors
- ✅ Reduces boilerplate by 60-70%

**Migration Difficulty:** 🟡 Medium (requires refactoring handlers)

**Priority:** 🔴 High (architectural improvement)

**Dependencies to add:**
```toml
# Option A (recommended for new projects):
garde = "0.20"
garde_axum = "0.20"  # For Axum integration

# Option B (established, stable):
validator = { version = "0.19", features = ["derive"] }
axum-valid = "0.23"  # For Axum integration
```

---

### 5. Query String Parsing - Fully Adopt `serde_qs` or `serde_urlencoded`

**Current Implementation:**
Custom query parser in `crates/spikard-http/src/query_parser.rs` (~300 lines)

**Analysis:**
- ✅ Already using `serde_qs` for bracket notation in middleware
- ❌ Custom parser duplicates much of what `serde_qs` does
- ❌ Custom type conversion logic could be handled by serde deserializers
- ❌ Mixing custom parser with `serde_qs` creates inconsistency

**Current Hybrid Approach:**
```rust
// middleware.rs uses serde_qs:
if body_str.contains('[') {
    let config = serde_qs::Config::new(10, false);
    let parsed: HashMap<String, serde_json::Value> = config.deserialize_str(body_str)?;
    // ...
} else {
    // Falls back to custom query_parser
    Ok(crate::query_parser::parse_query_string_to_json(data, true))
}
```

**Recommended Solution:**
```rust
// Use serde_qs consistently for all query/form parsing
use serde_qs;

fn parse_query_string<T: DeserializeOwned>(qs: &str) -> Result<T, Error> {
    let config = serde_qs::Config::new(10, false);
    config.deserialize_str(qs)
}

// For JSON values specifically:
fn parse_query_string_to_json(qs: &str) -> Result<Value, Error> {
    let config = serde_qs::Config::new(10, false);
    let parsed: HashMap<String, serde_json::Value> = config.deserialize_str(qs)?;
    Ok(serde_json::to_value(parsed)?)
}
```

**Benefits:**
- ✅ Remove ~200 lines of custom parsing code
- ✅ Consistent parsing everywhere
- ✅ Better nested object support
- ✅ Configurable max depth (prevents DoS)
- ✅ Better error messages
- ✅ Already using it, just expand usage

**Alternative:** Use `serde_urlencoded` for simpler flat key-value parsing (smaller dependency)

**Migration Difficulty:** 🟢 Easy (already partially using)

**Priority:** 🟡 Medium (cleanup, not critical)

**Current Dependencies:**
```toml
serde_qs = "0.15"  # Already present
```

---

## Medium Priority Recommendations

### 6. HTTP Header Utilities - Leverage `axum::http` More

**Current Implementation:**
```rust
// server.rs:95-105
fn extract_headers(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (name, value) in headers.iter() {
        if let Ok(val_str) = value.to_str() {
            // Convert header name to lowercase for consistent access
            map.insert(name.as_str().to_lowercase(), val_str.to_string());
        }
    }
    map
}
```

**Issues:**
- ❌ Loses header metadata (typed headers, repeated headers)
- ❌ String conversion ignores non-UTF8 header values
- ❌ Lowercasing loses original casing (sometimes needed)
- ❌ HashMap doesn't support multiple values for same header

**Recommended Solution:**
```rust
use axum::http::{HeaderMap, HeaderName, HeaderValue};
use axum_extra::TypedHeader;

// Option A: Use TypedHeader extractors in handlers
async fn my_handler(
    TypedHeader(content_type): TypedHeader<headers::ContentType>,
    TypedHeader(authorization): TypedHeader<headers::Authorization>,
) -> impl IntoResponse {
    // Type-safe header access
}

// Option B: Keep generic access but use HeaderMap directly
fn validate_headers(headers: &HeaderMap) -> Result<(), Error> {
    // Access headers by typed constants
    if let Some(ct) = headers.get(header::CONTENT_TYPE) {
        // ...
    }

    // Get all values for a header (for repeated headers)
    let all_cookies: Vec<_> = headers
        .get_all(header::COOKIE)
        .iter()
        .collect();

    Ok(())
}
```

**Benefits:**
- ✅ Type-safe header access
- ✅ Better handling of non-UTF8 values
- ✅ Support for repeated headers
- ✅ Better integration with Axum patterns
- ✅ No conversion overhead

**Migration Difficulty:** 🟡 Medium (changes handler signatures)

**Priority:** 🟡 Medium (quality improvement)

**Dependencies to add:**
```toml
axum-extra = { version = "0.9", features = ["typed-header"] }
headers = "0.4"  # For typed header types
```

---

### 7. Content-Type Parsing - Already Using `mime` (Good!)

**Current Implementation:**
```rust
// middleware.rs:72-79
let is_form_urlencoded = content_type_str
    .parse::<mime::Mime>()
    .map(|mime| {
        mime.type_() == mime::APPLICATION
            && mime.subtype() == "x-www-form-urlencoded"
    })
    .unwrap_or(false);
```

**Status:** ✅ Already using `mime` crate correctly!

**Recommendation:** Continue current approach. Consider:
1. Extract common MIME type checks into helper functions
2. Add more comprehensive MIME type constants

**Example improvement:**
```rust
// Add helper module
mod mime_helpers {
    use mime::Mime;

    pub fn is_json(mime: &Mime) -> bool {
        (mime.type_() == mime::APPLICATION && mime.subtype() == mime::JSON)
            || mime.suffix() == Some(mime::JSON)
    }

    pub fn is_form_urlencoded(mime: &Mime) -> bool {
        mime.type_() == mime::APPLICATION
            && mime.subtype() == "x-www-form-urlencoded"
    }

    pub fn is_multipart_form_data(mime: &Mime) -> bool {
        mime.type_() == mime::MULTIPART
            && mime.subtype() == "form-data"
    }
}
```

**Priority:** 🟢 Low (already good, just polish)

---

### 8. JSON Schema Validation - Already Using `jsonschema` (Good!)

**Current Implementation:**
```rust
// validation.rs:16-30
let compiled = jsonschema::options()
    .with_draft(jsonschema::Draft::Draft202012)
    .should_validate_formats(true)
    .build(&schema)
    .map_err(|e| format!("Invalid JSON Schema: {}", e))?;
```

**Status:** ✅ Already using `jsonschema` crate correctly!

**Recommendations:**
1. ✅ Using Draft 2020-12 (good!)
2. ✅ Format validation enabled (good!)
3. Consider: Configure regex engine for security
4. Consider: Cache compiled validators more aggressively

**Example improvement:**
```rust
// For better security, switch to regex engine (prevents ReDoS):
let compiled = jsonschema::options()
    .with_draft(jsonschema::Draft::Draft202012)
    .should_validate_formats(true)
    .with_regex_engine(jsonschema::RegexEngine::Regex)  // ← Add this
    .build(&schema)?;
```

**Benefits of regex engine:**
- ✅ Guaranteed linear-time matching
- ✅ Prevents ReDoS attacks
- ✅ More performant for most cases

**Priority:** 🟢 Low (already good, security enhancement)

---

### 9. Error Handling - Consider `anyhow` or `eyre` for Better Context

**Current Implementation:**
Mix of custom error types, String errors, and thiserror

**Observed:**
```rust
// Various error types throughout:
Result<T, String>  // In many places
Result<T, ValidationError>  // Custom type
map_err(|e| format!("..."))  // Lots of string formatting
```

**Recommended Solution:**
```rust
use anyhow::{Context, Result};

// Instead of:
fn parse_value(s: &str) -> Result<i32, String> {
    s.parse().map_err(|e| format!("Failed to parse: {}", e))
}

// Use:
fn parse_value(s: &str) -> anyhow::Result<i32> {
    s.parse()
        .context("Failed to parse integer value")
}

// Chaining context:
fn process_request(req: Request) -> anyhow::Result<Response> {
    let body = extract_body(&req)
        .context("Failed to extract request body")?;

    let parsed = parse_json(&body)
        .context("Failed to parse JSON")
        .context("Invalid request format")?;

    Ok(create_response(parsed))
}
```

**Benefits:**
- ✅ Better error context chaining
- ✅ Easier debugging (full error chain)
- ✅ Less boilerplate error formatting
- ✅ Supports automatic conversion from most error types
- ✅ Can use `.context()` to add context without mapping

**Alternative:** `eyre` (fork of anyhow with more features)

**Migration Difficulty:** 🟡 Medium (changes error types throughout)

**Priority:** 🟡 Medium (quality of life improvement)

**Dependencies to add:**
```toml
anyhow = "1.0"  # Already in spikard-codegen, add to http
# OR
eyre = "0.6"
```

---

### 10. Middleware Organization - Use `tower-http` Middleware Collection

**Current Implementation:**
Single custom middleware in `middleware.rs`

**Recommendation:**
Leverage `tower-http` middleware for common tasks:

```rust
use tower_http::{
    trace::TraceLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    limit::RequestBodyLimitLayer,
    validate_request::ValidateRequestHeaderLayer,
    cors::CorsLayer,
    catch_panic::CatchPanicLayer,
};

let app = Router::new()
    .route("/api/users", get(users_handler))
    // Add middleware layers
    .layer(
        ServiceBuilder::new()
            // Panic recovery
            .layer(CatchPanicLayer::new())
            // Timeout after 30 seconds
            .layer(TimeoutLayer::new(Duration::from_secs(30)))
            // Request size limit (10MB)
            .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
            // Compression
            .layer(CompressionLayer::new())
            // CORS
            .layer(CorsLayer::permissive())
            // Tracing
            .layer(TraceLayer::new_for_http())
            // Custom middleware
            .layer(axum::middleware::from_fn(validate_content_type_middleware))
    );
```

**Available `tower-http` middleware:**
- ✅ `TraceLayer` - Request tracing (already using!)
- `CompressionLayer` - Response compression (gzip, br, deflate)
- `TimeoutLayer` - Request timeouts
- `RequestBodyLimitLayer` - Body size limits (DoS prevention)
- `ValidateRequestHeaderLayer` - Header validation
- `CorsLayer` - CORS handling
- `CatchPanicLayer` - Panic recovery
- `SetRequestIdLayer` - Request ID generation
- `PropagateRequestIdLayer` - Request ID propagation
- `NormalizePath` - Path normalization (trailing slashes)
- `AddAuthorizationLayer` - Authorization headers
- `SetSensitiveHeadersLayer` - Sensitive header handling

**Benefits:**
- ✅ Battle-tested middleware
- ✅ Consistent patterns
- ✅ Less code to maintain
- ✅ Better performance
- ✅ Already using `tower-http` for `TraceLayer`

**Migration Difficulty:** 🟢 Easy (incremental addition)

**Priority:** 🟡 Medium (nice to have)

**Current Dependencies:**
```toml
tower-http = { version = "0.6", features = ["trace"] }
# Add more features:
tower-http = { version = "0.6", features = [
    "trace",
    "compression-gzip",
    "timeout",
    "limit",
    "cors",
    "catch-panic",
] }
```

---

## Low Priority Recommendations

### 11. URL Parsing - Consider `url` crate for Complex URL Operations

**Current Usage:**
Using Axum's `Uri` type (which is adequate for most cases)

**Recommendation:**
For complex URL manipulation (building URLs, query string manipulation), consider:

```rust
use url::Url;

fn build_api_url(base: &str, path: &str, params: &[(&str, &str)]) -> Result<Url, url::ParseError> {
    let mut url = Url::parse(base)?.join(path)?;
    url.query_pairs_mut().extend_pairs(params);
    Ok(url)
}
```

**When to use:**
- Building URLs programmatically
- Complex query string manipulation
- URL validation beyond what Axum provides
- Working with relative URLs

**Priority:** 🟢 Low (current approach is fine)

---

### 12. Regex Compilation - Use `lazy_static` or `LazyLock` (Already Using!)

**Current Implementation:**
```rust
// query_parser.rs:15-17
lazy_static! {
    static ref PARENTHESES_RE: Regex = Regex::new(r"(^\[.*\]$|^\{.*\}$)").unwrap();
}
```

**Status:** ✅ Already using `lazy_static` correctly!

**Modern Alternative:** Since Rust 1.80, consider migrating to `std::sync::LazyLock`:

```rust
use std::sync::LazyLock;

static PARENTHESES_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(^\[.*\]$|^\{.*\}$)").unwrap()
});
```

**Benefits:**
- ✅ No external dependency
- ✅ Standard library solution
- ✅ Same performance

**Priority:** 🟢 Low (nice to have, not critical)

---

## Summary Table

| # | Area | Current | Recommended | Priority | Difficulty | Lines Saved |
|---|------|---------|-------------|----------|------------|-------------|
| 1 | Cookie Parsing | Manual split/trim | `cookie` crate | 🔴 High | 🟢 Easy | ~20 |
| 2 | Cookie Handling | Manual extraction | `axum-extra` extractors | 🔴 High | 🟡 Medium | ~50 |
| 3 | Date/Time Validation | Manual parsing | `jiff` or `chrono` | 🔴 High | 🟢 Easy | ~120 |
| 4 | Validation Infrastructure | Manual validation | `garde` or `validator` | 🔴 High | 🟡 Medium | ~300 |
| 5 | Query String Parsing | Custom + serde_qs | Full `serde_qs` | 🟡 Medium | 🟢 Easy | ~200 |
| 6 | HTTP Headers | HashMap conversion | `TypedHeader` | 🟡 Medium | 🟡 Medium | ~30 |
| 7 | Content-Type Parsing | `mime` crate | ✅ Keep current | 🟢 Low | N/A | 0 |
| 8 | JSON Schema | `jsonschema` crate | ✅ Keep + enhance | 🟢 Low | 🟢 Easy | 0 |
| 9 | Error Context | String errors | `anyhow` or `eyre` | 🟡 Medium | 🟡 Medium | ~50 |
| 10 | Middleware | Custom only | `tower-http` suite | 🟡 Medium | 🟢 Easy | Varies |
| 11 | URL Parsing | `Uri` type | ✅ Adequate | 🟢 Low | N/A | 0 |
| 12 | Lazy Statics | `lazy_static` | ✅ Good (or `LazyLock`) | 🟢 Low | 🟢 Easy | 0 |

**Total Potential Lines Saved:** ~770 lines of custom code
**High Priority Issues:** 4
**Security-Related:** 3 (cookies, date/time validation, validation infrastructure)

---

## Migration Strategy

### Phase 1: Security & Correctness (Week 1-2)
1. ✅ Add `cookie` crate and fix cookie parsing
2. ✅ Add `jiff`/`chrono` and fix date/time validation
3. ✅ Add ReDoS protection to `jsonschema` configuration

### Phase 2: Validation Infrastructure (Week 3-4)
4. ✅ Add `garde` or `validator` crate
5. ✅ Refactor parameter validation to use declarative validators
6. ✅ Update handler signatures to use validation extractors

### Phase 3: Cleanup & Polish (Week 5-6)
7. ✅ Consolidate query string parsing to `serde_qs`
8. ✅ Migrate to `axum-extra` cookie extractors
9. ✅ Add `tower-http` middleware where beneficial
10. ✅ Replace String errors with `anyhow`/`eyre`

### Phase 4: Nice-to-Haves (Ongoing)
11. ✅ Migrate to `std::sync::LazyLock` when minimum Rust version allows
12. ✅ Enhance header handling with `TypedHeader` where appropriate

---

## Conclusion

Spikard is already using several ecosystem solutions correctly (`jsonschema`, `serde_qs` partially, `mime`, `lazy_static`). However, there are significant opportunities to:

1. **Reduce maintenance burden** by removing ~770 lines of custom code
2. **Improve security** with proper cookie parsing and ReDoS protection
3. **Fix correctness issues** with date/time validation
4. **Adopt modern patterns** with validation infrastructure

The highest ROI comes from:
- 🔴 **Cookie parsing** (security + correctness)
- 🔴 **Date/time validation** (correctness)
- 🔴 **Validation infrastructure** (architecture + maintainability)

These three changes alone would eliminate ~440 lines of custom code and fix multiple security/correctness issues.

---

## Resources

- **Cookie crate:** https://docs.rs/cookie/latest/cookie/
- **Jiff crate:** https://docs.rs/jiff/latest/jiff/
- **Chrono crate:** https://docs.rs/chrono/latest/chrono/
- **Garde crate:** https://docs.rs/garde/latest/garde/
- **Validator crate:** https://docs.rs/validator/latest/validator/
- **Tower-HTTP:** https://docs.rs/tower-http/latest/tower_http/
- **Axum-Extra:** https://docs.rs/axum-extra/latest/axum_extra/
- **JSON Schema crate:** https://docs.rs/jsonschema/latest/jsonschema/
