"""
Comprehensive fixture-driven test suite for all testing_data fixtures.

This module provides parametrized tests that load and validate all fixture
categories, ensuring consistent request handling across all supported platforms.

Test Coverage:
- Headers fixtures (standard headers, custom headers, validation)
- Cookies fixtures (secure cookies, cookie attributes)
- JSON body fixtures (nested objects, validation, edge cases)
- Validation error fixtures (422 responses, error payloads)
- Status code fixtures (all HTTP status codes)
- Query parameter fixtures (parsing, validation)
- Path parameter fixtures (type coercion, validation)
- HTTP method fixtures (GET, POST, PUT, PATCH, DELETE, etc.)
- Content type fixtures (JSON, form data, multipart, etc.)
- Edge case fixtures (deeply nested objects, large payloads, special characters)
- Auth fixtures (JWT, API keys, basic auth)
- CORS fixtures (preflight requests, allowed origins)
- Streaming fixtures (server-sent events, chunked responses)
- URL-encoded form data fixtures
- Multipart form data fixtures
- Lifecycle hooks fixtures (onRequest, preValidation, etc.)
- Rate limiting fixtures
- Request timeout fixtures
- Request ID fixtures
- Compression fixtures (gzip, brotli)
- Body limit fixtures
- Background task fixtures
"""

from __future__ import annotations

from typing import Protocol

import pytest


class FixtureValidator(Protocol):
    """Protocol for fixture validator callable."""

    def __call__(self, category: str, fixture_data: dict[str, object]) -> tuple[bool, list[str]]:
        """Validate fixture data against schema."""


FIXTURE_CATEGORIES = [
    "headers",
    "cookies",
    "json_bodies",
    "validation_errors",
    "status_codes",
    "query_params",
    "path_params",
    "http_methods",
    "content_types",
    "edge_cases",
    "auth",
    "cors",
    "streaming",
    "url_encoded",
    "multipart",
    "lifecycle_hooks",
    "rate_limit",
    "request_timeout",
    "request_id",
    "compression",
    "body_limits",
    "background",
]


def _validate_fixture_structure(
    category: str, fixture: dict[str, object], fixture_validator: FixtureValidator
) -> None:
    """Validate a single fixture's structure and content.

    Args:
        category: The fixture category name
        fixture: The fixture data to validate
        fixture_validator: Validator function for schema compliance

    Raises:
        AssertionError: If any validation check fails
    """
    fixture_name_value: object = fixture.get("name", "unknown")
    fixture_name = str(fixture_name_value) if fixture_name_value else "unknown"

    skip_value: object = fixture.get("skip", False)
    if isinstance(skip_value, bool) and skip_value:
        return

    is_valid, errors = fixture_validator(category, fixture)
    assert is_valid, f"Fixture validation failed for '{fixture_name}' in {category}: {errors}"

    assert "name" in fixture, f"Fixture in {category} must have a 'name' field"
    assert "description" in fixture, f"Fixture '{fixture_name}' in {category} must have a 'description' field"
    assert "request" in fixture, f"Fixture '{fixture_name}' in {category} must have a 'request' field"
    assert "expected_response" in fixture, (
        f"Fixture '{fixture_name}' in {category} must have an 'expected_response' field"
    )

    request_value: object = fixture["request"]
    if isinstance(request_value, dict):
        assert "method" in request_value, (
            f"Request in fixture '{fixture_name}' ({category}) must have a 'method' field"
        )
        assert "path" in request_value, (
            f"Request in fixture '{fixture_name}' ({category}) must have a 'path' field"
        )

    response_value: object = fixture["expected_response"]
    if isinstance(response_value, dict):
        assert "status_code" in response_value, (
            f"Response in fixture '{fixture_name}' ({category}) must have a 'status_code' field"
        )
        status_code = response_value.get("status_code")
        assert isinstance(status_code, int), (
            f"Status code in fixture '{fixture_name}' ({category}) must be an integer"
        )
        assert 100 <= status_code <= 599, (
            f"Status code in fixture '{fixture_name}' ({category}) must be between 100 and 599"
        )


@pytest.mark.parametrize("category", FIXTURE_CATEGORIES)
def test_fixtures(
    category: str, fixture_categories: dict[str, list[dict[str, object]]], fixture_validator: FixtureValidator
) -> None:
    """Test all fixtures in the specified category.

    Args:
        category: The fixture category name (e.g., 'headers', 'cookies')
        fixture_categories: All loaded fixtures organized by category
        fixture_validator: Validator function for schema compliance
    """
    fixtures = fixture_categories.get(category, [])

    if not fixtures:
        pytest.skip(f"No fixtures found for category '{category}'")

    for fixture in fixtures:
        _validate_fixture_structure(category, fixture, fixture_validator)
