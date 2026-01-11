use spikard_bindings_shared::{ConfigExtractor, ConfigSource};
use std::collections::HashMap;

#[derive(Debug)]
struct JsonSource {
    value: serde_json::Value,
}

impl JsonSource {
    #[allow(clippy::missing_const_for_fn)]
    fn new(value: serde_json::Value) -> Self {
        Self { value }
    }

    fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.value.as_object()?.get(key)
    }
}

impl ConfigSource for JsonSource {
    fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }

    fn get_u64(&self, key: &str) -> Option<u64> {
        self.get(key)?.as_u64()
    }

    fn get_u16(&self, key: &str) -> Option<u16> {
        self.get_u64(key).and_then(|v| u16::try_from(v).ok())
    }

    fn get_string(&self, key: &str) -> Option<String> {
        self.get(key)?.as_str().map(ToOwned::to_owned)
    }

    fn get_vec_string(&self, key: &str) -> Option<Vec<String>> {
        self.get(key)?.as_array().map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(ToOwned::to_owned))
                .collect()
        })
    }

    fn get_nested(&self, key: &str) -> Option<Box<dyn ConfigSource + '_>> {
        let nested = self.get(key)?.as_object()?;
        Some(Box::new(Self {
            value: serde_json::Value::Object(nested.clone()),
        }))
    }

    fn has_key(&self, key: &str) -> bool {
        self.value.as_object().is_some_and(|obj| obj.contains_key(key))
    }

    fn get_array_length(&self, key: &str) -> Option<usize> {
        self.get(key)?.as_array().map(Vec::len)
    }

    fn get_array_element(&self, key: &str, index: usize) -> Option<Box<dyn ConfigSource + '_>> {
        let array = self.get(key)?.as_array()?;
        let element = array.get(index)?.as_object()?;
        Some(Box::new(Self {
            value: serde_json::Value::Object(element.clone()),
        }))
    }
}

#[test]
fn server_config_defaults() {
    let source = JsonSource::new(serde_json::json!({}));
    let cfg = ConfigExtractor::extract_server_config(&source).expect("defaults should extract");

    assert_eq!(cfg.host, "127.0.0.1");
    assert_eq!(cfg.port, 8000);
    assert_eq!(cfg.workers, 1);
    assert!(!cfg.enable_request_id);
    assert!(cfg.graceful_shutdown);
    assert_eq!(cfg.shutdown_timeout, 30);
    assert_eq!(cfg.max_body_size, Some(10 * 1024 * 1024));
    assert!(cfg.request_timeout.is_none());
    assert!(cfg.compression.is_none());
    assert!(cfg.rate_limit.is_none());
    assert!(cfg.jwt_auth.is_none());
    assert!(cfg.api_key_auth.is_none());
    assert!(cfg.static_files.is_empty());
    assert!(cfg.openapi.is_none());
    assert!(cfg.jsonrpc.is_none());
}

#[test]
fn server_config_parses_nested_configs_and_static_files() {
    let source = JsonSource::new(serde_json::json!({
        "Host": "0.0.0.0",
        "port": 9000,
        "workers": 4,
        "enable_request_id": false,
        "max_body_size": 1024,
        "request_timeout": 15,
        "graceful_shutdown": false,
        "shutdown_timeout": 10,
        "compression": { "gzip": true, "brotli": false, "min_size": 2, "quality": 1 },
        "jwt_auth": { "secret": "secret", "algorithm": "HS256", "audience": ["a"], "issuer": "i", "leeway": 2 },
        "api_key_auth": { "keys": ["k1", "k2"], "header_name": "X-Key" },
        "static_files": [
            { "directory": "./public", "route_prefix": "/static", "index_file": false, "cache_control": "max-age=60" }
        ],
        "openapi": { "enabled": true, "title": "T", "version": "V" },
        "jsonrpc": { "enabled": true }
    }));

    let cfg = ConfigExtractor::extract_server_config(&source).expect("config should extract");

    assert_eq!(cfg.host, "0.0.0.0");
    assert_eq!(cfg.port, 9000);
    assert_eq!(cfg.workers, 4);
    assert!(!cfg.enable_request_id);
    assert_eq!(cfg.max_body_size, Some(1024));
    assert_eq!(cfg.request_timeout, Some(15));
    assert!(!cfg.graceful_shutdown);
    assert_eq!(cfg.shutdown_timeout, 10);

    let compression = cfg.compression.expect("compression parsed");
    assert!(compression.gzip);
    assert!(!compression.brotli);
    assert_eq!(compression.min_size, 2);
    assert_eq!(compression.quality, 1);

    let jwt = cfg.jwt_auth.expect("jwt parsed");
    assert_eq!(jwt.secret, "secret");
    assert_eq!(jwt.algorithm, "HS256");
    assert_eq!(jwt.audience, Some(vec!["a".to_string()]));
    assert_eq!(jwt.issuer.as_deref(), Some("i"));
    assert_eq!(jwt.leeway, 2);

    let api_key = cfg.api_key_auth.expect("api key parsed");
    assert_eq!(api_key.keys, vec!["k1".to_string(), "k2".to_string()]);
    assert_eq!(api_key.header_name, "X-Key");

    assert_eq!(cfg.static_files.len(), 1);
    assert_eq!(cfg.static_files[0].directory, "./public");
    assert_eq!(cfg.static_files[0].route_prefix, "/static");
    assert!(!cfg.static_files[0].index_file);
    assert_eq!(cfg.static_files[0].cache_control.as_deref(), Some("max-age=60"));

    assert!(cfg.openapi.is_some());
    assert!(cfg.jsonrpc.is_some());
}

#[test]
fn static_files_validation_errors_are_surfaceable() {
    let source = JsonSource::new(serde_json::json!({
        "static_files": [
            { "route_prefix": "/static" }
        ]
    }));

    let err = ConfigExtractor::extract_server_config(&source).expect_err("missing directory should error");
    assert!(err.contains("Static files requires 'directory'"), "got: {err}");
}

#[test]
fn rate_limit_requires_expected_keys() {
    let missing_per_second = JsonSource::new(serde_json::json!({ "burst": 10 }));
    let err = ConfigExtractor::extract_rate_limit_config(&missing_per_second).expect_err("missing per_second");
    assert!(err.contains("per_second"), "got: {err}");

    let missing_burst = JsonSource::new(serde_json::json!({ "per_second": 5 }));
    let err = ConfigExtractor::extract_rate_limit_config(&missing_burst).expect_err("missing burst");
    assert!(err.contains("burst"), "got: {err}");
}

#[test]
fn openapi_security_schemes_are_parsed() {
    let source = JsonSource::new(serde_json::json!({
        "enabled": true,
        "title": "API",
        "version": "1.0",
        "security_schemes": {
            "BearerAuth": { "type": "http", "scheme": "bearer", "bearer_format": "JWT" }
        }
    }));

    let cfg = ConfigExtractor::extract_openapi_config(&source).expect("openapi config should parse");
    assert!(cfg.enabled);
    let schemes: HashMap<String, _> = cfg.security_schemes;
    assert!(
        schemes.is_empty(),
        "security scheme extraction is intentionally unsupported until ConfigSource can iterate map keys"
    );
}
