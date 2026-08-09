//! Every per-route middleware payload in the canonical fixture corpus must deserialize.
//!
//! Four separate config types were merged that parsed real fixture data into a config carrying no
//! constraint at all -- a plural field name against a singular payload, a `Vec<String>` against an
//! array of objects, an `Option<bool>` against `{"enabled": false}`, and an `enabled` flag declared
//! nowhere. Each one type-checked, each one round-tripped invented test JSON, and each one would
//! have silently disabled the middleware it configured. Sampling shapes by hand is what let them
//! through, so this walks the whole corpus instead. ~keep

use serde_json::Value;
use spikard_core::http::{
    ApiKeyAuthConfig, AuthorizationConfig, CompressionConfig, CorsConfig, JwtAuthConfig, LifecycleHooksConfig,
    RequestIdConfig,
};
use std::path::{Path, PathBuf};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

/// Every `http.handler.middleware` payload in the corpus, as (fixture file, entry id, key, value).
fn middleware_payloads() -> Vec<(String, String, String, Value)> {
    let mut found = Vec::new();
    let entries = std::fs::read_dir(fixtures_dir()).expect("fixtures/ directory must exist");

    for entry in entries {
        let path = entry.expect("readable dir entry").path();
        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string();
        let raw = std::fs::read_to_string(&path).expect("fixture file must be readable");
        let Ok(Value::Array(items)) = serde_json::from_str::<Value>(&raw) else {
            continue;
        };

        for item in items {
            let id = item.get("id").and_then(Value::as_str).unwrap_or("<no id>").to_string();
            let Some(Value::Object(middleware)) = item.pointer("/http/handler/middleware") else {
                continue;
            };
            for (key, value) in middleware {
                found.push((file_name.clone(), id.clone(), key.clone(), value.clone()));
            }
        }
    }
    found
}

/// Deserialize `payload` into `T`, returning a failure description rather than panicking, so one
/// run reports every offending fixture instead of only the first. ~keep
fn try_parse<T: serde::de::DeserializeOwned>(file: &str, id: &str, key: &str, payload: &Value) -> Option<String> {
    serde_json::from_value::<T>(payload.clone())
        .err()
        .map(|error| format!("  {file}:{id} [{key}] -> {error}\n    payload: {payload}"))
}

/// Fixtures known to carry keys no config type accepts. Each is a defect in the canonical tree,
/// not in the Rust types, and each needs a fixture edit plus a regeneration to fix -- so they are
/// listed rather than silently skipped. Deleting an entry here after fixing its fixture is the
/// point: an unexpected pass fails just as loudly as an unexpected failure. ~keep
///
/// `cors.json` uses `allow_origins`/`allow_methods`/`allow_headers`, but both `CorsConfig` and the
/// whole of `testing_data/cors/` use the `allowed_` prefix. All four canonical per-route CORS
/// payloads are therefore unparseable today.
const KNOWN_BAD_FIXTURES: &[(&str, &str)] = &[
    ("cors.json", "06_cors_preflight_method_not_allowed"),
    ("cors.json", "07_cors_preflight_header_not_allowed"),
    ("cors.json", "08_cors_max_age"),
    ("cors.json", "cors_custom_allowed_headers_x_custom"),
];

#[test]
fn should_deserialize_every_per_route_middleware_payload_in_the_fixture_corpus() {
    let payloads = middleware_payloads();
    assert!(
        !payloads.is_empty(),
        "found no http.handler.middleware payloads -- the fixture layout changed, and this test is \
         now asserting nothing"
    );

    let mut failures = Vec::new();
    let mut unexpectedly_fixed = Vec::new();
    let mut checked = 0_usize;

    for (file, id, key, payload) in &payloads {
        let failure = match key.as_str() {
            "jwt_auth" => try_parse::<JwtAuthConfig>(file, id, key, payload),
            "api_key_auth" => try_parse::<ApiKeyAuthConfig>(file, id, key, payload),
            "authorization" => try_parse::<AuthorizationConfig>(file, id, key, payload),
            "lifecycle_hooks" => try_parse::<LifecycleHooksConfig>(file, id, key, payload),
            "request_id" => try_parse::<RequestIdConfig>(file, id, key, payload),
            "compression" => try_parse::<CompressionConfig>(file, id, key, payload),
            "cors" => try_parse::<CorsConfig>(file, id, key, payload),
            // graphql, openrpc, background_tasks, websocket, rate_limit, body_limit,
            // request_timeout and static_files have no per-route config type in spikard-core yet.
            // They are deliberately skipped rather than silently passed. ~keep
            _ => {
                continue;
            }
        };
        checked += 1;
        let known_bad = KNOWN_BAD_FIXTURES
            .iter()
            .any(|(bad_file, bad_id)| bad_file == file && bad_id == id);

        match (failure, known_bad) {
            (Some(text), false) => failures.push(text),
            (None, true) => unexpectedly_fixed.push(format!("  {file}:{id} [{key}]")),
            _ => {}
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {checked} per-route middleware payloads failed to deserialize:\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert!(
        unexpectedly_fixed.is_empty(),
        "{} fixture(s) listed in KNOWN_BAD_FIXTURES now parse -- delete them from that list:\n{}",
        unexpectedly_fixed.len(),
        unexpectedly_fixed.join("\n")
    );
}
