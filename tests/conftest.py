"""Pytest configuration and shared fixtures for Spikard tests."""

import json
from pathlib import Path
from typing import Any

import pytest

# Path to the testing data directory
FIXTURES_DIR = Path(__file__).parent.parent / "testing_data"


def load_fixture(category: str, name: str) -> dict[str, Any]:
    """Load a single test fixture by category and name.

    Args:
        category: The fixture category (e.g., 'query_params', 'headers')
        name: The fixture name without .json extension

    Returns:
        The loaded fixture dictionary
    """
    path = FIXTURES_DIR / category / f"{name}.json"
    with path.open() as f:
        return json.load(f)


def load_all_fixtures(category: str) -> list[tuple[str, dict[str, Any]]]:
    """Load all fixtures from a category.

    Args:
        category: The fixture category directory name

    Returns:
        List of tuples containing (fixture_id, fixture_data)
    """
    category_dir = FIXTURES_DIR / category
    fixtures = []

    for fixture_file in sorted(category_dir.glob("*.json")):
        if fixture_file.name == "schema.json":
            continue

        with fixture_file.open() as f:
            fixture = json.load(f)
            # Use the fixture name without extension as the test ID
            fixture_id = fixture_file.stem
            fixtures.append((fixture_id, fixture))

    return fixtures


def pytest_collection_modifyitems(config, items):
    """Add category marker to all tests based on their module name."""
    for item in items:
        # Extract category from test module name (e.g., test_query_params -> query_params)
        if item.module.__name__.startswith("tests.test_"):
            category = item.module.__name__.replace("tests.test_", "")
            item.add_marker(pytest.mark.category(category))


@pytest.fixture(scope="session")
def fixtures_dir() -> Path:
    """Provide the path to the fixtures directory."""
    return FIXTURES_DIR
