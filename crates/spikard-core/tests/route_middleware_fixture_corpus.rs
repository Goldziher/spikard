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
    RateLimitConfig, RequestIdConfig,
};
use spikard_core::{Route, RouteMetadata, SchemaRegistry};
use std::path::{Path, PathBuf};

/// Mirror of `spikard_http::background::BackgroundTaskConfig`'s field names.
///
/// `spikard-core` cannot depend on `spikard-http` (the dependency runs the other way, same as
/// `JwtAuthConfig`/`ApiKeyAuthConfig` above), and there is no per-route field on `RouteMetadata`
/// for `background_tasks` yet -- unlike `jwt_auth` or `cors`, the real config lives only on
/// `ServerConfig.background_tasks` (server-wide). This type exists solely so the fixture corpus
/// can be checked against the *real* field names (`enabled`, `max_queue_size`,
/// `max_concurrent_tasks`, `drain_timeout_secs` -- see `crates/spikard-http/src/background.rs`)
/// instead of drifting unnoticed the way `max_concurrent`/`timeout_seconds`/`retry_policy` did:
/// none of those three names exist on the real struct, so they parsed as no-op unknown fields and
/// silently defaulted every value they claimed to configure. `deny_unknown_fields` turns that
/// silent default into a loud parse failure. If `BackgroundTaskConfig` gains real per-route wiring
/// in `spikard-core`, this mirror should be deleted in favor of the genuine type. ~keep
// Fields are asserted only by shape (via `deny_unknown_fields`), never read individually -- the
// point of this type is rejecting unrecognized keys, not inspecting accepted ones. ~keep
#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BackgroundTaskConfigShape {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    max_queue_size: Option<usize>,
    #[serde(default)]
    max_concurrent_tasks: Option<usize>,
    #[serde(default)]
    drain_timeout_secs: Option<u64>,
}

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
/// Empty is the goal state, not an oversight. The four `cors.json` entries that used to live here
/// spelled their keys `allow_origins`/`allow_methods`/`allow_headers` while both `CorsConfig` and
/// the whole of `testing_data/cors/` use the `allowed_` prefix, so no canonical per-route CORS
/// payload had ever deserialized. The fixtures were corrected rather than the types, because
/// `allow_credentials` genuinely is `allow_`-prefixed and the `allowed_` collection keys are the
/// established spelling everywhere else.
const KNOWN_BAD_FIXTURES: &[(&str, &str)] = &[];

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
            "rate_limit" => try_parse::<RateLimitConfig>(file, id, key, payload),
            "background_tasks" => try_parse::<BackgroundTaskConfigShape>(file, id, key, payload),
            // graphql, websocket, body_limit, request_timeout and static_files have no per-route
            // config type in spikard-core yet. openrpc is checked separately below (its shape is
            // `{enabled, spec}`, not a struct matching a `RouteMetadata` field name, so it needs a
            // different assertion than "does this deserialize"). These keys are deliberately
            // skipped here rather than silently passed. ~keep
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

/// Parsing successfully is not the same as carrying a constraint.
///
/// `ApiKeyAuthConfig` has no `deny_unknown_fields` and defaults `keys` to empty, so a payload
/// that misspells `header_name` as `header` deserializes cleanly into a config that requires no
/// key at all -- exactly the shape `fixtures/server_config.json` shipped until it was corrected.
/// The deserialization test above cannot catch that, because nothing failed. This asserts the
/// property the corpus actually needs to hold: an enabled key check must have a key to check
/// against. ~keep
#[test]
fn should_not_have_an_enabled_api_key_auth_fixture_with_no_keys() {
    let offenders: Vec<String> = middleware_payloads()
        .iter()
        .filter(|(_, _, key, _)| key == "api_key_auth")
        .filter_map(|(file, id, _, payload)| {
            let config = serde_json::from_value::<ApiKeyAuthConfig>(payload.clone()).ok()?;
            (config.enabled && config.keys.is_empty())
                .then(|| format!("  {file}:{id} -> enabled with no keys\n    payload: {payload}"))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "{} api_key_auth fixture(s) enable key authentication while configuring no key, which \
         cannot reject anything:\n{}",
        offenders.len(),
        offenders.join("\n")
    );
}

/// The literal `OpenRPC` document a route supplies survives at `RouteMetadata.openrpc_spec` and
/// through `Route::from_metadata` unchanged.
///
/// `RouteMetadata.openrpc_spec` is `Option<Value>`, not a struct -- there is no per-route type to
/// check the fixture's `{enabled, spec}` shape against the way `try_parse` does for `jwt_auth`
/// etc, because the `enabled` flag and the `spec` document are not deserialized together: only
/// `spec` ever reaches `RouteMetadata`, and only when a route actually supplies one. What CAN be
/// verified with only `spikard-core` types is the mechanism
/// `crates/spikard-http/src/server/mod.rs`'s `route_supplied_openrpc_spec` depends on: that the
/// document carried on `RouteMetadata.openrpc_spec` reaches `Route.openrpc_spec` byte-for-byte.
/// Exercising every literal document actually present in `fixtures/openrpc.json` (real `$ref`
/// schemas, nested method arrays, numeric literals) catches corruption that a single hand-written
/// example in `router.rs`'s own unit test cannot. ~keep
#[test]
fn should_carry_every_route_supplied_openrpc_spec_through_route_from_metadata() {
    let registry = SchemaRegistry::new();
    let mut checked = 0_usize;

    for (file, id, key, payload) in middleware_payloads() {
        if key != "openrpc" {
            continue;
        }
        let enabled = payload.get("enabled").and_then(Value::as_bool).unwrap_or(false);
        if !enabled {
            continue;
        }
        let spec = payload
            .get("spec")
            .unwrap_or_else(|| panic!("{file}:{id} [openrpc] enabled with no spec document"))
            .clone();

        let metadata = RouteMetadata {
            path: format!("/{id}"),
            handler_name: id.clone(),
            openrpc_spec: Some(spec.clone()),
            ..Default::default()
        };

        let route = Route::from_metadata(metadata, &registry)
            .unwrap_or_else(|error| panic!("{file}:{id} [openrpc] failed to build route: {error}"));

        assert_eq!(
            route.openrpc_spec,
            Some(spec),
            "{file}:{id} [openrpc] spec document was not carried unchanged onto Route.openrpc_spec"
        );
        checked += 1;
    }

    assert!(
        checked > 0,
        "found no enabled openrpc fixture entries -- the fixture layout changed, and this test is \
         now asserting nothing"
    );
}
