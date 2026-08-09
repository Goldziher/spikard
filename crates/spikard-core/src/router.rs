//! Route management and handler registration

use crate::http::{ApiKeyAuthConfig, AuthorizationConfig, JwtAuthConfig, LifecycleHooksConfig, RequestIdConfig};
use crate::parameters::ParameterValidator;
use crate::schema_registry::SchemaRegistry;
use crate::validation::SchemaValidator;
use crate::{CorsConfig, Method, RateLimitConfig, RouteMetadata};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[cfg(test)]
use std::collections::HashMap;

/// JSON-RPC method metadata for routes that support JSON-RPC
///
/// This struct captures the metadata needed to expose HTTP routes as JSON-RPC methods,
/// enabling discovery and documentation of RPC-compatible endpoints.
///
/// # Examples
///
/// ```ignore
/// use spikard_core::router::JsonRpcMethodInfo;
/// use serde_json::json;
///
/// let rpc_info = JsonRpcMethodInfo {
///     method_name: "user.create".to_string(),
///     description: Some("Creates a new user".to_string()),
///     params_schema: Some(json!({
///         "type": "object",
///         "properties": {
///             "name": {"type": "string"}
///         }
///     })),
///     result_schema: Some(json!({
///         "type": "object",
///         "properties": {
///             "id": {"type": "integer"}
///         }
///     })),
///     deprecated: false,
///     tags: vec!["users".to_string()],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcMethodInfo {
    /// The JSON-RPC method name (e.g., "user.create")
    pub method_name: String,

    /// Optional description of what the method does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional JSON Schema for method parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params_schema: Option<serde_json::Value>,

    /// Optional JSON Schema for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_schema: Option<serde_json::Value>,

    /// Whether this method is deprecated
    #[serde(default)]
    pub deprecated: bool,

    /// Tags for categorizing and grouping methods
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Route definition with compiled validators
///
/// Validators are `Arc`-wrapped to enable cheap cloning across route instances
/// and to support schema deduplication via `SchemaRegistry`.
///
/// The `jsonrpc_method` field is optional and has zero overhead when None,
/// enabling routes to optionally expose themselves as JSON-RPC methods.
#[derive(Clone)]
pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler_name: String,
    pub request_validator: Option<Arc<SchemaValidator>>,
    pub response_validator: Option<Arc<SchemaValidator>>,
    pub parameter_validator: Option<ParameterValidator>,
    pub file_params: Option<Value>,
    pub is_async: bool,
    pub cors: Option<CorsConfig>,
    /// Precomputed flag: true if this route expects a JSON request body
    /// Used by middleware to validate Content-Type headers
    pub expects_json_body: bool,
    /// List of dependency keys this handler requires (for DI)
    #[cfg(feature = "di")]
    pub handler_dependencies: Vec<String>,
    /// Optional JSON-RPC method information
    /// When present, this route can be exposed as a JSON-RPC method
    pub jsonrpc_method: Option<JsonRpcMethodInfo>,
    /// Optional per-route compression configuration
    pub compression: Option<crate::http::CompressionConfig>,
    /// Optional per-route maximum request body size in bytes, overriding the server-global default
    pub body_limit: Option<usize>,
    /// Optional per-route request timeout in seconds, overriding the server-global default
    pub request_timeout_secs: Option<u64>,
    /// Optional per-route rate limiting configuration, overriding the server-global default
    pub rate_limit: Option<RateLimitConfig>,
    /// Optional per-route request-id generation override, overriding the server-global default
    pub request_id: Option<RequestIdConfig>,
    /// Optional per-route JWT authentication requirement, parsed from `RouteMetadata.jwt_auth`
    pub jwt_auth: Option<JwtAuthConfig>,
    /// Optional per-route API key authentication requirement, parsed from `RouteMetadata.api_key_auth`
    pub api_key_auth: Option<ApiKeyAuthConfig>,
    /// Optional per-route roles/scopes/permissions authorization requirement
    pub authorization: Option<AuthorizationConfig>,
    /// Optional per-route lifecycle hook selection, by registered hook name
    pub lifecycle_hooks: Option<LifecycleHooksConfig>,
    /// Optional literal `OpenRPC` method spec document, overriding auto-derivation from `jsonrpc_method`
    pub openrpc_spec: Option<Value>,
}

impl Default for Route {
    fn default() -> Self {
        Self {
            method: Method::Get,
            path: "/".to_string(),
            handler_name: String::new(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: Vec::new(),
            jsonrpc_method: None,
            compression: None,
            body_limit: None,
            request_timeout_secs: None,
            rate_limit: None,
            request_id: None,
            jwt_auth: None,
            api_key_auth: None,
            authorization: None,
            lifecycle_hooks: None,
            openrpc_spec: None,
        }
    }
}

fn is_empty_schema(schema: &Value) -> bool {
    matches!(schema, Value::Object(map) if map.is_empty())
}

impl Route {
    /// Create a route from metadata, using schema registry for deduplication
    ///
    /// Auto-generates parameter schema from type hints in the path if no explicit schema provided.
    /// Type hints like `/items/{id:uuid}` generate appropriate JSON Schema validation.
    /// Explicit `parameter_schema` overrides auto-generated schemas.
    ///
    /// # Errors
    /// Returns an error if the schema compilation fails or metadata is invalid.
    ///
    /// The schema registry ensures each unique schema is compiled only once, improving
    /// startup performance and memory usage for applications with many routes.
    pub fn from_metadata(metadata: RouteMetadata, registry: &SchemaRegistry) -> Result<Self, String> {
        let method = metadata.method.parse()?;

        let request_validator = metadata
            .request_schema
            .as_ref()
            .filter(|schema| !is_empty_schema(schema))
            .map(|schema| registry.get_or_compile(schema))
            .transpose()?;

        let response_validator = metadata
            .response_schema
            .as_ref()
            .filter(|schema| !is_empty_schema(schema))
            .map(|schema| registry.get_or_compile(schema))
            .transpose()?;

        let final_parameter_schema = match (
            crate::type_hints::auto_generate_parameter_schema(&metadata.path),
            metadata.parameter_schema,
        ) {
            (Some(auto_schema), Some(explicit_schema)) => {
                if is_empty_schema(&explicit_schema) {
                    Some(auto_schema)
                } else {
                    Some(crate::type_hints::merge_parameter_schemas(
                        &auto_schema,
                        &explicit_schema,
                    ))
                }
            }
            (Some(auto_schema), None) => Some(auto_schema),
            (None, Some(explicit_schema)) => (!is_empty_schema(&explicit_schema)).then_some(explicit_schema),
            (None, None) => None,
        };

        let parameter_validator = final_parameter_schema.map(ParameterValidator::new).transpose()?;

        let expects_json_body = request_validator.is_some();

        let jsonrpc_method = metadata
            .jsonrpc_method
            .as_ref()
            .and_then(|json_value| serde_json::from_value(json_value.clone()).ok());

        Ok(Self {
            method,
            path: metadata.path,
            handler_name: metadata.handler_name,
            request_validator,
            response_validator,
            parameter_validator,
            file_params: metadata.file_params,
            is_async: metadata.is_async,
            cors: metadata.cors,
            expects_json_body,
            #[cfg(feature = "di")]
            handler_dependencies: metadata.handler_dependencies.unwrap_or_default(),
            jsonrpc_method,
            compression: metadata.compression,
            body_limit: metadata.body_limit,
            request_timeout_secs: metadata.request_timeout_secs,
            rate_limit: metadata.rate_limit,
            request_id: metadata.request_id,
            jwt_auth: metadata.jwt_auth,
            api_key_auth: metadata.api_key_auth,
            authorization: metadata.authorization,
            lifecycle_hooks: metadata.lifecycle_hooks,
            openrpc_spec: metadata.openrpc_spec,
        })
    }

    /// Builder method to attach JSON-RPC method info to a route
    ///
    /// This is a convenient way to add JSON-RPC metadata after route creation.
    /// It consumes the route and returns a new route with the metadata attached.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let route = Route::from_metadata(metadata, &registry)?
    ///     .with_jsonrpc_method(JsonRpcMethodInfo {
    ///         method_name: "user.create".to_string(),
    ///         description: Some("Creates a new user".to_string()),
    ///         params_schema: Some(request_schema),
    ///         result_schema: Some(response_schema),
    ///         deprecated: false,
    ///         tags: vec!["users".to_string()],
    ///     });
    /// ```
    #[must_use]
    pub fn with_jsonrpc_method(mut self, info: JsonRpcMethodInfo) -> Self {
        self.jsonrpc_method = Some(info);
        self
    }

    /// Check if this route has JSON-RPC metadata
    #[must_use]
    pub const fn is_jsonrpc_method(&self) -> bool {
        self.jsonrpc_method.is_some()
    }

    /// Get the JSON-RPC method name if present
    #[must_use]
    pub fn jsonrpc_method_name(&self) -> Option<&str> {
        self.jsonrpc_method.as_ref().map(|m| m.method_name.as_str())
    }
}

#[cfg(test)]
pub(crate) struct Router {
    routes: HashMap<String, HashMap<Method, Route>>,
}

#[cfg(test)]
impl Router {
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    pub fn add_route(&mut self, route: Route) {
        let path_routes = self.routes.entry(route.path.clone()).or_default();
        path_routes.insert(route.method.clone(), route);
    }

    pub fn find_route(&self, method: &Method, path: &str) -> Option<&Route> {
        self.routes.get(path)?.get(method)
    }

    pub fn route_count(&self) -> usize {
        self.routes.values().map(std::collections::HashMap::len).sum()
    }
}

#[cfg(test)]
impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::LifecycleHookRef;
    use serde_json::json;

    #[test]
    fn test_router_add_and_find() {
        let mut router = Router::new();
        let registry = SchemaRegistry::new();

        let metadata = RouteMetadata {
            method: "GET".to_string(),
            path: "/users".to_string(),
            handler_name: "get_users".to_string(),
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            jsonrpc_method: None,
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
            openrpc_spec: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
        };

        let route = Route::from_metadata(metadata, &registry).unwrap();
        router.add_route(route);

        assert_eq!(router.route_count(), 1);
        assert!(router.find_route(&Method::Get, "/users").is_some());
        assert!(router.find_route(&Method::Post, "/users").is_none());
    }

    #[test]
    fn test_route_with_validators() {
        let registry = SchemaRegistry::new();

        let metadata = RouteMetadata {
            method: "POST".to_string(),
            path: "/users".to_string(),
            handler_name: "create_user".to_string(),
            request_schema: Some(json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"}
                },
                "required": ["name"]
            })),
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            jsonrpc_method: None,
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
            openrpc_spec: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
        };

        let route = Route::from_metadata(metadata, &registry).unwrap();
        assert!(route.request_validator.is_some());
        assert!(route.response_validator.is_none());
    }

    #[test]
    fn test_schema_deduplication_in_routes() {
        let registry = SchemaRegistry::new();

        let shared_schema = json!({
            "type": "object",
            "properties": {
                "id": {"type": "integer"}
            }
        });

        let metadata1 = RouteMetadata {
            method: "POST".to_string(),
            path: "/items".to_string(),
            handler_name: "create_item".to_string(),
            request_schema: Some(shared_schema.clone()),
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            jsonrpc_method: None,
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
            openrpc_spec: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
        };

        let metadata2 = RouteMetadata {
            method: "PUT".to_string(),
            path: "/items/{id}".to_string(),
            handler_name: "update_item".to_string(),
            request_schema: Some(shared_schema),
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            jsonrpc_method: None,
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
            openrpc_spec: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
        };

        let route1 = Route::from_metadata(metadata1, &registry).unwrap();
        let route2 = Route::from_metadata(metadata2, &registry).unwrap();

        assert!(route1.request_validator.is_some());
        assert!(route2.request_validator.is_some());

        let validator1 = route1.request_validator.as_ref().unwrap();
        let validator2 = route2.request_validator.as_ref().unwrap();
        assert!(Arc::ptr_eq(validator1, validator2));

        assert_eq!(registry.schema_count(), 1);
    }

    #[test]
    fn test_jsonrpc_method_info() {
        let rpc_info = JsonRpcMethodInfo {
            method_name: "user.create".to_string(),
            description: Some("Creates a new user account".to_string()),
            params_schema: Some(json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "email": {"type": "string"}
                },
                "required": ["name", "email"]
            })),
            result_schema: Some(json!({
                "type": "object",
                "properties": {
                    "id": {"type": "integer"},
                    "name": {"type": "string"},
                    "email": {"type": "string"}
                }
            })),
            deprecated: false,
            tags: vec!["users".to_string(), "admin".to_string()],
        };

        assert_eq!(rpc_info.method_name, "user.create");
        assert_eq!(rpc_info.description.as_ref().unwrap(), "Creates a new user account");
        assert!(rpc_info.params_schema.is_some());
        assert!(rpc_info.result_schema.is_some());
        assert!(!rpc_info.deprecated);
        assert_eq!(rpc_info.tags.len(), 2);
        assert!(rpc_info.tags.contains(&"users".to_string()));
    }

    #[test]
    fn test_route_with_jsonrpc_method() {
        let registry = SchemaRegistry::new();

        let metadata = RouteMetadata {
            method: "POST".to_string(),
            path: "/user/create".to_string(),
            handler_name: "create_user".to_string(),
            request_schema: Some(json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"}
                },
                "required": ["name"]
            })),
            response_schema: Some(json!({
                "type": "object",
                "properties": {
                    "id": {"type": "integer"}
                }
            })),
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            jsonrpc_method: None,
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
            openrpc_spec: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
        };

        let rpc_info = JsonRpcMethodInfo {
            method_name: "user.create".to_string(),
            description: Some("Creates a new user".to_string()),
            params_schema: Some(json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"}
                }
            })),
            result_schema: Some(json!({
                "type": "object",
                "properties": {
                    "id": {"type": "integer"}
                }
            })),
            deprecated: false,
            tags: vec!["users".to_string()],
        };

        let route = Route::from_metadata(metadata, &registry)
            .unwrap()
            .with_jsonrpc_method(rpc_info);

        assert!(route.is_jsonrpc_method());
        assert_eq!(route.jsonrpc_method_name(), Some("user.create"));
        assert!(route.jsonrpc_method.is_some());

        let rpc = route.jsonrpc_method.as_ref().unwrap();
        assert_eq!(rpc.method_name, "user.create");
        assert_eq!(rpc.description.as_ref().unwrap(), "Creates a new user");
        assert!(!rpc.deprecated);
    }

    #[test]
    fn test_jsonrpc_method_serialization() {
        let rpc_info = JsonRpcMethodInfo {
            method_name: "test.method".to_string(),
            description: Some("Test method".to_string()),
            params_schema: Some(json!({"type": "object"})),
            result_schema: Some(json!({"type": "string"})),
            deprecated: false,
            tags: vec!["test".to_string()],
        };

        let json = serde_json::to_value(&rpc_info).unwrap();
        assert_eq!(json["method_name"], "test.method");
        assert_eq!(json["description"], "Test method");

        let deserialized: JsonRpcMethodInfo = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.method_name, rpc_info.method_name);
        assert_eq!(deserialized.description, rpc_info.description);
    }

    #[test]
    fn test_route_without_jsonrpc_method_has_zero_overhead() {
        let registry = SchemaRegistry::new();

        let metadata = RouteMetadata {
            method: "GET".to_string(),
            path: "/status".to_string(),
            handler_name: "status".to_string(),
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: false,
            cors: None,
            body_param_name: None,
            jsonrpc_method: None,
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
            openrpc_spec: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
        };

        let route = Route::from_metadata(metadata, &registry).unwrap();

        assert!(!route.is_jsonrpc_method());
        assert_eq!(route.jsonrpc_method_name(), None);
        assert!(route.jsonrpc_method.is_none());
    }

    #[test]
    fn test_route_carries_rate_limit_and_request_id_from_metadata() {
        let registry = SchemaRegistry::new();
        let mut metadata = RouteMetadata {
            path: "/limited".to_string(),
            handler_name: "get_limited".to_string(),
            ..Default::default()
        };
        metadata.rate_limit = Some(RateLimitConfig {
            per_second: 5,
            burst: 10,
            ip_based: false,
        });
        metadata.request_id = Some(RequestIdConfig { enabled: true });

        let route = Route::from_metadata(metadata, &registry).unwrap();

        let rate_limit = route.rate_limit.as_ref().unwrap();
        assert_eq!(rate_limit.per_second, 5);
        assert_eq!(rate_limit.burst, 10);
        assert!(!rate_limit.ip_based);
        assert_eq!(route.request_id, Some(RequestIdConfig { enabled: true }));
    }

    #[test]
    fn should_carry_typed_jwt_auth_and_api_key_auth_through_route_construction() {
        let registry = SchemaRegistry::new();
        let mut metadata = RouteMetadata {
            path: "/secure".to_string(),
            handler_name: "get_secure".to_string(),
            ..Default::default()
        };
        metadata.jwt_auth = Some(JwtAuthConfig {
            enabled: true,
            secret: Some("top-secret".to_string()),
            public_key: None,
            algorithm: "HS512".to_string(),
            audience: None,
            issuer: None,
            leeway: 0,
        });
        metadata.api_key_auth = Some(ApiKeyAuthConfig {
            enabled: true,
            keys: vec!["k1".to_string(), "k2".to_string()],
            header_name: "X-API-Key".to_string(),
        });

        let route = Route::from_metadata(metadata, &registry).unwrap();

        let jwt_auth = route.jwt_auth.as_ref().unwrap();
        assert_eq!(jwt_auth.secret.as_deref(), Some("top-secret"));
        assert_eq!(jwt_auth.algorithm, "HS512");

        let api_key_auth = route.api_key_auth.as_ref().unwrap();
        assert_eq!(api_key_auth.keys, vec!["k1".to_string(), "k2".to_string()]);
        assert_eq!(api_key_auth.header_name, "X-API-Key");
    }

    /// `RouteMetadata`'s per-route middleware fields are typed structs now, so a malformed
    /// payload fails at `RouteMetadata` deserialization rather than at `Route::from_metadata`
    /// (see `spikard-core/src/http.rs`'s `JwtAuthConfig` tests for the field-level cases). This
    /// proves that failure is still reachable end to end from raw JSON.
    #[test]
    fn should_error_when_deserializing_route_metadata_with_malformed_jwt_auth_audience() {
        let mut json_value = json!(RouteMetadata {
            path: "/broken".to_string(),
            handler_name: "get_broken".to_string(),
            ..Default::default()
        });
        json_value["jwt_auth"] = json!({ "secret": "s3cr3t", "audience": "not-an-array" });

        let result: Result<RouteMetadata, _> = serde_json::from_value(json_value);
        assert!(result.is_err());
    }

    #[test]
    fn should_carry_typed_authorization_and_lifecycle_hooks_through_route_construction() {
        let registry = SchemaRegistry::new();
        let mut metadata = RouteMetadata {
            path: "/admin".to_string(),
            handler_name: "get_admin".to_string(),
            ..Default::default()
        };
        metadata.authorization = Some(AuthorizationConfig {
            required_roles: vec!["admin".to_string()],
            require_all: false,
            ..Default::default()
        });
        metadata.lifecycle_hooks = Some(LifecycleHooksConfig {
            on_request: vec![LifecycleHookRef {
                name: "audit_log".to_string(),
                handler: "run_audit_log".to_string(),
                dependencies: vec![],
                config: None,
                ..Default::default()
            }],
            on_error: vec![LifecycleHookRef {
                name: "alert".to_string(),
                handler: "run_alert".to_string(),
                dependencies: vec![],
                config: None,
                ..Default::default()
            }],
            ..Default::default()
        });

        let route = Route::from_metadata(metadata, &registry).unwrap();

        let authorization = route.authorization.as_ref().unwrap();
        assert_eq!(authorization.required_roles, vec!["admin".to_string()]);
        assert!(!authorization.require_all);

        let lifecycle_hooks = route.lifecycle_hooks.as_ref().unwrap();
        assert_eq!(lifecycle_hooks.on_request[0].name, "audit_log");
        assert_eq!(lifecycle_hooks.on_error[0].name, "alert");
        assert!(lifecycle_hooks.pre_handler.is_empty());
    }

    #[test]
    fn test_route_carries_literal_openrpc_spec_from_metadata() {
        let registry = SchemaRegistry::new();
        let spec = json!({ "name": "user.create", "params": [], "result": { "name": "user", "schema": {} } });
        let mut metadata = RouteMetadata {
            path: "/rpc".to_string(),
            handler_name: "post_rpc".to_string(),
            ..Default::default()
        };
        metadata.openrpc_spec = Some(spec.clone());

        let route = Route::from_metadata(metadata, &registry).unwrap();

        assert_eq!(route.openrpc_spec, Some(spec));
    }
}
