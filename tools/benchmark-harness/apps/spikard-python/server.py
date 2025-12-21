"""Spikard Python HTTP server for workload benchmarking.

This server implements all workload types to measure Python binding performance
against the pure Rust baseline.

Uses msgspec.Struct for proper validation following ADR 0003.
"""

from __future__ import annotations

import atexit
import os
import signal
import sys
import threading
from functools import wraps
from pathlib import Path as PathLib
from typing import Callable, Coroutine, ParamSpec, TypeVar

import msgspec

from spikard import Path, Query, Spikard, get, post
from spikard.config import ServerConfig

_profile_dir_env = os.environ.get("SPIKARD_PYTHON_PROFILE_DIR") or None
_pyinstrument_output = os.environ.get("SPIKARD_PYINSTRUMENT_OUTPUT") or None
_profile_enabled = os.environ.get("SPIKARD_PROFILE_ENABLED") == "1" or bool(
    _profile_dir_env or _pyinstrument_output
)

profiling_module = PathLib(__file__).parent.parent.parent / "profiling" / "python_metrics.py"
_profiling_collector: object | None = None
if _profile_enabled and profiling_module.exists():
    sys.path.insert(0, str(profiling_module.parent))
    try:
        import python_metrics

        _profiling_collector = python_metrics.enable_profiling()
    except ImportError:
        print("⚠ Failed to import profiling module", file=sys.stderr)

if _profile_enabled:
    from pyinstrument import Profiler
    from pyinstrument.renderers.speedscope import SpeedscopeRenderer
else:
    Profiler = object  # type: ignore[assignment]
    SpeedscopeRenderer = object  # type: ignore[assignment]

_profile_dir: str | None = _profile_dir_env if _profile_enabled else None
_profiled_endpoints: set[str] = set()
_profile_lock = threading.Lock()
_profile_state: dict[str, tuple[int, int, Profiler]] = {}
_profile_target_requests = int(os.environ.get("SPIKARD_PYINSTRUMENT_REQUESTS") or "200")

app = Spikard()

JsonScalar = str | int | float | bool | None

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


def _start_pyinstrument(output_path: str) -> bool:
    global _pyinstrument_dumped, _pyinstrument_output, _pyinstrument_profiler

    if not output_path:
        return False

    _pyinstrument_output = output_path
    _pyinstrument_dumped = False

    if _pyinstrument_profiler is not None:
        try:
            _pyinstrument_profiler.stop()
        except Exception:
            pass

    _pyinstrument_profiler = Profiler(async_mode="enabled")
    _pyinstrument_profiler.start()
    return True


def _stop_pyinstrument() -> bool:
    global _pyinstrument_profiler

    if _pyinstrument_profiler is None:
        return False

    _dump_profile()
    _pyinstrument_profiler = None
    return True


if _profile_enabled and _pyinstrument_output:
    _pyinstrument_profiler = Profiler(async_mode="enabled")
    _pyinstrument_profiler.start()
    atexit.register(_dump_profile)
    try:
        if hasattr(signal, "SIGUSR2"):
            signal.signal(signal.SIGUSR2, lambda *_args: _dump_profile())
    except Exception:
        pass


@get("/health")
def health() -> dict[str, str]:
    return {"status": "ok"}


@get("/__benchmark__/flush-profile")
def flush_profile() -> dict[str, bool]:
    _stop_pyinstrument()
    if _profiling_collector is not None:
        finalize = getattr(_profiling_collector, "finalize", None)
        if callable(finalize):
            finalize()
        return {"ok": True}
    return {"ok": False}


@get("/__benchmark__/profile/start")
async def start_profile(output: str | None = Query(default=None)) -> dict[str, bool]:
    if output is None:
        return {"ok": False}
    return {"ok": _start_pyinstrument(output)}


@get("/__benchmark__/profile/stop")
async def stop_profile() -> dict[str, bool]:
    return {"ok": _stop_pyinstrument()}


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
            thread_id = threading.get_ident()

            with _profile_lock:
                if name in _profiled_endpoints:
                    return await func(*args, **kwargs)

                state = _profile_state.get(name)
                if state is None:
                    profiler = Profiler(async_mode="enabled", interval=0.00001)
                    profiler.start()
                    _profile_state[name] = (thread_id, 0, profiler)
                    state = _profile_state[name]

                owner_thread_id, count, profiler = state
                if owner_thread_id != thread_id:
                    profiler = None

            try:
                return await func(*args, **kwargs)
            finally:
                if profiler is not None:
                    dump_profiler: Profiler | None = None
                    with _profile_lock:
                        state = _profile_state.get(name)
                        if state is not None:
                            owner_thread_id, count, active_profiler = state
                            if owner_thread_id == thread_id:
                                count += 1
                                if count >= _profile_target_requests:
                                    _profile_state.pop(name, None)
                                    _profiled_endpoints.add(name)
                                    dump_profiler = active_profiler
                                else:
                                    _profile_state[name] = (owner_thread_id, count, active_profiler)

                    if dump_profiler is not None:
                        try:
                            dump_profiler.stop()
                            out_dir = PathLib(_profile_dir)
                            out_dir.mkdir(parents=True, exist_ok=True)
                            out_path = out_dir / f"{name}.speedscope.json"
                            out_path.write_text(
                                dump_profiler.output(renderer=SpeedscopeRenderer()),
                                encoding="utf-8",
                            )
                        except Exception as exc:
                            print(f"⚠ Failed to write profile for {name}: {exc!r}", file=sys.stderr)

        return wrapper

    return decorator


class SmallPayload(msgspec.Struct):
    """Small JSON payload - matches 01_simple_object_success.json."""

    name: str
    description: str
    price: float
    tax: float | None = None


class MediumPayload(msgspec.Struct):
    """Medium JSON payload - matches 04_nested_object_success.json."""

    name: str
    price: float
    image: "Image"


class Image(msgspec.Struct):
    """Image data for nested structures."""

    url: str
    name: str


class Country(msgspec.Struct):
    """Country for deeply nested structures."""

    name: str
    code: str


class Address(msgspec.Struct):
    """Address for deeply nested structures."""

    street: str
    city: str
    country: Country


class SellerWithAddress(msgspec.Struct):
    """Seller with address for deeply nested payload."""

    name: str
    address: Address


class LargePayload(msgspec.Struct):
    """Large JSON payload - matches 25_deeply_nested_objects.json."""

    name: str
    price: float
    seller: SellerWithAddress


class VeryLargePayload(msgspec.Struct):
    """Very large JSON payload - matches 05_array_of_objects.json."""

    name: str
    tags: list[str]
    images: list[Image]


class MultipartMetrics(msgspec.Struct):
    files_received: int
    total_bytes: int


class MultipartFile(msgspec.Struct):
    filename: str
    size: int
    content: str
    content_type: str


class MultipartBody(msgspec.Struct):
    file: MultipartFile | list[MultipartFile] | None = None


@post("/json/small")
@profile_once("json-small")
async def post_json_small(body: SmallPayload) -> SmallPayload:
    """Small JSON payload (~100-500 bytes)."""
    return body


@post("/json/medium")
@profile_once("json-medium")
async def post_json_medium(body: MediumPayload) -> MediumPayload:
    """Medium JSON payload (nested object)."""
    return body


@post("/json/large")
@profile_once("json-large")
async def post_json_large(body: LargePayload) -> LargePayload:
    """Large JSON payload (~10-100KB)."""
    return body


@post("/json/very-large")
@profile_once("json-very-large")
async def post_json_very_large(body: VeryLargePayload) -> VeryLargePayload:
    """Very large JSON payload (arrays of values and objects)."""
    return body


def _multipart_body_metrics(body: MultipartBody) -> MultipartMetrics:
    file_value = body.file
    if isinstance(file_value, list):
        total = sum(item.size for item in file_value)
        return MultipartMetrics(files_received=len(file_value), total_bytes=total)
    if isinstance(file_value, MultipartFile):
        return MultipartMetrics(files_received=1, total_bytes=file_value.size)

    return MultipartMetrics(files_received=0, total_bytes=0)


@post("/multipart/small")
@profile_once("multipart-small")
async def post_multipart_small(body: MultipartBody) -> MultipartMetrics:
    """Small multipart form (~1KB)."""
    return _multipart_body_metrics(body)


@post("/multipart/medium")
@profile_once("multipart-medium")
async def post_multipart_medium(body: MultipartBody) -> MultipartMetrics:
    """Medium multipart form (~10KB)."""
    return _multipart_body_metrics(body)


@post("/multipart/large")
@profile_once("multipart-large")
async def post_multipart_large(body: MultipartBody) -> MultipartMetrics:
    """Large multipart form (~100KB)."""
    return _multipart_body_metrics(body)


@post("/urlencoded/simple")
@profile_once("urlencoded-simple")
async def post_urlencoded_simple(body: str) -> str:
    """Simple URL-encoded form (3-5 fields)."""
    return body


@post("/urlencoded/complex")
@profile_once("urlencoded-complex")
async def post_urlencoded_complex(body: str) -> str:
    """Complex URL-encoded form (10-20 fields)."""
    return body


@get("/path/simple/{id}")
@profile_once("path-simple")
async def get_path_simple(id: str = Path()) -> dict[str, JsonScalar]:
    """Single path parameter."""
    return {"id": id}


@get("/path/multiple/{user_id}/{post_id}")
@profile_once("path-multiple")
async def get_path_multiple(user_id: str = Path(), post_id: str = Path()) -> dict[str, JsonScalar]:
    """Multiple path parameters."""
    return {"user_id": user_id, "post_id": post_id}


@get("/path/deep/{org}/{team}/{project}/{resource}/{id}")
@profile_once("path-deep")
async def get_path_deep(
    org: str = Path(),
    team: str = Path(),
    project: str = Path(),
    resource: str = Path(),
    id: str = Path(),
) -> dict[str, JsonScalar]:
    """Deep path parameters (5 levels)."""
    return {
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id,
    }


@get("/path/int/{id}")
@profile_once("path-int")
async def get_path_int(id: int = Path()) -> dict[str, JsonScalar]:
    """Integer path parameter."""
    return {"id": id}


@get("/path/uuid/{uuid}")
@profile_once("path-uuid")
async def get_path_uuid(uuid: str = Path()) -> dict[str, JsonScalar]:
    """UUID path parameter."""
    return {"uuid": uuid}


@get("/path/date/{date}")
@profile_once("path-date")
async def get_path_date(date: str = Path()) -> dict[str, JsonScalar]:
    """Date path parameter."""
    return {"date": date}


@get("/query/few")
@profile_once("query-few")
async def get_query_few(
    q: str | None = Query(default=None),
    page: int | None = Query(default=None),
    limit: int | None = Query(default=None),
) -> dict[str, JsonScalar]:
    """Few query parameters (1-3)."""
    return {"q": q, "page": page, "limit": limit}


@get("/query/medium")
@profile_once("query-medium")
async def get_query_medium(
    category: str | None = Query(default=None),
    tags: str | None = Query(default=None),
    min_price: float | None = Query(default=None),
    max_price: float | None = Query(default=None),
    sort: str | None = Query(default=None),
    order: str | None = Query(default=None),
    page: int | None = Query(default=None),
    limit: int | None = Query(default=None),
) -> dict[str, JsonScalar]:
    """Medium number of query parameters (5-10)."""
    return {
        "category": category,
        "tags": tags,
        "min_price": min_price,
        "max_price": max_price,
        "sort": sort,
        "order": order,
        "page": page,
        "limit": limit,
    }


@get("/query/many")
@profile_once("query-many")
async def get_query_many(
    param1: str | None = Query(default=None),
    param2: str | None = Query(default=None),
    param3: str | None = Query(default=None),
    param4: str | None = Query(default=None),
    param5: str | None = Query(default=None),
    param6: str | None = Query(default=None),
    param7: str | None = Query(default=None),
    param8: str | None = Query(default=None),
    param9: str | None = Query(default=None),
    param10: str | None = Query(default=None),
    param11: str | None = Query(default=None),
    param12: str | None = Query(default=None),
    param13: str | None = Query(default=None),
    param14: str | None = Query(default=None),
    param15: str | None = Query(default=None),
) -> dict[str, JsonScalar]:
    """Many query parameters (15+)."""
    return {
        "param1": param1,
        "param2": param2,
        "param3": param3,
        "param4": param4,
        "param5": param5,
        "param6": param6,
        "param7": param7,
        "param8": param8,
        "param9": param9,
        "param10": param10,
        "param11": param11,
        "param12": param12,
        "param13": param13,
        "param14": param14,
        "param15": param15,
    }


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(
        f"Spikard Python workload server starting on port {port}",
        file=sys.stderr,
        flush=True,
    )

    config = ServerConfig(
        host="0.0.0.0",
        port=port,
        workers=1,
    )

    app.run(config=config)
