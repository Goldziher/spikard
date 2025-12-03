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

from typing import Any

import pytest


def create_fixture_test(category: str) -> type:
    """
    Create a test class for a fixture category dynamically.

    Args:
        category: The fixture category name (e.g., 'headers', 'cookies')

    Returns:
        Test class with parametrized tests for all fixtures in the category
    """

    class FixtureCategoryTest:
        """Generic fixture category test."""

        @pytest.mark.fixture_category(category)
        def test_fixtures(self, fixture_categories: dict[str, list[dict[str, Any]]], fixture_validator: Any) -> None:
            """Test all fixtures in this category.

            Args:
                fixture_categories: All loaded fixtures organized by category
                fixture_validator: Validator function for schema compliance
            """
            fixtures = fixture_categories.get(category, [])

            if not fixtures:
                pytest.skip(f"No fixtures found for category '{category}'")

            for fixture in fixtures:
                fixture_name = fixture.get("name", "unknown")

                if fixture.get("skip"):
                    continue

                is_valid, errors = fixture_validator(category, fixture)
                assert is_valid, f"Fixture validation failed for '{fixture_name}' in {category}: {errors}"

                assert "name" in fixture, f"Fixture in {category} must have a 'name' field"
                assert "description" in fixture, (
                    f"Fixture '{fixture_name}' in {category} must have a 'description' field"
                )
                assert "request" in fixture, f"Fixture '{fixture_name}' in {category} must have a 'request' field"
                assert "expected_response" in fixture, (
                    f"Fixture '{fixture_name}' in {category} must have an 'expected_response' field"
                )

                request = fixture["request"]
                assert "method" in request, (
                    f"Request in fixture '{fixture_name}' ({category}) must have a 'method' field"
                )
                assert "path" in request, f"Request in fixture '{fixture_name}' ({category}) must have a 'path' field"

                response = fixture["expected_response"]
                assert "status_code" in response, (
                    f"Response in fixture '{fixture_name}' ({category}) must have a 'status_code' field"
                )
                assert isinstance(response["status_code"], int), (
                    f"Status code in fixture '{fixture_name}' ({category}) must be an integer"
                )
                assert 100 <= response["status_code"] <= 599, (
                    f"Status code in fixture '{fixture_name}' ({category}) must be between 100 and 599"
                )

    FixtureCategoryTest.__name__ = f"Test{category.capitalize()}Fixtures"
    FixtureCategoryTest.__qualname__ = f"Test{category.capitalize()}Fixtures"

    return FixtureCategoryTest


# Create test classes for all fixture categories
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

for category in FIXTURE_CATEGORIES:
    test_class = create_fixture_test(category)
    globals()[test_class.__name__] = test_class
