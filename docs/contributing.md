# Contributing

Thanks for helping shape Spikard! The fastest way to get started is to follow the mise tasks (`mise tasks ls`) and language-specific READMEs.

## Where to start

Spikard is experimental and pre-1.0, so contributions land quickly and genuinely shape the design.

- **[Good first issues](https://github.com/Goldziher/spikard/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)** — scoped to be finishable in an evening, mostly self-contained documentation snippets for a single binding.
- **[Help wanted](https://github.com/Goldziher/spikard/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)** — larger subsystems from the [roadmap](roadmap.md), each backed by an ADR.
- **Found something wrong?** Opening an issue is a contribution. Rough edges, confusing errors, and docs that do not match reality are all worth reporting.

Before opening a PR, run `mise run check` (formatting, linting, and docs consistency).

## Setup

```bash
mise run setup
```

This installs Rust, Python, Node, and Ruby dependencies plus pre-commit hooks.

## Common Tasks

- `mise run build` – build Rust core and bindings
- `mise run test` – run all language test suites
- `mise run lint` / `mise run format` – apply linters and formatters
- `mise run docs:serve` – work on this documentation locally

## Guidelines

- Keep behavior consistent across bindings; add fixture-driven tests when adding new features.
- Prefer contract-first changes: update schemas, generators, and ADRs where relevant.
- Run `cargo fmt`, `cargo clippy`, `ruff`, `biome`, and language-specific tools before opening a PR.

For more detail, see the root `CONTRIBUTING.md` file and ADRs that cover design intent.
