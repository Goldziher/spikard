"""Helper utilities for loading test fixtures in tests."""

import json
from pathlib import Path
from typing import Any, Dict, List, Optional

FIXTURES_DIR = Path(__file__).parent.parent


def load_fixture(category: str, name: str) -> Dict[str, Any]:
    """Load a test fixture by category and name.

    Args:
        category: Fixture category (e.g., 'multipart', 'query_params')
        name: Fixture name without .json extension

    Returns:
        Parsed fixture data

    Raises:
        FileNotFoundError: If fixture doesn't exist
        json.JSONDecodeError: If fixture is invalid JSON

    Example:
        >>> fixture = load_fixture("multipart", "01_simple_file_upload")
        >>> print(fixture["name"])
        'Simple file upload'
    """
    path = FIXTURES_DIR / category / f"{name}.json"
    with open(path) as f:
        return json.load(f)


def load_all_fixtures(category: str) -> List[Dict[str, Any]]:
    """Load all fixtures in a category.

    Args:
        category: Fixture category

    Returns:
        List of all fixtures in the category (excluding schema.json)

    Example:
        >>> fixtures = load_all_fixtures("multipart")
        >>> len(fixtures)
        7
    """
    category_dir = FIXTURES_DIR / category
    fixtures = []

    for path in sorted(category_dir.glob("*.json")):
        if path.name == "schema.json":
            continue
        with open(path) as f:
            fixtures.append(json.load(f))

    return fixtures


def get_fixture_names(category: str) -> List[str]:
    """Get names of all fixtures in a category.

    Args:
        category: Fixture category

    Returns:
        List of fixture names (without .json extension)

    Example:
        >>> names = get_fixture_names("multipart")
        >>> names[0]
        '01_simple_file_upload'
    """
    category_dir = FIXTURES_DIR / category
    names = []

    for path in sorted(category_dir.glob("*.json")):
        if path.name == "schema.json":
            continue
        names.append(path.stem)

    return names


def load_file_content(category: str, filename: str) -> bytes:
    """Load file content referenced by fixtures.

    Args:
        category: Fixture category
        filename: Name of file in category's files/ directory

    Returns:
        Raw file contents as bytes

    Example:
        >>> content = load_file_content("multipart", "test.txt")
        >>> content
        b'<file content>'
    """
    path = FIXTURES_DIR / category / "files" / filename
    with open(path, "rb") as f:
        return f.read()


# Pytest parametrize helpers

def pytest_parametrize_fixtures(category: str, fixture_names: Optional[List[str]] = None):
    """Generate pytest.mark.parametrize arguments for fixtures.

    Args:
        category: Fixture category
        fixture_names: Specific fixture names to include, or None for all

    Returns:
        Tuple of (argnames, argvalues, ids) for pytest.mark.parametrize

    Example:
        >>> @pytest.mark.parametrize(*pytest_parametrize_fixtures("multipart"))
        >>> def test_multipart(fixture):
        >>>     assert fixture["name"]
    """
    if fixture_names is None:
        fixture_names = get_fixture_names(category)

    fixtures = [load_fixture(category, name) for name in fixture_names]
    ids = [f["name"] for f in fixtures]

    return ("fixture", fixtures, {"ids": ids})


# Example pytest usage:
"""
import pytest
from testing_data.scripts.loader import load_fixture, pytest_parametrize_fixtures

# Single fixture
def test_simple_upload(test_client):
    fixture = load_fixture("multipart", "01_simple_file_upload")
    # ... use fixture

# All fixtures in a category
@pytest.mark.parametrize(*pytest_parametrize_fixtures("multipart"))
def test_all_multipart(test_client, fixture):
    # Build request from fixture
    files = {
        f["field_name"]: (
            f.get("filename"),
            f["content"],
            f.get("content_type")
        )
        for f in fixture["request"]["files"]
    }

    response = test_client.post(fixture["request"]["path"], files=files)
    assert response.status_code == fixture["expected_response"]["status_code"]
    assert response.json() == fixture["expected_response"]["body"]
"""
