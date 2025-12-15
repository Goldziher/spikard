"""Spikard Python HTTP server for workload benchmarking.

This server implements all workload types to measure Python binding performance
against the pure Rust baseline.

Uses msgspec.Struct for proper validation following ADR 0003.
"""

import os
import sys
from pathlib import Path as PathLib
from typing import Any

import msgspec

from spikard import Body, Path, Query, Spikard, get, post
from spikard.config import ServerConfig

profiling_module = PathLib(__file__).parent.parent.parent / "profiling" / "python_metrics.py"
_profiling_collector = None
if profiling_module.exists():
    sys.path.insert(0, str(profiling_module.parent))
    try:
        import python_metrics

        _profiling_collector = python_metrics.enable_profiling()
    except ImportError:
        print("⚠ Failed to import profiling module", file=sys.stderr)


app = Spikard()


class SmallPayload(msgspec.Struct):
    """Small JSON payload - matches 01_simple_object_success.json."""

    name: str
    description: str
    price: float
    tax: float | None = None


class Seller(msgspec.Struct):
    """Seller information for nested payload."""

    name: str
    email: str | None = None


class MediumPayload(msgspec.Struct):
    """Medium JSON payload - matches 04_nested_object_success.json."""

    name: str
    description: str
    price: float
    seller: Seller


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


class Tag(msgspec.Struct):
    """Tag object for array of objects."""

    name: str


class VeryLargePayload(msgspec.Struct):
    """Very large JSON payload - matches 05_array_of_objects.json."""

    name: str
    description: str
    price: float
    tags: list[Tag]


@post("/json/small")
async def post_json_small(body: SmallPayload) -> SmallPayload:
    """Small JSON payload (~100-500 bytes)."""
    return body


@post("/json/medium")
async def post_json_medium(body: MediumPayload) -> MediumPayload:
    """Medium JSON payload (~1-10KB)."""
    return body


@post("/json/large")
async def post_json_large(body: LargePayload) -> LargePayload:
    """Large JSON payload (~10-100KB)."""
    return body


@post("/json/very-large")
async def post_json_very_large(body: VeryLargePayload) -> VeryLargePayload:
    """Very large JSON payload (~100KB-1MB)."""
    return body


@post("/multipart/small")
async def post_multipart_small(body: dict[str, Any]) -> dict[str, Any]:
    """Small multipart form (~1KB)."""
    return {"files_received": 1, "total_bytes": 1024}


@post("/multipart/medium")
async def post_multipart_medium(body: dict[str, Any]) -> dict[str, Any]:
    """Medium multipart form (~10KB)."""
    return {"files_received": 2, "total_bytes": 10240}


@post("/multipart/large")
async def post_multipart_large(body: dict[str, Any]) -> dict[str, Any]:
    """Large multipart form (~100KB)."""
    return {"files_received": 5, "total_bytes": 102400}


@post("/urlencoded/simple")
async def post_urlencoded_simple(body: dict[str, Any]) -> dict[str, Any]:
    """Simple URL-encoded form (3-5 fields)."""
    return body


@post("/urlencoded/complex")
async def post_urlencoded_complex(body: dict[str, Any]) -> dict[str, Any]:
    """Complex URL-encoded form (10-20 fields)."""
    return body


@get("/path/simple/{id}")
async def get_path_simple(id: str = Path()) -> dict[str, Any]:
    """Single path parameter."""
    return {"id": id}


@get("/path/multiple/{user_id}/{post_id}")
async def get_path_multiple(user_id: str = Path(), post_id: str = Path()) -> dict[str, Any]:
    """Multiple path parameters."""
    return {"user_id": user_id, "post_id": post_id}


@get("/path/deep/{org}/{team}/{project}/{resource}/{id}")
async def get_path_deep(
    org: str = Path(),
    team: str = Path(),
    project: str = Path(),
    resource: str = Path(),
    id: str = Path(),
) -> dict[str, Any]:
    """Deep path parameters (5 levels)."""
    return {
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id,
    }


@get("/path/int/{id}")
async def get_path_int(id: int = Path()) -> dict[str, Any]:
    """Integer path parameter."""
    return {"id": id}


@get("/path/uuid/{uuid}")
async def get_path_uuid(uuid: str = Path()) -> dict[str, Any]:
    """UUID path parameter."""
    return {"uuid": uuid}


@get("/path/date/{date}")
async def get_path_date(date: str = Path()) -> dict[str, Any]:
    """Date path parameter."""
    return {"date": date}


@get("/query/few")
async def get_query_few(
    q: str | None = Query(default=None),
    page: int | None = Query(default=None),
    limit: int | None = Query(default=None),
) -> dict[str, Any]:
    """Few query parameters (1-3)."""
    return {"q": q, "page": page, "limit": limit}


@get("/query/medium")
async def get_query_medium(
    category: str | None = Query(default=None),
    tags: str | None = Query(default=None),
    min_price: float | None = Query(default=None),
    max_price: float | None = Query(default=None),
    sort: str | None = Query(default=None),
    order: str | None = Query(default=None),
    page: int | None = Query(default=None),
    limit: int | None = Query(default=None),
) -> dict[str, Any]:
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
) -> dict[str, Any]:
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


@get("/health")
async def health() -> dict[str, str]:
    """Health check endpoint."""
    return {"status": "ok"}


@get("/__internal/flush-profile")
async def flush_profile() -> dict[str, str]:
    """Flush profiling/metrics outputs to disk (best-effort)."""
    if _profiling_collector is not None:
        _profiling_collector.finalize()
        return {"status": "flushed"}
    return {"status": "no-profiler"}


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
