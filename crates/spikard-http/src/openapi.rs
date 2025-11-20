//! OpenAPI 3.1.0 specification generation
//!
//! Generates OpenAPI specs from route definitions using existing JSON Schema infrastructure.
//! OpenAPI 3.1.0 is fully compatible with JSON Schema Draft 2020-12.

use crate::{RouteMetadata, SchemaRegistry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::openapi::security::SecurityScheme;
use utoipa::openapi::{
    Components, Contact, Info, License, OpenApi, OpenApiBuilder, PathItem, Paths, RefOr, Response, Responses, Schema,
    Server,
};

/// OpenAPI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiConfig {
    /// Enable OpenAPI generation (default: false for zero overhead)
    pub enabled: bool,

    /// API title
    pub title: String,

    /// API version
    pub version: String,

    /// API description (supports markdown)
    #[serde(default)]
    pub description: Option<String>,

    /// Path to serve Swagger UI (default: "/docs")
    #[serde(default = "default_swagger_path")]
    pub swagger_ui_path: String,

    /// Path to serve Redoc (default: "/redoc")
    #[serde(default = "default_redoc_path")]
    pub redoc_path: String,

    /// Path to serve OpenAPI JSON spec (default: "/openapi.json")
    #[serde(default = "default_openapi_json_path")]
    pub openapi_json_path: String,

    /// Contact information
    #[serde(default)]
    pub contact: Option<ContactInfo>,

    /// License information
    #[serde(default)]
    pub license: Option<LicenseInfo>,

    /// Server definitions
    #[serde(default)]
    pub servers: Vec<ServerInfo>,

    /// Security schemes (auto-detected from middleware if not provided)
    #[serde(default)]
    pub security_schemes: HashMap<String, SecuritySchemeInfo>,
}

impl Default for OpenApiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            title: "API".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            swagger_ui_path: default_swagger_path(),
            redoc_path: default_redoc_path(),
            openapi_json_path: default_openapi_json_path(),
            contact: None,
            license: None,
            servers: Vec::new(),
            security_schemes: HashMap::new(),
        }
    }
}

fn default_swagger_path() -> String {
    "/docs".to_string()
}

fn default_redoc_path() -> String {
    "/redoc".to_string()
}

fn default_openapi_json_path() -> String {
    "/openapi.json".to_string()
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
}

/// License information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub name: String,
    pub url: Option<String>,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub url: String,
    pub description: Option<String>,
}

/// Security scheme types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SecuritySchemeInfo {
    #[serde(rename = "http")]
    Http {
        scheme: String,
        #[serde(rename = "bearerFormat")]
        bearer_format: Option<String>,
    },
    #[serde(rename = "apiKey")]
    ApiKey {
        #[serde(rename = "in")]
        location: String,
        name: String,
    },
}

/// Generate OpenAPI specification from routes with auto-detection of security schemes
pub fn generate_openapi_spec(
    routes: &[RouteMetadata],
    config: &OpenApiConfig,
    _schema_registry: &SchemaRegistry,
    server_config: Option<&crate::ServerConfig>,
) -> Result<OpenApi, String> {
    let mut info = Info::new(&config.title, &config.version);
    if let Some(desc) = &config.description {
        info.description = Some(desc.clone());
    }
    if let Some(contact_info) = &config.contact {
        let mut contact = Contact::default();
        if let Some(name) = &contact_info.name {
            contact.name = Some(name.clone());
        }
        if let Some(email) = &contact_info.email {
            contact.email = Some(email.clone());
        }
        if let Some(url) = &contact_info.url {
            contact.url = Some(url.clone());
        }
        info.contact = Some(contact);
    }
    if let Some(license_info) = &config.license {
        let mut license = License::new(&license_info.name);
        if let Some(url) = &license_info.url {
            license.url = Some(url.clone());
        }
        info.license = Some(license);
    }

    let servers = if config.servers.is_empty() {
        None
    } else {
        Some(
            config
                .servers
                .iter()
                .map(|s| {
                    let mut server = Server::new(&s.url);
                    if let Some(desc) = &s.description {
                        server.description = Some(desc.clone());
                    }
                    server
                })
                .collect(),
        )
    };

    let mut paths = Paths::new();
    for route in routes {
        let path_item = route_to_path_item(route)?;
        paths.paths.insert(route.path.clone(), path_item);
    }

    let mut components = Components::new();
    let mut global_security = Vec::new();

    if let Some(server_cfg) = server_config {
        if let Some(_jwt_cfg) = &server_cfg.jwt_auth {
            let jwt_scheme = SecurityScheme::Http(
                utoipa::openapi::security::HttpBuilder::new()
                    .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            );
            components.add_security_scheme("bearerAuth", jwt_scheme);

            let security_req = utoipa::openapi::security::SecurityRequirement::new("bearerAuth", Vec::<String>::new());
            global_security.push(security_req);
        }

        if let Some(api_key_cfg) = &server_cfg.api_key_auth {
            use utoipa::openapi::security::ApiKey;
            let api_key_scheme = SecurityScheme::ApiKey(ApiKey::Header(utoipa::openapi::security::ApiKeyValue::new(
                &api_key_cfg.header_name,
            )));
            components.add_security_scheme("apiKeyAuth", api_key_scheme);

            let security_req = utoipa::openapi::security::SecurityRequirement::new("apiKeyAuth", Vec::<String>::new());
            global_security.push(security_req);
        }
    }

    if !config.security_schemes.is_empty() {
        for (name, scheme_info) in &config.security_schemes {
            let scheme = security_scheme_info_to_openapi(scheme_info);
            components.add_security_scheme(name, scheme);
        }
    }

    let mut openapi = OpenApiBuilder::new()
        .info(info)
        .paths(paths)
        .components(Some(components))
        .build();

    if let Some(servers) = servers {
        openapi.servers = Some(servers);
    }

    if !global_security.is_empty() {
        openapi.security = Some(global_security);
    }

    Ok(openapi)
}

/// Convert route to OpenAPI PathItem
fn route_to_path_item(route: &RouteMetadata) -> Result<PathItem, String> {
    use utoipa::openapi::HttpMethod;

    let operation = route_to_operation(route)?;

    let http_method = match route.method.to_uppercase().as_str() {
        "GET" => HttpMethod::Get,
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "DELETE" => HttpMethod::Delete,
        "PATCH" => HttpMethod::Patch,
        "HEAD" => HttpMethod::Head,
        "OPTIONS" => HttpMethod::Options,
        _ => return Err(format!("Unsupported HTTP method: {}", route.method)),
    };

    let path_item = PathItem::new(http_method, operation);

    Ok(path_item)
}

/// Convert route to OpenAPI Operation
fn route_to_operation(route: &RouteMetadata) -> Result<utoipa::openapi::path::Operation, String> {
    let mut operation = utoipa::openapi::path::Operation::new();

    if let Some(param_schema) = &route.parameter_schema {
        let parameters = extract_parameters_from_schema(param_schema, &route.path)?;
        if !parameters.is_empty() {
            let unwrapped: Vec<_> = parameters
                .into_iter()
                .filter_map(|p| if let RefOr::T(param) = p { Some(param) } else { None })
                .collect();
            operation.parameters = Some(unwrapped);
        }
    }

    if let Some(request_schema) = &route.request_schema {
        let request_body = json_schema_to_request_body(request_schema)?;
        operation.request_body = Some(request_body);
    }

    let mut responses = Responses::new();
    if let Some(response_schema) = &route.response_schema {
        let response = json_schema_to_response(response_schema)?;
        responses.responses.insert("200".to_string(), RefOr::T(response));
    } else {
        responses
            .responses
            .insert("200".to_string(), RefOr::T(Response::new("Successful response")));
    }
    operation.responses = responses;

    Ok(operation)
}

/// Extract parameters from JSON Schema parameter_schema
fn extract_parameters_from_schema(
    param_schema: &serde_json::Value,
    route_path: &str,
) -> Result<Vec<RefOr<utoipa::openapi::path::Parameter>>, String> {
    use utoipa::openapi::path::{ParameterBuilder, ParameterIn};

    let mut parameters = Vec::new();

    let path_params = extract_path_param_names(route_path);

    let properties = param_schema
        .get("properties")
        .and_then(|p| p.as_object())
        .ok_or_else(|| "Parameter schema missing 'properties' field".to_string())?;

    let required = param_schema
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    for (name, schema) in properties {
        let is_required = required.contains(&name.as_str());
        let param_in = if path_params.contains(&name.as_str()) {
            ParameterIn::Path
        } else {
            ParameterIn::Query
        };

        let openapi_schema = json_value_to_schema(schema)?;

        let is_path_param = matches!(param_in, ParameterIn::Path);

        let param = ParameterBuilder::new()
            .name(name)
            .parameter_in(param_in)
            .required(if is_path_param || is_required {
                utoipa::openapi::Required::True
            } else {
                utoipa::openapi::Required::False
            })
            .schema(Some(openapi_schema))
            .build();

        parameters.push(RefOr::T(param));
    }

    Ok(parameters)
}

/// Extract path parameter names from route pattern (e.g., "/users/{id}" -> ["id"])
fn extract_path_param_names(route: &str) -> Vec<&str> {
    route
        .split('/')
        .filter_map(|segment| {
            if segment.starts_with('{') && segment.ends_with('}') {
                Some(&segment[1..segment.len() - 1])
            } else {
                None
            }
        })
        .collect()
}

/// Convert JSON Schema to OpenAPI RequestBody
fn json_schema_to_request_body(
    schema: &serde_json::Value,
) -> Result<utoipa::openapi::request_body::RequestBody, String> {
    use utoipa::openapi::content::ContentBuilder;

    let openapi_schema = json_value_to_schema(schema)?;

    let content = ContentBuilder::new().schema(Some(openapi_schema)).build();

    let mut request_body = utoipa::openapi::request_body::RequestBody::new();
    request_body.content.insert("application/json".to_string(), content);

    request_body.required = Some(utoipa::openapi::Required::True);

    Ok(request_body)
}

/// Convert JSON Schema to OpenAPI Response
fn json_schema_to_response(schema: &serde_json::Value) -> Result<Response, String> {
    use utoipa::openapi::content::ContentBuilder;

    let openapi_schema = json_value_to_schema(schema)?;

    let content = ContentBuilder::new().schema(Some(openapi_schema)).build();

    let mut response = Response::new("Successful response");
    response.content.insert("application/json".to_string(), content);

    Ok(response)
}

/// Convert serde_json::Value (JSON Schema) to utoipa Schema
/// OpenAPI 3.1.0 is fully compatible with JSON Schema Draft 2020-12
fn json_value_to_schema(value: &serde_json::Value) -> Result<RefOr<Schema>, String> {
    if let Some(type_str) = value.get("type").and_then(|t| t.as_str()) {
        match type_str {
            "object" => {
                let mut object_schema = utoipa::openapi::ObjectBuilder::new();

                if let Some(properties) = value.get("properties").and_then(|p| p.as_object()) {
                    for (prop_name, prop_schema) in properties {
                        let prop_openapi_schema = json_value_to_schema(prop_schema)?;
                        object_schema = object_schema.property(prop_name, prop_openapi_schema);
                    }
                }

                if let Some(required) = value.get("required").and_then(|r| r.as_array()) {
                    for field in required {
                        if let Some(field_name) = field.as_str() {
                            object_schema = object_schema.required(field_name);
                        }
                    }
                }

                Ok(RefOr::T(Schema::Object(object_schema.build())))
            }
            "array" => {
                let mut array_schema = utoipa::openapi::ArrayBuilder::new();

                if let Some(items) = value.get("items") {
                    let items_schema = json_value_to_schema(items)?;
                    array_schema = array_schema.items(items_schema);
                }

                Ok(RefOr::T(Schema::Array(array_schema.build())))
            }
            "string" => {
                let mut schema_type = utoipa::openapi::schema::Type::String;

                if let Some(format) = value.get("format").and_then(|f| f.as_str()) {
                    match format {
                        "date-time" => schema_type = utoipa::openapi::schema::Type::String,
                        "date" => schema_type = utoipa::openapi::schema::Type::String,
                        "email" => schema_type = utoipa::openapi::schema::Type::String,
                        "uri" => schema_type = utoipa::openapi::schema::Type::String,
                        _ => {}
                    }
                }

                Ok(RefOr::T(Schema::Object(
                    utoipa::openapi::ObjectBuilder::new().schema_type(schema_type).build(),
                )))
            }
            "integer" => Ok(RefOr::T(Schema::Object(
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .build(),
            ))),
            "number" => Ok(RefOr::T(Schema::Object(
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Number)
                    .build(),
            ))),
            "boolean" => Ok(RefOr::T(Schema::Object(
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Boolean)
                    .build(),
            ))),
            _ => Err(format!("Unsupported schema type: {}", type_str)),
        }
    } else {
        Ok(RefOr::T(Schema::Object(utoipa::openapi::ObjectBuilder::new().build())))
    }
}

/// Convert SecuritySchemeInfo to OpenAPI SecurityScheme
fn security_scheme_info_to_openapi(info: &SecuritySchemeInfo) -> SecurityScheme {
    match info {
        SecuritySchemeInfo::Http { scheme, bearer_format } => {
            let mut http_scheme = SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            ));
            if let (SecurityScheme::Http(http), "bearer") = (&mut http_scheme, scheme.as_str()) {
                http.scheme = utoipa::openapi::security::HttpAuthScheme::Bearer;
                if let Some(format) = bearer_format {
                    http.bearer_format = Some(format.clone());
                }
            }
            http_scheme
        }
        SecuritySchemeInfo::ApiKey { location, name } => {
            use utoipa::openapi::security::ApiKey;

            let api_key = match location.as_str() {
                "header" => ApiKey::Header(utoipa::openapi::security::ApiKeyValue::new(name)),
                "query" => ApiKey::Query(utoipa::openapi::security::ApiKeyValue::new(name)),
                "cookie" => ApiKey::Cookie(utoipa::openapi::security::ApiKeyValue::new(name)),
                _ => ApiKey::Header(utoipa::openapi::security::ApiKeyValue::new(name)),
            };
            SecurityScheme::ApiKey(api_key)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_config_default() {
        let config = OpenApiConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.title, "API");
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.swagger_ui_path, "/docs");
        assert_eq!(config.redoc_path, "/redoc");
        assert_eq!(config.openapi_json_path, "/openapi.json");
    }

    #[test]
    fn test_generate_minimal_spec() {
        let config = OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        };

        let routes = vec![];
        let registry = SchemaRegistry::new();

        let spec = generate_openapi_spec(&routes, &config, &registry, None).unwrap();
        assert_eq!(spec.info.title, "Test API");
        assert_eq!(spec.info.version, "1.0.0");
    }

    #[test]
    fn test_generate_spec_with_contact() {
        let config = OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            contact: Some(ContactInfo {
                name: Some("API Team".to_string()),
                email: Some("api@example.com".to_string()),
                url: Some("https://example.com".to_string()),
            }),
            ..Default::default()
        };

        let routes = vec![];
        let registry = SchemaRegistry::new();

        let spec = generate_openapi_spec(&routes, &config, &registry, None).unwrap();
        assert!(spec.info.contact.is_some());
        let contact = spec.info.contact.unwrap();
        assert_eq!(contact.name, Some("API Team".to_string()));
        assert_eq!(contact.email, Some("api@example.com".to_string()));
    }

    #[test]
    fn test_generate_spec_with_license() {
        let config = OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            license: Some(LicenseInfo {
                name: "MIT".to_string(),
                url: Some("https://opensource.org/licenses/MIT".to_string()),
            }),
            ..Default::default()
        };

        let routes = vec![];
        let registry = SchemaRegistry::new();

        let spec = generate_openapi_spec(&routes, &config, &registry, None).unwrap();
        assert!(spec.info.license.is_some());
        let license = spec.info.license.unwrap();
        assert_eq!(license.name, "MIT");
    }

    #[test]
    fn test_generate_spec_with_servers() {
        let config = OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            servers: vec![
                ServerInfo {
                    url: "https://api.example.com".to_string(),
                    description: Some("Production".to_string()),
                },
                ServerInfo {
                    url: "http://localhost:8080".to_string(),
                    description: Some("Development".to_string()),
                },
            ],
            ..Default::default()
        };

        let routes = vec![];
        let registry = SchemaRegistry::new();

        let spec = generate_openapi_spec(&routes, &config, &registry, None).unwrap();
        assert!(spec.servers.is_some());
        let servers = spec.servers.unwrap();
        assert_eq!(servers.len(), 2);
        assert_eq!(servers[0].url, "https://api.example.com");
        assert_eq!(servers[1].url, "http://localhost:8080");
    }

    #[test]
    fn test_security_scheme_http_bearer() {
        let scheme_info = SecuritySchemeInfo::Http {
            scheme: "bearer".to_string(),
            bearer_format: Some("JWT".to_string()),
        };

        let scheme = security_scheme_info_to_openapi(&scheme_info);
        match scheme {
            SecurityScheme::Http(http) => {
                assert!(matches!(http.scheme, utoipa::openapi::security::HttpAuthScheme::Bearer));
                assert_eq!(http.bearer_format, Some("JWT".to_string()));
            }
            _ => panic!("Expected Http security scheme"),
        }
    }

    #[test]
    fn test_security_scheme_api_key() {
        let scheme_info = SecuritySchemeInfo::ApiKey {
            location: "header".to_string(),
            name: "X-API-Key".to_string(),
        };

        let scheme = security_scheme_info_to_openapi(&scheme_info);
        match scheme {
            SecurityScheme::ApiKey(api_key) => {
                assert!(matches!(api_key, utoipa::openapi::security::ApiKey::Header(_)));
            }
            _ => panic!("Expected ApiKey security scheme"),
        }
    }

    #[test]
    fn test_extract_path_param_names() {
        let names = extract_path_param_names("/users/{id}/posts/{post_id}");
        assert_eq!(names, vec!["id", "post_id"]);

        let names = extract_path_param_names("/users");
        assert_eq!(names, Vec::<&str>::new());

        let names = extract_path_param_names("/users/{user_id}");
        assert_eq!(names, vec!["user_id"]);
    }

    #[test]
    fn test_json_value_to_schema_string() {
        let schema_json = serde_json::json!({
            "type": "string"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_value_to_schema_integer() {
        let schema_json = serde_json::json!({
            "type": "integer"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_value_to_schema_number() {
        let schema_json = serde_json::json!({
            "type": "number"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_value_to_schema_boolean() {
        let schema_json = serde_json::json!({
            "type": "boolean"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_value_to_schema_object() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "integer" }
            },
            "required": ["name"]
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.properties.contains_key("name"));
            assert!(obj.properties.contains_key("age"));
            assert!(obj.required.contains(&"name".to_string()));
        } else {
            panic!("Expected Object schema");
        }
    }

    #[test]
    fn test_json_value_to_schema_array() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": {
                "type": "string"
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Array(_))) = result {
        } else {
            panic!("Expected Array schema");
        }
    }

    #[test]
    fn test_json_value_to_schema_nested_object() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "user": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "email": { "type": "string" }
                    }
                }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_schema_to_request_body() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "title": { "type": "string" },
                "count": { "type": "integer" }
            },
            "required": ["title"]
        });

        let result = json_schema_to_request_body(&schema_json);
        assert!(result.is_ok());

        let request_body = result.unwrap();
        assert!(request_body.content.contains_key("application/json"));
        assert!(matches!(request_body.required, Some(utoipa::openapi::Required::True)));
    }

    #[test]
    fn test_json_schema_to_request_body_array() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "id": { "type": "integer" }
                }
            }
        });

        let result = json_schema_to_request_body(&schema_json);
        assert!(result.is_ok());

        let request_body = result.unwrap();
        assert!(request_body.content.contains_key("application/json"));
    }

    #[test]
    fn test_json_schema_to_response() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "name": { "type": "string" }
            }
        });

        let result = json_schema_to_response(&schema_json);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.content.contains_key("application/json"));
        assert_eq!(response.description, "Successful response");
    }

    #[test]
    fn test_json_schema_to_response_array() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": {
                "type": "string"
            }
        });

        let result = json_schema_to_response(&schema_json);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.content.contains_key("application/json"));
    }

    #[test]
    fn test_extract_parameters_from_schema_path_params() {
        let param_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "user_id": { "type": "integer" },
                "post_id": { "type": "integer" }
            },
            "required": ["user_id", "post_id"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/users/{user_id}/posts/{post_id}");
        assert!(result.is_ok());

        let params = result.unwrap();
        assert_eq!(params.len(), 2);

        for param in params {
            if let RefOr::T(p) = param {
                assert!(matches!(p.parameter_in, utoipa::openapi::path::ParameterIn::Path));
                assert!(matches!(p.required, utoipa::openapi::Required::True));
            }
        }
    }

    #[test]
    fn test_extract_parameters_from_schema_query_params() {
        let param_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "page": { "type": "integer" },
                "limit": { "type": "integer" },
                "search": { "type": "string" }
            },
            "required": ["page"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/users");
        assert!(result.is_ok());

        let params = result.unwrap();
        assert_eq!(params.len(), 3);

        for param in &params {
            if let RefOr::T(p) = param {
                assert!(matches!(p.parameter_in, utoipa::openapi::path::ParameterIn::Query));
            }
        }

        for param in params {
            if let RefOr::T(p) = param {
                if p.name == "page" {
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                } else {
                    assert!(matches!(p.required, utoipa::openapi::Required::False));
                }
            }
        }
    }

    #[test]
    fn test_extract_parameters_from_schema_mixed() {
        let param_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "user_id": { "type": "integer" },
                "page": { "type": "integer" },
                "limit": { "type": "integer" }
            },
            "required": ["user_id"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/users/{user_id}");
        assert!(result.is_ok());

        let params = result.unwrap();
        assert_eq!(params.len(), 3);

        for param in params {
            if let RefOr::T(p) = param {
                if p.name == "user_id" {
                    assert!(matches!(p.parameter_in, utoipa::openapi::path::ParameterIn::Path));
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                } else {
                    assert!(matches!(p.parameter_in, utoipa::openapi::path::ParameterIn::Query));
                    assert!(matches!(p.required, utoipa::openapi::Required::False));
                }
            }
        }
    }

    #[test]
    fn test_json_value_to_schema_string_with_format() {
        let schema_json = serde_json::json!({
            "type": "string",
            "format": "date-time"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_schema_to_request_body_empty_object() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {}
        });

        let result = json_schema_to_request_body(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_parameters_error_on_missing_properties() {
        let param_schema = serde_json::json!({
            "type": "object"
        });

        let result = extract_parameters_from_schema(&param_schema, "/users");
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.contains("properties"));
        }
    }
}
