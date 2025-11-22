"""Litestar HTTP server for workload benchmarking with Granian.

This server implements all workload types to measure Litestar performance
against spikard-python, FastAPI, and the pure Rust baseline.

Uses msgspec.Struct for validation following Litestar best practices with
zero-overhead serialization. Granian is used as the ASGI server for
improved performance.
"""
# /// script
# dependencies = [
#     "litestar",
#     "granian",
#     "msgspec",
# ]
# ///

import sys
from typing import Any

import msgspec
from litestar import Litestar, get, post
from litestar.datastructures import FormMultiDict
from litestar.dto import MsgspecDTO
from litestar.enums import RequestEncodingType
from litestar.params import Body

# ============================================================================
# JSON Body DTOs (msgspec.Struct for validation)
# ============================================================================
# These match the fixtures in testing_data/json_bodies/ and spikard-python schemas


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


# ============================================================================
# JSON Body Workloads
# ============================================================================


@post("/json/small", sync_to_thread=False)
def post_json_small(data: SmallPayload) -> SmallPayload:
    """Small JSON payload (~100-500 bytes)."""
    return data


@post("/json/medium", sync_to_thread=False)
def post_json_medium(data: MediumPayload) -> MediumPayload:
    """Medium JSON payload (~1-10KB)."""
    return data


@post("/json/large", sync_to_thread=False)
def post_json_large(data: LargePayload) -> LargePayload:
    """Large JSON payload (~10-100KB)."""
    return data


@post("/json/very-large", sync_to_thread=False)
def post_json_very_large(data: VeryLargePayload) -> VeryLargePayload:
    """Very large JSON payload (~100KB-1MB)."""
    return data


# ============================================================================
# Multipart Form Workloads
# ============================================================================
# Note: These return mock responses matching spikard-python behavior


@post("/multipart/small", sync_to_thread=False)
def post_multipart_small() -> dict[str, Any]:
    """Small multipart form (~1KB)."""
    return {"files_received": 1, "total_bytes": 1024}


@post("/multipart/medium", sync_to_thread=False)
def post_multipart_medium() -> dict[str, Any]:
    """Medium multipart form (~10KB)."""
    return {"files_received": 2, "total_bytes": 10240}


@post("/multipart/large", sync_to_thread=False)
def post_multipart_large() -> dict[str, Any]:
    """Large multipart form (~100KB)."""
    return {"files_received": 5, "total_bytes": 102400}


# ============================================================================
# URL Encoded Form Workloads
# ============================================================================


class SimpleForm(msgspec.Struct):
    """Simple URL-encoded form data."""

    username: str
    email: str
    password: str


class ComplexForm(msgspec.Struct):
    """Complex URL-encoded form data."""

    # User details
    username: str
    email: str
    password: str
    first_name: str
    last_name: str
    # Address
    street: str
    city: str
    state: str
    zip_code: str
    country: str
    # Preferences
    newsletter: bool = False
    notifications: bool = False
    theme: str = "light"
    language: str = "en"
    timezone: str = "UTC"


@post(
    "/urlencoded/simple",
    sync_to_thread=False,
)
def post_urlencoded_simple(
    data: SimpleForm = Body(media_type=RequestEncodingType.URL_ENCODED),
) -> dict[str, str]:
    """Simple URL-encoded form (3-5 fields)."""
    return {"username": data.username, "email": data.email, "password": data.password}


@post(
    "/urlencoded/complex",
    sync_to_thread=False,
)
def post_urlencoded_complex(
    data: ComplexForm = Body(media_type=RequestEncodingType.URL_ENCODED),
) -> dict[str, Any]:
    """Complex URL-encoded form (10-20 fields)."""
    return {
        "username": data.username,
        "email": data.email,
        "password": data.password,
        "first_name": data.first_name,
        "last_name": data.last_name,
        "street": data.street,
        "city": data.city,
        "state": data.state,
        "zip_code": data.zip_code,
        "country": data.country,
        "newsletter": data.newsletter,
        "notifications": data.notifications,
        "theme": data.theme,
        "language": data.language,
        "timezone": data.timezone,
    }


# ============================================================================
# Path Parameter Workloads
# ============================================================================


@get("/path/simple/{id:str}", sync_to_thread=False)
def get_path_simple(id: str) -> dict[str, str]:
    """Single path parameter."""
    return {"id": id}


@get("/path/multiple/{user_id:str}/{post_id:str}", sync_to_thread=False)
def get_path_multiple(user_id: str, post_id: str) -> dict[str, str]:
    """Multiple path parameters."""
    return {"user_id": user_id, "post_id": post_id}


@get(
    "/path/deep/{org:str}/{team:str}/{project:str}/{resource:str}/{id:str}",
    sync_to_thread=False,
)
def get_path_deep(
    org: str,
    team: str,
    project: str,
    resource: str,
    id: str,
) -> dict[str, str]:
    """Deep path parameters (5 levels)."""
    return {
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id,
    }


@get("/path/int/{id:int}", sync_to_thread=False)
def get_path_int(id: int) -> dict[str, int]:
    """Integer path parameter with type validation."""
    return {"id": id}


@get("/path/uuid/{uuid:str}", sync_to_thread=False)
def get_path_uuid(uuid: str) -> dict[str, str]:
    """UUID path parameter."""
    return {"uuid": uuid}


@get("/path/date/{date:str}", sync_to_thread=False)
def get_path_date(date: str) -> dict[str, str]:
    """Date path parameter."""
    return {"date": date}


# ============================================================================
# Query Parameter Workloads
# ============================================================================


@get("/query/few", sync_to_thread=False)
def get_query_few(
    q: str | None = None,
    page: int | None = None,
    limit: int | None = None,
) -> dict[str, Any]:
    """Few query parameters (1-3)."""
    return {"q": q, "page": page, "limit": limit}


@get("/query/medium", sync_to_thread=False)
def get_query_medium(
    category: str | None = None,
    tags: str | None = None,
    min_price: float | None = None,
    max_price: float | None = None,
    sort: str | None = None,
    order: str | None = None,
    page: int | None = None,
    limit: int | None = None,
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


@get("/query/many", sync_to_thread=False)
def get_query_many(
    param1: str | None = None,
    param2: str | None = None,
    param3: str | None = None,
    param4: str | None = None,
    param5: str | None = None,
    param6: str | None = None,
    param7: str | None = None,
    param8: str | None = None,
    param9: str | None = None,
    param10: str | None = None,
    param11: str | None = None,
    param12: str | None = None,
    param13: str | None = None,
    param14: str | None = None,
    param15: str | None = None,
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


# ============================================================================
# Health Check
# ============================================================================


@get("/health", sync_to_thread=False)
def health() -> dict[str, str]:
    """Health check endpoint."""
    return {"status": "ok"}


# ============================================================================
# Litestar App Configuration
# ============================================================================

app = Litestar(
    route_handlers=[
        # JSON body endpoints
        post_json_small,
        post_json_medium,
        post_json_large,
        post_json_very_large,
        # Multipart form endpoints
        post_multipart_small,
        post_multipart_medium,
        post_multipart_large,
        # URL-encoded form endpoints
        post_urlencoded_simple,
        post_urlencoded_complex,
        # Path parameter endpoints
        get_path_simple,
        get_path_multiple,
        get_path_deep,
        get_path_int,
        get_path_uuid,
        get_path_date,
        # Query parameter endpoints
        get_query_few,
        get_query_medium,
        get_query_many,
        # Health check
        health,
    ],
    debug=False,
)

# ============================================================================
# Main Server Entry Point
# ============================================================================

if __name__ == "__main__":
    from granian import Granian
    from granian.constants import Interfaces

    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(
        f"Litestar + Granian server starting on port {port}",
        file=sys.stderr,
        flush=True,
    )

    Granian(
        "server:app",
        address="0.0.0.0",
        port=port,
        interface=Interfaces.ASGI,
        workers=1,
        threads=1,
        log_level="error",
    ).serve()
