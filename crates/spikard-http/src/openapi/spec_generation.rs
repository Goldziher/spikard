//! OpenAPI specification generation and assembly

use crate::RouteMetadata;
use utoipa::openapi::HttpMethod;
use utoipa::openapi::security::SecurityScheme;
use utoipa::openapi::{Components, Info, OpenApi, OpenApiBuilder, PathItem, Paths, RefOr, Response, Responses};

/// Convert route to OpenAPI PathItem
fn route_to_path_item(route: &RouteMetadata) -> Result<PathItem, String> {
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
        let parameters =
            crate::openapi::parameter_extraction::extract_parameters_from_schema(param_schema, &route.path)?;
        if !parameters.is_empty() {
            let unwrapped: Vec<_> = parameters
                .into_iter()
                .filter_map(|p| if let RefOr::T(param) = p { Some(param) } else { None })
                .collect();
            operation.parameters = Some(unwrapped);
        }
    }

    if let Some(request_schema) = &route.request_schema {
        let request_body = crate::openapi::schema_conversion::json_schema_to_request_body(request_schema)?;
        operation.request_body = Some(request_body);
    }

    let mut responses = Responses::new();
    if let Some(response_schema) = &route.response_schema {
        let response = crate::openapi::schema_conversion::json_schema_to_response(response_schema)?;
        responses.responses.insert("200".to_string(), RefOr::T(response));
    } else {
        responses
            .responses
            .insert("200".to_string(), RefOr::T(Response::new("Successful response")));
    }
    operation.responses = responses;

    Ok(operation)
}

/// Assemble OpenAPI specification from routes with auto-detection of security schemes
pub fn assemble_openapi_spec(
    routes: &[RouteMetadata],
    config: &super::OpenApiConfig,
    server_config: Option<&crate::ServerConfig>,
) -> Result<OpenApi, String> {
    let mut info = Info::new(&config.title, &config.version);
    if let Some(desc) = &config.description {
        info.description = Some(desc.clone());
    }
    if let Some(contact_info) = &config.contact {
        let mut contact = utoipa::openapi::Contact::default();
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
        let mut license = utoipa::openapi::License::new(&license_info.name);
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
                    let mut server = utoipa::openapi::Server::new(&s.url);
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
            let scheme = crate::openapi::security_scheme_info_to_openapi(scheme_info);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ApiKeyConfig, JwtConfig};

    fn make_route(method: &str, path: &str) -> RouteMetadata {
        RouteMetadata {
            method: method.to_string(),
            path: path.to_string(),
            handler_name: format!("{}_handler", method.to_lowercase()),
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
            jsonrpc_method: None,
        }
    }

    fn make_server_config_with_jwt() -> crate::ServerConfig {
        crate::ServerConfig {
            jwt_auth: Some(JwtConfig {
                secret: "test-secret".to_string(),
                algorithm: "HS256".to_string(),
                audience: None,
                issuer: None,
                leeway: 0,
            }),
            ..Default::default()
        }
    }

    fn make_server_config_with_api_key() -> crate::ServerConfig {
        crate::ServerConfig {
            api_key_auth: Some(ApiKeyConfig {
                keys: vec!["test-key".to_string()],
                header_name: "X-API-Key".to_string(),
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_route_to_path_item_get() {
        let route = make_route("GET", "/users");
        let result = route_to_path_item(&route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_to_path_item_post() {
        let route = make_route("POST", "/users");
        let result = route_to_path_item(&route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_to_path_item_put() {
        let route = make_route("PUT", "/users/123");
        let result = route_to_path_item(&route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_to_path_item_patch() {
        let route = make_route("PATCH", "/users/123");
        let result = route_to_path_item(&route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_to_path_item_delete() {
        let route = make_route("DELETE", "/users/123");
        let result = route_to_path_item(&route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_to_path_item_head() {
        let route = make_route("HEAD", "/users");
        let result = route_to_path_item(&route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_to_path_item_options() {
        let route = make_route("OPTIONS", "/users");
        let result = route_to_path_item(&route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_to_path_item_case_insensitive_method() {
        let route_lower = make_route("get", "/users");
        let route_mixed = make_route("GeT", "/users");

        assert!(route_to_path_item(&route_lower).is_ok());
        assert!(route_to_path_item(&route_mixed).is_ok());
    }

    #[test]
    fn test_route_to_path_item_unsupported_method() {
        let route = make_route("CONNECT", "/users");
        let result = route_to_path_item(&route);
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.contains("Unsupported HTTP method"));
        }
    }

    #[test]
    fn test_assemble_openapi_spec_minimal() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        };

        let result = assemble_openapi_spec(&[], &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert_eq!(spec.info.title, "Test API");
        assert_eq!(spec.info.version, "1.0.0");
    }

    #[test]
    fn test_assemble_openapi_spec_with_description() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            description: Some("This is a test API".to_string()),
            ..Default::default()
        };

        let result = assemble_openapi_spec(&[], &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert_eq!(spec.info.description, Some("This is a test API".to_string()));
    }

    #[test]
    fn test_assemble_openapi_spec_with_contact() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            contact: Some(super::super::ContactInfo {
                name: Some("Support Team".to_string()),
                email: Some("support@example.com".to_string()),
                url: Some("https://example.com/support".to_string()),
            }),
            ..Default::default()
        };

        let result = assemble_openapi_spec(&[], &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(spec.info.contact.is_some());
        let contact = spec.info.contact.unwrap();
        assert_eq!(contact.name, Some("Support Team".to_string()));
        assert_eq!(contact.email, Some("support@example.com".to_string()));
    }

    #[test]
    fn test_assemble_openapi_spec_with_license() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            license: Some(super::super::LicenseInfo {
                name: "Apache 2.0".to_string(),
                url: Some("https://www.apache.org/licenses/LICENSE-2.0.html".to_string()),
            }),
            ..Default::default()
        };

        let result = assemble_openapi_spec(&[], &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(spec.info.license.is_some());
        let license = spec.info.license.unwrap();
        assert_eq!(license.name, "Apache 2.0");
        assert_eq!(
            license.url,
            Some("https://www.apache.org/licenses/LICENSE-2.0.html".to_string())
        );
    }

    #[test]
    fn test_assemble_openapi_spec_with_servers() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            servers: vec![
                super::super::ServerInfo {
                    url: "https://api.example.com".to_string(),
                    description: Some("Production".to_string()),
                },
                super::super::ServerInfo {
                    url: "http://localhost:8080".to_string(),
                    description: Some("Development".to_string()),
                },
            ],
            ..Default::default()
        };

        let result = assemble_openapi_spec(&[], &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(spec.servers.is_some());
        let servers = spec.servers.unwrap();
        assert_eq!(servers.len(), 2);
    }

    #[test]
    fn test_assemble_openapi_spec_with_jwt_auth() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        };

        let server_config = make_server_config_with_jwt();
        let result = assemble_openapi_spec(&[], &config, Some(&server_config));
        assert!(result.is_ok());
        let spec = result.unwrap();

        assert!(spec.components.is_some());
        let components = spec.components.unwrap();
        assert!(components.security_schemes.get("bearerAuth").is_some());

        assert!(spec.security.is_some());
        let security_reqs = spec.security.unwrap();
        assert!(!security_reqs.is_empty());
        assert_eq!(security_reqs.len(), 1);
    }

    #[test]
    fn test_assemble_openapi_spec_with_api_key_auth() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        };

        let server_config = make_server_config_with_api_key();
        let result = assemble_openapi_spec(&[], &config, Some(&server_config));
        assert!(result.is_ok());
        let spec = result.unwrap();

        assert!(spec.components.is_some());
        let components = spec.components.unwrap();
        assert!(components.security_schemes.get("apiKeyAuth").is_some());

        assert!(spec.security.is_some());
        let security_reqs = spec.security.unwrap();
        assert!(!security_reqs.is_empty());
        assert_eq!(security_reqs.len(), 1);
    }

    #[test]
    fn test_assemble_openapi_spec_with_both_auth_schemes() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        };

        let mut server_config = make_server_config_with_jwt();
        server_config.api_key_auth = Some(ApiKeyConfig {
            keys: vec!["test-key".to_string()],
            header_name: "X-API-Key".to_string(),
        });

        let result = assemble_openapi_spec(&[], &config, Some(&server_config));
        assert!(result.is_ok());
        let spec = result.unwrap();

        assert!(spec.components.is_some());
        let components = spec.components.unwrap();
        assert!(components.security_schemes.get("bearerAuth").is_some());
        assert!(components.security_schemes.get("apiKeyAuth").is_some());
    }

    #[test]
    fn test_assemble_openapi_spec_with_custom_security_schemes() {
        use std::collections::HashMap;

        let mut security_schemes = HashMap::new();
        security_schemes.insert(
            "oauth2".to_string(),
            super::super::SecuritySchemeInfo::Http {
                scheme: "bearer".to_string(),
                bearer_format: Some("OAuth2".to_string()),
            },
        );

        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            security_schemes,
            ..Default::default()
        };

        let result = assemble_openapi_spec(&[], &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();

        assert!(spec.components.is_some());
        let components = spec.components.unwrap();
        assert!(components.security_schemes.get("oauth2").is_some());
    }

    #[test]
    fn test_assemble_openapi_spec_with_multiple_routes() {
        let routes: Vec<RouteMetadata> = vec![
            make_route("GET", "/users"),
            make_route("POST", "/users"),
            make_route("GET", "/users/{id}"),
            make_route("PUT", "/users/{id}"),
            make_route("DELETE", "/users/{id}"),
        ];

        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "User API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        };

        let result = assemble_openapi_spec(&routes, &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();

        assert!(!spec.paths.paths.is_empty());
        assert!(spec.paths.paths.contains_key("/users"));
        assert!(spec.paths.paths.contains_key("/users/{id}"));
    }

    #[test]
    fn test_route_to_operation_default_response() {
        let route = make_route("GET", "/health");
        let result = route_to_operation(&route);

        assert!(result.is_ok());
        let operation = result.unwrap();
        assert!(!operation.responses.responses.is_empty());
        assert!(operation.responses.responses.contains_key("200"));
    }

    #[test]
    fn test_assemble_openapi_spec_empty_routes() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Empty API".to_string(),
            version: "0.1.0".to_string(),
            ..Default::default()
        };

        let result = assemble_openapi_spec(&[], &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(spec.paths.paths.is_empty());
    }

    #[test]
    fn test_assemble_openapi_spec_with_partial_contact() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            contact: Some(super::super::ContactInfo {
                name: Some("Support".to_string()),
                email: None,
                url: None,
            }),
            ..Default::default()
        };

        let result = assemble_openapi_spec(&[], &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();
        let contact = spec.info.contact.unwrap();
        assert_eq!(contact.name, Some("Support".to_string()));
        assert!(contact.email.is_none());
    }

    #[test]
    fn test_assemble_openapi_spec_without_servers() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            servers: vec![],
            ..Default::default()
        };

        let result = assemble_openapi_spec(&[], &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(spec.servers.is_none());
    }

    #[test]
    fn test_route_to_path_item_lowercase_method() {
        let route = make_route("post", "/items");
        let result = route_to_path_item(&route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_to_path_item_mixed_case_method() {
        let route = make_route("PoSt", "/items");
        let result = route_to_path_item(&route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_assemble_openapi_spec_preserves_route_order() {
        let routes: Vec<RouteMetadata> = vec![
            make_route("GET", "/a"),
            make_route("GET", "/b"),
            make_route("GET", "/c"),
        ];

        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        };

        let result = assemble_openapi_spec(&routes, &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();

        assert!(spec.paths.paths.contains_key("/a"));
        assert!(spec.paths.paths.contains_key("/b"));
        assert!(spec.paths.paths.contains_key("/c"));
    }

    #[test]
    fn test_assemble_openapi_spec_with_server_config_none() {
        let config = super::super::OpenApiConfig {
            enabled: true,
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        };

        let result = assemble_openapi_spec(&[], &config, None);
        assert!(result.is_ok());
        let spec = result.unwrap();
        if let Some(components) = spec.components {
            assert!(!components.security_schemes.contains_key("bearerAuth"));
            assert!(!components.security_schemes.contains_key("apiKeyAuth"));
        }
    }

    #[test]
    fn test_route_to_operation_with_no_schemas() {
        let route = RouteMetadata {
            method: "GET".to_string(),
            path: "/endpoint".to_string(),
            handler_name: "test_handler".to_string(),
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
            jsonrpc_method: None,
        };

        let result = route_to_operation(&route);
        assert!(result.is_ok());
        let operation = result.unwrap();
        assert!(operation.request_body.is_none());
        assert!(operation.responses.responses.contains_key("200"));
    }
}
