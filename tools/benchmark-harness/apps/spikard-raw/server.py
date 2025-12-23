"""Spikard Python HTTP server for workload benchmarking (raw/no validation).

This server implements all workload types to measure Python binding performance
WITHOUT validation overhead - all handlers use raw dict[str, Any] instead of
msgspec types.

This allows benchmarking Spikard's raw performance separate from validation overhead.
"""

import os
import sys
from pathlib import Path as PathLib
from typing import Any

from spikard import Body, Path, Query, Spikard, get, post
from spikard.config import ServerConfig

_profile_enabled = os.environ.get("SPIKARD_PROFILE_ENABLED") == "1"

profiling_module = PathLib(__file__).parent.parent.parent / "profiling" / "python_metrics.py"
_profiling_collector = None
if _profile_enabled and profiling_module.exists():
    sys.path.insert(0, str(profiling_module.parent))
    try:
        import python_metrics

        _profiling_collector = python_metrics.enable_profiling()
    except ImportError:
        print("⚠ Failed to import profiling module", file=sys.stderr)


app = Spikard()

SMALL_PAYLOAD_SCHEMA = {
    "type": "object",
    "required": ["name", "description", "price", "tax"],
    "properties": {
        "name": {"type": "string"},
        "description": {"type": "string"},
        "price": {"type": "number"},
        "tax": {"type": "number"},
    },
    "additionalProperties": False,
}

MEDIUM_PAYLOAD_SCHEMA = {
    "type": "object",
    "required": ["name", "price", "image"],
    "properties": {
        "name": {"type": "string"},
        "price": {"type": "number"},
        "image": {
            "type": "object",
            "required": ["url", "name"],
            "properties": {
                "url": {"type": "string"},
                "name": {"type": "string"},
            },
            "additionalProperties": False,
        },
    },
    "additionalProperties": False,
}

LARGE_PAYLOAD_SCHEMA = {
    "type": "object",
    "required": ["name", "price", "seller"],
    "properties": {
        "name": {"type": "string"},
        "price": {"type": "number"},
        "seller": {
            "type": "object",
            "required": ["name", "address"],
            "properties": {
                "name": {"type": "string"},
                "address": {
                    "type": "object",
                    "required": ["street", "city", "country"],
                    "properties": {
                        "street": {"type": "string"},
                        "city": {"type": "string"},
                        "country": {
                            "type": "object",
                            "required": ["name", "code"],
                            "properties": {
                                "name": {"type": "string"},
                                "code": {"type": "string"},
                            },
                            "additionalProperties": False,
                        },
                    },
                    "additionalProperties": False,
                },
            },
            "additionalProperties": False,
        },
    },
    "additionalProperties": False,
}

VERY_LARGE_PAYLOAD_SCHEMA = {
    "type": "object",
    "required": ["name", "tags", "images"],
    "properties": {
        "name": {"type": "string"},
        "tags": {"type": "array", "items": {"type": "string"}},
        "images": {
            "type": "array",
            "items": {
                "type": "object",
                "required": ["url", "name"],
                "properties": {
                    "url": {"type": "string"},
                    "name": {"type": "string"},
                },
                "additionalProperties": False,
            },
        },
    },
    "additionalProperties": False,
}

URLENCODED_SIMPLE_SCHEMA = {
    "type": "object",
    "required": ["name", "email", "age", "subscribe"],
    "properties": {
        "name": {"type": "string"},
        "email": {"type": "string", "format": "email"},
        "age": {"type": "integer"},
        "subscribe": {"type": "boolean"},
    },
    "additionalProperties": False,
}

URLENCODED_COMPLEX_SCHEMA = {
    "type": "object",
    "required": [
        "username",
        "password",
        "email",
        "first_name",
        "last_name",
        "age",
        "country",
        "state",
        "city",
        "zip",
        "phone",
        "company",
        "job_title",
        "subscribe",
        "newsletter",
        "terms_accepted",
        "privacy_accepted",
        "marketing_consent",
        "two_factor_enabled",
    ],
    "properties": {
        "username": {"type": "string"},
        "password": {"type": "string"},
        "email": {"type": "string", "format": "email"},
        "first_name": {"type": "string"},
        "last_name": {"type": "string"},
        "age": {"type": "integer"},
        "country": {"type": "string"},
        "state": {"type": "string"},
        "city": {"type": "string"},
        "zip": {"type": "string"},
        "phone": {"type": "string"},
        "company": {"type": "string"},
        "job_title": {"type": "string"},
        "subscribe": {"type": "boolean"},
        "newsletter": {"type": "boolean"},
        "terms_accepted": {"type": "boolean"},
        "privacy_accepted": {"type": "boolean"},
        "marketing_consent": {"type": "boolean"},
        "two_factor_enabled": {"type": "boolean"},
    },
    "additionalProperties": False,
}

MULTIPART_FILE_SCHEMA = {
    "type": "object",
    "required": ["filename", "size", "content", "content_type"],
    "properties": {
        "filename": {"type": "string"},
        "size": {"type": "integer"},
        "content": {"type": "string"},
        "content_type": {"type": "string"},
    },
    "additionalProperties": False,
}

MULTIPART_SCHEMA = {
    "type": "object",
    "required": ["file"],
    "properties": {
        "file": {
            "oneOf": [
                MULTIPART_FILE_SCHEMA,
                {"type": "array", "items": MULTIPART_FILE_SCHEMA},
            ]
        }
    },
    "additionalProperties": False,
}


@get("/health")
def health() -> dict[str, Any]:
    return {"status": "ok"}


if _profile_enabled:

    @get("/__benchmark__/flush-profile")
    def flush_profile() -> dict[str, Any]:
        if _profiling_collector is not None:
            _profiling_collector.finalize()
            return {"ok": True}
        return {"ok": False}


@post("/json/small", body_schema=SMALL_PAYLOAD_SCHEMA)
async def post_json_small(body: dict[str, Any]) -> dict[str, Any]:
    """Small JSON payload (~100-500 bytes)."""
    return body


@post("/json/medium", body_schema=MEDIUM_PAYLOAD_SCHEMA)
async def post_json_medium(body: dict[str, Any]) -> dict[str, Any]:
    """Medium JSON payload (~1-10KB)."""
    return body


@post("/json/large", body_schema=LARGE_PAYLOAD_SCHEMA)
async def post_json_large(body: dict[str, Any]) -> dict[str, Any]:
    """Large JSON payload (~10-100KB)."""
    return body


@post("/json/very-large", body_schema=VERY_LARGE_PAYLOAD_SCHEMA)
async def post_json_very_large(body: dict[str, Any]) -> dict[str, Any]:
    """Very large JSON payload (~100KB-1MB)."""
    return body


@post("/multipart/small", body_schema=MULTIPART_SCHEMA)
async def post_multipart_small(body: dict[str, Any]) -> dict[str, Any]:
    """Small multipart form (~1KB)."""
    return {"files_received": 1, "total_bytes": 1024}


@post("/multipart/medium", body_schema=MULTIPART_SCHEMA)
async def post_multipart_medium(body: dict[str, Any]) -> dict[str, Any]:
    """Medium multipart form (~10KB)."""
    return {"files_received": 2, "total_bytes": 10240}


@post("/multipart/large", body_schema=MULTIPART_SCHEMA)
async def post_multipart_large(body: dict[str, Any]) -> dict[str, Any]:
    """Large multipart form (~100KB)."""
    return {"files_received": 5, "total_bytes": 102400}


@post("/urlencoded/simple", body_schema=URLENCODED_SIMPLE_SCHEMA)
async def post_urlencoded_simple(body: dict[str, Any]) -> dict[str, Any]:
    """Simple URL-encoded form (3-5 fields)."""
    return body


@post("/urlencoded/complex", body_schema=URLENCODED_COMPLEX_SCHEMA)
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


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(
        f"Spikard workload server (raw/no validation) starting on port {port}",
        file=sys.stderr,
        flush=True,
    )

    config = ServerConfig(
        host="0.0.0.0",
        port=port,
        workers=1,
    )

    app.run(config=config)
