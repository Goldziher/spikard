//! Build `spikard_core::RouteMetadata` from a scythe `AnalyzedQuery`.

use std::collections::BTreeMap;

use scythe_core::analyzer::AnalyzedQuery;
use scythe_core::catalog::Catalog;
use scythe_core::parser::QueryCommand;
use serde_json::{Map, Value, json};
use thiserror::Error;

use super::annotations::{
    AnnotationParseError, HttpAnnotations, HttpMethod, HttpParamBinding, default_status_for, parse_http_annotations,
};
use super::neutral_to_json_schema::{BuildOptions, NeutralTypeError, json_schema_for};

#[derive(Debug, Error)]
pub enum RouteBuildError {
    #[error("annotation error: {0}")]
    Annotation(#[from] AnnotationParseError),

    #[error("neutral type error: {0}")]
    NeutralType(#[from] NeutralTypeError),
}

/// One spikard route plus the SQL command and HTTP semantics needed to wire it
/// up. Returned as a single value so callers don't lose the join between the
/// route's identity (path/method/handler name) and the query metadata that
/// produced it.
#[derive(Debug, Clone)]
pub struct SqlRoute {
    /// `RouteMetadata` shape spikard-core consumes. Stored as JSON to avoid a
    /// hard dep on `spikard-core` from `spikard-codegen`; callers (the CLI)
    /// deserialize into the concrete type at the boundary.
    pub metadata: Value,
    /// HTTP semantics that built the route — preserved so the OpenAPI emitter
    /// and sidecar builder don't have to re-parse `query.custom`.
    pub http: HttpAnnotations,
    /// Mapping from SQL param name to its HTTP source. Combines explicit
    /// `@http_param` overrides with the inference rules in [`bin_param_locations`].
    pub param_locations: BTreeMap<String, HttpParamBinding>,
    /// Status code chosen for the default response (from `@http_status` or
    /// derived from the SQL command).
    pub default_status: u16,
    /// Bundle name for the body object when multiple body params exist.
    pub body_bundle_name: String,
    /// `operation_id` used in OpenAPI (`PascalCase`, taken from `@name`).
    pub operation_id: String,
    /// Handler name in generated code (`snake_case`, `handle_<name>`).
    pub handler_name: String,
}

/// Build a `RouteMetadata` (as JSON) from one analyzed query. Returns
/// `Ok(None)` when the query has no `@http` directive — those are skipped
/// silently so SQL files can mix HTTP and non-HTTP queries freely.
pub fn route_from_query(
    query: &AnalyzedQuery,
    catalog: &Catalog,
    opts: &BuildOptions,
) -> Result<Option<SqlRoute>, RouteBuildError> {
    let Some(http) = parse_http_annotations(&query.custom)? else {
        return Ok(None);
    };
    let default_status = default_status_for(&query.command, http.method)?;

    let param_locations = bin_param_locations(query, &http);
    let body_bundle_name = http.request_body_name.clone().unwrap_or_else(|| "payload".to_string());

    let parameter_schema = build_parameter_schema(query, &param_locations, catalog, opts)?;
    let request_schema = build_request_schema(query, &param_locations, &body_bundle_name, catalog, opts)?;
    let response_schema = build_response_schema(query, catalog, opts)?;

    let handler_name = format!("handle_{}", to_snake_case(&query.name));
    let operation_id = query.name.clone();

    let body_param_name = single_body_param(query, &param_locations).map(str::to_string);
    let expects_json_body = matches!(http.method, HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch)
        && param_locations.values().any(|v| *v == HttpParamBinding::Body);

    let mut metadata = Map::new();
    metadata.insert("method".into(), json!(http.method.as_str()));
    metadata.insert("path".into(), json!(&http.path));
    metadata.insert("handler_name".into(), json!(&handler_name));
    metadata.insert("request_schema".into(), request_schema);
    metadata.insert("response_schema".into(), response_schema);
    metadata.insert("parameter_schema".into(), parameter_schema);
    metadata.insert("is_async".into(), json!(true));
    metadata.insert("expects_json_body".into(), json!(expects_json_body));
    if let Some(body_name) = body_param_name {
        metadata.insert("body_param_name".into(), json!(body_name));
    }

    Ok(Some(SqlRoute {
        metadata: Value::Object(metadata),
        http,
        param_locations,
        default_status,
        body_bundle_name,
        operation_id,
        handler_name,
    }))
}

/// Decide where each `AnalyzedParam` is sourced from, falling back from
/// explicit `@http_param` overrides to inference rules:
/// 1. explicit binding wins,
/// 2. name appears as `{name}` in path → `path`,
/// 3. GET/DELETE → `query`,
/// 4. POST/PUT/PATCH → `body`.
pub fn bin_param_locations(query: &AnalyzedQuery, http: &HttpAnnotations) -> BTreeMap<String, HttpParamBinding> {
    let path_segments: Vec<&str> = extract_path_params(&http.path);
    let mut bindings = BTreeMap::new();
    for p in &query.params {
        if let Some(explicit) = http.param_bindings.get(&p.name) {
            bindings.insert(p.name.clone(), *explicit);
            continue;
        }
        if path_segments.iter().any(|s| *s == p.name) {
            bindings.insert(p.name.clone(), HttpParamBinding::Path);
            continue;
        }
        let inferred = match http.method {
            HttpMethod::Get | HttpMethod::Delete | HttpMethod::Head | HttpMethod::Options => HttpParamBinding::Query,
            HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch => HttpParamBinding::Body,
        };
        bindings.insert(p.name.clone(), inferred);
    }
    bindings
}

fn extract_path_params(path: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let bytes = path.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            let start = i + 1;
            while i < bytes.len() && bytes[i] != b'}' {
                i += 1;
            }
            if i < bytes.len() && bytes[i] == b'}' {
                out.push(&path[start..i]);
            }
        }
        i += 1;
    }
    out
}

fn single_body_param<'a>(query: &'a AnalyzedQuery, locations: &BTreeMap<String, HttpParamBinding>) -> Option<&'a str> {
    let body_names: Vec<&str> = query
        .params
        .iter()
        .filter(|p| locations.get(&p.name) == Some(&HttpParamBinding::Body))
        .map(|p| p.name.as_str())
        .collect();
    if body_names.len() == 1 {
        Some(body_names[0])
    } else {
        None
    }
}

fn build_parameter_schema(
    query: &AnalyzedQuery,
    locations: &BTreeMap<String, HttpParamBinding>,
    catalog: &Catalog,
    opts: &BuildOptions,
) -> Result<Value, RouteBuildError> {
    let mut props = Map::new();
    let mut required: Vec<String> = Vec::new();
    let optional_set: std::collections::HashSet<&str> = query.optional_params.iter().map(String::as_str).collect();
    for p in &query.params {
        let loc = locations.get(&p.name).copied().unwrap_or(HttpParamBinding::Body);
        if !matches!(loc, HttpParamBinding::Path | HttpParamBinding::Query) {
            continue;
        }
        let schema = json_schema_for(&p.neutral_type, p.nullable, &query.enums, catalog, opts)?;
        props.insert(p.name.clone(), schema);
        let is_required = matches!(loc, HttpParamBinding::Path) || !optional_set.contains(p.name.as_str());
        if is_required {
            required.push(p.name.clone());
        }
    }
    if props.is_empty() {
        return Ok(Value::Null);
    }
    let mut obj = Map::new();
    obj.insert("type".into(), json!("object"));
    obj.insert("properties".into(), Value::Object(props));
    if !required.is_empty() {
        obj.insert("required".into(), json!(required));
    }
    Ok(Value::Object(obj))
}

fn build_request_schema(
    query: &AnalyzedQuery,
    locations: &BTreeMap<String, HttpParamBinding>,
    _bundle_name: &str,
    catalog: &Catalog,
    opts: &BuildOptions,
) -> Result<Value, RouteBuildError> {
    let optional_set: std::collections::HashSet<&str> = query.optional_params.iter().map(String::as_str).collect();
    let mut props = Map::new();
    let mut required: Vec<String> = Vec::new();
    for p in &query.params {
        if locations.get(&p.name) != Some(&HttpParamBinding::Body) {
            continue;
        }
        let schema = json_schema_for(&p.neutral_type, p.nullable, &query.enums, catalog, opts)?;
        props.insert(p.name.clone(), schema);
        if !optional_set.contains(p.name.as_str()) {
            required.push(p.name.clone());
        }
    }
    if props.is_empty() {
        return Ok(Value::Null);
    }
    let mut obj = Map::new();
    obj.insert("type".into(), json!("object"));
    obj.insert("properties".into(), Value::Object(props));
    if !required.is_empty() {
        obj.insert("required".into(), json!(required));
    }
    Ok(Value::Object(obj))
}

fn build_response_schema(
    query: &AnalyzedQuery,
    catalog: &Catalog,
    opts: &BuildOptions,
) -> Result<Value, RouteBuildError> {
    match query.command {
        QueryCommand::Exec | QueryCommand::ExecResult | QueryCommand::Batch => Ok(Value::Null),
        QueryCommand::ExecRows => Ok(json!({
            "type": "object",
            "properties": { "rows": { "type": "integer", "format": "int64" } },
            "required": ["rows"],
        })),
        QueryCommand::One | QueryCommand::Opt => {
            let row = row_object_schema(query, catalog, opts)?;
            if matches!(query.command, QueryCommand::Opt) {
                Ok(json!({ "oneOf": [row, { "type": "null" }] }))
            } else {
                Ok(row)
            }
        }
        QueryCommand::Many | QueryCommand::Grouped => {
            let row = row_object_schema(query, catalog, opts)?;
            Ok(json!({ "type": "array", "items": row }))
        }
    }
}

fn row_object_schema(query: &AnalyzedQuery, catalog: &Catalog, opts: &BuildOptions) -> Result<Value, RouteBuildError> {
    let mut props = Map::new();
    let mut required: Vec<String> = Vec::new();
    for col in &query.columns {
        let schema = json_schema_for(&col.neutral_type, col.nullable, &query.enums, catalog, opts)?;
        props.insert(col.name.clone(), schema);
        required.push(col.name.clone());
    }
    let mut obj = Map::new();
    obj.insert("type".into(), json!("object"));
    obj.insert("properties".into(), Value::Object(props));
    if !required.is_empty() {
        obj.insert("required".into(), json!(required));
    }
    Ok(Value::Object(obj))
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    let mut prev_lower = false;
    for c in s.chars() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use scythe_core::analyzer::{AnalyzedColumn, AnalyzedParam, AnalyzedQuery};
    use scythe_core::parser::{CustomAnnotation, QueryCommand};

    fn empty_catalog() -> Catalog {
        Catalog::from_ddl(&[]).unwrap()
    }

    fn get_user_query() -> AnalyzedQuery {
        AnalyzedQuery {
            name: "GetUser".to_string(),
            command: QueryCommand::One,
            sql: "SELECT id, email, name FROM users WHERE id = $1".to_string(),
            columns: vec![
                AnalyzedColumn {
                    name: "id".to_string(),
                    neutral_type: "int64".to_string(),
                    nullable: false,
                },
                AnalyzedColumn {
                    name: "email".to_string(),
                    neutral_type: "string".to_string(),
                    nullable: false,
                },
                AnalyzedColumn {
                    name: "name".to_string(),
                    neutral_type: "string".to_string(),
                    nullable: true,
                },
            ],
            params: vec![AnalyzedParam {
                name: "id".to_string(),
                neutral_type: "int64".to_string(),
                nullable: false,
                position: 1,
            }],
            deprecated: None,
            source_table: Some("users".to_string()),
            composites: vec![],
            enums: vec![],
            optional_params: vec![],
            group_by: None,
            custom: vec![
                CustomAnnotation {
                    name: "http".into(),
                    value: "GET /users/{id}".into(),
                    line: 3,
                },
                CustomAnnotation {
                    name: "http_auth".into(),
                    value: "bearer:jwt".into(),
                    line: 4,
                },
                CustomAnnotation {
                    name: "http_status".into(),
                    value: "200,404".into(),
                    line: 5,
                },
            ],
        }
    }

    fn create_user_query() -> AnalyzedQuery {
        AnalyzedQuery {
            name: "CreateUser".to_string(),
            command: QueryCommand::ExecRows,
            sql: "INSERT INTO users (email, name) VALUES ($1, $2)".to_string(),
            columns: vec![],
            params: vec![
                AnalyzedParam {
                    name: "email".to_string(),
                    neutral_type: "string".to_string(),
                    nullable: false,
                    position: 1,
                },
                AnalyzedParam {
                    name: "name".to_string(),
                    neutral_type: "string".to_string(),
                    nullable: true,
                    position: 2,
                },
            ],
            deprecated: None,
            source_table: None,
            composites: vec![],
            enums: vec![],
            optional_params: vec![],
            group_by: None,
            custom: vec![
                CustomAnnotation {
                    name: "http".into(),
                    value: "POST /users".into(),
                    line: 1,
                },
                CustomAnnotation {
                    name: "http_status".into(),
                    value: "201".into(),
                    line: 2,
                },
            ],
        }
    }

    fn list_users_query() -> AnalyzedQuery {
        AnalyzedQuery {
            name: "ListUsers".to_string(),
            command: QueryCommand::Many,
            sql: "SELECT id, email FROM users LIMIT $1 OFFSET $2".to_string(),
            columns: vec![
                AnalyzedColumn {
                    name: "id".to_string(),
                    neutral_type: "int64".to_string(),
                    nullable: false,
                },
                AnalyzedColumn {
                    name: "email".to_string(),
                    neutral_type: "string".to_string(),
                    nullable: false,
                },
            ],
            params: vec![
                AnalyzedParam {
                    name: "limit".to_string(),
                    neutral_type: "int32".to_string(),
                    nullable: true,
                    position: 1,
                },
                AnalyzedParam {
                    name: "offset".to_string(),
                    neutral_type: "int32".to_string(),
                    nullable: true,
                    position: 2,
                },
            ],
            deprecated: None,
            source_table: Some("users".to_string()),
            composites: vec![],
            enums: vec![],
            optional_params: vec!["limit".to_string(), "offset".to_string()],
            group_by: None,
            custom: vec![CustomAnnotation {
                name: "http".into(),
                value: "GET /users".into(),
                line: 1,
            }],
        }
    }

    #[test]
    fn route_from_get_query_uses_get_method() {
        let q = get_user_query();
        let route = route_from_query(&q, &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        assert_eq!(route.metadata["method"], "GET");
        assert_eq!(route.metadata["path"], "/users/{id}");
        assert_eq!(route.metadata["handler_name"], "handle_get_user");
        assert_eq!(route.operation_id, "GetUser");
    }

    #[test]
    fn handler_name_distinct_from_scythe_fn() {
        let route = route_from_query(&get_user_query(), &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        assert_eq!(route.handler_name, "handle_get_user");
        assert_ne!(route.handler_name, "get_user");
    }

    #[test]
    fn path_param_bound_to_path() {
        let route = route_from_query(&get_user_query(), &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        assert_eq!(route.param_locations.get("id"), Some(&HttpParamBinding::Path));
    }

    #[test]
    fn parameter_schema_carries_path_param_as_required() {
        let route = route_from_query(&get_user_query(), &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        let params = &route.metadata["parameter_schema"];
        assert_eq!(params["type"], "object");
        assert!(params["properties"]["id"].is_object());
        assert_eq!(params["required"], json!(["id"]));
    }

    #[test]
    fn list_query_params_become_query_and_optional() {
        let route = route_from_query(&list_users_query(), &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        assert_eq!(route.param_locations.get("limit"), Some(&HttpParamBinding::Query));
        let params = &route.metadata["parameter_schema"];
        assert!(params["properties"]["limit"].is_object());
        assert!(params["required"].is_null() || !params["required"].as_array().unwrap().iter().any(|v| v == "limit"));
    }

    #[test]
    fn post_query_params_become_body() {
        let route = route_from_query(&create_user_query(), &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        assert_eq!(route.param_locations.get("email"), Some(&HttpParamBinding::Body));
        assert_eq!(route.metadata["method"], "POST");
        let req = &route.metadata["request_schema"];
        assert_eq!(req["type"], "object");
        assert!(req["properties"]["email"].is_object());
        assert!(req["properties"]["name"].is_object());
        assert_eq!(req["required"], json!(["email", "name"]));
        assert_eq!(route.metadata["expects_json_body"], true);
    }

    #[test]
    fn one_query_response_is_object_with_required_columns() {
        let route = route_from_query(&get_user_query(), &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        let resp = &route.metadata["response_schema"];
        assert_eq!(resp["type"], "object");
        assert_eq!(resp["required"], json!(["id", "email", "name"]));
    }

    #[test]
    fn many_query_response_is_array() {
        let route = route_from_query(&list_users_query(), &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        let resp = &route.metadata["response_schema"];
        assert_eq!(resp["type"], "array");
        assert_eq!(resp["items"]["type"], "object");
    }

    #[test]
    fn exec_rows_response_is_rows_object() {
        let route = route_from_query(&create_user_query(), &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        let resp = &route.metadata["response_schema"];
        assert_eq!(resp["type"], "object");
        assert_eq!(resp["properties"]["rows"]["type"], "integer");
        assert_eq!(resp["required"], json!(["rows"]));
    }

    #[test]
    fn nullable_column_emits_oneof_null() {
        let route = route_from_query(&get_user_query(), &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        let resp = &route.metadata["response_schema"];
        let name_schema = &resp["properties"]["name"];
        assert!(name_schema["oneOf"].is_array());
    }

    #[test]
    fn no_http_directive_returns_none() {
        let mut q = get_user_query();
        q.custom.clear();
        let route = route_from_query(&q, &empty_catalog(), &BuildOptions::default()).unwrap();
        assert!(route.is_none());
    }

    #[test]
    fn batch_command_with_http_errors() {
        let mut q = get_user_query();
        q.command = QueryCommand::Batch;
        let err = route_from_query(&q, &empty_catalog(), &BuildOptions::default()).unwrap_err();
        assert!(matches!(
            err,
            RouteBuildError::Annotation(AnnotationParseError::IncompatibleCommand { .. })
        ));
    }

    #[test]
    fn snake_case_handles_pascal_case() {
        assert_eq!(to_snake_case("GetUser"), "get_user");
        assert_eq!(to_snake_case("ListActiveUsers"), "list_active_users");
        assert_eq!(to_snake_case("CreateUser"), "create_user");
    }

    #[test]
    fn default_status_matches_command() {
        let route = route_from_query(&get_user_query(), &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        assert_eq!(route.default_status, 200);
        let route = route_from_query(&create_user_query(), &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        assert_eq!(route.default_status, 200);
    }

    #[test]
    fn single_body_param_recorded_in_metadata() {
        let mut q = create_user_query();
        q.params.truncate(1);
        let route = route_from_query(&q, &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        assert_eq!(route.metadata["body_param_name"], "email");
    }
}
