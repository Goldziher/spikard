# SQL → HTTP Handler Codegen

Spikard can turn an annotated SQL schema into a full HTTP layer: typed route metadata, an OpenAPI 3.1 spec, and a per-language sidecar that drives handler-stub generation across every binding.

The feature builds on [scythe](https://github.com/kreuzberg-dev/scythe) — the SQL-to-typed-query compiler used elsewhere in the Kreuzberg ecosystem. Scythe parses your SQL into an analyzed-query IR; Spikard reads scythe's IR and overlays its own HTTP vocabulary on top. The split is deliberate: scythe stays library-agnostic (no HTTP concepts), and Spikard owns the entire HTTP grammar.

## When to use this

You already write SQL queries against a database and want HTTP endpoints that expose them. Annotating each query with one extra line of metadata (`@http GET /users/{id}`) is faster and less drift-prone than maintaining a separate OpenAPI document. The resulting OpenAPI spec is a free byproduct of the generation and feeds straight into Spikard's existing per-language handler-stub generators.

## Quick start

Create a schema DDL and at least one annotated query file. Then run:

```bash
spikard generate sql queries.sql \
    --schema schema.sql \
    --output ./generated \
    --lang python --lang typescript --lang rust
```

This writes three files to `./generated/`:

| File               | Purpose                                                            |
| ------------------ | ------------------------------------------------------------------ |
| `handlers.json`    | One `RouteMetadata` entry per HTTP-annotated query                 |
| `openapi.json`     | Vanilla OpenAPI 3.1 document (no `x-*` extensions)                 |
| `spikard-sql.json` | Per-language sidecar: function names, argument sources, lang types |

Pass `--no-openapi` to skip the OpenAPI emission if you don't need it.

## Annotation reference

All HTTP annotations are recognised by Spikard, not by scythe — scythe captures them verbatim and passes them through. Pick distinctive prefixes (`http_*`) to avoid future collisions.

### `@http <METHOD> <PATH>`

Defines the route. **Required** for a query to become a handler; queries without `@http` are silently skipped. Method is `GET`/`POST`/`PUT`/`PATCH`/`DELETE`/`HEAD`/`OPTIONS`. Path uses `{name}` placeholders (`:name` is accepted and normalised on the fly).

```sql
-- @http GET /users/{id}
-- @http POST /users
-- @http DELETE /users/{id}/avatar
```

### `@http_auth <scheme>`

Security requirement. One of:

- `none` — explicitly skip auth on this route
- `bearer` — bearer-token HTTP auth (no format hint)
- `bearer:<format>` — e.g. `bearer:jwt`, `bearer:opaque`
- `api_key:<location>:<name>` — `header`/`query`/`cookie` + the key name

```sql
-- @http_auth bearer:jwt
-- @http_auth api_key:header:X-API-Key
```

The scheme is registered once at the OpenAPI spec level and referenced by every operation that uses it.

### `@http_param <name> <binding>`

Override where a SQL parameter is sourced from. Bindings: `path`, `query`, `body`, `header`. Without overrides, Spikard infers: path placeholder match → `path`; GET/DELETE → `query`; POST/PUT/PATCH → `body`.

```sql
-- @http_param id path
-- @http_param email body
-- @http_param limit query
```

### `@http_status <code,code,...>`

Status codes to document in the OpenAPI spec. The first code in the list is treated as the primary response and gets the response body schema attached. If omitted, Spikard picks a default based on the SQL command (`:one`/`:many` → 200, `:exec` → 204, `:exec_rows` → 200 with `{rows: int}` body).

```sql
-- @http_status 200,404
-- @http_status 201
```

### `@http_request_body <name>`

Name for the bundled body object when there are multiple body parameters. Defaults to `payload`. A single body parameter is unwrapped — its name becomes the body's `body_param_name` on `RouteMetadata`.

### `@http_tags <tag,tag,...>`

Comma-separated tags for OpenAPI grouping.

### `@http_summary <text>` / `@http_description <text>`

Short and long descriptions for the operation. Used in the OpenAPI `summary` and `description` fields.

## Command compatibility

Not every scythe `:returns` command maps to HTTP. Spikard enforces the table below at codegen time:

| scythe command | Allowed HTTP method                 | Default status | Response body               |
| -------------- | ----------------------------------- | -------------- | --------------------------- |
| `:one`         | `GET`                               | 200            | Row object                  |
| `:opt`         | `GET`                               | 200            | Row object or `null`        |
| `:many`        | `GET`                               | 200            | Array of row objects        |
| `:grouped`     | `GET`                               | 200            | Array of nested row objects |
| `:exec`        | `POST` / `PUT` / `PATCH` / `DELETE` | 204            | None                        |
| `:exec_rows`   | `POST` / `PUT` / `PATCH` / `DELETE` | 200            | `{"rows": <int>}`           |
| `:exec_result` | — (rejected)                        | —              | —                           |
| `:batch`       | — (rejected)                        | —              | —                           |

A mismatch (`:one` with `POST`, or `:exec` with `GET`) raises `AnnotationParseError::MethodCommandMismatch` with the source line.

## Worked example

A minimal users table with three handlers (GET by id, POST, GET list with filter):

```sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    name TEXT,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

```sql
-- @name GetUser
-- @returns :one
-- @http GET /users/{id}
-- @http_auth bearer:jwt
-- @http_status 200,404
-- @http_summary Fetch one user by id
-- @http_tags users
SELECT id, email, name, active, created_at
FROM users
WHERE id = $1;

-- @name CreateUser
-- @returns :exec_rows
-- @http POST /users
-- @http_auth bearer:jwt
-- @http_status 201
-- @http_summary Create a new user
-- @http_tags users
-- @http_param email body
-- @http_param name body
INSERT INTO users (email, name) VALUES ($1, $2);

-- @name ListUsersByStatus
-- @returns :many
-- @http GET /users
-- @http_auth bearer:jwt
-- @http_summary List users filtered by active status
-- @http_tags users
-- @http_param active query
SELECT id, email, name, active, created_at
FROM users
WHERE active = $1;
```

Run the CLI:

```bash
spikard generate sql users.sql --schema schema.sql -o ./out --lang python
```

The output of this exact fixture lives at `testing_data/sql_handlers/` for inspection — `expected_handlers.json`, `expected_openapi.json`, and `expected_sidecar.json`.

## What the sidecar carries

`spikard-sql.json` is keyed by language, then by `operation_id`. Each entry tells Spikard's per-language handler-stub generator how to call into the scythe-generated query function:

```json
{
  "by_language": {
    "python": {
      "GetUser": {
        "scythe_fn": "get_user",
        "scythe_module": "queries",
        "params": [{ "name": "id", "lang_type": "int", "source": "path" }],
        "return_lang_type": "int, str, str | None, bool, datetime",
        "is_async": true,
        "command": "One"
      }
    }
  }
}
```

The OpenAPI spec stays vanilla so it works with any third-party tooling (Swagger UI, openapi-typescript, etc.); language-specific details only cross via the sidecar.

## Type mapping

Scythe normalises SQL types to a language-agnostic "neutral type" string (`int32`, `string`, `uuid`, `datetime_tz`, etc.). Spikard maps each neutral type to a JSON Schema fragment. The full mapping table — including the `array<T>`, `range<T>`, `enum::<name>`, and `composite::<name>` recursive forms — lives in `crates/spikard-codegen/src/sql/neutral_to_json_schema.rs` and is exercised by one unit test per type.

Nullability uses the OpenAPI 3.1 idiom: `{"oneOf": [<schema>, {"type": "null"}]}`. Optional parameters (`@optional name1, name2` in scythe) stay in `properties` but are omitted from `required`.

The `decimal` type has no native exact representation in JSON Schema. Spikard exposes two modes via `--decimal-mode`:

- `string-pattern` (default): `{"type": "string", "pattern": "^-?\\d+(\\.\\d+)?$"}` — lossless.
- `number`: `{"type": "number"}` — lossy, but ergonomic.

## Strict mode

By default, Spikard falls back to `{}` (any JSON) when scythe emits a neutral type it doesn't recognise. Pass `--strict` to make unknown neutral types a hard error instead.

## Generated route metadata

Each entry in `handlers.json` is a Spikard `RouteMetadata` (the same type spikard-core consumes for OpenAPI-driven handlers). Fields:

- `method` — uppercase HTTP method.
- `path` — canonical `{name}` form.
- `handler_name` — `handle_<snake>` of the scythe `@name`. Deliberately distinct from scythe's own query-function name to prevent collisions.
- `parameter_schema` — combined path + query JSON Schema.
- `request_schema` — body object schema (or `null`).
- `response_schema` — row / array / `{rows: int}` / `null` per the command table.
- `body_param_name` — set when a single body parameter exists so the generator preserves the unwrapped UX.
- `is_async` — always `true`.
- `expects_json_body` — `true` for POST/PUT/PATCH when body params exist.

These slot straight into Spikard's runtime alongside any OpenAPI-derived routes.

## How it relates to other codegen targets

`spikard generate sql` is a peer of `spikard generate openapi`, `spikard generate graphql`, etc. — it produces the same `RouteMetadata` shape and feeds the same per-language handler-stub generators. If you already have an OpenAPI document for parts of your API, the two outputs can coexist: emit the SQL portion via `generate sql`, the rest via `generate openapi`, and merge.

## Further reading

- [Code Generation guide](code-generation.md) — common architecture across all codegen targets.
- [scythe documentation](https://github.com/kreuzberg-dev/scythe) — annotation reference for native scythe annotations (`@name`, `@returns`, `@param`, `@nullable`, etc.) and the type system.
- `testing_data/sql_handlers/` in the Spikard repo — minimal fixture with reference outputs.
