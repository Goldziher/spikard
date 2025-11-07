//! Analyze fixtures and extract route definitions

use crate::fixture::{Fixture, Handler, Parameters};
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

    /// Example fixtures using this route
    pub example_fixtures: Vec<String>,

    /// How many fixtures use this route
    pub fixture_count: usize,
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
    let mut route_map: IndexMap<RouteSignature, (Parameters, Vec<String>)> = IndexMap::new();
    let mut by_method: IndexMap<String, usize> = IndexMap::new();
    let mut by_category: IndexMap<String, usize> = IndexMap::new();

    for fixture in fixtures {
        // Count by method
        *by_method.entry(fixture.handler.method.clone()).or_insert(0) += 1;

        // Count by category
        if let Some(cat) = &fixture.category {
            *by_category.entry(cat.clone()).or_insert(0) += 1;
        }

        // Group by route signature (route + method only)
        let sig = RouteSignature {
            route: normalize_route(&fixture.handler.route),
            method: fixture.handler.method.clone(),
        };

        route_map
            .entry(sig)
            .or_insert_with(|| (fixture.handler.parameters.clone(), Vec::new()))
            .1
            .push(fixture.name.clone());
    }

    // Convert to RouteInfo
    let routes: Vec<RouteInfo> = route_map
        .into_iter()
        .map(|(sig, (params, example_fixtures))| RouteInfo {
            route: sig.route,
            method: sig.method,
            params,
            fixture_count: example_fixtures.len(),
            example_fixtures: example_fixtures.into_iter().take(3).collect(),
        })
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
    // Convert FastAPI :param to Axum {param}
    // Also strip type hints: {param:int} -> {param}
    let mut result = route.to_string();

    if result.contains(':') || result.contains('{') {
        result = result
            .split('/')
            .map(|segment| {
                // Handle FastAPI style :param
                if let Some(param) = segment.strip_prefix(':') {
                    format!("{{{}}}", param)
                }
                // Handle Axum/FastAPI style {param:type}
                else if segment.starts_with('{') && segment.ends_with('}') {
                    let inner = &segment[1..segment.len() - 1];
                    if let Some(colon_pos) = inner.find(':') {
                        // Strip type hint
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

/// Extract path parameter names from a route pattern
pub fn extract_path_params(route: &str) -> Vec<String> {
    route
        .split('/')
        .filter_map(|segment| {
            if segment.starts_with('{') && segment.ends_with('}') {
                let inner = &segment[1..segment.len() - 1];
                // Strip type hints
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
