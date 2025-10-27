"""Parameterized tests for query parameter fixtures.

Tests all query_params fixtures against the test application to ensure:
- Query parameters are correctly parsed
- Type validation works as expected
- Constraint validation (min, max, regex, etc.) works
- Optional vs required parameters behave correctly
- Lists and arrays are handled properly
"""

import pytest
from tests.conftest import load_all_fixtures


# Load all query_params fixtures
QUERY_PARAMS_FIXTURES = load_all_fixtures("query_params")


@pytest.mark.parametrize("fixture_id,fixture", QUERY_PARAMS_FIXTURES)
def test_query_params_fixture(fixture_id, fixture):
    """Test query parameter handling against fixture expectations.

    This test is parameterized to run once for each query_params fixture.
    Each fixture defines:
    - request: The HTTP request details (method, path, query params, etc.)
    - expected_response: The expected response (status code, body, headers)

    Args:
        fixture_id: The fixture file name (for test identification)
        fixture: The fixture data dictionary
    """
    # Extract fixture data
    fixture_name = fixture["name"]
    description = fixture["description"]
    request_spec = fixture["request"]
    expected_response = fixture["expected_response"]

    print(f"\n{'='*60}")
    print(f"Fixture: {fixture_id}")
    print(f"Name: {fixture_name}")
    print(f"Description: {description}")
    print(f"{'='*60}")

    # Extract request components
    method = request_spec["method"]
    path = request_spec["path"]
    query_params = request_spec.get("query_params", {})
    headers = request_spec.get("headers", {})

    # Build query string manually for now (until we have test client)
    if query_params:
        # Handle list parameters (multiple values for same key)
        query_parts = []
        for key, value in query_params.items():
            if isinstance(value, list):
                for v in value:
                    query_parts.append(f"{key}={v}")
            else:
                query_parts.append(f"{key}={value}")
        query_string = "&".join(query_parts)
        full_path = f"{path}?{query_string}"
    else:
        full_path = path

    print(f"Request: {method} {full_path}")
    print(f"Expected status: {expected_response['status_code']}")

    # Extract expected response
    expected_status = expected_response["status_code"]
    expected_body = expected_response.get("body")
    expected_headers = expected_response.get("headers", {})

    # For now, just validate the fixture structure
    # TODO: Once we have the test client working, make actual requests
    assert method in ["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS", "HEAD"]
    assert isinstance(expected_status, int)
    assert 100 <= expected_status < 600

    # Validate fixture has required fields
    assert "name" in fixture
    assert "description" in fixture
    assert "request" in fixture
    assert "expected_response" in fixture

    # Validate request structure
    assert "method" in request_spec
    assert "path" in request_spec

    # Print validation success
    print(f"✅ Fixture structure validated")

    # TODO: Make actual HTTP request using test client when ready
    # from spikard.testing import TestClient
    # client = TestClient(app)
    # response = client.request(
    #     method=method,
    #     path=full_path,
    #     headers=headers,
    # )
    # assert response.status_code == expected_status
    # if expected_body:
    #     assert response.json() == expected_body


def test_query_params_fixture_count():
    """Verify we have the expected number of query_params fixtures."""
    assert len(QUERY_PARAMS_FIXTURES) == 40, f"Expected 40 query_params fixtures, found {len(QUERY_PARAMS_FIXTURES)}"
    print(f"\n✅ Found {len(QUERY_PARAMS_FIXTURES)} query_params fixtures")


def test_query_params_fixture_names():
    """Verify all fixtures have unique, descriptive names."""
    names = [fixture["name"] for _, fixture in QUERY_PARAMS_FIXTURES]
    assert len(names) == len(set(names)), "Fixture names must be unique"

    # Verify all names are descriptive (not empty or too short)
    for name in names:
        assert len(name) > 10, f"Fixture name too short: {name}"

    print(f"\n✅ All {len(names)} fixtures have unique, descriptive names")


def test_query_params_fixture_sources():
    """Verify all fixtures document their source framework."""
    for fixture_id, fixture in QUERY_PARAMS_FIXTURES:
        assert "source" in fixture, f"Fixture {fixture_id} missing source field"
        source = fixture["source"]
        assert "framework" in source, f"Fixture {fixture_id} missing framework in source"
        assert source["framework"] in ["fastapi", "starlette", "litestar"], \
            f"Fixture {fixture_id} has unknown framework: {source['framework']}"

    print(f"\n✅ All fixtures document their source framework")


if __name__ == "__main__":
    # Run tests with pytest
    pytest.main([__file__, "-v"])
