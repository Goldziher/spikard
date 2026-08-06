---
title: "MCP Server"
---

## MCP Server

The Spikard CLI ships a [Model Context Protocol](https://modelcontextprotocol.io) server, so a coding
agent can scaffold projects and drive every codegen pipeline without a human typing commands.

It is built on [`rmcp`](https://crates.io/crates/rmcp) and is **enabled by default** — the `mcp`
feature is in `spikard-cli`'s default feature set, so a normal `cargo install spikard-cli` gets it.

!!! warning "Experimental"

    Spikard is experimental and pre-1.0. Tool names and parameters may change between releases.

### Install

```bash
cargo install spikard-cli
# or, without compiling from source:
cargo binstall spikard-cli
```

### Transports

| Transport | Availability | Command |
|---|---|---|
| stdio | default | `spikard mcp` |
| streamable HTTP | requires the `mcp-http` feature | `spikard mcp --transport http --host 127.0.0.1 --port 3001` |

`--transport` defaults to `stdio`. `--host` defaults to `127.0.0.1` and `--port` to `3001`. Building
with `--features all` enables both `mcp` and `mcp-http`.

### Client configuration

#### Claude Code

```bash
claude mcp add spikard -- spikard mcp
```

#### Cursor, Claude Desktop, and other JSON-configured clients

```json
{
  "mcpServers": {
    "spikard": {
      "command": "spikard",
      "args": ["mcp"]
    }
  }
}
```

For the HTTP transport, start the server separately and point the client at
`http://127.0.0.1:3001`.

### Tools

Thirteen tools are exposed. Descriptions below are the ones the server actually advertises.

#### Project setup

| Tool | Read-only | What it does |
|---|---|---|
| `init_project` | no | Initialize a new Spikard project in the requested language and return the created files and next steps. |
| `get_features` | yes | Return the current Spikard feature summary and binding installation hints. |

#### Specification codegen

| Tool | Read-only | What it does |
|---|---|---|
| `generate_openapi` | no | Generate Spikard server handlers from an OpenAPI schema. |
| `generate_asyncapi_handlers` | no | Generate AsyncAPI handler scaffolding for a target language. |
| `generate_jsonrpc` | no | Generate JSON-RPC handlers from an OpenRPC schema. |
| `generate_graphql` | no | Generate GraphQL types, resolvers, or schema definitions for a target language. |
| `generate_protobuf` | no | Generate Protobuf messages and gRPC services for a target language. |
| `validate_asyncapi` | yes | Validate an AsyncAPI schema and return its protocol and channel summary. |

#### SQL to HTTP

| Tool | Read-only | What it does |
|---|---|---|
| `generate_sql` | no | Generate routes, an OpenAPI 3.1 spec, and a language sidecar from annotated SQL queries (via scythe). |

#### Testing and integration

| Tool | Read-only | What it does |
|---|---|---|
| `generate_asyncapi_fixtures` | no | Generate AsyncAPI test fixtures used by Spikard's codegen-first testing flows. |
| `generate_asyncapi_test_app` | no | Generate a language-specific AsyncAPI test application. |
| `generate_asyncapi_bundle` | no | Generate AsyncAPI fixtures and test apps for all supported languages. |
| `generate_php_dto` | no | Generate the PHP DTO classes used for Spikard integrations. |

### A worked agent flow

The tools compose into the codegen-first workflow the toolkit is built around. A typical agent session:

1. **`init_project`** — scaffold a project in the target language and get back the created files.
2. **`generate_openapi`** — turn an existing OpenAPI document into typed handlers and validators for
   that language.
3. **`generate_sql`** — point at annotated `.sql` files and get routes, an OpenAPI 3.1 spec, and a
   typed sidecar. Query parsing and type inference come from
   [scythe](https://github.com/Goldziher/scythe); Spikard overlays the HTTP vocabulary. See the
   [SQL codegen guide](guides/sql-codegen.md).
4. **`validate_asyncapi`** — check an event schema before generating against it, since it is read-only
   and safe to call speculatively.
5. **`generate_asyncapi_bundle`** — produce fixtures and test apps across every supported language, so
   cross-language behaviour is verified rather than assumed.

Because every step writes real files rather than mutating hidden state, the diff is reviewable — the
agent generates, and a human reads the output before it ships.

### Related

- [CLI usage](cli/usage.md) — the same operations as direct commands.
- [Code generation](guides/code-generation.md) — what each generator emits.
- [SQL codegen](guides/sql-codegen.md) — the full annotation reference.
