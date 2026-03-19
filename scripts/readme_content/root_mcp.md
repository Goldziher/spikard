The CLI ships with an MCP server by default, exposing the same init and code generation surface over stdio:

```bash
spikard mcp
```

That MCP surface includes `init_project`, `generate_openapi`, `generate_asyncapi_handlers`, `generate_jsonrpc`, `generate_graphql`, `generate_protobuf`, `generate_asyncapi_fixtures`, `generate_asyncapi_test_app`, `generate_asyncapi_bundle`, `validate_asyncapi`, and `get_features`.

For HTTP transport, build or install with the `mcp-http` feature and run:

```bash
cargo run -p spikard-cli --features mcp-http -- mcp --transport http --host 127.0.0.1 --port 3001
```
