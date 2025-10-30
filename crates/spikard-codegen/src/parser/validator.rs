//! Configuration validation

use crate::error::{CodegenError, Result};
use crate::parser::Config;

/// Validate configuration structure and constraints
pub fn validate_config(config: &Config) -> Result<()> {
    // Validate version
    if config.version != "1.0" {
        return Err(CodegenError::ValidationError(format!(
            "Unsupported configuration version: {}. Expected 1.0",
            config.version
        )));
    }

    // Validate service name
    if config.name.is_empty() {
        return Err(CodegenError::ValidationError(
            "Service name cannot be empty".to_string(),
        ));
    }

    // Validate HTTP routes
    if let Some(ref http) = config.http {
        for route in &http.routes {
            validate_route(route)?;
        }
    }

    // TODO: Validate gRPC services
    // TODO: Validate queue consumers/producers
    // TODO: Validate CloudEvents subscriptions

    Ok(())
}

fn validate_route(route: &crate::parser::config::HttpRoute) -> Result<()> {
    // Validate path
    if !route.path.starts_with('/') {
        return Err(CodegenError::ValidationError(format!(
            "Route path must start with '/': {}",
            route.path
        )));
    }

    // Validate method
    let valid_methods = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];
    if !valid_methods.contains(&route.method.as_str()) {
        return Err(CodegenError::ValidationError(format!(
            "Invalid HTTP method: {}. Must be one of: {}",
            route.method,
            valid_methods.join(", ")
        )));
    }

    // Validate handler path
    if route.handler.is_empty() {
        return Err(CodegenError::ValidationError(
            "Handler path cannot be empty".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::config::*;

    #[test]
    fn test_validate_valid_config() {
        let config = Config {
            version: "1.0".to_string(),
            name: "test-service".to_string(),
            description: None,
            runtime: None,
            http: Some(HttpConfig {
                routes: vec![HttpRoute {
                    path: "/users".to_string(),
                    method: "GET".to_string(),
                    handler: "handlers.get_users".to_string(),
                    parameters: None,
                    request: None,
                    response: None,
                    errors: None,
                    middleware: None,
                    tracing: None,
                    metrics: None,
                }],
                middleware: None,
            }),
            grpc: None,
            queues: None,
            cloudevents: None,
            schemas: None,
            openapi: None,
        };

        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_invalid_version() {
        let mut config = Config {
            version: "2.0".to_string(),
            name: "test-service".to_string(),
            description: None,
            runtime: None,
            http: None,
            grpc: None,
            queues: None,
            cloudevents: None,
            schemas: None,
            openapi: None,
        };

        let result = validate_config(&config);
        assert!(matches!(result, Err(CodegenError::ValidationError(_))));
    }

    #[test]
    fn test_validate_empty_service_name() {
        let config = Config {
            version: "1.0".to_string(),
            name: "".to_string(),
            description: None,
            runtime: None,
            http: None,
            grpc: None,
            queues: None,
            cloudevents: None,
            schemas: None,
            openapi: None,
        };

        let result = validate_config(&config);
        assert!(matches!(result, Err(CodegenError::ValidationError(_))));
    }
}
