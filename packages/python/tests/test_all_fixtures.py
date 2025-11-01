"""Universal integration tests for all fixture categories.

This module runs all 238 fixtures as real integration tests using TestClient.
Each test makes actual HTTP requests and validates responses against expected results.

Categories tested:
- query_params (40 fixtures)
- headers (20 fixtures)
- json_bodies (25 fixtures)
- cookies (15 fixtures)
- path_params (20 fixtures)
- status_codes (15 fixtures)
- content_types (12 fixtures)
- http_methods (15 fixtures)
- cors (10 fixtures)
- multipart (15 fixtures)
- url_encoded (15 fixtures)
- validation_errors (20 fixtures)
- edge_cases (16 fixtures)
"""

from collections.abc import Callable
from typing import Any, cast

import pytest

from spikard import Spikard
from spikard.testing import TestClient
from tests.conftest import FIXTURES_DIR, load_all_fixtures
from tests.fixture_app import (
    cookies_app,
    headers_app,
    json_bodies_app,
    path_params_app,
    query_params_app,
    status_codes_app,
    validation_errors_app,
)

AppFactory = Callable[[], Spikard]
QUERY_PARAMS_APP_FACTORY: AppFactory = query_params_app
HEADERS_APP_FACTORY: AppFactory = headers_app
JSON_BODIES_APP_FACTORY: AppFactory = json_bodies_app
COOKIES_APP_FACTORY: AppFactory = cookies_app
PATH_PARAMS_APP_FACTORY: AppFactory = path_params_app
STATUS_CODES_APP_FACTORY: AppFactory = status_codes_app
VALIDATION_ERRORS_APP_FACTORY: AppFactory = validation_errors_app


# Discover all fixture categories dynamically
def get_all_categories() -> list[str]:
    """Get all fixture categories from the testing_data directory."""
    categories: list[str] = []
    for path in FIXTURES_DIR.iterdir():
        if path.is_dir() and not path.name.startswith("."):
            # Check if directory has fixture files
            fixture_files = list(path.glob("*.json"))
            if fixture_files and not all(f.name == "schema.json" for f in fixture_files):
                categories.append(path.name)
    return sorted(categories)


# Load all fixtures from all categories
ALL_CATEGORIES: list[str] = get_all_categories()
ALL_FIXTURES: list[tuple[str, str, dict[str, Any]]] = []

for category in ALL_CATEGORIES:
    fixtures = load_all_fixtures(category)
    for fixture_id, fixture in fixtures:
        ALL_FIXTURES.append((category, fixture_id, fixture))


# Map categories to their app factories
APP_FACTORIES: dict[str, AppFactory] = {
    "query_params": QUERY_PARAMS_APP_FACTORY,
    "headers": HEADERS_APP_FACTORY,
    "json_bodies": JSON_BODIES_APP_FACTORY,
    "cookies": COOKIES_APP_FACTORY,
    "path_params": PATH_PARAMS_APP_FACTORY,
    "status_codes": STATUS_CODES_APP_FACTORY,
    "validation_errors": VALIDATION_ERRORS_APP_FACTORY,
    "url_encoded": JSON_BODIES_APP_FACTORY,  # Similar to json_bodies
    "multipart": JSON_BODIES_APP_FACTORY,  # File uploads use similar routes
    "http_methods": JSON_BODIES_APP_FACTORY,  # Tests different methods on same routes
    "edge_cases": QUERY_PARAMS_APP_FACTORY,  # Edge cases mostly around query params
}


@pytest.mark.parametrize(
    "category,fixture_id,fixture", ALL_FIXTURES, ids=[f"{cat}::{fid}" for cat, fid, _ in ALL_FIXTURES]
)
@pytest.mark.asyncio
async def test_fixture(category: str, fixture_id: str, fixture: dict[str, Any]) -> None:
    """Universal test for all fixtures across all categories.

    This test is parameterized to run once for each fixture in all categories.
    Each fixture defines:
    - request: The HTTP request details (method, path, params, body, headers, etc.)
    - expected_response: The expected response (status code, body, headers)

    Args:
        category: The fixture category (e.g., 'query_params', 'headers')
        fixture_id: The fixture file name (for test identification)
        fixture: The fixture data dictionary
    """
    # Get app factory for this category
    app_factory = APP_FACTORIES.get(category, QUERY_PARAMS_APP_FACTORY)
    app = app_factory()

    # Create test client
    client = TestClient(app)

    # Extract fixture data
    fixture["name"]
    fixture["description"]
    request_spec = cast("dict[str, Any]", fixture["request"])
    expected_response = cast("dict[str, Any]", fixture["expected_response"])

    # Extract request components
    method = cast("str", request_spec["method"])
    path = cast("str", request_spec["path"])
    query_params = cast("dict[str, Any]", request_spec.get("query_params", {}))
    headers = cast("dict[str, str]", request_spec.get("headers", {}))
    body = request_spec.get("body")
    cookies = cast("dict[str, Any]", request_spec.get("cookies", {}))

    if query_params:
        pass
    if headers:
        pass
    if body:
        pass
    if cookies:
        pass

    # Extract expected response
    expected_status = cast("int", expected_response["status_code"])
    expected_body = expected_response.get("body")

    # Handle validation_errors field - transform to body format
    if expected_body is None and "validation_errors" in expected_response:
        expected_body = {"detail": expected_response["validation_errors"]}

    expected_headers = cast("dict[str, str]", expected_response.get("headers", {}))

    # Make actual HTTP request using test client
    try:
        if method == "GET":
            response = await client.get(
                path,
                query_params=query_params if query_params else None,
                headers=headers if headers else None,
            )
        elif method == "POST":
            response = await client.post(
                path,
                json=body,
                query_params=query_params if query_params else None,
                headers=headers if headers else None,
            )
        elif method == "PUT":
            response = await client.put(
                path,
                json=body,
                query_params=query_params if query_params else None,
                headers=headers if headers else None,
            )
        elif method == "PATCH":
            response = await client.patch(
                path,
                json=body,
                query_params=query_params if query_params else None,
                headers=headers if headers else None,
            )
        elif method == "DELETE":
            response = await client.delete(
                path, query_params=query_params if query_params else None, headers=headers if headers else None
            )
        elif method == "OPTIONS":
            response = await client.options(
                path, query_params=query_params if query_params else None, headers=headers if headers else None
            )
        elif method == "HEAD":
            response = await client.head(
                path, query_params=query_params if query_params else None, headers=headers if headers else None
            )
        else:
            pytest.skip(f"Unsupported HTTP method: {method}")

    except Exception as e:
        pytest.fail(f"Request failed with exception: {e}")

    # Assert status code
    assert response.status_code == expected_status, (
        f"Expected status {expected_status}, got {response.status_code}. Response: {response.text()}"
    )

    # Assert body if expected
    if expected_body is not None:
        actual_body = response.json()
        assert actual_body == expected_body, f"Expected body {expected_body}, got {actual_body}"

    # Assert headers if expected
    for header_name, expected_value in expected_headers.items():
        actual_value = response.headers.get(header_name)
        assert actual_value == expected_value, f"Expected header {header_name}={expected_value}, got {actual_value}"


def test_fixture_discovery() -> None:
    """Verify all fixture categories are discovered."""
    for category in ALL_CATEGORIES:
        len([1 for cat, _, _ in ALL_FIXTURES if cat == category])

    assert len(ALL_FIXTURES) > 0, "No fixtures discovered"


def test_all_categories_have_fixtures() -> None:
    """Verify each category has at least one fixture."""
    for category in ALL_CATEGORIES:
        category_fixtures = [1 for cat, _, _ in ALL_FIXTURES if cat == category]
        count = len(category_fixtures)
        assert count > 0, f"Category {category} has no fixtures"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
