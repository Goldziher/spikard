# Spikard Init Command

Scaffold a new Spikard project with language-specific structure, dependencies, and example handlers.

## Quick Start

```bash
spikard init --name my_api --language python
cd my_api
uv sync
python -m my_api.app
```

## Installation

```bash
cargo install spikard-cli
# or
spikard --version
```

## Supported Languages

| Language | Package Manager | Init Time |
|----------|-----------------|-----------|
| Python | pip/uv | ~2s |
| TypeScript | npm/pnpm/yarn | ~3s |
| Ruby | bundler | ~2s |
| PHP | composer | ~2s |
| Rust | cargo | ~3s |

## Command

```bash
spikard init --name PROJECT_NAME --language LANGUAGE [OPTIONS]
```

### Options

- `--name` / `-n` - Project name (required)
- `--language` / `-l` - Target language: `python`, `typescript`, `ruby`, `php`, `rust` (required)
- `--project-dir` / `-d` - Directory to create project in (optional)
- `--schema` / `-s` - Path to existing API schema (optional)

## Example: Python Project

```bash
spikard init --name user_service --language python
```

Creates:
```
user_service/
├── pyproject.toml
├── src/user_service/
│   ├── app.py
│   ├── handlers.py
│   └── models.py
└── tests/
    └── test_handlers.py
```

Next steps:
```bash
cd user_service
uv sync                         # Install dependencies
python -m user_service.app      # Run the app
uv run pytest tests/            # Run tests
```

## Advanced Usage

For comprehensive documentation including:
- All language examples (TypeScript, Ruby, PHP, Rust)
- Project structure details
- Schema integration
- Customizing templates
- CI/CD integration

See the [Init Command Reference](/reference/init-advanced.md).

## Support

- [Documentation](https://docs.spikard.dev)
- [GitHub Issues](https://github.com/spikard/spikard/issues)
- [Discussions](https://github.com/spikard/spikard/discussions)
