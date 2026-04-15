"""Litestar benchmark server (raw + validated endpoints)."""

import sys
from datetime import date as DateType
from typing import Any
from uuid import UUID

from litestar import Litestar, Request, get, post
from msgspec import Struct


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


class QueryFew(Struct):
    """Query parameters validation schema for few params."""

    q: str
    page: int | None = None
    limit: int | None = None


class QueryMedium(Struct):
    """Query parameters validation schema for medium params."""

    search: str
    category: str | None = None
    sort: str | None = None
    order: str | None = None
    page: int | None = None
    limit: int | None = None
    filter: str | None = None


class QueryMany(Struct):
    """Query parameters validation schema for many params."""

    q: str
    category: str | None = None
    subcategory: str | None = None
    brand: str | None = None
    min_price: float | None = None
    max_price: float | None = None
    color: str | None = None
    size: str | None = None
    material: str | None = None
    rating: int | None = None
    sort: str | None = None
    order: str | None = None
    page: int | None = None
    limit: int | None = None
    in_stock: bool | None = None
    on_sale: bool | None = None


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


@get("/validated/path/simple/{id:str}")
async def get_path_simple_validated(id: str) -> Any:
    if not id or len(id) > 255 or not id.replace("-", "").replace("_", "").isalnum():
        from litestar.exceptions import ValidationException

        raise ValidationException(
            "Path parameter 'id' must be non-empty, alphanumeric (with - or _), and max 255 characters"
        )
    return {"id": id}


@get("/validated/path/multiple/{user_id:str}/{post_id:str}")
async def get_path_multiple_validated(user_id: str, post_id: str) -> Any:
    if not user_id or len(user_id) > 255 or not user_id.replace("-", "").replace("_", "").isalnum():
        from litestar.exceptions import ValidationException

        raise ValidationException(
            "Path parameter 'user_id' must be non-empty, alphanumeric (with - or _), and max 255 characters"
        )
    if not post_id or len(post_id) > 255 or not post_id.replace("-", "").replace("_", "").isalnum():
        from litestar.exceptions import ValidationException

        raise ValidationException(
            "Path parameter 'post_id' must be non-empty, alphanumeric (with - or _), and max 255 characters"
        )
    return {"user_id": user_id, "post_id": post_id}


@get("/validated/path/deep/{org:str}/{team:str}/{project:str}/{resource:str}/{id:str}")
async def get_path_deep_validated(org: str, team: str, project: str, resource: str, id: str) -> Any:
    from litestar.exceptions import ValidationException

    for param_name, param_value in [
        ("org", org),
        ("team", team),
        ("project", project),
        ("resource", resource),
        ("id", id),
    ]:
        if not param_value or len(param_value) > 255 or not param_value.replace("-", "").replace("_", "").isalnum():
            raise ValidationException(
                f"Path parameter '{param_name}' must be non-empty, alphanumeric (with - or _), and max 255 characters"
            )
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
async def get_query_few_validated(request: Request) -> Any:
    params = dict(request.query_params)
    try:
        import msgspec

        validated = msgspec.convert(params, QueryFew)
        result = msgspec.structs.asdict(validated)
        return {k: v for k, v in result.items() if v is not None}
    except Exception as e:
        from litestar.exceptions import ValidationException

        raise ValidationException(str(e))


@get("/validated/query/medium")
async def get_query_medium_validated(request: Request) -> Any:
    params = dict(request.query_params)
    try:
        import msgspec

        validated = msgspec.convert(params, QueryMedium)
        result = msgspec.structs.asdict(validated)
        return {k: v for k, v in result.items() if v is not None}
    except Exception as e:
        from litestar.exceptions import ValidationException

        raise ValidationException(str(e))


@get("/validated/query/many")
async def get_query_many_validated(request: Request) -> Any:
    params = dict(request.query_params)
    try:
        import msgspec

        validated = msgspec.convert(params, QueryMany)
        result = msgspec.structs.asdict(validated)
        return {k: v for k, v in result.items() if v is not None}
    except Exception as e:
        from litestar.exceptions import ValidationException

        raise ValidationException(str(e))


app = Litestar(
    route_handlers=[
        # Raw endpoints
        post_json_small,
        post_json_medium,
        post_json_large,
        post_json_very_large,
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
    ).serve()
