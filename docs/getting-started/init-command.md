# Spikard Init Command

`spikard init` creates a real starter project for a supported binding with the
right package metadata, entrypoints, and starter tests already in place.

## Quick Start

```bash
spikard init my_api --lang python --dir .
cd my_api
uv sync
uv run python -m my_api.app
```

## Command Shape

```bash
spikard init <project-name> --lang <language> [--dir <parent-directory>]
```

### Options

- `project-name` - required positional project name
- `--lang` / `-l` - one of `python`, `typescript`, `rust`, `ruby`, `php`, `elixir`
- `--dir` / `-d` - parent directory to create the project inside, default `.`

There is currently no CLI `--schema` option for `init`. If you want schema-aware
automation, use the code generation commands after init, or the MCP `init_project`
tool if you are integrating through MCP.

## Supported Bindings

| Language | Tooling | Starter entrypoint |
|----------|---------|--------------------|
| Python | `uv`, `pytest`, `ruff` | `src/<package>/app.py` |
| TypeScript | `pnpm`, `tsc`, `vitest` | `src/server.ts` |
| Rust | `cargo` | `src/main.rs` |
| Ruby | `bundler`, `rspec`, `rbs` | `bin/server` |
| PHP | `composer`, `phpunit`, `phpstan` | `bin/server.php` |
| Elixir | `mix`, `ExUnit` | `run.exs` |

## Example Output

### Python

```bash
spikard init user_service --lang python --dir .
```

Creates:

```text
user_service/
├── pyproject.toml
├── README.md
├── src/user_service/
│   ├── __init__.py
│   └── app.py
└── tests/
    └── test_app.py
```

### TypeScript

```bash
spikard init user-service --lang typescript --dir .
```

Creates:

```text
user-service/
├── package.json
├── tsconfig.json
├── vitest.config.ts
├── src/
│   ├── app.ts
│   └── server.ts
└── tests/
    └── app.spec.ts
```

### Rust

```bash
spikard init user_service --lang rust --dir .
```

Creates:

```text
user_service/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── main.rs
└── tests/
    └── integration_test.rs
```

### Ruby

```bash
spikard init user_service --lang ruby --dir .
```

Creates:

```text
user_service/
├── Gemfile
├── bin/server
├── lib/user_service.rb
├── sig/user_service.rbs
└── spec/
    ├── spec_helper.rb
    └── user_service_spec.rb
```

### PHP

```bash
spikard init UserService --lang php --dir .
```

Creates:

```text
UserService/
├── composer.json
├── phpstan.neon
├── phpunit.xml
├── src/AppController.php
├── bin/server.php
└── tests/AppTest.php
```

### Elixir

```bash
spikard init user_service --lang elixir --dir .
```

Creates:

```text
user_service/
├── mix.exs
├── .formatter.exs
├── lib/user_service.ex
├── lib/user_service/router.ex
├── run.exs
└── test/
    ├── test_helper.exs
    └── user_service_test.exs
```

## Next Steps

After init, the normal flow is:

1. Install dependencies for the target binding.
2. Run the starter app once.
3. Run the starter tests.
4. Generate handlers from a schema with `spikard generate ...`.

Examples:

```bash
# Python
uv sync
uv run python -m my_api.app
uv run pytest

# TypeScript
pnpm install
pnpm exec tsc --noEmit
pnpm test

# Rust
cargo run
cargo test
```

## Related Docs

- [CLI Usage](../cli/usage.md)
- [Code Generation Guide](../guides/code-generation.md)
- [Init Command Reference](../reference/init-advanced.md)
