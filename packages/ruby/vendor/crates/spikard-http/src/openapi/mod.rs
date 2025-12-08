//! OpenAPI 3.1.0 specification generation
//!
//! Generates OpenAPI specs from route definitions using existing JSON Schema infrastructure.
//! OpenAPI 3.1.0 is fully compatible with JSON Schema Draft 2020-12.

pub mod parameter_extraction;
pub mod schema_conversion;
pub mod spec_generation;

use crate::SchemaRegistry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::openapi::security::SecurityScheme;

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

/// Convert SecuritySchemeInfo to OpenAPI SecurityScheme
pub fn security_scheme_info_to_openapi(info: &SecuritySchemeInfo) -> SecurityScheme {
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

/// Generate OpenAPI specification from routes with auto-detection of security schemes
pub fn generate_openapi_spec(
    routes: &[crate::RouteMetadata],
    config: &OpenApiConfig,
    _schema_registry: &SchemaRegistry,
    server_config: Option<&crate::ServerConfig>,
) -> Result<utoipa::openapi::OpenApi, String> {
    spec_generation::assemble_openapi_spec(routes, config, server_config)
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
}
