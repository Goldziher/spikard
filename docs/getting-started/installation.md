# Installation

Spikard ships a Rust core plus bindings for Python, TypeScript/Node, and Ruby. Install only what you need; every binding shares the same runtime behavior.

## Prerequisites
- Rust toolchain (for the core, CLI, and building bindings)
- Python 3.10+ for the Python package
- Node.js 18+ / pnpm for the TypeScript package
- Ruby 3.2+ for the Ruby gem
- `uv`, `cargo`, and `pnpm` are used throughout the Taskfile

## Install the Rust Core
```bash
cargo add spikard
```

## Install the CLI
```bash
cargo install spikard-cli
```
Use `spikard run` to serve an app entrypoint (see [CLI usage](../cli/usage.md)).

## Install Language Bindings

### Python
```bash
pip install spikard
```

### TypeScript / Node.js
```bash
npm install spikard
# or
pnpm add spikard
```

### Ruby
```bash
gem install spikard
```

## Local Development Setup

The repo uses `uv` to manage Python deps and pnpm for JavaScript:

```bash
# install all languages + hooks
task setup

# or only the Python/Node deps needed for docs
uv sync --group docs --group doc --no-install-workspace
pnpm install --frozen-lockfile
```

When working on docs locally, run `task docs:serve` to launch MkDocs Material with live reload.
