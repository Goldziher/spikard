pub fn is_http_fixture_category(category: &str) -> bool {
    !matches!(
        category,
        "asyncapi_schemas"
            | "openapi_schemas"
            | "openrpc_schemas"
            | "protobuf"
            | "jsonrpc"
            | "sse"
            | "websockets"
            | "scripts"
    )
}
