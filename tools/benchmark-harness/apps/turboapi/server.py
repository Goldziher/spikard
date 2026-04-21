"""turboAPI benchmark server (raw + validated endpoints).

turboAPI is a Python framework with a Zig HTTP backend for high performance.
Uses direct parameter injection (FastAPI-compatible) instead of request objects.
Validated endpoints use dhi BaseModel for type checking and validation.

NOTE: turboAPI does not natively support multipart or URL-encoded form parsing.
Those endpoints are omitted from this benchmark app. The harness will skip
workloads that target missing endpoints gracefully via health-check preflight.
"""

import sys
from datetime import date as DateType
from typing import Optional
from uuid import UUID

from dhi import BaseModel
from turboapi import TurboAPI

app = TurboAPI()


# ============================================================================
# dhi Models for Validation
# ============================================================================


class SmallPayload(BaseModel):
    """Small JSON payload - matches 01_simple_object_success.json."""

    name: str
    description: str
    price: float
    tax: Optional[float] = None


class Image(BaseModel):
    url: str
    name: str


class MediumPayload(BaseModel):
    name: str
    price: float
    image: Image


class Country(BaseModel):
    name: str
    code: str


class Address(BaseModel):
    street: str
    city: str
    country: Country


class SellerWithAddress(BaseModel):
    name: str
    address: Address


class LargePayload(BaseModel):
    name: str
    price: float
    seller: SellerWithAddress


class VeryLargePayload(BaseModel):
    name: str
    tags: list[str]
    images: list[Image]


class UrlencodedSimple(BaseModel):
    """Simple URL-encoded form validation schema."""

    name: str
    email: str
    age: int
    subscribe: bool


class UrlencodedComplex(BaseModel):
    """Complex URL-encoded form validation schema."""

    username: str
    password: str
    email: str
    first_name: str
    last_name: str
    age: int
    country: str
    state: str
    city: str
    zip: str
    phone: str
    company: str
    job_title: str
    subscribe: bool
    newsletter: bool
    terms_accepted: bool
    privacy_accepted: bool
    marketing_consent: bool
    two_factor_enabled: bool


class QueryFew(BaseModel):
    """Query parameters validation schema for few params."""

    q: str
    page: Optional[int] = None
    limit: Optional[int] = None


class QueryMedium(BaseModel):
    """Query parameters validation schema for medium params."""

    search: str
    category: Optional[str] = None
    sort: Optional[str] = None
    order: Optional[str] = None
    page: Optional[int] = None
    limit: Optional[int] = None
    filter: Optional[str] = None


class QueryMany(BaseModel):
    """Query parameters validation schema for many params."""

    q: str
    category: Optional[str] = None
    subcategory: Optional[str] = None
    brand: Optional[str] = None
    min_price: Optional[float] = None
    max_price: Optional[float] = None
    color: Optional[str] = None
    size: Optional[str] = None
    material: Optional[str] = None
    rating: Optional[int] = None
    sort: Optional[str] = None
    order: Optional[str] = None
    page: Optional[int] = None
    limit: Optional[int] = None
    in_stock: Optional[bool] = None
    on_sale: Optional[bool] = None


# ============================================================================
# Raw Endpoints (No Validation) - JSON Bodies
# ============================================================================


@app.post("/json/small")
def post_json_small(body: dict):
    """Small JSON body (~100 bytes) - no validation."""
    return body


@app.post("/json/medium")
def post_json_medium(body: dict):
    """Medium JSON body (~1KB) - no validation."""
    return body


@app.post("/json/large")
def post_json_large(body: dict):
    """Large JSON body (~10KB) - no validation."""
    return body


@app.post("/json/very-large")
def post_json_very_large(body: dict):
    """Very large JSON body (~100KB) - no validation."""
    return body


# ============================================================================
# Raw Endpoints (No Validation) - Path Parameters
# ============================================================================


@app.get("/path/simple/{id}")
def get_path_simple(id: str):
    """Single path parameter."""
    return {"id": id}


@app.get("/path/multiple/{user_id}/{post_id}")
def get_path_multiple(user_id: str, post_id: str):
    """Multiple path parameters."""
    return {"user_id": user_id, "post_id": post_id}


@app.get("/path/deep/{org}/{team}/{project}/{resource}/{id}")
def get_path_deep(org: str, team: str, project: str, resource: str, id: str):
    """Deep nested path parameters."""
    return {
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id,
    }


@app.get("/path/int/{id}")
def get_path_int(id: int):
    """Path parameter with int type."""
    return {"id": id}


@app.get("/path/uuid/{uuid}")
def get_path_uuid(uuid: str):
    """Path parameter with UUID."""
    parsed = UUID(uuid)
    return {"uuid": str(parsed)}


@app.get("/path/date/{date}")
def get_path_date(date: str):
    """Path parameter with date."""
    parsed = DateType.fromisoformat(date)
    return {"date": parsed.isoformat()}


# ============================================================================
# Raw Endpoints (No Validation) - Query Parameters
# ============================================================================


@app.get("/query/few")
def get_query_few(q: str, page: Optional[int] = None, limit: Optional[int] = None):
    """Few query parameters (1-2)."""
    result = {"q": q}
    if page is not None:
        result["page"] = str(page)
    if limit is not None:
        result["limit"] = str(limit)
    return result


@app.get("/query/medium")
def get_query_medium(
    search: str,
    category: Optional[str] = None,
    sort: Optional[str] = None,
    order: Optional[str] = None,
    page: Optional[int] = None,
    limit: Optional[int] = None,
    filter: Optional[str] = None,
):
    """Medium query parameters (3-5)."""
    result = {"search": search}
    for key, val in [
        ("category", category),
        ("sort", sort),
        ("order", order),
        ("page", page),
        ("limit", limit),
        ("filter", filter),
    ]:
        if val is not None:
            result[key] = str(val)
    return result


@app.get("/query/many")
def get_query_many(
    q: str,
    category: Optional[str] = None,
    subcategory: Optional[str] = None,
    brand: Optional[str] = None,
    min_price: Optional[float] = None,
    max_price: Optional[float] = None,
    color: Optional[str] = None,
    size: Optional[str] = None,
    material: Optional[str] = None,
    rating: Optional[int] = None,
    sort: Optional[str] = None,
    order: Optional[str] = None,
    page: Optional[int] = None,
    limit: Optional[int] = None,
    in_stock: Optional[bool] = None,
    on_sale: Optional[bool] = None,
):
    """Many query parameters (6-10)."""
    result = {"q": q}
    for key, val in [
        ("category", category),
        ("subcategory", subcategory),
        ("brand", brand),
        ("min_price", min_price),
        ("max_price", max_price),
        ("color", color),
        ("size", size),
        ("material", material),
        ("rating", rating),
        ("sort", sort),
        ("order", order),
        ("page", page),
        ("limit", limit),
        ("in_stock", in_stock),
        ("on_sale", on_sale),
    ]:
        if val is not None:
            result[key] = str(val)
    return result


# ============================================================================
# Validated Endpoints (dhi Validation) - JSON Bodies
# ============================================================================


@app.post("/validated/json/small")
def post_json_small_validated(body: SmallPayload):
    """Small JSON body (~100 bytes) with dhi validation."""
    return body.model_dump()


@app.post("/validated/json/medium")
def post_json_medium_validated(body: MediumPayload):
    """Medium JSON body (~1KB) with dhi validation."""
    return body.model_dump()


@app.post("/validated/json/large")
def post_json_large_validated(body: LargePayload):
    """Large JSON body (~10KB) with dhi validation."""
    return body.model_dump()


@app.post("/validated/json/very-large")
def post_json_very_large_validated(body: VeryLargePayload):
    """Very large JSON body (~100KB) with dhi validation."""
    return body.model_dump()


# ============================================================================
# Validated Endpoints (dhi Validation) - Path Parameters
# ============================================================================


@app.get("/validated/path/simple/{id}")
def get_path_simple_validated(id: str):
    """Single path parameter with validation."""
    if not id or len(id) > 255 or not id.replace("-", "").replace("_", "").isalnum():
        return {
            "detail": "Path parameter 'id' must be non-empty, alphanumeric (with - or _), and max 255 characters"
        }, 400
    return {"id": id}


@app.get("/validated/path/multiple/{user_id}/{post_id}")
def get_path_multiple_validated(user_id: str, post_id: str):
    """Multiple path parameters with validation."""
    if not user_id or len(user_id) > 255 or not user_id.replace("-", "").replace("_", "").isalnum():
        return {
            "detail": "Path parameter 'user_id' must be non-empty, alphanumeric (with - or _), and max 255 characters"
        }, 400
    if not post_id or len(post_id) > 255 or not post_id.replace("-", "").replace("_", "").isalnum():
        return {
            "detail": "Path parameter 'post_id' must be non-empty, alphanumeric (with - or _), and max 255 characters"
        }, 400
    return {"user_id": user_id, "post_id": post_id}


@app.get("/validated/path/deep/{org}/{team}/{project}/{resource}/{id}")
def get_path_deep_validated(org: str, team: str, project: str, resource: str, id: str):
    """Deep nested path parameters with validation."""
    for param_name, param_value in [
        ("org", org),
        ("team", team),
        ("project", project),
        ("resource", resource),
        ("id", id),
    ]:
        if not param_value or len(param_value) > 255 or not param_value.replace("-", "").replace("_", "").isalnum():
            return {
                "detail": f"Path parameter '{param_name}' must be non-empty, alphanumeric (with - or _), and max 255 characters"
            }, 400
    return {
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id,
    }


@app.get("/validated/path/int/{id}")
def get_path_int_validated(id: int):
    """Path parameter with int type."""
    return {"id": id}


@app.get("/validated/path/uuid/{uuid}")
def get_path_uuid_validated(uuid: str):
    """Path parameter with UUID."""
    parsed = UUID(uuid)
    return {"uuid": str(parsed)}


@app.get("/validated/path/date/{date}")
def get_path_date_validated(date: str):
    """Path parameter with date."""
    parsed = DateType.fromisoformat(date)
    return {"date": parsed.isoformat()}


# ============================================================================
# Validated Endpoints (dhi Validation) - Query Parameters
# ============================================================================


@app.get("/validated/query/few")
def get_query_few_validated(q: str, page: Optional[int] = None, limit: Optional[int] = None):
    """Few query parameters (1-2) with dhi validation."""
    validated = QueryFew(q=q, page=page, limit=limit)
    return validated.model_dump(exclude_none=True)


@app.get("/validated/query/medium")
def get_query_medium_validated(
    search: str,
    category: Optional[str] = None,
    sort: Optional[str] = None,
    order: Optional[str] = None,
    page: Optional[int] = None,
    limit: Optional[int] = None,
    filter: Optional[str] = None,
):
    """Medium query parameters (3-5) with dhi validation."""
    validated = QueryMedium(
        search=search,
        category=category,
        sort=sort,
        order=order,
        page=page,
        limit=limit,
        filter=filter,
    )
    return validated.model_dump(exclude_none=True)


@app.get("/validated/query/many")
def get_query_many_validated(
    q: str,
    category: Optional[str] = None,
    subcategory: Optional[str] = None,
    brand: Optional[str] = None,
    min_price: Optional[float] = None,
    max_price: Optional[float] = None,
    color: Optional[str] = None,
    size: Optional[str] = None,
    material: Optional[str] = None,
    rating: Optional[int] = None,
    sort: Optional[str] = None,
    order: Optional[str] = None,
    page: Optional[int] = None,
    limit: Optional[int] = None,
    in_stock: Optional[bool] = None,
    on_sale: Optional[bool] = None,
):
    """Many query parameters (6-10) with dhi validation."""
    validated = QueryMany(
        q=q,
        category=category,
        subcategory=subcategory,
        brand=brand,
        min_price=min_price,
        max_price=max_price,
        color=color,
        size=size,
        material=material,
        rating=rating,
        sort=sort,
        order=order,
        page=page,
        limit=limit,
        in_stock=in_stock,
        on_sale=on_sale,
    )
    return validated.model_dump(exclude_none=True)


# ============================================================================
# Health & Root Endpoints
# ============================================================================


@app.get("/health")
def health():
    """Health check endpoint."""
    return {"status": "ok"}


@app.get("/")
def root():
    """Root endpoint."""
    return {"status": "ok"}


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(
        f"[turboapi] Starting server on port {port}",
        file=sys.stderr,
        flush=True,
    )
    app.run(host="0.0.0.0", port=port)
