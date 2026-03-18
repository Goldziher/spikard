# Spikard CLI Reference

Primary commands:

```bash
spikard init NAME --lang python --dir .
spikard mcp
spikard generate openapi SCHEMA --lang python
spikard generate asyncapi SCHEMA --lang typescript --output app.ts
spikard generate jsonrpc SCHEMA --lang ruby --output handlers.rb
spikard generate graphql SCHEMA --lang rust --target all
spikard generate protobuf SCHEMA --lang php --output src/Proto.php
spikard generate php-dto --output src/Generated
spikard testing asyncapi fixtures SCHEMA --output testing_data
spikard testing asyncapi test-app SCHEMA --lang python --output app.py
spikard testing asyncapi all SCHEMA --output e2e
spikard validate-asyncapi SCHEMA
spikard features
```

Notes:
- `spikard mcp` starts the stdio MCP server.
- `generate openapi` and `generate jsonrpc` can emit in-memory output when no `--output` is supplied.
- `generate asyncapi`, `generate protobuf`, and AsyncAPI test-app generation require explicit output paths.
- `generate php-dto` writes helper DTO classes into a directory, defaulting to `src/Generated`.

Read [mcp-tools.md](mcp-tools.md) when using the server form of these capabilities.
