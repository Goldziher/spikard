"""Spikard Python HTTP server for workload benchmarking.

This server implements all workload types to measure Python binding performance.
Includes both raw (no validation) and validated endpoints.

Raw endpoints: original paths (e.g., /json/small)
Validated endpoints: /validated/... prefix (e.g., /validated/json/small)
"""

import atexit
import json
import os
import signal
import sys
import threading
from datetime import date as DateType
from functools import wraps
from pathlib import Path as PathLib
from typing import Any, Callable, ParamSpec, TypeVar
from uuid import UUID

from spikard import Path, Query, Response, Spikard, get, post
from spikard.config import ServerConfig

_profile_dir_env = os.environ.get("SPIKARD_PYTHON_PROFILE_DIR") or None
_pyinstrument_output = os.environ.get("SPIKARD_PYINSTRUMENT_OUTPUT") or None
_profile_enabled = os.environ.get("SPIKARD_PROFILE_ENABLED") == "1" or bool(_profile_dir_env or _pyinstrument_output)

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

schema_dir = PathLib(__file__).resolve().parent.parent / "schemas"
with (schema_dir / "request_schemas.json").open("r", encoding="utf-8") as request_schema_file:
    REQUEST_SCHEMAS = json.load(request_schema_file)
with (schema_dir / "parameter_schemas.json").open("r", encoding="utf-8") as parameter_schema_file:
    PARAMETER_SCHEMAS = json.load(parameter_schema_file)
with (schema_dir / "response_schemas.json").open("r", encoding="utf-8") as response_schema_file:
    RESPONSE_SCHEMAS = json.load(response_schema_file)


def request_schema(key: str) -> dict[str, Any]:
    return REQUEST_SCHEMAS[key]


def parameter_schema(key: str) -> dict[str, Any]:
    return PARAMETER_SCHEMAS[key]


def response_schema(key: str) -> dict[str, Any]:
    return RESPONSE_SCHEMAS[key]


def _coerce_bool(value: Any) -> Any:
    if isinstance(value, bool):
        return value
    if isinstance(value, str):
        lowered = value.lower()
        if lowered in ("true", "1", "yes", "on"):
            return True
        if lowered in ("false", "0", "no", "off"):
            return False
    return value


def _coerce_int(value: Any) -> Any:
    if isinstance(value, bool):
        return value
    if isinstance(value, int):
        return value
    if isinstance(value, str):
        stripped = value.strip()
        if stripped and stripped.lstrip("-").isdigit():
            return int(stripped)
    return value


def _coerce_urlencoded_simple(body: dict[str, Any]) -> dict[str, Any]:
    coerced = dict(body)
    if "age" in coerced:
        coerced["age"] = _coerce_int(coerced["age"])
    if "subscribe" in coerced:
        coerced["subscribe"] = _coerce_bool(coerced["subscribe"])
    return coerced


def _coerce_urlencoded_complex(body: dict[str, Any]) -> dict[str, Any]:
    coerced = dict(body)
    if "age" in coerced:
        coerced["age"] = _coerce_int(coerced["age"])
    for key in (
        "subscribe",
        "newsletter",
        "terms_accepted",
        "privacy_accepted",
        "marketing_consent",
        "two_factor_enabled",
    ):
        if key in coerced:
            coerced[key] = _coerce_bool(coerced[key])
    return coerced


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


@get("/health", response_schema=response_schema("health"))
def health() -> dict[str, str]:
    return {"status": "ok"}


if _profile_enabled:

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


# ===== RAW ENDPOINTS (no validation) =====


@post("/json/small")
def post_json_small(body: dict[str, Any]) -> dict[str, Any]:
    """Small JSON payload (~100-500 bytes) - raw."""
    return body


@post("/json/medium")
def post_json_medium(body: dict[str, Any]) -> dict[str, Any]:
    """Medium JSON payload (~1-10KB) - raw."""
    return body


@post("/json/large")
def post_json_large(body: dict[str, Any]) -> dict[str, Any]:
    """Large JSON payload (~10-100KB) - raw."""
    return body


@post("/json/very-large")
def post_json_very_large(body: dict[str, Any]) -> dict[str, Any]:
    """Very large JSON payload (~100KB-1MB) - raw."""
    return body


@post("/multipart/small")
def post_multipart_small(body: dict[str, Any]) -> dict[str, Any]:
    """Small multipart form (~1KB) - raw."""
    files = body.get("files", {})
    files_received = 0
    total_bytes = 0

    for key, file_data in files.items():
        if key.startswith("file") and isinstance(file_data, dict):
            files_received += 1
            total_bytes += file_data.get("size", 0)

    return {"files_received": files_received, "total_bytes": total_bytes}


@post("/multipart/medium")
def post_multipart_medium(body: dict[str, Any]) -> dict[str, Any]:
    """Medium multipart form (~10KB) - raw."""
    files = body.get("files", {})
    files_received = 0
    total_bytes = 0

    for key, file_data in files.items():
        if key.startswith("file") and isinstance(file_data, dict):
            files_received += 1
            total_bytes += file_data.get("size", 0)

    return {"files_received": files_received, "total_bytes": total_bytes}


@post("/multipart/large")
def post_multipart_large(body: dict[str, Any]) -> dict[str, Any]:
    """Large multipart form (~100KB) - raw."""
    files = body.get("files", {})
    files_received = 0
    total_bytes = 0

    for key, file_data in files.items():
        if key.startswith("file") and isinstance(file_data, dict):
            files_received += 1
            total_bytes += file_data.get("size", 0)

    return {"files_received": files_received, "total_bytes": total_bytes}


@post("/urlencoded/simple")
def post_urlencoded_simple(body: dict[str, Any]) -> dict[str, Any]:
    """Simple URL-encoded form (3-5 fields) - raw."""
    return body


@post("/urlencoded/complex")
def post_urlencoded_complex(body: dict[str, Any]) -> dict[str, Any]:
    """Complex URL-encoded form (10-20 fields) - raw."""
    return body


@get("/path/simple/{id}")
def get_path_simple(id: str = Path()) -> dict[str, Any]:
    """Single path parameter - raw."""
    return {"id": id}


@get("/path/multiple/{user_id}/{post_id}")
def get_path_multiple(user_id: str = Path(), post_id: str = Path()) -> dict[str, Any]:
    """Multiple path parameters - raw."""
    return {"user_id": user_id, "post_id": post_id}


@get("/path/deep/{org}/{team}/{project}/{resource}/{id}")
def get_path_deep(
    org: str = Path(),
    team: str = Path(),
    project: str = Path(),
    resource: str = Path(),
    id: str = Path(),
) -> dict[str, Any]:
    """Deep path parameters (5 levels) - raw."""
    return {
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id,
    }


@get("/path/int/{id}")
def get_path_int(id: int = Path()) -> dict[str, Any]:
    """Integer path parameter - raw."""
    return {"id": id}


@get("/path/uuid/{uuid}")
def get_path_uuid(uuid: UUID = Path()) -> dict[str, Any]:
    """UUID path parameter - raw."""
    return {"uuid": str(uuid)}


@get("/path/date/{date}")
def get_path_date(date: DateType = Path()) -> dict[str, Any]:
    """Date path parameter - raw."""
    return {"date": date.isoformat()}


@get("/query/few")
def get_query_few(
    q: str | None = Query(default=None),
    page: int | None = Query(default=None),
    limit: int | None = Query(default=None),
) -> dict[str, Any]:
    """Few query parameters (1-3) - raw."""
    result = {}
    if q is not None:
        result["q"] = q
    if page is not None:
        result["page"] = page
    if limit is not None:
        result["limit"] = limit
    return result


@get("/query/medium")
def get_query_medium(
    search: str | None = Query(default=None),
    category: str | None = Query(default=None),
    sort: str | None = Query(default=None),
    order: str | None = Query(default=None),
    page: int | None = Query(default=None),
    limit: int | None = Query(default=None),
    filter: str | None = Query(default=None),
) -> dict[str, Any]:
    """Medium number of query parameters (5-10) - raw."""
    result = {}
    if search is not None:
        result["search"] = search
    if category is not None:
        result["category"] = category
    if sort is not None:
        result["sort"] = sort
    if order is not None:
        result["order"] = order
    if page is not None:
        result["page"] = page
    if limit is not None:
        result["limit"] = limit
    if filter is not None:
        result["filter"] = filter
    return result


@get("/query/many")
def get_query_many(
    q: str | None = Query(default=None),
    category: str | None = Query(default=None),
    subcategory: str | None = Query(default=None),
    brand: str | None = Query(default=None),
    min_price: float | None = Query(default=None),
    max_price: float | None = Query(default=None),
    color: str | None = Query(default=None),
    size: str | None = Query(default=None),
    material: str | None = Query(default=None),
    rating: int | None = Query(default=None),
    sort: str | None = Query(default=None),
    order: str | None = Query(default=None),
    page: int | None = Query(default=None),
    limit: int | None = Query(default=None),
    in_stock: bool | None = Query(default=None),
    on_sale: bool | None = Query(default=None),
) -> dict[str, Any]:
    """Many query parameters (15+) - raw."""
    result = {}
    if q is not None:
        result["q"] = q
    if category is not None:
        result["category"] = category
    if subcategory is not None:
        result["subcategory"] = subcategory
    if brand is not None:
        result["brand"] = brand
    if min_price is not None:
        result["min_price"] = min_price
    if max_price is not None:
        result["max_price"] = max_price
    if color is not None:
        result["color"] = color
    if size is not None:
        result["size"] = size
    if material is not None:
        result["material"] = material
    if rating is not None:
        result["rating"] = rating
    if sort is not None:
        result["sort"] = sort
    if order is not None:
        result["order"] = order
    if page is not None:
        result["page"] = page
    if limit is not None:
        result["limit"] = limit
    if in_stock is not None:
        result["in_stock"] = in_stock
    if on_sale is not None:
        result["on_sale"] = on_sale
    return result


# ===== VALIDATED ENDPOINTS =====


@post("/validated/json/small", body_schema=request_schema("json/small"), response_schema=response_schema("json/small"))
@profile_once("validated-json-small")
def post_validated_json_small(body: dict[str, Any]) -> dict[str, Any]:
    """Small JSON payload (~100-500 bytes) - validated."""
    return body


@post(
    "/validated/json/medium", body_schema=request_schema("json/medium"), response_schema=response_schema("json/medium")
)
@profile_once("validated-json-medium")
def post_validated_json_medium(body: dict[str, Any]) -> dict[str, Any]:
    """Medium JSON payload (nested object) - validated."""
    return body


@post("/validated/json/large", body_schema=request_schema("json/large"), response_schema=response_schema("json/large"))
@profile_once("validated-json-large")
def post_validated_json_large(body: dict[str, Any]) -> dict[str, Any]:
    """Large JSON payload (~10-100KB) - validated."""
    return body


@post(
    "/validated/json/very-large",
    body_schema=request_schema("json/very-large"),
    response_schema=response_schema("json/very-large"),
)
@profile_once("validated-json-very-large")
def post_validated_json_very_large(body: dict[str, Any]) -> dict[str, Any]:
    """Very large JSON payload (arrays of values and objects) - validated."""
    return body


@post(
    "/validated/multipart/small",
    body_schema=request_schema("multipart/small"),
    response_schema=response_schema("multipart/small"),
)
@profile_once("validated-multipart-small")
def post_validated_multipart_small(body: dict[str, Any]) -> dict[str, int]:
    """Small multipart form (~1KB) - validated."""
    files = body.get("files", {})
    files_received = 0
    total_bytes = 0

    for key, file_data in files.items():
        if key.startswith("file") and isinstance(file_data, dict):
            files_received += 1
            total_bytes += file_data.get("size", 0)

    if files_received == 0:
        return Response(content={"error": "No files received"}, status_code=400)

    return {"files_received": files_received, "total_bytes": total_bytes}


@post(
    "/validated/multipart/medium",
    body_schema=request_schema("multipart/medium"),
    response_schema=response_schema("multipart/medium"),
)
@profile_once("validated-multipart-medium")
def post_validated_multipart_medium(body: dict[str, Any]) -> dict[str, int]:
    """Medium multipart form (~10KB) - validated."""
    files = body.get("files", {})
    files_received = 0
    total_bytes = 0

    for key, file_data in files.items():
        if key.startswith("file") and isinstance(file_data, dict):
            files_received += 1
            total_bytes += file_data.get("size", 0)

    if files_received == 0:
        return Response(content={"error": "No files received"}, status_code=400)

    return {"files_received": files_received, "total_bytes": total_bytes}


@post(
    "/validated/multipart/large",
    body_schema=request_schema("multipart/large"),
    response_schema=response_schema("multipart/large"),
)
@profile_once("validated-multipart-large")
def post_validated_multipart_large(body: dict[str, Any]) -> dict[str, int]:
    """Large multipart form (~100KB) - validated."""
    files = body.get("files", {})
    files_received = 0
    total_bytes = 0

    for key, file_data in files.items():
        if key.startswith("file") and isinstance(file_data, dict):
            files_received += 1
            total_bytes += file_data.get("size", 0)

    if files_received == 0:
        return Response(content={"error": "No files received"}, status_code=400)

    return {"files_received": files_received, "total_bytes": total_bytes}


@post(
    "/validated/urlencoded/simple",
    body_schema=request_schema("urlencoded/simple"),
    response_schema=response_schema("urlencoded/simple"),
)
@profile_once("validated-urlencoded-simple")
def post_validated_urlencoded_simple(body: dict[str, Any]) -> dict[str, Any]:
    """Simple URL-encoded form (3-5 fields) - validated."""
    return _coerce_urlencoded_simple(body)


@post(
    "/validated/urlencoded/complex",
    body_schema=request_schema("urlencoded/complex"),
    response_schema=response_schema("urlencoded/complex"),
)
@profile_once("validated-urlencoded-complex")
def post_validated_urlencoded_complex(body: dict[str, Any]) -> dict[str, Any]:
    """Complex URL-encoded form (10-20 fields) - validated."""
    return _coerce_urlencoded_complex(body)


@get(
    "/validated/path/simple/{id}",
    response_schema=response_schema("path/simple"),
    parameter_schema=parameter_schema("path/simple"),
)
@profile_once("validated-path-simple")
def get_validated_path_simple(id: str = Path()) -> dict[str, JsonScalar]:
    """Single path parameter - validated."""
    return {"id": id}


@get(
    "/validated/path/multiple/{user_id}/{post_id}",
    response_schema=response_schema("path/multiple"),
    parameter_schema=parameter_schema("path/multiple"),
)
@profile_once("validated-path-multiple")
def get_validated_path_multiple(user_id: str = Path(), post_id: str = Path()) -> dict[str, JsonScalar]:
    """Multiple path parameters - validated."""
    return {"user_id": user_id, "post_id": post_id}


@get(
    "/validated/path/deep/{org}/{team}/{project}/{resource}/{id}",
    response_schema=response_schema("path/deep"),
    parameter_schema=parameter_schema("path/deep"),
)
@profile_once("validated-path-deep")
def get_validated_path_deep(
    org: str = Path(),
    team: str = Path(),
    project: str = Path(),
    resource: str = Path(),
    id: str = Path(),
) -> dict[str, JsonScalar]:
    """Deep path parameters (5 levels) - validated."""
    return {
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id,
    }


@get(
    "/validated/path/int/{id}",
    response_schema=response_schema("path/int"),
    parameter_schema=parameter_schema("path/int"),
)
@profile_once("validated-path-int")
def get_validated_path_int(id: int = Path()) -> dict[str, JsonScalar]:
    """Integer path parameter - validated."""
    return {"id": id}


@get(
    "/validated/path/uuid/{uuid}",
    response_schema=response_schema("path/uuid"),
    parameter_schema=parameter_schema("path/uuid"),
)
@profile_once("validated-path-uuid")
def get_validated_path_uuid(uuid: UUID = Path()) -> dict[str, JsonScalar]:
    """UUID path parameter - validated."""
    return {"uuid": str(uuid)}


@get(
    "/validated/path/date/{date}",
    response_schema=response_schema("path/date"),
    parameter_schema=parameter_schema("path/date"),
)
@profile_once("validated-path-date")
def get_validated_path_date(date: DateType = Path()) -> dict[str, JsonScalar]:
    """Date path parameter - validated."""
    return {"date": date.isoformat()}


@get(
    "/validated/query/few", response_schema=response_schema("query/few"), parameter_schema=parameter_schema("query/few")
)
@profile_once("validated-query-few")
def get_validated_query_few(
    q: str = Query(),
    page: int | None = Query(default=None),
    limit: int | None = Query(default=None),
) -> dict[str, JsonScalar]:
    """Few query parameters (1-3) - validated."""
    result: dict[str, JsonScalar] = {"q": q}
    if page is not None:
        result["page"] = page
    if limit is not None:
        result["limit"] = limit
    return result


@get(
    "/validated/query/medium",
    response_schema=response_schema("query/medium"),
    parameter_schema=parameter_schema("query/medium"),
)
@profile_once("validated-query-medium")
def get_validated_query_medium(
    search: str = Query(),
    category: str | None = Query(default=None),
    sort: str | None = Query(default=None),
    order: str | None = Query(default=None),
    page: int | None = Query(default=None),
    limit: int | None = Query(default=None),
    filter: str | None = Query(default=None),
) -> dict[str, JsonScalar]:
    """Medium number of query parameters (5-10) - validated."""
    result: dict[str, JsonScalar] = {"search": search}
    if category is not None:
        result["category"] = category
    if sort is not None:
        result["sort"] = sort
    if order is not None:
        result["order"] = order
    if page is not None:
        result["page"] = page
    if limit is not None:
        result["limit"] = limit
    if filter is not None:
        result["filter"] = filter
    return result


@get(
    "/validated/query/many",
    response_schema=response_schema("query/many"),
    parameter_schema=parameter_schema("query/many"),
)
@profile_once("validated-query-many")
def get_validated_query_many(
    q: str = Query(),
    category: str | None = Query(default=None),
    subcategory: str | None = Query(default=None),
    brand: str | None = Query(default=None),
    min_price: float | None = Query(default=None),
    max_price: float | None = Query(default=None),
    color: str | None = Query(default=None),
    size: str | None = Query(default=None),
    material: str | None = Query(default=None),
    rating: int | None = Query(default=None),
    sort: str | None = Query(default=None),
    order: str | None = Query(default=None),
    page: int | None = Query(default=None),
    limit: int | None = Query(default=None),
    in_stock: bool | None = Query(default=None),
    on_sale: bool | None = Query(default=None),
) -> dict[str, JsonScalar]:
    """Many query parameters (15+) - validated."""
    result: dict[str, JsonScalar] = {"q": q}
    if category is not None:
        result["category"] = category
    if subcategory is not None:
        result["subcategory"] = subcategory
    if brand is not None:
        result["brand"] = brand
    if min_price is not None:
        result["min_price"] = min_price
    if max_price is not None:
        result["max_price"] = max_price
    if color is not None:
        result["color"] = color
    if size is not None:
        result["size"] = size
    if material is not None:
        result["material"] = material
    if rating is not None:
        result["rating"] = rating
    if sort is not None:
        result["sort"] = sort
    if order is not None:
        result["order"] = order
    if page is not None:
        result["page"] = page
    if limit is not None:
        result["limit"] = limit
    if in_stock is not None:
        result["in_stock"] = in_stock
    if on_sale is not None:
        result["on_sale"] = on_sale
    return result


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(
        f"Spikard Python workload server (raw + validated) starting on port {port}",
        file=sys.stderr,
        flush=True,
    )

    config = ServerConfig(
        host="0.0.0.0",
        port=port,
        workers=1,
    )

    app.run(config=config)
