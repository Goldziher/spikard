"""Robyn benchmark server for workload comparison.

Robyn is a Rust-based Python web framework with high performance.
"""

import atexit
import json
import os
import sys
from functools import wraps
from pathlib import Path as PathLib
from typing import Callable, Coroutine, ParamSpec, TypeVar

import msgspec
from pyinstrument import Profiler
from pyinstrument.renderers.speedscope import SpeedscopeRenderer
from robyn import Robyn, Request, jsonify

app = Robyn(__file__)

_profile_dir: str | None = os.environ.get("SPIKARD_PYTHON_PROFILE_DIR") or None
_profiled_endpoints: set[str] = set()

P = ParamSpec("P")
R = TypeVar("R")

_pyinstrument_output: str | None = os.environ.get("SPIKARD_PYINSTRUMENT_OUTPUT") or None
_pyinstrument_profiler: Profiler | None = None
_pyinstrument_dumped = False


def _dump_profile() -> None:
    global _pyinstrument_dumped
    if _pyinstrument_dumped or _pyinstrument_profiler is None or _pyinstrument_output is None:
        return

    _pyinstrument_dumped = True
    try:
        _pyinstrument_profiler.stop()
        out_path = PathLib(_pyinstrument_output)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(
            _pyinstrument_profiler.output(renderer=SpeedscopeRenderer()),
            encoding="utf-8",
        )
    except Exception as exc:
        print(f"⚠ Failed to write pyinstrument profile: {exc!r}", file=sys.stderr)


if _pyinstrument_output:
    _pyinstrument_profiler = Profiler(async_mode="enabled")
    _pyinstrument_profiler.start()
    atexit.register(_dump_profile)


def _read_json(request: Request) -> object:
    """Robust JSON extraction across Robyn versions."""
    try:
        data = request.json()
    except Exception:
        data = None

    if data is not None:
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


def profile_once(
    name: str,
) -> Callable[[Callable[P, Coroutine[object, object, R]]], Callable[P, Coroutine[object, object, R]]]:
    def decorator(func: Callable[P, Coroutine[object, object, R]]) -> Callable[P, Coroutine[object, object, R]]:
        if _profile_dir is None:
            return func

        @wraps(func)
        async def wrapper(*args: P.args, **kwargs: P.kwargs) -> R:
            if name in _profiled_endpoints:
                return await func(*args, **kwargs)

            _profiled_endpoints.add(name)
            profiler = Profiler(async_mode="enabled")
            profiler.start()
            try:
                return await func(*args, **kwargs)
            finally:
                try:
                    profiler.stop()
                    out_dir = PathLib(_profile_dir)
                    out_dir.mkdir(parents=True, exist_ok=True)
                    out_path = out_dir / f"{name}.speedscope.json"
                    out_path.write_text(
                        profiler.output(renderer=SpeedscopeRenderer()),
                        encoding="utf-8",
                    )
                except Exception as exc:
                    print(f"⚠ Failed to write profile for {name}: {exc!r}", file=sys.stderr)

        return wrapper

    return decorator


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
@profile_once("json-small")
async def post_json_small(request: Request):
    """Small JSON body (~100 bytes)."""
    body = _read_json(request)
    payload = msgspec.convert(body, type=SmallPayload)
    return jsonify(msgspec.to_builtins(payload))


@app.post("/json/medium")
@profile_once("json-medium")
async def post_json_medium(request: Request):
    """Medium JSON body (~1KB)."""
    body = _read_json(request)
    payload = msgspec.convert(body, type=MediumPayload)
    return jsonify(msgspec.to_builtins(payload))


@app.post("/json/large")
@profile_once("json-large")
async def post_json_large(request: Request):
    """Large JSON body (~10KB)."""
    body = _read_json(request)
    payload = msgspec.convert(body, type=LargePayload)
    return jsonify(msgspec.to_builtins(payload))


@app.post("/json/very-large")
@profile_once("json-very-large")
async def post_json_very_large(request: Request):
    """Very large JSON body (~100KB)."""
    body = _read_json(request)
    payload = msgspec.convert(body, type=VeryLargePayload)
    return jsonify(msgspec.to_builtins(payload))


@app.post("/multipart/small")
@profile_once("multipart-small")
async def post_multipart_small():
    """Small multipart form (~1KB)."""
    return jsonify({"files_received": 1, "total_bytes": 1024})


@app.post("/multipart/medium")
@profile_once("multipart-medium")
async def post_multipart_medium():
    """Medium multipart form (~10KB)."""
    return jsonify({"files_received": 2, "total_bytes": 10240})


@app.post("/multipart/large")
@profile_once("multipart-large")
async def post_multipart_large():
    """Large multipart form (~100KB)."""
    return jsonify({"files_received": 5, "total_bytes": 102400})


@app.post("/urlencoded/simple")
@profile_once("urlencoded-simple")
async def post_urlencoded_simple(request: Request):
    """Simple URL-encoded form."""
    body = request.json()
    return jsonify(body)


@app.post("/urlencoded/complex")
@profile_once("urlencoded-complex")
async def post_urlencoded_complex(request: Request):
    """Complex URL-encoded form."""
    body = request.json()
    return jsonify(body)


@app.get("/path/simple/:id")
@profile_once("path-simple")
async def get_path_simple(request: Request):
    """Single path parameter."""
    return jsonify({"id": request.path_params["id"]})


@app.get("/path/multiple/:user_id/:post_id")
@profile_once("path-multiple")
async def get_path_multiple(request: Request):
    """Multiple path parameters."""
    return jsonify(
        {
            "user_id": request.path_params["user_id"],
            "post_id": request.path_params["post_id"],
        }
    )


@app.get("/path/deep/:org/:team/:project/:resource/:id")
@profile_once("path-deep")
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
@profile_once("path-int")
async def get_path_int(request: Request):
    """Path parameter with int type."""
    return jsonify({"id": int(request.path_params["id"])})


@app.get("/path/uuid/:uuid")
@profile_once("path-uuid")
async def get_path_uuid(request: Request):
    """Path parameter with UUID."""
    return jsonify({"uuid": request.path_params["uuid"]})


@app.get("/path/date/:date")
@profile_once("path-date")
async def get_path_date(request: Request):
    """Path parameter with date."""
    return jsonify({"date": request.path_params["date"]})


@app.get("/query/few")
@profile_once("query-few")
async def get_query_few(request: Request):
    """Few query parameters (1-2)."""
    return jsonify(dict(request.query_params))


@app.get("/query/medium")
@profile_once("query-medium")
async def get_query_medium(request: Request):
    """Medium query parameters (3-5)."""
    return jsonify(dict(request.query_params))


@app.get("/query/many")
@profile_once("query-many")
async def get_query_many(request: Request):
    """Many query parameters (6-10)."""
    return jsonify(dict(request.query_params))


@app.get("/health")
async def health():
    """Health check endpoint."""
    return jsonify({"status": "ok"})


@app.get("/__benchmark__/flush-profile")
async def flush_profile():
    _dump_profile()
    return jsonify({"ok": True})


@app.get("/")
async def root():
    """Root endpoint."""
    return jsonify({"status": "ok"})


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(f"[robyn] Starting server on port {port}", file=sys.stderr)
    app.start(host="0.0.0.0", port=port)
