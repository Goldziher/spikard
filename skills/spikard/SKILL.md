---
name: spikard
description: >-
  Scaffold Spikard projects and generate code from OpenAPI, AsyncAPI, OpenRPC,
  GraphQL, and Protobuf schemas using the Spikard CLI or its MCP server. Use
  when building or updating Spikard-based apps, fixtures, test apps, or agent
  workflows that should prefer code generation over handwritten boilerplate.
license: MIT
metadata:
  author: spikard
  version: "1.0"
---

# Spikard Codegen-First Workflows

Spikard is a Rust-centric framework with polyglot bindings and a codegen-first CLI. For agent workflows, prefer the MCP server because it exposes the same scaffolding, generation, validation, and fixture flows as structured tools instead of shell output.

Use this skill when the task involves:
- Scaffolding a new Spikard project
- Generating handlers or types from API schemas
- Building AsyncAPI fixtures or language-specific test apps
- Validating AsyncAPI schemas before generation
- Driving Spikard from an agent through MCP instead of shelling out repeatedly

## Use MCP First

Start the tracked MCP server from the repo or installed binary:

```bash
spikard mcp
```

Default transport is `stdio`. HTTP transport exists behind the `mcp-http` Cargo feature and is not the default workflow.

Use the MCP tools for:
- `init_project`
- `generate_openapi`
- `generate_asyncapi_handlers`
- `generate_jsonrpc`
- `generate_graphql`
- `generate_protobuf`
- `generate_php_dto`
- `generate_asyncapi_fixtures`
- `generate_asyncapi_test_app`
- `generate_asyncapi_bundle`
- `validate_asyncapi`
- `get_features`

Read [references/mcp-tools.md](references/mcp-tools.md) when you need parameter details or defaults.

## CLI Fallback

If MCP is unavailable, use the CLI directly. The CLI and MCP are backed by the same typed application layer in `crates/spikard-cli/src/app.rs`, so behavior should match.

Read:
- [references/cli-reference.md](references/cli-reference.md) for command syntax
- [references/codegen-workflows.md](references/codegen-workflows.md) for schema-to-code workflows
- [references/project-scaffolding.md](references/project-scaffolding.md) for `spikard init`

## Working Style

- Prefer generated scaffolding over handwritten boilerplate.
- Validate schemas before broad generation when the source spec may be invalid.
- For AsyncAPI work, keep fixtures and generated test apps in sync.
- Treat bindings as thin adapters over the Rust core; do not duplicate framework logic per language.
- When touching generator behavior, run real CLI or library tests rather than only snapshotting text.

## References

- [references/mcp-tools.md](references/mcp-tools.md): MCP tool names, parameters, and defaults
- [references/cli-reference.md](references/cli-reference.md): CLI commands and examples
- [references/codegen-workflows.md](references/codegen-workflows.md): Generation flows by schema type
- [references/testing-and-fixtures.md](references/testing-and-fixtures.md): AsyncAPI fixtures, test apps, and validation
- [references/language-bindings.md](references/language-bindings.md): Binding expectations and install hints
- [references/project-scaffolding.md](references/project-scaffolding.md): Project initialization rules and outputs
