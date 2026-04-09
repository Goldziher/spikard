# Spikard MCP Tools

Tool surface exposed by `spikard mcp`:

## `init_project`

Params:

- `name`: project name
- `language`: `python`, `typescript`, `rust`, `ruby`, or `php`
- `directory`: optional base directory, default `.` before appending `name`
- `schema_path`: optional schema path to seed the project

Returns created files and next steps.

## `generate_openapi`

Params:

- `schema`
- `language`
- `output`: optional
- `dto`: optional

Supported DTO values:

- Python: `dataclass`, `msgspec`
- TypeScript: `zod`
- Ruby: `dry_schema`
- Rust: `serde`
- PHP: `readonly_class`

## `generate_asyncapi_handlers`

Params:

- `schema`
- `language`
- `output`
- `dto`: optional, same language-specific rules as OpenAPI

## `generate_jsonrpc`

Params:

- `schema`
- `language`
- `output`: optional

If `output` is omitted, the current implementation uses the CLI default path behavior rather than printing a structured in-memory file list.

## `generate_graphql`

Params:

- `schema`
- `language`
- `output`: optional
- `target`: optional, default `all`

## `generate_protobuf`

Params:

- `schema`
- `language`
- `output`
- `target`: optional, default `all`
- `include`: optional list of import directories

## `generate_php_dto`

Params:

- `output`: optional, default `src/Generated`

## `generate_asyncapi_fixtures`

Params:

- `schema`
- `output`: optional, default `testing_data`

## `generate_asyncapi_test_app`

Params:

- `schema`
- `language`
- `output`

## `generate_asyncapi_bundle`

Params:

- `schema`
- `output`: optional, default `.`

## `validate_asyncapi`

Params:

- `schema`

Returns a structured summary with title, version, primary protocol, and channel count.

## `get_features`

No params. Returns the feature summary and binding hints.
