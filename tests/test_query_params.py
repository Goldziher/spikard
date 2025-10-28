"""Parameterized tests for query parameter fixtures.

Tests all query_params fixtures against the test application to ensure:
- Query parameters are correctly parsed
- Type validation works as expected
- Constraint validation (min, max, regex, etc.) works
- Optional vs required parameters behave correctly
- Lists and arrays are handled properly
"""

import pytest

from spikard.testing import TestClient
from tests.conftest import load_all_fixtures
from tests.fixture_app import query_params_app

# Load all query_params fixtures
QUERY_PARAMS_FIXTURES = load_all_fixtures("query_params")


@pytest.fixture
def app():
    """Create the query params test app."""
    return query_params_app()


@pytest.fixture
def client(app):
    """Create a test client for the app."""
    return TestClient(app)


@pytest.mark.parametrize("fixture_id,fixture", QUERY_PARAMS_FIXTURES)
@pytest.mark.asyncio
async def test_query_params_fixture(fixture_id, fixture, client):
    """Test query parameter handling against fixture expectations.

    This test is parameterized to run once for each query_params fixture.
    Each fixture defines:
    - request: The HTTP request details (method, path, query params, etc.)
    - expected_response: The expected response (status code, body, headers)

    Args:
        fixture_id: The fixture file name (for test identification)
        fixture: The fixture data dictionary
        client: TestClient instance
    """
    # Extract fixture data
    fixture["name"]
    fixture["description"]
    request_spec = fixture["request"]
    expected_response = fixture["expected_response"]

    # Extract request components
    method = request_spec["method"]
    path = request_spec["path"]
    query_params = request_spec.get("query_params", {})
    headers = request_spec.get("headers", {})

    # Extract expected response
    expected_status = expected_response["status_code"]
    expected_body = expected_response.get("body")

    # Make actual HTTP request using test client
    if method == "GET":
        response = await client.get(path, query_params=query_params, headers=headers)
    elif method == "POST":
        response = await client.post(path, query_params=query_params, headers=headers)
    elif method == "PUT":
        response = await client.put(path, query_params=query_params, headers=headers)
    elif method == "PATCH":
        response = await client.patch(path, query_params=query_params, headers=headers)
    elif method == "DELETE":
        response = await client.delete(path, query_params=query_params, headers=headers)
    else:
        pytest.fail(f"Unsupported HTTP method: {method}")

    # Assert status code
    assert response.status_code == expected_status, (
        f"Expected status {expected_status}, got {response.status_code}. Response: {response.text()}"
    )

    # Assert body if expected
    if expected_body:
        actual_body = response.json()
        assert actual_body == expected_body, f"Expected body {expected_body}, got {actual_body}"


def test_query_params_fixture_count():
    """Verify we have the expected number of query_params fixtures."""
    assert len(QUERY_PARAMS_FIXTURES) == 40, f"Expected 40 query_params fixtures, found {len(QUERY_PARAMS_FIXTURES)}"


def test_query_params_fixture_names():
    """Verify all fixtures have unique, descriptive names."""
    names = [fixture["name"] for _, fixture in QUERY_PARAMS_FIXTURES]
    assert len(names) == len(set(names)), "Fixture names must be unique"

    # Verify all names are descriptive (not empty or too short)
    for name in names:
        assert len(name) > 10, f"Fixture name too short: {name}"


def test_query_params_fixture_sources():
    """Verify all fixtures document their source framework."""
    for fixture_id, fixture in QUERY_PARAMS_FIXTURES:
        assert "source" in fixture, f"Fixture {fixture_id} missing source field"
        source = fixture["source"]
        assert "framework" in source, f"Fixture {fixture_id} missing framework in source"
        assert source["framework"] in ["fastapi", "starlette", "litestar"], (
            f"Fixture {fixture_id} has unknown framework: {source['framework']}"
        )


if __name__ == "__main__":
    # Run tests with pytest
    pytest.main([__file__, "-v"])
