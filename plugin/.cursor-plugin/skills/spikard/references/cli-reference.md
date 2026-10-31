<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:c4ed906524a7768898f2fe4d2495f02b04a2192232b8e60bb44e8fcbb7a2875b
Source-Hash: blake3:abccd7eb49522b31a5fd250dd2cb28fb991f8bdb6528b9fd032c3d50821ccf1b
Schema-Version: v1
-->

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
spikard generate sql queries --schema schema.sql --lang python --output generated
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
