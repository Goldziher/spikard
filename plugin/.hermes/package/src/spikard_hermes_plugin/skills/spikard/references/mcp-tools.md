<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:1a430263e98109668e184b2939f9e4bba30a970a0ea24d023a3f125872b22654
Source-Hash: blake3:6ce2fb06ea35e1723d9c4b97abfb515db05b7e9052ba746196665eeef918c097
Schema-Version: v1
-->

# Spikard MCP Tools

Tool surface exposed by `spikard mcp`:

## `init_project`

Params:

- `name`: project name
- `language`: `python`, `typescript`, `rust`, `ruby`, `php`, or `elixir`
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

## `generate_sql`

Generate SQL-to-HTTP routing, language sidecars and an OpenAPI 3.1 spec.

Required parameters: `queries` (annotated SQL file or directory), `schema` (nonempty array of DDL paths), `lang` (nonempty array of target languages).
Optional: `dialect` (`postgresql`, `mysql`, `sqlite`, `mssql`, `oracle`, `snowflake`; default `postgresql`), `output` (default `generated`), `decimal_mode` (`string-pattern` or `number`), `strict`, `no_openapi`, `api_title`, `api_version`.
Query routing annotations use `-- @http <METHOD> <PATH>`. Prefer lossless `string-pattern` decimals. Call `list_tools` to inspect the installed server's exact input schema before submitting requests.

## `get_features`

No params. Returns the feature summary and binding hints.
