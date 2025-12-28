"""Smoke-test the DI guide snippet to ensure it stays runnable."""

from __future__ import annotations

import re
from pathlib import Path

import pytest


def _extract_python_snippet(path: Path) -> str:
    content = path.read_text(encoding="utf-8")
    match = re.search(r"```python\n(.*?)```", content, re.DOTALL)
    if not match:
        raise AssertionError("DI snippet not found in dependency_injection.md")
    return match.group(1).strip()


def test_di_snippet_builds_app() -> None:
    pytest.importorskip("_spikard")

    snippet_path = Path(__file__).parents[3] / "docs" / "snippets" / "python" / "dependency_injection.md"
    code = _extract_python_snippet(snippet_path)

    ns: dict[str, object] = {}
    exec(code, ns, ns)

    app = ns.get("app")
    assert app is not None, "snippet should define an 'app' variable"
    dependencies = getattr(app, "_dependencies", {})
    assert "db_pool" in dependencies, "db_pool should be registered by the snippet"
    assert "config" in dependencies, "config should be registered by the snippet"
