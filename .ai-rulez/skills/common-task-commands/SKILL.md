---
type: Playbook
title: Common Task Commands
description: Common Task Commands
x-ai-rulez:
  kind: skill
  id: common-task-commands
  metadata:
    priority: critical
---

# Common Task Commands

**Setup & Installation**:

- `mise run setup`: Install all dependencies (Rust, Python, Node, Go, Java, Ruby, etc.)
- `poly hooks install`: Install poly's git hook shims

**Build Commands**:

- `mise run build`: Build core only (respects BUILD_PROFILE)
- `mise run build:bindings`: Build language bindings explicitly
- `mise run build:all`: Build core plus bindings explicitly
- `mise run rust:build`: Build Rust core (respects BUILD_PROFILE)
- `mise run rust:build:dev`: Build Rust in debug mode
- `mise run rust:build:release`: Build Rust in release mode
- `mise run python:build`: Build Python bindings (maturin)
- `mise run node:build`: Build TypeScript/Node bindings (NAPI-RS)
- `mise run go:build`: Build Go bindings
- `mise run java:build`: Build Java bindings (Maven)
- `mise run ruby:build`: Compile Ruby native extensions
- `mise run csharp:build`: Build C# bindings
- `mise run wasm:build`: Build WebAssembly bindings

**Test Commands**:

- `mise run test`: Run core tests (respects BUILD_PROFILE)
- `mise run rust:test`: Run Rust tests
- `mise run python:test`: Run Python tests (pytest)
- `mise run node:test`: Run TypeScript tests (vitest)
- `mise run go:test`: Run Go tests
- `mise run java:test`: Run Java tests (Maven)
- `mise run ruby:test`: Run Ruby tests (RSpec)
- `mise run e2e:generate`: Generate E2E tests from fixtures
- `mise run e2e:build`: Build bindings and generated E2E support binaries
- `mise run e2e:test`: Run E2E tests
- `mise run e2e:all`: Run all E2E tests across all languages

**Linting & Formatting**:

- `mise run lint`: Lint current project
- `mise run lint:all`: Lint all languages
- `mise run rust:lint:check`: CI Rust linting (cargo fmt --check + clippy, fails on issues)
- `mise run format`: Format repo code (auto-fixes); excludes Alef formatting
- `mise run format:check`: Check formatting (fails if needs formatting)
- `mise run alef:format`: Format Alef-generated output explicitly
- `mise run rust:fmt`: Format Rust (cargo fmt)
- `mise run rust:clippy`: Lint Rust (cargo clippy)
- `mise run python:lint`: Lint Python (ruff, mypy)
- `mise run python:format`: Format Python (ruff, black)
- `mise run node:lint`: Lint TypeScript (biome)
- `mise run node:format`: Format TypeScript (biome)

**Utilities**:

- `mise run clean`: Clean build artifacts
- `mise run version:sync`: Sync version from Cargo.toml to all manifests (package.json, pyproject.toml, pom.xml, go.mod, Gemfile, etc.)
- `poly hooks run`: Run poly's git hooks manually
- `mise run pdfium:install`: Download and install PDFium library

**Alef Generation**:

- `mise run alef:generate`: Run `alef all --clean --format=false`; does not build bindings or format generated output
- Do not add legacy aliases for build, Alef, or e2e workflows
