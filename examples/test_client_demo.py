"""Demo test file showing how to use the Spikard test client."""

from typing import Any

import pytest

from spikard import Spikard
from spikard.testing import TestClient

# Create a simple app for testing
app = Spikard()


@app.get("/")
async def root() -> dict[str, str]:
    """Root endpoint."""
    return {"message": "Hello, World!"}


@app.get("/users/{user_id}")
async def get_user(user_id: int) -> dict[str, Any]:
    """Get user by ID."""
    return {"user_id": user_id, "name": f"User {user_id}"}


@app.get("/search")
async def search(query: str = "default", limit: int = 10) -> dict[str, Any]:
    """Search with query parameters."""
    return {"query": query, "limit": limit, "results": [f"Result {i + 1}" for i in range(limit)]}


@app.post("/users")
async def create_user(body: dict[str, Any]) -> dict[str, Any]:
    """Create a new user."""
    return {"id": 123, "name": body.get("name"), "email": body.get("email")}


# Test functions
@pytest.mark.asyncio
async def test_root_endpoint() -> None:
    """Test the root endpoint."""
    client = TestClient(app)
    response = await client.get("/")

    # Test status code
    assert response.status_code == 200
    response.assert_status_ok()

    # Test JSON response
    data = response.json()
    assert data == {"message": "Hello, World!"}


@pytest.mark.asyncio
async def test_path_parameters() -> None:
    """Test path parameters."""
    client = TestClient(app)
    response = await client.get("/users/42")

    response.assert_status_ok()
    data = response.json()
    assert data["user_id"] == 42
    assert data["name"] == "User 42"


@pytest.mark.asyncio
async def test_query_parameters() -> None:
    """Test query parameters."""
    client = TestClient(app)
    response = await client.get("/search", query_params={"query": "rust", "limit": "3"})

    response.assert_status_ok()
    data = response.json()
    assert data["query"] == "rust"
    assert data["limit"] == 3
    assert len(data["results"]) == 3


@pytest.mark.asyncio
async def test_post_with_json() -> None:
    """Test POST request with JSON body."""
    client = TestClient(app)
    response = await client.post("/users", json={"name": "Alice", "email": "alice@example.com"})

    response.assert_status_ok()
    data = response.json()
    assert data["id"] == 123
    assert data["name"] == "Alice"
    assert data["email"] == "alice@example.com"


@pytest.mark.asyncio
async def test_response_headers() -> None:
    """Test response headers."""
    client = TestClient(app)
    response = await client.get("/")

    # Check that we have headers
    assert "content-type" in response.headers
    assert "application/json" in response.headers["content-type"]


@pytest.mark.asyncio
async def test_response_text() -> None:
    """Test response text method."""
    client = TestClient(app)
    response = await client.get("/")

    # Get as text
    text = response.text()
    assert "Hello, World!" in text


@pytest.mark.asyncio
async def test_response_bytes() -> None:
    """Test response bytes method."""
    client = TestClient(app)
    response = await client.get("/")

    # Get as bytes
    body_bytes = response.bytes()
    assert isinstance(body_bytes, bytes)
    assert b"Hello, World!" in body_bytes


if __name__ == "__main__":
    """Run all tests manually."""
    import asyncio

    async def run_all_tests() -> None:
        await test_root_endpoint()
        await test_path_parameters()
        await test_query_parameters()
        await test_post_with_json()
        await test_response_headers()
        await test_response_text()
        await test_response_bytes()

    asyncio.run(run_all_tests())
