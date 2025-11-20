#!/usr/bin/env python3
"""FastAPI benchmark server for comparison.

Uses ORJSONResponse for optimal JSON performance.
"""

from typing import Any

from fastapi import FastAPI
from fastapi.responses import ORJSONResponse

app = FastAPI(default_response_class=ORJSONResponse)


@app.get("/health")
async def health() -> dict[str, Any]:
    """Health check endpoint."""
    return {"status": "ok"}


@app.post("/items/")
async def post_items_() -> dict[str, Any]:
    """Handler for POST /items/."""
    return {}


@app.post("/items/nested")
async def post_items_nested() -> dict[str, Any]:
    """Handler for POST /items/nested."""
    return {}


@app.post("/items/list")
async def post_items_list() -> dict[str, Any]:
    """Handler for POST /items/list."""
    return {}


@app.post("/items/validated")
async def post_items_validated() -> dict[str, Any]:
    """Handler for POST /items/validated."""
    return {}


@app.post("/items/optional-all")
async def post_items_optional_all() -> dict[str, Any]:
    """Handler for POST /items/optional-all."""
    return {}


@app.patch("/items/{id}")
async def patch_items_id(id: str) -> dict[str, Any]:
    """Handler for PATCH /items/{id}."""
    return {"id": id}


if __name__ == "__main__":
    import sys

    import uvicorn

    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(f"Starting FastAPI server on port {port}", file=sys.stderr)
    uvicorn.run(app, host="0.0.0.0", port=port, log_level="error")
