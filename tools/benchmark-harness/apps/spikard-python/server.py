"""Spikard Python HTTP server for workload benchmarking.

This server implements all workload types to measure Python binding performance.
Includes both raw (no validation) and validated endpoints.

Raw endpoints: original paths (e.g., /json/small)
Validated endpoints: /validated/... prefix (e.g., /validated/json/small)
"""

import json
import sys
from datetime import date as DateType
from pathlib import Path as PathLib
from typing import Any
from uuid import UUID

from spikard import Path, Query, Spikard, get, post
from spikard.config import ServerConfig
from spikard.routing import get_default_router

app = Spikard()

JsonScalar = str | int | float | bool | None

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


@get("/health", response_schema=response_schema("health"))
def health() -> dict[str, str]:
    return {"status": "ok"}


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
def post_validated_json_small(body: dict[str, Any]) -> dict[str, Any]:
    """Small JSON payload (~100-500 bytes) - validated."""
    return body


@post(
    "/validated/json/medium", body_schema=request_schema("json/medium"), response_schema=response_schema("json/medium")
)
def post_validated_json_medium(body: dict[str, Any]) -> dict[str, Any]:
    """Medium JSON payload (nested object) - validated."""
    return body


@post("/validated/json/large", body_schema=request_schema("json/large"), response_schema=response_schema("json/large"))
def post_validated_json_large(body: dict[str, Any]) -> dict[str, Any]:
    """Large JSON payload (~10-100KB) - validated."""
    return body


@post(
    "/validated/json/very-large",
    body_schema=request_schema("json/very-large"),
    response_schema=response_schema("json/very-large"),
)
def post_validated_json_very_large(body: dict[str, Any]) -> dict[str, Any]:
    """Very large JSON payload (arrays of values and objects) - validated."""
    return body


@get(
    "/validated/path/simple/{id}",
    response_schema=response_schema("path/simple"),
    parameter_schema=parameter_schema("path/simple"),
)
def get_validated_path_simple(id: str = Path()) -> dict[str, JsonScalar]:
    """Single path parameter - validated."""
    return {"id": id}


@get(
    "/validated/path/multiple/{user_id}/{post_id}",
    response_schema=response_schema("path/multiple"),
    parameter_schema=parameter_schema("path/multiple"),
)
def get_validated_path_multiple(user_id: str = Path(), post_id: str = Path()) -> dict[str, JsonScalar]:
    """Multiple path parameters - validated."""
    return {"user_id": user_id, "post_id": post_id}


@get(
    "/validated/path/deep/{org}/{team}/{project}/{resource}/{id}",
    response_schema=response_schema("path/deep"),
    parameter_schema=parameter_schema("path/deep"),
)
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
def get_validated_path_int(id: int = Path()) -> dict[str, JsonScalar]:
    """Integer path parameter - validated."""
    return {"id": id}


@get(
    "/validated/path/uuid/{uuid}",
    response_schema=response_schema("path/uuid"),
    parameter_schema=parameter_schema("path/uuid"),
)
def get_validated_path_uuid(uuid: UUID = Path()) -> dict[str, JsonScalar]:
    """UUID path parameter - validated."""
    return {"uuid": str(uuid)}


@get(
    "/validated/path/date/{date}",
    response_schema=response_schema("path/date"),
    parameter_schema=parameter_schema("path/date"),
)
def get_validated_path_date(date: DateType = Path()) -> dict[str, JsonScalar]:
    """Date path parameter - validated."""
    return {"date": date.isoformat()}


@get(
    "/validated/query/few", response_schema=response_schema("query/few"), parameter_schema=parameter_schema("query/few")
)
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

    app.include_router(get_default_router())
    app.run(config=config)
