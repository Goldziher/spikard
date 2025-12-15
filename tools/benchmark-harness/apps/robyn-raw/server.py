"""Robyn benchmark server for workload comparison (raw/no validation).

Robyn is a Rust-based Python web framework with high performance.
This is a raw variant without pydantic validation to measure framework overhead.
"""

import os
import sys
from functools import wraps
from pathlib import Path as PathLib
from typing import Callable, Coroutine, ParamSpec, TypeVar

from pyinstrument import Profiler
from pyinstrument.renderers.speedscope import SpeedscopeRenderer
from robyn import Robyn, Request, jsonify

app = Robyn(__file__)

_profile_dir: str | None = os.environ.get("SPIKARD_PYTHON_PROFILE_DIR") or None
_profiled_endpoints: set[str] = set()

P = ParamSpec("P")
R = TypeVar("R")


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


@app.post("/json/small")
@profile_once("json-small")
async def post_json_small(request: Request) -> object:
    """Small JSON payload (~100 bytes) - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/json/medium")
@profile_once("json-medium")
async def post_json_medium(request: Request) -> object:
    """Medium JSON payload (~1KB) - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/json/large")
@profile_once("json-large")
async def post_json_large(request: Request) -> object:
    """Large JSON payload (~10KB) - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/json/very-large")
@profile_once("json-very-large")
async def post_json_very_large(request: Request) -> object:
    """Very large JSON payload (~100KB) - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/multipart/small")
@profile_once("multipart-small")
async def post_multipart_small() -> object:
    """Small multipart form (~1KB)."""
    return jsonify({"files_received": 1, "total_bytes": 1024})


@app.post("/multipart/medium")
@profile_once("multipart-medium")
async def post_multipart_medium() -> object:
    """Medium multipart form (~10KB)."""
    return jsonify({"files_received": 2, "total_bytes": 10240})


@app.post("/multipart/large")
@profile_once("multipart-large")
async def post_multipart_large() -> object:
    """Large multipart form (~100KB)."""
    return jsonify({"files_received": 5, "total_bytes": 102400})


@app.post("/urlencoded/simple")
@profile_once("urlencoded-simple")
async def post_urlencoded_simple(request: Request) -> object:
    """Simple URL-encoded form - no validation."""
    body = request.json()
    return jsonify(body)


@app.post("/urlencoded/complex")
@profile_once("urlencoded-complex")
async def post_urlencoded_complex(request: Request) -> object:
    """Complex URL-encoded form - no validation."""
    body = request.json()
    return jsonify(body)


@app.get("/path/simple/:id")
@profile_once("path-simple")
async def get_path_simple(request: Request) -> object:
    """Single path parameter."""
    return jsonify({"id": request.path_params["id"]})


@app.get("/path/multiple/:user_id/:post_id")
@profile_once("path-multiple")
async def get_path_multiple(request: Request) -> object:
    """Multiple path parameters."""
    return jsonify(
        {
            "user_id": request.path_params["user_id"],
            "post_id": request.path_params["post_id"],
        }
    )


@app.get("/path/deep/:org/:team/:project/:resource/:id")
@profile_once("path-deep")
async def get_path_deep(request: Request) -> object:
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
async def get_path_int(request: Request) -> object:
    """Path parameter with int type."""
    return jsonify({"id": int(request.path_params["id"])})


@app.get("/path/uuid/:uuid")
@profile_once("path-uuid")
async def get_path_uuid(request: Request) -> object:
    """Path parameter with UUID."""
    return jsonify({"uuid": request.path_params["uuid"]})


@app.get("/path/date/:date")
@profile_once("path-date")
async def get_path_date(request: Request) -> object:
    """Path parameter with date."""
    return jsonify({"date": request.path_params["date"]})


@app.get("/query/few")
@profile_once("query-few")
async def get_query_few(request: Request) -> object:
    """Few query parameters (1-2)."""
    return jsonify(dict(request.query_params))


@app.get("/query/medium")
@profile_once("query-medium")
async def get_query_medium(request: Request) -> object:
    """Medium query parameters (3-5)."""
    return jsonify(dict(request.query_params))


@app.get("/query/many")
@profile_once("query-many")
async def get_query_many(request: Request) -> object:
    """Many query parameters (6-10)."""
    return jsonify(dict(request.query_params))


@app.get("/health")
async def health() -> object:
    """Health check endpoint."""
    return jsonify({"status": "ok"})


@app.get("/__benchmark__/flush-profile")
async def flush_profile() -> object:
    return jsonify({"ok": True})


@app.get("/")
async def root() -> object:
    """Root endpoint."""
    return jsonify({"status": "ok"})


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(f"[robyn-raw] Starting server on port {port}", file=sys.stderr)
    app.start(host="0.0.0.0", port=port)
