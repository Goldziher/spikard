#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::doc_markdown,
    clippy::too_long_first_doc_paragraph,
    clippy::module_name_repetitions
)]
//! SQL-driven HTTP handler generation.
//!
//! Consumes scythe's `AnalyzedQuery` IR (plus the `Catalog` it was analyzed
//! against) and emits routes, JSON Schema validators, and an OpenAPI 3.1 spec.
//! The HTTP vocabulary (`@http`, `@http_param`, `@http_auth`, …) lives
//! entirely in this module — scythe knows nothing about HTTP. Spikard parses
//! them out of `AnalyzedQuery.custom`.
//!
//! Pipeline:
//!
//! ```text
//!   AnalyzedQuery.custom  ─►  parse_http_annotations  ─►  HttpAnnotations
//!   AnalyzedQuery (+ Catalog) ─►  route_from_query   ─►  RouteMetadata
//!   [RouteMetadata + AnalyzedQuery]  ─►  openapi_from_routes  ─►  Value (OpenAPI 3.1)
//!   [AnalyzedQuery]  ─►  build_sidecar  ─►  Sidecar  (per-language call info)
//! ```

pub mod annotations;
pub mod neutral_to_json_schema;
pub mod openapi;
pub mod route;
pub mod sidecar;

pub use annotations::{
    AnnotationParseError, ApiKeyLocation, AuthRequirement, HttpAnnotations, HttpMethod, HttpParamBinding,
    parse_http_annotations,
};
pub use neutral_to_json_schema::{BuildOptions, DecimalMode, neutral_to_json_schema};
pub use openapi::{OpenApiInfo, openapi_from_routes};
pub use route::{RouteBuildError, SqlRoute, route_from_query};
pub use sidecar::{Sidecar, SidecarEntry, SidecarParam};

use scythe_core::analyzer::AnalyzedQuery;
use scythe_core::catalog::Catalog;
use serde_json::Value;

/// Aggregate output of [`build_handler_set`]: routes + OpenAPI spec + sidecar.
#[derive(Debug, Clone)]
pub struct HandlerSet {
    /// One JSON `RouteMetadata` value per HTTP-annotated query.
    pub routes: Vec<Value>,
    /// The same routes, paired with their HTTP annotations and command —
    /// useful for callers that need to wire OpenAPI emission or sidecar entries
    /// without re-parsing.
    pub sql_routes: Vec<SqlRoute>,
    /// OpenAPI 3.1 spec built from `sql_routes`.
    pub openapi: Value,
    /// Per-language call info for handler-stub generators.
    pub sidecar: Sidecar,
}

/// Walk a slice of analyzed queries and build the full handler set: routes,
/// OpenAPI spec, and a per-language sidecar. Queries without an `@http`
/// directive are skipped silently. The sidecar is populated by passing per-
/// language `lang_type_for` resolvers in `languages`.
///
/// `languages` maps language name → `(scythe_module, scythe_fn_for, is_async,
/// lang_type_for)` tuples. The closures are evaluated once per query, giving
/// callers full control over per-language naming and type resolution without
/// pulling scythe's backend trait into this crate's surface.
pub fn build_handler_set(
    catalog: &Catalog,
    queries: &[AnalyzedQuery],
    info: &OpenApiInfo,
    opts: &BuildOptions,
    languages: &[LanguageBackend<'_>],
) -> Result<HandlerSet, RouteBuildError> {
    let mut sql_routes = Vec::new();
    let mut routes = Vec::new();

    for query in queries {
        let Some(route) = route_from_query(query, catalog, opts)? else {
            continue;
        };
        routes.push(route.metadata.clone());
        sql_routes.push(route);
    }

    let openapi = openapi_from_routes(&sql_routes, info);

    let mut sidecar = Sidecar::new();
    for backend in languages {
        for (route, query) in sql_routes.iter().zip(matching_queries(queries, &sql_routes)) {
            let scythe_fn = (backend.scythe_fn_for)(&query.name);
            let entry = sidecar::build_sidecar_entry(
                query,
                &route.param_locations,
                backend.scythe_module,
                &scythe_fn,
                backend.is_async,
                |neutral, nullable| (backend.lang_type_for)(neutral, nullable),
            );
            sidecar.insert(backend.name, &route.operation_id, entry);
        }
    }

    Ok(HandlerSet {
        routes,
        sql_routes,
        openapi,
        sidecar,
    })
}

/// Per-language inputs to [`build_handler_set`]. Each backend names itself
/// (used as the sidecar key) and supplies callbacks that translate scythe
/// metadata into language-native names and types.
pub struct LanguageBackend<'a> {
    pub name: &'a str,
    pub scythe_module: &'a str,
    pub is_async: bool,
    pub scythe_fn_for: &'a dyn Fn(&str) -> String,
    pub lang_type_for: &'a dyn Fn(&str, bool) -> String,
}

fn matching_queries<'a>(queries: &'a [AnalyzedQuery], routes: &[SqlRoute]) -> Vec<&'a AnalyzedQuery> {
    routes
        .iter()
        .filter_map(|r| queries.iter().find(|q| q.name == r.operation_id))
        .collect()
}

#[cfg(test)]
mod orchestrator_tests {
    use super::*;
    use scythe_core::analyzer::{AnalyzedColumn, AnalyzedParam, AnalyzedQuery};
    use scythe_core::parser::{CustomAnnotation, QueryCommand};

    fn empty_catalog() -> Catalog {
        Catalog::from_ddl(&[]).unwrap()
    }

    fn snake_for(name: &str) -> String {
        let mut out = String::new();
        let mut prev_lower = false;
        for c in name.chars() {
            if c.is_ascii_uppercase() {
                if prev_lower {
                    out.push('_');
                }
                out.push(c.to_ascii_lowercase());
                prev_lower = false;
            } else {
                out.push(c);
                prev_lower = c.is_ascii_lowercase() || c.is_ascii_digit();
            }
        }
        out
    }

    fn py_type(neutral: &str, nullable: bool) -> String {
        let base = match neutral {
            "int32" | "int64" | "int16" => "int",
            "string" => "str",
            "bool" => "bool",
            _ => "Any",
        };
        if nullable {
            format!("{base} | None")
        } else {
            base.to_string()
        }
    }

    fn get_user() -> AnalyzedQuery {
        AnalyzedQuery {
            name: "GetUser".to_string(),
            command: QueryCommand::One,
            sql: "SELECT id FROM users WHERE id = $1".into(),
            columns: vec![AnalyzedColumn {
                name: "id".into(),
                neutral_type: "int64".into(),
                nullable: false,
            }],
            params: vec![AnalyzedParam {
                name: "id".into(),
                neutral_type: "int64".into(),
                nullable: false,
                position: 1,
            }],
            deprecated: None,
            source_table: Some("users".into()),
            composites: vec![],
            enums: vec![],
            optional_params: vec![],
            group_by: None,
            custom: vec![CustomAnnotation {
                name: "http".into(),
                value: "GET /users/{id}".into(),
                line: 1,
            }],
        }
    }

    fn no_http() -> AnalyzedQuery {
        AnalyzedQuery {
            name: "InternalQuery".to_string(),
            command: QueryCommand::One,
            sql: "SELECT 1".into(),
            columns: vec![],
            params: vec![],
            deprecated: None,
            source_table: None,
            composites: vec![],
            enums: vec![],
            optional_params: vec![],
            group_by: None,
            custom: vec![],
        }
    }

    #[test]
    fn skips_queries_without_http_directive() {
        let queries = vec![get_user(), no_http()];
        let set = build_handler_set(
            &empty_catalog(),
            &queries,
            &OpenApiInfo::new("t", "0.1"),
            &BuildOptions::default(),
            &[],
        )
        .unwrap();
        assert_eq!(set.routes.len(), 1);
        assert_eq!(set.sql_routes.len(), 1);
        assert_eq!(set.sql_routes[0].operation_id, "GetUser");
    }

    #[test]
    fn populates_sidecar_per_language() {
        let queries = vec![get_user()];
        let snake = |s: &str| snake_for(s);
        let set = build_handler_set(
            &empty_catalog(),
            &queries,
            &OpenApiInfo::new("t", "0.1"),
            &BuildOptions::default(),
            &[LanguageBackend {
                name: "python",
                scythe_module: "queries",
                is_async: true,
                scythe_fn_for: &snake,
                lang_type_for: &py_type,
            }],
        )
        .unwrap();
        let entry = set.sidecar.entry_for("python", "GetUser").unwrap();
        assert_eq!(entry.scythe_module, "queries");
        assert_eq!(entry.scythe_fn, "get_user");
        assert!(entry.is_async);
    }

    #[test]
    fn openapi_emitted_in_set() {
        let queries = vec![get_user()];
        let set = build_handler_set(
            &empty_catalog(),
            &queries,
            &OpenApiInfo::new("t", "0.1"),
            &BuildOptions::default(),
            &[],
        )
        .unwrap();
        assert_eq!(set.openapi["openapi"], "3.1.0");
        assert!(set.openapi["paths"]["/users/{id}"]["get"].is_object());
    }
}
