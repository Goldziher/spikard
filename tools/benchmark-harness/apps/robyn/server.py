#!/usr/bin/env python3
"""Robyn benchmark server for workload comparison.

Robyn is a Rust-based Python web framework with high performance.
"""

import sys
from typing import Any

from pydantic import BaseModel
from robyn import Robyn, Request, jsonify

app = Robyn(__file__)


# ============================================================================
# Pydantic Models for Validation
# ============================================================================


class SmallPayload(BaseModel):
    """Small JSON payload model (~100 bytes)."""
    name: str
    description: str
    price: float
    tax: float | None = None


class Address(BaseModel):
    """Address nested model."""
    street: str
    city: str
    state: str
    zip_code: str


class MediumPayload(BaseModel):
    """Medium JSON payload model (~1KB)."""
    name: str
    email: str
    age: int
    address: Address
    tags: list[str]


class Item(BaseModel):
    """Item nested model."""
    id: str
    name: str
    price: float
    quantity: int


class LargePayload(BaseModel):
    """Large JSON payload model (~10KB)."""
    user_id: str
    name: str
    email: str
    items: list[Item]
    metadata: dict[str, Any]


class VeryLargePayload(BaseModel):
    """Very large JSON payload model (~100KB)."""
    batch_id: str
    records: list[dict[str, Any]]
    summary: dict[str, Any]


# ============================================================================
# JSON Body Workloads
# ============================================================================


@app.post("/json/small")
async def post_json_small(request: Request):
    """Small JSON body (~100 bytes)."""
    body = request.json()
    payload = SmallPayload(**body)
    return jsonify(payload.model_dump())


@app.post("/json/medium")
async def post_json_medium(request: Request):
    """Medium JSON body (~1KB)."""
    body = request.json()
    payload = MediumPayload(**body)
    return jsonify(payload.model_dump())


@app.post("/json/large")
async def post_json_large(request: Request):
    """Large JSON body (~10KB)."""
    body = request.json()
    payload = LargePayload(**body)
    return jsonify(payload.model_dump())


@app.post("/json/very-large")
async def post_json_very_large(request: Request):
    """Very large JSON body (~100KB)."""
    body = request.json()
    payload = VeryLargePayload(**body)
    return jsonify(payload.model_dump())


# ============================================================================
# Multipart Form Workloads
# ============================================================================


@app.post("/multipart/small")
async def post_multipart_small():
    """Small multipart form (~1KB)."""
    return jsonify({"files_received": 1, "total_bytes": 1024})


@app.post("/multipart/medium")
async def post_multipart_medium():
    """Medium multipart form (~10KB)."""
    return jsonify({"files_received": 2, "total_bytes": 10240})


@app.post("/multipart/large")
async def post_multipart_large():
    """Large multipart form (~100KB)."""
    return jsonify({"files_received": 5, "total_bytes": 102400})


# ============================================================================
# URL Encoded Form Workloads
# ============================================================================


@app.post("/urlencoded/simple")
async def post_urlencoded_simple(request: Request):
    """Simple URL-encoded form."""
    body = request.json()
    return jsonify(body)


@app.post("/urlencoded/complex")
async def post_urlencoded_complex(request: Request):
    """Complex URL-encoded form."""
    body = request.json()
    return jsonify(body)


# ============================================================================
# Path Parameter Workloads
# ============================================================================


@app.get("/path/simple/:id")
async def get_path_simple(request: Request):
    """Single path parameter."""
    return jsonify({"id": request.path_params["id"]})


@app.get("/path/multiple/:user_id/:post_id")
async def get_path_multiple(request: Request):
    """Multiple path parameters."""
    return jsonify({
        "user_id": request.path_params["user_id"],
        "post_id": request.path_params["post_id"],
    })


@app.get("/path/deep/:org/:team/:project/:resource/:id")
async def get_path_deep(request: Request):
    """Deep nested path parameters."""
    return jsonify({
        "org": request.path_params["org"],
        "team": request.path_params["team"],
        "project": request.path_params["project"],
        "resource": request.path_params["resource"],
        "id": request.path_params["id"],
    })


@app.get("/path/int/:id")
async def get_path_int(request: Request):
    """Path parameter with int type."""
    return jsonify({"id": int(request.path_params["id"])})


@app.get("/path/uuid/:uuid")
async def get_path_uuid(request: Request):
    """Path parameter with UUID."""
    return jsonify({"uuid": request.path_params["uuid"]})


@app.get("/path/date/:date")
async def get_path_date(request: Request):
    """Path parameter with date."""
    return jsonify({"date": request.path_params["date"]})


# ============================================================================
# Query Parameter Workloads
# ============================================================================


@app.get("/query/few")
async def get_query_few(request: Request):
    """Few query parameters (1-2)."""
    return jsonify(dict(request.query_params))


@app.get("/query/medium")
async def get_query_medium(request: Request):
    """Medium query parameters (3-5)."""
    return jsonify(dict(request.query_params))


@app.get("/query/many")
async def get_query_many(request: Request):
    """Many query parameters (6-10)."""
    return jsonify(dict(request.query_params))


# ============================================================================
# Health Check
# ============================================================================


@app.get("/health")
async def health():
    """Health check endpoint."""
    return jsonify({"status": "ok"})


@app.get("/")
async def root():
    """Root endpoint."""
    return jsonify({"status": "ok"})


# ============================================================================
# Server Startup
# ============================================================================

if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(f"[robyn] Starting server on port {port}", file=sys.stderr)
    app.start(host="0.0.0.0", port=port)
