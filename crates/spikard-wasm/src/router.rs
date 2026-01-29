//! Route matching using spikard-core compiled routes.

use crate::adapter::InternalRequest;
use std::sync::OnceLock;

/// Matched route result.
pub struct RouteMatch {
    pub handler_id: String,
    pub path_params: Vec<(String, String)>,
}

/// Simple router that matches HTTP method + path to handlers.
pub struct Router {
    routes: Vec<CompiledRoute>,
}

struct CompiledRoute {
    method: String,
    path_pattern: String,
    segments: Vec<Segment>,
    handler_id: String,
}

enum Segment {
    Literal(String),
    Param(String),
}

impl Router {
    /// Get the global router instance (initialized once).
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<Router> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    const fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Register a route.
    pub fn add_route(&mut self, method: &str, path: &str, handler_id: &str) {
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.strip_prefix('{')
                    .and_then(|s| s.strip_suffix('}'))
                    .map_or_else(
                        || Segment::Literal(s.to_string()),
                        |param| Segment::Param(param.to_string()),
                    )
            })
            .collect();

        self.routes.push(CompiledRoute {
            method: method.to_uppercase(),
            path_pattern: path.to_string(),
            segments,
            handler_id: handler_id.to_string(),
        });
    }

    /// Dispatch a request to the matching handler.
    pub fn dispatch(&self, request: &InternalRequest) -> HandlerResult {
        let path_segments: Vec<&str> = request.path.split('/').filter(|s| !s.is_empty()).collect();

        for route in &self.routes {
            if route.method != request.method {
                continue;
            }
            if route.segments.len() != path_segments.len() {
                continue;
            }

            let mut params = Vec::new();
            let mut matched = true;

            for (seg, actual) in route.segments.iter().zip(path_segments.iter()) {
                match seg {
                    Segment::Literal(lit) if lit == actual => {}
                    Segment::Param(name) => {
                        params.push((name.clone(), (*actual).to_string()));
                    }
                    Segment::Literal(_) => {
                        matched = false;
                        break;
                    }
                }
            }

            if matched {
                return HandlerResult {
                    status: 200,
                    headers: vec![("content-type".to_string(), "application/json".to_string())],
                    body: serde_json::to_vec(&serde_json::json!({
                        "matched": route.handler_id,
                        "path_params": params.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect::<Vec<_>>(),
                    }))
                    .unwrap_or_default(),
                };
            }
        }

        HandlerResult {
            status: 404,
            headers: vec![("content-type".to_string(), "application/json".to_string())],
            body: serde_json::to_vec(&serde_json::json!({
                "error": "Not Found",
                "code": 404,
            }))
            .unwrap_or_default(),
        }
    }
}

/// Result of handling a request.
pub struct HandlerResult {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}
