"""Litestar benchmark server (raw + validated endpoints)."""

import sys
import urllib.parse
from datetime import date as DateType
from typing import Any
from uuid import UUID

from litestar import Litestar, Request, get, post
from msgspec import Struct


async def _parse_urlencoded(request: Request) -> dict[str, Any]:
    raw = await request.body()
    text = raw.decode("utf-8", errors="replace")
    parsed = urllib.parse.parse_qs(text, keep_blank_values=True, strict_parsing=False)
    return {k: (v[0] if len(v) == 1 else v) for k, v in parsed.items()}


# Validation schemas
class SmallPayload(Struct):
    name: str
    description: str
    price: float
    tax: float | None = None


class Image(Struct):
    url: str
    name: str


class MediumPayload(Struct):
    name: str
    price: float
    image: Image


class Country(Struct):
    name: str
    code: str


class Address(Struct):
    street: str
    city: str
    country: Country


class SellerWithAddress(Struct):
    name: str
    address: Address


class LargePayload(Struct):
    name: str
    price: float
    seller: SellerWithAddress


class VeryLargePayload(Struct):
    name: str
    tags: list[str]
    images: list[Image]


# Raw endpoints (no validation)
@post("/json/small", status_code=200)
async def post_json_small(request: Request) -> dict[str, Any]:
    return await request.json()


@post("/json/medium", status_code=200)
async def post_json_medium(request: Request) -> dict[str, Any]:
    return await request.json()


@post("/json/large", status_code=200)
async def post_json_large(request: Request) -> dict[str, Any]:
    return await request.json()


@post("/json/very-large", status_code=200)
async def post_json_very_large(request: Request) -> dict[str, Any]:
    return await request.json()


@post("/multipart/small", status_code=200)
async def post_multipart_small() -> dict[str, int]:
    return {"files_received": 1, "total_bytes": 1024}


@post("/multipart/medium", status_code=200)
async def post_multipart_medium() -> dict[str, int]:
    return {"files_received": 2, "total_bytes": 10240}


@post("/multipart/large", status_code=200)
async def post_multipart_large() -> dict[str, int]:
    return {"files_received": 5, "total_bytes": 102400}


@post("/urlencoded/simple", status_code=200)
async def post_urlencoded_simple(request: Request) -> dict[str, Any]:
    return await _parse_urlencoded(request)


@post("/urlencoded/complex", status_code=200)
async def post_urlencoded_complex(request: Request) -> dict[str, Any]:
    return await _parse_urlencoded(request)


@get("/path/simple/{id:str}")
async def get_path_simple(id: str) -> dict[str, str]:
    return {"id": id}


@get("/path/multiple/{user_id:str}/{post_id:str}")
async def get_path_multiple(user_id: str, post_id: str) -> dict[str, str]:
    return {"user_id": user_id, "post_id": post_id}


@get("/path/deep/{org:str}/{team:str}/{project:str}/{resource:str}/{id:str}")
async def get_path_deep(org: str, team: str, project: str, resource: str, id: str) -> dict[str, str]:
    return {"org": org, "team": team, "project": project, "resource": resource, "id": id}


@get("/path/int/{id:int}")
async def get_path_int(id: int) -> dict[str, int]:
    return {"id": id}


@get("/path/uuid/{uuid:uuid}")
async def get_path_uuid(uuid: UUID) -> dict[str, str]:
    return {"uuid": str(uuid)}


@get("/path/date/{date:date}")
async def get_path_date(date: DateType) -> dict[str, str]:
    return {"date": date.isoformat()}


@get("/query/few")
async def get_query_few(request: Request) -> dict[str, Any]:
    return dict(request.query_params)


@get("/query/medium")
async def get_query_medium(request: Request) -> dict[str, Any]:
    return dict(request.query_params)


@get("/query/many")
async def get_query_many(request: Request) -> dict[str, Any]:
    return dict(request.query_params)


@get("/health")
async def health() -> dict[str, str]:
    return {"status": "ok"}


@get("/")
async def root() -> dict[str, str]:
    return {"status": "ok"}


# Validated endpoints (with _validated suffix, under /validated prefix)
@post("/validated/json/small", status_code=200)
async def post_json_small_validated(data: SmallPayload) -> SmallPayload:
    return data


@post("/validated/json/medium", status_code=200)
async def post_json_medium_validated(data: MediumPayload) -> MediumPayload:
    return data


@post("/validated/json/large", status_code=200)
async def post_json_large_validated(data: LargePayload) -> LargePayload:
    return data


@post("/validated/json/very-large", status_code=200)
async def post_json_very_large_validated(data: VeryLargePayload) -> VeryLargePayload:
    return data


@post("/validated/multipart/small", status_code=200)
async def post_multipart_small_validated() -> dict[str, int]:
    return {"files_received": 1, "total_bytes": 1024}


@post("/validated/multipart/medium", status_code=200)
async def post_multipart_medium_validated() -> dict[str, int]:
    return {"files_received": 2, "total_bytes": 10240}


@post("/validated/multipart/large", status_code=200)
async def post_multipart_large_validated() -> dict[str, int]:
    return {"files_received": 5, "total_bytes": 102400}


@post("/validated/urlencoded/simple", status_code=200)
async def post_urlencoded_simple_validated(request: Request) -> dict[str, Any]:
    return await _parse_urlencoded(request)


@post("/validated/urlencoded/complex", status_code=200)
async def post_urlencoded_complex_validated(request: Request) -> dict[str, Any]:
    return await _parse_urlencoded(request)


@get("/validated/path/simple/{id:str}")
async def get_path_simple_validated(id: str) -> dict[str, str]:
    return {"id": id}


@get("/validated/path/multiple/{user_id:str}/{post_id:str}")
async def get_path_multiple_validated(user_id: str, post_id: str) -> dict[str, str]:
    return {"user_id": user_id, "post_id": post_id}


@get("/validated/path/deep/{org:str}/{team:str}/{project:str}/{resource:str}/{id:str}")
async def get_path_deep_validated(org: str, team: str, project: str, resource: str, id: str) -> dict[str, str]:
    return {"org": org, "team": team, "project": project, "resource": resource, "id": id}


@get("/validated/path/int/{id:int}")
async def get_path_int_validated(id: int) -> dict[str, int]:
    return {"id": id}


@get("/validated/path/uuid/{uuid:uuid}")
async def get_path_uuid_validated(uuid: UUID) -> dict[str, str]:
    return {"uuid": str(uuid)}


@get("/validated/path/date/{date:date}")
async def get_path_date_validated(date: DateType) -> dict[str, str]:
    return {"date": date.isoformat()}


@get("/validated/query/few")
async def get_query_few_validated(request: Request) -> dict[str, Any]:
    return dict(request.query_params)


@get("/validated/query/medium")
async def get_query_medium_validated(request: Request) -> dict[str, Any]:
    return dict(request.query_params)


@get("/validated/query/many")
async def get_query_many_validated(request: Request) -> dict[str, Any]:
    return dict(request.query_params)


app = Litestar(
    route_handlers=[
        # Raw endpoints
        post_json_small,
        post_json_medium,
        post_json_large,
        post_json_very_large,
        post_multipart_small,
        post_multipart_medium,
        post_multipart_large,
        post_urlencoded_simple,
        post_urlencoded_complex,
        get_path_simple,
        get_path_multiple,
        get_path_deep,
        get_path_int,
        get_path_uuid,
        get_path_date,
        get_query_few,
        get_query_medium,
        get_query_many,
        health,
        root,
        # Validated endpoints
        post_json_small_validated,
        post_json_medium_validated,
        post_json_large_validated,
        post_json_very_large_validated,
        post_multipart_small_validated,
        post_multipart_medium_validated,
        post_multipart_large_validated,
        post_urlencoded_simple_validated,
        post_urlencoded_complex_validated,
        get_path_simple_validated,
        get_path_multiple_validated,
        get_path_deep_validated,
        get_path_int_validated,
        get_path_uuid_validated,
        get_path_date_validated,
        get_query_few_validated,
        get_query_medium_validated,
        get_query_many_validated,
    ]
)

if __name__ == "__main__":
    from granian import Granian
    from granian.constants import Interfaces

    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(
        f"[litestar] Starting server on port {port}",
        file=sys.stderr,
        flush=True,
    )

    Granian(
        "__main__:app",
        address="0.0.0.0",
        port=port,
        interface=Interfaces.ASGI,
        workers=1,
        log_level="error",
        http1_buffer_size=16 * 1024 * 1024,
    ).serve()
