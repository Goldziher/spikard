//! Emit an OpenAPI 3.1 document from a slice of [`SqlRoute`].
//!
//! The spec is built as a raw `serde_json::Value` rather than reusing
//! `crate::openapi::OpenApiSpec` because the existing struct is a subset that
//! lacks several 3.1 idioms we need (array-typed `type`, `oneOf` for
//! nullability, `enum`). Emitting as `Value` keeps this module decoupled and
//! the output round-trips through any OpenAPI 3.1 consumer.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use super::annotations::{ApiKeyLocation, AuthRequirement, HttpMethod, HttpParamBinding};
use super::route::SqlRoute;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiInfo {
    pub title: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl OpenApiInfo {
    pub fn new(title: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            version: version.into(),
            description: None,
        }
    }
}

/// Build an OpenAPI 3.1 document from a list of SQL-derived routes. The
/// returned `Value` is ready to be `serde_json::to_writer_pretty`-ed to disk.
pub fn openapi_from_routes(routes: &[SqlRoute], info: &OpenApiInfo) -> Value {
    let (security_schemes, scheme_names) = collect_security_schemes(routes);

    let mut paths: IndexMap<String, Map<String, Value>> = IndexMap::new();
    for route in routes {
        let entry = paths.entry(route.http.path.clone()).or_default();
        let operation = build_operation(route, &scheme_names);
        entry.insert(method_key(route.http.method).to_string(), operation);
    }

    let mut paths_obj = Map::new();
    for (path, methods) in paths {
        paths_obj.insert(path, Value::Object(methods));
    }

    let mut spec = Map::new();
    spec.insert("openapi".into(), json!("3.1.0"));
    spec.insert("info".into(), serde_json::to_value(info).expect("info serializes"));
    spec.insert("paths".into(), Value::Object(paths_obj));

    let mut components = Map::new();
    if !security_schemes.is_empty() {
        components.insert("securitySchemes".into(), Value::Object(security_schemes));
    }
    if !components.is_empty() {
        spec.insert("components".into(), Value::Object(components));
    }

    Value::Object(spec)
}

fn build_operation(route: &SqlRoute, scheme_names: &std::collections::BTreeMap<AuthRequirement, String>) -> Value {
    let mut op = Map::new();
    op.insert("operationId".into(), json!(&route.operation_id));

    if let Some(s) = &route.http.summary {
        op.insert("summary".into(), json!(s));
    }
    if let Some(d) = &route.http.description {
        op.insert("description".into(), json!(d));
    }
    if !route.http.tags.is_empty() {
        op.insert("tags".into(), json!(&route.http.tags));
    }

    let parameters = build_parameters(route);
    if !parameters.is_empty() {
        op.insert("parameters".into(), Value::Array(parameters));
    }

    if let Some(request_body) = build_request_body(route) {
        op.insert("requestBody".into(), request_body);
    }

    op.insert("responses".into(), build_responses(route));

    if let Some(auth) = &route.http.auth
        && !matches!(auth, AuthRequirement::None)
        && let Some(name) = scheme_names.get(auth)
    {
        op.insert("security".into(), json!([{ name.as_str(): [] }]));
    }

    Value::Object(op)
}

fn build_parameters(route: &SqlRoute) -> Vec<Value> {
    let mut out = Vec::new();
    let parameter_schema = &route.metadata["parameter_schema"];
    let properties = parameter_schema.get("properties").and_then(Value::as_object);
    let Some(properties) = properties else {
        return out;
    };
    let required: std::collections::HashSet<&str> = parameter_schema
        .get("required")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();

    for (name, schema) in properties {
        let location = match route.param_locations.get(name) {
            Some(HttpParamBinding::Path) => "path",
            Some(HttpParamBinding::Query) => "query",
            Some(HttpParamBinding::Header) => "header",
            _ => continue,
        };
        let is_required = location == "path" || required.contains(name.as_str());
        let mut p = Map::new();
        p.insert("name".into(), json!(name));
        p.insert("in".into(), json!(location));
        p.insert("required".into(), json!(is_required));
        p.insert("schema".into(), schema.clone());
        out.push(Value::Object(p));
    }
    out
}

fn build_request_body(route: &SqlRoute) -> Option<Value> {
    let request_schema = route.metadata.get("request_schema")?;
    if request_schema.is_null() {
        return None;
    }
    Some(json!({
        "required": true,
        "content": {
            "application/json": { "schema": request_schema }
        }
    }))
}

fn build_responses(route: &SqlRoute) -> Value {
    let mut responses = Map::new();
    let response_schema = route.metadata.get("response_schema").cloned().unwrap_or(Value::Null);
    let codes: Vec<u16> = if route.http.status_codes.is_empty() {
        vec![route.default_status]
    } else {
        route.http.status_codes.clone()
    };
    for (idx, code) in codes.iter().enumerate() {
        let is_primary = idx == 0;
        let mut body = Map::new();
        body.insert("description".into(), json!(describe_status(*code)));
        if is_primary && !response_schema.is_null() && *code != 204 {
            body.insert(
                "content".into(),
                json!({ "application/json": { "schema": response_schema.clone() } }),
            );
        }
        responses.insert(code.to_string(), Value::Object(body));
    }
    Value::Object(responses)
}

const fn describe_status(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        409 => "Conflict",
        422 => "Unprocessable Entity",
        500 => "Internal Server Error",
        _ => "Response",
    }
}

fn collect_security_schemes(
    routes: &[SqlRoute],
) -> (Map<String, Value>, std::collections::BTreeMap<AuthRequirement, String>) {
    let mut schemes = Map::new();
    let mut name_for = std::collections::BTreeMap::new();
    for route in routes {
        let Some(auth) = &route.http.auth else { continue };
        if matches!(auth, AuthRequirement::None) {
            continue;
        }
        if name_for.contains_key(auth) {
            continue;
        }
        let name = match auth {
            AuthRequirement::None => unreachable!(),
            AuthRequirement::Bearer { format: None } => "bearerAuth".to_string(),
            AuthRequirement::Bearer { format: Some(f) } => format!("bearer{}", f.to_uppercase()),
            AuthRequirement::ApiKey { location, name } => {
                format!("apiKey_{}_{}", location_short(*location), name.replace('-', "_"))
            }
        };
        let scheme_value = match auth {
            AuthRequirement::None => unreachable!(),
            AuthRequirement::Bearer { format } => {
                let mut s = Map::new();
                s.insert("type".into(), json!("http"));
                s.insert("scheme".into(), json!("bearer"));
                if let Some(f) = format {
                    s.insert("bearerFormat".into(), json!(f));
                }
                Value::Object(s)
            }
            AuthRequirement::ApiKey { location, name } => json!({
                "type": "apiKey",
                "in": location_str(*location),
                "name": name,
            }),
        };
        schemes.insert(name.clone(), scheme_value);
        name_for.insert(auth.clone(), name);
    }
    (schemes, name_for)
}

const fn location_short(loc: ApiKeyLocation) -> &'static str {
    match loc {
        ApiKeyLocation::Header => "h",
        ApiKeyLocation::Query => "q",
        ApiKeyLocation::Cookie => "c",
    }
}

const fn location_str(loc: ApiKeyLocation) -> &'static str {
    match loc {
        ApiKeyLocation::Header => "header",
        ApiKeyLocation::Query => "query",
        ApiKeyLocation::Cookie => "cookie",
    }
}

const fn method_key(m: HttpMethod) -> &'static str {
    match m {
        HttpMethod::Get => "get",
        HttpMethod::Post => "post",
        HttpMethod::Put => "put",
        HttpMethod::Patch => "patch",
        HttpMethod::Delete => "delete",
        HttpMethod::Head => "head",
        HttpMethod::Options => "options",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sql::neutral_to_json_schema::BuildOptions;
    use crate::sql::route::route_from_query;
    use scythe_core::analyzer::{AnalyzedColumn, AnalyzedParam, AnalyzedQuery};
    use scythe_core::catalog::Catalog;
    use scythe_core::parser::{CustomAnnotation, QueryCommand};

    fn empty_catalog() -> Catalog {
        Catalog::from_ddl(&[]).unwrap()
    }

    fn get_user_query() -> AnalyzedQuery {
        AnalyzedQuery {
            name: "GetUser".to_string(),
            command: QueryCommand::One,
            sql: "SELECT id, email FROM users WHERE id = $1".to_string(),
            columns: vec![
                AnalyzedColumn {
                    name: "id".into(),
                    neutral_type: "int64".into(),
                    nullable: false,
                },
                AnalyzedColumn {
                    name: "email".into(),
                    neutral_type: "string".into(),
                    nullable: false,
                },
            ],
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
            custom: vec![
                CustomAnnotation {
                    name: "http".into(),
                    value: "GET /users/{id}".into(),
                    line: 1,
                },
                CustomAnnotation {
                    name: "http_auth".into(),
                    value: "bearer:jwt".into(),
                    line: 2,
                },
                CustomAnnotation {
                    name: "http_status".into(),
                    value: "200,404".into(),
                    line: 3,
                },
                CustomAnnotation {
                    name: "http_tags".into(),
                    value: "users".into(),
                    line: 4,
                },
                CustomAnnotation {
                    name: "http_summary".into(),
                    value: "Fetch a user".into(),
                    line: 5,
                },
            ],
        }
    }

    fn create_user_query() -> AnalyzedQuery {
        AnalyzedQuery {
            name: "CreateUser".to_string(),
            command: QueryCommand::ExecRows,
            sql: "INSERT INTO users (email) VALUES ($1)".to_string(),
            columns: vec![],
            params: vec![AnalyzedParam {
                name: "email".into(),
                neutral_type: "string".into(),
                nullable: false,
                position: 1,
            }],
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
                    name: "http_auth".into(),
                    value: "bearer:jwt".into(),
                    line: 2,
                },
                CustomAnnotation {
                    name: "http_status".into(),
                    value: "201".into(),
                    line: 3,
                },
            ],
        }
    }

    fn build_two_routes() -> Vec<SqlRoute> {
        let opts = BuildOptions::default();
        let r1 = route_from_query(&get_user_query(), &empty_catalog(), &opts)
            .unwrap()
            .unwrap();
        let r2 = route_from_query(&create_user_query(), &empty_catalog(), &opts)
            .unwrap()
            .unwrap();
        vec![r1, r2]
    }

    #[test]
    fn emits_openapi_3_1_header() {
        let routes = build_two_routes();
        let spec = openapi_from_routes(&routes, &OpenApiInfo::new("test", "0.1.0"));
        assert_eq!(spec["openapi"], "3.1.0");
        assert_eq!(spec["info"]["title"], "test");
        assert_eq!(spec["info"]["version"], "0.1.0");
    }

    #[test]
    fn groups_methods_under_shared_path() {
        let routes = build_two_routes();
        let spec = openapi_from_routes(&routes, &OpenApiInfo::new("t", "1"));
        assert!(spec["paths"]["/users"]["post"].is_object());
        assert!(spec["paths"]["/users/{id}"]["get"].is_object());
    }

    #[test]
    fn operation_carries_operation_id_summary_tags() {
        let routes = build_two_routes();
        let spec = openapi_from_routes(&routes, &OpenApiInfo::new("t", "1"));
        let op = &spec["paths"]["/users/{id}"]["get"];
        assert_eq!(op["operationId"], "GetUser");
        assert_eq!(op["summary"], "Fetch a user");
        assert_eq!(op["tags"], json!(["users"]));
    }

    #[test]
    fn path_parameter_emitted() {
        let routes = build_two_routes();
        let spec = openapi_from_routes(&routes, &OpenApiInfo::new("t", "1"));
        let params = spec["paths"]["/users/{id}"]["get"]["parameters"].as_array().unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0]["name"], "id");
        assert_eq!(params[0]["in"], "path");
        assert_eq!(params[0]["required"], true);
    }

    #[test]
    fn post_carries_request_body() {
        let routes = build_two_routes();
        let spec = openapi_from_routes(&routes, &OpenApiInfo::new("t", "1"));
        let body = &spec["paths"]["/users"]["post"]["requestBody"];
        assert_eq!(body["required"], true);
        assert!(body["content"]["application/json"]["schema"]["properties"]["email"].is_object());
    }

    #[test]
    fn responses_keyed_by_status_codes() {
        let routes = build_two_routes();
        let spec = openapi_from_routes(&routes, &OpenApiInfo::new("t", "1"));
        let resp = &spec["paths"]["/users/{id}"]["get"]["responses"];
        assert!(resp["200"].is_object());
        assert!(resp["404"].is_object());
    }

    #[test]
    fn primary_response_includes_schema() {
        let routes = build_two_routes();
        let spec = openapi_from_routes(&routes, &OpenApiInfo::new("t", "1"));
        let primary = &spec["paths"]["/users/{id}"]["get"]["responses"]["200"];
        assert!(primary["content"]["application/json"]["schema"]["properties"]["id"].is_object());
    }

    #[test]
    fn registers_bearer_security_scheme_once() {
        let routes = build_two_routes();
        let spec = openapi_from_routes(&routes, &OpenApiInfo::new("t", "1"));
        let schemes = &spec["components"]["securitySchemes"];
        assert_eq!(schemes.as_object().unwrap().len(), 1);
        let (_name, scheme) = schemes.as_object().unwrap().iter().next().unwrap();
        assert_eq!(scheme["type"], "http");
        assert_eq!(scheme["scheme"], "bearer");
        assert_eq!(scheme["bearerFormat"], "jwt");
    }

    #[test]
    fn operations_reference_security_scheme() {
        let routes = build_two_routes();
        let spec = openapi_from_routes(&routes, &OpenApiInfo::new("t", "1"));
        let op = &spec["paths"]["/users/{id}"]["get"];
        let sec = op["security"].as_array().unwrap();
        assert_eq!(sec.len(), 1);
        let scheme_name = sec[0].as_object().unwrap().keys().next().unwrap();
        assert!(spec["components"]["securitySchemes"][scheme_name].is_object());
    }

    #[test]
    fn no_204_response_carries_body() {
        let mut q = create_user_query();
        q.command = QueryCommand::Exec;
        q.custom.retain(|a| a.name != "http_status");
        let route = route_from_query(&q, &empty_catalog(), &BuildOptions::default())
            .unwrap()
            .unwrap();
        let spec = openapi_from_routes(&[route], &OpenApiInfo::new("t", "1"));
        let resp = &spec["paths"]["/users"]["post"]["responses"]["204"];
        assert!(resp["content"].is_null());
    }
}
