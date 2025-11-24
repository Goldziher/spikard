# Contributing

Thanks for helping shape Spikard! The fastest way to get started is to follow the existing Taskfile and language-specific READMEs.

## Setup
```bash
task setup
```
This installs Rust, Python, Node, and Ruby dependencies plus pre-commit hooks.

## Common Tasks
- `task build` – build Rust core and bindings
- `task test` – run all language test suites
- `task lint` / `task format` – apply linters and formatters
- `task docs:serve` – work on this documentation locally

## Guidelines
- Keep behavior consistent across bindings; add fixture-driven tests when adding new features.
- Prefer contract-first changes: update schemas, generators, and ADRs where relevant.
- Run `cargo fmt`, `cargo clippy`, `ruff`, `biome`, and language-specific tools before opening a PR.

For more detail, see the root `CONTRIBUTING.md` file and ADRs that cover design intent.
