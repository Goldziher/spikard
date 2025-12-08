//! Route management and handler registration

use crate::parameters::ParameterValidator;
use crate::schema_registry::SchemaRegistry;
use crate::validation::SchemaValidator;
use crate::{CorsConfig, Method, RouteMetadata};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Handler function type (placeholder - will be enhanced with Python callbacks)
pub type RouteHandler = Arc<dyn Fn() -> String + Send + Sync>;

/// Route definition with compiled validators
///
/// Validators are Arc-wrapped to enable cheap cloning across route instances
/// and to support schema deduplication via SchemaRegistry.
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
}

impl Route {
    /// Create a route from metadata, using schema registry for deduplication
    ///
    /// Auto-generates parameter schema from type hints in the path if no explicit schema provided.
    /// Type hints like `/items/{id:uuid}` generate appropriate JSON Schema validation.
    /// Explicit parameter_schema overrides auto-generated schemas.
    ///
    /// The schema registry ensures each unique schema is compiled only once, improving
    /// startup performance and memory usage for applications with many routes.
    pub fn from_metadata(metadata: RouteMetadata, registry: &SchemaRegistry) -> Result<Self, String> {
        let method = metadata.method.parse()?;

        let request_validator = metadata
            .request_schema
            .as_ref()
            .map(|schema| registry.get_or_compile(schema))
            .transpose()?;

        let response_validator = metadata
            .response_schema
            .as_ref()
            .map(|schema| registry.get_or_compile(schema))
            .transpose()?;

        let final_parameter_schema = match (
            crate::type_hints::auto_generate_parameter_schema(&metadata.path),
            metadata.parameter_schema,
        ) {
            (Some(auto_schema), Some(explicit_schema)) => {
                Some(crate::type_hints::merge_parameter_schemas(auto_schema, explicit_schema))
            }
            (Some(auto_schema), None) => Some(auto_schema),
            (None, Some(explicit_schema)) => Some(explicit_schema),
            (None, None) => None,
        };

        let parameter_validator = final_parameter_schema.map(ParameterValidator::new).transpose()?;

        let expects_json_body = request_validator.is_some();

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
        })
    }
}

/// Router that manages routes
pub struct Router {
    routes: HashMap<String, HashMap<Method, Route>>,
}

impl Router {
    /// Create a new router
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    /// Add a route to the router
    pub fn add_route(&mut self, route: Route) {
        let path_routes = self.routes.entry(route.path.clone()).or_default();
        path_routes.insert(route.method.clone(), route);
    }

    /// Find a route by method and path
    pub fn find_route(&self, method: &Method, path: &str) -> Option<&Route> {
        self.routes.get(path)?.get(method)
    }

    /// Get all routes
    pub fn routes(&self) -> Vec<&Route> {
        self.routes.values().flat_map(|methods| methods.values()).collect()
    }

    /// Get route count
    pub fn route_count(&self) -> usize {
        self.routes.values().map(|m| m.len()).sum()
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
}
