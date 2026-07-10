//! Shared HTTP-test driver.
//!
//! Calls trait primitives on a [`TestClientRenderer`] in the canonical order
//! a TestClient-driven test takes:
//!
//! 1. `render_test_open` — doc, signature, opening brace, language-native skip annotation.
//! 2. `render_call` — `let response = client.METHOD(...)`.
//! 3. `render_assert_status` — status code assertion.
//! 4. `render_assert_header` (per header) — header assertions.
//! 5. `render_assert_json_body` / `render_assert_partial_body` — body assertion.
//! 6. `render_assert_validation_errors` — 422 validation errors, if present.
//! 7. `render_test_close` — closing brace / `end`.
//!
//! Steps 3-6 are skipped automatically when the corresponding expectation is empty.

use super::{CallCtx, TestClientRenderer, has_meaningful_body, is_skipped};
use alef::e2e::fixture::Fixture;

/// Default name for the response binding inside a generated test.
pub const DEFAULT_RESPONSE_VAR: &str = "response";

/// Render a single HTTP test for `fixture` to `out` using `renderer`.
///
/// Returns `true` if a test was emitted (the fixture has an `http` block),
/// `false` otherwise — caller is responsible for handling non-HTTP fixtures
/// (WebSocket, `AsyncAPI` spec validation, etc.) via different drivers.
pub fn render_http_test<R: TestClientRenderer + ?Sized>(out: &mut String, renderer: &R, fixture: &Fixture) -> bool {
    let Some(http) = fixture.http.as_ref() else {
        return false;
    };

    let fn_name = renderer.sanitize_test_name(&fixture.id);

    let skip_reason = if is_skipped(fixture, renderer.language_name()) {
        Some(
            fixture
                .skip
                .as_ref()
                .and_then(|s| s.reason.as_deref())
                .unwrap_or("skipped"),
        )
    } else {
        None
    };

    renderer.render_test_open(out, &fn_name, &fixture.description, skip_reason);

    if skip_reason.is_some() {
        renderer.render_test_close(out);
        return true;
    }

    let response_var = DEFAULT_RESPONSE_VAR;
    let request_path = &http.request.path;
    let namespaced_path = format!("/fixtures/{}{}", fixture.id, request_path);
    let req = &http.request;
    let ctx = CallCtx {
        method: req.method.as_str(),
        path: &namespaced_path,
        headers: &req.headers,
        query_params: &req.query_params,
        cookies: &req.cookies,
        body: req.body.as_ref(),
        content_type: req.content_type.as_deref(),
        response_var,
    };
    renderer.render_call(out, &ctx);

    renderer.render_assert_status(out, response_var, http.expected_response.status_code);

    let mut header_names: Vec<&String> = http.expected_response.headers.keys().collect();
    header_names.sort();
    for name in header_names {
        let value = &http.expected_response.headers[name];
        if name.eq_ignore_ascii_case("content-encoding") {
            continue;
        }
        renderer.render_assert_header(out, response_var, name, value);
    }

    if has_meaningful_body(&http.expected_response)
        && let Some(body) = http.expected_response.body.as_ref()
    {
        renderer.render_assert_json_body(out, response_var, body);
    }

    if let Some(partial) = http.expected_response.body_partial.as_ref() {
        renderer.render_assert_partial_body(out, response_var, partial);
    }

    if let Some(errors) = http.expected_response.validation_errors.as_ref()
        && !errors.is_empty()
    {
        renderer.render_assert_validation_errors(out, response_var, errors);
    }

    renderer.render_test_close(out);
    true
}

#[cfg(test)]
mod tests {
    use super::super::{CallCtx, TestClientRenderer};
    use super::render_http_test;
    use alef::e2e::fixture::{Fixture, HttpExpectedResponse, HttpFixture, HttpRequest, ValidationErrorExpectation};
    use std::collections::BTreeMap;

    /// Mock renderer that records every call as a tag in `out`. Lets us assert
    /// the exact sequence of trait calls the shared driver makes for each
    /// expected-response shape.
    struct TagRenderer;

    impl TestClientRenderer for TagRenderer {
        fn language_name(&self) -> &'static str {
            "mock"
        }
        fn render_test_open(&self, out: &mut String, fn_name: &str, _: &str, skip: Option<&str>) {
            let skip_marker = skip.map(|r| format!("|skip={r}")).unwrap_or_default();
            out.push_str(&format!("OPEN({fn_name}{skip_marker})\n"));
        }
        fn render_test_close(&self, out: &mut String) {
            out.push_str("CLOSE\n");
        }
        fn render_call(&self, out: &mut String, ctx: &CallCtx<'_>) {
            out.push_str(&format!("CALL({} {} -> {})\n", ctx.method, ctx.path, ctx.response_var));
        }
        fn render_assert_status(&self, out: &mut String, _: &str, status: u16) {
            out.push_str(&format!("STATUS={status}\n"));
        }
        fn render_assert_header(&self, out: &mut String, _: &str, name: &str, value: &str) {
            out.push_str(&format!("HEADER({name}={value})\n"));
        }
        fn render_assert_json_body(&self, out: &mut String, _: &str, expected: &serde_json::Value) {
            out.push_str(&format!("JSON_BODY({expected})\n"));
        }
        fn render_assert_partial_body(&self, out: &mut String, _: &str, expected: &serde_json::Value) {
            out.push_str(&format!("PARTIAL_BODY({expected})\n"));
        }
        fn render_assert_validation_errors(&self, out: &mut String, _: &str, errors: &[ValidationErrorExpectation]) {
            out.push_str(&format!("VALIDATION({})\n", errors.len()));
        }
    }

    fn http_fixture(id: &str, expected: HttpExpectedResponse) -> Fixture {
        Fixture {
            id: id.into(),
            description: "test".into(),
            category: Some("smoke".into()),
            tags: vec![],
            skip: None,
            env: None,
            setup: Vec::new(),
            call: None,
            input: serde_json::Value::Null,
            mock_response: None,
            visitor: None,
            args: vec![],
            assertion_recipes: vec![],
            assertions: vec![],
            source: String::new(),
            http: Some(HttpFixture {
                handler: alef::e2e::fixture::HttpHandler {
                    route: String::new(),
                    method: "GET".into(),
                    body_schema: None,
                    parameters: BTreeMap::new(),
                    middleware: None,
                },
                request: HttpRequest {
                    method: "GET".into(),
                    path: String::new(),
                    headers: BTreeMap::new(),
                    query_params: BTreeMap::new(),
                    cookies: BTreeMap::new(),
                    body: None,
                    form_data: None,
                    content_type: None,
                },
                expected_response: expected,
            }),
        }
    }

    fn empty_expected(status: u16) -> HttpExpectedResponse {
        HttpExpectedResponse {
            status_code: status,
            body: None,
            body_partial: None,
            headers: BTreeMap::new(),
            validation_errors: None,
        }
    }

    #[test]
    fn driver_emits_open_call_status_close_in_order() {
        let fixture = http_fixture("simple", empty_expected(200));
        let mut out = String::new();
        let emitted = render_http_test(&mut out, &TagRenderer, &fixture);
        assert!(emitted);
        assert_eq!(
            out,
            "OPEN(simple)\nCALL(GET /fixtures/simple -> response)\nSTATUS=200\nCLOSE\n"
        );
    }

    #[test]
    fn driver_skips_when_no_http_block() {
        let mut fixture = http_fixture("noop", empty_expected(200));
        fixture.http = None;
        let mut out = String::new();
        let emitted = render_http_test(&mut out, &TagRenderer, &fixture);
        assert!(!emitted);
        assert!(out.is_empty());
    }

    #[test]
    fn driver_emits_skip_marker_and_short_circuits_assertions() {
        let mut fixture = http_fixture("skipme", empty_expected(200));
        fixture.skip = Some(alef::e2e::fixture::SkipDirective {
            languages: vec!["mock".into()],
            reason: Some("not yet".into()),
        });
        let mut out = String::new();
        render_http_test(&mut out, &TagRenderer, &fixture);
        assert!(out.contains("OPEN(skipme|skip=not yet)"));
        assert!(out.contains("CLOSE"));
        assert!(!out.contains("CALL"));
        assert!(!out.contains("STATUS"));
    }

    #[test]
    fn driver_strips_content_encoding_header_assertion() {
        let mut expected = empty_expected(200);
        expected.headers.insert("Content-Encoding".into(), "gzip".into());
        expected.headers.insert("X-Foo".into(), "bar".into());
        let fixture = http_fixture("hdr", expected);
        let mut out = String::new();
        render_http_test(&mut out, &TagRenderer, &fixture);
        assert!(!out.contains("HEADER(Content-Encoding"));
        assert!(out.contains("HEADER(X-Foo=bar)"));
    }

    #[test]
    fn driver_emits_headers_in_sorted_order() {
        let mut expected = empty_expected(200);
        expected.headers.insert("Z-Header".into(), "z".into());
        expected.headers.insert("A-Header".into(), "a".into());
        expected.headers.insert("M-Header".into(), "m".into());
        let fixture = http_fixture("hdr", expected);
        let mut out = String::new();
        render_http_test(&mut out, &TagRenderer, &fixture);
        let a_pos = out.find("HEADER(A-Header").unwrap();
        let m_pos = out.find("HEADER(M-Header").unwrap();
        let z_pos = out.find("HEADER(Z-Header").unwrap();
        assert!(a_pos < m_pos);
        assert!(m_pos < z_pos);
    }

    #[test]
    fn driver_skips_body_assert_for_null_and_empty_string_sentinels() {
        let mut expected = empty_expected(200);
        expected.body = Some(serde_json::Value::Null);
        let fixture = http_fixture("nullbody", expected);
        let mut out = String::new();
        render_http_test(&mut out, &TagRenderer, &fixture);
        assert!(!out.contains("JSON_BODY"));

        let mut expected = empty_expected(200);
        expected.body = Some(serde_json::Value::String(String::new()));
        let fixture = http_fixture("emptybody", expected);
        let mut out = String::new();
        render_http_test(&mut out, &TagRenderer, &fixture);
        assert!(!out.contains("JSON_BODY"));
    }

    #[test]
    fn driver_emits_body_partial_assertion_independently_of_body() {
        let mut expected = empty_expected(200);
        expected.body_partial = Some(serde_json::json!({"k": "v"}));
        let fixture = http_fixture("partial", expected);
        let mut out = String::new();
        render_http_test(&mut out, &TagRenderer, &fixture);
        assert!(out.contains("PARTIAL_BODY"));
    }

    #[test]
    fn driver_emits_validation_errors_assertion_when_present_and_nonempty() {
        let mut expected = empty_expected(422);
        expected.validation_errors = Some(vec![ValidationErrorExpectation {
            loc: vec!["name".into()],
            msg: "field required".into(),
            error_type: "missing".into(),
        }]);
        let fixture = http_fixture("ve", expected);
        let mut out = String::new();
        render_http_test(&mut out, &TagRenderer, &fixture);
        assert!(out.contains("VALIDATION(1)"));

        let mut expected = empty_expected(422);
        expected.validation_errors = Some(vec![]);
        let fixture = http_fixture("ve_empty", expected);
        let mut out = String::new();
        render_http_test(&mut out, &TagRenderer, &fixture);
        assert!(!out.contains("VALIDATION"));
    }
}
