# CLI Usage

`spikard` is a codegen-first CLI with project scaffolding, AsyncAPI testing
helpers, validation, and an MCP server.

## Install

```bash
cargo install spikard-cli
```

## Top-Level Commands

```text
spikard init <name> --lang <language> [--dir <parent>]
spikard generate <target> ...
spikard testing asyncapi <target> ...
spikard validate-asyncapi <schema>
spikard features
spikard mcp
```

## Init

Create a starter project for a supported binding:

```bash
spikard init my_api --lang python --dir .
```

Supported languages: `python`, `typescript`, `rust`, `ruby`, `php`, `elixir`

## Generate

Generate code from a contract or schema:

```bash
# OpenAPI
spikard generate openapi openapi.yaml --lang python --output ./generated.py

# AsyncAPI
spikard generate asyncapi asyncapi.yaml --lang typescript --output ./src/handlers.ts

# JSON-RPC from OpenRPC
spikard generate jsonrpc rpc-spec.json --lang ruby --output ./lib/generated.rb

# GraphQL
spikard generate graphql schema.graphql --lang typescript --target all --output ./src/generated.ts

# Protobuf / gRPC
spikard generate protobuf user.proto --lang rust --output ./src/generated.rs

# PHP DTO helpers
spikard generate php-dto --output ./src/Generated
```

## AsyncAPI Testing Helpers

```bash
spikard testing asyncapi fixtures chat.asyncapi.yaml --output ./testing_data
spikard testing asyncapi test-app chat.asyncapi.yaml --lang elixir --output ./e2e/elixir
spikard testing asyncapi all chat.asyncapi.yaml --output ./generated
```

## Validation

```bash
spikard validate-asyncapi chat.asyncapi.yaml
spikard features
```

## MCP

Start the MCP server over stdio:

```bash
spikard mcp
```

With HTTP transport:

```bash
cargo run -p spikard-cli --features mcp-http -- mcp --transport http --host 127.0.0.1 --port 3001
```

## Runtime Note

The CLI does not currently provide a generic `run` or `serve` command for
applications. Start apps using the generated or binding-native entrypoints:

- Python: `uv run python -m my_api.app`
- TypeScript: `node src/server.ts` or the package's dev script
- Rust: `cargo run`
- Ruby: `bundle exec ruby bin/server`
- PHP: `php bin/server.php`
- Elixir: `mix run run.exs`
