//! OpenRPC 1.3.2 specification generation from registered JSON-RPC methods.

use super::JsonRpcMethodRegistry;
use serde::Serialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize)]
struct OpenRpcSpec {
    openrpc: &'static str,
    info: OpenRpcInfo,
    methods: Vec<OpenRpcMethod>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    servers: Vec<OpenRpcServer>,
    #[serde(default)]
    components: OpenRpcComponents,
}

#[derive(Debug, Serialize)]
struct OpenRpcInfo {
    title: String,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<OpenRpcContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<OpenRpcLicense>,
}

#[derive(Debug, Serialize)]
struct OpenRpcContact {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

#[derive(Debug, Serialize)]
struct OpenRpcLicense {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

#[derive(Debug, Serialize)]
struct OpenRpcServer {
    name: String,
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Serialize)]
struct OpenRpcMethod {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    params: Vec<OpenRpcParam>,
    result: OpenRpcResult,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    errors: Vec<OpenRpcError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    examples: Vec<OpenRpcExample>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<OpenRpcTag>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    deprecated: bool,
}

#[derive(Debug, Serialize)]
struct OpenRpcTag {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Serialize)]
struct OpenRpcParam {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    required: bool,
    schema: Value,
}

#[derive(Debug, Serialize)]
struct OpenRpcResult {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    schema: Value,
}

#[derive(Debug, Serialize)]
struct OpenRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

#[derive(Debug, Serialize)]
struct OpenRpcExample {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    params: Vec<OpenRpcExampleParam>,
    result: OpenRpcExampleResult,
}

#[derive(Debug, Serialize)]
struct OpenRpcExampleParam {
    name: String,
    value: Value,
}

#[derive(Debug, Serialize)]
struct OpenRpcExampleResult {
    name: String,
    value: Value,
}

#[derive(Debug, Default, Serialize)]
struct OpenRpcComponents {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    schemas: HashMap<String, Value>,
}

pub fn generate_openrpc_spec(
    registry: &JsonRpcMethodRegistry,
    server_config: &crate::ServerConfig,
) -> Result<Value, String> {
    let mut methods = registry
        .list_all()
        .map_err(|err| format!("Failed to list JSON-RPC methods: {err}"))?;
    methods.sort_by(|(left, _, _), (right, _, _)| left.cmp(right));

    let info = if let Some(openapi) = server_config.openapi.as_ref() {
        OpenRpcInfo {
            title: openapi.title.clone(),
            version: openapi.version.clone(),
            description: openapi.description.clone(),
            contact: openapi.contact.as_ref().map(|contact| OpenRpcContact {
                name: contact.name.clone(),
                email: contact.email.clone(),
                url: contact.url.clone(),
            }),
            license: openapi.license.as_ref().map(|license| OpenRpcLicense {
                name: license.name.clone(),
                url: license.url.clone(),
            }),
        }
    } else {
        OpenRpcInfo {
            title: "Spikard JSON-RPC API".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            contact: None,
            license: None,
        }
    };

    let endpoint_path = server_config
        .jsonrpc
        .as_ref()
        .map(|config| config.endpoint_path.as_str())
        .unwrap_or("/rpc");
    let servers = openrpc_servers(server_config, endpoint_path);

    let spec = OpenRpcSpec {
        openrpc: "1.3.2",
        info,
        methods: methods
            .into_iter()
            .map(|(_, _, metadata)| method_to_openrpc_method(metadata))
            .collect(),
        servers,
        components: OpenRpcComponents::default(),
    };

    serde_json::to_value(spec).map_err(|err| format!("Failed to serialize OpenRPC spec: {err}"))
}

fn openrpc_servers(server_config: &crate::ServerConfig, endpoint_path: &str) -> Vec<OpenRpcServer> {
    if let Some(openapi) = server_config.openapi.as_ref()
        && !openapi.servers.is_empty()
    {
        return openapi
            .servers
            .iter()
            .enumerate()
            .map(|(idx, server)| OpenRpcServer {
                name: server
                    .description
                    .clone()
                    .unwrap_or_else(|| format!("server-{}", idx + 1)),
                url: join_server_url(&server.url, endpoint_path),
                description: server.description.clone(),
            })
            .collect();
    }

    vec![OpenRpcServer {
        name: "jsonrpc".to_string(),
        url: endpoint_path.to_string(),
        description: Some("JSON-RPC HTTP endpoint".to_string()),
    }]
}

fn join_server_url(base: &str, endpoint_path: &str) -> String {
    if endpoint_path.starts_with("http://") || endpoint_path.starts_with("https://") {
        endpoint_path.to_string()
    } else if endpoint_path.starts_with('/') {
        format!("{}{}", base.trim_end_matches('/'), endpoint_path)
    } else {
        format!("{}/{}", base.trim_end_matches('/'), endpoint_path)
    }
}

fn method_to_openrpc_method(metadata: super::MethodMetadata) -> OpenRpcMethod {
    OpenRpcMethod {
        summary: metadata.description.clone(),
        description: metadata.description.clone(),
        params: metadata
            .params_schema
            .as_ref()
            .map_or_else(Vec::new, extract_params_from_schema),
        result: OpenRpcResult {
            name: "result".to_string(),
            description: metadata.description.clone(),
            schema: metadata.result_schema.unwrap_or_else(|| serde_json::json!({})),
        },
        errors: metadata.error_schema.map_or_else(Vec::new, |schema| {
            vec![OpenRpcError {
                code: -32000,
                message: "Application error".to_string(),
                data: Some(schema),
            }]
        }),
        examples: metadata
            .examples
            .into_iter()
            .map(|example| OpenRpcExample {
                name: example.name,
                description: example.description,
                params: vec![OpenRpcExampleParam {
                    name: "params".to_string(),
                    value: example.params,
                }],
                result: OpenRpcExampleResult {
                    name: "result".to_string(),
                    value: example.result,
                },
            })
            .collect(),
        tags: metadata
            .tags
            .into_iter()
            .map(|tag| OpenRpcTag {
                name: tag,
                description: None,
            })
            .collect(),
        deprecated: metadata.deprecated,
        name: metadata.name,
    }
}

fn extract_params_from_schema(schema: &Value) -> Vec<OpenRpcParam> {
    let Some(schema_obj) = schema.as_object() else {
        return vec![single_params_descriptor(schema)];
    };

    if schema_obj.get("type").and_then(Value::as_str) != Some("object") {
        return vec![single_params_descriptor(schema)];
    }

    let Some(properties) = schema_obj.get("properties").and_then(Value::as_object) else {
        return vec![single_params_descriptor(schema)];
    };

    if properties.is_empty() {
        return Vec::new();
    }

    let required: HashSet<&str> = schema_obj
        .get("required")
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();

    let mut params: Vec<_> = properties
        .iter()
        .map(|(name, property_schema)| OpenRpcParam {
            name: name.clone(),
            description: property_schema
                .get("description")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            required: required.contains(name.as_str()),
            schema: property_schema.clone(),
        })
        .collect();
    params.sort_by(|left, right| left.name.cmp(&right.name));
    params
}

fn single_params_descriptor(schema: &Value) -> OpenRpcParam {
    OpenRpcParam {
        name: "params".to_string(),
        description: schema
            .get("description")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        required: true,
        schema: schema.clone(),
    }
}
