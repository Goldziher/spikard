//! Route management and handler registration

use crate::parameters::ParameterValidator;
use crate::schema_registry::SchemaRegistry;
use crate::validation::SchemaValidator;
use crate::{CorsConfig, Method, RouteMetadata};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Handler function type (placeholder - will be enhanced with Python callbacks)
pub type RouteHandler = Arc<dyn Fn() -> String + Send + Sync>;

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
    pub params_schema: Option<Value>,

    /// Optional JSON Schema for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_schema: Option<Value>,

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
    #[allow(clippy::items_after_statements)]
    pub fn from_metadata(metadata: RouteMetadata, registry: &SchemaRegistry) -> Result<Self, String> {
        let method = metadata.method.parse()?;

        fn is_empty_schema(schema: &Value) -> bool {
            matches!(schema, Value::Object(map) if map.is_empty())
        }

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

/// Router that manages routes
pub struct Router {
    routes: HashMap<String, HashMap<Method, Route>>,
}

impl Router {
    /// Create a new router
    #[must_use]
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    /// Add a route to the router
    pub fn add_route(&mut self, route: Route) {
        let path_routes = self.routes.entry(route.path.clone()).or_default();
        path_routes.insert(route.method.clone(), route);
    }

    /// Find a route by method and path
    #[must_use]
    pub fn find_route(&self, method: &Method, path: &str) -> Option<&Route> {
        self.routes.get(path)?.get(method)
    }

    /// Get all routes
    #[must_use]
    pub fn routes(&self) -> Vec<&Route> {
        self.routes.values().flat_map(|methods| methods.values()).collect()
    }

    /// Get route count
    #[must_use]
    pub fn route_count(&self) -> usize {
        self.routes.values().map(std::collections::HashMap::len).sum()
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            #[cfg(feature = "di")]
            handler_dependencies: None,
        };

        let route = Route::from_metadata(metadata, &registry).unwrap();

        assert!(!route.is_jsonrpc_method());
        assert_eq!(route.jsonrpc_method_name(), None);
        assert!(route.jsonrpc_method.is_none());
    }
}
