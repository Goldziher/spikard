//! Shared HTTP/WebSocket test-codegen abstractions.
//!
//! Per-language e2e codegen (`crates/alef-e2e/src/codegen/<lang>.rs`) was previously
//! a monolithic ~1k-2k-line file per language that duplicated the structural shape
//! of every test (function header, request build, response assert) and only
//! differed in syntax. This module pulls the common shape into a trait + driver
//! pair so each language file becomes thin: implement primitives, delegate to
//! [`http_call::render_http_test`] / [`ws_script::render_ws_test`].
//!
//! The trait targets the **TestClient-driven** test shape — i.e. tests call
//! `client.METHOD(path, body, headers)` against a `TestClient` exposed by the
//! language binding, rather than spinning up a TCP mock server. Languages that
//! cannot expose `TestClient` over FFI (Go/Java/C#) implement the same trait but
//! emit code that spawns the binding's `App.serve()` on a loopback port and
//! drives it with their stdlib HTTP client.

use alef::e2e::fixture::{Fixture, HttpExpectedResponse, HttpRequest, ValidationErrorExpectation};
use std::collections::BTreeMap;

pub mod http_call;
pub mod ws_script;

/// Context for rendering a single `TestClient` HTTP call.
///
/// `response_var` is the binding-side identifier the renderer should use when
/// emitting subsequent assertions (e.g. `response`, `_resp`, `let response =`).
#[derive(Debug)]
pub struct CallCtx<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub headers: &'a BTreeMap<String, String>,
    pub query_params: &'a BTreeMap<String, serde_json::Value>,
    pub cookies: &'a BTreeMap<String, String>,
    pub body: Option<&'a serde_json::Value>,
    pub content_type: Option<&'a str>,
    pub response_var: &'a str,
}

impl<'a> CallCtx<'a> {
    #[must_use]
    pub fn from_request(req: &'a HttpRequest, response_var: &'a str) -> Self {
        Self {
            method: req.method.as_str(),
            path: req.path.as_str(),
            headers: &req.headers,
            query_params: &req.query_params,
            cookies: &req.cookies,
            body: req.body.as_ref(),
            content_type: req.content_type.as_deref(),
            response_var,
        }
    }
}

/// Per-language `TestClient` test renderer.
///
/// Implementations live alongside the per-language codegen module
/// (`crates/alef-e2e/src/codegen/<lang>/client.rs`). The shared driver
/// [`http_call::render_http_test`] calls these primitives in order to assemble
/// a complete test. Methods append to `out`; they MUST NOT clear or seek it.
///
/// Most methods take a `response_var: &str` argument so the renderer can
/// reference the value bound by `render_call`. Default value: `"response"`.
pub trait TestClientRenderer {
    /// Identifier used in fixture skip directives (e.g. `"python"`, `"node"`).
    fn language_name(&self) -> &'static str;

    /// Convert a fixture id (`my_test_id`) to a language-valid identifier.
    /// Default implementation lower-cases and replaces non-alphanumerics with `_`.
    fn sanitize_test_name(&self, id: &str) -> String {
        alef::e2e::escape::sanitize_ident(id)
    }

    /// Render the test-function opening: doc, signature, opening brace.
    /// `skip_reason: Some(...)` means the fixture is skipped for this language;
    /// the renderer should emit the language-native skip annotation.
    fn render_test_open(&self, out: &mut String, fn_name: &str, description: &str, skip_reason: Option<&str>);

    /// Render the test-function closing brace / `end` / etc.
    fn render_test_close(&self, out: &mut String);

    /// Render `let <response_var> = client.METHOD(path, body, query, headers, cookies)`
    /// (or per-language equivalent). Including the trailing newline.
    fn render_call(&self, out: &mut String, ctx: &CallCtx<'_>);

    /// Render `assert <response_var>.status == status`.
    fn render_assert_status(&self, out: &mut String, response_var: &str, status: u16);

    /// Render `assert <response_var>.headers[name] == expected`.
    /// `expected` may be a literal value or one of the special tokens `<<uuid>>`,
    /// `<<present>>`, `<<absent>>` per the fixture schema; the renderer is
    /// responsible for handling those.
    fn render_assert_header(&self, out: &mut String, response_var: &str, name: &str, expected: &str);

    /// Render an exact-equality JSON body assertion. The renderer is responsible
    /// for parsing the response body as JSON (or the appropriate language-native
    /// equivalent) and comparing it to `expected`.
    fn render_assert_json_body(&self, out: &mut String, response_var: &str, expected: &serde_json::Value);

    /// Render a partial JSON body assertion: every field in `expected` must be
    /// present in the response with the same value, but the response may have
    /// additional fields.
    fn render_assert_partial_body(&self, out: &mut String, response_var: &str, expected: &serde_json::Value);

    /// Render a validation-errors assertion for 422 responses.
    fn render_assert_validation_errors(
        &self,
        out: &mut String,
        response_var: &str,
        errors: &[ValidationErrorExpectation],
    );
}

/// Whether a fixture is skipped for the given language.
///
/// Pulled into the shared driver layer so individual renderers don't reimplement.
#[must_use]
pub fn is_skipped(fixture: &Fixture, language: &str) -> bool {
    fixture
        .skip
        .as_ref()
        .is_some_and(|s| s.languages.iter().any(|l| l == language))
}

/// Whether the expected-response carries any header expectations beyond
/// content-encoding (which the mock layer strips and is therefore not asserted).
#[must_use]
pub fn has_meaningful_headers(expected: &HttpExpectedResponse) -> bool {
    expected
        .headers
        .iter()
        .any(|(k, _)| !k.eq_ignore_ascii_case("content-encoding"))
}

/// Whether the expected-response carries a non-empty body.
///
/// The fixture schema uses `null` and `""` as "no body" sentinels — neither
/// triggers a body assertion.
#[must_use]
pub fn has_meaningful_body(expected: &HttpExpectedResponse) -> bool {
    match &expected.body {
        Some(v) if v.is_null() => false,
        Some(v) if v.as_str() == Some("") => false,
        Some(_) => true,
        None => false,
    }
}
