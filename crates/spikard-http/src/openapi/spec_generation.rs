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

    #[test]
    fn test_route_to_path_item_get() {
        let route = RouteMetadata {
            method: "GET".to_string(),
            path: "/users".to_string(),
            handler_name: "list_users".to_string(),
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

        let result = route_to_path_item(&route);
        assert!(result.is_ok());
    }
}
