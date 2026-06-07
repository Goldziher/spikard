"""Smoke test: import the published package."""

import importlib


def test_imports_published_package():
    module = importlib.import_module("")
    assert module is not None
