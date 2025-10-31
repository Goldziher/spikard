# Ecosystem Audit - Quick Reference

**Full Report:** [03-ecosystem-audit-2025.md](./03-ecosystem-audit-2025.md)

---

## 🔴 Critical Issues (Fix First)

### 1. Cookie Parsing - SECURITY ISSUE
**Problem:** Manual cookie parsing is RFC-violating and insecure
- No percent-decoding
- No quoted value support
- Missing edge case handling

**Fix:** Add `cookie = "0.18"` and use `Cookie::split_parse_encoded()`
**Lines saved:** ~20 | **Difficulty:** Easy

---

### 2. Date/Time Validation - CORRECTNESS ISSUE
**Problem:** Custom validation doesn't actually parse dates
- Accepts invalid dates like "2023-02-30"
- Duration validation is a no-op
- No timezone validation

**Fix:** Add `jiff = "0.1"` or `chrono = "0.4"`
**Lines saved:** ~120 | **Difficulty:** Easy

---

### 3. Validation Infrastructure - ARCHITECTURE ISSUE
**Problem:** No declarative validation, scattered logic
- Can't validate at extraction time
- Verbose error mapping
- Not reusable

**Fix:** Add `garde = "0.20"` or `validator = "0.19"`
**Lines saved:** ~300 | **Difficulty:** Medium

---

### 4. Cookie Handling - ARCHITECTURE IMPROVEMENT
**Problem:** Manual extraction in every handler

**Fix:** Add `axum-extra = { version = "0.9", features = ["cookie"] }`
**Lines saved:** ~50 | **Difficulty:** Medium

---

## 🟡 Medium Priority

### 5. Query String Parsing
**Fix:** Use `serde_qs` consistently (already partially using)
**Lines saved:** ~200 | **Difficulty:** Easy

### 6. HTTP Headers
**Fix:** Use `axum-extra` `TypedHeader` extractors
**Lines saved:** ~30 | **Difficulty:** Medium

### 7. Error Context
**Fix:** Add `anyhow = "1.0"` for better error chains
**Lines saved:** ~50 | **Difficulty:** Medium

### 8. Middleware Collection
**Fix:** Use `tower-http` features (already have trace)
**Lines saved:** Varies | **Difficulty:** Easy

---

## 🟢 Already Good

✅ JSON Schema validation (using `jsonschema` correctly)
✅ Content-Type parsing (using `mime` correctly)
✅ Regex compilation (using `lazy_static`)
✅ Query parsing with brackets (using `serde_qs`)
✅ HTTP framework (Axum + Tower)

**Enhancement:** Add `with_regex_engine(RegexEngine::Regex)` to jsonschema for ReDoS protection

---

## Quick Wins (< 1 day each)

1. **Cookie parsing** - Add crate, replace function ✅
2. **Date/time validation** - Add crate, replace functions ✅
3. **Query string consolidation** - Remove custom parser ✅
4. **ReDoS protection** - Add one config line ✅

---

## Dependencies to Add

```toml
# Critical (add immediately)
cookie = "0.18"
jiff = "0.1"  # Or chrono = "0.4"
garde = "0.20"  # Or validator = "0.19"

# Important (add soon)
axum-extra = { version = "0.9", features = ["cookie", "typed-header"] }
anyhow = "1.0"

# Nice to have
tower-http = { version = "0.6", features = [
    "trace",  # Already have
    "compression-gzip",
    "timeout",
    "limit",
    "cors",
] }
```

---

## Impact Summary

**Total lines of custom code to remove:** ~770
**Security issues fixed:** 3
**Correctness issues fixed:** 2
**Maintenance burden reduced:** High

---

## Migration Phases

**Phase 1 (Week 1-2):** Security & Correctness
- Cookie parsing ✅
- Date/time validation ✅
- ReDoS protection ✅

**Phase 2 (Week 3-4):** Validation Infrastructure
- Add garde/validator ✅
- Refactor handlers ✅

**Phase 3 (Week 5-6):** Cleanup
- Query string consolidation ✅
- Cookie extractors ✅
- Error handling ✅

---

## Most Important Fixes

1. 🔴 **Cookie parsing** - Security risk, RFC violation
2. 🔴 **Date validation** - Accepts invalid dates
3. 🔴 **Validation infra** - Reduces 300+ lines, better patterns
4. 🟡 **Query parsing** - Removes 200 lines of duplicate code

**Start here:** Fix #1 and #2 (both are easy, high impact)
