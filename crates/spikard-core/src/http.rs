use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::OnceLock;

/// HTTP method
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Method {
    #[default]
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
    Connect,
    Trace,
}

impl Method {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
            Self::Connect => "CONNECT",
            Self::Trace => "TRACE",
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<Method> for http::method::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::Get => Self::GET,
            Method::Post => Self::POST,
            Method::Put => Self::PUT,
            Method::Patch => Self::PATCH,
            Method::Delete => Self::DELETE,
            Method::Head => Self::HEAD,
            Method::Options => Self::OPTIONS,
            Method::Connect => Self::CONNECT,
            Method::Trace => Self::TRACE,
        }
    }
}

impl From<&Method> for http::method::Method {
    fn from(method: &Method) -> Self {
        match method {
            Method::Get => Self::GET,
            Method::Post => Self::POST,
            Method::Put => Self::PUT,
            Method::Patch => Self::PATCH,
            Method::Delete => Self::DELETE,
            Method::Head => Self::HEAD,
            Method::Options => Self::OPTIONS,
            Method::Connect => Self::CONNECT,
            Method::Trace => Self::TRACE,
        }
    }
}

impl std::str::FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            "HEAD" => Ok(Self::Head),
            "OPTIONS" => Ok(Self::Options),
            "CONNECT" => Ok(Self::Connect),
            "TRACE" => Ok(Self::Trace),
            _ => Err(format!("Unknown HTTP method: {s}")),
        }
    }
}

/// CORS configuration for a route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    #[serde(default)]
    pub allowed_headers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,

    #[serde(skip)]
    #[doc(hidden)]
    #[cfg_attr(alef, alef(skip))]
    pub methods_joined_cache: OnceLock<String>,
    #[serde(skip)]
    #[doc(hidden)]
    #[cfg_attr(alef, alef(skip))]
    pub headers_joined_cache: OnceLock<String>,
}

impl CorsConfig {
    /// Get the cached joined methods string for preflight responses
    pub fn allowed_methods_joined(&self) -> &str {
        self.methods_joined_cache
            .get_or_init(|| self.allowed_methods.join(", "))
    }

    /// Get the cached joined headers string for preflight responses
    pub fn allowed_headers_joined(&self) -> &str {
        self.headers_joined_cache
            .get_or_init(|| self.allowed_headers.join(", "))
    }

    /// Check if an origin is allowed (O(1) with wildcard, O(n) for exact match)
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        if origin.is_empty() {
            return false;
        }
        self.allowed_origins.iter().any(|o| o == "*" || o == origin)
    }

    /// Check if a method is allowed (O(1) with wildcard, O(n) for exact match)
    pub fn is_method_allowed(&self, method: &str) -> bool {
        self.allowed_methods
            .iter()
            .any(|m| m == "*" || m.eq_ignore_ascii_case(method))
    }

    /// Check if all requested headers are allowed (O(n) where n = num requested headers)
    pub fn are_headers_allowed(&self, requested: &[&str]) -> bool {
        if self.allowed_headers.iter().any(|h| h == "*") {
            return true;
        }

        requested.iter().all(|req_header| {
            self.allowed_headers
                .iter()
                .any(|h| h.to_lowercase() == req_header.to_lowercase())
        })
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["*".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
            methods_joined_cache: OnceLock::new(),
            headers_joined_cache: OnceLock::new(),
        }
    }
}

/// Route metadata extracted from bindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteMetadata {
    pub method: String,
    pub path: String,
    pub handler_name: String,
    pub request_schema: Option<Value>,
    pub response_schema: Option<Value>,
    pub parameter_schema: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_params: Option<Value>,
    #[serde(default)]
    pub is_async: bool,
    pub cors: Option<CorsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<CompressionConfig>,
    /// Optional per-route maximum request body size in bytes, overriding the server-global default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_limit: Option<usize>,
    /// Optional per-route request timeout in seconds, overriding the server-global default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout_secs: Option<u64>,
    /// Optional per-route rate limiting configuration, overriding the server-global default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<RateLimitConfig>,
    /// Optional per-route request-id generation override (`Some(RequestIdConfig { enabled: true })`
    /// forces it on, `Some(RequestIdConfig { enabled: false })` forces it off, `None` inherits the
    /// server-global default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestIdConfig>,
    /// Optional per-route JWT authentication requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_auth: Option<JwtAuthConfig>,
    /// Optional per-route API key authentication requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_auth: Option<ApiKeyAuthConfig>,
    /// Optional per-route roles/scopes/permissions authorization requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<AuthorizationConfig>,
    /// Optional per-route lifecycle hook selection, by registered hook name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_hooks: Option<LifecycleHooksConfig>,
    /// Name of the body parameter (defaults to "body" if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_param_name: Option<String>,
    /// List of dependency keys this handler requires (for DI)
    #[cfg(feature = "di")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler_dependencies: Option<Vec<String>>,
    /// JSON-RPC method metadata (if this route is exposed as a JSON-RPC method)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonrpc_method: Option<Value>,
    /// Optional literal `OpenRPC` method spec document, overriding auto-derivation from
    /// `jsonrpc_method` when present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openrpc_spec: Option<Value>,
    /// Optional static response configuration: `{"status": 200, "body": "OK", "content_type": "text/plain"}`
    /// When present, the handler is replaced by a `StaticResponseHandler` that bypasses the full
    /// middleware pipeline for maximum throughput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_response: Option<Value>,
}

impl Default for RouteMetadata {
    fn default() -> Self {
        Self {
            method: "GET".to_string(),
            path: "/".to_string(),
            handler_name: String::new(),
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
            jsonrpc_method: None,
            openrpc_spec: None,
            static_response: None,
            compression: None,
            body_limit: None,
            request_timeout_secs: None,
            rate_limit: None,
            request_id: None,
            jwt_auth: None,
            api_key_auth: None,
            authorization: None,
            lifecycle_hooks: None,
        }
    }
}

/// Compression configuration shared across runtimes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// Enable gzip compression
    #[serde(default = "default_true")]
    pub gzip: bool,
    /// Enable brotli compression
    #[serde(default = "default_true")]
    pub brotli: bool,
    /// Minimum response size to compress (bytes)
    #[serde(default = "default_compression_min_size")]
    pub min_size: usize,
    /// Compression quality (0-11 for brotli, 0-9 for gzip)
    #[serde(default = "default_compression_quality")]
    pub quality: u32,
}

const fn default_true() -> bool {
    true
}

const fn default_compression_min_size() -> usize {
    1024
}

const fn default_compression_quality() -> u32 {
    6
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            gzip: true,
            brotli: true,
            min_size: default_compression_min_size(),
            quality: default_compression_quality(),
        }
    }
}

/// Rate limiting configuration shared across runtimes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per second
    pub per_second: u64,
    /// Burst allowance
    pub burst: u32,
    /// Use IP-based rate limiting
    #[serde(default = "default_true")]
    pub ip_based: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            per_second: 100,
            burst: 200,
            ip_based: true,
        }
    }
}

/// Per-route request-id generation/propagation override.
///
/// Modeled as a struct rather than `Option<bool>` on `RouteMetadata` because the wire shape is an
/// object, not a bare boolean: `fixtures/request_id.json`'s `request_id_middleware_can_be_disabled`
/// sends `{"enabled": false}`, which `Option<bool>` cannot deserialize at all ("invalid type: map,
/// expected a boolean") — the very fixture whose purpose is proving the middleware can be disabled
/// was the one that failed to parse. ~keep
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestIdConfig {
    /// Whether request-id generation/propagation is active for this route
    pub enabled: bool,
}

impl From<bool> for RequestIdConfig {
    fn from(enabled: bool) -> Self {
        Self { enabled }
    }
}

/// Per-route JWT authentication requirement.
///
/// `spikard-http` defines the canonical `JwtConfig` used by `ServerConfig.jwt_auth`, but
/// `spikard-core` cannot depend on `spikard-http` (the dependency runs the other way), so that
/// type cannot be reused here. This mirrors its fields so a later enforcement phase in
/// `spikard-http` can convert between the two without losing information. ~keep
///
/// `secret` and `public_key` are both optional because asymmetric algorithms (RS256, ES256, ...)
/// verify against a public key rather than a shared secret; see `fixtures/auth.json`'s
/// `jwt_config_algorithm_rs256`, which carries `public_key` and no `secret` at all. Exactly one is
/// expected to be populated for a given `algorithm`, but that cross-field invariant is left to a
/// later enforcement phase rather than the type itself. ~keep
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JwtAuthConfig {
    /// Whether this per-route JWT auth requirement is active. Present on 21/21 `jwt_auth` fixture
    /// payloads in the corpus; defaults to `true` because presence of a `jwt_auth` block has always
    /// meant "enabled" up to now. Without this field a fixture setting `"enabled": false` would
    /// silently keep auth on while the fixture's parse still succeeds — a vacuous pass. ~keep
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Symmetric secret key for JWT verification (HS256, HS384, HS512)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Asymmetric public key for JWT verification (RS256, ES256, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// Required algorithm (HS256, HS384, HS512, RS256, etc.)
    #[serde(default = "default_jwt_algorithm")]
    pub algorithm: String,
    /// Required audience claim
    pub audience: Option<Vec<String>>,
    /// Required issuer claim
    pub issuer: Option<String>,
    /// Leeway for expiration checks (seconds)
    #[serde(default)]
    pub leeway: u64,
}

fn default_jwt_algorithm() -> String {
    "HS256".to_string()
}

/// Per-route API key authentication requirement.
///
/// Mirrors `spikard_http::ApiKeyConfig` for the same reason `JwtAuthConfig` mirrors
/// `spikard_http::JwtConfig`: `spikard-core` cannot depend on `spikard-http`. ~keep
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiKeyAuthConfig {
    /// Whether this per-route API key auth requirement is active. Present on 10/10 `api_key_auth`
    /// fixture payloads in the corpus; defaults to `true` for the same reason as
    /// [`JwtAuthConfig::enabled`]. ~keep
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Valid API keys. Defaults to empty (rather than being a required field) so that
    /// `fixtures/server_config.json`'s `server_jwt_and_api_key_auth_combined` payload
    /// (`{"enabled": true, "header": "X-API-Key"}`, no `keys` at all) deserializes instead of
    /// hard-failing with "missing field `keys`". An empty list is NOT "allow everyone": a later
    /// enforcement phase MUST treat an empty `keys` list as a hard misconfiguration error, never
    /// as an open gate. ~keep
    #[serde(default)]
    pub keys: Vec<String>,
    /// Header name to check (e.g., "X-API-Key")
    #[serde(default = "default_api_key_header")]
    pub header_name: String,
}

fn default_api_key_header() -> String {
    "X-API-Key".to_string()
}

/// Per-route roles/scopes/permissions authorization requirement.
///
/// `spikard_http::auth::Claims` does not yet carry roles, scopes, or permissions, so nothing can
/// enforce this today. This type only defines the requirement shape; a later phase must extend
/// `Claims` (or an equivalent claims-decoding path) to populate them before enforcement is
/// possible. ~keep
///
/// Deserialization goes through [`AuthorizationConfigRepr`] rather than a derive so the fixture's
/// singular `{"required_role": "admin"}` shape (`fixtures/problem_details.json`'s
/// `problem_details_403_forbidden`) populates `required_roles` instead of being silently dropped
/// as an unrecognized field — a config that parses to "no constraint" from real authorization
/// data is a vacuous-pass bug, not a compatibility shim. `deny_unknown_fields` on the repr means
/// any other unrecognized key is a loud deserialize error instead. ~keep
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthorizationConfig {
    /// Roles the authenticated caller must have
    pub required_roles: Vec<String>,
    /// OAuth-style scopes the authenticated caller must have
    pub required_scopes: Vec<String>,
    /// Fine-grained permissions the authenticated caller must have
    pub required_permissions: Vec<String>,
    /// When true, the caller must satisfy every listed requirement (AND); when false, any single
    /// listed requirement is sufficient (OR)
    pub require_all: bool,
}

impl Default for AuthorizationConfig {
    fn default() -> Self {
        Self {
            required_roles: Vec::new(),
            required_scopes: Vec::new(),
            required_permissions: Vec::new(),
            require_all: true,
        }
    }
}

/// Wire representation of [`AuthorizationConfig`], matching the exact JSON shapes fixtures and
/// bindings send (both the singular `required_role` string and a plural `required_roles` array).
/// `deny_unknown_fields` here is what turns future fixture drift into a deserialize error instead
/// of a silent no-op. ~keep
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorizationConfigRepr {
    #[serde(default)]
    required_role: Option<String>,
    #[serde(default)]
    required_roles: Vec<String>,
    #[serde(default)]
    required_scopes: Vec<String>,
    #[serde(default)]
    required_permissions: Vec<String>,
    #[serde(default = "default_true")]
    require_all: bool,
}

impl<'de> Deserialize<'de> for AuthorizationConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let repr = AuthorizationConfigRepr::deserialize(deserializer)?;
        let mut required_roles = repr.required_roles;
        if let Some(role) = repr.required_role {
            required_roles.push(role);
        }
        Ok(Self {
            required_roles,
            required_scopes: repr.required_scopes,
            required_permissions: repr.required_permissions,
            require_all: repr.require_all,
        })
    }
}

/// A single lifecycle hook reference within a [`LifecycleHooksConfig`] phase.
///
/// Matches the fixture shape exactly (`fixtures/lifecycle_hooks.json`, `fixtures/di.json`): each
/// entry is an object with a required `name` and `handler`, an optional list of dependency keys,
/// and an optional free-form `config` blob (e.g. `{"max_requests": 10, "window_seconds": 60}` for
/// a rate-limiting hook). `deny_unknown_fields` turns future fixture drift into a loud error. ~keep
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct LifecycleHookRef {
    /// Registered name of the hook to run, resolved against the server's `LifecycleHooks`
    pub name: String,
    /// Name of the handler function this hook invokes
    pub handler: String,
    /// Dependency keys this hook requires (for DI), resolved before the hook runs
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// Optional free-form configuration passed to the hook (e.g. rate-limit thresholds)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
    /// Explicit execution order within the phase, where the corpus states one
    /// (`fixtures/lifecycle_hooks.json`'s `hook_execution_order`). Array position already implies
    /// an order, so this exists to let a fixture assert ordering rather than rely on it. ~keep
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

/// Per-route selection of registered lifecycle hooks.
///
/// `ServerConfig.lifecycle_hooks` holds `Arc<dyn LifecycleHook>` function pointers and is marked
/// `#[serde(skip)]` / `#[alef(skip)]` because closures cannot be serialized or cross the FFI
/// boundary. A per-route field with that same shape would be invisible to alef, and therefore
/// invisible to every binding — defeating the purpose of exposing it here. This descriptor
/// carries [`LifecycleHookRef`] entries instead: each one names a registered hook (plus its
/// declared dependencies and optional config), so a later enforcement phase can resolve those
/// names against the server's registered `LifecycleHooks` and run the matches for this route. The
/// five fields mirror the five hook phases documented in the `tower-middleware-and-lifecycle`
/// project convention (onRequest, preValidation, preHandler, onResponse, onError). ~keep
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LifecycleHooksConfig {
    /// Hooks to run in the `on_request` phase
    #[serde(default)]
    pub on_request: Vec<LifecycleHookRef>,
    /// Hooks to run in the `pre_validation` phase
    #[serde(default)]
    pub pre_validation: Vec<LifecycleHookRef>,
    /// Hooks to run in the `pre_handler` phase
    #[serde(default)]
    pub pre_handler: Vec<LifecycleHookRef>,
    /// Hooks to run in the `on_response` phase
    #[serde(default)]
    pub on_response: Vec<LifecycleHookRef>,
    /// Hooks to run in the `on_error` phase
    #[serde(default)]
    pub on_error: Vec<LifecycleHookRef>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_method_as_str_get() {
        assert_eq!(Method::Get.as_str(), "GET");
    }

    #[test]
    fn test_method_as_str_post() {
        assert_eq!(Method::Post.as_str(), "POST");
    }

    #[test]
    fn test_method_as_str_put() {
        assert_eq!(Method::Put.as_str(), "PUT");
    }

    #[test]
    fn test_method_as_str_patch() {
        assert_eq!(Method::Patch.as_str(), "PATCH");
    }

    #[test]
    fn test_method_as_str_delete() {
        assert_eq!(Method::Delete.as_str(), "DELETE");
    }

    #[test]
    fn test_method_as_str_head() {
        assert_eq!(Method::Head.as_str(), "HEAD");
    }

    #[test]
    fn test_method_as_str_options() {
        assert_eq!(Method::Options.as_str(), "OPTIONS");
    }

    #[test]
    fn test_method_as_str_trace() {
        assert_eq!(Method::Trace.as_str(), "TRACE");
    }

    #[test]
    fn test_method_display_get() {
        assert_eq!(Method::Get.to_string(), "GET");
    }

    #[test]
    fn test_method_display_post() {
        assert_eq!(Method::Post.to_string(), "POST");
    }

    #[test]
    fn test_method_display_put() {
        assert_eq!(Method::Put.to_string(), "PUT");
    }

    #[test]
    fn test_method_display_patch() {
        assert_eq!(Method::Patch.to_string(), "PATCH");
    }

    #[test]
    fn test_method_display_delete() {
        assert_eq!(Method::Delete.to_string(), "DELETE");
    }

    #[test]
    fn test_method_display_head() {
        assert_eq!(Method::Head.to_string(), "HEAD");
    }

    #[test]
    fn test_method_display_options() {
        assert_eq!(Method::Options.to_string(), "OPTIONS");
    }

    #[test]
    fn test_method_display_trace() {
        assert_eq!(Method::Trace.to_string(), "TRACE");
    }

    #[test]
    fn test_from_str_get() {
        assert_eq!(Method::from_str("GET"), Ok(Method::Get));
    }

    #[test]
    fn test_from_str_post() {
        assert_eq!(Method::from_str("POST"), Ok(Method::Post));
    }

    #[test]
    fn test_from_str_put() {
        assert_eq!(Method::from_str("PUT"), Ok(Method::Put));
    }

    #[test]
    fn test_from_str_patch() {
        assert_eq!(Method::from_str("PATCH"), Ok(Method::Patch));
    }

    #[test]
    fn test_from_str_delete() {
        assert_eq!(Method::from_str("DELETE"), Ok(Method::Delete));
    }

    #[test]
    fn test_from_str_head() {
        assert_eq!(Method::from_str("HEAD"), Ok(Method::Head));
    }

    #[test]
    fn test_from_str_options() {
        assert_eq!(Method::from_str("OPTIONS"), Ok(Method::Options));
    }

    #[test]
    fn test_from_str_trace() {
        assert_eq!(Method::from_str("TRACE"), Ok(Method::Trace));
    }

    #[test]
    fn test_from_str_lowercase() {
        assert_eq!(Method::from_str("get"), Ok(Method::Get));
    }

    #[test]
    fn test_from_str_mixed_case() {
        assert_eq!(Method::from_str("PoSt"), Ok(Method::Post));
    }

    #[test]
    fn test_from_str_invalid_method() {
        let result = Method::from_str("INVALID");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown HTTP method: INVALID");
    }

    #[test]
    fn test_from_str_empty_string() {
        let result = Method::from_str("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown HTTP method: ");
    }

    #[test]
    fn test_compression_config_default() {
        let config = CompressionConfig::default();
        assert!(config.gzip);
        assert!(config.brotli);
        assert_eq!(config.min_size, 1024);
        assert_eq!(config.quality, 6);
    }

    #[test]
    fn test_default_true() {
        assert!(default_true());
    }

    #[test]
    fn test_default_compression_min_size() {
        assert_eq!(default_compression_min_size(), 1024);
    }

    #[test]
    fn test_default_compression_quality() {
        assert_eq!(default_compression_quality(), 6);
    }

    #[test]
    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.per_second, 100);
        assert_eq!(config.burst, 200);
        assert!(config.ip_based);
    }

    #[test]
    fn test_method_equality() {
        assert_eq!(Method::Get, Method::Get);
        assert_ne!(Method::Get, Method::Post);
    }

    #[test]
    fn test_method_clone() {
        let method = Method::Post;
        let cloned = method.clone();
        assert_eq!(method, cloned);
    }

    #[test]
    fn test_compression_config_custom_values() {
        let config = CompressionConfig {
            gzip: false,
            brotli: false,
            min_size: 2048,
            quality: 11,
        };
        assert!(!config.gzip);
        assert!(!config.brotli);
        assert_eq!(config.min_size, 2048);
        assert_eq!(config.quality, 11);
    }

    #[test]
    fn test_rate_limit_config_custom_values() {
        let config = RateLimitConfig {
            per_second: 50,
            burst: 100,
            ip_based: false,
        };
        assert_eq!(config.per_second, 50);
        assert_eq!(config.burst, 100);
        assert!(!config.ip_based);
    }

    #[test]
    fn test_cors_config_construction() {
        let cors = CorsConfig {
            allowed_origins: vec!["http://localhost:3000".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            allowed_headers: vec![],
            expose_headers: None,
            max_age: None,
            allow_credentials: None,
            ..Default::default()
        };
        assert_eq!(cors.allowed_origins.len(), 1);
        assert_eq!(cors.allowed_methods.len(), 2);
        assert_eq!(cors.allowed_headers.len(), 0);
    }

    #[test]
    fn test_route_metadata_construction() {
        let metadata = RouteMetadata {
            method: "GET".to_string(),
            path: "/api/users".to_string(),
            handler_name: "get_users".to_string(),
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
            jsonrpc_method: None,
            openrpc_spec: None,
            static_response: None,
            compression: None,
            body_limit: None,
            request_timeout_secs: None,
            rate_limit: None,
            request_id: None,
            jwt_auth: None,
            api_key_auth: None,
            authorization: None,
            lifecycle_hooks: None,
        };
        assert_eq!(metadata.method, "GET");
        assert_eq!(metadata.path, "/api/users");
        assert_eq!(metadata.handler_name, "get_users");
        assert!(metadata.is_async);
    }

    #[test]
    fn test_route_metadata_default_has_no_middleware_configured() {
        let metadata = RouteMetadata::default();
        assert!(metadata.rate_limit.is_none());
        assert!(metadata.request_id.is_none());
        assert!(metadata.jwt_auth.is_none());
        assert!(metadata.api_key_auth.is_none());
        assert!(metadata.authorization.is_none());
        assert!(metadata.lifecycle_hooks.is_none());
        assert!(metadata.openrpc_spec.is_none());
    }

    /// Real fixture payload: `fixtures/request_id.json` id `request_id_is_generated_when_not_provided`.
    #[test]
    fn should_enable_request_id_when_given_real_enabled_true_fixture_payload() {
        let json = serde_json::json!({ "enabled": true });
        let config: RequestIdConfig = serde_json::from_value(json).unwrap();
        assert!(config.enabled);
    }

    /// Real fixture payload: `fixtures/request_id.json` id `request_id_middleware_can_be_disabled`.
    /// This is the payload that could not deserialize at all when `request_id` was `Option<bool>`:
    /// `{"enabled": false}` is an object, and serde reports "invalid type: map, expected a boolean"
    /// against a bare `bool` field.
    #[test]
    fn should_disable_request_id_when_given_real_enabled_false_fixture_payload() {
        let json = serde_json::json!({ "enabled": false });
        let config: RequestIdConfig = serde_json::from_value(json).unwrap();
        assert!(!config.enabled);
    }

    /// End-to-end proof that `RouteMetadata` itself (not just the leaf `RequestIdConfig` type)
    /// deserializes the disable fixture, since that is the payload's actual real-world path.
    #[test]
    fn should_deserialize_route_metadata_request_id_when_given_real_disabled_fixture_payload() {
        let mut json_value = serde_json::json!(RouteMetadata::default());
        json_value["request_id"] = serde_json::json!({ "enabled": false });

        let metadata: RouteMetadata = serde_json::from_value(json_value).unwrap();
        let request_id = metadata.request_id.expect("request_id present");
        assert!(!request_id.enabled);
    }

    #[test]
    fn test_jwt_auth_config_deserializes_with_default_algorithm() {
        let json = serde_json::json!({ "secret": "s3cr3t" });
        let config: JwtAuthConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.secret.as_deref(), Some("s3cr3t"));
        assert!(config.public_key.is_none());
        assert_eq!(config.algorithm, "HS256");
        assert_eq!(config.leeway, 0);
        assert!(config.audience.is_none());
        assert!(config.issuer.is_none());
        assert!(config.enabled);
    }

    /// Real fixture payload: `fixtures/auth.json` id `jwt_config_algorithm_rs256`. RS256 verifies
    /// against a public key and carries no `secret` at all; a `secret`-required type would fail
    /// to deserialize this fixture. Also proves `enabled` is captured rather than silently
    /// discarded.
    #[test]
    fn should_deserialize_rs256_public_key_when_given_real_jwt_fixture_payload() {
        let json = serde_json::json!({
            "algorithm": "RS256",
            "enabled": true,
            "public_key": "-----BEGIN PUBLIC KEY-----\nMFwwDQYJKoZIhvcNAQEBBQADSwAwSAJBALRiMLAA\n-----END PUBLIC KEY-----"
        });
        let config: JwtAuthConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.algorithm, "RS256");
        assert!(config.secret.is_none());
        assert_eq!(
            config.public_key.as_deref(),
            Some("-----BEGIN PUBLIC KEY-----\nMFwwDQYJKoZIhvcNAQEBBQADSwAwSAJBALRiMLAA\n-----END PUBLIC KEY-----")
        );
        assert!(config.enabled);
    }

    /// Synthetic payload (no corpus fixture currently sets `"enabled": false` on `jwt_auth`), used
    /// to prove `false` is representable and preserved rather than silently coerced to `true`.
    #[test]
    fn should_preserve_false_enabled_value_when_jwt_auth_disabled() {
        let json = serde_json::json!({ "algorithm": "HS256", "enabled": false, "secret": "s3cr3t" });
        let config: JwtAuthConfig = serde_json::from_value(json).unwrap();
        assert!(!config.enabled);
    }

    #[test]
    fn test_api_key_auth_config_deserializes_with_default_header_name() {
        let json = serde_json::json!({ "keys": ["abc123"] });
        let config: ApiKeyAuthConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.keys, vec!["abc123".to_string()]);
        assert_eq!(config.header_name, "X-API-Key");
        assert!(config.enabled);
    }

    /// Real fixture payload: `fixtures/auth.json` id `api_key_authentication_valid_key`. Proves
    /// `enabled` is captured rather than silently discarded.
    #[test]
    fn should_deserialize_api_key_auth_config_capturing_enabled_field_from_real_fixture_payload() {
        let json = serde_json::json!({
            "enabled": true,
            "header_name": "X-API-Key",
            "keys": ["sk_test_123456", "sk_test_789012"]
        });
        let config: ApiKeyAuthConfig = serde_json::from_value(json).unwrap();
        assert_eq!(
            config.keys,
            vec!["sk_test_123456".to_string(), "sk_test_789012".to_string()]
        );
        assert_eq!(config.header_name, "X-API-Key");
        assert!(config.enabled);
    }

    /// Synthetic payload (no corpus fixture currently sets `"enabled": false` on `api_key_auth`),
    /// used to prove `false` is representable and preserved rather than silently coerced to `true`.
    #[test]
    fn should_preserve_false_enabled_value_when_api_key_auth_disabled() {
        let json = serde_json::json!({ "enabled": false, "header_name": "X-API-Key", "keys": ["k1"] });
        let config: ApiKeyAuthConfig = serde_json::from_value(json).unwrap();
        assert!(!config.enabled);
    }

    /// Real fixture payload: `fixtures/server_config.json` id `server_jwt_and_api_key_auth_combined`.
    /// This is the only `api_key_auth` payload in the corpus that omits `keys` entirely and spells
    /// the header field `header` instead of `header_name`. `keys` must default to empty rather than
    /// hard-failing with "missing field `keys`" — see the field doc comment for why an empty list
    /// is not itself a security hole (enforcement, not parsing, must reject it). The unrecognized
    /// `header` key is simply ignored since `ApiKeyAuthConfig` has no `deny_unknown_fields`.
    #[test]
    fn should_default_keys_to_empty_when_given_real_fixture_payload_missing_keys() {
        let json = serde_json::json!({ "enabled": true, "header": "X-API-Key" });
        let config: ApiKeyAuthConfig = serde_json::from_value(json).unwrap();
        assert!(config.keys.is_empty());
        assert_eq!(config.header_name, "X-API-Key");
        assert!(config.enabled);
    }

    #[test]
    fn test_authorization_config_default_requires_all() {
        let config = AuthorizationConfig::default();
        assert!(config.required_roles.is_empty());
        assert!(config.required_scopes.is_empty());
        assert!(config.required_permissions.is_empty());
        assert!(config.require_all);
    }

    #[test]
    fn test_authorization_config_custom_values() {
        let config = AuthorizationConfig {
            required_roles: vec!["admin".to_string()],
            required_scopes: vec!["users:write".to_string()],
            required_permissions: vec![],
            require_all: false,
        };
        assert_eq!(config.required_roles, vec!["admin".to_string()]);
        assert_eq!(config.required_scopes, vec!["users:write".to_string()]);
        assert!(!config.require_all);
    }

    /// Real fixture payload: `fixtures/problem_details.json` id `problem_details_403_forbidden`.
    /// The fixture uses the singular `required_role` string form; the config must actually
    /// require that role, not silently parse to an empty (vacuously-passing) requirement.
    #[test]
    fn should_require_admin_role_when_given_real_singular_required_role_fixture_payload() {
        let json = serde_json::json!({ "required_role": "admin" });
        let config: AuthorizationConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.required_roles, vec!["admin".to_string()]);
        assert!(config.required_scopes.is_empty());
        assert!(config.required_permissions.is_empty());
    }

    #[test]
    fn should_merge_singular_and_plural_role_fields_when_both_present() {
        let json = serde_json::json!({ "required_role": "admin", "required_roles": ["editor"] });
        let config: AuthorizationConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.required_roles, vec!["editor".to_string(), "admin".to_string()]);
    }

    #[test]
    fn should_error_when_authorization_config_has_unknown_field() {
        let json = serde_json::json!({ "required_group": "admin" });
        let result: Result<AuthorizationConfig, _> = serde_json::from_value(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_lifecycle_hooks_config_default_has_no_hooks_selected() {
        let config = LifecycleHooksConfig::default();
        assert!(config.on_request.is_empty());
        assert!(config.pre_validation.is_empty());
        assert!(config.pre_handler.is_empty());
        assert!(config.on_response.is_empty());
        assert!(config.on_error.is_empty());
    }

    #[test]
    fn test_lifecycle_hooks_config_serialization_round_trip() {
        let config = LifecycleHooksConfig {
            on_request: vec![LifecycleHookRef {
                name: "auth_check".to_string(),
                handler: "run_auth_check".to_string(),
                dependencies: vec![],
                config: None,
                ..Default::default()
            }],
            pre_validation: vec![],
            pre_handler: vec![LifecycleHookRef {
                name: "load_tenant".to_string(),
                handler: "run_load_tenant".to_string(),
                dependencies: vec![],
                config: None,
                ..Default::default()
            }],
            on_response: vec![LifecycleHookRef {
                name: "add_headers".to_string(),
                handler: "run_add_headers".to_string(),
                dependencies: vec![],
                config: None,
                ..Default::default()
            }],
            on_error: vec![],
        };
        let json = serde_json::to_value(&config).unwrap();
        let deserialized: LifecycleHooksConfig = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.on_request[0].name, "auth_check");
        assert_eq!(deserialized.pre_handler[0].name, "load_tenant");
        assert_eq!(deserialized.on_response[0].name, "add_headers");
    }

    /// Real fixture payload: `fixtures/lifecycle_hooks.json` id `onrequest_request_logging`.
    #[test]
    fn should_deserialize_lifecycle_hooks_when_given_real_on_request_fixture_payload() {
        let json = serde_json::json!({
            "on_request": [
                {"handler": "log_request", "name": "request_logger"},
                {"handler": "add_request_id", "name": "request_id_generator"}
            ]
        });
        let config: LifecycleHooksConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.on_request.len(), 2);
        assert_eq!(config.on_request[0].name, "request_logger");
        assert_eq!(config.on_request[0].handler, "log_request");
        assert!(config.on_request[0].dependencies.is_empty());
        assert!(config.on_request[0].config.is_none());
        assert_eq!(config.on_request[1].name, "request_id_generator");
    }

    /// Real fixture payload: `fixtures/lifecycle_hooks.json` id `prevalidation_rate_limiting`.
    /// Covers the optional `config` blob.
    #[test]
    fn should_deserialize_lifecycle_hook_config_blob_when_given_real_rate_limit_fixture_payload() {
        let json = serde_json::json!({
            "pre_validation": [{
                "config": {"max_requests": 10, "window_seconds": 60},
                "handler": "check_rate_limit",
                "name": "rate_limiter"
            }]
        });
        let config: LifecycleHooksConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.pre_validation.len(), 1);
        let hook = &config.pre_validation[0];
        assert_eq!(hook.name, "rate_limiter");
        assert_eq!(hook.handler, "check_rate_limit");
        let hook_config = hook.config.as_ref().unwrap();
        assert_eq!(hook_config.get("max_requests"), Some(&serde_json::json!(10)));
        assert_eq!(hook_config.get("window_seconds"), Some(&serde_json::json!(60)));
    }

    /// Real fixture payload: `fixtures/di.json` id `dependency_injection_in_lifecycle_hooks_success`.
    /// Covers the optional `dependencies` array and multiple phases in one payload.
    #[test]
    fn should_deserialize_lifecycle_hook_dependencies_when_given_real_di_fixture_payload() {
        let json = serde_json::json!({
            "on_request": [{
                "dependencies": ["logger"],
                "handler": "log_incoming_request",
                "name": "log_request"
            }],
            "pre_handler": [{
                "dependencies": ["auth_service"],
                "handler": "check_authentication",
                "name": "auth_check"
            }]
        });
        let config: LifecycleHooksConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.on_request[0].dependencies, vec!["logger".to_string()]);
        assert_eq!(config.pre_handler[0].dependencies, vec!["auth_service".to_string()]);
    }

    /// Real fixture payload: `fixtures/lifecycle_hooks.json` id `multiple_hooks_all_phases`.
    #[test]
    fn should_deserialize_lifecycle_hooks_when_given_real_multi_phase_fixture_payload() {
        let json = serde_json::json!({
            "on_error": [{"handler": "log_error", "name": "error_logger"}],
            "on_request": [
                {"handler": "log_request", "name": "request_logger"},
                {"handler": "add_request_id", "name": "request_id_generator"}
            ],
            "on_response": [
                {"handler": "add_security_headers", "name": "security_headers"},
                {"handler": "add_response_time", "name": "response_timer"},
                {"handler": "log_successful_action", "name": "audit_logger"}
            ]
        });
        let config: LifecycleHooksConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.on_error.len(), 1);
        assert_eq!(config.on_request.len(), 2);
        assert_eq!(config.on_response.len(), 3);
        assert!(config.pre_validation.is_empty());
        assert!(config.pre_handler.is_empty());
    }

    #[test]
    fn should_error_when_lifecycle_hook_ref_has_unknown_field() {
        let json = serde_json::json!({
            "on_request": [{"name": "x", "handler": "y", "unexpected": true}]
        });
        let result: Result<LifecycleHooksConfig, _> = serde_json::from_value(json);
        assert!(result.is_err());
    }
}
