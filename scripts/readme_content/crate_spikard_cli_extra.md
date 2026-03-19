### testing asyncapi

Generate AsyncAPI fixtures, language-specific test apps, or a full bundle:

```bash
spikard testing asyncapi fixtures ./chat.asyncapi.yaml --output ./testing_data
spikard testing asyncapi test-app ./chat.asyncapi.yaml --lang elixir --output ./e2e/elixir
spikard testing asyncapi all ./chat.asyncapi.yaml --output ./generated
```

### mcp

Expose the same init and code generation surface over MCP stdio:

```bash
spikard mcp
```

For streamable HTTP transport, build with `--features mcp-http` and run:

```bash
cargo run -p spikard-cli --features mcp-http -- mcp --transport http --host 127.0.0.1 --port 3001
```
