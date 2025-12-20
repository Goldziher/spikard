"""Robyn benchmark server for workload comparison."""

import inspect
import json
import sys
import urllib.parse

import msgspec
from robyn import Robyn, Request, jsonify

app = Robyn(__file__)


async def _read_json(request: Request) -> object:
    """Robust JSON extraction across Robyn versions."""
    try:
        data = request.json()
        if inspect.isawaitable(data):
            data = await data
    except Exception:
        data = None

    if data is not None:
        if isinstance(data, (bytes, bytearray, memoryview)):
            return json.loads(bytes(data))
        if isinstance(data, str):
            return json.loads(data)
        return data

    raw = getattr(request, "body", None)
    if raw is None:
        return {}

    if isinstance(raw, (bytes, bytearray, memoryview)):
        raw_text = bytes(raw).decode("utf-8", errors="replace")
    else:
        raw_text = str(raw)

    if not raw_text:
        return {}

    return json.loads(raw_text)


async def _decode_payload(request: Request, payload_type: type) -> object:
    raw = getattr(request, "body", None)
    if inspect.isawaitable(raw):
        raw = await raw

    if raw is not None:
        if isinstance(raw, (bytes, bytearray, memoryview)):
            return msgspec.json.decode(bytes(raw), type=payload_type)
        if isinstance(raw, str):
            return msgspec.json.decode(raw.encode("utf-8"), type=payload_type)

    body = await _read_json(request)
    return msgspec.convert(body, type=payload_type)


class SmallPayload(msgspec.Struct):
    """Small JSON payload model (~100 bytes)."""

    name: str
    description: str
    price: float
    tax: float | None = None


class Image(msgspec.Struct):
    """Image nested model."""

    url: str
    name: str


class MediumPayload(msgspec.Struct):
    """Medium JSON payload model (~1KB)."""

    name: str
    price: float
    image: Image


class Country(msgspec.Struct):
    """Country nested model."""

    name: str
    code: str


class Address(msgspec.Struct):
    """Address nested model."""

    street: str
    city: str
    country: Country


class SellerWithAddress(msgspec.Struct):
    """Seller nested model."""

    name: str
    address: Address


class LargePayload(msgspec.Struct):
    """Large JSON payload model (~10KB)."""

    name: str
    price: float
    seller: SellerWithAddress


class VeryLargePayload(msgspec.Struct):
    """Very large JSON payload model (~100KB)."""

    name: str
    tags: list[str]
    images: list[Image]


@app.post("/json/small")
async def post_json_small(request: Request):
    """Small JSON body (~100 bytes)."""
    payload = await _decode_payload(request, SmallPayload)
    return jsonify(msgspec.to_builtins(payload))


@app.post("/json/medium")
async def post_json_medium(request: Request):
    """Medium JSON body (~1KB)."""
    payload = await _decode_payload(request, MediumPayload)
    return jsonify(msgspec.to_builtins(payload))


@app.post("/json/large")
async def post_json_large(request: Request):
    """Large JSON body (~10KB)."""
    payload = await _decode_payload(request, LargePayload)
    return jsonify(msgspec.to_builtins(payload))


@app.post("/json/very-large")
async def post_json_very_large(request: Request):
    """Very large JSON body (~100KB)."""
    payload = await _decode_payload(request, VeryLargePayload)
    return jsonify(msgspec.to_builtins(payload))


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
    """Simple URL-encoded form."""
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
    """Complex URL-encoded form."""
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
    qp = getattr(request, "query_params", None)
    if qp is None:
        return jsonify({})
    if isinstance(qp, dict):
        return jsonify(qp)
    if hasattr(qp, "items"):
        return jsonify({k: v for k, v in qp.items()})
    return jsonify({})


@app.get("/query/medium")
async def get_query_medium(request: Request):
    """Medium query parameters (3-5)."""
    qp = getattr(request, "query_params", None)
    if qp is None:
        return jsonify({})
    if isinstance(qp, dict):
        return jsonify(qp)
    if hasattr(qp, "items"):
        return jsonify({k: v for k, v in qp.items()})
    return jsonify({})


@app.get("/query/many")
async def get_query_many(request: Request):
    """Many query parameters (6-10)."""
    qp = getattr(request, "query_params", None)
    if qp is None:
        return jsonify({})
    if isinstance(qp, dict):
        return jsonify(qp)
    if hasattr(qp, "items"):
        return jsonify({k: v for k, v in qp.items()})
    return jsonify({})


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
