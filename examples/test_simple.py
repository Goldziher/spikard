"""Simple test of the Spikard test client."""

import asyncio
from typing import Any

from spikard import Spikard
from spikard.testing import TestClient

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


async def main() -> None:
    """Run all tests."""
    client = TestClient(app)

    response = await client.get("/")
    assert response.status_code == 200
    data = response.json()
    assert data == {"message": "Hello, World!"}

    response = await client.get("/users/42")
    assert response.status_code == 200
    data = response.json()
    assert data["user_id"] == 42
    assert data["name"] == "User 42"

    response = await client.get("/search", query_params={"query": "rust", "limit": "3"})
    assert response.status_code == 200
    data = response.json()
    assert data["query"] == "rust"
    assert data["limit"] == 3
    assert len(data["results"]) == 3

    response = await client.post("/users", json={"name": "Alice", "email": "alice@example.com"})
    assert response.status_code == 200
    data = response.json()
    assert data["name"] == "Alice"
    assert data["email"] == "alice@example.com"

    response = await client.get("/")
    text = response.text()
    assert "Hello, World!" in text
    body_bytes = response.bytes()
    assert b"Hello, World!" in body_bytes
    assert "content-type" in response.headers


if __name__ == "__main__":
    asyncio.run(main())
