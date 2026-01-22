//! Python bindings for `spikard`
//!
//! This crate provides Python bindings using `PyO3`

#![allow(clippy::similar_names)] // Common in FFI code
#![allow(clippy::missing_errors_doc)] // Many FFI functions return Result
#![allow(clippy::doc_markdown)] // FFI types don't need backticks
#![allow(clippy::missing_const_for_fn)] // FFI functions can't be const
#![allow(clippy::too_many_arguments)] // FFI bridge functions often need many parameters
#![allow(clippy::too_many_lines)] // FFI wrappers accumulate code
#![allow(clippy::unused_self)] // PyO3 methods may not use self
#![allow(clippy::unnecessary_wraps)] // PyO3 patterns require Result wrappers
#![allow(clippy::must_use_candidate)] // FFI constructors follow Rust patterns
#![allow(clippy::struct_excessive_bools)] // FFI configs use multiple bools
#![allow(clippy::fn_params_excessive_bools)] // FFI builders pass multiple bools
#![allow(clippy::items_after_statements)] // Common in Rust code
#![allow(clippy::if_not_else)] // FFI code style preference
#![allow(clippy::redundant_clone)] // May be necessary in FFI boundary
#![allow(clippy::uninlined_format_args)] // FFI error messages
#![allow(clippy::cast_lossless)] // Type conversions in FFI
#![allow(clippy::option_if_let_else)] // FFI error handling patterns
#![allow(clippy::missing_panics_doc)] // Runtime server panics acceptable in server context
#![allow(clippy::unused_async)] // Async trait methods may not await
#![allow(clippy::non_std_lazy_statics)] // using_once_cell pattern
#![allow(clippy::ptr_as_ptr)] // Raw pointer casts in FFI code
#![allow(clippy::ptr_cast_constness)] // Cast constness for FFI interop
#![allow(clippy::significant_drop_tightening)] // Drop timing in FFI bridges
#![allow(clippy::trivially_copy_pass_by_ref)] // FFI compatibility
#![allow(clippy::cast_possible_wrap)] // Cast wrapping in FFI
#![allow(clippy::cast_possible_truncation)] // Type size differences in FFI
#![allow(clippy::used_underscore_binding)] // Internal FFI code
#![allow(clippy::redundant_closure)] // FFI closure patterns
#![allow(clippy::explicit_iter_loop)] // FFI iteration style
#![allow(clippy::cast_sign_loss)] // Unsigned/signed casts in FFI
#![allow(clippy::map_unwrap_or)] // Idiomatic Option/Result handling
#![allow(clippy::implicit_clone)] // String conversions in FFI
#![allow(clippy::ref_option_ref)] // Reference patterns in FFI
#![allow(clippy::should_implement_trait)] // FFI trait implementation
#![allow(clippy::match_like_matches_macro)] // FFI match patterns
#![allow(clippy::match_bool)] // Boolean matching in FFI
#![allow(clippy::format_push_string)] // String formatting in FFI
#![allow(clippy::option_option)] // Option nesting in FFI
#![allow(clippy::enum_variant_names)] // FFI variant naming
#![allow(clippy::identity_op)] // FFI operations
#![allow(clippy::filter_next)] // Filter operations in FFI
#![allow(clippy::manual_let_else)] // Let-else patterns in FFI
#![allow(clippy::if_then_some_else_none)] // If-then-some patterns
#![allow(clippy::clone_on_copy)] // Clone on copy types in FFI
#![allow(clippy::unit_arg)] // Unit argument handling
#![allow(clippy::impl_trait_in_params)] // Trait parameters in FFI
#![allow(clippy::match_same_arms)] // Identical match arms
#![allow(clippy::needless_pass_by_value)] // FFI argument passing style
#![allow(clippy::ref_as_ptr)] // Explicit pointer casts in FFI
#![allow(clippy::while_let_on_iterator)] // Iterator patterns in FFI
#![allow(clippy::redundant_closure_for_method_calls)] // Closure patterns in FFI
#![allow(clippy::as_ptr_cast_mut)] // Raw pointer casting in FFI
#![allow(clippy::match_wildcard_for_single_variants)] // Wildcard patterns in FFI
#![allow(clippy::ignored_unit_patterns)] // Unit pattern handling in FFI
#![allow(clippy::option_as_ref_deref)] // Option reference patterns
#![allow(clippy::semicolon_if_nothing_returned)] // Return statement consistency
#![allow(clippy::map_identity)] // Identity mapping patterns

mod background;
pub mod conversion;
#[cfg(feature = "di")]
pub mod di;
#[cfg(feature = "graphql")]
pub mod graphql;
pub mod grpc;
pub mod handler;
mod handler_request;
pub mod lifecycle;
pub mod request;
pub mod response;
mod response_interpreter;
pub mod sse;
pub mod testing;
pub mod websocket;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use spikard_http::RouteMetadata;
use spikard_http::server::Server;

pub use handler::{PythonHandler, init_python_event_loop};

/// Route with Python `handler`
pub struct RouteWithHandler {
    pub metadata: RouteMetadata,
    pub handler: Py<PyAny>,
}

/// Extract routes from a Python Spikard application instance (internal function)
///
/// This function is meant to be called from Rust code that has GIL access.
/// It's not exposed as a Python function.
pub fn extract_routes_from_app(py: Python<'_>, app: &Bound<'_, PyAny>) -> PyResult<Vec<RouteWithHandler>> {
    let routes_list = app.call_method0("get_routes")?;

    let mut routes = Vec::new();

    for route_obj in routes_list.cast::<PyList>()?.iter() {
        let metadata = extract_route_metadata(py, &route_obj)?;

        let handler: Py<PyAny> = route_obj.getattr("handler")?.into();

        routes.push(RouteWithHandler { metadata, handler });
    }

    Ok(routes)
}

/// Extract route metadata from a Python Route object
fn extract_route_metadata(py: Python<'_>, route: &Bound<'_, PyAny>) -> PyResult<RouteMetadata> {
    let method: String = route.getattr("method")?.extract()?;
    let path: String = route.getattr("path")?.extract()?;
    let handler_name: String = route.getattr("handler_name")?.extract()?;
    let is_async: bool = route.getattr("is_async")?.extract()?;

    let request_schema_value = extract_json_field(py, route, "request_schema")?;
    let response_schema_value = extract_json_field(py, route, "response_schema")?;
    let parameter_schema_value = extract_json_field(py, route, "parameter_schema")?;
    let file_params_value = extract_json_field(py, route, "file_params")?;

    let body_param_name = route.getattr("body_param_name")?;
    let body_param_name_value = if body_param_name.is_none() {
        None
    } else {
        Some(body_param_name.extract()?)
    };

    let handler_dependencies = {
        let deps = match route.getattr("handler_dependencies") {
            Ok(value) => value,
            Err(_) => py.None().into_bound(py),
        };
        if deps.is_none() { None } else { Some(deps.extract()?) }
    };

    let jsonrpc_method_value = {
        let jsonrpc_method = match route.getattr("jsonrpc_method") {
            Ok(value) => value,
            Err(_) => py.None().into_bound(py),
        };
        if jsonrpc_method.is_none() {
            None
        } else {
            extract_jsonrpc_method_info(py, &jsonrpc_method)?
        }
    };

    Ok(RouteMetadata {
        method,
        path,
        handler_name,
        request_schema: request_schema_value,
        response_schema: response_schema_value,
        parameter_schema: parameter_schema_value,
        file_params: file_params_value,
        is_async,
        cors: None,
        body_param_name: body_param_name_value,
        handler_dependencies,
        jsonrpc_method: jsonrpc_method_value,
    })
}

fn extract_json_field(py: Python<'_>, route: &Bound<'_, PyAny>, field: &str) -> PyResult<Option<serde_json::Value>> {
    let value = route.getattr(field)?;
    if value.is_none() {
        return Ok(None);
    }
    py_to_json_value(py, &value).map(Some)
}

fn extract_jsonrpc_method_info(
    py: Python<'_>,
    jsonrpc_method: &Bound<'_, PyAny>,
) -> PyResult<Option<serde_json::Value>> {
    let dict = match jsonrpc_method.call_method0("to_dict") {
        Ok(dict_result) => dict_result,
        Err(_) => {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "jsonrpc_method must be a JsonRpcMethodInfo instance with a to_dict() method or a dict. \
                 Received object type that doesn't support to_dict() conversion.",
            ));
        }
    };

    if dict.cast::<PyDict>().is_err() {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "to_dict() must return a dictionary, got different type",
        ));
    }

    let dict_obj = dict.cast::<PyDict>()?;
    if !dict_obj.contains("method_name")? {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "JsonRpcMethodInfo.to_dict() is missing required field 'method_name'",
        ));
    }

    match dict_obj.get_item("method_name")? {
        Some(method_name_obj) => {
            if method_name_obj.extract::<String>().is_err() {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "'method_name' must be a string",
                ));
            }
        }
        None => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "'method_name' must not be None",
            ));
        }
    }

    py_to_json_value(py, &dict).map(Some)
}

#[allow(clippy::only_used_in_recursion)]
fn py_to_json_value(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
    if obj.is_none() {
        return Ok(serde_json::Value::Null);
    }
    if let Ok(b) = obj.extract::<bool>() {
        return Ok(serde_json::Value::Bool(b));
    }
    if let Ok(i) = obj.extract::<i64>() {
        return Ok(serde_json::Value::Number(serde_json::Number::from(i)));
    }
    if let Ok(f) = obj.extract::<f64>() {
        return serde_json::Number::from_f64(f)
            .map(serde_json::Value::Number)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("NaN not supported in JSON"));
    }
    if let Ok(s) = obj.extract::<String>() {
        return Ok(serde_json::Value::String(s));
    }
    if let Ok(seq) = obj.cast::<PyList>() {
        let mut items = Vec::with_capacity(seq.len());
        for item in seq {
            items.push(py_to_json_value(py, &item)?);
        }
        return Ok(serde_json::Value::Array(items));
    }
    if let Ok(dict) = obj.cast::<PyDict>() {
        let mut map = serde_json::Map::with_capacity(dict.len());
        for (k, v) in dict {
            let key: String = k.extract()?;
            map.insert(key, py_to_json_value(py, &v)?);
        }
        return Ok(serde_json::Value::Object(map));
    }

    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
        "Unsupported type for JSON conversion in route metadata",
    ))
}

/// Process using `spikard` (legacy function)
#[pyfunction]
fn process() -> PyResult<()> {
    Ok(())
}

/// Create a test client from a Spikard application
///
/// Args:
///     app: A Spikard application instance
///
/// Returns:
///     TestClient: A test client for making requests to the app
#[pyfunction]
fn create_test_client(py: Python<'_>, app: &Bound<'_, PyAny>) -> PyResult<testing::client::TestClient> {
    let write_debug_log = |name: &str, contents: &str| {
        let path = std::env::temp_dir().join(name);
        let _ = std::fs::write(path, contents);
    };

    // DEBUG: Log test client creation
    write_debug_log("create_test_client.log", "create_test_client() called\n");
    eprintln!("[UNCONDITIONAL DEBUG] create_test_client() called");

    spikard_http::debug::init();

    let routes_with_handlers = extract_routes_from_app(py, app)?;
    write_debug_log(
        "routes_extracted.log",
        &format!("Extracted {} routes\n", routes_with_handlers.len()),
    );

    let schema_registry = spikard_http::SchemaRegistry::new();

    let routes: Vec<_> = routes_with_handlers
        .into_iter()
        .filter_map(|r| {
            let has_explicit_parameter_schema = r.metadata.parameter_schema.is_some();
            eprintln!(
                "[UNCONDITIONAL DEBUG] Route: {} {} has_explicit_parameter_schema={}",
                r.metadata.method, r.metadata.path, has_explicit_parameter_schema
            );

            let metadata_clone = r.metadata.clone();

            match spikard_http::Route::from_metadata(r.metadata, &schema_registry) {
                Ok(route) => Some((route, metadata_clone, r.handler)),
                Err(e) => {
                    eprintln!("[UNCONDITIONAL DEBUG] Failed to create route: {e}");
                    None
                }
            }
        })
        .collect();

    write_debug_log("routes_converted.log", &format!("Converted {} routes\n", routes.len()));

    #[cfg(feature = "di")]
    let mut config = if let Ok(py_config) = app.getattr("_config") {
        if !py_config.is_none() {
            extract_server_config(py, &py_config)?
        } else {
            spikard_http::ServerConfig::default()
        }
    } else {
        spikard_http::ServerConfig::default()
    };
    #[cfg(not(feature = "di"))]
    let config = if let Ok(py_config) = app.getattr("_config") {
        if !py_config.is_none() {
            extract_server_config(py, &py_config)?
        } else {
            spikard_http::ServerConfig::default()
        }
    } else {
        spikard_http::ServerConfig::default()
    };

    #[cfg(feature = "di")]
    {
        use std::sync::Arc;

        let dependencies = app.call_method0("get_dependencies")?;
        if !dependencies.is_none() {
            config.di_container = Some(Arc::new(build_dependency_container(py, &dependencies)?));
        }
    }

    eprintln!(
        "[UNCONDITIONAL DEBUG] Building Axum router with {} routes",
        routes.len()
    );

    let route_metadata: Vec<spikard_http::RouteMetadata> =
        routes.iter().map(|(_, metadata, _)| metadata.clone()).collect();

    let handler_routes: Vec<(spikard_http::Route, std::sync::Arc<dyn spikard_http::Handler>)> = routes
        .into_iter()
        .map(|(route, metadata, py_handler)| {
            let python_handler = PythonHandler::new(
                py_handler,
                route.is_async,
                route.response_validator.clone(),
                route.parameter_validator.clone(),
                metadata.body_param_name.clone(),
            );
            let arc_handler: std::sync::Arc<dyn spikard_http::Handler> = std::sync::Arc::new(python_handler);
            (route, arc_handler)
        })
        .collect();

    let mut axum_router = Server::with_handlers_and_metadata(config, handler_routes, route_metadata)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to build router: {e}")))?;

    use axum::routing::get;

    let websocket_handlers = app.call_method0("get_websocket_handlers")?;
    let ws_dict = websocket_handlers.cast::<pyo3::types::PyDict>()?;
    for (path, factory) in ws_dict.iter() {
        let path_str: String = path.extract()?;
        let ws_state = crate::websocket::create_websocket_state(&factory)?;
        eprintln!("[spikard-test] Registered WebSocket endpoint: {path_str}");

        axum_router = axum_router.route(
            &path_str,
            get(spikard_http::websocket_handler::<crate::websocket::PythonWebSocketHandler>).with_state(ws_state),
        );
    }

    let sse_producers = app.call_method0("get_sse_producers")?;
    let sse_dict = sse_producers.cast::<pyo3::types::PyDict>()?;
    for (path, factory) in sse_dict.iter() {
        let path_str: String = path.extract()?;
        let sse_state = crate::sse::create_sse_state(&factory)?;
        eprintln!("[spikard-test] Registered SSE endpoint: {path_str}");

        axum_router = axum_router.route(
            &path_str,
            get(spikard_http::sse_handler::<crate::sse::PythonSseEventProducer>).with_state(sse_state),
        );
    }

    write_debug_log("axum_router_built.log", "Axum router built successfully\n");

    eprintln!("[UNCONDITIONAL DEBUG] Creating TestClient from Axum router");

    let client = testing::client::TestClient::from_router(axum_router)?;
    write_debug_log("test_client_created.log", "TestClient created successfully\n");

    Ok(client)
}

/// Convert Python ServerConfig to Rust ServerConfig
fn extract_server_config(_py: Python<'_>, py_config: &Bound<'_, PyAny>) -> PyResult<spikard_http::ServerConfig> {
    use spikard_http::{
        ApiKeyConfig, CompressionConfig, ContactInfo, JwtConfig, LicenseInfo, OpenApiConfig, RateLimitConfig,
        SecuritySchemeInfo, ServerConfig, ServerInfo, StaticFilesConfig,
    };
    use std::collections::HashMap;

    let host: String = py_config.getattr("host")?.extract()?;
    let port: u16 = py_config.getattr("port")?.extract()?;
    let workers: usize = py_config.getattr("workers")?.extract()?;
    let enable_request_id: bool = py_config.getattr("enable_request_id")?.extract()?;
    let graceful_shutdown: bool = py_config.getattr("graceful_shutdown")?.extract()?;
    let shutdown_timeout: u64 = py_config.getattr("shutdown_timeout")?.extract()?;

    let max_body_size: Option<usize> = py_config.getattr("max_body_size")?.extract()?;
    let request_timeout: Option<u64> = py_config.getattr("request_timeout")?.extract()?;

    let compression = py_config.getattr("compression")?;
    let compression_config = if compression.is_none() {
        None
    } else {
        let gzip: bool = compression.getattr("gzip")?.extract()?;
        let brotli: bool = compression.getattr("brotli")?.extract()?;
        let min_size: usize = compression.getattr("min_size")?.extract()?;
        let quality: u32 = compression.getattr("quality")?.extract()?;
        Some(CompressionConfig {
            gzip,
            brotli,
            min_size,
            quality,
        })
    };

    let rate_limit = py_config.getattr("rate_limit")?;
    let rate_limit_config = if rate_limit.is_none() {
        None
    } else {
        let per_second: u64 = rate_limit.getattr("per_second")?.extract()?;
        let burst: u32 = rate_limit.getattr("burst")?.extract()?;
        let ip_based: bool = rate_limit.getattr("ip_based")?.extract()?;
        Some(RateLimitConfig {
            per_second,
            burst,
            ip_based,
        })
    };

    let jwt_auth = py_config.getattr("jwt_auth")?;
    let jwt_auth_config = if jwt_auth.is_none() {
        None
    } else {
        let secret: String = jwt_auth.getattr("secret")?.extract()?;
        let algorithm: String = jwt_auth.getattr("algorithm")?.extract()?;
        let audience: Option<Vec<String>> = jwt_auth.getattr("audience")?.extract()?;
        let issuer: Option<String> = jwt_auth.getattr("issuer")?.extract()?;
        let leeway: u64 = jwt_auth.getattr("leeway")?.extract()?;
        Some(JwtConfig {
            secret,
            algorithm,
            audience,
            issuer,
            leeway,
        })
    };

    let api_key_auth = py_config.getattr("api_key_auth")?;
    let api_key_auth_config = if api_key_auth.is_none() {
        None
    } else {
        let keys: Vec<String> = api_key_auth.getattr("keys")?.extract()?;
        let header_name: String = api_key_auth.getattr("header_name")?.extract()?;
        Some(ApiKeyConfig { keys, header_name })
    };

    let static_files_list: Vec<Bound<'_, PyAny>> = py_config.getattr("static_files")?.extract()?;
    let static_files: Vec<StaticFilesConfig> = static_files_list
        .iter()
        .map(|sf| {
            let directory: String = sf.getattr("directory")?.extract()?;
            let route_prefix: String = sf.getattr("route_prefix")?.extract()?;
            let index_file: bool = sf.getattr("index_file")?.extract()?;
            let cache_control: Option<String> = sf.getattr("cache_control")?.extract()?;
            Ok(StaticFilesConfig {
                directory,
                route_prefix,
                index_file,
                cache_control,
            })
        })
        .collect::<PyResult<Vec<_>>>()?;

    let openapi_py = py_config.getattr("openapi")?;
    let openapi_config = if openapi_py.is_none() {
        None
    } else {
        let enabled: bool = openapi_py.getattr("enabled")?.extract()?;
        let title: String = openapi_py.getattr("title")?.extract()?;
        let version: String = openapi_py.getattr("version")?.extract()?;
        let description: Option<String> = openapi_py.getattr("description")?.extract()?;
        let swagger_ui_path: String = openapi_py.getattr("swagger_ui_path")?.extract()?;
        let redoc_path: String = openapi_py.getattr("redoc_path")?.extract()?;
        let openapi_json_path: String = openapi_py.getattr("openapi_json_path")?.extract()?;

        let contact_py = openapi_py.getattr("contact")?;
        let contact = if contact_py.is_none() {
            None
        } else {
            let name: Option<String> = contact_py.getattr("name")?.extract()?;
            let email: Option<String> = contact_py.getattr("email")?.extract()?;
            let url: Option<String> = contact_py.getattr("url")?.extract()?;
            Some(ContactInfo { name, email, url })
        };

        let license_py = openapi_py.getattr("license")?;
        let license = if license_py.is_none() {
            None
        } else {
            let name: String = license_py.getattr("name")?.extract()?;
            let url: Option<String> = license_py.getattr("url")?.extract()?;
            Some(LicenseInfo { name, url })
        };

        let servers_list: Vec<Bound<'_, PyAny>> = openapi_py.getattr("servers")?.extract()?;
        let servers: Vec<ServerInfo> = servers_list
            .iter()
            .map(|s| {
                let url: String = s.getattr("url")?.extract()?;
                let description: Option<String> = s.getattr("description")?.extract()?;
                Ok(ServerInfo { url, description })
            })
            .collect::<PyResult<Vec<_>>>()?;

        let security_schemes_dict: HashMap<String, Bound<'_, PyAny>> =
            openapi_py.getattr("security_schemes")?.extract()?;
        let security_schemes: HashMap<String, SecuritySchemeInfo> = security_schemes_dict
            .iter()
            .map(|(name, scheme_py)| {
                let scheme_type: String = scheme_py.getattr("type")?.extract()?;
                let scheme_info = match scheme_type.as_str() {
                    "http" => {
                        let scheme: String = scheme_py.getattr("scheme")?.extract()?;
                        let bearer_format: Option<String> = scheme_py.getattr("bearer_format")?.extract()?;
                        SecuritySchemeInfo::Http { scheme, bearer_format }
                    }
                    "apiKey" => {
                        let location: String = scheme_py.getattr("location")?.extract()?;
                        let param_name: String = scheme_py.getattr("name")?.extract()?;
                        SecuritySchemeInfo::ApiKey {
                            location,
                            name: param_name,
                        }
                    }
                    _ => {
                        return Err(pyo3::exceptions::PyValueError::new_err(format!(
                            "Invalid security scheme type: {scheme_type}"
                        )));
                    }
                };
                Ok((name.clone(), scheme_info))
            })
            .collect::<PyResult<HashMap<_, _>>>()?;

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

    Ok(ServerConfig {
        host,
        port,
        workers,
        enable_request_id,
        max_body_size,
        request_timeout,
        compression: compression_config,
        rate_limit: rate_limit_config,
        jwt_auth: jwt_auth_config,
        api_key_auth: api_key_auth_config,
        static_files,
        graceful_shutdown,
        shutdown_timeout,
        background_tasks: spikard_http::BackgroundTaskConfig::default(),
        enable_http_trace: false,
        openapi: openapi_config,
        jsonrpc: None,
        grpc: None,
        lifecycle_hooks: None,
        di_container: None,
    })
}

/// Build dependency container from Python dependencies
///
/// Converts Python dependencies (values and Provide wrappers) to Rust DependencyContainer
#[cfg(feature = "di")]
fn build_dependency_container(
    _py: Python<'_>,
    dependencies: &Bound<'_, PyAny>,
) -> PyResult<spikard_core::di::DependencyContainer> {
    use pyo3::types::PyDict;
    use spikard_core::di::DependencyContainer;
    use std::sync::Arc;

    let mut container = DependencyContainer::new();
    let deps_dict = dependencies.cast::<PyDict>()?;

    for (key, value) in deps_dict.iter() {
        let key_str: String = key.extract()?;

        if value.hasattr("dependency")? {
            let factory = value.getattr("dependency")?;
            let depends_on: Vec<String> = value.getattr("depends_on")?.extract().unwrap_or_default();
            let singleton: bool = value.getattr("singleton")?.extract().unwrap_or(false);
            let use_cache: bool = value.getattr("use_cache")?.extract().unwrap_or(false);
            let is_async: bool = value.getattr("is_async")?.extract().unwrap_or(false);
            let is_async_generator: bool = value.getattr("is_async_generator")?.extract().unwrap_or(false);

            let py_factory = factory.into();
            let factory_dep = crate::di::PythonFactoryDependency::new(
                key_str.clone(),
                py_factory,
                depends_on,
                singleton,
                use_cache || singleton,
                is_async,
                is_async_generator,
            );

            container.register(key_str, Arc::new(factory_dep)).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to register factory dependency: {}",
                    e
                ))
            })?;
        } else {
            let py_value = value.into();
            let value_dep = crate::di::PythonValueDependency::new(key_str.clone(), py_value);

            container.register(key_str, Arc::new(value_dep)).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to register value dependency: {e}"))
            })?;
        }
    }

    Ok(container)
}

/// Run Spikard server from Python
///
/// This function enables Python to run Spikard, rather than having the Rust CLI embed Python.
/// A dedicated Python thread is created to run an asyncio event loop for async handlers.
///
/// Args:
///     app: Spikard application instance
///     config: ServerConfig instance with all middleware settings
///
/// Example:
///     ```python
///     from spikard import Spikard, ServerConfig, CompressionConfig
///
///     config = ServerConfig(
///         host="0.0.0.0",
///         port=8080,
///         compression=CompressionConfig(quality=9)
///     )
///
///     app = Spikard()
///
///     @app.get("/")
///     async def root():
///         return {"message": "Hello"}
///
///     if __name__ == "__main__":
///         app.run(config=config)
///     ```
#[pyfunction]
#[pyo3(signature = (app, config))]
fn run_server(py: Python<'_>, app: &Bound<'_, PyAny>, config: &Bound<'_, PyAny>) -> PyResult<()> {
    use spikard_http::{Route, Server};
    use std::sync::Arc;

    let mut config = extract_server_config(py, config)?;

    if config.workers > 1 {
        eprintln!("⚠️  Multi-worker mode not yet implemented, using single worker");
    }

    init_python_event_loop()?;

    let routes_with_handlers = extract_routes_from_app(py, app)?;

    let hooks_dict = app.call_method0("get_lifecycle_hooks")?;
    let lifecycle_hooks = crate::lifecycle::build_lifecycle_hooks(py, &hooks_dict)?;

    config.lifecycle_hooks = Some(Arc::new(lifecycle_hooks));

    #[cfg(feature = "di")]
    {
        let dependencies = app.call_method0("get_dependencies")?;
        if !dependencies.is_none() {
            config.di_container = Some(Arc::new(build_dependency_container(py, &dependencies)?));
        }
    }

    let schema_registry = spikard_http::SchemaRegistry::new();

    let routes: Vec<(Route, Arc<dyn spikard_http::Handler>)> = routes_with_handlers
        .into_iter()
        .map(|rwh| {
            let path = rwh.metadata.path.clone();
            Route::from_metadata(rwh.metadata.clone(), &schema_registry)
                .map(|route| {
                    let python_handler = PythonHandler::new(
                        rwh.handler,
                        rwh.metadata.is_async,
                        route.response_validator.clone(),
                        route.parameter_validator.clone(),
                        rwh.metadata.body_param_name.clone(),
                    );
                    let arc_handler: Arc<dyn spikard_http::Handler> = Arc::new(python_handler);
                    (route, arc_handler)
                })
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to create route for {}: {}",
                        path, e
                    ))
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Server::init_logging();

    eprintln!("[spikard] Starting Spikard server (Python manages event loop)");
    eprintln!("[spikard] Registered {} routes", routes.len());
    eprintln!("[spikard] Listening on http://{}:{}", config.host, config.port);

    let mut app_router = Server::with_handlers(config.clone(), routes)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to build Axum router: {e}")))?;

    let websocket_handlers = app.call_method0("get_websocket_handlers")?;
    let ws_dict = websocket_handlers.cast::<pyo3::types::PyDict>()?;
    for (path, factory) in ws_dict.iter() {
        let path_str: String = path.extract()?;
        let ws_state = crate::websocket::create_websocket_state(&factory)?;
        eprintln!("[spikard] Registered WebSocket endpoint: {path_str}");

        use axum::routing::get;
        app_router = app_router.route(
            &path_str,
            get(spikard_http::websocket_handler::<crate::websocket::PythonWebSocketHandler>).with_state(ws_state),
        );
    }

    let sse_producers = app.call_method0("get_sse_producers")?;
    let sse_dict = sse_producers.cast::<pyo3::types::PyDict>()?;
    for (path, factory) in sse_dict.iter() {
        let path_str: String = path.extract()?;
        let sse_state = crate::sse::create_sse_state(&factory)?;
        eprintln!("[spikard] Registered SSE endpoint: {path_str}");

        use axum::routing::get;
        app_router = app_router.route(
            &path_str,
            get(spikard_http::sse_handler::<crate::sse::PythonSseEventProducer>).with_state(sse_state),
        );
    }

    py.detach(|| {
        // Spikard-Python is fundamentally bound by the Python GIL. Running the HTTP runtime on a
        // single Tokio thread significantly reduces cross-thread GIL handoff overhead under load.
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| {
                pyo3::Python::attach(|_py| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to create Tokio runtime: {e}"))
                })
            })?
            .block_on(async {
                let addr = format!("{}:{}", config.host, config.port);
                let socket_addr: std::net::SocketAddr = addr.parse().map_err(|e| {
                    pyo3::Python::attach(|_py| {
                        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid socket address {addr}: {e}"))
                    })
                })?;

                let listener = tokio::net::TcpListener::bind(socket_addr).await.map_err(|e| {
                    pyo3::Python::attach(|_py| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                            "Failed to bind to {}:{}: {}",
                            config.host, config.port, e
                        ))
                    })
                })?;

                eprintln!("[spikard] Server listening on {socket_addr}");

                let background_runtime = spikard_http::BackgroundRuntime::start(config.background_tasks.clone()).await;
                crate::background::install_handle(background_runtime.handle());

                let serve_result = axum::serve(listener, app_router).await;

                crate::background::clear_handle();
                let shutdown_result = background_runtime.shutdown().await;

                serve_result.map_err(|e| {
                    pyo3::Python::attach(|_py| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Server error: {e}"))
                    })
                })?;

                shutdown_result.map_err(|_| {
                    pyo3::Python::attach(|_py| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                            "Failed to drain background tasks during shutdown",
                        )
                    })
                })
            })
    })
}

/// Python module for `spikard`
#[pymodule]
pub fn _spikard(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<request::PyRequest>()?;
    m.add_class::<handler_request::PyHandlerRequest>()?;
    m.add_class::<response::Response>()?;
    m.add_class::<response::StreamingResponse>()?;
    m.add_class::<testing::client::TestClient>()?;
    m.add_class::<testing::client::TestResponse>()?;
    m.add_class::<testing::websocket::WebSocketTestConnection>()?;
    m.add_class::<testing::websocket::WebSocketMessage>()?;
    m.add_class::<testing::sse::SseStream>()?;
    m.add_class::<testing::sse::SseEvent>()?;
    m.add_function(wrap_pyfunction!(background::background_run, m)?)?;
    m.add_function(wrap_pyfunction!(create_test_client, m)?)?;
    m.add_function(wrap_pyfunction!(process, m)?)?;
    m.add_function(wrap_pyfunction!(run_server, m)?)?;

    #[cfg(feature = "graphql")]
    {
        m.add_class::<graphql::PySchemaConfig>()?;
        m.add_class::<graphql::PySchemaBuilder>()?;
    }

    // Add gRPC classes
    grpc::init_module(m)?;

    Ok(())
}
