"""Example Spikard application with query parameters."""

from typing import Any

from spikard import Spikard

app = Spikard()


@app.get("/")
async def index() -> dict[str, str]:
    """Root endpoint."""
    return {"message": "Hello from Spikard!"}


@app.get("/search")
async def search(query: str = "default", limit: int = 10) -> dict[str, Any]:
    """Search endpoint with query parameters."""
    results = [f"Result {i + 1} for '{query}'" for i in range(limit)]
    return {"query": query, "limit": limit, "results": results}


@app.get("/users/{user_id}")
async def get_user(user_id: int, include_details: bool = False) -> dict[str, Any]:
    """Get user by ID with optional query parameter."""
    user: dict[str, Any] = {"user_id": user_id, "name": f"User {user_id}"}

    if include_details:
        user["email"] = f"user{user_id}@example.com"
        user["created_at"] = "2025-01-01"

    return user


if __name__ == "__main__":
    app.run(host="127.0.0.1", port=8000)
