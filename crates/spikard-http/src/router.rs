//! Route management and handler registration

use crate::validation::SchemaValidator;
use crate::{Method, RouteMetadata};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Handler function type (placeholder - will be enhanced with Python callbacks)
pub type RouteHandler = Arc<dyn Fn() -> String + Send + Sync>;

/// Route definition with compiled validators
#[derive(Clone)]
pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler_name: String,
    pub request_validator: Option<SchemaValidator>,
    pub response_validator: Option<SchemaValidator>,
    pub is_async: bool,
}

impl Route {
    /// Create a route from metadata
    pub fn from_metadata(metadata: RouteMetadata) -> Result<Self, String> {
        let method = metadata.method.parse()?;

        let request_validator = metadata
            .request_schema
            .map(|schema| SchemaValidator::new(schema))
            .transpose()?;

        let response_validator = metadata
            .response_schema
            .map(|schema| SchemaValidator::new(schema))
            .transpose()?;

        Ok(Self {
            method,
            path: metadata.path,
            handler_name: metadata.handler_name,
            request_validator,
            response_validator,
            is_async: metadata.is_async,
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
        Self {
            routes: HashMap::new(),
        }
    }

    /// Add a route to the router
    pub fn add_route(&mut self, route: Route) {
        let path_routes = self.routes.entry(route.path.clone()).or_insert_with(HashMap::new);
        path_routes.insert(route.method.clone(), route);
    }

    /// Find a route by method and path
    pub fn find_route(&self, method: &Method, path: &str) -> Option<&Route> {
        self.routes.get(path)?.get(method)
    }

    /// Get all routes
    pub fn routes(&self) -> Vec<&Route> {
        self.routes
            .values()
            .flat_map(|methods| methods.values())
            .collect()
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

        let metadata = RouteMetadata {
            method: "GET".to_string(),
            path: "/users".to_string(),
            handler_name: "get_users".to_string(),
            request_schema: None,
            response_schema: None,
            is_async: true,
        };

        let route = Route::from_metadata(metadata).unwrap();
        router.add_route(route);

        assert_eq!(router.route_count(), 1);
        assert!(router.find_route(&Method::Get, "/users").is_some());
        assert!(router.find_route(&Method::Post, "/users").is_none());
    }

    #[test]
    fn test_route_with_validators() {
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
            is_async: true,
        };

        let route = Route::from_metadata(metadata).unwrap();
        assert!(route.request_validator.is_some());
        assert!(route.response_validator.is_none());
    }
}
