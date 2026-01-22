#!/usr/bin/env python3

"""Sync versions across the repository to match the workspace version.

Reads the version from the root Cargo.toml [workspace.package] unless an
explicit version is provided as the first CLI argument. Updates:
- package.json files in crates/ and packages/ (plus root)
- Python pyproject.toml files under crates/ and packages/
- Ruby version.rb files under crates/ and packages/
- Cargo package versions (including workspace.package and select workspace deps)
- Taskfile wheel name references
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Iterable

REPO_ROOT = Path(__file__).resolve().parent.parent

PACKAGE_JSON_PATHS = [
    Path("package.json"),
    Path("packages/node/package.json"),
    Path("packages/wasm/package.json"),
    Path("crates/spikard-node/package.json"),
    Path("crates/spikard-wasm/package.json"),
    Path("crates/spikard-wasm/dist-node/package.json"),
    Path("crates/spikard-wasm/dist-web/package.json"),
    Path("crates/spikard-wasm/dist-bundler/package.json"),
]

PYPROJECT_PATHS = [
    Path("packages/python/pyproject.toml"),
    Path("crates/spikard-py/pyproject.toml"),
]

RUBY_VERSION_PATHS = [
    Path("packages/ruby/lib/spikard/version.rb"),
    Path("crates/spikard-rb/lib/spikard/version.rb"),
]

COMPOSER_JSON_PATHS = [
    Path("composer.json"),
    Path("packages/php/composer.json"),
]

README_PATHS = [
    Path("crates/spikard/README.md"),
    Path("crates/spikard-http/README.md"),
    Path("crates/spikard-core/README.md"),
    Path("crates/spikard-bindings-shared/README.md"),
    Path("tests/test_apps/README.md"),
    Path("tests/test_apps/python/README.md"),
    Path("tests/test_apps/rust/README.md"),
    Path("tests/test_apps/wasm/README.md"),
    Path("tests/test_apps/php/README.md"),
    Path("tests/test_apps/node/README.md"),
    Path("tests/test_apps/ruby/README.md"),
    Path("packages/wasm/README.md"),
]


def log_line(message: str) -> None:
    """Write a single message to stdout (ruff-safe helper)."""
    sys.stdout.write(f"{message}\n")


def get_workspace_version(version_override: str | None) -> str:
    """Return the desired version from override or the workspace package."""
    if version_override:
        return version_override

    cargo_toml = REPO_ROOT / "Cargo.toml"
    content = cargo_toml.read_text()
    match = re.search(r'(?m)^\[workspace\.package\]\s.*?^version\s*=\s*"([^"]+)"', content, re.DOTALL)
    if not match:
        raise RuntimeError("Could not find workspace package version in Cargo.toml")
    return match.group(1)


def update_package_json(path: Path, version: str) -> bool:
    """Update the version field inside a package.json."""
    data = json.loads(path.read_text())
    old_version = data.get("version")
    if old_version == version:
        return False

    data["version"] = version
    path.write_text(json.dumps(data, indent=2) + "\n")
    return True


def update_pyproject(path: Path, version: str) -> bool:
    """Update the version field in a pyproject.toml if present."""
    content = path.read_text()
    pattern = re.compile(r'(?m)^(version\s*=\s*)(?:"[^"]+"|[^\n#]+)')

    if not pattern.search(content):
        return False

    new_content = pattern.sub(lambda m: f'{m.group(1)}"{version}"', content, count=1)
    if new_content == content:
        return False
    path.write_text(new_content)
    return True


def update_ruby_version(path: Path, version: str) -> bool:
    """Update Ruby VERSION constants."""
    content = path.read_text()
    new_content, count = re.subn(r"(VERSION\s*=\s*['\"])[^'\"]+(['\"])", rf"\g<1>{version}\g<2>", content, count=1)
    if count == 0 or new_content == content:
        return False
    path.write_text(new_content)
    return True


def update_uv_lock(_path: Path, _package_names: Iterable[str], _version: str) -> bool:
    """Deprecated: uv.lock files are intentionally not updated."""
    return False


def update_cargo_versions(path: Path, version: str) -> bool:
    """Update Cargo package versions and pinned workspace dependency versions."""
    original = path.read_text()
    content = original

    pattern_workspace = re.compile(
        r'(\[workspace\.package\][^\[]*?^version\s*=\s*")([^"]+)(")', re.MULTILINE | re.DOTALL
    )
    content, _ = pattern_workspace.subn(rf"\g<1>{version}\g<3>", content)

    pattern_package = re.compile(r'(\[package\][^\[]*?^version\s*=\s*")([^"]+)(")', re.MULTILINE | re.DOTALL)
    content, _ = pattern_package.subn(rf"\g<1>{version}\g<3>", content, count=1)

    pattern_internal_dep = re.compile(
        r'^((?:_)?spikard(?:-[a-z]+)?\s*=\s*\{[^}]*version\s*=\s*")([^"]+)(")', re.MULTILINE
    )
    content, _ = pattern_internal_dep.subn(rf"\g<1>{version}\g<3>", content)

    if content != original:
        path.write_text(content)
        return True
    return False


def update_taskfile(version: str) -> bool:
    """Update the pinned wheel filename reference in Taskfile.yaml."""
    path = REPO_ROOT / "Taskfile.yaml"
    content = path.read_text()
    new_content, count = re.subn(r"(spikard_bindings-)([0-9]+\.[0-9]+\.[0-9]+)", rf"\g<1>{version}", content, count=10)
    if count == 0 or new_content == content:
        return False
    path.write_text(new_content)
    return True


def update_readme_versions(path: Path, version: str) -> bool:
    """Update version references in README files.

    Updates:
    - Cargo.toml dependency strings: spikard = "0.x.y"
    - Test app version references: v0.x.y
    - npm package version strings: @spikard/wasm@0.x.y
    """
    if not path.exists():
        return False

    content = path.read_text()
    original = content

    # Pattern 1: Cargo.toml dependencies (spikard = "0.x.y", spikard-http = "0.x.y", etc.)
    cargo_dep_pattern = re.compile(
        r'((?:_)?spikard(?:-[a-z]+)?\s*=\s*(?:\{\s*version\s*=\s*)?")([0-9]+\.[0-9]+\.[0-9]+)(")'
    )
    content = cargo_dep_pattern.sub(rf"\g<1>{version}\g<3>", content)

    # Pattern 2: Version references like "v0.7.0" or "v0.x.y" in test documentation
    version_ref_pattern = re.compile(r'\bv([0-9]+\.[0-9]+\.[0-9]+)\b')
    content = version_ref_pattern.sub(rf"v{version}", content)

    # Pattern 3: npm package versions (@spikard/wasm@0.x.y)
    npm_package_pattern = re.compile(r'(@spikard/[a-z-]+@)([0-9]+\.[0-9]+\.[0-9]+)')
    content = npm_package_pattern.sub(rf"\g<1>{version}", content)

    # Pattern 4: Migration note references (As of v0.x.y)
    migration_pattern = re.compile(r'(As of v)([0-9]+\.[0-9]+\.[0-9]+)')
    content = migration_pattern.sub(rf"\g<1>{version}", content)

    if content != original:
        path.write_text(content)
        return True
    return False


def main() -> None:
    """Entry point."""
    version_override = sys.argv[1] if len(sys.argv) > 1 else None
    version = get_workspace_version(version_override)

    changed_files: list[Path] = []

    for rel in PACKAGE_JSON_PATHS:
        path = REPO_ROOT / rel
        if update_package_json(path, version):
            changed_files.append(rel)

    for rel in COMPOSER_JSON_PATHS:
        path = REPO_ROOT / rel
        if update_package_json(path, version):
            changed_files.append(rel)

    for rel in PYPROJECT_PATHS:
        path = REPO_ROOT / rel
        if update_pyproject(path, version):
            changed_files.append(rel)

    for rel in RUBY_VERSION_PATHS:
        path = REPO_ROOT / rel
        if update_ruby_version(path, version):
            changed_files.append(rel)

    for cargo_path in REPO_ROOT.rglob("Cargo.toml"):
        if "target" in cargo_path.parts or "node_modules" in cargo_path.parts:
            continue
        if update_cargo_versions(cargo_path, version):
            changed_files.append(cargo_path.relative_to(REPO_ROOT))

    if update_taskfile(version):
        changed_files.append(Path("Taskfile.yaml"))

    for rel in README_PATHS:
        path = REPO_ROOT / rel
        if update_readme_versions(path, version):
            changed_files.append(rel)

    if changed_files:
        log_line(f"Updated {len(changed_files)} file(s):")
        for path_str in sorted({str(p) for p in changed_files}):
            log_line(f" - {path_str}")
    else:
        log_line(f"No changes needed; all versions already set to {version}.")


if __name__ == "__main__":
    main()
