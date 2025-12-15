"""Robyn benchmark server for workload comparison (raw/no validation).

Robyn is a Rust-based Python web framework with high performance.
This is a raw variant without pydantic validation to measure framework overhead.
"""

import os
import sys
import signal
from pathlib import Path as PathLib
from typing import Any

from pyinstrument import Profiler
from pyinstrument.renderers.speedscope import SpeedscopeRenderer
from robyn import Robyn, Request, jsonify

app = Robyn(__file__)

_pyinstrument_profiler: Profiler | None = None
_pyinstrument_output: str | None = os.environ.get("SPIKARD_PYTHON_PROFILE_OUTPUT")
_pyinstrument_dumped = False
if _pyinstrument_output:
    _pyinstrument_profiler = Profiler()
    _pyinstrument_profiler.start()


def _dump_profile_from_signal(_signum: int, _frame: Any) -> None:  # noqa: ANN401
    global _pyinstrument_dumped
    if _pyinstrument_dumped or _pyinstrument_profiler is None or _pyinstrument_output is None:
        return

    _pyinstrument_dumped = True
    try:
        _pyinstrument_profiler.stop()
        out_path = PathLib(_pyinstrument_output)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(_pyinstrument_profiler.output(renderer=SpeedscopeRenderer()), encoding="utf-8")
    except Exception:
        pass


if _pyinstrument_output:
    signal.signal(signal.SIGUSR1, _dump_profile_from_signal)


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
    body = request.json()
    return jsonify(body)


@app.post("/urlencoded/complex")
async def post_urlencoded_complex(request: Request):
    """Complex URL-encoded form - no validation."""
    body = request.json()
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
    return jsonify(dict(request.query_params))


@app.get("/query/medium")
async def get_query_medium(request: Request):
    """Medium query parameters (3-5)."""
    return jsonify(dict(request.query_params))


@app.get("/query/many")
async def get_query_many(request: Request):
    """Many query parameters (6-10)."""
    return jsonify(dict(request.query_params))


@app.get("/health")
async def health():
    """Health check endpoint."""
    return jsonify({"status": "ok"})


@app.get("/__benchmark__/flush-profile")
async def flush_profile():
    if _pyinstrument_profiler is None or _pyinstrument_output is None:
        return jsonify({"ok": False})

    os.kill(os.getpid(), signal.SIGUSR1)
    return jsonify({"ok": True})


@app.get("/")
async def root():
    """Root endpoint."""
    return jsonify({"status": "ok"})


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(f"[robyn-raw] Starting server on port {port}", file=sys.stderr)
    app.start(host="0.0.0.0", port=port)
