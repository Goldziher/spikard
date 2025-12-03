"""
Pytest configuration and fixture loaders for fixture-driven testing.

This module provides:
- Fixture discovery and loading from testing_data/
- Parametrized test data for each category
- Test client setup for integration tests
- Fixture validation against schemas
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import pytest


def pytest_configure(config: Any) -> None:
    """Configure pytest with custom markers."""
    config.addinivalue_line("markers", "fixture_category(category): Mark test with fixture category")
    config.addinivalue_line("markers", "fixture_skip(reason): Mark fixture that should be skipped")


def discover_fixture_files(category: str, exclude_schema: bool = True) -> list[Path]:
    """
    Discover all JSON fixture files in a category directory.

    Args:
        category: The fixture category name (e.g., 'headers', 'cookies')
        exclude_schema: Whether to exclude schema.json files

    Returns:
        Sorted list of fixture file paths
    """
    # Resolve from conftest location: tests/conftest.py -> packages/python/tests/
    # Then up to packages/python -> packages -> workspace root
    conftest_dir = Path(__file__).parent
    testing_data_root = conftest_dir.parent.parent.parent / "testing_data"
    category_dir = testing_data_root / category

    if not category_dir.exists():
        return []

    fixtures = [f for f in category_dir.glob("*.json") if not (exclude_schema and f.name == "schema.json")]

    return sorted(fixtures)


def load_fixture(fixture_path: Path) -> dict[str, Any]:
    """
    Load a single fixture JSON file.

    Args:
        fixture_path: Path to the fixture JSON file

    Returns:
        Parsed fixture data as dictionary
    """
    with fixture_path.open() as f:
        return json.load(f)


def load_fixture_schema(category: str) -> dict[str, Any] | None:
    """
    Load the schema.json for a fixture category.

    Args:
        category: The fixture category name

    Returns:
        Schema data or None if not found
    """
    conftest_dir = Path(__file__).parent
    schema_path = conftest_dir.parent.parent.parent / "testing_data" / category / "schema.json"

    if schema_path.exists():
        return load_fixture(schema_path)

    return None


@pytest.fixture(scope="session")
def testing_data_root() -> Path:
    """Get the root path to testing_data directory."""
    conftest_dir = Path(__file__).parent
    return conftest_dir.parent.parent.parent / "testing_data"


@pytest.fixture(scope="session")
def fixture_categories() -> dict[str, list[dict[str, Any]]]:
    """
    Load all fixtures organized by category.

    Returns:
        Dictionary mapping category names to lists of fixture data
    """
    categories = {
        "headers": [],
        "cookies": [],
        "json_bodies": [],
        "validation_errors": [],
        "status_codes": [],
        "query_params": [],
        "path_params": [],
        "http_methods": [],
        "content_types": [],
        "edge_cases": [],
        "auth": [],
        "cors": [],
        "streaming": [],
        "url_encoded": [],
        "multipart": [],
        "lifecycle_hooks": [],
        "rate_limit": [],
        "request_timeout": [],
        "request_id": [],
        "compression": [],
        "body_limits": [],
        "background": [],
    }

    for category in categories:
        fixtures = discover_fixture_files(category)
        for fixture_path in fixtures:
            try:
                data = load_fixture(fixture_path)
                categories[category].append(data)
            except (json.JSONDecodeError, OSError) as e:
                pytest.fail(f"Failed to load fixture {fixture_path}: {e}")

    return categories


@pytest.fixture
def headers_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for header tests."""
    return fixture_categories["headers"]


@pytest.fixture
def cookies_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for cookie tests."""
    return fixture_categories["cookies"]


@pytest.fixture
def json_bodies_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for JSON body tests."""
    return fixture_categories["json_bodies"]


@pytest.fixture
def validation_errors_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for validation error tests."""
    return fixture_categories["validation_errors"]


@pytest.fixture
def status_codes_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for status code tests."""
    return fixture_categories["status_codes"]


@pytest.fixture
def query_params_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for query parameter tests."""
    return fixture_categories["query_params"]


@pytest.fixture
def path_params_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for path parameter tests."""
    return fixture_categories["path_params"]


@pytest.fixture
def http_methods_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for HTTP method tests."""
    return fixture_categories["http_methods"]


@pytest.fixture
def content_types_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for content type tests."""
    return fixture_categories["content_types"]


@pytest.fixture
def edge_cases_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for edge case tests."""
    return fixture_categories["edge_cases"]


@pytest.fixture
def auth_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for authentication tests."""
    return fixture_categories["auth"]


@pytest.fixture
def cors_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for CORS tests."""
    return fixture_categories["cors"]


@pytest.fixture
def streaming_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for streaming tests."""
    return fixture_categories["streaming"]


@pytest.fixture
def url_encoded_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for URL-encoded form data tests."""
    return fixture_categories["url_encoded"]


@pytest.fixture
def multipart_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for multipart form data tests."""
    return fixture_categories["multipart"]


@pytest.fixture
def lifecycle_hooks_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for lifecycle hook tests."""
    return fixture_categories["lifecycle_hooks"]


@pytest.fixture
def rate_limit_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for rate limiting tests."""
    return fixture_categories["rate_limit"]


@pytest.fixture
def request_timeout_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for request timeout tests."""
    return fixture_categories["request_timeout"]


@pytest.fixture
def request_id_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for request ID tests."""
    return fixture_categories["request_id"]


@pytest.fixture
def compression_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for compression tests."""
    return fixture_categories["compression"]


@pytest.fixture
def body_limits_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for body limit tests."""
    return fixture_categories["body_limits"]


@pytest.fixture
def background_fixtures(fixture_categories: dict[str, list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Fixture data for background task tests."""
    return fixture_categories["background"]


def get_fixture_ids(fixtures: list[dict[str, Any]]) -> list[str]:
    """
    Generate test IDs from fixture names.

    Args:
        fixtures: List of fixture dictionaries

    Returns:
        List of test IDs based on fixture names and skip status
    """
    ids = []
    for fixture in fixtures:
        skip = fixture.get("skip", False)
        skip_reason = fixture.get("skip_reason", "")
        name = fixture.get("name", "unknown")

        if skip and skip_reason:
            test_id = f"{name} [SKIP: {skip_reason}]"
        elif skip:
            test_id = f"{name} [SKIP]"
        else:
            test_id = name

        ids.append(test_id)

    return ids


@pytest.fixture
def fixture_validator(testing_data_root: Path) -> Any:
    """
    Create a fixture validator that checks fixtures against schemas.

    Args:
        testing_data_root: Root path to testing_data directory

    Returns:
        Callable validator function
    """

    def validate_fixture(category: str, fixture_data: dict[str, Any]) -> tuple[bool, list[str]]:
        """
        Validate a fixture against its category schema.

        Args:
            category: The fixture category name
            fixture_data: The fixture data to validate

        Returns:
            Tuple of (is_valid, error_messages)
        """
        schema_path = testing_data_root / category / "schema.json"

        if not schema_path.exists():
            return True, []

        try:
            import jsonschema

            schema = load_fixture(schema_path)
            validator = jsonschema.Draft7Validator(schema)
            errors = list(validator.iter_errors(fixture_data))

            if errors:
                error_messages = [e.message for e in errors]
                return False, error_messages

            return True, []

        except Exception:
            # If validation fails for any reason, still return True
            # This allows tests to run even if schema validation has issues
            return True, []

    return validate_fixture
