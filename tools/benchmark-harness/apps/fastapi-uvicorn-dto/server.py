"""FastAPI comparison server for benchmarking."""

import sys

import uvicorn

from fastapi import FastAPI

app = FastAPI()


@app.get("/")
async def root() -> dict[str, str]:
    """Simple root endpoint for basic throughput testing."""
    return {"message": "Hello, World!"}


@app.get("/health")
async def health() -> dict[str, str]:
    """Health check endpoint."""
    return {"status": "healthy"}


@app.get("/users/{user_id}")
async def get_user(user_id: int) -> dict[str, int | str]:
    """Path parameter extraction and validation."""
    return {"user_id": user_id, "name": f"User {user_id}"}


@app.post("/echo")
async def echo() -> dict[str, bool]:
    """Simple POST endpoint."""
    return {"echoed": True}


@app.get("/items")
async def list_items() -> dict[str, list[dict[str, int | str]]]:
    """Return a list of items."""
    return {"items": [{"id": 1, "name": "Item 1"}, {"id": 2, "name": "Item 2"}]}


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    uvicorn.run(
        app,
        host="127.0.0.1",
        port=port,
        log_level="error",
        access_log=False,
    )
