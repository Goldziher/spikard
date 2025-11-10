# Ecosystem Library Comparisons

This document provides detailed comparisons of alternative crates for key functionality.

---

## Cookie Parsing

### Option 1: `cookie` crate ⭐ RECOMMENDED
- **Stars:** 600+
- **Maturity:** Very stable, used by Rocket, Actix-Web
- **Features:** RFC 6265 compliant, signed/private cookies, percent-encoding
- **Dependencies:** Minimal (time, subtle for crypto features)
- **Version:** 0.18 (stable)
- **Use when:** You need robust cookie handling
- **Docs:** https://docs.rs/cookie/

### Option 2: `tower-cookies`
- **Stars:** 300+
- **Maturity:** Stable
- **Features:** Built on `cookie` crate, Tower middleware layer
- **Dependencies:** `cookie` + `tower`
- **Version:** 0.10
- **Use when:** You want Tower middleware for cookies (but `axum-extra` is better for Axum)
- **Docs:** https://docs.rs/tower-cookies/

### Option 3: Manual (current) ❌ NOT RECOMMENDED
- **Issues:** No percent-decoding, missing RFC compliance, security risks
- **When to use:** Never for production

**Verdict:** Use `cookie` crate directly, or `axum-extra` cookie extractors for Axum integration

---

## Date/Time Handling

### Option 1: `jiff` ⭐ RECOMMENDED FOR NEW CODE
- **Stars:** 1,800+
- **Maturity:** Recently released (2024), active development
- **Author:** BurntSushi (creator of ripgrep, regex)
- **Features:**
  - Built-in IANA timezone database
  - ISO 8601 duration parsing (best-in-class)
  - Civil date/time types (no timezone confusion)
  - Designed for "pit of success"
  - Excellent error messages
- **Dependencies:** Reasonable
- **Version:** 0.1.x (targeting 1.0 in summer 2025)
- **Use when:** Starting new code, need timezone-aware operations
- **Docs:** https://docs.rs/jiff/

**Pros:**
- ✅ Best timezone support
- ✅ Clear API design
- ✅ Comprehensive ISO 8601 support
- ✅ Modern Rust idioms

**Cons:**
- ⚠️ Not yet 1.0 (but from trusted author)
- ⚠️ Smaller ecosystem than chrono

### Option 2: `chrono` - RECOMMENDED FOR STABILITY
- **Stars:** 3,300+
- **Maturity:** Very mature, de facto standard
- **Features:**
  - RFC 3339 / ISO 8601 parsing
  - Timezone support (via separate `chrono-tz` crate)
  - Widely used across ecosystem
- **Dependencies:** Moderate
- **Version:** 0.4.x (stable)
- **Use when:** Need maximum stability and ecosystem compatibility
- **Docs:** https://docs.rs/chrono/

**Pros:**
- ✅ Battle-tested
- ✅ Huge ecosystem
- ✅ Stable API

**Cons:**
- ⚠️ API can be confusing (fixed vs timezone-aware)
- ⚠️ Requires separate crate for timezones

### Option 3: `time`
- **Stars:** 1,100+
- **Maturity:** Stable
- **Features:** Similar to chrono, focuses on security and correctness
- **Version:** 0.3.x
- **Use when:** You want an alternative to chrono
- **Docs:** https://docs.rs/time/

**Pros:**
- ✅ Security-focused
- ✅ Good API design

**Cons:**
- ⚠️ Smaller ecosystem than chrono
- ⚠️ Different API patterns

### Option 4: Manual (current) ❌ NOT RECOMMENDED
- **Issues:** Doesn't validate semantic correctness, duration validation is broken
- **When to use:** Never

**Verdict:**
- **New code:** Use `jiff` (best features, clear API)
- **Conservative choice:** Use `chrono` (most stable, widest usage)
- **Security focus:** Use `time`

---

## Validation

### Option 1: `garde` ⭐ RECOMMENDED FOR NEW CODE
- **Stars:** 500+
- **Maturity:** Modern rewrite of validator
- **Features:**
  - Derive macro validation
  - Better error messages than validator
  - Nested validation with `dive`
  - Custom validators
  - Context support
  - Axum integration via `garde_axum`
- **Version:** 0.20
- **Use when:** Starting new validation or want better errors
- **Docs:** https://docs.rs/garde/

**Pros:**
- ✅ Modern design
- ✅ Better error messages
- ✅ Cleaner API than validator
- ✅ Good Axum integration

**Cons:**
- ⚠️ Newer (less battle-tested)
- ⚠️ Smaller ecosystem

### Option 2: `validator` - RECOMMENDED FOR STABILITY
- **Stars:** 3,600+
- **Maturity:** Very mature, widely used
- **Features:**
  - Derive macro validation
  - Built-in validators (email, url, ip, length, range)
  - Custom validation functions
  - Nested validation
  - i18n support
  - Axum integration via `axum-valid`
- **Version:** 0.19
- **Use when:** Need maximum ecosystem compatibility
- **Docs:** https://docs.rs/validator/

**Pros:**
- ✅ Battle-tested
- ✅ Huge ecosystem support
- ✅ Many framework integrations

**Cons:**
- ⚠️ API is older, less ergonomic
- ⚠️ Error messages not as good as garde

### Option 3: `validify`
- **Stars:** 100+
- **Maturity:** Newer
- **Features:** Another validation library, similar to validator
- **Use when:** You want to try alternatives
- **Docs:** https://docs.rs/validify/

### Option 4: Manual (current) ❌ NOT RECOMMENDED
- **Issues:** Verbose, not reusable, scattered logic
- **When to use:** Only for very domain-specific validation

**Verdict:**
- **New code:** Use `garde` (better UX)
- **Conservative:** Use `validator` (most proven)

---

## Query String / Form Parsing

### Option 1: `serde_qs` ⭐ RECOMMENDED (already using)
- **Stars:** 400+
- **Maturity:** Stable
- **Features:**
  - Bracket notation support: `tags[]=a&tags[]=b`
  - Nested objects: `profile[name]=John&profile[age]=30`
  - Configurable max depth (DoS prevention)
  - Built on serde
- **Version:** 0.15
- **Use when:** Need nested objects or arrays
- **Docs:** https://docs.rs/serde_qs/

**Pros:**
- ✅ Most feature-rich
- ✅ Already using it
- ✅ Handles complex structures

**Cons:**
- ⚠️ Slightly larger dependency

### Option 2: `serde_urlencoded`
- **Stars:** Built into serde ecosystem
- **Maturity:** Very stable
- **Features:**
  - Simple flat key-value parsing
  - Official serde solution
  - Minimal dependencies
- **Version:** 0.7
- **Use when:** Only need simple flat forms
- **Docs:** https://docs.rs/serde_urlencoded/

**Pros:**
- ✅ Official serde crate
- ✅ Minimal dependencies
- ✅ Simple and fast

**Cons:**
- ⚠️ No nested object support
- ⚠️ No array notation

### Option 3: Custom (current) ⚠️ HYBRID APPROACH
- **Status:** Using both custom + serde_qs
- **Issues:** Code duplication, inconsistency
- **Recommendation:** Consolidate to serde_qs

**Verdict:** Continue using `serde_qs`, remove custom parser

---

## Error Handling

### Option 1: `anyhow` ⭐ RECOMMENDED FOR APPLICATIONS
- **Stars:** 3,900+
- **Maturity:** Very stable
- **Author:** David Tolnay (Rust core team, creator of serde)
- **Features:**
  - Context chaining: `.context("description")`
  - Automatic conversion from any error
  - Good for application-level errors
  - Ergonomic error handling
- **Version:** 1.x (stable)
- **Use when:** Building applications (not libraries)
- **Docs:** https://docs.rs/anyhow/

**Pros:**
- ✅ Most ergonomic
- ✅ Great for quick iteration
- ✅ Excellent context support

**Cons:**
- ⚠️ Not type-safe (loses original error types)
- ⚠️ Not ideal for libraries

### Option 2: `eyre`
- **Stars:** 1,600+
- **Maturity:** Stable (fork of anyhow)
- **Features:**
  - Similar to anyhow
  - Customizable error reports
  - Better for debugging
  - Plugin system
- **Version:** 0.6
- **Use when:** Need customizable error reporting
- **Docs:** https://docs.rs/eyre/

**Pros:**
- ✅ More flexible than anyhow
- ✅ Better for complex applications

**Cons:**
- ⚠️ Slightly more complex
- ⚠️ Smaller ecosystem

### Option 3: `thiserror` - RECOMMENDED FOR LIBRARIES
- **Stars:** 4,800+
- **Maturity:** Very stable
- **Features:**
  - Derive macro for custom error types
  - Type-safe error handling
  - Good for libraries
  - Preserves error types
- **Version:** 2.x (latest) or 1.x
- **Use when:** Building libraries or need type-safe errors
- **Docs:** https://docs.rs/thiserror/

**Pros:**
- ✅ Type-safe
- ✅ Best for libraries
- ✅ Preserves error information

**Cons:**
- ⚠️ More boilerplate than anyhow
- ⚠️ Not as ergonomic for quick prototyping

### Option 4: `miette`
- **Stars:** 2,000+
- **Maturity:** Stable
- **Features:**
  - Fancy error reporting
  - Diagnostic-quality error messages
  - Source code snippets in errors
- **Use when:** Need beautiful CLI error output
- **Docs:** https://docs.rs/miette/

**Verdict:**
- **Applications:** Use `anyhow` (most ergonomic)
- **Libraries:** Use `thiserror` (type-safe, already using in core)
- **CLI tools:** Consider `miette` for fancy output

---

## Comparison Matrix

| Category | Option A (Modern) | Option B (Stable) | Current | Recommendation |
|----------|------------------|-------------------|---------|----------------|
| **Cookies** | `cookie` | `axum-extra` | Manual ❌ | `cookie` + `axum-extra` |
| **Date/Time** | `jiff` | `chrono` | Manual ❌ | `jiff` (new) / `chrono` (safe) |
| **Validation** | `garde` | `validator` | Manual ❌ | `garde` (new) / `validator` (safe) |
| **Query Parse** | `serde_qs` | `serde_urlencoded` | Custom ❌ | `serde_qs` ✅ |
| **Errors** | `anyhow` | `thiserror` | Mixed | `anyhow` + `thiserror` |
| **JSON Schema** | `jsonschema` | - | ✅ Using | Keep ✅ |
| **MIME** | `mime` | - | ✅ Using | Keep ✅ |

---

## Decision Guidelines

### When to choose "Modern" option:
- ✅ Starting new code
- ✅ Want best features
- ✅ Can tolerate minor API changes
- ✅ Active maintenance team

### When to choose "Stable" option:
- ✅ Need maximum stability
- ✅ Large production deployment
- ✅ Risk-averse environment
- ✅ Need wide ecosystem support

### When to keep custom code:
- ✅ Very domain-specific logic
- ✅ Performance-critical (and proven faster)
- ✅ No ecosystem solution exists
- ❌ **NOT** for common patterns like cookies, dates, validation

---

## Dependency Weight Analysis

### Lightweight (< 20 dependencies)
- `cookie` (7 deps)
- `serde_urlencoded` (3 deps)
- `anyhow` (0 deps!)
- `thiserror` (1 dep)

### Medium (20-50 dependencies)
- `validator` (~25 deps)
- `garde` (~30 deps)
- `serde_qs` (~15 deps)
- `mime` (~0 deps)

### Heavier (50+ dependencies)
- `chrono` (~35 deps base, more with timezones)
- `jiff` (~40 deps)
- `jsonschema` (~60 deps) ✅ Already using

**Note:** Spikard already uses Axum + Tower + PyO3, so baseline is ~150 dependencies. Adding these crates is a <10% increase for significant value.

---

## References

- **Rust crate stats:** https://lib.rs/
- **Crate comparison:** https://crates.io/
- **Ecosystem overview:** https://blessed.rs/
- **Security audits:** https://rustsec.org/
