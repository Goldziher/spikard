# Init Command Reference

Detailed reference for `spikard init`.

## Synopsis

```bash
spikard init <project-name> --lang <language> [--dir <parent-directory>]
```

## Arguments

### `project-name`

Required positional project name. The scaffolder normalizes names to match the
target binding's conventions:

- Python: snake_case package/module names
- TypeScript: kebab-case package names, `src/` layout
- Rust: snake_case crate names
- Ruby: snake_case gem/module file names
- PHP: PascalCase project names with PSR-4-friendly structure
- Elixir: snake_case OTP app names with generated module casing

### `--lang`, `-l`

Required target language:

- `python`
- `typescript`
- `rust`
- `ruby`
- `php`
- `elixir`

### `--dir`, `-d`

Optional parent directory. The CLI creates `<dir>/<project-name>`.

Examples:

```bash
spikard init billing_api --lang rust --dir .
spikard init user-service --lang typescript --dir ./services
```

## What `init` Creates

`spikard init` is meant to produce a runnable starter project, not just an empty
directory tree. Every scaffold includes:

- package/project manifest
- `.gitignore`
- `README.md`
- runnable entrypoint
- starter test

### Python

```text
<project>/
├── pyproject.toml
├── README.md
├── src/<package>/
│   ├── __init__.py
│   └── app.py
└── tests/test_app.py
```

Notes:

- uses `pyproject.toml`
- assumes `uv`
- starter app is import-safe and runnable with `uv run python -m <package>.app`

### TypeScript

```text
<project>/
├── package.json
├── tsconfig.json
├── vitest.config.ts
├── src/
│   ├── app.ts
│   └── server.ts
└── tests/app.spec.ts
```

Notes:

- uses `src/`
- separates app construction from process startup
- scaffold is ready for `pnpm install` and `pnpm exec tsc --noEmit`

### Rust

```text
<project>/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── main.rs
└── tests/integration_test.rs
```

Notes:

- no fake `Cargo.lock`
- current edition/rust-version pairing is valid

### Ruby

```text
<project>/
├── Gemfile
├── bin/server
├── lib/<project>.rb
├── sig/<project>.rbs
└── spec/
    ├── spec_helper.rb
    └── <project>_spec.rb
```

Notes:

- no forced `.ruby-version`
- starter server uses `bin/server`

### PHP

```text
<project>/
├── composer.json
├── phpstan.neon
├── phpunit.xml
├── src/AppController.php
├── bin/server.php
└── tests/AppTest.php
```

Notes:

- uses Composer-first structure
- includes PHPStan and PHPUnit wiring

### Elixir

```text
<project>/
├── mix.exs
├── .formatter.exs
├── lib/<app>.ex
├── lib/<app>/router.ex
├── run.exs
└── test/
    ├── test_helper.exs
    └── <app>_test.exs
```

Notes:

- formatter-clean output
- starter app is runnable with `mix run run.exs`

## Name Handling

The CLI accepts a user-facing name and normalizes it for the target binding.
Examples:

| Input | Python | TypeScript | Rust | Ruby | PHP | Elixir |
|------|--------|------------|------|------|-----|--------|
| `UserService` | `user_service` | `user-service` | `user_service` | `user_service` | `UserService` | `user_service` |
| `user-service` | `user_service` | `user-service` | `user_service` | `user_service` | `UserService` | `user_service` |

## Current Limitation

The CLI `init` command does not currently accept a schema path. If you need
schema-aware automation:

1. run `spikard init`
2. run `spikard generate ...` for the desired protocol

If you are integrating through MCP, the `init_project` tool can also accept a
`schema_path` field on the MCP side.

## Typical Flows

### Python

```bash
spikard init my_api --lang python --dir .
cd my_api
uv sync
uv run python -m my_api.app
uv run pytest
```

### TypeScript

```bash
spikard init my-api --lang typescript --dir .
cd my-api
pnpm install
pnpm exec tsc --noEmit
pnpm test
```

### Rust

```bash
spikard init my_api --lang rust --dir .
cd my_api
cargo run
cargo test
```

## Troubleshooting

### Directory already exists

Pick a new name or remove the existing directory first:

```bash
spikard init my_api --lang python --dir .
```

### Wrong command shape

The current CLI uses positional project names. These are invalid old forms:

```bash
spikard init --name my_api --language python
spikard init --name my_api --language python --schema openapi.yaml
```

Use:

```bash
spikard init my_api --lang python --dir .
```

## Related Docs

- [Getting Started: Init](../getting-started/init-command.md)
- [CLI Usage](../cli/usage.md)
- [Code Generation Guide](../guides/code-generation.md)
