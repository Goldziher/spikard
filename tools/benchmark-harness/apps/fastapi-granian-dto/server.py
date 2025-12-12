"""FastAPI benchmark server with Granian for workload comparison.

Uses ORJSONResponse for optimal JSON performance + Granian Rust server.
"""

import sys
from typing import Any

from fastapi import FastAPI, Request
from fastapi.responses import ORJSONResponse
from pydantic import BaseModel

app = FastAPI(default_response_class=ORJSONResponse)


class SmallPayload(BaseModel):
    """Small JSON payload - matches 01_simple_object_success.json."""

    name: str
    description: str
    price: float
    tax: float | None = None


class Address(BaseModel):
    """Address model for nested payloads."""

    street: str
    city: str
    state: str
    zip_code: str


class MediumPayload(BaseModel):
    """Medium JSON payload with nested objects."""

    user_id: int
    username: str
    email: str
    is_active: bool
    address: Address
    tags: list[str]


class Item(BaseModel):
    """Item model for large payloads."""

    id: int
    name: str
    price: float
    in_stock: bool


class LargePayload(BaseModel):
    """Large JSON payload with arrays."""

    order_id: str
    customer_name: str
    items: list[Item]
    total: float
    notes: str


class VeryLargePayload(BaseModel):
    """Very large JSON payload."""

    data: list[dict[str, Any]]
    metadata: dict[str, Any]


@app.post("/json/small")
async def post_json_small(body: SmallPayload) -> SmallPayload:
    """Small JSON body (~100 bytes)."""
    return body


@app.post("/json/medium")
async def post_json_medium(body: MediumPayload) -> MediumPayload:
    """Medium JSON body (~1KB)."""
    return body


@app.post("/json/large")
async def post_json_large(body: LargePayload) -> LargePayload:
    """Large JSON body (~10KB)."""
    return body


@app.post("/json/very-large")
async def post_json_very_large(body: VeryLargePayload) -> VeryLargePayload:
    """Very large JSON body (~100KB)."""
    return body


@app.post("/multipart/small")
async def post_multipart_small() -> dict[str, Any]:
    """Small multipart form (~1KB)."""
    return {"files_received": 1, "total_bytes": 1024}


@app.post("/multipart/medium")
async def post_multipart_medium() -> dict[str, Any]:
    """Medium multipart form (~10KB)."""
    return {"files_received": 2, "total_bytes": 10240}


@app.post("/multipart/large")
async def post_multipart_large() -> dict[str, Any]:
    """Large multipart form (~100KB)."""
    return {"files_received": 5, "total_bytes": 102400}


@app.post("/urlencoded/simple")
async def post_urlencoded_simple(request: Request) -> dict[str, Any]:
    """Simple URL-encoded form."""
    body = await request.json()
    return body


@app.post("/urlencoded/complex")
async def post_urlencoded_complex(request: Request) -> dict[str, Any]:
    """Complex URL-encoded form."""
    body = await request.json()
    return body


@app.get("/path/simple/{id}")
async def get_path_simple(id: str) -> dict[str, Any]:
    """Single path parameter."""
    return {"id": id}


@app.get("/path/multiple/{user_id}/{post_id}")
async def get_path_multiple(user_id: str, post_id: str) -> dict[str, Any]:
    """Multiple path parameters."""
    return {"user_id": user_id, "post_id": post_id}


@app.get("/path/deep/{org}/{team}/{project}/{resource}/{id}")
async def get_path_deep(org: str, team: str, project: str, resource: str, id: str) -> dict[str, Any]:
    """Deep nested path parameters."""
    return {
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id,
    }


@app.get("/path/int/{id}")
async def get_path_int(id: int) -> dict[str, Any]:
    """Path parameter with int type."""
    return {"id": id}


@app.get("/path/uuid/{uuid}")
async def get_path_uuid(uuid: str) -> dict[str, Any]:
    """Path parameter with UUID."""
    return {"uuid": uuid}


@app.get("/path/date/{date}")
async def get_path_date(date: str) -> dict[str, Any]:
    """Path parameter with date."""
    return {"date": date}


@app.get("/query/few")
async def get_query_few(request: Request) -> dict[str, Any]:
    """Few query parameters (1-2)."""
    return dict(request.query_params)


@app.get("/query/medium")
async def get_query_medium(request: Request) -> dict[str, Any]:
    """Medium query parameters (3-5)."""
    return dict(request.query_params)


@app.get("/query/many")
async def get_query_many(request: Request) -> dict[str, Any]:
    """Many query parameters (6-10)."""
    return dict(request.query_params)


@app.get("/health")
async def health() -> dict[str, Any]:
    """Health check endpoint."""
    return {"status": "ok"}


@app.get("/")
async def root() -> dict[str, Any]:
    """Root endpoint."""
    return {"status": "ok"}


if __name__ == "__main__":
    import uvicorn

    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(f"[fastapi-granian] Starting server on port {port}", file=sys.stderr)
    uvicorn.run(app, host="0.0.0.0", port=port, log_level="error")
