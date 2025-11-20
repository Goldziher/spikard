//! Analyze fixtures and extract route definitions

use crate::fixture::{Fixture, Handler, MiddlewareConfig, Parameters};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// A unique route handler signature
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RouteSignature {
    pub route: String,
    pub method: String,
}

/// Analysis result containing unique routes
#[derive(Debug, Clone, Serialize)]
pub struct RouteAnalysis {
    /// Unique route handlers needed
    pub routes: Vec<RouteInfo>,

    /// Statistics
    pub stats: AnalysisStats,
}

#[derive(Debug, Clone, Serialize)]
pub struct RouteInfo {
    /// Route pattern (e.g., "/users/{id}")
    pub route: String,

    /// HTTP method
    pub method: String,

    /// Parameter definitions
    pub params: Parameters,

    /// Middleware configuration
    pub middleware: Option<MiddlewareConfig>,

    /// Example fixtures using this route
    pub example_fixtures: Vec<String>,

    /// How many fixtures use this route
    pub fixture_count: usize,
}

impl RouteInfo {
    pub fn has_middleware(&self) -> bool {
        self.middleware.is_some()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AnalysisStats {
    pub total_fixtures: usize,
    pub unique_routes: usize,
    pub by_method: IndexMap<String, usize>,
    pub by_category: IndexMap<String, usize>,
}

/// Analyze fixtures and extract unique route signatures
pub fn analyze_fixtures(fixtures: &[Fixture]) -> RouteAnalysis {
    let mut route_map: IndexMap<RouteSignature, (String, Parameters, Option<MiddlewareConfig>, Vec<String>)> =
        IndexMap::new();
    let mut canonical_paths: IndexMap<String, String> = IndexMap::new();
    let mut method_canonical_seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut by_method: IndexMap<String, usize> = IndexMap::new();
    let mut by_category: IndexMap<String, usize> = IndexMap::new();

    for fixture in fixtures {
        *by_method.entry(fixture.handler.method.clone()).or_insert(0) += 1;

        if let Some(cat) = &fixture.category {
            *by_category.entry(cat.clone()).or_insert(0) += 1;
        }

        let normalized = normalize_route(&fixture.handler.route);
        let canonical = canonicalize_route(&normalized);

        // Use just the canonical form (without method) to ensure all methods
        // for the same path pattern use consistent parameter names
        // e.g., PATCH /items/{id} and GET /items/{item_id} both use /items/{id}
        let consistent_normalized = canonical_paths
            .entry(canonical.clone())
            .or_insert_with(|| normalized.clone())
            .clone();

        // Skip if we've already seen this method + canonical combination
        // e.g., skip second POST /{} (whether it's /{id} or /{lang})
        let method_canonical_key = format!("{} {}", fixture.handler.method, canonical);
        if method_canonical_seen.contains(&method_canonical_key) {
            continue;
        }
        method_canonical_seen.insert(method_canonical_key);

        let sig = RouteSignature {
            route: consistent_normalized.clone(),
            method: fixture.handler.method.clone(),
        };

        route_map
            .entry(sig)
            .or_insert_with(|| {
                (
                    consistent_normalized,
                    fixture.handler.parameters.clone(),
                    fixture.handler.middleware.clone(),
                    Vec::new(),
                )
            })
            .3
            .push(fixture.name.clone());
    }

    let routes: Vec<RouteInfo> = route_map
        .into_iter()
        .map(
            |(sig, (normalized_route, params, middleware, example_fixtures))| RouteInfo {
                route: normalized_route,
                method: sig.method,
                params,
                middleware,
                fixture_count: example_fixtures.len(),
                example_fixtures: example_fixtures.into_iter().take(3).collect(),
            },
        )
        .collect();

    RouteAnalysis {
        stats: AnalysisStats {
            total_fixtures: fixtures.len(),
            unique_routes: routes.len(),
            by_method,
            by_category,
        },
        routes,
    }
}

/// Normalize route patterns to Axum format and extract path parameter names
fn normalize_route(route: &str) -> String {
    let mut result = route.to_string();

    if result.contains(':') || result.contains('{') {
        result = result
            .split('/')
            .map(|segment| {
                if let Some(param) = segment.strip_prefix(':') {
                    format!("{{{}}}", param)
                } else if segment.starts_with('{') && segment.ends_with('}') {
                    let inner = &segment[1..segment.len() - 1];
                    if let Some(colon_pos) = inner.find(':') {
                        format!("{{{}}}", &inner[..colon_pos])
                    } else {
                        segment.to_string()
                    }
                } else {
                    segment.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("/");
    }

    result
}

/// Create a canonical route pattern for deduplication
/// Replaces all path parameter names with a generic placeholder to detect conflicts
/// e.g. "/items/{id}" and "/items/{item_id}" both become "/items/{}"
fn canonicalize_route(route: &str) -> String {
    route
        .split('/')
        .map(|segment| {
            if segment.starts_with('{') && segment.ends_with('}') {
                "{}".to_string()
            } else {
                segment.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

/// Extract path parameter names from a route pattern
pub fn extract_path_params(route: &str) -> Vec<String> {
    route
        .split('/')
        .filter_map(|segment| {
            if segment.starts_with('{') && segment.ends_with('}') {
                let inner = &segment[1..segment.len() - 1];
                let param_name = if let Some(colon_pos) = inner.find(':') {
                    &inner[..colon_pos]
                } else {
                    inner
                };
                Some(param_name.to_string())
            } else if segment.starts_with(':') {
                Some(segment[1..].to_string())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_route() {
        assert_eq!(normalize_route("/users/:id"), "/users/{id}");
        assert_eq!(normalize_route("/api/:version/users/:id"), "/api/{version}/users/{id}");
        assert_eq!(normalize_route("/users/{id}"), "/users/{id}");
        assert_eq!(normalize_route("/health"), "/health");
    }
}
