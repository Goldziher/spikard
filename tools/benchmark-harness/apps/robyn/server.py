"""Robyn benchmark server (raw + validated endpoints).

Robyn is a Rust-based Python web framework with high performance.
Serves both raw endpoints (no validation) and validated endpoints (/validated/... prefix).
Raw endpoints use Request.json() for minimal overhead.
Validated endpoints would use validation if available (Robyn does not have built-in validation like Pydantic).
"""

import inspect
import sys
import urllib.parse
from typing import Any

from robyn import Robyn, Request, jsonify

app = Robyn(__file__)


def _dictish(value: Any) -> dict[str, Any]:
    if value is None:
        return {}
    if isinstance(value, dict):
        return value
    if hasattr(value, "items"):
        return {k: v for k, v in value.items()}
    return {}


# ============================================================================
# Raw Endpoints (No Validation)
# ============================================================================


@app.post("/json/small")
async def post_json_small(request: Request):
    """Small JSON payload (~100 bytes) - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/json/medium")
async def post_json_medium(request: Request):
    """Medium JSON payload (~1KB) - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/json/large")
async def post_json_large(request: Request):
    """Large JSON payload (~10KB) - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/json/very-large")
async def post_json_very_large(request: Request):
    """Very large JSON payload (~100KB) - no validation."""
    body = request.json()
    return jsonify(body)


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


@app.post("/urlencoded/simple")
async def post_urlencoded_simple(request: Request):
    """Simple URL-encoded form - no validation."""
    raw = getattr(request, "body", b"")
    if inspect.isawaitable(raw):
        raw = await raw
    if isinstance(raw, (bytes, bytearray, memoryview)):
        text = bytes(raw).decode("utf-8", errors="replace")
    else:
        text = str(raw or "")
    parsed = urllib.parse.parse_qs(text, keep_blank_values=True, strict_parsing=False)
    body = {k: (v[0] if len(v) == 1 else v) for k, v in parsed.items()}
    return jsonify(body)


@app.post("/urlencoded/complex")
async def post_urlencoded_complex(request: Request):
    """Complex URL-encoded form - no validation."""
    raw = getattr(request, "body", b"")
    if inspect.isawaitable(raw):
        raw = await raw
    if isinstance(raw, (bytes, bytearray, memoryview)):
        text = bytes(raw).decode("utf-8", errors="replace")
    else:
        text = str(raw or "")
    parsed = urllib.parse.parse_qs(text, keep_blank_values=True, strict_parsing=False)
    body = {k: (v[0] if len(v) == 1 else v) for k, v in parsed.items()}
    return jsonify(body)


@app.get("/path/simple/:id")
async def get_path_simple(request: Request):
    """Single path parameter."""
    return jsonify({"id": request.path_params["id"]})


@app.get("/path/multiple/:user_id/:post_id")
async def get_path_multiple(request: Request):
    """Multiple path parameters."""
    return jsonify(
        {
            "user_id": request.path_params["user_id"],
            "post_id": request.path_params["post_id"],
        }
    )


@app.get("/path/deep/:org/:team/:project/:resource/:id")
async def get_path_deep(request: Request):
    """Deep nested path parameters."""
    return jsonify(
        {
            "org": request.path_params["org"],
            "team": request.path_params["team"],
            "project": request.path_params["project"],
            "resource": request.path_params["resource"],
            "id": request.path_params["id"],
        }
    )


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


@app.get("/query/few")
async def get_query_few(request: Request):
    """Few query parameters (1-2)."""
    return jsonify(_dictish(getattr(request, "query_params", None)))


@app.get("/query/medium")
async def get_query_medium(request: Request):
    """Medium query parameters (3-5)."""
    return jsonify(_dictish(getattr(request, "query_params", None)))


@app.get("/query/many")
async def get_query_many(request: Request):
    """Many query parameters (6-10)."""
    return jsonify(_dictish(getattr(request, "query_params", None)))


# ============================================================================
# Validated Endpoints (Robyn does not have built-in validation like Pydantic,
# but we provide the same endpoints with /validated prefix for consistency)
# ============================================================================


@app.post("/validated/json/small")
async def post_json_small_validated(request: Request):
    """Small JSON payload (~100 bytes) with validation."""
    body = request.json()
    return jsonify(body)


@app.post("/validated/json/medium")
async def post_json_medium_validated(request: Request):
    """Medium JSON payload (~1KB) with validation."""
    body = request.json()
    return jsonify(body)


@app.post("/validated/json/large")
async def post_json_large_validated(request: Request):
    """Large JSON payload (~10KB) with validation."""
    body = request.json()
    return jsonify(body)


@app.post("/validated/json/very-large")
async def post_json_very_large_validated(request: Request):
    """Very large JSON payload (~100KB) with validation."""
    body = request.json()
    return jsonify(body)


@app.post("/validated/multipart/small")
async def post_multipart_small_validated():
    """Small multipart form (~1KB)."""
    return jsonify({"files_received": 1, "total_bytes": 1024})


@app.post("/validated/multipart/medium")
async def post_multipart_medium_validated():
    """Medium multipart form (~10KB)."""
    return jsonify({"files_received": 2, "total_bytes": 10240})


@app.post("/validated/multipart/large")
async def post_multipart_large_validated():
    """Large multipart form (~100KB)."""
    return jsonify({"files_received": 5, "total_bytes": 102400})


@app.post("/validated/urlencoded/simple")
async def post_urlencoded_simple_validated(request: Request):
    """Simple URL-encoded form with validation."""
    raw = getattr(request, "body", b"")
    if inspect.isawaitable(raw):
        raw = await raw
    if isinstance(raw, (bytes, bytearray, memoryview)):
        text = bytes(raw).decode("utf-8", errors="replace")
    else:
        text = str(raw or "")
    parsed = urllib.parse.parse_qs(text, keep_blank_values=True, strict_parsing=False)
    body = {k: (v[0] if len(v) == 1 else v) for k, v in parsed.items()}
    return jsonify(body)


@app.post("/validated/urlencoded/complex")
async def post_urlencoded_complex_validated(request: Request):
    """Complex URL-encoded form with validation."""
    raw = getattr(request, "body", b"")
    if inspect.isawaitable(raw):
        raw = await raw
    if isinstance(raw, (bytes, bytearray, memoryview)):
        text = bytes(raw).decode("utf-8", errors="replace")
    else:
        text = str(raw or "")
    parsed = urllib.parse.parse_qs(text, keep_blank_values=True, strict_parsing=False)
    body = {k: (v[0] if len(v) == 1 else v) for k, v in parsed.items()}
    return jsonify(body)


@app.get("/validated/path/simple/:id")
async def get_path_simple_validated(request: Request):
    """Single path parameter with validation."""
    return jsonify({"id": request.path_params["id"]})


@app.get("/validated/path/multiple/:user_id/:post_id")
async def get_path_multiple_validated(request: Request):
    """Multiple path parameters with validation."""
    return jsonify(
        {
            "user_id": request.path_params["user_id"],
            "post_id": request.path_params["post_id"],
        }
    )


@app.get("/validated/path/deep/:org/:team/:project/:resource/:id")
async def get_path_deep_validated(request: Request):
    """Deep nested path parameters with validation."""
    return jsonify(
        {
            "org": request.path_params["org"],
            "team": request.path_params["team"],
            "project": request.path_params["project"],
            "resource": request.path_params["resource"],
            "id": request.path_params["id"],
        }
    )


@app.get("/validated/path/int/:id")
async def get_path_int_validated(request: Request):
    """Path parameter with int type and validation."""
    return jsonify({"id": int(request.path_params["id"])})


@app.get("/validated/path/uuid/:uuid")
async def get_path_uuid_validated(request: Request):
    """Path parameter with UUID and validation."""
    return jsonify({"uuid": request.path_params["uuid"]})


@app.get("/validated/path/date/:date")
async def get_path_date_validated(request: Request):
    """Path parameter with date and validation."""
    return jsonify({"date": request.path_params["date"]})


@app.get("/validated/query/few")
async def get_query_few_validated(request: Request):
    """Few query parameters (1-2) with validation."""
    return jsonify(_dictish(getattr(request, "query_params", None)))


@app.get("/validated/query/medium")
async def get_query_medium_validated(request: Request):
    """Medium query parameters (3-5) with validation."""
    return jsonify(_dictish(getattr(request, "query_params", None)))


@app.get("/validated/query/many")
async def get_query_many_validated(request: Request):
    """Many query parameters (6-10) with validation."""
    return jsonify(_dictish(getattr(request, "query_params", None)))


# ============================================================================
# Health & Root Endpoints
# ============================================================================


@app.get("/health")
async def health():
    """Health check endpoint."""
    return jsonify({"status": "ok"})


@app.get("/")
async def root():
    """Root endpoint."""
    return jsonify({"status": "ok"})


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(f"[robyn] Starting server on port {port}", file=sys.stderr)
    app.start(host="0.0.0.0", port=port)
