"""Integration tests for query parameters using real HTTP requests.

These tests create a Spikard application and use the TestClient to make
real HTTP requests, verifying the framework works end-to-end.
"""

import pytest
from typing import Optional, List
from uuid import UUID
from datetime import date, datetime
from enum import Enum

from spikard import Spikard
from spikard.testing import TestClient
from pydantic import Field


# Define test app models
class Status(str, Enum):
    active = "active"
    inactive = "inactive"
    pending = "pending"


# Create test application
@pytest.fixture
def app():
    """Create a Spikard app for query parameter testing."""
    app = Spikard()

    @app.get("/items")
    def get_items(
        q: Optional[str] = None,
        page: int = 1,
        limit: Optional[int] = None,
        tags: Optional[List[str]] = None,
        active: Optional[bool] = None,
        item_id: Optional[UUID] = None,
        status: Optional[Status] = None,
        created_after: Optional[date] = None,
        updated_at: Optional[datetime] = None,
        min_price: Optional[float] = None,
        search: Optional[str] = Field(None, min_length=3, max_length=50),
        code: Optional[str] = Field(None, pattern=r"^[A-Z]{3}$"),
    ):
        """Generic endpoint for query parameter testing."""
        result = {"message": "Query parameters received", "page": page}

        if q is not None:
            result["q"] = q
        if limit is not None:
            result["limit"] = limit
        if tags is not None:
            result["tags"] = tags
        if active is not None:
            result["active"] = active
        if item_id is not None:
            result["item_id"] = str(item_id)
        if status is not None:
            result["status"] = status
        if created_after is not None:
            result["created_after"] = created_after.isoformat()
        if updated_at is not None:
            result["updated_at"] = updated_at.isoformat()
        if min_price is not None:
            result["min_price"] = min_price
        if search is not None:
            result["search"] = search
        if code is not None:
            result["code"] = code

        return result

    @app.get("/users/{user_id}")
    def get_user(user_id: int = Field(gt=0)):
        """Endpoint with validated path param."""
        return {"user_id": user_id}

    return app


@pytest.fixture
def client(app):
    """Create a test client for the app."""
    return TestClient(app)


# Integration tests
@pytest.mark.asyncio
async def test_required_string_success(client):
    """Test required string query parameter."""
    response = await client.get("/items", query_params={"q": "test", "page": "1"})

    assert response.status_code == 200
    data = response.json()
    assert data["q"] == "test"
    assert data["page"] == 1


@pytest.mark.asyncio
async def test_optional_string_missing(client):
    """Test optional string parameter can be omitted."""
    response = await client.get("/items", query_params={"page": "1"})

    assert response.status_code == 200
    data = response.json()
    assert "q" not in data
    assert data["page"] == 1


@pytest.mark.asyncio
async def test_int_query_param(client):
    """Test integer query parameter parsing."""
    response = await client.get("/items", query_params={"page": "1", "limit": "10"})

    assert response.status_code == 200
    data = response.json()
    assert data["page"] == 1
    assert data["limit"] == 10


@pytest.mark.asyncio
async def test_int_validation_error(client):
    """Test integer validation with invalid type."""
    response = await client.get("/items", query_params={"page": "1", "limit": "not_a_number"})

    assert response.status_code == 422
    data = response.json()
    assert "detail" in data


@pytest.mark.asyncio
async def test_bool_query_param_true(client):
    """Test boolean query parameter with 'true' value."""
    response = await client.get("/items", query_params={"page": "1", "active": "true"})

    assert response.status_code == 200
    data = response.json()
    assert data["active"] is True


@pytest.mark.asyncio
async def test_bool_query_param_false(client):
    """Test boolean query parameter with 'false' value."""
    response = await client.get("/items", query_params={"page": "1", "active": "false"})

    assert response.status_code == 200
    data = response.json()
    assert data["active"] is False


@pytest.mark.asyncio
async def test_list_query_param(client):
    """Test list query parameter with multiple values."""
    response = await client.get("/items", query_params={
        "page": "1",
        "tags": ["python", "rust", "web"]
    })

    assert response.status_code == 200
    data = response.json()
    assert data["tags"] == ["python", "rust", "web"]


@pytest.mark.asyncio
async def test_enum_query_param(client):
    """Test enum query parameter."""
    response = await client.get("/items", query_params={"page": "1", "status": "active"})

    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "active"


@pytest.mark.asyncio
async def test_enum_invalid_value(client):
    """Test enum query parameter with invalid value."""
    response = await client.get("/items", query_params={"page": "1", "status": "invalid"})

    assert response.status_code == 422
    data = response.json()
    assert "detail" in data


@pytest.mark.asyncio
async def test_uuid_query_param(client):
    """Test UUID query parameter."""
    test_uuid = "123e4567-e89b-12d3-a456-426614174000"
    response = await client.get("/items", query_params={"page": "1", "item_id": test_uuid})

    assert response.status_code == 200
    data = response.json()
    assert data["item_id"] == test_uuid


@pytest.mark.asyncio
async def test_date_query_param(client):
    """Test date query parameter."""
    response = await client.get("/items", query_params={"page": "1", "created_after": "2024-01-01"})

    assert response.status_code == 200
    data = response.json()
    assert data["created_after"] == "2024-01-01"


@pytest.mark.asyncio
async def test_float_query_param(client):
    """Test float query parameter."""
    response = await client.get("/items", query_params={"page": "1", "min_price": "9.99"})

    assert response.status_code == 200
    data = response.json()
    assert data["min_price"] == 9.99


@pytest.mark.asyncio
async def test_string_min_length_validation(client):
    """Test string min_length validation."""
    response = await client.get("/items", query_params={"page": "1", "search": "ab"})

    assert response.status_code == 422
    data = response.json()
    assert "detail" in data


@pytest.mark.asyncio
async def test_string_pattern_validation(client):
    """Test string pattern validation."""
    # Valid pattern
    response = await client.get("/items", query_params={"page": "1", "code": "ABC"})
    assert response.status_code == 200

    # Invalid pattern
    response = await client.get("/items", query_params={"page": "1", "code": "abc"})
    assert response.status_code == 422


@pytest.mark.asyncio
async def test_multiple_query_params(client):
    """Test multiple query parameters together."""
    response = await client.get("/items", query_params={
        "page": "2",
        "limit": "20",
        "q": "search term",
        "active": "true",
        "tags": ["tag1", "tag2"]
    })

    assert response.status_code == 200
    data = response.json()
    assert data["page"] == 2
    assert data["limit"] == 20
    assert data["q"] == "search term"
    assert data["active"] is True
    assert data["tags"] == ["tag1", "tag2"]


@pytest.mark.asyncio
async def test_path_param_validation(client):
    """Test path parameter validation."""
    # Valid path param
    response = await client.get("/users/123")
    assert response.status_code == 200
    assert response.json()["user_id"] == 123

    # Invalid path param (not an int)
    response = await client.get("/users/abc")
    assert response.status_code == 422

    # Invalid path param (violates gt=0 constraint)
    response = await client.get("/users/0")
    assert response.status_code == 422


if __name__ == "__main__":
    pytest.main([__file__, "-v", "-s"])
