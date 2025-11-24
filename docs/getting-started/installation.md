# Installation

Spikard ships a Rust core plus bindings for Python, TypeScript/Node, and Ruby. Install only what you need; every binding shares the same runtime behavior.

## Install by binding

=== "Python"

    ```bash
    pip install spikard
    ```

=== "TypeScript / Node"

    ```bash
    npm install spikard
    # or
    pnpm add spikard
    ```

=== "Ruby"

    ```bash
    gem install spikard
    ```

=== "Rust"

    ```bash
    cargo add spikard
    ```

## CLI

Install the CLI for code generation and schema validation:

```bash
cargo install spikard-cli
```

## Local repo setup

The repo uses `uv` to manage Python deps and pnpm for JavaScript:

```bash
# install all languages + hooks
task setup

# or only the Python/Node deps needed for docs
uv sync --group docs --group doc --no-install-workspace
pnpm install --frozen-lockfile
```

When working on docs locally, run `task docs:serve` to launch MkDocs Material with live reload.
