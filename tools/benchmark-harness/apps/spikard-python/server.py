"""Simple Spikard-Python benchmark server with sync handlers."""

import sys

from spikard import Spikard, get, post

app = Spikard()


@get("/")
def root() -> dict:
    """Health check endpoint."""
    return {"status": "ok", "framework": "spikard-python"}


@get("/health")
def health() -> dict:
    """Health check endpoint."""
    return {"status": "healthy"}


@get("/simple")
def get_simple() -> dict:
    """Simple GET with no parameters."""
    return {"message": "Simple GET response"}


@get("/users/{id}")
def get_user(user_id: str) -> dict:
    """GET with path parameter."""
    return {"id": user_id, "name": f"User {user_id}"}


@get("/search")
def search(q: str = "") -> dict:
    """GET with query parameter."""
    return {"query": q, "results": []}


@post("/data")
def post_data(name: str, value: int) -> dict:
    """POST with simple JSON body."""
    return {"name": name, "value": value, "created": True}


@post("/nested")
def post_nested(user: dict) -> dict:
    """POST with nested JSON."""
    return {"user": user, "processed": True}


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    app.run(host="127.0.0.1", port=port)
