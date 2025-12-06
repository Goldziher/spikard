# Config Extraction Infrastructure

## Overview

The config extraction infrastructure in `spikard-bindings-shared` provides a trait-based abstraction for extracting `ServerConfig` and related configuration structs from language-specific objects without duplicating code across bindings (Python, Node.js, Ruby, PHP, WASM).

This eliminates approximately 720 lines of duplicated configuration extraction logic (~180 lines per binding × 4 bindings).

## Architecture

### ConfigSource Trait

The `ConfigSource` trait provides a unified interface for reading configuration values from language-specific objects:

```rust
pub trait ConfigSource {
    fn get_bool(&self, key: &str) -> Option<bool>;
    fn get_u64(&self, key: &str) -> Option<u64>;
    fn get_u16(&self, key: &str) -> Option<u16>;
    fn get_string(&self, key: &str) -> Option<String>;
    fn get_vec_string(&self, key: &str) -> Option<Vec<String>>;
    fn get_nested(&self, key: &str) -> Option<Box<dyn ConfigSource + '_>>;
    fn has_key(&self, key: &str) -> bool;
    fn get_array_length(&self, key: &str) -> Option<usize>;
    fn get_array_element(&self, key: &str, index: usize) -> Option<Box<dyn ConfigSource + '_>>;
    // Helper methods...
    fn get_u32(&self, key: &str) -> Option<u32>;
    fn get_usize(&self, key: &str) -> Option<usize>;
}
```

### ConfigExtractor Methods

`ConfigExtractor` provides static methods that work with any `ConfigSource` implementation:

- `extract_server_config()` - Extracts complete `ServerConfig`
- `extract_compression_config()` - Extracts compression settings
- `extract_rate_limit_config()` - Extracts rate limiting settings
- `extract_jwt_config()` - Extracts JWT authentication settings
- `extract_api_key_config()` - Extracts API Key authentication settings
- `extract_static_files_config()` - Extracts static file serving configuration
- `extract_openapi_config()` - Extracts OpenAPI documentation settings

## Implementation Guide

### Step 1: Implement ConfigSource for Your Language

Each binding needs to implement `ConfigSource` for its language-specific objects.

#### Python (PyO3) Example

```rust
use pyo3::prelude::*;
use pyo3::types::PyDict;
use spikard_bindings_shared::ConfigSource;

pub struct PyConfigSource<'a> {
    dict: &'a Bound<'a, PyDict>,
}

impl<'a> PyConfigSource<'a> {
    pub fn new(dict: &'a Bound<'a, PyDict>) -> Self {
        Self { dict }
    }
}

impl<'a> ConfigSource for PyConfigSource<'a> {
    fn get_bool(&self, key: &str) -> Option<bool> {
        self.dict
            .get_item(key)
            .ok()?
            .and_then(|val| val.extract::<bool>().ok())
    }

    fn get_u64(&self, key: &str) -> Option<u64> {
        self.dict
            .get_item(key)
            .ok()?
            .and_then(|val| val.extract::<u64>().ok())
    }

    // ... implement other methods ...
}
```

#### Node.js (napi-rs) Example

```rust
use napi::{JsObject, Env};
use spikard_bindings_shared::ConfigSource;

pub struct NapiConfigSource<'a> {
    obj: &'a JsObject,
    env: &'a Env,
}

impl<'a> NapiConfigSource<'a> {
    pub fn new(obj: &'a JsObject, env: &'a Env) -> Self {
        Self { obj, env }
    }
}

impl<'a> ConfigSource for NapiConfigSource<'a> {
    fn get_bool(&self, key: &str) -> Option<bool> {
        self.obj
            .get::<String>(key)
            .ok()
            .and_then(|val| val.get_boolean().ok())
    }

    fn get_u64(&self, key: &str) -> Option<u64> {
        self.obj
            .get::<String>(key)
            .ok()
            .and_then(|val| val.get_int64().ok().map(|v| v as u64))
    }

    // ... implement other methods ...
}
```

### Step 2: Use ConfigExtractor in Your Binding

Replace language-specific extraction code with calls to `ConfigExtractor`:

#### Before (Python - ~180 lines)

```rust
fn extract_server_config(py: Python<'_>, py_config: &Bound<'_, PyAny>) -> PyResult<ServerConfig> {
    let host: String = py_config.getattr("host")?.extract()?;
    let port: u16 = py_config.getattr("port")?.extract()?;
    // ... 170+ more lines of extraction code ...
}
```

#### After (Python - ~5 lines)

```rust
fn extract_server_config(py: Python<'_>, py_config: &Bound<'_, PyDict>) -> PyResult<ServerConfig> {
    let config_source = PyConfigSource::new(py_config);
    ConfigExtractor::extract_server_config(&config_source)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))
}
```

## Configuration Structure

### ServerConfig

Top-level server configuration containing all sub-configs:

```rust
pub struct ServerConfig {
    pub host: String,              // default: "127.0.0.1"
    pub port: u16,                 // default: 8000
    pub workers: usize,            // default: 1
    pub enable_request_id: bool,   // default: true
    pub max_body_size: Option<usize>,
    pub request_timeout: Option<u64>,
    pub compression: Option<CompressionConfig>,
    pub rate_limit: Option<RateLimitConfig>,
    pub jwt_auth: Option<JwtConfig>,
    pub api_key_auth: Option<ApiKeyConfig>,
    pub static_files: Vec<StaticFilesConfig>,
    pub graceful_shutdown: bool,   // default: true
    pub shutdown_timeout: u64,     // default: 30
    pub openapi: Option<OpenApiConfig>,
    // ...
}
```

### CompressionConfig

```rust
pub struct CompressionConfig {
    pub gzip: bool,        // default: true
    pub brotli: bool,      // default: true
    pub min_size: usize,   // default: 1024
    pub quality: u32,      // default: 6
}
```

### RateLimitConfig

```rust
pub struct RateLimitConfig {
    pub per_second: u64,   // REQUIRED
    pub burst: u32,        // REQUIRED
    pub ip_based: bool,    // default: true
}
```

### JwtConfig

```rust
pub struct JwtConfig {
    pub secret: String,                    // REQUIRED
    pub algorithm: String,                 // default: "HS256"
    pub audience: Option<Vec<String>>,
    pub issuer: Option<String>,
    pub leeway: u64,                       // default: 0
}
```

### ApiKeyConfig

```rust
pub struct ApiKeyConfig {
    pub keys: Vec<String>,                 // REQUIRED
    pub header_name: String,               // default: "X-API-Key"
}
```

### StaticFilesConfig

```rust
pub struct StaticFilesConfig {
    pub directory: String,                 // REQUIRED
    pub route_prefix: String,              // REQUIRED
    pub index_file: bool,                  // default: true
    pub cache_control: Option<String>,
}
```

### OpenApiConfig

```rust
pub struct OpenApiConfig {
    pub enabled: bool,                     // default: false
    pub title: String,                     // default: "API"
    pub version: String,                   // default: "1.0.0"
    pub description: Option<String>,
    pub swagger_ui_path: String,           // default: "/docs"
    pub redoc_path: String,                // default: "/redoc"
    pub openapi_json_path: String,         // default: "/openapi.json"
    pub contact: Option<ContactInfo>,
    pub license: Option<LicenseInfo>,
    pub servers: Vec<ServerInfo>,
    pub security_schemes: HashMap<String, SecuritySchemeInfo>,
}
```

## Error Handling

All extraction methods return `Result<T, String>` with descriptive error messages:

```rust
// Missing required field
let config = ConfigExtractor::extract_rate_limit_config(&source)?;
// Err: "Rate limit requires 'per_second'"

// Missing required nested field
let config = ConfigExtractor::extract_api_key_config(&source)?;
// Err: "API Key auth requires 'keys' as Vec<String>)"
```

## Testing

The infrastructure includes comprehensive unit tests:

```bash
cargo test -p spikard-bindings-shared --lib config_extractor
```

Test coverage includes:
- Successful extraction with explicit values
- Default value application
- Missing required field error handling
- Type conversion edge cases

## Examples

See `examples/config_extraction.rs` for a complete working example:

```bash
cargo run -p spikard-bindings-shared --example config_extraction
```

## Migration Checklist

For each language binding:

- [ ] Create wrapper struct implementing `ConfigSource` for language-specific objects
- [ ] Implement all required trait methods
- [ ] Replace ~180 lines of extraction code with `ConfigExtractor` calls
- [ ] Update error handling to map `Result<T, String>` to language-specific errors
- [ ] Run binding tests to verify parity with original code
- [ ] Remove duplicated extraction code

## Performance

- **Zero-copy trait design**: Methods return `Option<T>` to avoid unnecessary allocations
- **Lazy evaluation**: Nested configs only parsed when accessed
- **Default values**: Applied efficiently without allocating
- **No reflection overhead**: Direct method dispatch on trait implementations

## Future Enhancements

- [ ] Support for HashMap/dict iteration in `extract_security_schemes_config()`
- [ ] CORS configuration extraction
- [ ] Custom middleware configuration extraction
- [ ] Validation schema integration

## References

- Architecture Decision Record: `docs/adr/0002-runtime-and-middleware.md`
- Server configuration examples: `examples/`
- Test fixtures: `testing_data/`
