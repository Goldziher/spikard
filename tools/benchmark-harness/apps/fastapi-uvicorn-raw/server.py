#!/usr/bin/env python3
# /// script
# dependencies = [
#   "fastapi",
#   "uvicorn[standard]",
# ]
# ///
"""FastAPI comparison server for benchmarking (raw/no validation)."""

import sys

import uvicorn
from fastapi import FastAPI, Request

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
async def echo(request: Request) -> dict[str, bool]:
    """Simple POST endpoint with raw JSON handling."""
    await request.json()
    return {"echoed": True}


@app.get("/items")
async def list_items() -> dict[str, list[dict[str, int | str]]]:
    """Return a list of items."""
    return {"items": [{"id": 1, "name": "Item 1"}, {"id": 2, "name": "Item 2"}]}


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(f"FastAPI workload server (raw/no validation) starting on port {port}...")
    uvicorn.run(
        app,
        host="127.0.0.1",
        port=port,
        log_level="error",
        access_log=False,
    )
