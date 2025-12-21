//! ServerConfig extraction from Ruby objects.
//!
//! This module handles converting Ruby ServerConfig objects to the Rust
//! spikard_http::ServerConfig type.

use magnus::prelude::*;
use magnus::{Error, RArray, RHash, Ruby, TryConvert, Value};
use spikard_http::{
    ApiKeyConfig, CompressionConfig, ContactInfo, JwtConfig, LicenseInfo, OpenApiConfig, RateLimitConfig, ServerInfo,
    StaticFilesConfig,
};
use std::collections::HashMap;

/// Extract ServerConfig from Ruby ServerConfig object
pub fn extract_server_config(ruby: &Ruby, config_value: Value) -> Result<spikard_http::ServerConfig, Error> {
    let host: String = config_value.funcall("host", ())?;

    let port: u32 = config_value.funcall("port", ())?;

    let workers: usize = config_value.funcall("workers", ())?;

    let enable_request_id: bool = config_value.funcall("enable_request_id", ())?;

    let max_body_size_value: Value = config_value.funcall("max_body_size", ())?;
    let max_body_size = if max_body_size_value.is_nil() {
        None
    } else {
        Some(u64::try_convert(max_body_size_value)? as usize)
    };

    let request_timeout_value: Value = config_value.funcall("request_timeout", ())?;
    let request_timeout = if request_timeout_value.is_nil() {
        None
    } else {
        Some(u64::try_convert(request_timeout_value)?)
    };

    let graceful_shutdown: bool = config_value.funcall("graceful_shutdown", ())?;

    let shutdown_timeout: u64 = config_value.funcall("shutdown_timeout", ())?;

    let compression_value: Value = config_value.funcall("compression", ())?;
    let compression = if compression_value.is_nil() {
        None
    } else {
        let gzip: bool = compression_value.funcall("gzip", ())?;
        let brotli: bool = compression_value.funcall("brotli", ())?;
        let min_size: usize = compression_value.funcall("min_size", ())?;
        let quality: u32 = compression_value.funcall("quality", ())?;
        Some(CompressionConfig {
            gzip,
            brotli,
            min_size,
            quality,
        })
    };

    let rate_limit_value: Value = config_value.funcall("rate_limit", ())?;
    let rate_limit = if rate_limit_value.is_nil() {
        None
    } else {
        let per_second: u64 = rate_limit_value.funcall("per_second", ())?;
        let burst: u32 = rate_limit_value.funcall("burst", ())?;
        let ip_based: bool = rate_limit_value.funcall("ip_based", ())?;
        Some(RateLimitConfig {
            per_second,
            burst,
            ip_based,
        })
    };

    let jwt_auth_value: Value = config_value.funcall("jwt_auth", ())?;
    let jwt_auth = if jwt_auth_value.is_nil() {
        None
    } else {
        let secret: String = jwt_auth_value.funcall("secret", ())?;
        let algorithm: String = jwt_auth_value.funcall("algorithm", ())?;
        let audience_value: Value = jwt_auth_value.funcall("audience", ())?;
        let audience = if audience_value.is_nil() {
            None
        } else {
            Some(Vec::<String>::try_convert(audience_value)?)
        };
        let issuer_value: Value = jwt_auth_value.funcall("issuer", ())?;
        let issuer = if issuer_value.is_nil() {
            None
        } else {
            Some(String::try_convert(issuer_value)?)
        };
        let leeway: u64 = jwt_auth_value.funcall("leeway", ())?;
        Some(JwtConfig {
            secret,
            algorithm,
            audience,
            issuer,
            leeway,
        })
    };

    let api_key_auth_value: Value = config_value.funcall("api_key_auth", ())?;
    let api_key_auth = if api_key_auth_value.is_nil() {
        None
    } else {
        let keys: Vec<String> = api_key_auth_value.funcall("keys", ())?;
        let header_name: String = api_key_auth_value.funcall("header_name", ())?;
        Some(ApiKeyConfig { keys, header_name })
    };

    let static_files_value: Value = config_value.funcall("static_files", ())?;
    let static_files_array = RArray::from_value(static_files_value)
        .ok_or_else(|| Error::new(ruby.exception_type_error(), "static_files must be an Array"))?;

    let mut static_files = Vec::new();
    for i in 0..static_files_array.len() {
        let sf_value = static_files_array.entry::<Value>(i as isize)?;
        let directory: String = sf_value.funcall("directory", ())?;
        let route_prefix: String = sf_value.funcall("route_prefix", ())?;
        let index_file: bool = sf_value.funcall("index_file", ())?;
        let cache_control_value: Value = sf_value.funcall("cache_control", ())?;
        let cache_control = if cache_control_value.is_nil() {
            None
        } else {
            Some(String::try_convert(cache_control_value)?)
        };
        static_files.push(StaticFilesConfig {
            directory,
            route_prefix,
            index_file,
            cache_control,
        });
    }

    let openapi_value: Value = config_value.funcall("openapi", ())?;
    let openapi = if openapi_value.is_nil() {
        None
    } else {
        let enabled: bool = openapi_value.funcall("enabled", ())?;
        let title: String = openapi_value.funcall("title", ())?;
        let version: String = openapi_value.funcall("version", ())?;
        let description_value: Value = openapi_value.funcall("description", ())?;
        let description = if description_value.is_nil() {
            None
        } else {
            Some(String::try_convert(description_value)?)
        };
        let swagger_ui_path: String = openapi_value.funcall("swagger_ui_path", ())?;
        let redoc_path: String = openapi_value.funcall("redoc_path", ())?;
        let openapi_json_path: String = openapi_value.funcall("openapi_json_path", ())?;

        let contact_value: Value = openapi_value.funcall("contact", ())?;
        let contact = if contact_value.is_nil() {
            None
        } else if let Some(contact_hash) = RHash::from_value(contact_value) {
            let name = get_optional_string_from_hash(contact_hash, "name")?;
            let email = get_optional_string_from_hash(contact_hash, "email")?;
            let url = get_optional_string_from_hash(contact_hash, "url")?;
            Some(ContactInfo { name, email, url })
        } else {
            let name_value: Value = contact_value.funcall("name", ())?;
            let email_value: Value = contact_value.funcall("email", ())?;
            let url_value: Value = contact_value.funcall("url", ())?;
            Some(ContactInfo {
                name: if name_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(name_value)?)
                },
                email: if email_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(email_value)?)
                },
                url: if url_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(url_value)?)
                },
            })
        };

        let license_value: Value = openapi_value.funcall("license", ())?;
        let license = if license_value.is_nil() {
            None
        } else if let Some(license_hash) = RHash::from_value(license_value) {
            let name = get_required_string_from_hash(license_hash, "name", ruby)?;
            let url = get_optional_string_from_hash(license_hash, "url")?;
            Some(LicenseInfo { name, url })
        } else {
            let name: String = license_value.funcall("name", ())?;
            let url_value: Value = license_value.funcall("url", ())?;
            let url = if url_value.is_nil() {
                None
            } else {
                Some(String::try_convert(url_value)?)
            };
            Some(LicenseInfo { name, url })
        };

        let servers_value: Value = openapi_value.funcall("servers", ())?;
        let servers_array = RArray::from_value(servers_value)
            .ok_or_else(|| Error::new(ruby.exception_type_error(), "servers must be an Array"))?;

        let mut servers = Vec::new();
        for i in 0..servers_array.len() {
            let server_value = servers_array.entry::<Value>(i as isize)?;

            let (url, description) = if let Some(server_hash) = RHash::from_value(server_value) {
                let url = get_required_string_from_hash(server_hash, "url", ruby)?;
                let description = get_optional_string_from_hash(server_hash, "description")?;
                (url, description)
            } else {
                let url: String = server_value.funcall("url", ())?;
                let description_value: Value = server_value.funcall("description", ())?;
                let description = if description_value.is_nil() {
                    None
                } else {
                    Some(String::try_convert(description_value)?)
                };
                (url, description)
            };

            servers.push(ServerInfo { url, description });
        }

        let security_schemes = HashMap::new();

        Some(OpenApiConfig {
            enabled,
            title,
            version,
            description,
            swagger_ui_path,
            redoc_path,
            openapi_json_path,
            contact,
            license,
            servers,
            security_schemes,
        })
    };

    Ok(spikard_http::ServerConfig {
        host,
        port: port as u16,
        workers,
        enable_request_id,
        max_body_size,
        request_timeout,
        compression,
        rate_limit,
        jwt_auth,
        api_key_auth,
        static_files,
        graceful_shutdown,
        shutdown_timeout,
        background_tasks: spikard_http::BackgroundTaskConfig::default(),
        enable_http_trace: false,
        openapi,
        jsonrpc: None,
        lifecycle_hooks: None,
        di_container: None,
    })
}

/// Helper to extract an optional string from a Ruby Hash
pub fn get_optional_string_from_hash(hash: RHash, key: &str) -> Result<Option<String>, Error> {
    match hash.get(String::from(key)) {
        Some(v) if !v.is_nil() => Ok(Some(String::try_convert(v)?)),
        _ => Ok(None),
    }
}

/// Helper to extract a required string from a Ruby Hash
pub fn get_required_string_from_hash(hash: RHash, key: &str, ruby: &Ruby) -> Result<String, Error> {
    let value = hash
        .get(String::from(key))
        .ok_or_else(|| Error::new(ruby.exception_arg_error(), format!("missing required key '{}'", key)))?;
    if value.is_nil() {
        return Err(Error::new(
            ruby.exception_arg_error(),
            format!("key '{}' cannot be nil", key),
        ));
    }
    String::try_convert(value)
}
