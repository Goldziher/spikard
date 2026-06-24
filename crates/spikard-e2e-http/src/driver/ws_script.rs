//! Shared WebSocket-test driver.
//!
//! Drives a [`WebSocketScriptRenderer`] through the canonical sequence a scripted
//! WebSocket fixture takes:
//!
//! 1. `render_test_open` — language-native test header.
//! 2. `render_connect` — open WS connection (`let conn = client.connect_websocket(path)`).
//! 3. For each scripted message:
//!    - `direction = send` → `render_send_text` / `render_send_json` / `render_send_binary`.
//!    - `direction = receive` → `render_expect_text` / `render_expect_json` / `render_expect_binary`.
//! 4. `render_close` — close the connection with the expected close code.
//! 5. `render_test_close` — closing brace / `end`.
//!
//! The fixture schema for WebSocket scripts lives in `fixtures/websocket*.json` —
//! see the SSE/WS subset in `alef::e2e::fixture::HttpFixture` (currently the WS
//! schema is loaded as raw JSON; a typed schema will be introduced in a follow-up
//! once the language renderers are in place and we know exactly which fields
//! they consume).

use alef::e2e::fixture::Fixture;

/// Per-language WebSocket script renderer.
///
/// Implementations live next to the per-language codegen module
/// (`crates/alef-e2e/src/codegen/<lang>/ws.rs`).
///
/// This is currently a stub: the trait methods will be filled in alongside
/// Phase 2B/2C of the e2e flip, when each language's WebSocket assertions are
/// migrated to drive the binding's `TestClient.connect_websocket()` API.
pub trait WebSocketScriptRenderer {
    /// Identifier used in fixture skip directives (e.g. `"python"`).
    fn language_name(&self) -> &'static str;
}

/// Render a WebSocket script test for `fixture` to `out` using `renderer`.
///
/// Currently returns `false` (no test emitted) because the WS schema and
/// per-language renderers are not yet wired up; per-language codegen still
/// uses its monolithic WebSocket renderer until Phase 2B lands. This function
/// exists as the integration point so the per-language modules can begin
/// migrating one at a time.
pub const fn render_ws_test<R: WebSocketScriptRenderer + ?Sized>(
    _out: &mut String,
    _renderer: &R,
    _fixture: &Fixture,
) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::{WebSocketScriptRenderer, render_ws_test};
    use alef::e2e::fixture::Fixture;

    struct StubRenderer;
    impl WebSocketScriptRenderer for StubRenderer {
        fn language_name(&self) -> &'static str {
            "stub"
        }
    }

    #[test]
    fn stub_driver_emits_no_test() {
        let fixture = Fixture {
            id: "ws_stub".into(),
            description: "stub".into(),
            category: None,
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
            http: None,
        };
        let mut out = String::new();
        let emitted = render_ws_test(&mut out, &StubRenderer, &fixture);
        assert!(!emitted);
        assert!(out.is_empty());
    }
}
