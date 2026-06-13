# Installation

Spikard ships a Rust core plus 15 language bindings. Install only what you need; every binding shares the same runtime behavior.

## Install by binding

=== "Python"

    ```bash
    pip install spikard
    ```

    Requires Python ≥ 3.10. Wheels ship for CPython 3.10–3.13 on Linux (amd64, arm64), macOS (amd64, arm64), and Windows.

=== "TypeScript / Node"

    ```bash
    npm install @spikard/node
    # or
    pnpm add @spikard/node
    # or
    yarn add @spikard/node
    ```

    Requires Node ≥ 18. Prebuilt binaries available for Linux, macOS, and Windows.

=== "Ruby"

    ```bash
    gem install spikard
    ```

    Requires Ruby ≥ 3.2. Precompiled gems available; source builds require Rust.

=== "PHP"

    ```bash
    composer require spikard/spikard
    ```

    Requires PHP ≥ 8.2. For CI or non-interactive Composer runs:

    ```bash
    composer config allow-plugins.spikard/spikard true
    composer require spikard/spikard
    ```

=== "Elixir"

    Add to your `mix.exs` dependencies:

    ```elixir
    {:spikard, "~> 0.16"}
    ```

    Requires Elixir ≥ 1.14, OTP ≥ 25. Precompiled NIFs available; source builds require Rust.

=== "Go"

    ```bash
    go get github.com/spikard-dev/spikard/packages/go
    ```

    Requires Go ≥ 1.21.

=== "Java"

    Add to `pom.xml`:

    ```xml
    <dependency>
        <groupId>dev.spikard</groupId>
        <artifactId>spikard</artifactId>
        <version>0.16.0</version>
    </dependency>
    ```

    Requires Java ≥ 21.

=== "C#"

    ```bash
    dotnet add package Spikard
    ```

    Requires .NET 9.0 or later (Core + Framework).

=== "Kotlin"

    Add to `build.gradle.kts`:

    ```kotlin
    implementation("dev.spikard:spikard-kt:0.16.0")
    ```

    Requires Kotlin ≥ 1.8.

=== "Dart"

    Add to `pubspec.yaml`:

    ```yaml
    dependencies:
      spikard: ^0.16.0
    ```

    Requires Dart ≥ 3.0.

=== "Swift"

    Add to `Package.swift`:

    ```swift
    .package(url: "https://github.com/spikard-dev/spikard.git", from: "0.16.0")
    ```

    Requires Swift ≥ 5.9.

=== "Zig"

    Add to `build.zig.zon`:

    ```zig
    .spikard = .{
        .url = "https://github.com/spikard-dev/spikard/archive/v0.16.0.tar.gz",
        .hash = "...",
    },
    ```

    Requires Zig 0.13 or later.

=== "C"

    Download the prebuilt C library and header from [GitHub Releases](https://github.com/spikard-dev/spikard/releases):

    ```c
    #include "spikard.h"
    // Link against libspikard.so / libspikard.dylib / spikard.dll
    ```

=== "WebAssembly"

    ```bash
    npm install @spikard/wasm
    ```

    WASM provides client-side type stubs and serialization helpers for talking to a remote Spikard server. No server-side runtime.

=== "R"

    ```r
    devtools::install_github("spikard-dev/spikard/packages/r")
    ```

    Requires R ≥ 4.1.

=== "Rust"

    ```bash
    cargo add spikard
    ```

    Requires Rust ≥ 1.80.

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
