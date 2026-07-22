---
name: release-workflow
description: >-
  Release/publish the spikard Rust core crate and CLI end-to-end. Load when
  releasing or publishing spikard — cutting a new version, tagging, running
  `gh release create`, and installing the released build locally. Covers version
  set via Taskfile, CHANGELOG roll, the clean-tree precondition, GitHub release,
  local install, and artifact cleanup.
---

# Spikard Release Workflow

Ground truth: spikard is a Rust workspace (core library + `spikard-cli` +
polyglot bindings). It has **no `set-version` task** — the equivalent is
`task version:set` (a `version-sync` include). `Cargo.toml` is the single source
of truth; `version:set` syncs it to every binding manifest.

## 1. Set the version

```bash
task version:set -- X.Y.Z        # e.g. task version:set -- 0.7.0-rc.1
```

`version:set` takes the version as a `--` positional argument (CLI_ARGS). It runs
`alef sync-versions --set <version>` then the full `sync` (regenerate READMEs,
docs, bindings, scaffold, stubs, e2e, and precise `cargo update` for the core
crate across dependent manifests). Bump variants also exist:
`task version:bump:patch`, `task version:bump:minor`, `task version:bump:major`.
`task version:show` prints the current version.

Verify:

```bash
task version:show
grep -m1 '^version' Cargo.toml
```

## 2. Update the CHANGELOG

Move every `[Unreleased]` bullet in `CHANGELOG.md` into a new
`## [X.Y.Z] - YYYY-MM-DD` section (grouped Added / Changed / Fixed / Removed).
Re-create an empty `[Unreleased]`. Never tag an empty section.

## 3. Clean-tree precondition (hard gate)

Never release a dirty or failing tree.

```bash
poly fmt --check .    # formatting clean (task format:check)
poly lint .           # lint clean (task lint)
task test             # Rust core tests pass (rust:test)
```

Use `poly fmt --fix .` (or `task format`) to apply formatting, then re-stage.
For a full cross-binding gate before a release, run `task test:all`
(`task test` + `alef test`). Optionally validate manifests with
`task publish:validate` (`alef publish validate`). Fix any failure — do not
release past it.

## 4. Commit, tag, and publish the GitHub release

```bash
git add -A
git commit -m "chore(release): X.Y.Z"
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin main
git push origin vX.Y.Z
gh release create vX.Y.Z --title "vX.Y.Z" --generate-notes
```

Add `--prerelease` for RC/beta tags. Use `--notes-file` from the new CHANGELOG
section instead of `--generate-notes` when the changelog entry is richer. A bare
`git tag` is not a release — always run `gh release create`. Tag-based releases
trigger the multi-platform / multi-registry publish workflows.

## 5. Install the released CLI locally

```bash
cargo install --path crates/spikard-cli --force
```

Installs the `spikard` CLI binary from the workspace. (If a change is only in the
core crate, `task build:release` builds `spikard` core; `task build:cli` builds
the CLI binary to `target/release/spikard`.) Confirm `which spikard` and
`spikard --version` reflect X.Y.Z.

## 6. Clean up build artifacts

```bash
task clean        # cargo clean + alef clean + rm -rf dist/ .alef/ caches
```

`task clean` runs `cargo clean` (removing `target/` to reclaim space) plus
`alef clean` and removes `dist/`, `.alef/`, and language caches.

## Anti-patterns

- Reaching for a `set-version` task — spikard uses `task version:set -- X.Y.Z`.
- Hand-editing `version` in `Cargo.toml` or any binding manifest instead of
  `task version:set`.
- Releasing a dirty or lint/test-failing tree.
- Tagging without `gh release create`.
- AI attribution in commit/tag/release text.
