//! Testing utilities for Spikard Elixir bindings.
//!
//! This module provides a TestClient that wraps the core spikard-http test client
//! and exposes it to Elixir via NIFs. It enables testing HTTP handlers without
//! actual network overhead.
//!
//! The implementation follows the PHP binding pattern of direct router dispatch
//! rather than spawning a test server, which is simpler and sufficient for
//! Elixir's use case.

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Method, Request, Uri};
use axum::Router;
use http_body_util::BodyExt;
use rustler::{Encoder, Env, LocalPid, MapIterator, NifResult, ResourceArc, Term};
use serde_json::Value as JsonValue;
use spikard_http::testing::{ResponseSnapshot, SnapshotError};
use spikard_http::{Handler, Route, RouteMetadata, SchemaRegistry, Server, ServerConfig};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

use crate::atoms;
use crate::conversion::{elixir_to_json, json_to_elixir};
use crate::error::struct_error;
use crate::handler::ElixirHandler;
use crate::lifecycle::{create_lifecycle_hooks, HookCounts};
use crate::server::GLOBAL_RUNTIME;

/// Inner state of a test client.
struct TestClientInner {
    /// The Axum router for dispatching requests
    router: Router,
    /// Handler runner PID for Elixir callbacks
    #[allow(dead_code)]
    handler_runner_pid: LocalPid,
}

/// Resource wrapper for the test client.
pub struct TestClientResource {
    inner: Mutex<Option<TestClientInner>>,
}

impl TestClientResource {
    fn new(router: Router, handler_runner_pid: LocalPid) -> Self {
        Self {
            inner: Mutex::new(Some(TestClientInner {
                router,
                handler_runner_pid,
            })),
        }
    }
}

/// Register the TestClient resource type with Rustler.
pub fn on_load(env: Env) -> bool {
    #[allow(non_local_definitions)]
    let _ = rustler::resource!(TestClientResource, env);
    true
}

/// Create a new test client from routes configuration.
///
/// # Arguments
///
/// * `env` - Elixir environment
/// * `routes_json` - JSON string containing route metadata
/// * `handler_runner_pid` - PID of the HandlerRunner GenServer
/// * `config_map` - Optional server configuration
///
/// # Returns
///
/// `{:ok, client_ref}` or `{:error, reason}`
#[rustler::nif(schedule = "DirtyCpu")]
pub fn test_client_new<'a>(
    env: Env<'a>,
    routes_json: String,
    handler_runner_pid: LocalPid,
    config_map: Term<'a>,
) -> NifResult<Term<'a>> {
    // Parse route metadata from JSON
    let metadata: Vec<RouteMetadata> = match serde_json::from_str(&routes_json) {
        Ok(meta) => meta,
        Err(err) => {
            let error_msg = format!("Failed to parse routes JSON: {}", err);
            return Ok(struct_error(env, atoms::invalid_routes_json(), &error_msg));
        }
    };

    if metadata.is_empty() {
        return Ok(struct_error(env, atoms::no_routes(), "No routes provided"));
    }

    // Extract server config and lifecycle hook counts
    let (mut config, hook_counts) = extract_test_config(config_map);

    // Create lifecycle hooks if any are configured
    if !hook_counts.is_empty() {
        let hooks = create_lifecycle_hooks(handler_runner_pid, hook_counts);
        config.lifecycle_hooks = Some(Arc::new(hooks));
    }

    // Create schema registry
    let schema_registry = SchemaRegistry::new();

    // Create routes with Elixir handlers
    let mut routes_with_handlers: Vec<(Route, Arc<dyn Handler>)> = Vec::new();

    for route_meta in metadata {
        let route = match Route::from_metadata(route_meta.clone(), &schema_registry) {
            Ok(r) => r,
            Err(e) => {
                let error_msg = format!("Failed to create route: {}", e);
                return Ok(struct_error(env, atoms::route_creation_failed(), &error_msg));
            }
        };

        let elixir_handler = match ElixirHandler::new(&route, handler_runner_pid) {
            Ok(h) => h,
            Err(e) => {
                let error_msg = format!("Failed to create handler for {}: {}", route_meta.path, e);
                return Ok(struct_error(env, atoms::route_creation_failed(), &error_msg));
            }
        };

        routes_with_handlers.push((route, Arc::new(elixir_handler) as Arc<dyn Handler>));
    }

    // Build the router
    let router = match Server::with_handlers(config, routes_with_handlers) {
        Ok(r) => r,
        Err(e) => {
            let error_msg = format!("Failed to build router: {}", e);
            return Ok(struct_error(env, atoms::router_build_failed(), &error_msg));
        }
    };

    // Create resource
    let resource = ResourceArc::new(TestClientResource::new(router, handler_runner_pid));

    Ok((atoms::ok(), resource).encode(env))
}

/// Make a request to the test client.
///
/// # Arguments
///
/// * `env` - Elixir environment
/// * `client` - TestClient resource
/// * `method` - HTTP method string (GET, POST, etc.)
/// * `path` - Request path
/// * `opts` - Request options map (headers, query, json, form, cookies)
///
/// # Returns
///
/// `{:ok, response_map}` or `{:error, reason}`
#[rustler::nif(schedule = "DirtyCpu")]
pub fn test_client_request<'a>(
    env: Env<'a>,
    client: ResourceArc<TestClientResource>,
    method: String,
    path: String,
    opts: Term<'a>,
) -> NifResult<Term<'a>> {
    let inner = client.inner.lock().map_err(|_| rustler::Error::BadArg)?;
    let inner = match inner.as_ref() {
        Some(i) => i,
        None => {
            return Ok(struct_error(env, atoms::error(), "Test client has been closed"));
        }
    };

    // Parse request options
    let options = parse_request_options(env, opts)?;

    // Build the full path with query params
    let full_path = build_full_path(&path, &options.query_params);

    // Parse method
    let http_method = match method.to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "PATCH" => Method::PATCH,
        "DELETE" => Method::DELETE,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        "TRACE" => Method::TRACE,
        _ => {
            return Ok(struct_error(env, atoms::error(), &format!("Invalid HTTP method: {}", method)));
        }
    };

    // Parse URI
    let uri: Uri = match full_path.parse() {
        Ok(u) => u,
        Err(e) => {
            return Ok(struct_error(env, atoms::error(), &format!("Invalid URI: {}", e)));
        }
    };

    // Build request
    let mut request_builder = Request::builder().method(http_method).uri(uri);

    // Add headers
    for (key, value) in &options.headers {
        let header_name = match HeaderName::from_bytes(key.as_bytes()) {
            Ok(n) => n,
            Err(e) => {
                return Ok(struct_error(env, atoms::error(), &format!("Invalid header name '{}': {}", key, e)));
            }
        };
        let header_value = match HeaderValue::from_str(value) {
            Ok(v) => v,
            Err(e) => {
                return Ok(struct_error(env, atoms::error(), &format!("Invalid header value for '{}': {}", key, e)));
            }
        };
        request_builder = request_builder.header(header_name, header_value);
    }

    // Add cookies as Cookie header
    if !options.cookies.is_empty() {
        let cookie_str: Vec<String> = options.cookies.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
        let cookie_value = cookie_str.join("; ");
        request_builder = request_builder.header("cookie", cookie_value);
    }

    // Build body
    let body = if let Some(json_body) = &options.json_body {
        // Set content-type if not already set
        if !options.headers.contains_key("content-type") {
            request_builder = request_builder.header("content-type", "application/json");
        }
        Body::from(serde_json::to_vec(json_body).unwrap_or_default())
    } else if let Some(form_data) = &options.form_data {
        // URL-encoded form data
        if !options.headers.contains_key("content-type") {
            request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
        }
        let encoded = encode_form_data(form_data);
        Body::from(encoded)
    } else {
        Body::empty()
    };

    // Build final request
    let request = match request_builder.body(body) {
        Ok(r) => r,
        Err(e) => {
            return Ok(struct_error(env, atoms::error(), &format!("Failed to build request: {}", e)));
        }
    };

    // Dispatch request using the router
    let router = inner.router.clone();

    // Use the global multi-threaded runtime for proper async/message handling
    let runtime = match &*GLOBAL_RUNTIME {
        Ok(rt) => rt,
        Err(e) => {
            return Ok(struct_error(env, atoms::error(), &format!("Failed to get runtime: {}", e)));
        }
    };

    // Spawn the async task on a worker thread (not the current dirty scheduler)
    // and use a channel to get the result. This allows OwnedEnv message sending
    // to work from the unmanaged Tokio worker threads.
    let (tx, rx) = std::sync::mpsc::channel();

    runtime.spawn(async move {
        let result = match router.oneshot(request).await {
            Ok(response) => snapshot_response(response).await,
            Err(e) => Err(SnapshotError::Decompression(format!("Request failed: {}", e))),
        };
        let _ = tx.send(result);
    });

    // Wait for the result from the worker thread
    let response = match rx.recv() {
        Ok(result) => result,
        Err(e) => Err(SnapshotError::Decompression(format!("Channel recv error: {}", e))),
    };

    match response {
        Ok(snapshot) => {
            // Convert to Elixir map
            let response_map = snapshot_to_elixir(env, &snapshot)?;
            Ok((atoms::ok(), response_map).encode(env))
        }
        Err(e) => {
            Ok(struct_error(env, atoms::error(), &format!("Request error: {}", e)))
        }
    }
}

/// Close the test client and release resources.
#[rustler::nif]
pub fn test_client_close<'a>(
    env: Env<'a>,
    client: ResourceArc<TestClientResource>,
) -> NifResult<Term<'a>> {
    let mut inner = client.inner.lock().map_err(|_| rustler::Error::BadArg)?;
    *inner = None;
    Ok(atoms::ok().encode(env))
}

/// Request options parsed from Elixir map.
struct RequestOptions {
    headers: HashMap<String, String>,
    query_params: Vec<(String, String)>,
    cookies: HashMap<String, String>,
    json_body: Option<JsonValue>,
    form_data: Option<Vec<(String, String)>>,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            headers: HashMap::new(),
            query_params: Vec::new(),
            cookies: HashMap::new(),
            json_body: None,
            form_data: None,
        }
    }
}

/// Parse request options from Elixir term.
fn parse_request_options(env: Env, opts: Term) -> NifResult<RequestOptions> {
    let mut options = RequestOptions::default();

    let iter = match MapIterator::new(opts) {
        Some(it) => it,
        None => return Ok(options),
    };

    for (key, value) in iter {
        let key_str = decode_key(key)?;

        match key_str.as_str() {
            "headers" => {
                options.headers = decode_string_map(value)?;
            }
            "query" => {
                options.query_params = decode_tuple_list(value)?;
            }
            "cookies" => {
                options.cookies = decode_string_map_from_list(value)?;
            }
            "json" => {
                options.json_body = Some(elixir_to_json(env, value)?);
            }
            "form" => {
                options.form_data = Some(decode_tuple_list(value)?);
            }
            _ => {
                // Ignore unknown keys
            }
        }
    }

    Ok(options)
}

/// Decode a key from an Elixir term (string or atom).
fn decode_key(term: Term) -> NifResult<String> {
    if let Ok(s) = term.decode::<String>() {
        return Ok(s);
    }
    if let Ok(atom) = term.decode::<rustler::Atom>() {
        return Ok(format!("{:?}", atom).trim_start_matches(':').to_string());
    }
    Err(rustler::Error::BadArg)
}

/// Decode a string map from an Elixir map.
fn decode_string_map(term: Term) -> NifResult<HashMap<String, String>> {
    let mut map = HashMap::new();

    let iter = match MapIterator::new(term) {
        Some(it) => it,
        None => return Ok(map),
    };

    for (key, value) in iter {
        let key_str = decode_key(key)?;
        let value_str: String = value.decode()?;
        map.insert(key_str, value_str);
    }

    Ok(map)
}

/// Decode a string map from a list of tuples.
fn decode_string_map_from_list(term: Term) -> NifResult<HashMap<String, String>> {
    let mut map = HashMap::new();

    if let Ok(list) = term.decode::<Vec<Term>>() {
        for item in list {
            if let Ok((key, value)) = item.decode::<(String, String)>() {
                map.insert(key, value);
            }
        }
    } else if let Some(iter) = MapIterator::new(term) {
        for (key, value) in iter {
            let key_str = decode_key(key)?;
            let value_str: String = value.decode()?;
            map.insert(key_str, value_str);
        }
    }

    Ok(map)
}

/// Decode a list of tuples to Vec<(String, String)>.
fn decode_tuple_list(term: Term) -> NifResult<Vec<(String, String)>> {
    let mut result = Vec::new();

    if let Ok(list) = term.decode::<Vec<Term>>() {
        for item in list {
            if let Ok((key, value)) = item.decode::<(String, String)>() {
                result.push((key, value));
            }
        }
    }

    Ok(result)
}

/// Build full path with query parameters.
fn build_full_path(path: &str, query_params: &[(String, String)]) -> String {
    if query_params.is_empty() {
        return path.to_string();
    }

    let query_string: Vec<String> = query_params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect();

    if path.contains('?') {
        format!("{}&{}", path, query_string.join("&"))
    } else {
        format!("{}?{}", path, query_string.join("&"))
    }
}

/// Encode form data as URL-encoded string.
fn encode_form_data(form_data: &[(String, String)]) -> String {
    form_data
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

/// Convert an axum Response to ResponseSnapshot.
async fn snapshot_response(response: axum::response::Response<Body>) -> Result<ResponseSnapshot, SnapshotError> {
    let (parts, body) = response.into_parts();
    let status = parts.status.as_u16();

    let mut headers = HashMap::new();
    for (name, value) in parts.headers.iter() {
        let header_value = value
            .to_str()
            .map_err(|e| SnapshotError::InvalidHeader(e.to_string()))?;
        headers.insert(name.to_string().to_ascii_lowercase(), header_value.to_string());
    }

    let collected = body
        .collect()
        .await
        .map_err(|e| SnapshotError::Decompression(e.to_string()))?;
    let bytes = collected.to_bytes().to_vec();

    Ok(ResponseSnapshot {
        status,
        headers,
        body: bytes,
    })
}

/// Convert ResponseSnapshot to Elixir map.
fn snapshot_to_elixir<'a>(env: Env<'a>, snapshot: &ResponseSnapshot) -> NifResult<Term<'a>> {
    let status_term = snapshot.status.encode(env);

    // Convert headers to Elixir map
    let headers_pairs: Vec<(Term<'a>, Term<'a>)> = snapshot
        .headers
        .iter()
        .map(|(k, v)| (k.as_str().encode(env), v.as_str().encode(env)))
        .collect();
    let headers_term = Term::map_from_pairs(env, &headers_pairs).map_err(|_| rustler::Error::BadArg)?;

    // Body as string (try to decode as UTF-8)
    let body_text = String::from_utf8_lossy(&snapshot.body).to_string();
    let body_term = body_text.encode(env);

    // Try to parse body as JSON
    let json_term = if let Ok(json_value) = serde_json::from_slice::<JsonValue>(&snapshot.body) {
        json_to_elixir(env, &json_value)?
    } else {
        atoms::nil().encode(env)
    };

    // Build response map
    let pairs: Vec<(Term<'a>, Term<'a>)> = vec![
        ("status_code".encode(env), status_term),
        ("headers".encode(env), headers_term),
        ("body".encode(env), body_term),
        ("json".encode(env), json_term),
    ];

    Term::map_from_pairs(env, &pairs).map_err(|_| rustler::Error::BadArg)
}

/// Extract test configuration from Elixir map (simplified version of server config).
/// Returns (ServerConfig, HookCounts) to allow lifecycle hook integration.
fn extract_test_config(config_term: Term) -> (ServerConfig, HookCounts) {
    let config = ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 0, // Not used for testing
        ..Default::default()
    };

    // Extract lifecycle hook counts from config
    let hook_counts = extract_lifecycle_counts(config_term);

    (config, hook_counts)
}

/// Extract lifecycle hook counts from the config map.
/// Looks for a "lifecycle" key with nested hook type keys.
fn extract_lifecycle_counts(config_term: Term) -> HookCounts {
    let mut counts = HookCounts::default();

    let iter = match MapIterator::new(config_term) {
        Some(it) => it,
        None => return counts,
    };

    for (key, value) in iter {
        let key_str = match decode_key(key) {
            Ok(s) => s,
            Err(_) => continue,
        };

        if key_str == "lifecycle" {
            // Parse lifecycle sub-map
            if let Some(lifecycle_iter) = MapIterator::new(value) {
                for (hook_key, hook_value) in lifecycle_iter {
                    let hook_key_str = match decode_key(hook_key) {
                        Ok(s) => s,
                        Err(_) => continue,
                    };

                    // Get the count of hooks for this type
                    let count = if let Ok(list) = hook_value.decode::<Vec<Term>>() {
                        list.len()
                    } else {
                        0
                    };

                    match hook_key_str.as_str() {
                        "on_request" => counts.on_request = count,
                        "pre_validation" => counts.pre_validation = count,
                        "pre_handler" => counts.pre_handler = count,
                        "on_response" => counts.on_response = count,
                        "on_error" => counts.on_error = count,
                        _ => {}
                    }
                }
            }
        }
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_full_path_no_params() {
        let path = build_full_path("/users", &[]);
        assert_eq!(path, "/users");
    }

    #[test]
    fn test_build_full_path_with_params() {
        let params = vec![
            ("page".to_string(), "1".to_string()),
            ("limit".to_string(), "10".to_string()),
        ];
        let path = build_full_path("/users", &params);
        assert!(path.contains("page=1"));
        assert!(path.contains("limit=10"));
    }

    #[test]
    fn test_encode_form_data() {
        let data = vec![
            ("name".to_string(), "Alice Bob".to_string()),
            ("email".to_string(), "alice@example.com".to_string()),
        ];
        let encoded = encode_form_data(&data);
        assert!(encoded.contains("name=Alice%20Bob"));
        assert!(encoded.contains("email=alice%40example.com"));
    }
}
